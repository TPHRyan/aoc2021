use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum FuelModel {
    LINEAR,
    TRIANGULAR,
}

pub struct Metadata {
    pub fuel_model: FuelModel,
    pub subs_at_positions: HashMap<u32, u32>,
}

pub fn generate_metadata_for_subs(sorted_subs: &Vec<u32>, fuel_model: FuelModel) -> Metadata {
    let mut subs_at_positions: HashMap<u32, u32> = HashMap::new();
    for &position in sorted_subs.iter() {
        let new_value = subs_at_positions
            .get(&position)
            .map_or(1, |count| *count + 1);
        subs_at_positions.insert(position, new_value);
    }
    Metadata {
        fuel_model,
        subs_at_positions,
    }
}
