use std::{fs, str::FromStr};

#[derive(Debug)]
struct GameIteration {
    num_blue: i32,
    num_red: i32,
    num_green: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameIterationError {
    msg: String,
}

#[derive(Debug)]
struct CubeGame {
    game_id: i32,
    games: Vec<GameIteration>,
}

impl FromStr for GameIteration {
    type Err = ParseGameIterationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("DEBUG '{}'", s);
        if s.trim().is_empty() {
            return Err(ParseGameIterationError {
                msg: String::from("Given empty string"),
            });
        }
        let mut new_game_iter = GameIteration {
            num_blue: 0,
            num_green: 0,
            num_red: 0,
        };
        for iter in s.split(',') {
            let cleaned = iter.trim();
            let sep_index = cleaned
                .find(' ')
                .expect("every game iteration must be of the form '<num> <color>' ");
            let number = cleaned[..sep_index].parse::<i32>().expect("parse failed");
            let color = &cleaned[(sep_index + 1)..];
            match color {
                "blue" => new_game_iter.num_blue = number,
                "red" => new_game_iter.num_red = number,
                "green" => new_game_iter.num_red = number,
                _ => {
                    return Err(ParseGameIterationError {
                        msg: format!(
                            "unexpected color '{}' given. sep_index was {}",
                            color, sep_index
                        ),
                    })
                }
            }
        }
        Ok(new_game_iter)
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
    let mut cursor = first_semicolon + 1;
    let mut games: Vec<GameIteration> = vec![];
    loop {
        let rest_of_line = &line[cursor..];
        if let Some(next_semicolon) = rest_of_line.find(';') {
            let game_slice = &rest_of_line[..next_semicolon];
            // println!("rest_of_line: {:?}\npre parse {}", rest_of_line, game_slice);
            match game_slice.parse::<GameIteration>() {
                Ok(game) => games.push(game),
                Err(error) => panic!("{}", error.msg),
            }
            cursor += next_semicolon + 1;
        } else {
            break;
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
