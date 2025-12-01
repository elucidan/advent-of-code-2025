use std::fs;

fn main() {
    let result = read_in_file();
    let mut safe_value = 50;
    let mut counter = 0;
    for index in 0..result.0.len() {
        let direction = result.0[index];
        let value = result.1[index];
        safe_value = adjust_dial(&safe_value, direction, value);
        if safe_value == 0 {
            counter += 1;
        }
    }
    println!("{}", counter);
}

fn adjust_dial(dial_value: &i32, direction: char, value: i32) -> i32 {
    let holding: i32;

    if direction == 'L' {
        holding = (dial_value - value) % 100;
    } else {
        holding = (dial_value + value) % 100;
    }

    if holding < 0 {
        let retval = 100 + holding;
        return retval;
    }
    if holding >= 100 {
        let retval = holding - 100;
        return retval;
    } else {
        return holding;
    }
}

fn read_in_file() -> (Vec<char>, Vec<i32>) {
    let binding = fs::read_to_string("input.txt").unwrap();
    let mut direction_list: Vec<char> = Vec::new();
    let mut value_list: Vec<i32> = Vec::new();
    for line in binding.lines() {
        let (direction, value) = line.split_at(1);
        direction_list.push(direction.parse::<char>().unwrap());
        value_list.push(value.parse::<i32>().unwrap());
    }
    return (direction_list, value_list);
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_adjust_dial() {
        let dial1 = 5;
        let result = adjust_dial(&dial1, 'L', 5);

        assert_eq!(0, result);
    }

    #[test]
    fn test_adjust_dial_right() {
        let dial: i32 = 95;
        let result = adjust_dial(&dial, 'R', 5);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_adjust_dial_without_moderation() {
        let dial: i32 = 10;
        let result = adjust_dial(&dial, 'L', 500);
        assert_eq!(result, 10);
    }
    #[test]
    fn test_adjust_dial_over_1_spin() {
        let dial: i32 = 85;
        let result = adjust_dial(&dial, 'L', 444);
        assert_eq!(result, 41);
    }
    #[test]
    fn test_mod() {
        let retval = 444 % 100;
        assert_eq!(retval, 44);
    }
}
