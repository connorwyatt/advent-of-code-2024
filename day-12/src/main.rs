use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

const INPUT: &str = include_str!("aoc-input/input.txt");

fn main() {
    println!(
        "Result: {:?}",
        fencing_total_price_with_bulk_discount(INPUT)
    );
}

#[allow(dead_code)]
fn fencing_total_price(input: &str) -> usize {
    let grid = GardenPlotsGrid::from_str(input).unwrap();

    let regions = calculate_regions(&grid);

    regions
        .iter()
        .map(|region| {
            let mut perimeter = 0;

            for position in region {
                perimeter += Direction::all()
                    .iter()
                    .filter(|direction| {
                        let Some(adjacent_position) = position.translate(direction) else {
                            return true;
                        };
                        !region.contains(&adjacent_position)
                    })
                    .count();
            }

            perimeter * region.len()
        })
        .sum()
}

fn fencing_total_price_with_bulk_discount(input: &str) -> usize {
    let grid = GardenPlotsGrid::from_str(input).unwrap();

    let regions = calculate_regions(&grid);

    regions
        .iter()
        .map(|region| {
            let mut grouped_edges: HashMap<(Direction, usize), HashSet<usize>> = HashMap::new();

            for position in region {
                for direction in Direction::all() {
                    let main_axis = match direction {
                        Direction::Up | Direction::Down => position.y,
                        Direction::Left | Direction::Right => position.x,
                    };
                    let secondary_axis = match direction {
                        Direction::Up | Direction::Down => position.x,
                        Direction::Left | Direction::Right => position.y,
                    };
                    let Some(adjacent_position) = position.translate(&direction) else {
                        grouped_edges
                            .entry((direction, main_axis))
                            .or_default()
                            .insert(secondary_axis);
                        continue;
                    };
                    if region.contains(&adjacent_position) {
                        continue;
                    }
                    grouped_edges
                        .entry((direction, main_axis))
                        .or_default()
                        .insert(secondary_axis);
                }
            }

            grouped_edges
                .values()
                .map(|positions| {
                    let mut positions = positions.iter().cloned().collect::<Vec<_>>();

                    positions.sort();

                    positions
                        .windows(2)
                        .filter(|&x| {
                            let a = x.first().unwrap();
                            let b = x.last().unwrap();
                            *b != *a + 1
                        })
                        .count()
                        + 1
                })
                .sum::<usize>() * region.len()

        })
        .sum()}

fn calculate_regions(grid: &GardenPlotsGrid) -> Vec<HashSet<Position>> {
    let mut plots_by_plant: HashMap<char, HashSet<Position>> = HashMap::new();

    for y in 0..*grid.height() {
        for x in 0..*grid.width() {
            let position = Position::new(x, y);
            let plant = grid.get(&position).unwrap();
            let plant_regions = plots_by_plant.entry(*plant).or_default();
            plant_regions.insert(position);
        }
    }

    let mut regions: Vec<HashSet<Position>> = Vec::new();

    for (_, positions) in plots_by_plant {
        let mut plant_regions: Vec<HashSet<Position>> = Vec::new();

        for position in positions {
            let plant_regions_clone = plant_regions.clone();

            let adjacent_regions = get_adjacent_regions(&plant_regions_clone, &position);

            if adjacent_regions.is_empty() {
                plant_regions.push(HashSet::from([position]));
            } else if adjacent_regions.len() == 1 {
                plant_regions
                    .iter_mut()
                    .find(|pr| pr == adjacent_regions.first().unwrap())
                    .unwrap()
                    .insert(position);
            } else {
                let mut merged_region = HashSet::from([position]);
                for &adjacent_region in adjacent_regions.as_slice() {
                    for position in adjacent_region {
                        merged_region.insert(position.clone());
                    }
                }
                plant_regions.retain(|pr| !adjacent_regions.iter().any(|&ar| ar == pr));
                plant_regions.push(merged_region);
            }
        }

        for plant_region in plant_regions {
            regions.push(plant_region);
        }
    }

    regions
}

fn get_adjacent_regions<'a>(
    regions: &'a [HashSet<Position>],
    position: &Position,
) -> Vec<&'a HashSet<Position>> {
    regions
        .iter()
        .filter(|region| {
            Direction::all().iter().any(|direction| {
                let Some(adjacent_position) = position.translate(direction) else {
                    return false;
                };
                region.contains(&adjacent_position)
            })
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn translate(&self, direction: &Direction) -> Option<Position> {
        match direction {
            Direction::Up => {
                let new_y = self.y.checked_sub(1)?;
                Some(Position::new(self.x, new_y))
            }
            Direction::Right => {
                let new_x = self.x.checked_add(1)?;
                Some(Position::new(new_x, self.y))
            }
            Direction::Down => {
                let new_y = self.y.checked_add(1)?;
                Some(Position::new(self.x, new_y))
            }
            Direction::Left => {
                let new_x = self.x.checked_sub(1)?;
                Some(Position::new(new_x, self.y))
            }
        }
    }
}

struct GardenPlotsGrid {
    width: usize,
    height: usize,
    plants: Vec<Vec<char>>,
}

impl FromStr for GardenPlotsGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut height = 0;
        let mut width = 0;

        let plants = s
            .lines()
            .map(|line| {
                height += 1;

                let line_len = line.len();
                if line_len < width {
                    panic!("width is variable");
                } else {
                    width = line_len;
                }

                line.chars().collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self {
            width,
            height,
            plants,
        })
    }
}

impl GardenPlotsGrid {
    fn width(&self) -> &usize {
        &self.width
    }

    fn height(&self) -> &usize {
        &self.height
    }

    fn get(&self, position: &Position) -> Option<&char> {
        self.plants.get(position.y)?.get(position.x)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT_1: &str = include_str!("aoc-input/example-input-1.txt");

    const EXAMPLE_INPUT_2: &str = include_str!("aoc-input/example-input-2.txt");

    #[test]
    fn fencing_total_price_works() {
        assert_eq!(fencing_total_price(EXAMPLE_INPUT_1), 140);
        assert_eq!(fencing_total_price(EXAMPLE_INPUT_2), 1930);
    }

    #[test]
    fn fencing_total_price_with_bulk_discount_works() {
        assert_eq!(fencing_total_price_with_bulk_discount(EXAMPLE_INPUT_1), 80);
        assert_eq!(
            fencing_total_price_with_bulk_discount(EXAMPLE_INPUT_2),
            1206
        );
    }
}
