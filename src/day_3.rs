use crate::input_parser::*;

pub fn day_3_1() -> i32
{
    let input = parse_input_at("src/inputs/day_3_input.txt")
                    .unwrap_or("".to_string());

    return get_gamma_epsilon_product_from_input(input);
}

fn get_gamma_epsilon_product_from_input(input: String) -> i32
{
    let bit_counters = count_bits(input);

    let mut gamma   = String::new();
    let mut epsilon = String::new();

    for bit in bit_counters
    {
        gamma   += if bit.0 > bit.1 { "0" } else { "1"};
        epsilon += if bit.0 < bit.1 { "0" } else { "1"};
    }

    let gamma   = isize::from_str_radix(gamma.as_str(),   2).unwrap_or(0);
    let epsilon = isize::from_str_radix(epsilon.as_str(), 2).unwrap_or(0);

    return (gamma * epsilon) as i32;
}

fn count_bits(input: String) -> Vec<(usize, usize)>
{
    let mut result: Vec<(usize, usize)> = Vec::new();

    for line in input.lines()
    {
        let mut chars = line.chars();
        let num_chars = line.len();

        if result.is_empty()
        {
            result.resize_with(num_chars, Default::default);
        }

        for i in 0..num_chars
        {
            match chars.next()
            {
                Some(c) => match c
                {
                    '0' => result[i].0 += 1,
                    '1' => result[i].1 += 1,
                    _ => ()
                },
                _ => ()
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests
{
    use super::*;

    // PUZZLE 1
    #[test]
    fn test_example_1()
    {
        let input = parse_input_at("src/inputs/day_3_example.txt");
        assert!(input.is_ok());

        let input = input.unwrap();

        assert_eq!(get_gamma_epsilon_product_from_input(input), 198);
    }

    #[test]
    fn test_reading_example_input()
    {
        let input = parse_input_at("src/inputs/day_3_example.txt");
        assert!(input.is_ok());

        let input = input.unwrap();
        let lines = input.lines();

        assert!(lines.count() == 12);
    }

    #[test]
    fn test_string_to_binary_to_decimal()
    {
        let input = "10101";
        let decimal = isize::from_str_radix(input, 2);
        assert!(decimal.is_ok());
        let decimal = decimal.unwrap();

        assert_eq!(decimal, 21);
    }
}