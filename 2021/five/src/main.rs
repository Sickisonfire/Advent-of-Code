use std::cmp;
use std::collections::HashMap;
use std::str::Lines;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}
impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
}
#[derive(Debug)]
struct LineSegment {
    start_coordinate: Coordinate,
    end_coordinate: Coordinate,
}

impl LineSegment {
    // create new linesegment from raw coords
    pub fn new(line_iter: &mut Lines) -> Option<LineSegment> {
        let line = line_iter.next()?;

        let coords = line
            .split(" -> ")
            .map(|item| item.split(",").collect::<Vec<_>>())
            .flatten()
            .map(|item| item.parse::<i32>().expect("should only be numbers"))
            .collect::<Vec<_>>();

        Some(LineSegment {
            start_coordinate: Coordinate::new(coords[0], coords[1]),
            end_coordinate: Coordinate::new(coords[2], coords[3]),
        })
    }
}

struct HydroField {
    diagram: HashMap<Coordinate, i32>,
}
impl HydroField {
    pub fn add_linesegment_to_diagram(&mut self, ls: LineSegment) {
        let y1 = ls.start_coordinate.y;
        let y2 = ls.end_coordinate.y;
        let x1 = ls.start_coordinate.x;
        let x2 = ls.end_coordinate.x;

        if ls.start_coordinate.x == ls.end_coordinate.x {
            let start = cmp::min(y1, y2);
            let end = cmp::max(y1, y2);

            for pos in start..=end {
                let new_coord = Coordinate::new(x1, pos);

                let diagram_coord = self.diagram.entry(new_coord).or_insert(0);
                *diagram_coord += 1;
            }
        } else if ls.start_coordinate.y == ls.end_coordinate.y {
            let start = cmp::min(x1, x2);
            let end = cmp::max(x1, x2);

            for pos in start..=end {
                let new_coord = Coordinate::new(pos, y1);
                self.add_coord(new_coord);
            }
        } else {
            let diff = (x1 - x2).abs();
            let add_x = (x2 - x1) / diff;
            let add_y = (y2 - y1) / diff;
            let mut new_coord_vals = vec![x1, y1];

            for _ in 1..diff {
                new_coord_vals[0] += add_x;
                new_coord_vals[1] += add_y;

                let new_coord = Coordinate::new(new_coord_vals[0], new_coord_vals[1]);
                self.add_coord(new_coord);
            }

            let start_coord = Coordinate::new(x1, y1);
            self.add_coord(start_coord);

            let end_coord = Coordinate::new(x2, y2);
            self.add_coord(end_coord);
        }
    }

    fn add_coord(&mut self, coordinate: Coordinate) {
        let diagram_coord = self.diagram.entry(coordinate).or_insert(0);
        *diagram_coord += 1;
    }
}

fn main() {
    let mut lines = include_str!("input").lines();

    let mut line_segments = Vec::new();

    while let Some(line_segment) = LineSegment::new(&mut lines) {
        line_segments.push(line_segment);
    }

    let mut hydro_field = HydroField {
        diagram: HashMap::new(),
    };

    for ls in line_segments {
        hydro_field.add_linesegment_to_diagram(ls);
    }
    let mut count = 0;
    for (coord, num_occ) in &hydro_field.diagram {
        if *num_occ >= 2 {
            count += 1;
            println!("{:#?}", coord)
        }
    }
    println!("{:#?}", count);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_linesegment_new() {
        let mut lines = include_str!("example").lines();
        match LineSegment::new(&mut lines) {
            Some(result) => {
                let (x1, y1, x2, y2) = (0, 9, 5, 9);
                assert_eq!(x1, result.start_coordinate.x);
                assert_eq!(y1, result.start_coordinate.y);
                assert_eq!(x2, result.end_coordinate.x);
                assert_eq!(y2, result.end_coordinate.y);
            }
            None => (),
        }
    }
    #[test]
    fn test_multiple_linesegments_new() {
        let mut lines = include_str!("example").lines();

        let mut line_segments = Vec::new();

        while let Some(line_segment) = LineSegment::new(&mut lines) {
            line_segments.push(line_segment);
        }

        assert_eq!(line_segments.len(), 10);
    }

    #[test]
    
}
