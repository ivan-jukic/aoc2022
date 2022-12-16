use extras::files;

fn main() {
    let input = "src/day08/input";

    let data = decode(input);

    let pt1 = pt1_solution(&data);
    let pt2 = pt2_solution(&data);

    println!("day08: Pt1 = {}", pt1);
    println!("day08: Pt2 = {}", pt2);
}

fn pt1_solution(data: &Vec<Vec<u8>>) -> u32 {
    let rows = data.len();
    let cols = data.get(0).unwrap().len();
    let mut count = 2 * (rows as u32) + 2 * (cols as u32) - 4;

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            let checking = data[i][j];

            // ...
            let mut visibility_left = true;
            for k in 0..j {
                if data[i][k] >= checking {
                    visibility_left = false;
                    break;
                }
            }

            // ...
            let mut visibility_right = true;
            for k in (j + 1)..cols {
                if data[i][k] >= checking {
                    visibility_right = false;
                    break;
                }
            }

            // ...
            let mut visibility_top = true;
            for k in 0..i {
                if data[k][j] >= checking {
                    visibility_top = false;
                    break;
                }
            }

            // ...
            let mut visibility_bottom = true;
            for k in (i + 1)..rows {
                if data[k][j] >= checking {
                    visibility_bottom = false;
                    break;
                }
            }

            if visibility_left || visibility_right || visibility_top || visibility_bottom {
                count += 1;
            }
        }
    }

    count
}

fn pt2_solution(data: &Vec<Vec<u8>>) -> u32 {
    let rows = data.len();
    let cols = data.get(0).unwrap().len();
    let mut scores: Vec<u32> = vec![];

    for i in 1..(rows - 1) {
        for j in 1..(cols - 1) {
            let checking = data[i][j];

            // ...
            let mut count_left = 0;
            for k in (0..j).rev() {
                count_left += 1;
                if data[i][k] >= checking {
                    break;
                }
            }

            // ...
            let mut count_right = 0;
            for k in (j + 1)..cols {
                count_right += 1;
                if data[i][k] >= checking {
                    break;
                }
            }

            // ...
            let mut count_top = 0;
            for k in (0..i).rev() {
                count_top += 1;
                if data[k][j] >= checking {
                    break;
                }
            }

            // ...
            let mut count_bottom = 0;
            for k in (i + 1)..rows {
                count_bottom += 1;
                if data[k][j] >= checking {
                    break;
                }
            }

            scores.push(count_left * count_right * count_top * count_bottom);
        }
    }

    *scores.iter().max().unwrap_or(&0)
}

fn decode(filename: &str) -> Vec<Vec<u8>> {
    files::read_into_vec_of(filename, |line| {
        if !line.is_empty() {
            Some(line.chars().into_iter().map(|c| (c as u8) - 48).collect())
        } else {
            None
        }
    })
}

///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {
    use super::decode;

    #[test]
    fn test_pt1_solution() {
        let data = decode("src/day08/input_test");
        let answer = super::pt1_solution(&data);
        assert_eq!(answer, 21);
    }

    #[test]
    fn test_pt2_solution() {
        let data = decode("src/day08/input_test");
        let answer = super::pt2_solution(&data);
        assert_eq!(answer, 8);
    }
}
