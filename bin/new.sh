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
fn main() {
    println!("TO IMPLEMENT >> $new_day")
}


///
/// Tests
/// ............................................................................

#[cfg(test)]
mod test {

    #[test]
    fn pt1_solution() {
        assert!(true);
    }

    #[test]
    fn pt2_solution() {
        assert!(true);
    }
}
EOF

# Append to Cargo.toml
cat >> Cargo.toml <<EOF

[[bin]]
name = "$new_day"
path = "src/$new_day/$new_day.rs"
EOF
