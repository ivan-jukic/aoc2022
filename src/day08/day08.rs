use extras::files;

fn main() {
    let input = "src/day08/input";

    let pt1 = pt1_solution(input);
    let pt2 = pt2_solution(input);

    println!("day08: Pt1 = {}", pt1);
    println!("day08: Pt2 = {}", pt2);
}

fn pt1_solution(filename: &str) -> u32 {
    let _data: Vec<Vec<u8>> = files::read_into_vec_of(filename, |line| {
        if !line.is_empty() {
            Some(line.chars().into_iter().map(|c| (c as u8) - 64u8).collect())
        } else {
            None
        }
    });
    0
}

fn pt2_solution(filename: &str) -> u32 {
    let _data: Vec<String> = files::read_into_vec_of(filename, |line| Some(line));
    0
}

///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {

    #[test]
    fn test_pt1_solution() {
        let answer = super::pt1_solution("src/day08/input_test");
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_pt2_solution() {
        let answer = super::pt2_solution("src/day08/input_test");
        assert_eq!(answer, 0);
    }
}
