fn main() {
    let filename = "input.txt";
    let grid = Grid::new(filename);
    let mut neighbors: Vec<Vec<usize>> = vec![Vec::new(); grid.width * grid.height];

    for i in 0..grid.width {
        for j in 0..grid.height {
            let value = grid.value[i + j * grid.width];

            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (x, y) in dirs.map(|(d_i, d_j)| (i as i64 + d_i, j as i64 + d_j)) {
                if x < 0 || x >= grid.width as i64 {
                    continue;
                }
                if y < 0 || y >= grid.height as i64 {
                    continue;
                }
                if grid.value[x as usize + y as usize * grid.width] != value + 1 {
                    continue;
                }
                neighbors[i + j * grid.width].push(x as usize + y as usize * grid.width);
            }
        }
    }
    println!("{}", part_one(&grid, &neighbors));
    println!("{}", part_two(&grid, &neighbors));
}

fn part_one(grid: &Grid, neighbors: &[Vec<usize>]) -> usize {
    let n_peaks = grid.value.iter().filter(|x| x == &&9).count();
    let mut access_peaks: Vec<PeakRegistry> = vec![PeakRegistry::new(n_peaks); grid.value.len()];

    let mut c_peak = 0;
    for i in 0..grid.value.len() {
        if grid.value[i] == 9 {
            access_peaks[i].inner[c_peak] = true;
            c_peak += 1;
        }
    }

    for value in (0..9).rev() {
        for i in 0..grid.value.len() {
            if grid.value[i] != value {
                continue;
            }
            for n in neighbors[i].iter() {
                let other = access_peaks[*n].clone();
                access_peaks[i].fuse(&other)
            }
        }
    }

    let mut score = 0;
    for i in 0..grid.value.len() {
        if grid.value[i] != 0 {
            continue;
        }
        score += access_peaks[i].inner.iter().filter(|x| **x).count();
    }
    score
}

fn part_two(grid: &Grid, neighbors: &[Vec<usize>]) -> usize {
    let mut trail_score: Vec<usize> = vec![0; grid.value.len()];

    for i in 0..grid.value.len() {
        if grid.value[i] == 9 {
            trail_score[i] = 1;
        }
    }

    for value in (0..9).rev() {
        for i in 0..grid.value.len() {
            if grid.value[i] != value {
                continue;
            }
            for n in neighbors[i].iter() {
                trail_score[i] += trail_score[*n];
            }
        }
    }

    let mut score = 0;
    for i in 0..grid.value.len() {
        if grid.value[i] != 0 {
            continue;
        }
        score += trail_score[i];
    }
    score
}
#[derive(Debug, Clone)]
struct PeakRegistry {
    inner: Vec<bool>,
}

impl PeakRegistry {
    fn new(n_peaks: usize) -> Self {
        Self {
            inner: vec![false; n_peaks],
        }
    }

    fn fuse(&mut self, other: &Self) {
        for (own, oth) in self.inner.iter_mut().zip(other.inner.iter()) {
            *own = *own || *oth;
        }
    }
}

struct Grid {
    value: Vec<u8>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(filename: &str) -> Self {
        let text = std::fs::read_to_string(filename).unwrap();
        let height = text.lines().count();
        let width = text.lines().next().unwrap().chars().count();
        let value: Vec<u8> = text
            .lines()
            .map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as u8))
            .flatten()
            .collect();

        Self {
            value,
            height,
            width,
        }
    }
}
