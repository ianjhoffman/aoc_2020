use std::collections::HashMap;
use util::res::Result;

struct Cups {
    curr: usize,
    circle: Vec<usize>,
}

impl Cups {
    fn new(order: &Vec<usize>) -> Self {
        let mut circle = vec![0; order.len() + 1];
        for (&curr, &next) in order.iter().zip(order.iter().cycle().skip(1)) {
            circle[curr] = next;
        }
        Cups{ curr: order[0], circle }
    }

    fn next_3(&mut self) -> [(usize, usize); 3] {
        let mut out = [(0, 0); 3];
        let mut next_val = self.circle[self.curr];
        for i in 0..3 {
            let curr_val = next_val;
            next_val = self.circle[curr_val];
            out[i] = (curr_val, next_val);
        }
        out
    }

    fn do_move(&mut self) {
        let next3 = self.next_3();
        let mut dest = self.curr;
        loop {
            dest = if dest == 1 { self.circle.len() - 1 } else { dest - 1 };
            if next3[0].0 != dest && next3[1].0 != dest && next3[2].0 != dest { break }
        }

        self.circle[self.curr] = next3[2].1;
        self.circle[next3[2].0] = self.circle[dest];
        self.circle[dest] = next3[0].0;
        self.curr = self.circle[self.curr];
    }

    fn get_n_after_val(&self, mut val: usize, n: usize) -> Vec<usize> {
        let mut out: Vec<usize> = vec![];
        for _ in 0..n {
            val = self.circle[val];
            out.push(val);
        }
        out
    }
}

fn part1(start: &Vec<usize>) {
    let mut cups = Cups::new(start);
    for _ in 0..100 {
        cups.do_move();
    }

    let order = cups.get_n_after_val(1, start.len() - 1).iter()
        .map(|n| n.to_string()).collect::<String>();
    println!("[Part 1] Order after cup 1: {}", order);
}

fn part2(start: &Vec<usize>) {
    let start_extended = start.iter().cloned().chain(10..=1_000_000).collect::<Vec<usize>>();
    let mut cups = Cups::new(&start_extended);
    for _ in 0..10_000_000 {
        cups.do_move();
    }

    let product: usize = cups.get_n_after_val(1, 2).iter().product();
    println!("[Part 2] Product of 2 labels after cup 1: {}", product);
}

fn main() -> Result<()> {
    let start: Vec<usize> = vec![1, 2, 3, 4, 8, 7, 5, 9, 6];
    part1(&start);
    part2(&start);
    Ok(())
}