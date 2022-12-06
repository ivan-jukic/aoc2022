use extras::files;
use itertools::Itertools;

fn main() {
    let input = "src/day03/input";
    let pt1 = pt1_solution(input);
    let pt2 = pt2_solution(input);

    println!("Day 03: Pt1 = {}", pt1);
    println!("Day 03: Pt2 = {}", pt2);
}

fn pt1_solution(filename: &str) -> u32 {
    let data: Vec<u32> = files::read_into_vec_of(filename, |line| {
        let half_len: usize = line.len() / 2 as usize;
        let total = score(&line[0..half_len], &line[half_len..]);
        Some(total)
    });

    data.into_iter().sum()
}

fn pt2_solution(filename: &str) -> u32 {
    let data = files::read_into_vec_of(filename, |line| Some(line));
    let data_str: Vec<&str> = data.iter().map(|s| &**s).collect();

    to_tripples(data_str, vec![])
        .into_iter()
        .map(|(fst, snd, trd)| score_in_three(fst, snd, trd))
        .sum()
}


///
/// ............................................................................

fn to_value_vec(val: &str) -> Vec<u32> {
    val.chars()
        .into_iter()
        .map(|c| {
            if c.is_uppercase() {
                (c as u32) - 38
            } else {
                (c as u32) - 96
            }
        })
        .collect()
}

fn score(fst: &str, snd: &str) -> u32 {
    let fst = to_value_vec(fst);
    let snd = to_value_vec(snd);
    fst.into_iter().unique().filter(|v| snd.contains(v)).sum()
}

fn score_in_three(fst: &str, snd: &str, trd: &str) -> u32 {
    let fst = to_value_vec(fst);
    let snd = to_value_vec(snd);
    let trd = to_value_vec(trd);
    fst.into_iter()
        .unique()
        .filter(|v| snd.contains(v) && trd.contains(v))
        .sum()
}

fn to_tripples<'a>(
    data: Vec<&'a str>,
    res: Vec<(&'a str, &'a str, &'a str)>,
) -> Vec<(&'a str, &'a str, &'a str)> {
    if data.len() < 3 {
        res
    } else {
        let (left, right) = data.split_at(3);
        if let [fst, snd, trd] = left {
            let mut mres = res.clone();
            mres.push((fst, snd, trd));
            to_tripples(right.to_vec(), mres)
        } else {
            res
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
        let answer = super::pt1_solution("src/day03/input_test");
        assert_eq!(answer, 157);
    }

    #[test]
    fn test_pt2_solution() {
        let answer = super::pt2_solution("src/day03/input_test");
        assert_eq!(answer, 70);
    }
}
