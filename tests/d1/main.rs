use advent23::d1::run;


#[test]
fn part_1_small() {
    let input = include_str!("in1a.txt");
    assert_eq!(run(input.lines().collect(), false), 142);
}

#[test]
fn part_1_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 55386);
}

#[test]
fn part_2_small() {
    let input = include_str!("in1b.txt");
    assert_eq!(run(input.lines().collect(), true), 281);
}

#[test]
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), true), 54824);
}
