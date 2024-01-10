use advent23::d15::run;

#[test]
fn part_1_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input, true), 1320);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input, true), 510388);
}

#[test]
fn part_2_small() {
    let input = include_str!("in1.txt");
    assert_eq!(run(input, false), 145);
}

#[test]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input, false), 291774);
}
