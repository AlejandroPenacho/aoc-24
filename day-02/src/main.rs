fn main() {
    let filename = "input.txt";
    let data = std::fs::read_to_string(filename).unwrap();

    println!("{}", part_one(&data));
    println!("{}", part_two(&data));
}

fn find_error_point(seq: &[i64]) -> Vec<usize> {
    let direction = seq.last().unwrap() - seq.first().unwrap();
    let mut output = Vec::new();
    for i in 1..seq.len() {
        let diff = seq[i] - seq[i - 1];
        if diff.abs() > 3 || diff * direction <= 0 {
            if output.last().map_or(false, |x| *x != i - 1) {
                output.push(i - 1);
            }
            output.push(i);
        }
    }
    output
}

fn is_valid(seq: &[i64]) -> bool {
    let direction = seq.last().unwrap() - seq.first().unwrap();
    for i in 1..seq.len() {
        let diff = seq[i] - seq[i - 1];
        if diff.abs() > 3 || diff * direction <= 0 {
            return false;
        }
    }
    true
}

fn part_one(data: &str) -> u64 {
    let mut count = 0;
    for numbers in data.lines().map(|x| {
        x.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }) {
        if is_valid(&numbers) {
            count += 1;
        }
    }
    count
}

fn part_two(data: &str) -> u64 {
    let mut count = 0;
    for numbers in data.lines().map(|x| {
        x.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }) {
        let errors = find_error_point(&numbers);
        if errors.is_empty() {
            count += 1;
        } else {
            for i in 0..numbers.len() {
                let mut new_numbers = numbers.clone();
                new_numbers.remove(i);
                if is_valid(&new_numbers) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}
