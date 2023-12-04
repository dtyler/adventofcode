use std::{fs, str::FromStr};

#[derive(Debug)]
struct GameIteration {
    num_blue: usize,
    num_red: usize,
    num_green: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameIterationError;

#[derive(Debug)]
struct CubeGame {
    game_id: i32,
    games: Vec<GameIteration>,
}

impl FromStr for GameIteration {
    type Err = ParseGameIterationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("DEBUG {}", s);
        todo!()
    }
}

fn parse_line(line: &str) -> CubeGame {
    let (first_digit, _) = line
        .match_indices(|c: char| c.is_numeric())
        .next()
        .expect("Line doesn't contain any digits!");
    let (first_semicolon, _) = line
        .match_indices(':')
        .next()
        .expect("Line doesn't contain any ':' !");
    let game_id = line
        .get(first_digit..first_semicolon)
        .expect("Couldn't get game id")
        .parse::<i32>()
        .expect("game id could not be parsed as number");
    let mut cursor = first_semicolon;
    let mut games: Vec<GameIteration> = vec![];
    while cursor < line.len() {
        let rest_of_line = &line[cursor..];
        if let Some(next_semicolon) = rest_of_line.find(';') {
            let game_slice = &rest_of_line[..next_semicolon];
            println!("rest_of_line: {:?}\npre parse {}", rest_of_line, game_slice);
            games.push(game_slice.parse::<GameIteration>().unwrap());
            cursor = next_semicolon;
        }
    }
    CubeGame { game_id, games }
}

fn parse_file(raw_input: &str) -> i32 {
    raw_input
        .lines()
        .map(parse_line)
        .map(|game| game.game_id)
        .sum::<i32>()
}

fn main() {
    let raw_input = fs::read_to_string("input.txt").expect("expected to read input file");
    let result = parse_file(&raw_input);
    println!("Total is {}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_game_id() {
        let raw_input = fs::read_to_string("test_input").expect("expected to read input file");
        let result = parse_file(&raw_input);
        assert_eq!(result, 15);
    }
}
