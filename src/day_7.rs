use crate::input_parser::parse_input_at;

pub fn day_7_1() -> usize
{
    let mut initial_positions = parse_positions_from_input("src/inputs/day_7_input.txt");
    let final_position = calculate_optimal_position(&mut initial_positions);

    return calculate_total_fuel_consumption(&initial_positions, final_position);
}

pub fn day_7_2() -> usize
{
    let mut initial_positions = parse_positions_from_input("src/inputs/day_7_input.txt");
    let final_position = calculate_optimal_position_2(&mut initial_positions);

    return calculate_total_fuel_consumption_2(&initial_positions, final_position);
}

fn parse_positions_from_input(input_path: &str) -> Vec<i32>
{
    let input = parse_input_at(input_path)
                    .unwrap_or("".to_string());

    let mut positions = Vec::new();
    for entry in input.split(",")
    {
        positions.push( entry.parse::<i32>().unwrap_or(0) );
    }
    return positions;
}

// Basically a median
fn calculate_optimal_position(initial_positions: &mut Vec<i32>) -> i32
{
    initial_positions.sort_unstable();

    if initial_positions.len() % 2 == 0
    {
        let i = ( initial_positions.len() as f32 * 0.5 ).floor() as usize;
        let j = ( initial_positions.len() as f32 * 0.5 ).ceil()  as usize;

        let a = initial_positions[i];
        let b = initial_positions[j];

        return (a+b) / 2;
    }

    let median_idx = initial_positions.len() / 2;
    return initial_positions[median_idx];
}

fn calculate_optimal_position_2(initial_positions: &mut Vec<i32>) -> i32
{
    if initial_positions.is_empty() { return 0; }

    initial_positions.sort_unstable();

    let mut current_min = usize::MAX;
    let max_pos = initial_positions.last()
                                   .unwrap();

    for pos in 0..*max_pos
    {
        let candidate = calculate_total_fuel_consumption_2(initial_positions, pos);

        if candidate > current_min { return pos - 1; }

        current_min = candidate;
    }

    return *max_pos;
}

fn calculate_total_fuel_consumption(initial_positions: &Vec<i32>, end_position: i32) -> usize
{
    let mut total_fuel_consumption = 0;
    for pos in initial_positions
    {
        if *pos == end_position { continue; }

        total_fuel_consumption += (end_position - pos).abs();
    }

    return total_fuel_consumption as usize;
}

fn calculate_total_fuel_consumption_2(initial_positions: &Vec<i32>, end_position: i32) -> usize
{
    let mut total_fuel_consumption: u32 = 0;
    for pos in initial_positions
    {
        if *pos == end_position { continue; }

        let distance = (end_position - pos).abs() as u32;
        let fuel_needed: u32 = (0..distance+1).sum();

        total_fuel_consumption += fuel_needed;
    }

    return total_fuel_consumption as usize;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_example()
    {
        let mut initial_positions = parse_positions_from_input("src/inputs/day_7_example.txt");
        let final_position        = calculate_optimal_position(&mut initial_positions);
        let fuel_consumption      = calculate_total_fuel_consumption(&initial_positions,
                                                                     final_position);
        assert_eq!(fuel_consumption, 37);
    }

    #[test]
    fn test_second_example()
    {
        let mut initial_positions = parse_positions_from_input("src/inputs/day_7_example.txt");
        let final_position        = calculate_optimal_position_2(&mut initial_positions);
        let fuel_consumption      = calculate_total_fuel_consumption_2(&initial_positions,
                                                                       final_position);
        assert_eq!(fuel_consumption, 168);
    }

    #[test]
    fn test_parse_positions()
    {
        let positions = parse_positions_from_input("src/inputs/day_7_example.txt");
        assert_eq!(positions.len(), 10);
    }

    #[test]
    fn test_calculate_optimal_position()
    {
        let mut positions = parse_positions_from_input("src/inputs/day_7_example.txt");
        assert_eq!(calculate_optimal_position(&mut positions), 2);
    }

    #[test ]
    fn test_calculate_total_fuel_consumption()
    {
        let initial_positions = parse_positions_from_input("src/inputs/day_7_example.txt");

        assert_eq!(calculate_total_fuel_consumption(&initial_positions, 1),  41);
        assert_eq!(calculate_total_fuel_consumption(&initial_positions, 2),  37);
        assert_eq!(calculate_total_fuel_consumption(&initial_positions, 3),  39);
        assert_eq!(calculate_total_fuel_consumption(&initial_positions, 10), 71);
    }

    #[test]
    fn test_calculate_optimal_position_2()
    {
        let mut positions = parse_positions_from_input("src/inputs/day_7_example.txt");
        assert_eq!(calculate_optimal_position_2(&mut positions), 5);
    }

    #[test ]
    fn test_calculate_total_fuel_consumption_2()
    {
        let initial_positions = parse_positions_from_input("src/inputs/day_7_example.txt");

        assert_eq!(calculate_total_fuel_consumption_2(&initial_positions, 1), 242);
        assert_eq!(calculate_total_fuel_consumption_2(&initial_positions, 2), 206);
        assert_eq!(calculate_total_fuel_consumption_2(&initial_positions, 3), 183);
        assert_eq!(calculate_total_fuel_consumption_2(&initial_positions, 5), 168);
        assert_eq!(calculate_total_fuel_consumption_2(&initial_positions, 6), 176);
    }
}