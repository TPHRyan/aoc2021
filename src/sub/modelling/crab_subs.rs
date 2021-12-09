use std::collections::HashMap;

pub fn find_cheapest_alignment_cost(mut subs: Vec<u32>) -> u32 {
    subs.sort();
    let mut subs_at_positions: HashMap<u32, u32> = HashMap::new();
    for position in subs.iter() {
        let new_value = subs_at_positions
            .get(&position)
            .map_or(1, |count| *count + 1);
        subs_at_positions.insert(*position, new_value);
    }
    let unique_position_sum: u32 = subs_at_positions.keys().fold(0, |acc, pos| acc + *pos);
    find_cheapest_alignment_from(&subs, subs_at_positions, unique_position_sum)
}

fn find_cheapest_alignment_from(
    available_positions: &[u32],
    subs_at_positions: HashMap<u32, u32>,
    sum_of_unique_positions: u32,
) -> u32 {
    let i_median = available_positions.len() / 2;
    find_cheapest_alignment_from_index(
        i_median as isize,
        available_positions,
        &subs_at_positions,
        sum_of_unique_positions,
    )
}

fn find_cheapest_alignment_from_index(
    i: isize,
    available_positions: &[u32],
    subs_at_positions: &HashMap<u32, u32>,
    sum_of_unique_positions: u32,
) -> u32 {
    if i < 0 || i >= available_positions.len() as isize {
        return u32::MAX;
    }
    match available_positions.len() {
        0 => u32::MAX,
        1 => find_cost_for_position(
            available_positions[0],
            &subs_at_positions,
            sum_of_unique_positions,
        ),
        2 => find_cost_for_position(
            available_positions[0],
            subs_at_positions,
            sum_of_unique_positions,
        )
        .min(find_cost_for_position(
            available_positions[1],
            subs_at_positions,
            sum_of_unique_positions,
        )),
        _ => {
            let cost_for_i = find_cost_for_position(
                available_positions[i as usize],
                &subs_at_positions,
                sum_of_unique_positions,
            );
            let left_neighbor_cost = if i == 0 {
                u32::MAX
            } else {
                find_cost_for_position(
                    available_positions[(i - 1) as usize],
                    subs_at_positions,
                    sum_of_unique_positions,
                )
            };
            let right_neighbor_cost = if i + 1 >= available_positions.len() as isize {
                u32::MAX
            } else {
                find_cost_for_position(
                    available_positions[(i + 1) as usize],
                    subs_at_positions,
                    sum_of_unique_positions,
                )
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
                    subs_at_positions,
                    sum_of_unique_positions,
                )))
            };
            let find_cheapest_right = || {
                cost_for_i.min(right_neighbor_cost.min(find_cheapest_alignment_from_index(
                    i + 2,
                    right,
                    subs_at_positions,
                    sum_of_unique_positions,
                )))
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

fn find_cost_for_position(
    pos: u32,
    subs_at_positions: &HashMap<u32, u32>,
    sum_of_unique_positions: u32,
) -> u32 {
    subs_at_positions
        .keys()
        .try_fold(0, |acc, other_pos| {
            let new_total = acc + cost_to_move(*other_pos, pos) * subs_at_positions[other_pos];
            if new_total > sum_of_unique_positions {
                None
            } else {
                Some(new_total)
            }
        })
        .unwrap_or(u32::MAX)
}

fn cost_to_move(from: u32, to: u32) -> u32 {
    let difference: i32 = (to as i32) - (from as i32);
    difference.abs() as u32
}
