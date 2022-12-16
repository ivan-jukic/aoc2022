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
    let data = decode("src/$new_day/input");

    let pt1 = pt1_solution(&data);
    let pt2 = pt2_solution(&data);

    println!("$new_day: Pt1 = {}", pt1);
    println!("$new_day: Pt2 = {}", pt2);
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

    #[test]
    fn test_pt1_solution() {
        let data = decode("src/$new_day/input_test");
        let answer = super::pt1_solution(&data);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_pt2_solution() {
        let data = decode("src/$new_day/input_test");
        let answer = super::pt2_solution(&data);
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
