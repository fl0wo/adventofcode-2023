use std::ops::Index;
use ::phf::{phf_map, Map};

const COLORS: [&str; 3] = ["red", "green", "blue"];
static CUBES: Map<&'static str, u32> = phf_map! {
    "red" => 12,
    "green" => 13,
    "blue" => 14,
};

fn main() {
    let input = include_str!("in.txt");

    let res = input
        .split("\n")
        .map(|line| {

            let index = line.find(":").unwrap();
            let line_split = line.split_at(index);

            let game_id = line_split
                .0
                .replace("Game ","")
                .parse::<u32>()
                .unwrap();

            let games = line_split
                .1
                .replace(": ","")
                .split(|c| c == ';' || c == ',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            return solve(
                game_id,
                games
            );
        })
        .sum::<u32>();

    println!("{}", res);
}

fn solve(game_id:u32, games: Vec<String>) -> u32 {

    for game in games {
        for col in COLORS {
            if game.contains(col) {

                let max = CUBES.index(col);
                let val = game
                    .replace(col,"")
                    .trim()
                    .parse::<u32>()
                    .unwrap();

                if val > *max {
                    return 0;
                }
            }
        }
    }

    return game_id;
}