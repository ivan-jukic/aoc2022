use extras::files;

fn main() {
    let data = decode("src/day09/input");

    let pt1 = pt1_solution(&data);
    let pt2 = pt2_solution(&data);

    println!("day09: Pt1 = {}", pt1);
    println!("day09: Pt2 = {}", pt2);
}

fn pt1_solution(_data: &Vec<String>) -> u32 {
    0
}

fn pt2_solution(_data: &Vec<String>) -> u32 {
    0
}

fn decode(filename: &str) -> Vec<String> {
    files::read_into_vec_of(filename, |line| Some(line))
}

///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {
    use super::decode;

    #[test]
    fn test_pt1_solution() {
        let data = decode("src/day09/input_test");
        let answer = super::pt1_solution(&data);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_pt2_solution() {
        let data = decode("src/day09/input_test");
        let answer = super::pt2_solution(&data);
        assert_eq!(answer, 0);
    }
}
