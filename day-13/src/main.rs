fn main() {
    let filename = "input.txt";
    println!("{}", part_one(filename));
    println!("{}", part_two(filename));
}

fn part_one(filename: &str) -> isize {
    let input = Input::new(filename);

    let mut count = 0;
    for machine in input.inner.iter() {
        let Some(x) = machine.get_tokens_for_prize() else {
            continue;
        };
        count += x;
    }

    count
}

fn part_two(filename: &str) -> isize {
    let mut input = Input::new(filename);

    for machine in input.inner.iter_mut() {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
    }

    let mut count = 0;
    for machine in input.inner.iter() {
        let Some(x) = machine.get_tokens_for_prize() else {
            continue;
        };
        count += x;
    }

    count
}
#[derive(Debug)]
struct Machine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    fn new(line_a: &str, line_b: &str, line_reward: &str) -> Self {
        let buttons: Vec<(isize, isize)> = [line_a, line_b]
            .iter()
            .map(|x| {
                let plus_x = x.find("+").unwrap();
                let comma = x.find(",").unwrap();
                let plus_y = x.rfind("+").unwrap();
                (
                    x[plus_x + 1..comma].parse().unwrap(),
                    x[plus_y + 1..].parse().unwrap(),
                )
            })
            .collect();

        let left_eq = line_reward.find("=").unwrap();
        let comma = line_reward.find(",").unwrap();
        let right_eq = line_reward.rfind("=").unwrap();

        let prize = (
            line_reward[left_eq + 1..comma].parse().unwrap(),
            line_reward[right_eq + 1..].parse().unwrap(),
        );

        Self {
            button_a: buttons[0],
            button_b: buttons[1],
            prize,
        }
    }

    fn get_tokens_for_prize(&self) -> Option<isize> {
        let denominator = self.button_a.0 * self.button_b.1 - self.button_b.0 * self.button_a.1;
        let a_row = self.button_b.1 * self.prize.0 - self.button_b.0 * self.prize.1;
        let b_row = -self.button_a.1 * self.prize.0 + self.button_a.0 * self.prize.1;

        if a_row % denominator != 0 || b_row % denominator != 0 {
            return None;
        }

        let a = a_row / denominator;
        let b = b_row / denominator;

        if a < 0 || b < 0 {
            return None;
        }

        Some(a * 3 + b)
    }
}

#[derive(Debug)]
struct Input {
    inner: Vec<Machine>,
}

impl Input {
    fn new(filename: &str) -> Self {
        let text = std::fs::read_to_string(filename).unwrap();
        let mut lines = text.lines();

        let mut inner = Vec::new();
        loop {
            let line_a = lines.next().unwrap();
            let line_b = lines.next().unwrap();
            let line_reward = lines.next().unwrap();
            inner.push(Machine::new(line_a, line_b, line_reward));
            if lines.next().is_none() {
                break;
            }
        }

        Self { inner }
    }
}
