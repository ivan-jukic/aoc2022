use extras::files;

fn main() {
    env_logger::init();

    let pt1 = score(process_input("src/day02/input", pt1_decoder));
    let pt2 = score(process_input("src/day02/input", pt2_decoder));

    println!("Pt1 >>> Score: {}", pt1);
    println!("Pt2 >>> Score: {}", pt2);
}

fn pt1_decoder(play: Vec<&str>) -> Option<(Play, Outcome)> {
    match play.as_slice() {
        // Plays for Rock
        ["A", "X"] => Some((Play::Rock, Outcome::Draw)),
        ["A", "Y"] => Some((Play::Paper, Outcome::Win)),
        ["A", "Z"] => Some((Play::Scissors, Outcome::Loss)),

        // Plays for Paper
        ["B", "X"] => Some((Play::Rock, Outcome::Loss)),
        ["B", "Y"] => Some((Play::Paper, Outcome::Draw)),
        ["B", "Z"] => Some((Play::Scissors, Outcome::Win)),

        // Play for Scissors
        ["C", "X"] => Some((Play::Rock, Outcome::Win)),
        ["C", "Y"] => Some((Play::Paper, Outcome::Loss)),
        ["C", "Z"] => Some((Play::Scissors, Outcome::Draw)),
        _ => None,
    }
}

fn pt2_decoder(play: Vec<&str>) -> Option<(Play, Outcome)> {
    match play.as_slice() {
        // Plays for Rock
        ["A", "X"] => Some((Play::Scissors, Outcome::Loss)),
        ["A", "Y"] => Some((Play::Rock, Outcome::Draw)),
        ["A", "Z"] => Some((Play::Paper, Outcome::Win)),

        // Plays for Paper
        ["B", "X"] => Some((Play::Rock, Outcome::Loss)),
        ["B", "Y"] => Some((Play::Paper, Outcome::Draw)),
        ["B", "Z"] => Some((Play::Scissors, Outcome::Win)),

        // Play for Scissors
        ["C", "X"] => Some((Play::Paper, Outcome::Loss)),
        ["C", "Y"] => Some((Play::Scissors, Outcome::Draw)),
        ["C", "Z"] => Some((Play::Rock, Outcome::Win)),
        _ => None,
    }
}


///
/// Local fns
/// ............................................................................

type Moves = Vec<(Play, Outcome)>;

fn score(moves: Moves) -> u32 {
    moves.into_iter().fold(0 as u32, |tot, (play, outcome)| {
        tot + (play as u32) + (outcome as u32)
    })
}

#[derive(Debug, PartialEq)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn process_input<F>(filename: &str, decoder: F) -> Moves
where
    F: Fn(Vec<&str>) -> Option<(Play, Outcome)>,
{
    files::read_into_vec_of(filename, |line| {
        decoder(line.split_whitespace().collect::<Vec<&str>>())
    })
}


///
/// TESTS
/// ............................................................................

#[cfg(test)]
mod test {

    use super::{process_input, pt1_decoder, pt2_decoder, score, Outcome, Play};

    #[test]
    fn test_pt1_decoder() {
        let moves = process_input("src/day02/input_test", pt1_decoder);
        assert_eq!(moves[0], (Play::Paper, Outcome::Win));
        assert_eq!(moves[1], (Play::Rock, Outcome::Loss));
        assert_eq!(moves[2], (Play::Scissors, Outcome::Draw));
    }

    #[test]
    fn test_pt1_result() {
        let score = score(process_input("src/day02/input_test", pt1_decoder));
        assert_eq!(score, 15);
    }

    #[test]
    fn test_pt2_decoder() {
        let moves = process_input("src/day02/input_test", pt2_decoder);
        assert_eq!(moves[0], (Play::Rock, Outcome::Draw));
        assert_eq!(moves[1], (Play::Rock, Outcome::Loss));
        assert_eq!(moves[2], (Play::Rock, Outcome::Win));
    }

    #[test]
    fn test_pt2_result() {
        let score = score(process_input("src/day02/input_test", pt2_decoder));
        assert_eq!(score, 12);
    }
}
