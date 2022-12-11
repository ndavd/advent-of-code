#[derive(Clone, Copy)]
enum GameMove {
    Rock = 1,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum GameResult {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

fn get_game_result(player_a_move: GameMove, player_b_move: GameMove) -> GameResult {
    match (player_a_move, player_b_move) {
        (GameMove::Rock, GameMove::Scissors)
        | (GameMove::Paper, GameMove::Rock)
        | (GameMove::Scissors, GameMove::Paper) => GameResult::Win,
        (GameMove::Rock, GameMove::Rock)
        | (GameMove::Paper, GameMove::Paper)
        | (GameMove::Scissors, GameMove::Scissors) => GameResult::Draw,
        _ => GameResult::Lose,
    }
}

fn get_game_move(game_result: GameResult, player_b_move: GameMove) -> GameMove {
    match (game_result, player_b_move) {
        (GameResult::Draw, GameMove::Rock)
        | (GameResult::Win, GameMove::Scissors)
        | (GameResult::Lose, GameMove::Paper) => GameMove::Rock,
        (GameResult::Draw, GameMove::Paper)
        | (GameResult::Win, GameMove::Rock)
        | (GameResult::Lose, GameMove::Scissors) => GameMove::Paper,
        _ => GameMove::Scissors,
    }
}

pub fn get_answer(input: aoc::Input) -> aoc::Answer<i32, i32> {
    let mut points_won = 0;
    let mut real_points_won = 0;

    input.iter().for_each(|prediction| {
        let mut iter = prediction.split_whitespace().into_iter();

        let opponent_move = match iter.next().unwrap() {
            "A" => GameMove::Rock,
            "B" => GameMove::Paper,
            "C" => GameMove::Scissors,
            _ => panic!("Invalid move from opponent!"),
        };

        let guess = iter.next().unwrap();
        let my_move = match guess {
            "X" => GameMove::Rock,
            "Y" => GameMove::Paper,
            "Z" => GameMove::Scissors,
            _ => panic!("Invalid move from me!"),
        };

        points_won += my_move as i32 + get_game_result(my_move, opponent_move) as i32;

        let game_result_prediction = match guess {
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            "Z" => GameResult::Win,
            _ => panic!("Invalid game result!"),
        };
        let my_real_move = get_game_move(game_result_prediction, opponent_move);
        real_points_won += my_real_move as i32 + game_result_prediction as i32;
    });

    aoc::Answer(points_won, real_points_won)
}

fn main() -> Result<(), ()> {
    aoc::AoC::new(2, 15, 12).compute(&get_answer)
}
