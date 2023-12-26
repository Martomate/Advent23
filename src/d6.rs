fn calc_distance(race_time: u64, charge_time: u64) -> u64 {
    let speed = charge_time;
    let travel_time = race_time - charge_time;
    travel_time * speed
}

fn calc_good_choices(race_time: u64, record_distance: u64) -> u64 {
    let mut good_choices = 0;
    for charge_time in 1..race_time {
        let distance = calc_distance(race_time, charge_time);
        if distance > record_distance {
            good_choices += 1;
        }
    }
    good_choices
}

struct Input {
    race_times: Vec<u64>,
    record_distances: Vec<u64>,
}

fn parse_line(prefix: &str, line: &str, use_kerning: bool) -> Vec<u64> {
    if use_kerning {
        vec![line
            .strip_prefix(prefix)
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<u64>()
            .unwrap()]
    } else {
        line.strip_prefix(prefix)
            .unwrap()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect()
    }
}

fn parse_input(lines: Vec<&str>, use_kerning: bool) -> Input {
    let race_times: Vec<u64> = parse_line("Time:", lines[0], use_kerning);
    let record_distances: Vec<u64> = parse_line("Distance:", lines[1], use_kerning);

    Input {
        race_times,
        record_distances,
    }
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let Input {
        race_times,
        record_distances,
    } = parse_input(lines, !part1);

    assert_eq!(race_times.len(), record_distances.len());

    let mut res = 1;
    for race_id in 0..race_times.len() {
        res *= calc_good_choices(race_times[race_id], record_distances[race_id]);
    }
    res
}
