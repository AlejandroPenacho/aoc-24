use std::collections::HashMap;

fn main() {
    let filename = "input.txt";
    let stones: Vec<usize> = std::fs::read_to_string(filename)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // let mut rocks: Vec<usize> = vec![125, 17];
    println!("{:?}", part_one(stones.clone()));
    println!("{:?}", part_two(&stones));
}

fn part_one(mut rocks: Vec<usize>) -> usize {
    for i in 0..25 {
        // println!("Blink {i}");
        for j in 0..rocks.len() {
            if rocks[j] == 0 {
                rocks[j] = 1;
            } else if rocks[j].ilog(10) % 2 == 1 {
                let n_digits = rocks[j].ilog(10) + 1;
                let first_number = rocks[j] / 10usize.pow(n_digits / 2);
                let second_number = rocks[j] - first_number * 10usize.pow(n_digits / 2);
                rocks[j] = first_number;
                rocks.push(second_number);
            } else {
                rocks[j] *= 2024;
            }
        }
        // println!("N rocks: {}", rocks.len());
        // println!("N rocks: {:?}", rocks);
    }
    rocks.len()
}

fn part_two(stone_vector: &Vec<usize>) {
    let mut stones = Stones::from_vector(stone_vector);

    for i in 0..75 {
        let mut new_stones = Stones::new();
        println!("Blink {i}");
        for (&stone, &quantity) in stones.inner.iter() {
            if stone == 0 {
                new_stones.add_stone(1, quantity);
            } else if stone.ilog(10) % 2 == 1 {
                let n_digits = stone.ilog(10) + 1;
                let first_number = stone / 10usize.pow(n_digits / 2);
                let second_number = stone - first_number * 10usize.pow(n_digits / 2);
                new_stones.add_stone(first_number, quantity);
                new_stones.add_stone(second_number, quantity);
            } else {
                new_stones.add_stone(stone * 2024, quantity);
            }
        }
        stones = new_stones;
        println!(
            "N rocks: {}",
            stones.inner.iter().map(|(_, x)| x).sum::<usize>()
        );
        // println!("N rocks: {:?}", rocks);
    }
}

struct Stones {
    inner: HashMap<usize, usize>,
}

impl Stones {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn from_vector(stone_vector: &[usize]) -> Self {
        let mut out = Self {
            inner: HashMap::new(),
        };
        for stone in stone_vector {
            out.add_stone(*stone, 1);
        }
        out
    }

    fn add_stone(&mut self, stone: usize, quantity: usize) {
        use std::collections::hash_map::Entry;
        let mut entry = self.inner.entry(stone);
        match entry {
            Entry::Vacant(_) => {
                entry.or_insert(quantity);
            }
            Entry::Occupied(_) => {
                entry.and_modify(|x| *x += quantity);
            }
        };
    }
}
