fn main() {
    let filename = "test.txt";
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
    let mut data = obtain_part_two_input(filename);
    println!("{:?}", data);

    let mut output = Vec::new();

    let mut index = 0;
    let mut remaining_blank_space = None;
    loop {
        let next_block;
        if let Some(x) = remaining_blank_space {
            next_block = DiskSpace::Empty(x);
            remaining_blank_space = None;
        } else {
            if data.len() <= index {
                break;
            }
            next_block = data[index].clone();
            index += 1;
        }

        match next_block {
            DiskSpace::File(_, _) => output.push(next_block),
            DiskSpace::Empty(space) => {
                let mut block_allocated = false;
                for i in (0..data.len()).rev() {
                    let DiskSpace::File(file_size, _) = data[i].clone() else {
                        continue;
                    };
                    if file_size > space {
                        continue;
                    }
                    output.push(data[i].clone());
                    data.remove(i);
                    let remaining_space = space - file_size;
                    if remaining_space != 0 {
                        remaining_blank_space = Some(remaining_space);
                    }
                    block_allocated = true;
                    break;
                }
                if !block_allocated {
                    output.push(DiskSpace::Empty(space));
                }
            }
        }
    }

    println!("{:?}", output);

    4
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
