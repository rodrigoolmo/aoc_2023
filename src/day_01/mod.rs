pub fn part1(input: String) -> String {
    let result: u32 = input.split("\n").map(
        |line| {
            let numbers: Vec<u32> = line.chars()
                .flat_map(|c| c.to_digit(10))
                .collect();

            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        }
    ).sum();

    result.to_string()
}

pub fn part2(input: String) -> String {
    let number_map = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ];

    let result: u32 = input.split("\n").map(
        |line| {
            // From left of the line
            let line_from_left = number_map.iter()
                .map(|(num_label, num)| {
                    (num_label, num, line.find(num_label))
                })
                .min_by_key(|(_num_label, _num, position)| position.unwrap_or(usize::MAX))
                .map(|(num_label, num, _position)| line.replace(num_label, num))
                .unwrap();
            
            // From right of the line
            let line_rev = rev_str(line);
            let line_from_right_rev = number_map.iter()
                .map(|(num_label, num)| {
                    (rev_str(num_label), num, line_rev.find(&rev_str(num_label)))
                })
                .min_by_key(|(_num_label, _num, position)| position.unwrap_or(usize::MAX))
                .map(|(num_label, num, _position)| line_rev.replace(&num_label, num))
                .unwrap();
            let line_from_right = rev_string(&line_from_right_rev);

            let first_num: u32 = line_from_left.chars()
                .flat_map(|c| c.to_digit(10))
                .next().unwrap();

            let last_num: u32 = line_from_right.chars()
                .flat_map(|c| c.to_digit(10))
                .last().unwrap();

            first_num * 10 + last_num
        }
    ).sum();

    result.to_string()
}

fn rev_string(str: &String) -> String {
    str.chars().rev().collect::<String>()
}

fn rev_str(str: &str) -> String {
    str.chars().rev().collect::<String>()
}