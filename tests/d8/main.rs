use advent23::d8::run;

#[test]
fn part_1_small_1() {
    let input = include_str!("in1a1.txt");
    assert_eq!(run(input.lines().collect(), true), 2);
}

#[test]
fn part_1_small_2() {
    let input = include_str!("in1a2.txt");
    assert_eq!(run(input.lines().collect(), true), 6);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), true), 20659);
}

#[test]
fn part_2_small() {
    let input = include_str!("in1b.txt");
    assert_eq!(run(input.lines().collect(), false), 6);
}

#[test]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 15690466351717);
}
