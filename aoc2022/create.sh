#!/bin/bash

set -e

DATE_NUMBER=${1:-$(date '+%d')}
YEAR_NUMBER="2022"

# Download input
cargo aoc input -y $YEAR_NUMBER -d $DATE_NUMBER

sed -e "s/\$DAY_NUMBER/$DATE_NUMBER/" src/template.rs > src/day$DATE_NUMBER.rs
sed -e "s/\/\/INSERTNEW/pub mod day$DATE_NUMBER;\n\/\/INSERTNEW/" src/lib.rs > tmp.rs
mv tmp.rs src/lib.rs

echo "Created files for AOC day $DATE_NUMBER, happy hacking!"
