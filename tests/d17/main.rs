use advent23::d17::run;

#[test]
fn part_1_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), true), 102);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), true), 859);
}

#[test]
fn part_2_small_1() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), false), 94);
}

#[test]
fn part_2_small_2() {
    let input = include_str!("in1b.txt");
    assert_eq!(run(input.lines().collect(), false), 71);
}

#[test]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 1027);
}
