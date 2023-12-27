fn predict(numbers: &[i64], forward: bool) -> i64 {
    if numbers.iter().all(|&n| n == 0) {
        return 0;
    }

    let mut diffs = Vec::with_capacity(numbers.len() - 1);
    for i in 0..numbers.len() - 1 {
        diffs.push(numbers[i + 1] - numbers[i]);
    }

    let p = predict(&diffs, forward);
    if forward {
        numbers[numbers.len() - 1] + p
    } else {
        numbers[0] - p
    }
}

pub fn run(lines: Vec<&str>, part1: bool) -> i64 {
    let mut res = 0;
    for line in lines {
        let numbers: Vec<i64> = line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
        let p = predict(&numbers, part1);
        res += p;
    }
    res
}
