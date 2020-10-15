use std::cmp::Ordering;
use std::collections::HashMap;
use std::mem;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u64);
#[derive(Clone, PartialEq, Eq, Hash)]
struct ID(pub u64);
type Offset = u64;
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Clone, Debug)]
pub struct Data(Vec<(Offset, Vec<u8>)>);
impl Data {
    fn new() -> Self {
        Self(vec![])
    }
    fn search(target: u64) -> impl Fn(&(u64, Vec<u8>)) -> Ordering {
        move |(start, x): &(u64, Vec<u8>)| {
            let end = start + x.len() as u64;
            if end < target {
                Ordering::Less
            } else if target < *start {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
    }
    fn append(&mut self, offset: u64, data: &[u8]) {
        let (mut fill, start_idx, start_off) = match self.0.binary_search_by(Self::search(offset)) {
            Ok(i) => {
                let mut v = mem::take(&mut self.0[i].1);
                v.truncate((offset - self.0[i].0) as usize);
                (v, i, self.0[i].0)
            }
            Err(i) => (vec![], i, offset),
        };
        fill.extend_from_slice(data);
        let end_idx = match self
            .0
            .binary_search_by(Self::search(offset + data.len() as u64))
        {
            Ok(i) => {
                let tgt_offset = (offset + data.len() as u64) - self.0[i].0;
                fill.extend_from_slice(&self.0[i].1[tgt_offset as usize..]);
                i
            }
            Err(i) => i.saturating_sub(1),
        };
        if (start_idx + 1) <= end_idx && self.0.len() > 0 {
            self.0.drain((start_idx + 1)..=end_idx);
        }
        if start_idx == self.0.len() {
            self.0.push((start_off, fill));
        } else {
            self.0[start_idx] = (start_off, fill);
        }
        // A
    }
    fn overlap(&self, offset: u64, len: u64) -> bool {
        let start = match self.0.binary_search_by(Self::search(offset)) {
            Ok(_) => return true,
            Err(i) => i,
        };
        let end = match self.0.binary_search_by(Self::search(offset + len)) {
            Ok(_) => return true,
            Err(i) => i,
        };
        if (end - start) > 0 {
            true
        } else {
            false
        }
    }
}

struct DataStore(HashMap<ID, (Version, Data)>);

impl DataStore {
    fn lookup(&self, q: &Query<Incoming>) -> Option<(Version, &Data)> {
        self.0
            .get(&q.find)
            .map(|(v, d)| (*v, d))
            .filter(|(v, _)| v > &q.min_version)
            .filter(|(_, d)| d.overlap(q.offset, q.length))
    }
    fn store(&mut self, a: &Answer<Incoming>) {
        if let Some((v, d)) = self.0.get_mut(&a.has) {
            if a.version == *v {
                d.append(a.offset, &a.data)
            } else if a.version > *v {
                *d = Data::new();
                *v = a.version;
                d.append(a.offset, &a.data)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Incoming;
#[derive(Copy, Clone)]
pub struct Outgoing;

#[derive(Clone, PartialEq, Eq)]
pub struct Host(pub u64);

#[derive(Clone)]
pub struct Query<T> {
    find: ID,
    seq: u64, // increments on retransmission, distinguishes retransmitted packets from packets caught in a routing loop
    min_version: Version,
    from: Host,
    tell: Host,
    offset: u64,
    length: u64,
    kind: T,
}

impl Query<Incoming> {
    fn same(&self, other: &Self) -> bool {
        self.find == other.find
            && self.seq == other.seq
            && self.min_version == other.min_version
            && self.tell == other.tell
            && self.offset == other.offset
            && self.length == other.length
    }
}

pub struct ProtocolRunner {
    data: DataStore,
    pending_queries: Vec<Query<Incoming>>,
    seen_queries: Vec<Query<Incoming>>,
    peers: Vec<Host>,
}

pub struct Answer<T> {
    has: ID,
    tell: Host, // TODO: is this necessary
    version: Version,
    offset: u64,
    data: Vec<u8>,
    kind: T,
}

impl Answer<Incoming> {
    fn answers(&self, q: &Query<Incoming>) -> bool {
        self.has == q.find
            && self.version >= q.min_version
            // check overlap
            && (self.offset <= q.offset && (self.offset + self.data.len() as u64) > (q.offset + q.length))
            || self.offset < (q.offset + q.length)
    }
}

pub trait NetworkInterface {
    // TODO: these should probably take Query/Answer <Outgoing> instead
    fn send_answer(&mut self, q: &Query<Incoming>, v: Version, d: &Data);
    fn send_query(&mut self, q: &Query<Incoming>, peer: Host);
}

impl ProtocolRunner {
    pub fn process_query<I: NetworkInterface>(&mut self, n: &mut I, q: Query<Incoming>) {
        // check seen list, if we've already seen, ignore
        if self.seen_queries.iter().find(|x| x.same(&q)).is_some() {
            return;
        }
        // check if we have the data available, if so send back an answer
        if let Some((v, data)) = self.data.lookup(&q) {
            n.send_answer(&q, v, data);
            return;
        }
        // If not, send out queries to our peers
        for p in self.peers.iter().filter(|p| *p != &q.from && *p != &q.tell) {
            n.send_query(&q, p.clone())
        }
        // Add query to seen list
        self.seen_queries.push(q.clone());
        self.pending_queries.push(q)
    }
    pub fn process_answer<I: NetworkInterface>(&mut self, n: &mut I, a: Answer<Incoming>) {
        // Store the answer
        self.data.store(&a);
        // If we've seen a query for this data, send out an answer and mark the query as answered
        let answered: Vec<_> = self
            .seen_queries
            .iter()
            .enumerate()
            .filter(|(_i, q)| a.answers(q))
            .flat_map(|(i, q)| {
                if let Some((v, data)) = self.data.lookup(q) {
                    n.send_answer(&q, v, data);
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        for i in answered.iter().rev() {
            self.seen_queries.remove(*i);
        }
    }
}

#[cfg(test)]
mod tests;
