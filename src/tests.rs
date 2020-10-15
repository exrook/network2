use super::Data;
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_data_append() {
    let data = [
        Data(vec![]),
        Data(vec![(0, vec![2, 2, 2, 2])]),
        Data(vec![(5, vec![2, 2, 2, 2])]),
        Data(vec![(0, vec![2, 2, 2, 2]), (6, vec![2, 2, 2, 2])]),
        Data(vec![(5, vec![2, 2, 2, 2]), (11, vec![2, 2, 2, 2])]),
        Data(vec![
            (5, vec![2, 2, 2, 2]),
            (11, vec![2, 2, 2, 2]),
            (16, vec![2, 2, 2, 2]),
        ]),
    ];

    let input = [
        (0, vec![1; 5]),
        (1, vec![1; 5]),
        (5, vec![1; 5]),
        (0, vec![1; 10]),
        (1, vec![1; 10]),
        (5, vec![1; 10]),
        (0, vec![1; 15]),
        (1, vec![1; 15]),
        (5, vec![1; 15]),
    ];

    let expected = [
        [
            Data(vec![(0, vec![1, 1, 1, 1, 1])]),
            Data(vec![(1, vec![1, 1, 1, 1, 1])]),
            Data(vec![(5, vec![1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
        ],
        [
            Data(vec![(0, vec![1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![2, 1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![2, 2, 2, 2]), (5, vec![1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![
                (0, vec![2, 2, 2, 2]),
                (5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            ]),
            Data(vec![(0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(
                0,
                vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            )]),
            Data(vec![
                (0, vec![2, 2, 2, 2]),
                (5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            ]),
        ],
        [
            Data(vec![(0, vec![1, 1, 1, 1, 1, 2, 2, 2, 2])]),
            Data(vec![(1, vec![1, 1, 1, 1, 1, 2, 2, 2])]),
            Data(vec![(5, vec![1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
        ],
        [
            Data(vec![(0, vec![1, 1, 1, 1, 1]), (6, vec![2, 2, 2, 2])]),
            Data(vec![(0, vec![2, 1, 1, 1, 1, 1, 2, 2, 2, 2])]),
            Data(vec![(0, vec![2, 2, 2, 2]), (5, vec![1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![
                (0, vec![2, 2, 2, 2]),
                (5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            ]),
            Data(vec![(0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(
                0,
                vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            )]),
            Data(vec![
                (0, vec![2, 2, 2, 2]),
                (5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            ]),
        ],
        [
            Data(vec![
                (0, vec![1, 1, 1, 1, 1, 2, 2, 2, 2]),
                (11, vec![2, 2, 2, 2]),
            ]),
            Data(vec![
                (1, vec![1, 1, 1, 1, 1, 2, 2, 2]),
                (11, vec![2, 2, 2, 2]),
            ]),
            Data(vec![(5, vec![1, 1, 1, 1, 1]), (11, vec![2, 2, 2, 2])]),
            Data(vec![
                (0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
                (11, vec![2, 2, 2, 2]),
            ]),
            Data(vec![(1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2])]),
            Data(vec![(5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
            Data(vec![(5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
        ],
        [
            Data(vec![
                (0, vec![1, 1, 1, 1, 1, 2, 2, 2, 2]),
                (11, vec![2, 2, 2, 2]),
                (16, vec![2, 2, 2, 2]),
            ]),
            Data(vec![
                (1, vec![1, 1, 1, 1, 1, 2, 2, 2]),
                (11, vec![2, 2, 2, 2]),
                (16, vec![2, 2, 2, 2]),
            ]),
            Data(vec![
                (5, vec![1, 1, 1, 1, 1]),
                (11, vec![2, 2, 2, 2]),
                (16, vec![2, 2, 2, 2]),
            ]),
            Data(vec![
                (0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
                (11, vec![2, 2, 2, 2]),
                (16, vec![2, 2, 2, 2]),
            ]),
            Data(vec![
                (1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2]),
                (16, vec![2, 2, 2, 2]),
            ]),
            Data(vec![
                (5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
                (16, vec![2, 2, 2, 2]),
            ]),
            Data(vec![
                (0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
                (16, vec![2, 2, 2, 2]),
            ]),
            Data(vec![(
                1,
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2],
            )]),
            Data(vec![(5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])]),
        ],
    ];

    println!("let expected = [");
    for (di, d) in data.iter().enumerate() {
        println!("    [");
        for (ii, i) in input.iter().enumerate() {
            let mut d = (*d).clone();
            d.append(i.0, &i.1);
            assert_eq!(expected[di][ii], d);
            print!("        Data(vec![");
            for (xi, x) in d.0 {
                print!("({}, vec!{:?}),", xi, x);
            }
            println!("]),");
        }
        println!("    ],");
    }
    println!("];");
}