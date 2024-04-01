use std::{io::{BufReader, BufRead}, fs::File, fmt::Display, num::ParseIntError};
use std::str::FromStr;
/// A draw represents the result of the elve taking cubes out of the bag
/// A game can have many draws and for each game the number of cubes of each color is recorded

#[derive(Debug, PartialEq)]
enum SyntaxError {
    UndefinedColor,
    WrongTokenCount,
    TokenNotInteger,
    WrongTokenCountGID,
    InvalidSectionCount,
    LiteralNotFound(&'static str),
}

impl From<ParseIntError> for SyntaxError {
    fn from(_: ParseIntError) -> Self {
        Self::TokenNotInteger
    }
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndefinedColor => write!(f, "The color defined in a draw can only be 'red', 'green', or 'blue'"),
            Self::WrongTokenCount => write!(f, ""),
            Self::InvalidSectionCount => write!(f, "Invalid Section count. A game needs to have a ID and a Draws section"),
            Self::WrongTokenCountGID => write!(f, "The Game field needs to consist of the phrase 'Game' followed by a number"),
            Self::TokenNotInteger => write!(f, "An integer was expected here"),
            Self::LiteralNotFound(lit) => write!(f, "The literal '{}' was not found", lit),
        }
    }
}

#[derive(PartialEq, Debug)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(SyntaxError::UndefinedColor)
        }
    }
}

#[derive(PartialEq, Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}
impl Draw {
    fn new() -> Self {
        Draw{ red: 0, green: 0, blue: 0 }
    }
}

impl FromStr for Draw {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut d = Draw::new();
        let sections: Vec<_> = s.split(',').collect();
        if sections.len() > 3 {
            return Err(SyntaxError::WrongTokenCount);
        }
        for section in sections.into_iter() {
            let parts: Vec<&str> = section.trim().split(' ').collect();
            if parts.len() != 2 {
                return Err(SyntaxError::WrongTokenCount);
            }
            let col = parts[1].parse::<CubeColor>()?;
            let num_cubes = parts[0].parse::<u32>()?;
            match col {
                CubeColor::Red => d.red = num_cubes,
                CubeColor::Green => d.green = num_cubes,
                CubeColor::Blue => d.blue = num_cubes,
            }
        }
        Ok(d)
    }
}

/// A game has an ID and a sequence of draws that are done for each game
#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections: Vec<_> = s.split(':').collect();
        if sections.len() != 2 {
            return Err(SyntaxError::InvalidSectionCount);
        }
        let draws: Result<Vec<Draw>, SyntaxError> = sections[1].trim().split(';').map(|s| s.parse::<Draw>()).collect();
        let game_id_tokens: Vec<_> = sections[0].trim().split(' ').collect();
        if game_id_tokens.len() != 2 {
            return Err(SyntaxError::WrongTokenCountGID);
        }
        if game_id_tokens[0] != "Game" {
            return Err(SyntaxError::LiteralNotFound("Game"));
        }
        let game_id = game_id_tokens[1].parse().unwrap();
        Ok(Game { id: game_id, draws: draws? })
    }
}

/// checks if the draws from a game are within the limits prescribed as the function arguments
fn check_game_within_limits(game: &Game, red_blocks: u32, blue_blocks: u32, green_blocks: u32) -> bool {
    for draw in game.draws.iter() {
        if draw.red > red_blocks || draw.green > green_blocks || draw.blue > blue_blocks {
            return false
        }
    }
    return true
}

fn calculate_power_of_set(game: &Game) -> usize {
    let min_red: u32 = game.draws.iter().map(|g| g.red).max().unwrap_or(0);
    let min_green: u32 = game.draws.iter().map(|g| g.green).max().unwrap_or(0);
    let min_blue: u32 = game.draws.iter().map(|g| g.blue).max().unwrap_or(0);
    min_red as usize * min_green as usize * min_blue as usize

}

pub fn solve_day_2(freader: BufReader<File>, part_two: bool) {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    let mut games: Vec<Game> = Vec::new();
    for line in freader.lines() {
        if let Ok(line) = line {
            match line.parse::<Game>() {
                Ok(game) => games.push(game),
                Err(e) => {
                    println!("Syntax error: {}", e);
                    return;
                }
            }
        } else {
            println!("Error encountered during reading of the file");
            return;
        }
    }
    if !part_two {
        // now that we have all the games, it is time to actually process them
        let sum_of_ids: u32 = games.iter().filter(|g| check_game_within_limits(g, MAX_RED, MAX_BLUE, MAX_GREEN)).map(|g| g.id).sum();
        println!("The sum of the ids is: {}", sum_of_ids);
    } else {
        let sum_of_powers: usize = games.iter().map(calculate_power_of_set).sum();
        println!("The sum of the powers is: {}", sum_of_powers);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str_valid() {
        assert_eq!("red".parse::<CubeColor>(), Ok(CubeColor::Red));
        assert_eq!("green".parse::<CubeColor>(), Ok(CubeColor::Green));
        assert_eq!("blue".parse::<CubeColor>(), Ok(CubeColor::Blue));
        
        // Test with leading and trailing whitespaces
        assert_eq!("   red   ".parse::<CubeColor>(), Ok(CubeColor::Red));
    }

    #[test]
    fn test_from_str_invalid() {
        // Test with an invalid color
        assert!("invalid_color".parse::<CubeColor>().is_err());

        // Test with empty string
        assert!("".parse::<CubeColor>().is_err());

        // Test with whitespaces only
        assert!("   ".parse::<CubeColor>().is_err());
    }

    #[test]
    fn test_draw_from_str_valid() {
        // Test valid input with all three colors
        assert_eq!("1 red, 2 green, 3 blue".parse::<Draw>(), Ok(Draw { red: 1, green: 2, blue: 3 }));

        // Test valid input with only one color
        assert_eq!("5 red".parse::<Draw>(), Ok(Draw { red: 5, green: 0, blue: 0 }));

        // Test valid input with leading and trailing whitespaces
        assert_eq!("  2 red, 4 green   ,  6 blue  ".parse::<Draw>(), Ok(Draw { red: 2, green: 4, blue: 6 }));
    }

    #[test]
    fn test_draw_from_str_invalid() {
        // Test invalid input with more than three colors
        assert!("1 red, 2 green, 3 blue, 4 yellow".parse::<Draw>().is_err());

        // Test invalid input with an unknown color
        assert!("2 red, 3 invalid, 4 blue".parse::<Draw>().is_err());

        // Test invalid input with missing cube count
        assert!("red, 2 green, 3 blue".parse::<Draw>().is_err());

        // Test invalid input with missing color
        assert!("1, 2 green, 3 blue".parse::<Draw>().is_err());

        // Test invalid input with wrong number of tokens in a section
        assert!("1 red, 2 green 3 blue".parse::<Draw>().is_err());
    }
}
