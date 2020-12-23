use std::collections::HashMap;
use util::res::Result;

struct Cups {
    curr: u64,
    circle: HashMap<u64, u64>,
}

impl Cups {
    fn new(order: &Vec<u64>) -> Self {
        let circle = order.iter().zip(order.iter().cycle().skip(1))
            .map(|(&from, &to)| (from, to)).collect::<HashMap<u64, u64>>();
        Cups{ curr: order[0], circle }
    }

    fn remove_next_3(&mut self) -> [(u64, u64); 3] {
        let mut out = [(0, 0); 3];
        let mut next_val = *self.circle.get(&self.curr).unwrap();
        for i in 0..3 {
            let curr_val = next_val;
            next_val = self.circle.remove(&curr_val).unwrap();
            out[i] = (curr_val, next_val);
        }
        self.circle.entry(self.curr).and_modify(|e| *e = next_val);
        out
    }

    fn do_move(&mut self) {
        let mut next3 = self.remove_next_3();
        let mut dest = (self.curr - 1) as i64;
        while !self.circle.contains_key(&(dest as u64)) {
            dest = if dest <= 1 { self.circle.len() as i64 + 3 } else { dest - 1 };
        }

        self.circle.entry(dest as u64).and_modify(|e| {
            next3[2].1 = *e;
            *e = next3[0].0;
        });
        for (label, next) in &next3 { self.circle.insert(*label, *next); }
        self.curr = *self.circle.get(&self.curr).unwrap();
    }

    fn get_n_after_val(&self, mut val: u64, n: usize) -> Vec<u64> {
        let mut out: Vec<u64> = vec![];
        for _ in 0..n {
            val = *self.circle.get(&val).unwrap();
            out.push(val);
        }
        out
    }
}

fn part1(start: &Vec<u64>) {
    let mut cups = Cups::new(start);
    for _ in 0..100 {
        cups.do_move();
    }

    let order = cups.get_n_after_val(1, start.len() - 1).iter()
        .map(|n| n.to_string()).collect::<String>();
    println!("[Part 1] Order after cup 1: {}", order);
}

fn part2(start: &Vec<u64>) {
    let start_extended = start.iter().cloned().chain(10..=1_000_000).collect::<Vec<u64>>();
    let mut cups = Cups::new(&start_extended);
    for _ in 0..10_000_000 {
        cups.do_move();
    }

    let product: u64 = cups.get_n_after_val(1, 2).iter().product();
    println!("[Part 2] Product of 2 labels after cup 1: {}", product);
}

fn main() -> Result<()> {
    let start: Vec<u64> = vec![1, 2, 3, 4, 8, 7, 5, 9, 6];
    part1(&start);
    part2(&start);
    Ok(())
}