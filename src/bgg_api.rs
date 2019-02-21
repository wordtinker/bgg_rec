use serde_derive::{Serialize, Deserialize};
use serde_json::Value;
use failure::{Error, ResultExt, bail};
use reqwest;

pub fn get_rec_list(game: &Game) -> Result<Vec<usize>, Error> {
    let url = format!("https://api.geekdo.com/api/geekitem/recs?objectid={}", game.id);
    let mut resp = reqwest::get(&url)
        .with_context(|_| format!("could not download page `{}`", url))?;
    let txt = resp.text()?;
    let json: Value = serde_json::from_str(&txt)?;
    let size = &json["numrecs"].to_string();
    let size: usize = size.parse::<usize>()
        .with_context(|_| format!("can't parse total number of recs for `{}`", game.name))?;
    let mut games = Vec::new();
    for i in 0..size {
        let href = &json["recs"][i]["item"]["item"]["href"].to_string();
        games.push(href_to_id(href)?);
    }
    Ok(games)
}

fn href_to_id(href: &str) -> Result<usize, Error> {
    let parts: Vec<&str> = href.rsplit('/').take(2).collect();
    let id = match parts.get(1) {
        Some(x) => x.parse::<usize>()?,
        None => bail!("Can't parse id of the game: {}", href)
    };
    Ok(id)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Game {
    pub id: usize,
    pub name: String,
    pub year: isize
}