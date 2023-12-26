use advent23::d6::run;

#[test]
fn part_1_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), true), 288);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), true), 505494);
}

#[test]
fn part_2_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), false), 71503);
}

#[test]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 23632299);
}
