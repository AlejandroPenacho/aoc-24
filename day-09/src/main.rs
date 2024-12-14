fn main() {
    let filename = "input.txt";
    // println!("{}", part_one(filename));
    println!("{}", part_two(filename));
}

fn part_one(filename: &str) -> u64 {
    let data = obtain_ordered_input(filename);

    let mut right_index = data.len();
    let mut output = Vec::new();

    for i in 0..data.len() {
        if right_index <= i {
            break;
        }
        match data[i] {
            Some(x) => {
                output.push(x);
            }
            None => loop {
                right_index -= 1;
                if right_index <= i {
                    break;
                }
                if let Some(x) = data[right_index] {
                    output.push(x);
                    break;
                }
            },
        }
    }
    let final_sum: u64 = output.iter().enumerate().map(|(a, b)| a as u64 * b).sum();
    final_sum
}

#[derive(Debug, Clone)]
enum DiskSpace {
    File(u64, u64),
    Empty(u64),
}

fn part_two(filename: &str) -> u64 {
    let mut data: Vec<DiskSpace> = obtain_part_two_input(filename);

    let mut i = data.len();
    loop {
        i -= 1;
        if i == 0 {
            break;
        }

        let val = data[i].clone();
        let DiskSpace::File(file_size, _) = val.clone() else {
            continue;
        };

        for j in 0..i {
            let DiskSpace::Empty(empty_size) = data[j] else {
                continue;
            };
            if empty_size < file_size {
                continue;
            }

            if empty_size == file_size {
                data[j] = val;
            } else {
                data[j] = DiskSpace::Empty(empty_size - file_size);
                data.insert(j, val);
                i += 1;
            }
            data[i] = DiskSpace::Empty(file_size);
            break;
        }
    }

    // println!("{:?}", data);

    let mut value = 0;
    let mut index = 0;
    for space in data.iter() {
        match space {
            DiskSpace::Empty(x) => index += x,
            DiskSpace::File(file_size, file_id) => {
                value += file_id * file_size * index + file_id * file_size * (file_size - 1) / 2;
                index += file_size;
            }
        }
    }

    value
}

fn obtain_part_two_input(filename: &str) -> Vec<DiskSpace> {
    let input = std::fs::read_to_string(filename).unwrap();

    let input_numbers: Vec<u64> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();

    let mut data = vec![];
    let mut current_id = 0;
    let mut next_is_file = true;
    for number in input_numbers.iter() {
        if next_is_file {
            data.push(DiskSpace::File(*number, current_id));
            next_is_file = false;
            current_id += 1;
        } else {
            data.push(DiskSpace::Empty(*number));
            next_is_file = true;
        }
    }

    data
}

fn obtain_ordered_input(filename: &str) -> Vec<Option<u64>> {
    let input = std::fs::read_to_string(filename).unwrap();

    let input_numbers: Vec<u64> = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();

    let total_chars: usize = input_numbers.iter().sum::<u64>() as usize;

    let mut data = vec![None; total_chars];
    let mut current_index = 0;
    let mut current_id = 0;
    let mut next_is_file = true;
    for number in input_numbers.iter() {
        if !next_is_file {
            next_is_file = true;
            current_index += number;
            continue;
        }

        for i in current_index..(current_index + number) {
            data[i as usize] = Some(current_id);
        }
        next_is_file = false;
        current_index += number;
        current_id += 1;
    }
    data
}
