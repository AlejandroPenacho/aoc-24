fn main() {
    let filename = "test.txt";
    let filename = "input.txt";
    let data = std::fs::read_to_string(filename).unwrap();
    let mut lists: [Vec<i64>; 2] = [Vec::new(), Vec::new()];
    for line in data.lines() {
        let mut split = line.split_whitespace();
        lists[0].push(split.next().unwrap().parse().unwrap());
        lists[1].push(split.next().unwrap().parse().unwrap());
    }

    println!("{}", first_part(&mut lists));
    println!("{}", second_part(&mut lists));
}

fn first_part(lists: &mut [Vec<i64>; 2]) -> i64 {
    for list in lists.iter_mut() {
        list.sort()
    }
    lists[0]
        .iter()
        .zip(lists[1].iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn second_part(lists: &mut [Vec<i64>; 2]) -> i64 {
    let mut similitude = 0;
    for number in lists[0].iter() {
        let n_ocur = lists[1].iter().filter(|x| x == &number).count() as i64;
        similitude += n_ocur * number;
    }

    similitude
}
