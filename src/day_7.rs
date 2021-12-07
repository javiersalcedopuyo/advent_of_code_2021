use crate::input_parser::parse_input_at;

pub fn day_7_1() -> usize
{
    let mut initial_positions = parse_positions_from_input("src/inputs/day_7_input.txt");
    let final_position = calculate_optimal_position(&mut initial_positions);

    return calculate_total_fuel_consumption(&initial_positions, final_position);
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
}