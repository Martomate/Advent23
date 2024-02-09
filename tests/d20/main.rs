use advent23::d20::run;

#[test]
fn part_1_small_1() {
    let input = include_str!("in1_1.txt");
    assert_eq!(run(input.lines().collect(), true), 32000000);
}

#[test]
fn part_1_small_2() {
    let input = include_str!("in1_2.txt");
    assert_eq!(run(input.lines().collect(), true), 11687500);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), true), 832957356);
}

#[test]
#[ignore]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 0);
}
