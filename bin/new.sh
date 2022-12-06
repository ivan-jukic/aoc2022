#!/bin/bash

src_dir=src
re='^[0-9]+$'
max_day=1
for entry in "$src_dir"/*; do
    day_no="${entry: -2}"
    if [[ $day_no =~ $re ]]; then
        day=$((day_no+1))
        if [[ $day -gt $max_day ]]; then
            max_day=$day
        fi
    fi
done

if [ $max_day -lt 10 ]
then new_day="day0$max_day"
else new_day="day$max_day"
fi

# Make dir and input files
mkdir -p src/$new_day
touch src/$new_day/input
touch src/$new_day/input_test

# Add default code
cat > src/$new_day/$new_day.rs <<EOF
use extras::files;

fn main() {
    let input = "src/$new_day/input";

    let pt1 = pt1_solution(input);
    let pt2 = pt2_solution(input);

    println!("$new_day: Pt1 = {}", pt1);
    println!("$new_day: Pt2 = {}", pt2);
}

fn pt1_solution(filename: &str) -> u32 {
    let _data: Vec<String> = files::read_into_vec_of(filename, |line| Some(line));
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
        let answer = super::pt1_solution("src/$new_day/input_test");
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_pt2_solution() {
        let answer = super::pt2_solution("src/$new_day/input_test");
        assert_eq!(answer, 0);
    }
}
EOF

# Append to Cargo.toml
cat >> Cargo.toml <<EOF

[[bin]]
name = "$new_day"
path = "src/$new_day/$new_day.rs"
EOF
