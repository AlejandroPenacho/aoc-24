use std::collections::HashMap;

fn main() {
    let filename = "input.txt";
    let antennas = AntennaGrid::new(filename);
    println!("{}", part_one(&antennas));
    println!("{}", part_two(&antennas));
}

fn part_one(antennas: &AntennaGrid) -> usize {
    let mut antinodes = Antinodes::new(antennas);

    for ants in antennas.antennas.values() {
        for (x0, y0) in ants.iter() {
            for (x1, y1) in ants.iter() {
                if x0 == x1 && y0 == y1 {
                    continue;
                }
                let xf = 2 * x1 - x0;
                let yf = 2 * y1 - y0;

                if xf >= 0 && xf < antennas.width as i32 && yf >= 0 && yf < antennas.height as i32 {
                    antinodes.antinodes[xf as usize + yf as usize * antinodes.width] = true;
                }
            }
        }
    }

    antinodes.antinodes.iter().filter(|x| **x).count()
}

fn part_two(antennas: &AntennaGrid) -> usize {
    let mut antinodes = Antinodes::new(antennas);

    for ants in antennas.antennas.values() {
        for (x0, y0) in ants.iter() {
            for (x1, y1) in ants.iter() {
                if x0 == x1 && y0 == y1 {
                    continue;
                }
                let dir = (x1 - x0, y1 - y0);

                for k in 0.. {
                    let xf = x1 + dir.0 * k;
                    let yf = y1 + dir.1 * k;
                    if xf >= 0
                        && xf < antennas.width as i32
                        && yf >= 0
                        && yf < antennas.height as i32
                    {
                        antinodes.antinodes[xf as usize + yf as usize * antinodes.width] = true;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    antinodes.antinodes.iter().filter(|x| **x).count()
}
struct Antinodes {
    width: usize,
    height: usize,
    antinodes: Vec<bool>,
}

impl Antinodes {
    fn new(antennas: &AntennaGrid) -> Self {
        Antinodes {
            width: antennas.width,
            height: antennas.height,
            antinodes: vec![false; antennas.width * antennas.height],
        }
    }
}

struct AntennaGrid {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<(i32, i32)>>,
}

impl AntennaGrid {
    fn new(filename: &str) -> Self {
        let mut antennas = HashMap::new();

        let data = std::fs::read_to_string(filename).unwrap();

        let height = data.lines().count();
        let width = data.lines().next().unwrap().chars().count();

        for (y, line) in data.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '.' {
                    continue;
                }
                if !antennas.contains_key(&char) {
                    antennas.insert(char, Vec::new());
                }
                antennas.get_mut(&char).unwrap().push((x as i32, y as i32));
            }
        }

        Self {
            width,
            height,
            antennas,
        }
    }
}
