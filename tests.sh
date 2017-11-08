#!/bin/sh
set -e

cargo build


printf "\ntest missing arg\n"
./target/debug/csv_changer


printf "\ntest input file missing\n"
./target/debug/csv_changer not-exist.csv city london output.csv

printf "\ntest no such column\n"
tail -n +2 input.csv > missing-col.csv
./target/debug/csv_changer missing-col.csv city london output.csv
rm -f missing-col.csv

printf "\ntest missing input content\n"
echo "" > empty.csv
./target/debug/csv_changer empty.csv city london output.csv
rm empty.csv

printf "\ntest no perms to output file\n"
touch output.csv
chmod 111 output.csv
./target/debug/csv_changer input.csv city london output.csv
rm -f output.csv

