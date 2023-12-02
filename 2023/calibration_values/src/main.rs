use std::fs;

#[derive(Debug)]
struct Number {
    value: usize,
    index: usize,
}

const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn parse_line(line: &str) -> [usize; 2] {
    let mut found_words = Vec::<Number>::new();
    for (num, number_word) in NUMBER_WORDS.iter().enumerate() {
        //  Search using text
        for (index, _) in line.match_indices(number_word) {
            found_words.push(Number {
                value: num + 1,
                index,
            });
        }
        // Search using digit
        for (index, _) in line.match_indices(format!("{}", num + 1).as_str()) {
            found_words.push(Number {
                value: num + 1,
                index,
            });
        }
    }
    let found_words_slice = found_words.as_mut_slice();
    found_words_slice.sort_by_key(|num| num.index);
    println!("Line: {}\nBreakdown{:?}", line, found_words_slice);
    [
        found_words_slice[0].value,
        found_words_slice[found_words_slice.len() - 1].value,
    ]
}

fn parse_file(raw_input: &str) -> i32 {
    let parsed = raw_input.lines().map(|line| {
        let [first_num, last_num] = parse_line(line);
        println!("Result: {:?}{:?}", first_num, last_num);
        format!("{}{}", first_num, last_num)
            .parse::<i32>()
            .expect("concat of first and last digit of line will be parable as int")
    });
    parsed.sum::<i32>()
}

fn main() {
    let raw_input = fs::read_to_string("input.txt").expect("expected to read input file");
    let result = parse_file(&raw_input);
    println!("Total is {}", result)
}

#[cfg(test)]
mod tests {
    use crate::parse_line;

    #[test]
    fn it_finds_words() {
        let result = parse_line("six1");
        assert_eq!(result, [6, 1]);
    }
    #[test]
    fn it_handles_duplicate_digits() {
        let result = parse_line("six12");
        assert_eq!(result, [6, 2]);
    }
    #[test]
    fn it_handles_digits_and_words() {
        let result = parse_line("six16");
        assert_eq!(result, [6, 6]);
    }
    #[test]
    fn it_handles_only_single_digit() {
        let result = parse_line("9vvcsgxq");
        assert_eq!(result, [9, 9]);
    }
    #[test]
    fn it_handles_thisthing() {
        let result = parse_line("oneight");
        assert_eq!(result, [1, 8]);
    }
    #[test]
    fn it_handles_digits_twice() {
        let result = parse_line("l9649twothree");
        assert_eq!(result, [9, 3]);
    }
}
