use advent23::d9::run;

#[test]
fn part_1_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), true), 114);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), true), 1980437560);
}

#[test]
fn part_2_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), false), 2);
}

#[test]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 977);
}
