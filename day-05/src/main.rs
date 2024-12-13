use std::collections::HashSet;

fn main() {
    let input = Input::new("input.txt");

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &Input) -> u64 {
    let mut sum: u64 = 0;
    for list in input.lists.iter() {
        if input.is_valid(&list) {
            sum += list[(list.len() - 1) / 2] as u64;
        }
    }
    sum
}

fn part_two(input: &Input) -> u64 {
    let mut sum: u64 = 0;
    for list in input.lists.iter() {
        if input.is_valid(&list) {
            continue;
        }
        let reordered_list = input.reorder_list(&list);
        sum += reordered_list[(reordered_list.len() - 1) / 2] as u64;
    }
    sum
}

struct Input {
    rules: HashSet<(u8, u8)>,
    lists: Vec<Vec<u8>>,
}

impl Input {
    fn new(filename: &str) -> Self {
        let input = std::fs::read_to_string(filename).unwrap();
        let mut lines = input.lines();

        let mut rules: HashSet<(u8, u8)> = HashSet::new();

        for line in &mut lines {
            if line.is_empty() {
                break;
            }
            let mut split = line.split("|");
            let a = split.next().unwrap().parse().unwrap();
            let b = split.next().unwrap().parse().unwrap();

            rules.insert((b, a));
        }

        let mut lists = Vec::new();

        for line in lines {
            lists.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        }

        Input { rules, lists }
    }

    fn is_valid(&self, list: &[u8]) -> bool {
        // println!("Analyzing: {:?}", list);
        for i in 0..list.len() {
            for j in i..list.len() {
                if self.rules.contains(&(list[i], list[j])) {
                    // println!("Problem: {}, {}", list[i], list[j]);
                    return false;
                }
            }
        }
        true
    }

    fn reorder_list(&self, list: &[u8]) -> Vec<u8> {
        let mut list = list.to_owned();
        loop {
            let mut repeat = false;
            for i in 0..list.len() {
                for j in i..list.len() {
                    if self.rules.contains(&(list[i], list[j])) {
                        list.swap(i, j);
                        repeat = true;
                    }
                }
            }
            if !repeat {
                break;
            }
        }

        list
    }
}
