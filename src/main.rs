mod bgg_api;
mod io;

use exitfailure::ExitFailure;
use failure::{Error, bail};
use bgg_api::Game;
use indicatif::ProgressBar;

const FILE_NAME: &str = "top.json";
const NODES: &str = "nodes.csv";
const EDGES: &str ="edges.csv";

fn main() -> Result<(), ExitFailure> {

    let mut game_list = io::read_games(FILE_NAME)?;
    let res = gather_recs(&mut game_list);
    io::save_games(FILE_NAME, &game_list)?;
    Ok(res?)
}

fn gather_recs(game_list: &mut Vec<Game>) -> Result<(), Error> {
    let bar = ProgressBar::new(game_list.len() as u64);
    while let Some(game) = game_list.pop() {
        match bgg_api::get_rec_list(&game) {
            Err(e) => {
                game_list.push(game);
                bail!(e);
            },
            Ok(edges) => {
                io::append_node(NODES, &game)?;
                io::append_edges(EDGES, &game, &edges)?;
                bar.inc(1);
            }
        }
    }
    bar.finish();
    Ok(())
}
