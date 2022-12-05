use extras::{files::read_lines, logging};
use std::path::Path;


fn main() {
    env_logger::init();

    let cals = get_cals("src/day01/input");
    let pt1 = solve_pt1(&cals);
    let pt2 = solve_pt2(cals);

    println!("Pt1 >>> Highest cal: {}", pt1);
    println!("Pt2 >>> Top 3 cal sum: {}", pt2);
}

fn solve_pt1(cals: &Vec<i32>) -> i32 {
    // Get max cals
    match cals.iter().max() {
        Some(val) => *val,
        None => -1,
    }
}

fn solve_pt2(mut cals: Vec<i32>) -> i32 {
    cals.sort_by(|a, b| b.cmp(a));
    cals[0..3].iter().sum()
}


///
/// FNS
/// ............................................................................

fn get_cals(filename: impl AsRef<Path>) -> Vec<i32> {
    let to_num = |v: String| -> Result<i32, String> {
        if v.is_empty() {
            Ok(-1)
        } else {
            v.parse::<i32>().map_err(logging::err_to_str)
        }
    };

    read_lines(filename)
        .into_iter()
        .fold((0, vec![0; 1]), |(idx, mut cals), curr| {
            let curr = curr.map_err(logging::err_to_str).and_then(to_num);
            match curr {
                Ok(-1) => {
                    cals.push(0);
                    (idx + 1, cals)
                }
                Ok(cal) => {
                    cals[idx] += cal;
                    (idx, cals)
                }
                Err(err) => logging::log_and_panic(err),
            }
        })
        .1
}



///
/// TESTS
/// ............................................................................

#[test]
fn test_find_most_cal() {
    let max_cal = solve_pt1(&get_cals("src/day01/input_test"));
    assert_eq!(max_cal, 24000);
}

#[test]
fn test_find_top3_cal_sum() {
    let top3_cal_sum = solve_pt2(get_cals("src/day01/input_test"));
    assert_eq!(top3_cal_sum, 45000);
}
