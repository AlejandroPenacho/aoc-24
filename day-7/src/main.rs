fn main() {
    let filename = "input.txt";
    let input: Vec<Operation> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|x| Operation::new(x))
        .collect();

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &[Operation]) -> u64 {
    let mut count = 0;
    for op in input {
        if op.can_be_produced_two_operators() {
            count += op.result;
        }
    }
    count
}

fn part_two(input: &[Operation]) -> u64 {
    let mut count = 0;
    for op in input {
        if op.can_be_produced_three_operators() {
            count += op.result;
        }
    }
    count
}

struct Operation {
    result: u64,
    inputs: Vec<u64>,
    input_lengths: Vec<u64>,
}

impl Operation {
    fn new(line: &str) -> Self {
        let mut split = line.split_whitespace();
        let result = split.next().unwrap();
        let result = result[..(result.len() - 1)].parse().unwrap();

        let inputs = split.clone().map(|x| x.parse().unwrap()).collect();
        let input_lengths = split.clone().map(|x| 10u64.pow(x.len() as u32)).collect();

        Self {
            result,
            inputs,
            input_lengths,
        }
    }

    fn can_be_produced_two_operators(&self) -> bool {
        let n_attempts = 2u64.pow(self.inputs.len() as u32 - 1);

        for attempt in 0..n_attempts {
            let mut key = attempt;
            let mut value = self.inputs[0];
            for i in 0..(self.inputs.len() - 1) {
                if key % 2 == 0 {
                    value += self.inputs[i + 1]
                } else {
                    value *= self.inputs[i + 1]
                }
                key >>= 1;
            }

            if value == self.result {
                return true;
            }
        }

        false
    }

    fn can_be_produced_three_operators(&self) -> bool {
        let n_attempts = 3u64.pow(self.inputs.len() as u32 - 1);

        for attempt in 0..n_attempts {
            let mut key = attempt;
            let mut value = self.inputs[0];
            for i in 0..(self.inputs.len() - 1) {
                if key % 3 == 0 {
                    value += self.inputs[i + 1]
                } else if key % 3 == 1 {
                    value *= self.inputs[i + 1]
                } else {
                    // value = format!("{}{}", value, self.inputs[i + 1]).parse().unwrap();
                    value = value * self.input_lengths[i + 1] + self.inputs[i + 1];
                }
                key /= 3;
            }

            if value == self.result {
                return true;
            }
        }

        false
    }
}
