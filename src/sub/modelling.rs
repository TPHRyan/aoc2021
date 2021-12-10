mod crab_subs;
mod lanternfish;

pub use crate::sub::modelling::crab_subs::FuelModel;
pub use lanternfish::{Lanternfish, LanternfishPool};

pub fn simulate_lanternfish(initial_fish: Vec<Lanternfish>, for_days: u32) -> usize {
    let mut pool = LanternfishPool::from(initial_fish);
    pool.simulate_days(for_days);
    pool.num_fish()
}

pub fn calculate_cheapest_alignment_fuel(crab_subs: Vec<u32>, model: FuelModel) -> u32 {
    crab_subs::find_cheapest_alignment_cost(crab_subs, model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_cheapest_alignment_fuel_linear_is_correct_for_example_data() {
        let subs = get_example_subs();
        let cheapest_fuel = calculate_cheapest_alignment_fuel(subs, FuelModel::LINEAR);
        assert_eq!(37, cheapest_fuel);
    }

    #[test]
    fn calculate_cheapest_alignment_fuel_triangular_is_correct_for_example_data() {
        let subs = get_example_subs();
        let cheapest_fuel = calculate_cheapest_alignment_fuel(subs, FuelModel::TRIANGULAR);
        assert_eq!(168, cheapest_fuel);
    }

    fn get_example_subs() -> Vec<u32> {
        vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    }

    #[test]
    fn simulate_lanternfish_is_correct_for_example_data() {
        let fish_count = simulate_lanternfish(get_example_fish(), 80);
        assert_eq!(5934, fish_count);
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
