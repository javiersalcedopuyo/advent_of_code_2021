use crate::height_map::HeightMap;

pub fn day_9_1() -> usize
{
    let mut map = HeightMap::new_from_file("src/inputs/day_9_input.txt");
    let low_points = map.get_low_points();

    return calculate_risk(&low_points);
}

fn calculate_risk(low_points: &Vec<usize>) -> usize
{
    let mut risk = 0;
    for p in low_points
    {
        risk += p + 1;
    }
    return risk;
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_puzzle()
    {
        let mut map = HeightMap::new_from_file("src/inputs/day_9_example.txt");
        let low_points = map.get_low_points();
        assert_eq!(calculate_risk(&low_points), 15);
    }

    #[test]
    fn test_first_puzzle_full_input()
    {
        let mut map = HeightMap::new_from_file("src/inputs/day_9_input.txt");
        let low_points = map.get_low_points();
        assert_eq!(calculate_risk(&low_points), 541);
    }

    #[test]
    fn test_calculate_risk()
    {
        let low_points = vec![1,0,5,5];
        assert_eq!(calculate_risk(&low_points), 15);
    }
}