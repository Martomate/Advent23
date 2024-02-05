use advent23::d19::run;

#[test]
fn part_1_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), true), 19114);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), true), 367602);
}

#[test]
fn part_2_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), false), 167409079868000);
}

#[test]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 125317461667458);
}
