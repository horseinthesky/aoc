#!/bin/bash
set -euo pipefail
SCRIPT_DIR=$(realpath "$(dirname "$0")")

if [[ $# != 1 ]]; then
  echo "Please provide a day number."
  echo "usage: $0 DAY"
  exit 1
fi

if [[ ! "$1" =~ ^(0[1-9]|1[0-9]|2[0-5])$ ]]; then
  echo "Not a valid day: $1"
  exit 1
fi

# You can find aoc session cookie by using Chrome tools:
# 1) Go to https://adventofcode.com/2024/day/1/input
# 2) right-click -> inspect -> click the "Application" tab.
# 3) Refresh
# 4) Click https://adventofcode.com under "Cookies"
# 5) Grab the value for the session. Put it to an env var AOC_SESSION
if [[ -z "${AOC_SESSION-""}" ]]; then
  echo "\$AOC_SESSION not set"
  exit 1
fi

TMPFILE=$(mktemp)
trap 'rm -f "$TMPFILE"' EXIT

# #0 removes leading 0 from valus of $1. Makes sure we use correct day value
# ./fetch.sh 05 becomes day5/input.txt
# ./fetch.sh 12 becomes day12/input.txt
curl "https://adventofcode.com/2023/day/${1#0}/input" \
  --fail \
  --connect-timeout 5 \
  --cookie "session=$AOC_SESSION" \
  -A "Bash script at $(git remote -v | awk 'NR==1{print $2}')" \
  | tee "$TMPFILE"

mv "$TMPFILE" "$SCRIPT_DIR/day$1/input.txt"
