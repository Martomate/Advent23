use advent23::d3::run;

#[test]
fn part_1_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), true), 4361);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), true), 554003);
}

#[test]
fn part_2_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input.lines().collect(), false), 467835);
}

#[test]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 87263515);
}
