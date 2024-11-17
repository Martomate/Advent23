use advent23::d22::run;

const IN_1: &str = include_str!("in1.txt");
const IN_2: &str = include_str!("in2.txt");

#[test]
fn part_1_small() {
    assert_eq!(run(IN_1.lines().collect(), false), 5);
}

#[test]
fn part_1_big() {
    assert_eq!(run(IN_2.lines().collect(), false), 395);
}

#[test]
fn part_2_small() {
    assert_eq!(run(IN_1.lines().collect(), true), 7);
}

#[test]
fn part_2_big() {
    let res = run(IN_2.lines().collect(), true);
    assert_eq!(res, 64714);
}
