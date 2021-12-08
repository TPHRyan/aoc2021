use super::Lanternfish;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct LanternfishPool {
    data: BinaryHeap<Reverse<Lanternfish>>,
    current_tick: u32,
}

impl LanternfishPool {
    pub fn from(initial_fish: Vec<Lanternfish>) -> LanternfishPool {
        let mut min_heap = BinaryHeap::new();
        for fish in initial_fish {
            min_heap.push(Reverse(fish));
        }
        LanternfishPool {
            data: min_heap,
            current_tick: 0,
        }
    }

    pub fn num_fish(&self) -> usize {
        self.data.len()
    }

    pub fn simulate_days(&mut self, days: u32) {
        let final_tick = self.current_tick + days;
        while self.current_tick < final_tick {
            self.process_tick(self.current_tick);
            self.current_tick += 1;
        }
    }

    fn process_tick(&mut self, t: u32) {
        loop {
            if let Some(Reverse(next_fish)) = self.data.peek() {
                if next_fish.t_next_reproduction > t {
                    break;
                }
                if let Some(Reverse(mut current_fish)) = self.data.pop() {
                    if let Some(mut new_fish) = current_fish.tick(t) {
                        new_fish.reset(t);
                        self.data.push(Reverse(new_fish));
                    }
                    self.data.push(Reverse(current_fish));
                }
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_count_initially_zero() {
        let pool = LanternfishPool::from(vec![]);
        assert_eq!(0, pool.num_fish());
    }

    #[test]
    fn pool_count_equal_to_initial_fish_count() {
        let pool = LanternfishPool::from(vec![
            Lanternfish::new(),
            Lanternfish::new(),
            Lanternfish::new(),
            Lanternfish::new(),
            Lanternfish::new(),
            Lanternfish::new(),
        ]);
        assert_eq!(6, pool.num_fish());
    }

    #[test]
    fn fish_on_zero_reproduce() {
        let mut pool = LanternfishPool::from(vec![Lanternfish::from(0)]);
        assert_eq!(1, pool.num_fish());
        pool.simulate_days(1);
        assert_eq!(2, pool.num_fish());
    }

    #[test]
    fn pool_can_grow_many_times_per_tick() {
        let mut pool = LanternfishPool::from(vec![
            Lanternfish::from(0),
            Lanternfish::from(0),
            Lanternfish::from(0),
        ]);
        assert_eq!(3, pool.num_fish());
        pool.simulate_days(1);
        assert_eq!(6, pool.num_fish());
    }
}
