use std::collections::HashSet;

fn main() {
    let filename = "input.txt";
    let grid_size = (11, 7);
    let grid_size = (101, 103);
    let input = Input::new(filename, grid_size);

    // println!("{:?}", part_one(&input));
    // println!("{:?}", part_two_help(&input));
    println!("{:?}", part_two(&input));
}

fn part_two_help(input: &Input) -> usize {
    let mut base_quadrant = HashSet::new();
    for i in 0..10 {
        for j in 0..10 {
            base_quadrant.insert((i as isize, input.grid_size.1 - j as isize));
            base_quadrant.insert((
                input.grid_size.0 - i as isize,
                input.grid_size.1 - j as isize,
            ));
        }
    }

    let mut potentials = Vec::new();

    for t in 6174..8800 {
        let mut final_positions: HashSet<(isize, isize)> = HashSet::new();
        for robot in input.robots.iter() {
            let mut final_x = (robot.pos.0 + t * robot.vel.0) % input.grid_size.0;
            let mut final_y = (robot.pos.1 + t * robot.vel.1) % input.grid_size.1;
            if final_x < 0 {
                final_x += input.grid_size.0;
            }
            if final_y < 0 {
                final_y += input.grid_size.1;
            }
            final_positions.insert((final_x, final_y));
        }
        if final_positions.is_disjoint(&base_quadrant) {
            potentials.push(t);
        }
    }
    println!("{:?}", potentials);
    8
}

fn part_two(input: &Input) {
    let mut t = 0;
    let mut dir = 1;
    loop {
        let mut final_positions: Vec<(isize, isize)> = Vec::new();
        for robot in input.robots.iter() {
            let mut final_x = (robot.pos.0 + t * robot.vel.0) % input.grid_size.0;
            let mut final_y = (robot.pos.1 + t * robot.vel.1) % input.grid_size.1;
            if final_x < 0 {
                final_x += input.grid_size.0;
            }
            if final_y < 0 {
                final_y += input.grid_size.1;
            }
            final_positions.push((final_x, final_y));
        }
        draw_robots(final_positions, input.grid_size);
        println!("\nTime: {}\n", t);
        let mut a = String::new();
        std::io::stdin().read_line(&mut a);
        if a.trim() == "k" {
            dir *= -1;
        }
        if let Ok(x) = a.trim().parse::<isize>() {
            t = x;
        } else {
            t += dir;
        }
    }
}

fn part_one(input: &Input) -> usize {
    let t = 100;
    let mut final_positions: Vec<(isize, isize)> = Vec::new();
    for robot in input.robots.iter() {
        let mut final_x = (robot.pos.0 + t * robot.vel.0) % input.grid_size.0;
        let mut final_y = (robot.pos.1 + t * robot.vel.1) % input.grid_size.1;
        if final_x < 0 {
            final_x += input.grid_size.0;
        }
        if final_y < 0 {
            final_y += input.grid_size.1;
        }
        final_positions.push((final_x, final_y));
    }

    println!("{:?}", final_positions);

    let mid_x = (input.grid_size.0 - 1) / 2;
    let mid_y = (input.grid_size.1 - 1) / 2;
    println!("{}, {}", mid_x, mid_y);

    let mut quadrant_counts: Vec<usize> = vec![0; 4];

    for (x, y) in final_positions {
        if x < mid_x {
            if y < mid_y {
                quadrant_counts[0] += 1;
            } else if y > mid_y {
                quadrant_counts[1] += 1;
            }
        }
        if x > mid_x {
            if y < mid_y {
                quadrant_counts[2] += 1;
            } else if y > mid_y {
                quadrant_counts[3] += 1;
            }
        }
    }
    println!("{:?}", quadrant_counts);
    quadrant_counts.iter().product()
}

#[derive(Debug)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

#[derive(Debug)]
struct Input {
    grid_size: (isize, isize),
    robots: Vec<Robot>,
}

impl Input {
    fn new(filename: &str, grid_size: (isize, isize)) -> Self {
        let text = std::fs::read_to_string(filename).unwrap();
        let robots: Vec<Robot> = text
            .lines()
            .map(|line| {
                let eq_1 = line.find('=').unwrap();
                let comma_1 = line.find(',').unwrap();
                let space = line.find(' ').unwrap();
                let eq_2 = line.rfind('=').unwrap();
                let comma_2 = line.rfind(',').unwrap();

                Robot {
                    pos: (
                        line[eq_1 + 1..comma_1].parse().unwrap(),
                        line[comma_1 + 1..space].parse().unwrap(),
                    ),
                    vel: (
                        line[eq_2 + 1..comma_2].parse().unwrap(),
                        line[comma_2 + 1..].parse().unwrap(),
                    ),
                }
            })
            .collect();

        Self { grid_size, robots }
    }
}

fn draw_robots(pos: Vec<(isize, isize)>, grid_size: (isize, isize)) {
    let line_template: Vec<char> = std::iter::repeat(' ').take(grid_size.0 as usize).collect();
    let mut all_lines = vec![line_template; grid_size.1 as usize];

    for (x, y) in pos {
        all_lines[y as usize][x as usize] = 'x';
    }

    for line in all_lines {
        println!("{}", line.iter().collect::<String>());
    }
}
