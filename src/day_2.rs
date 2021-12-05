use crate::
{
    input_parser::*,
    coordinates::Coordinate
};

pub fn day_2_1() -> i32
{
    let input = parse_input_at("src/inputs/day_2_input.txt");
    return match input
    {
        Err(_)    => return 0,
        Ok(input) => compute_position_product_from_input(input)
    };
}

pub fn day_2_2() -> i32
{
    let input = parse_input_at("src/inputs/day_2_input.txt");
    return match input
    {
        Err(_)    => return 0,
        Ok(input) => compute_position_product_from_input_2(input)
    };
}

fn compute_position_product_from_input(input: String) -> i32
{
    let lines = input.split("\n");

    let mut position = Coordinate{y:0, z:0};

    for line in lines
    {
        position += get_movement_from_input_entry(line);
    }

    // We're using DEPTH, so we have to negate Y
    return -position.y * position.z;
}

fn compute_position_product_from_input_2(input: String) -> i32
{
    let lines = input.lines();

    let mut aim      = 0;
    let mut position = Coordinate{y:0, z:0};

    for line in lines
    {
        let (dir, dist) = parse_input_entry(line);
        aim        -= dir.y * dist;
        position.y -= dir.z * aim * dist;
        position.z += dir.z * dist;
    }

    // We're using DEPTH, so we have to negate Y
    return -position.y * position.z;
}

fn get_movement_from_input_entry(entry: &str) -> Coordinate
{
    let (dir, dist) = parse_input_entry(entry);
    return dir * dist;
}

fn parse_input_entry(entry: &str) -> (Coordinate, i32)
{
    let mut words = entry.split_whitespace();

    let dir = match words.next().unwrap_or("")
    {
        "forward" => Coordinate{y:0,  z:1},
        "up"      => Coordinate{y:1,  z:0},
        "down"    => Coordinate{y:-1, z:0},
        _         => Coordinate{y:0,  z:0}
    };

    let dist = words.next()
                    .unwrap_or("0")
                    .parse::<i32>()
                    .unwrap_or(0);

    return (dir, dist);
}

#[cfg(test)]
mod tests
{
    use super::*;

    // PUZZLE 1
    #[test]
    fn test_example_1()
    {
        let input = parse_input_at("src/inputs/day_2_example.txt");
        assert!(input.is_ok());

        let input = input.unwrap();

        let result = compute_position_product_from_input(input);
        assert_eq!(result, 150);
    }

    #[test]
    fn test_no_distance()
    {
        let entry = "forward";
        let movement = get_movement_from_input_entry(entry);

        let expected = Coordinate{y:0, z:0};

        assert_eq!(movement, expected);
    }

    #[test]
    fn test_no_direction()
    {
        let entry = "5";
        let movement = get_movement_from_input_entry(entry);

        let expected = Coordinate{y:0, z:0};

        assert_eq!(movement, expected);
    }

    #[test]
    fn test_get_movement_from_input_entry()
    {
        let entry = "forward 2";
        let movement = get_movement_from_input_entry(entry);

        let expected = Coordinate{y:0, z:2};

        assert_eq!(movement, expected);
    }

    #[test]
    fn test_parse_input_entry()
    {
        let entry = "forward 2";
        let (dir, dist) = parse_input_entry(entry);

        let expected_dir  = Coordinate{y:0, z:1};
        let expected_dist = 2;

        assert_eq!(dir, expected_dir);
        assert_eq!(dist, expected_dist);
    }

    // PUZZLE 2
    #[test]
    fn test_example_2()
    {
        let input = parse_input_at("src/inputs/day_2_example.txt");
        assert!(input.is_ok());

        let input = input.unwrap();

        let result = compute_position_product_from_input_2(input);
        assert_eq!(result, 900);
    }
}