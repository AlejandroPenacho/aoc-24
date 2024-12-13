use std::collections::{HashMap, HashSet};

fn main() {
    let filename = "input.txt";
    let grid = Grid::new(filename);
    println!("{}", part_one(&grid));
    println!("{}", part_two(&grid));
}

fn part_one(grid: &Grid) -> usize {
    let mut analyzed_cells: HashSet<(isize, isize)> = HashSet::new();

    let mut count = 0;

    for i in 0..grid.width {
        for j in 0..grid.height {
            if analyzed_cells.contains(&(i as isize, j as isize)) {
                continue;
            }
            let (cells, walls) = get_cells_and_walls((i as isize, j as isize), &grid);

            for v in cells.iter() {
                analyzed_cells.insert(*v);
            }
            count += cells.len() * walls;
        }
    }

    count
}

fn part_two(grid: &Grid) -> usize {
    let mut analyzed_cells: HashSet<(isize, isize)> = HashSet::new();

    let mut count = 0;

    for i in 0..grid.width {
        for j in 0..grid.height {
            if analyzed_cells.contains(&(i as isize, j as isize)) {
                continue;
            }
            let (cells, walls) = get_cells_and_straight_walls((i as isize, j as isize), &grid);

            for v in cells.iter() {
                analyzed_cells.insert(*v);
            }

            count += cells.len() * walls;
        }
    }

    count
}

fn get_cells_and_walls(x0: (isize, isize), grid: &Grid) -> (HashSet<(isize, isize)>, usize) {
    let mut queue = vec![x0];
    let plant = grid.get_plant(x0).unwrap();

    let mut n_walls = 0;

    let mut analyzed_cells = HashSet::new();
    let all_dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while queue.len() > 0 {
        let cell = queue.pop().unwrap();
        if analyzed_cells.contains(&cell) {
            continue;
        }
        analyzed_cells.insert(cell);

        for new_cell in all_dirs.map(|(d0, d1)| (cell.0 + d0, cell.1 + d1)) {
            let new_plant = grid.get_plant(new_cell);
            if new_plant.map_or(false, |x| x == plant) {
                queue.push(new_cell);
            } else {
                n_walls += 1;
            }
        }
    }

    (analyzed_cells, n_walls)
}

fn get_cells_and_straight_walls(
    x0: (isize, isize),
    grid: &Grid,
) -> (HashSet<(isize, isize)>, usize) {
    let mut queue = vec![x0];
    let plant = grid.get_plant(x0).unwrap();

    let mut wall_registry = WallRegistry::new();
    let mut n_straight_walls = 0;

    let mut analyzed_cells = HashSet::new();
    let all_dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while queue.len() > 0 {
        let cell = queue.pop().unwrap();
        if analyzed_cells.contains(&cell) {
            continue;
        }
        analyzed_cells.insert(cell);

        for new_cell in all_dirs.map(|(d0, d1)| (cell.0 + d0, cell.1 + d1)) {
            let new_plant = grid.get_plant(new_cell);
            if new_plant.map_or(false, |x| x == plant) {
                queue.push(new_cell);
            } else {
                n_straight_walls += 1;
                n_straight_walls -= wall_registry.get_n_contiguous_walls(cell, new_cell);
            }
        }
    }

    (analyzed_cells, n_straight_walls)
}

struct WallRegistry {
    inner: HashSet<((isize, isize), u8)>,
}
impl WallRegistry {
    fn new() -> Self {
        Self {
            inner: HashSet::new(),
        }
    }
    fn get_n_contiguous_walls(&mut self, cell: (isize, isize), other: (isize, isize)) -> usize {
        if cell.0 != other.0 && cell.1 != other.1 {
            panic!()
        }
        let orientation;
        if other.0 == cell.0 + 1 {
            orientation = 0;
        } else if other.0 == cell.0 - 1 {
            orientation = 2;
        } else if other.1 == cell.1 + 1 {
            orientation = 1;
        } else if other.1 == cell.1 - 1 {
            orientation = 3;
        } else {
            panic!();
        }

        let is_horizontal = cell.0 == other.0;
        let index = (cell, orientation);

        let mut n_contiguous = 0;
        if is_horizontal {
            if self.inner.contains(&((cell.0 + 1, cell.1), orientation)) {
                n_contiguous += 1;
            }
            if self.inner.contains(&((cell.0 - 1, cell.1), orientation)) {
                n_contiguous += 1;
            }
        } else {
            if self.inner.contains(&((cell.0, cell.1 - 1), orientation)) {
                n_contiguous += 1;
            }
            if self.inner.contains(&((cell.0, cell.1 + 1), orientation)) {
                n_contiguous += 1;
            }
        }

        self.inner.insert(index);

        n_contiguous
    }
}

struct Grid {
    width: usize,
    height: usize,
    plant: Vec<u8>,
}

impl Grid {
    fn new(filename: &str) -> Grid {
        let text = std::fs::read_to_string(filename).unwrap();
        let height = text.lines().count();
        let width = text.lines().next().unwrap().chars().count();

        let mut letter_map: HashMap<char, u8> = HashMap::new();
        let mut i = 0;
        let mut plant: Vec<u8> = Vec::new();

        text.lines().map(|x| x.chars()).flatten().for_each(|x| {
            let value = match letter_map.get(&x) {
                Some(i) => *i,
                None => {
                    letter_map.insert(x, i);
                    i += 1;
                    i - 1
                }
            };
            plant.push(value);
        });

        Grid {
            width,
            height,
            plant,
        }
    }

    fn get_plant(&self, x: (isize, isize)) -> Option<u8> {
        if x.0 < 0 || x.0 >= self.width as isize || x.1 < 0 || x.1 >= self.height as isize {
            return None;
        }

        Some(self.plant[x.0 as usize + x.1 as usize * self.width])
    }
}
