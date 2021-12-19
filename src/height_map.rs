use crate::input_parser::parse_input_at;

pub struct HeightMap
{
    // NOTE: ROW MAJOR!
    cells: Vec<MapCell>,
    num_rows: usize,
    num_cols: usize
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

    pub fn get_num_cells(&self)  -> usize          { self.cells.len() }
    pub fn get_cell_value(&self, i: usize) -> usize
    {
        assert!(i < self.cells.len());
        return self.cells[i].val;
    }

    pub fn get_low_points(&mut self) -> Vec<usize>
    {
        let mut low_points = Vec::new();

        for i in 0..self.cells.len()
        {
            let cell = &mut self.cells[i];

            if cell.is_visited { continue; }

            (*cell).is_visited = true;

            let low = self.flow_down_from(i);
            if low.is_some()
            {
                low_points.push(low.unwrap());
            }
        }

        return low_points;
    }

    // Simulate a "ball" falling from a given height map position.
    // The ball will always move to the smaller neighbour, as long as it's
    // lower than the current point.
    fn flow_down_from(&mut self, i: usize) -> Option<usize>
    {
        let mut current_value = self.cells[i].val;
        let (mut cell, mut idx) = self.get_smallest_neighbour_and_idx(i);
        while cell.val < current_value
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
            let sn        = self.get_smallest_neighbour_and_idx(idx);
            cell = sn.0;
            idx  = sn.1;
        }

        if cell.val == current_value { return  None; }
        return Some(current_value);
    }

    fn get_smallest_neighbour_and_idx(&mut self, i: usize) -> (&mut MapCell, usize)
    {
        let mut smaller_neighbours = self.get_neighbours(i);

        smaller_neighbours.sort_unstable_by(|a, b|{ a.0.cmp(&b.0) });

        assert!(!smaller_neighbours.is_empty());
        // The only case when smaller_neighbours would be empty is when the height map is 1x1
        let idx = if smaller_neighbours.is_empty() { i } else { smaller_neighbours[0].1 };
        return (&mut self.cells[idx], idx);
    }

    pub fn get_neighbours(&self, i: usize) -> Vec<(usize, usize)>
    {
        let y = i / self.num_cols;
        let x = i - y * self.num_cols;

        let left_idx    = i.checked_sub(1);
        let right_idx   = if i < self.cells.len() - 1 { Some(i + 1) } else { None };
        let top_idx     = i.checked_sub(self.num_cols);
        let bottom_idx  = if i < self.cells.len() - self.num_cols { Some(i + self.num_cols) } else { None };

        let mut smaller_neighbours = Vec::with_capacity(4);
        if x > 0 && left_idx.is_some()
        {
            let idx = left_idx.unwrap();
            smaller_neighbours.push( (self.cells[idx].val, idx) );
        }
        if x < self.num_cols && right_idx.is_some()
        {
            let idx = right_idx.unwrap();
            smaller_neighbours.push( (self.cells[idx].val, idx) );
        }
        if y > 0 && top_idx.is_some()
        {
            let idx = top_idx.unwrap();
            smaller_neighbours.push( (self.cells[idx].val, idx) );
        }
        if y < self.num_rows && bottom_idx.is_some()
        {
            let idx = bottom_idx.unwrap();
            smaller_neighbours.push( (self.cells[idx].val, idx) );
        }

        return smaller_neighbours;
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

        println!("{:?}", low_points);
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
        let low = map.flow_down_from(0).unwrap();

        assert_eq!(low, 1);
    }

    #[test]
    fn test_trickle_down()
    {
        let mut map = HeightMap::new_from_file("src/inputs/day_9_example.txt");
        let low = map.flow_down_from(14).unwrap();

        assert_eq!(low, 5);
    }

    #[test]
    fn test_get_smaller_neighbour()
    {
        let mut map = HeightMap::new_from_file("src/inputs/day_9_example.txt");
        let smaller_neighbour = map.get_smallest_neighbour_and_idx(24);

        assert_eq!(smaller_neighbour.0.val, 6); // value
        assert_eq!(smaller_neighbour.1, 23);     // idx
    }
}