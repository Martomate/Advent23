use advent23::d25::run;

const IN_1: &str = include_str!("in1.txt");
const IN_2: &str = include_str!("in2.txt");

#[test]
fn part_1_small() {
    assert_eq!(run(IN_1.lines().collect()), 54);
}

#[test]
fn part_1_big() {
    assert_eq!(run(IN_2.lines().collect()), 518391);
}
