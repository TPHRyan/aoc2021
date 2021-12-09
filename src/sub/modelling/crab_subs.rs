mod metadata;

pub use metadata::FuelModel;
use metadata::Metadata;

pub fn find_cheapest_alignment_cost(mut subs: Vec<u32>, model: FuelModel) -> u32 {
    subs.sort();
    let metadata = metadata::generate_metadata_for_subs(&subs, model);
    find_cheapest_alignment_from(&subs, metadata)
}

fn find_cheapest_alignment_from(existing_positions: &[u32], metadata: Metadata) -> u32 {
    let i_median = existing_positions.len() / 2;
    let available_positions: Vec<u32> =
        (0..existing_positions[existing_positions.len() - 1]).collect();
    find_cheapest_alignment_from_index(i_median as isize, &available_positions, &metadata)
}

fn find_cheapest_alignment_from_index(
    i: isize,
    available_positions: &[u32],
    metadata: &Metadata,
) -> u32 {
    if i < 0 || i >= available_positions.len() as isize {
        return u32::MAX;
    }
    match available_positions.len() {
        0 => u32::MAX,
        1 => find_cost_for_position(available_positions[0], metadata),
        2 => find_cost_for_position(available_positions[0], metadata)
            .min(find_cost_for_position(available_positions[1], metadata)),
        _ => {
            let cost_for_i = find_cost_for_position(available_positions[i as usize], metadata);
            let left_neighbor_cost = if i == 0 {
                u32::MAX
            } else {
                find_cost_for_position(available_positions[(i - 1) as usize], metadata)
            };
            let right_neighbor_cost = if i + 1 >= available_positions.len() as isize {
                u32::MAX
            } else {
                find_cost_for_position(available_positions[(i + 1) as usize], metadata)
            };
            let left = &available_positions[..(i - 1).max(0) as usize];
            let right = &available_positions
                [(i + 2).min((available_positions.len() - 1) as isize) as usize..];
            if cost_for_i < left_neighbor_cost && cost_for_i < right_neighbor_cost {
                return cost_for_i;
            }
            let find_cheapest_left = || {
                cost_for_i.min(left_neighbor_cost.min(find_cheapest_alignment_from_index(
                    i - 2,
                    left,
                    metadata,
                )))
            };
            let find_cheapest_right = || {
                cost_for_i.min(
                    right_neighbor_cost.min(find_cheapest_alignment_from_index(0, right, metadata)),
                )
            };
            if cost_for_i < right_neighbor_cost {
                find_cheapest_left()
            } else if cost_for_i < left_neighbor_cost {
                find_cheapest_right()
            } else {
                find_cheapest_left().min(find_cheapest_right())
            }
        }
    }
}

fn find_cost_for_position(pos: u32, metadata: &Metadata) -> u32 {
    metadata.subs_at_positions.keys().fold(0, |acc, other_pos| {
        acc + cost_to_move(*other_pos, pos, metadata) * metadata.subs_at_positions[other_pos]
    })
}

fn cost_to_move(from: u32, to: u32, metadata: &Metadata) -> u32 {
    match metadata.fuel_model {
        FuelModel::LINEAR => cost_to_move_linear(from, to),
        FuelModel::TRIANGULAR => cost_to_move_triangular(from, to),
    }
}

fn cost_to_move_linear(from: u32, to: u32) -> u32 {
    let difference: i32 = (to as i32) - (from as i32);
    difference.abs() as u32
}

fn cost_to_move_triangular(from: u32, to: u32) -> u32 {
    let linear_cost = cost_to_move_linear(from, to);
    linear_cost * (linear_cost + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cost_to_move_linear_works_on_test_data() {
        let test_distances: Vec<(i32, i32)> = vec![(0, 2), (3, 6), (1, 5), (9, 14)];
        let test_answers = vec![2, 3, 4, 5];
        for ((from, to), answer) in test_distances.iter().zip(test_answers) {
            assert_eq!(answer, cost_to_move_linear(*from as u32, *to as u32));
        }
    }

    #[test]
    fn cost_to_move_triangular_works_on_test_data() {
        let test_distances: Vec<(i32, i32)> = vec![(0, 2), (3, 6), (1, 5), (9, 14)];
        let test_answers = vec![3, 6, 10, 15];
        for ((from, to), answer) in test_distances.iter().zip(test_answers) {
            assert_eq!(answer, cost_to_move_triangular(*from as u32, *to as u32));
        }
    }
}
