#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <day_number>"
    exit 1
fi

# Get the day number from the first argument
DAY_NUMBER=$(printf "%02d" "$1")
CRATE_NAME="day$DAY_NUMBER"

# Create a new binary crate for the specified day
cargo new --bin "$CRATE_NAME"

# Navigate into the new crate directory
cd "$CRATE_NAME" || exit

# Add a basic template to main.rs
cat <<EOL > src/main.rs
fn main() {
    let input = common::read_input("data/inputs/$DAY_NUMBER.txt");
    println!("Solution for Day $DAY_NUMBER: {}", solve_day$DAY_NUMBER(&input));
}

fn solve_day$DAY_NUMBER(input: &str) -> String {
    // Implement solution logic here
    "result".to_string()
}
EOL

# Add common as a dependency in Cargo.toml
cat <<EOL >> Cargo.toml

[dependencies]
common = { path = "../common" }
EOL

# Navigate back to the root of the workspace
cd ..

# Update the root Cargo.toml to include the new crate as a member
sed -i.bak "/members = \[/a \    \"$CRATE_NAME\"," Cargo.toml

echo "Crate $CRATE_NAME created and added to the workspace."
