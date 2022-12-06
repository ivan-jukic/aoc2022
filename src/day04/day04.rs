use extras::{files, logging};

fn main() {
    let input = "src/day04/input";

    let pt1 = pt1_solution(input);
    let pt2 = pt2_solution(input);

    println!("day04: Pt1 = {}", pt1);
    println!("day04: Pt2 = {}", pt2);
}

fn pt1_solution(filename: &str) -> u32 {
    0
}

fn pt2_solution(filename: &str) -> u32 {
    0
}


///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {

    #[test]
    fn test_pt1_solution() {
        let answer = super::pt1_solution("src/day04/input_test");
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_pt2_solution() {
        let answer = super::pt2_solution("src/day04/input_test");
        assert_eq!(answer, 0);
    }
}
