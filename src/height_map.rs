use crate::input_parser::parse_input_at;

pub struct HeightMap
{
    // NOTE: ROW MAJOR!
    cells: Vec<MapCell>,
    pub num_rows: usize,
    pub num_cols: usize
}

struct MapCell
{
    val: usize,
    is_visited: bool
}

impl HeightMap
{
    fn new() -> Self { Self{ cells: Vec::new(), num_rows: 0, num_cols: 0} }

    pub fn new_from_file(path: &str) -> Self
    {
        let input = parse_input_at(path).unwrap_or_default();

        let mut new_map = Self::new();
        for line in input.lines()
        {
            new_map.num_cols = 0; // Stupid but works
            for c in line.chars()
            {
                let val = c.to_digit(10)
                        .unwrap_or(0);

                new_map.push( val as usize );
                new_map.num_cols += 1
            }
            new_map.num_rows += 1;
        }
        return new_map;
    }

    fn push(&mut self, val: usize)
    {
        self.cells.push( MapCell{ val, is_visited: false} );
    }

    pub fn get_low_points(&mut self) -> Vec<usize>
    {
        let mut low_points = Vec::new();

        for i in 0..self.cells.len()
        {
            let cell = &mut self.cells[i];

            if cell.is_visited { continue; }

            (*cell).is_visited = true;

            let low = self.trickle_down_from(i);
            if low.is_some()
            {
                low_points.push(low.unwrap());
            }
        }

        return low_points;
    }

    // Simulate a "ball" falling from a given height map position.
    // The ball will always move to the smaller neighbour, as long as it's not
    // higher than the current point.
    fn trickle_down_from(&mut self, i: usize) -> Option<usize>
    {
        let mut current_value = self.cells[i].val;
        let (mut cell, mut idx) = self.get_smaller_or_equal_neighbour_and_idx(i);
        while cell.val <= current_value
        {
            if cell.is_visited
            {
                // OPTIMISATION
                // Other cell already went through this, so the resulting low
                // point is already cached
                return None;
            }

            cell.is_visited = true;

            current_value = cell.val;
            let sn        = self.get_smaller_or_equal_neighbour_and_idx(idx);

            if idx == sn.1 { break; } // We're already at the lowest possible point

            cell = sn.0;
            idx  = sn.1;
        }

        assert!(current_value < 9);
        return Some(current_value);
    }

    fn get_smaller_or_equal_neighbour_and_idx(&mut self, i: usize) -> (&mut MapCell, usize)
    {
        let y = i / self.num_cols;
        let x = i - y * self.num_cols;

        let left_idx    = i.checked_sub(1).unwrap_or(i);
        let right_idx   = if i < self.cells.len() - 1 { i + 1 } else { i };
        let top_idx     = i.checked_sub(self.num_cols).unwrap_or(i);
        let bottom_idx  = if i < self.cells.len() - self.num_cols { i + self.num_cols } else { i };

        let current_val = self.cells[i].val;

        let mut smaller_neighbours = Vec::new();
        if x > 0 && self.cells[left_idx].val <= current_val
        {
            smaller_neighbours.push( (self.cells[left_idx].val, left_idx) );
        }
        if x < self.num_cols && self.cells[right_idx].val <= current_val
        {
            smaller_neighbours.push( (self.cells[right_idx].val, right_idx) );
        }
        if y > 0 && self.cells[top_idx].val <= current_val
        {
            smaller_neighbours.push( (self.cells[top_idx].val, top_idx) );
        }
        if y < self.num_rows && self.cells[bottom_idx].val <= current_val
        {
            smaller_neighbours.push( (self.cells[bottom_idx].val, bottom_idx) );
        }

        smaller_neighbours.sort_by(|a, b|{ a.0.cmp(&b.0) });

        let idx = if smaller_neighbours.is_empty() { i } else { smaller_neighbours[0].1 };

        return (&mut self.cells[idx], idx);
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parse_map()
    {
        let map = HeightMap::new_from_file("src/inputs/day_9_example.txt");
        assert_eq!(map.cells.len(), 50);
        assert_eq!(map.num_rows, 5);
        assert_eq!(map.num_cols, 10);
    }

    #[test]
    fn test_get_low_points()
    {
        let mut map    = HeightMap::new_from_file("src/inputs/day_9_example.txt");
        let low_points = map.get_low_points();

        assert_eq!(low_points.len(), 4);

        assert_eq!(low_points[0], 1);
        assert_eq!(low_points[1], 5);
        assert_eq!(low_points[2], 0);
        assert_eq!(low_points[3], 5);
    }

    #[test]
    fn test_trickle_down_from_neighbour()
    {
        let mut map = HeightMap::new_from_file("src/inputs/day_9_example.txt");
        let low = map.trickle_down_from(0).unwrap();

        assert_eq!(low, 1);
    }

    #[test]
    fn test_trickle_down()
    {
        let mut map = HeightMap::new_from_file("src/inputs/day_9_example.txt");
        let low = map.trickle_down_from(14).unwrap();

        assert_eq!(low, 5);
    }

    #[test]
    fn test_get_smaller_neighbour()
    {
        let mut map = HeightMap::new_from_file("src/inputs/day_9_example.txt");
        let smaller_neighbour = map.get_smaller_or_equal_neighbour_and_idx(24);

        assert_eq!(smaller_neighbour.0.val, 6); // value
        assert_eq!(smaller_neighbour.1, 23);     // idx
    }
}