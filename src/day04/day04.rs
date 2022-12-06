use extras::files;
use regex::Regex;

fn main() {
    let input = "src/day04/input";

    let pt1 = pt1_solution(input);
    let pt2 = pt2_solution(input);

    println!("day04: Pt1 = {}", pt1);
    println!("day04: Pt2 = {}", pt2);
}

type Range = (u32, u32);


fn pt1_solution(filename: &str) -> u32 {
    let data: Vec<(Range, Range)> = files::read_into_vec_of(filename, range_decoder);
    data.into_iter()
        .map(|((la, lb), (ra, rb))| {
            let fst_in_snd = la >= ra && lb <= rb;
            let snd_in_fst = ra >= la && rb <= lb;

            if fst_in_snd || snd_in_fst {
                1
            } else {
                0
            }
        })
        .sum()
}

fn pt2_solution(filename: &str) -> u32 {
    let data: Vec<(Range, Range)> = files::read_into_vec_of(filename, range_decoder);
    data.into_iter()
        .map(|((la, lb), (ra, rb))| {
            let r1: Vec<u32> = (la..=lb).collect();
            let r2: Vec<u32> = (ra..=rb).collect();

            let overlap: bool = r1
                .into_iter()
                .fold(false, |c, v| if !c { r2.contains(&v) } else { c });

            if overlap {
                1
            } else {
                0
            }
        })
        .sum()
}


///
/// ............................................................................

fn range_decoder(line: String) -> Option<(Range, Range)> {
    let separator = Regex::new(r",|\-").expect("ERROR: invalid regex");
    let splits: Vec<u32> = separator
        .split(line.as_str())
        .into_iter()
        .map(|v| v.parse::<u32>().ok())
        .filter_map(|n| n)
        .collect();

    match splits.as_slice() {
        [la, lb, ra, rb] => Some(((*la, *lb), (*ra, *rb))),
        _ => None,
    }
}



///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {

    #[test]
    fn test_range_decoder() {
        let answer = super::range_decoder("1-21,56-103".to_string());
        assert_eq!(answer, Some(((1, 21), (56, 103))));
    }

    #[test]
    fn test_pt1_solution() {
        let answer = super::pt1_solution("src/day04/input_test");
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_pt2_solution() {
        let answer = super::pt2_solution("src/day04/input_test");
        assert_eq!(answer, 4);
    }
}
