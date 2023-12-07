fn extract_value(s: &str, allow_words: bool) -> u32 {
    if allow_words {
        let mut lo_idx = s.len();
        let mut lo = 0;
        let mut hi_idx = -1;
        let mut hi = 0;

        let numbers: Vec<_> = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .into_iter()
        .enumerate()
        .map(|(i, s)| (i + 1, s.to_string()))
        .chain((0..=9).map(|n| (n, format!("{}", n))))
        .collect();

        for (n, w) in numbers {
            if let Some(idx) = s.find(&w) {
                if idx < lo_idx {
                    lo_idx = idx;
                    lo = n;
                }
            }
            if let Some(idx) = s.rfind(&w) {
                if idx as i32 > hi_idx {
                    hi_idx = idx as i32;
                    hi = n;
                }
            }
        }

        (lo * 10 + hi) as u32
    } else {
        let mut v = 0;
        v += (s.chars().find(|c| c.is_numeric()).unwrap() as u32) - ('0' as u32);
        v *= 10;
        v += (s.chars().rfind(|c| c.is_numeric()).unwrap() as u32) - ('0' as u32);
        v
    }
}

pub fn run(lines: Vec<&str>, allow_words: bool) -> u32 {
    lines
        .into_iter()
        .map(|s| extract_value(s, allow_words))
        .sum()
}
