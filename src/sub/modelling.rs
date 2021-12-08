pub use lanternfish::{Lanternfish, LanternfishPool};

mod lanternfish;

pub fn simulate_lanternfish(initial_fish: Vec<Lanternfish>, for_days: u32) -> usize {
    let mut pool = LanternfishPool::from(initial_fish);
    pool.simulate_days(for_days);
    pool.num_fish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_lanternfish_is_correct_for_example_data() {
        let fish_count = simulate_lanternfish(get_example_fish(), 80);
        println!("{}", fish_count);
    }

    fn get_example_fish() -> Vec<Lanternfish> {
        vec![
            Lanternfish::from(3),
            Lanternfish::from(4),
            Lanternfish::from(3),
            Lanternfish::from(1),
            Lanternfish::from(2),
        ]
    }
}
