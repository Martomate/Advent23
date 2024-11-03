use advent23::d21::run;

const IN_1: &str = include_str!("in1.txt");
const IN_2: &str = include_str!("in2.txt");

#[test]
fn part_1_small() {
    assert_eq!(run(IN_1.lines().collect(), 6, false), 16);
}

#[test]
fn part_1_big() {
    assert_eq!(run(IN_2.lines().collect(), 64, false), 3585);
}

#[test]
fn part_2_small_6() {
    assert_eq!(run(IN_1.lines().collect(), 6, true), 16);
}

#[test]
fn part_2_small_10() {
    assert_eq!(run(IN_1.lines().collect(), 10, true), 50);
}

#[test]
fn part_2_small_50() {
    assert_eq!(run(IN_1.lines().collect(), 50, true), 1594);
}

#[test]
fn part_2_small_100() {
    assert_eq!(run(IN_1.lines().collect(), 100, true), 6536);
}

#[test]
fn part_2_small_500() {
    assert_eq!(run(IN_1.lines().collect(), 500, true), 167004);
}

#[test]
fn part_2_small_1000() {
    assert_eq!(run(IN_1.lines().collect(), 1000, true), 668697);
}

#[test]
fn part_2_small_5000() {
    assert_eq!(run(IN_1.lines().collect(), 5000, true), 16733044);
}

#[test]
fn part_2_big() {
    assert_eq!(run(IN_2.lines().collect(), 26501365, true), 597102953699891);
}
