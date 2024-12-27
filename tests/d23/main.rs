use advent23::d23::run;

const IN_1: &str = include_str!("in1.txt");
const IN_2: &str = include_str!("in2.txt");

#[test]
fn part_1_small() {
    assert_eq!(run(IN_1.lines().collect(), false), 94);
}

#[test]
fn part_1_big() {
    assert_eq!(run(IN_2.lines().collect(), false), 2086);
}

#[test]
fn part_2_small() {
    assert_eq!(run(IN_1.lines().collect(), true), 154);
}

#[test]
fn part_2_big() {
    // this runs in about 150 seconds (2.5 min)
    assert_eq!(run(IN_2.lines().collect(), true), 6526);
}
