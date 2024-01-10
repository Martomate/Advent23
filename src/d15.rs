pub fn run(line: &str, part1: bool) -> u64 {
    let steps: Vec<&str> = line.split(',').collect();

    if part1 {
        steps.iter().map(|s| calc_hash(s) as u64).sum()
    } else {
        let mut hashmap = HashMap::new();

        for step in steps {
            if let Some(label) = step.strip_suffix('-') {
                hashmap.remove(label);
            } else if let Some((label, focus)) = step.split_once('=') {
                hashmap.add(Lens {
                    label,
                    focal_length: focus.parse().unwrap(),
                });
            }
        }

        hashmap.focusing_power()
    }
}

fn calc_hash(s: &str) -> u8 {
    let mut value: u8 = 0;
    for ch in s.chars() {
        value = value.wrapping_add(ch as u8).wrapping_mul(17);
    }
    value
}

struct Lens<'a> {
    label: &'a str,
    focal_length: u64,
}

struct HashMap<'a> {
    boxes: [Vec<Lens<'a>>; 256],
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        Self {
            boxes: [0; 256].map(|_| Vec::new()),
        }
    }

    fn add(&mut self, lens: Lens<'a>) {
        let box_idx = calc_hash(lens.label);
        let slots = &mut self.boxes[box_idx as usize];

        if let Some(slot_idx) = slots.iter().position(|l| l.label == lens.label) {
            slots[slot_idx] = lens;
        } else {
            slots.push(lens);
        }
    }

    fn remove(&mut self, label: &'a str) {
        let box_idx = calc_hash(label);
        let slots = &mut self.boxes[box_idx as usize];

        if let Some(slot_idx) = slots.iter().position(|l| l.label == label) {
            slots.remove(slot_idx);
        }
    }

    fn focusing_power(&self) -> u64 {
        let mut power = 0;
        for (box_idx, b) in self.boxes.iter().enumerate() {
            for (slot_idx, l) in b.iter().enumerate() {
                power += (box_idx + 1) as u64 * (slot_idx + 1) as u64 * l.focal_length;
            }
        }
        power
    }
}
