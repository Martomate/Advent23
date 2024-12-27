use advent23::d24::run;

const IN_1: &str = include_str!("in1.txt");
const IN_2: &str = include_str!("in2.txt");

#[test]
fn part_1_small() {
    assert_eq!(run(IN_1.lines().collect(), 7, 27, false), 2);
}

#[test]
fn part_1_big() {
    assert_eq!(run(IN_2.lines().collect(), 200000000000000, 400000000000000, false), 25261);
}

#[test]
fn part_2_small() {
    assert_eq!(run(IN_1.lines().collect(), 7, 27, true), 47);
}

#[test]
fn part_2_big() {
    assert_eq!(run(IN_2.lines().collect(), 200000000000000, 400000000000000, true), 549873212220117);
}
