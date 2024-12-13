fn main() {
    let filename = "input.txt";
    let grid = Grid::new(filename);

    println!("{}", part_one(&grid));
    println!("{}", part_two(&grid));
}

fn part_one(grid: &Grid) -> u64 {
    let directions = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let mut count = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            for dir in directions {
                let positions: Option<String> = (0..4)
                    .map(|l| (x + dir.0 * l, y + dir.1 * l))
                    .map(|x| grid.get_char(x))
                    .collect();

                if let Some(word) = positions {
                    if word == "XMAS" {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part_two(grid: &Grid) -> u64 {
    let mut count = 0;

    let dir_1 = [(-1, -1), (0, 0), (1, 1)];
    let dir_2 = [(-1, 1), (0, 0), (1, -1)];

    for x in 0..grid.width {
        for y in 0..grid.height {
            let word_1: Option<String> = dir_1
                .iter()
                .map(|d| grid.get_char((x + d.0, y + d.1)))
                .collect();

            let word_2: Option<String> = dir_2
                .iter()
                .map(|d| grid.get_char((x + d.0, y + d.1)))
                .collect();

            let Some(word_1) = word_1 else { continue };
            let Some(word_2) = word_2 else { continue };

            if word_1 != "MAS" && word_1 != "SAM" {
                continue;
            }
            if word_2 != "MAS" && word_2 != "SAM" {
                continue;
            }
            count += 1;
        }
    }
    count
}

struct Grid {
    inner: Vec<char>,
    width: i64,
    height: i64,
}

impl Grid {
    fn new(filename: &str) -> Self {
        let data = std::fs::read_to_string(filename).unwrap();
        let inner: Vec<char> = data.lines().map(|x| x.chars()).flatten().collect();
        let height = data.lines().count() as i64;
        let width = inner.len() as i64 / height;
        Self {
            inner,
            width,
            height,
        }
    }

    fn get_char(&self, pos: (i64, i64)) -> Option<char> {
        if pos.0 < 0 || pos.0 >= self.width || pos.1 < 0 || pos.1 >= self.height {
            return None;
        }

        Some(self.inner[(pos.0 + pos.1 * self.width) as usize])
    }
}
