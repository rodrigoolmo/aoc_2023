use std::collections::BTreeMap;
use crate::utils;

type GameSet = BTreeMap<String, i32>;
type GameSets = Vec<GameSet>;

struct Game {
    id: i32,
    game_sets: GameSets
}

struct State {
    games: Vec<Game>
}

impl State {
    fn from_string(input: String) -> Self {
        let mut games: Vec<Game> = Vec::new();
        for line in input.lines() {
            let (game_str, game_sets_str) = line.split_once(": ").unwrap();
            let game_id = utils::to_i32(&game_str.replace("Game ", ""));

            let mut game_sets: GameSets = GameSets::new();
            for game_set_str in game_sets_str.split("; ") {
                let mut game_set: GameSet = GameSet::new();
                for number_color_str in game_set_str.split(", ") {
                    let (number_str, color) = number_color_str.split_once(" ").unwrap();
                    game_set.insert(color.to_string(), utils::to_i32(number_str));
                }
                game_sets.push(game_set);
            }

            games.push(Game {
                id: game_id,
                game_sets: game_sets
            });
        }

        State {
            games: games
        }
    }
}

pub fn part1(input: String) -> String {
    // Input parsing
    let state = State::from_string(input);

    // Solving
    let result: i32 = state.games.iter()
        .filter(|game| {
            game.game_sets.iter().all(|game_set| {
                game_set.get("red").unwrap_or(&0) <= &12 &&
                game_set.get("green").unwrap_or(&0) <= &13 &&
                game_set.get("blue").unwrap_or(&0) <= &14
            })
        })
        .map(|game| game.id)
        .sum();

    result.to_string()
}

pub fn part2(input: String) -> String {
    // Input parsing
    let state = State::from_string(input);

    // Solving
    let result: i32 = state.games.iter()
        .map(|game| {
            vec!["red", "green", "blue"].iter()
                .map(|color| {
                    *game.game_sets.iter()
                        .map(|game_set| {
                            game_set.get(*color).unwrap_or(&0)
                        })
                        .max().unwrap()
                })
                .reduce(|acc, value| acc * value).unwrap()
        })
        .sum();

    result.to_string()
}