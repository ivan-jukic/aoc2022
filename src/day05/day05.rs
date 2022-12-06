use extras::files;

fn main() {
    let input = "src/day05/input";

    let pt1 = pt1_solution(&mut Stack::input(), input);
    let pt2 = pt2_solution(&mut Stack::input(), input);

    println!("day05: Pt1 = {}", pt1);
    println!("day05: Pt2 = {}", pt2);
}

fn pt1_solution(stack: &mut Stack, filename: &str) -> String {
    let moves: Vec<Move> = files::read_into_vec_of(filename, |line| decode_move(line));
    for mv in moves {
        stack.crate_mover_9000(mv);
    }
    stack.top_crates()
}

fn pt2_solution(stack: &mut Stack, filename: &str) -> String {
    let moves: Vec<Move> = files::read_into_vec_of(filename, |line| decode_move(line));
    for mv in moves {
        stack.crate_mover_9001(mv);
    }
    stack.top_crates()
}


fn decode_move(move_str: String) -> Option<Move> {
    let parts: Vec<&str> = move_str.split(' ').collect();
    match parts.as_slice() {
        ["move", count, "from", from, "to", to] => {
            match (
                count.parse::<usize>(),
                from.parse::<usize>(),
                to.parse::<usize>(),
            ) {
                (Ok(count), Ok(from), Ok(to)) => Some((count, from - 1, to - 1)),
                _ => None,
            }
        }
        _ => {
            println!("Unknown move: {:?}", move_str);
            None
        }
    }
}


///
/// Stack setup & Moves
/// ............................................................................

type Move = (usize, usize, usize);

#[derive(Clone, Debug)]
struct Stack {
    stack: Vec<Vec<char>>,
}

impl Stack {
    // To save a bit of time parsing the input from the file
    pub fn input() -> Stack {
        let stack = vec![
            vec!['R', 'G', 'J', 'B', 'T', 'V', 'Z'],
            vec!['J', 'R', 'V', 'L'],
            vec!['S', 'Q', 'F'],
            vec!['Z', 'H', 'N', 'L', 'F', 'V', 'Q', 'G'],
            vec!['R', 'Q', 'T', 'J', 'C', 'S', 'M', 'W'],
            vec!['S', 'W', 'T', 'C', 'H', 'F'],
            vec!['D', 'Z', 'C', 'V', 'F', 'N', 'J'],
            vec!['L', 'G', 'Z', 'D', 'W', 'R', 'F', 'Q'],
            vec!['J', 'B', 'W', 'V', 'P'],
        ];
        Stack { stack }
    }

    pub fn _test() -> Stack {
        let stack = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        Stack { stack }
    }

    pub fn crate_mover_9000(&mut self, moves: Move) -> &Self {
        self.move_crates(moves, true)
    }

    pub fn crate_mover_9001(&mut self, moves: Move) -> &Self {
        self.move_crates(moves, false)
    }

    fn move_crates(&mut self, (count, from, to): Move, should_reverse: bool) -> &Self {
        let stack = &mut self.stack;
        let src = stack[from].clone();
        let mut dst = stack[to].clone();
        let (src_keep, src_move) = src.split_at(src.len() - count);

        // Remove items from src stack
        stack[from] = src_keep.to_vec();

        // Reverse and add to dst stack
        let mut move_vec: Vec<char> = src_move.to_vec();

        if should_reverse {
            move_vec.reverse();
        }

        dst.append(&mut move_vec);
        stack[to] = dst;

        self
    }

    pub fn top_crates(&self) -> String {
        self.stack
            .clone()
            .into_iter()
            .map(|v| v.last().copied())
            .filter_map(|v| v)
            .collect()
    }
}



///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {

    #[test]
    fn test_pt1_solution() {
        let mut stack = super::Stack::_test();
        let answer = super::pt1_solution(&mut stack, "src/day05/input_test");
        assert_eq!(answer, "CMZ");
    }

    #[test]
    fn test_pt2_solution() {
        let mut stack = super::Stack::_test();
        let answer = super::pt2_solution(&mut stack, "src/day05/input_test");
        assert_eq!(answer, "MCD");
    }
}
