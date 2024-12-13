fn main() {
    let filename = "input.txt";
    println!("{}", part_a(filename));
    println!("{}", part_b(filename));
}

fn part_a(filename: &str) -> i64 {
    let data = std::fs::read_to_string(filename).unwrap();

    let mut sum = 0;

    let mut i = 0;
    loop {
        let Some(mul_point) = data[i..].find("mul(") else {
            break;
        };

        let Some(final_paren) = data[i + mul_point..].find(")") else {
            break;
        };

        let in_string = &data[(i + mul_point + 4)..(i + mul_point + final_paren)];

        sum += run_multiplication(in_string).unwrap_or(0);

        i += mul_point + 1;
    }

    sum
}

fn part_b(filename: &str) -> i64 {
    let data = std::fs::read_to_string(filename).unwrap();

    let mut sum = 0;

    let mut active = true;

    let mut i = 0;

    loop {
        let do_point = data[i..].find("do()");
        let dont_point = data[i..].find("don't()");
        let mul_point = data[i..].find("mul(");

        let Some(min_point) = [do_point, dont_point, mul_point]
            .iter()
            .filter_map(|&x| x)
            .min()
        else {
            break;
        };

        if do_point.map_or(false, |x| x == min_point) {
            active = true;
        } else if dont_point.map_or(false, |x| x == min_point) {
            active = false;
        } else {
            let mul_point = mul_point.unwrap();
            let Some(final_paren) = data[i + mul_point..].find(")") else {
                break;
            };

            let in_string = &data[(i + mul_point + 4)..(i + mul_point + final_paren)];

            if active {
                sum += run_multiplication(in_string).unwrap_or(0);
            }
        }

        i += min_point + 1;
    }

    sum
}

fn run_multiplication(x: &str) -> Option<i64> {
    let mut numbers = x.split(",");
    let a = numbers.next().map(|x| x.parse::<i64>().ok()).flatten()?;
    let b = numbers.next().map(|x| x.parse::<i64>().ok()).flatten()?;
    Some(a * b)
}
