use extras::files;

fn main() {
    let input = "src/day06/input";

    let pt1 = pt1_solution(input);
    let pt2 = pt2_solution(input);

    println!("day06: Pt1 = {}", pt1);
    println!("day06: Pt2 = {}", pt2);
}

fn pt1_solution(filename: &str) -> u32 {
    let buff: String = files::read_into_vec_of(filename, |line| Some(line)).concat();
    0
}

fn pt2_solution(filename: &str) -> u32 {
    let _buff: String = files::read_into_vec_of(filename, |line| Some(line)).concat();
    0
}

fn calc_marker_offset(buff: String) -> u32 {
    buff.chars()
        .into_iter()
        .fold((0, vec![]), |(cnt, prev), c| -> (u32, Vec<char>) {
            // ...
            (cnt, prev)
        })
        .0
}


///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {

    #[test]
    fn test_pt1_solution() {
        let answer = 42;
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_pt2_solution() {
        let answer = 42;
        assert_eq!(answer, 0);
    }
}
