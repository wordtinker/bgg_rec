use crate::bgg_api::Game;
use failure::{Error, ResultExt};
use serde_json::{from_str, to_string_pretty};
use std::fs;
use std::io::Write;

const DELIMETER: &str = ";";
const SUBST: &str =".";

pub fn read_games(file_name: &str) -> Result<Vec<Game>, Error> {
    let games = fs::read_to_string(file_name)
        .with_context(|_| format!("Can't open: {}", file_name))?;
    let games: Vec<Game> = from_str(&games)?;
    Ok(games)
}

pub fn save_games(file_name: &str, game_list: &Vec<Game>) -> Result<(), Error> {
    let serialized = to_string_pretty(game_list)?;
    fs::write(file_name, serialized)?;
    Ok(())
}

pub fn append_node(file_name: &str, game: &Game) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)?;
    let name = game.name.replace(DELIMETER, SUBST);
    let node = format!("{}{}{}{}{}{}true\n", game.id, DELIMETER, name, DELIMETER, game.year, DELIMETER);
    file.write(node.as_bytes())?;
    Ok(())
}

pub fn append_edges(file_name: &str, game: &Game, edges: &Vec<usize>) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)?;
    for edge in edges  {
        let edge = format!("{}{}{}\n", game.id, DELIMETER, edge);
        file.write(edge.as_bytes())?;
    }
    Ok(())
}