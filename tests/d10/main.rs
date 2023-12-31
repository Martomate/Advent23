use advent23::d10::run;

#[test]
fn part_1_small() {
    let input = include_str!("in1a.txt");
    assert_eq!(run(input.lines().collect(), true), 8);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), true), 7030);
}

#[test]
fn part_2_small_1() {
    let input = include_str!("in1b1.txt");
    assert_eq!(run(input.lines().collect(), false), 4);
}

#[test]
fn part_2_small_2() {
    let input = include_str!("in1b2.txt");
    assert_eq!(run(input.lines().collect(), false), 8);
}

#[test]
fn part_2_small_3() {
    let input = include_str!("in1b3.txt");
    assert_eq!(run(input.lines().collect(), false), 10);
}

#[test]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 285);
}
