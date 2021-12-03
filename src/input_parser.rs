use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub fn parse_input_at(path_str: &str) -> io::Result<String>
{
    let path = Path::new(path_str);
    let mut file = File::open(path)?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    return Ok(buf);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_load_example_file()
    {
        let path = Path::new("src/inputs/day_2_example.txt");
        let _ = match File::open(path)
        {
            Err(why) => panic!("Couldn't open because: {}", why),
            Ok(file) => file,
        };
    }

    #[test]
    fn test_parse_example_file()
    {
        let contents = parse_input_at("src/inputs/day_2_example.txt");
        assert!(contents.is_ok());

        let contents = contents.unwrap();

        let lines = contents.split("\n");
        let mut count = 0;
        for _ in lines
        {
            count += 1;
        }

        assert_eq!(count, 6);
    }
}