#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }

    fn from_pairs(numbers: &[u64]) -> Vec<Range> {
        assert!(numbers.len() % 2 == 0);

        let mut ranges = Vec::new();
        for arr in numbers.chunks_exact(2) {
            let s = arr[0];
            let l = arr[1];
            ranges.push(Range::new(s, l));
        }
        ranges
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Remap {
    dst: u64,
    src: u64,
    length: u64,
}

impl Remap {
    fn new(dst: u64, src: u64, length: u64) -> Self {
        Self { dst, src, length }
    }

    fn apply_range(&self, values: Range) -> Range {
        assert!(values.start >= self.src);

        let start = values.start - self.src + self.dst;
        assert!(start + values.length <= self.dst + self.length);

        Range::new(start, values.length)
    }

    fn split_input_map(&self, values: Range) -> (Range, Range, Range) {
        let vl = values.start;
        let vr = values.start + values.length;

        let sl = self.src;
        let sr = self.src + self.length;

        let l = vl.max(sl).min(vr);
        let r = vr.min(sr).max(vl);

        (
            Range::new(vl, if l > vl { l - vl } else { 0 }),
            Range::new(l, if r > l { r - l } else { 0 }),
            Range::new(r, if vr > r { vr - r } else { 0 }),
        )
    }
}

impl From<&str> for Remap {
    fn from(line: &str) -> Self {
        let mut parts = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .take(3);

        let dst_start = parts.next().unwrap();
        let src_start = parts.next().unwrap();
        let length = parts.next().unwrap();

        Self::new(dst_start, src_start, length)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    remaps: Vec<Remap>,
}

impl Mapping {
    fn apply_ranges(&self, values: &[Range]) -> Vec<Range> {
        let mut res = Vec::new();

        let mut current: Vec<Range> = values.to_vec();
        let mut left: Vec<Range> = Vec::new();

        for remap in self.remaps.iter() {
            for vs in current {
                let (l, m, r) = remap.split_input_map(vs);

                if l.length > 0 {
                    left.push(l);
                }
                if r.length > 0 {
                    left.push(r);
                }
                if m.length > 0 {
                    let value = remap.apply_range(m);
                    res.push(value);
                }
            }
            current = left;
            left = Vec::new();
        }

        res.extend(current);

        merge_ranges(res)
    }
}

fn merge_ranges(mut ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return ranges;
    };

    ranges.sort_by(|r1, r2| {
        r1.start
            .cmp(&r2.start)
            .then(r1.length.cmp(&r2.length).reverse())
    });
    let mut res = Vec::new();
    let mut current: Range = ranges[0];
    for r in ranges {
        let r_start = r.start;
        let r_end = r.start + r.length;
        let c_start = current.start;
        let c_end = current.start + current.length;

        if r_start <= c_end {
            if r_end > c_end {
                current.length = r_end - c_start;
            }
        } else {
            res.push(current);
            current = r;
        }
    }
    res.push(current);
    res
}

impl From<&[&str]> for Mapping {
    fn from(lines: &[&str]) -> Self {
        Self {
            remaps: lines.iter().map(|&line| Remap::from(line)).collect(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    seeds: Vec<Range>,
    mappings: Vec<Mapping>,
}

impl Almanac {
    fn locations(&self) -> Vec<Range> {
        let mut current = self.seeds.clone();
        for mapping in self.mappings.iter() {
            current = mapping.apply_ranges(current.as_ref());
        }
        current
    }
}

fn parse_almanac(lines: &[&str], seeds_as_ranges: bool) -> Almanac {
    let mut almanac = Almanac {
        seeds: Vec::new(),
        mappings: Vec::new(),
    };
    let mut current_part: Option<&str> = None;
    let mut waiting_lines: Vec<&str> = Vec::new();
    for line in lines {
        if line.is_empty() {
            if let Some(_name) = current_part {
                almanac.mappings.push(Mapping::from(waiting_lines.as_ref()));
            }
            current_part = None;
            waiting_lines.clear();
        } else if current_part.is_none() {
            if let Some(line) = line.strip_prefix("seeds: ") {
                let numbers: Vec<u64> = line
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();

                if seeds_as_ranges {
                    almanac.seeds.extend(Range::from_pairs(numbers.as_ref()))
                } else {
                    almanac
                        .seeds
                        .extend(numbers.into_iter().map(|n| Range::new(n, 1)));
                }
            } else {
                current_part = Some(line);
            }
        } else {
            waiting_lines.push(line);
        }
    }
    if let Some(_name) = current_part {
        almanac.mappings.push(Mapping::from(waiting_lines.as_ref()));
    }
    almanac
}

pub fn run(lines: Vec<&str>, part1: bool) -> u64 {
    let almanac = parse_almanac(lines.as_ref(), !part1);
    almanac
        .locations()
        .into_iter()
        .flat_map(|locs| locs.start..(locs.start + locs.length))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::d5::*;

    #[test]
    fn remap_from_line() {
        assert_eq!(
            Remap::from("37 52 2"),
            Remap {
                dst: 37,
                src: 52,
                length: 2
            }
        )
    }

    #[test]
    fn almanac_from_lines() {
        assert_eq!(
            parse_almanac(
                [
                    "seeds: 1 2",
                    "",
                    "seed-to-soil map:",
                    "50 98 2",
                    "52 50 48",
                    "",
                    "soil-to-fertilizer map:",
                    "0 15 37"
                ]
                .as_ref(),
                false
            ),
            Almanac {
                seeds: vec![Range::new(1, 1), Range::new(2, 1)],
                mappings: vec![
                    Mapping {
                        remaps: vec![Remap::new(50, 98, 2), Remap::new(52, 50, 48)]
                    },
                    Mapping {
                        remaps: vec![Remap::new(0, 15, 37)]
                    },
                ]
            }
        )
    }

    #[test]
    fn almanac_from_lines_with_seeds_as_ranges() {
        assert_eq!(
            parse_almanac(["seeds: 1 2 13 14"].as_ref(), true),
            Almanac {
                seeds: vec![Range::new(1, 2), Range::new(13, 14)],
                mappings: vec![]
            }
        )
    }

    #[test]
    fn range_mapping() {
        let m = Mapping {
            remaps: vec![Remap {
                src: 5,
                dst: 15,
                length: 3,
            }],
        };

        assert_eq!(
            m.apply_ranges([Range::new(5, 2)].as_ref()),
            vec![Range::new(15, 2)]
        );
        assert_eq!(
            m.apply_ranges([Range::new(5, 4)].as_ref()),
            vec![Range::new(15, 3), Range::new(8, 1)]
        );
        assert_eq!(
            m.apply_ranges([Range::new(4, 4)].as_ref()),
            vec![Range::new(15, 3), Range::new(4, 1)]
        );
    }

    #[test]
    fn split_ranges() {
        let r = Remap {
            src: 5,
            dst: 15,
            length: 3,
        };
        
        assert_eq!(r.split_input_map(Range::new(5, 3)), (Range::new(5, 0), Range::new(5, 3), Range::new(8, 0)));
        assert_eq!(r.split_input_map(Range::new(2, 6)), (Range::new(2, 3), Range::new(5, 3), Range::new(8, 0)));
        assert_eq!(r.split_input_map(Range::new(5, 6)), (Range::new(5, 0), Range::new(5, 3), Range::new(8, 3)));
        
        assert_eq!(r.split_input_map(Range::new(1, 3)), (Range::new(1, 3), Range::new(4, 0), Range::new(4, 0)));
        assert_eq!(r.split_input_map(Range::new(10, 3)), (Range::new(10, 0), Range::new(10, 0), Range::new(10, 3)));
        
        assert_eq!(r.split_input_map(Range::new(7, 3)), (Range::new(7, 0), Range::new(7, 1), Range::new(8, 2)));
        assert_eq!(r.split_input_map(Range::new(3, 3)), (Range::new(3, 2), Range::new(5, 1), Range::new(6, 0)));
    }
}
