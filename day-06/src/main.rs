fn main() {
    let grid = Grid::new("input.txt");
    println!("{:?}", grid.initial_position);
    part_one(&grid);
    part_two(&grid);
}

fn part_one(grid: &Grid) {
    let mut guard = Guard::new(grid.initial_position);

    let mut passed = vec![false; grid.height * grid.width];

    loop {
        passed[guard.position.0 as usize + grid.width * guard.position.1 as usize] = true;
        guard.step(&grid, None);

        if guard.is_out {
            break;
        }
    }

    println!("{}", passed.iter().filter(|x| **x).count())
}

fn part_two(grid: &Grid) {
    let mut guard = Guard::new(grid.initial_position);

    let mut pass_direction: Vec<Vec<Direction>> = vec![vec![]; grid.height * grid.width];

    let mut new_obstacle_positions: Vec<(i64, i64)> = Vec::new();

    loop {
        let next_position = guard.direction.add(guard.position);

        if grid.what_is(&next_position, None) == Element::Nothing
            && pass_direction[next_position.0 as usize + grid.width * next_position.1 as usize]
                .is_empty()
        {
            if does_loop(guard.clone(), grid, next_position, &pass_direction) {
                new_obstacle_positions.push(next_position);
            }
        }

        pass_direction[guard.position.0 as usize + grid.width * guard.position.1 as usize]
            .push(guard.direction);

        guard.step(&grid, None);

        if guard.is_out {
            break;
        }
    }

    // println!("{:?}", new_obstacle_positions);
    println!("{}", new_obstacle_positions.len());
}

fn does_loop(
    mut guard: Guard,
    grid: &Grid,
    new_obstacle: (i64, i64),
    prev_steps: &[Vec<Direction>],
) -> bool {
    let mut prev_steps = prev_steps.to_owned();

    loop {
        guard.step(&grid, Some(new_obstacle));

        if guard.is_out {
            return false;
        }

        if prev_steps[guard.position.0 as usize + grid.width * guard.position.1 as usize]
            .contains(&guard.direction)
        {
            return true;
        }

        prev_steps[guard.position.0 as usize + grid.width * guard.position.1 as usize]
            .push(guard.direction);

        // println!("{:?}", guard.position);
    }
}

#[derive(Clone)]
struct Guard {
    position: (i64, i64),
    direction: Direction,
    is_out: bool,
}

impl Guard {
    fn new(initial_position: (i64, i64)) -> Self {
        Self {
            position: initial_position,
            direction: Direction::Up,
            is_out: false,
        }
    }

    fn step(&mut self, grid: &Grid, extra_obstacle: Option<(i64, i64)>) {
        let next_position = self.direction.add(self.position);
        match grid.what_is(&next_position, extra_obstacle) {
            Element::Nothing => {
                self.position = next_position;
            }
            Element::Obstacle => {
                self.direction = self.direction.rotate();
            }
            Element::Exit => {
                self.position = next_position;
                self.is_out = true;
            }
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    obstacles: Vec<bool>,
    initial_position: (i64, i64),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn to_vec(&self) -> (i64, i64) {
        use Direction::*;
        match self {
            Up => (0, -1),
            Right => (1, 0),
            Down => (0, 1),
            Left => (-1, 0),
        }
    }

    fn add(&self, position: (i64, i64)) -> (i64, i64) {
        let dir = self.to_vec();
        (position.0 + dir.0, position.1 + dir.1)
    }
}

#[derive(PartialEq, Eq)]
enum Element {
    Nothing,
    Obstacle,
    Exit,
}

impl Grid {
    fn new(filename: &str) -> Self {
        let text = std::fs::read_to_string(filename).unwrap();

        let mut obstacles = Vec::new();
        let height = text.lines().count();

        let mut initial_position = (0, 0);

        for (line_number, line) in text.lines().enumerate() {
            obstacles.extend(line.chars().map(|x| x == '#'));
            let init = line.chars().position(|x| x == '^');

            if let Some(init) = init {
                initial_position = (init as i64, line_number as i64);
            }
        }

        let width = obstacles.len() / height;

        Self {
            width,
            height,
            obstacles,
            initial_position,
        }
    }

    fn what_is(&self, x: &(i64, i64), extra_obstacle: Option<(i64, i64)>) -> Element {
        if x.0 < 0 || x.0 >= self.width as i64 || x.1 < 0 || x.1 >= self.height as i64 {
            return Element::Exit;
        }

        if self.obstacles[x.0 as usize + x.1 as usize * self.width] {
            return Element::Obstacle;
        };

        if let Some(extra_obstacle) = extra_obstacle {
            if extra_obstacle.0 == x.0 && extra_obstacle.1 == x.1 {
                return Element::Obstacle;
            }
        }

        Element::Nothing
    }
}
