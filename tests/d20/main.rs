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
fn part_2_big() {
    let input = include_str!("in2.txt");
    assert_eq!(run(input.lines().collect(), false), 240162699605221);
}

// Idea: could you express this problem as "topological sorting"?
//  When a signal is emitted from a module the recipients are added to the queue in order, which means that one is < the next one.
//  A conjunction acts on previously recieved signals, which implies that those signals are < the conjunction and all future signals from the same emitter.
//  One could potentially construct these kinds of constraints to model the time dependencies between events.
