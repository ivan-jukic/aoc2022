use extras::files;
use itertools::Itertools;

fn main() {
    let input = "src/day06/input";

    let pt1 = pt1_solution(input);
    let pt2 = pt2_solution(input);

    println!("day06: Pt1 = {}", pt1.unwrap_or(0));
    println!("day06: Pt2 = {}", pt2.unwrap_or(0));
}

fn pt1_solution(filename: &str) -> Option<u32> {
    search_unique_four_idx(files::read_into_vec_of(filename, |line| Some(line)).concat())
}

fn pt2_solution(filename: &str) -> Option<u32> {
    search_unique_fourteen_idx(files::read_into_vec_of(filename, |line| Some(line)).concat())
}

#[derive(Clone)]
enum UniqueCount {
    Four = 4,
    Fourteen = 14,
}

fn search_unique_four_idx(val: String) -> Option<u32> {
    search_unique_idx(val, UniqueCount::Four, 0)
}

fn search_unique_fourteen_idx(val: String) -> Option<u32> {
    search_unique_idx(val, UniqueCount::Fourteen, 0)
}

fn search_unique_idx(val: String, unique_cnt: UniqueCount, char_cnt: u32) -> Option<u32> {
    let u_cnt_size = unique_cnt.clone() as usize;
    if val.len() < u_cnt_size {
        None
    } else {
        let fours_unique = val[..u_cnt_size].chars().into_iter().all_unique();
        if fours_unique {
            Some(char_cnt + unique_cnt.clone() as u32)
        } else {
            let new_str = val[1..].to_string().clone();
            search_unique_idx(new_str, unique_cnt, char_cnt + 1)
        }
    }
}

///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {

    #[test]
    fn test_pt1_solution() {
        let answer1 = super::search_unique_four_idx("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
        let answer2 = super::search_unique_four_idx("nppdvjthqldpwncqszvftbrmjlhg".to_string());
        let answer3 =
            super::search_unique_four_idx("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string());
        let answer4 = super::search_unique_four_idx("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string());

        assert_eq!(answer1, Some(5));
        assert_eq!(answer2, Some(6));
        assert_eq!(answer3, Some(10));
        assert_eq!(answer4, Some(11));
    }

    #[test]
    fn test_pt2_solution() {
        let answer1 =
            super::search_unique_fourteen_idx("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string());
        let answer2 = super::search_unique_fourteen_idx("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string());
        let answer3 = super::search_unique_fourteen_idx("nppdvjthqldpwncqszvftbrmjlhg".to_string());
        let answer4 =
            super::search_unique_fourteen_idx("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string());
        let answer5 =
            super::search_unique_fourteen_idx("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string());

        assert_eq!(answer1, Some(19));
        assert_eq!(answer2, Some(23));
        assert_eq!(answer3, Some(23));
        assert_eq!(answer4, Some(29));
        assert_eq!(answer5, Some(26));
    }
}
