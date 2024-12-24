#!/bin/bash

print_usage() {
  echo "Usage: fetch_input.sh [--force|-f] <year> <day>"
  echo
  echo "This script fetches the input for a specific day of Advent of Code."
  echo
  echo "Options:"
  echo "  --force, -f    Force fetch the input even if it already exists"
  echo
  echo "Environment Variables:"
  echo "  AOC_SESSION    The session cookie value from Advent of Code"
  echo
  echo "You can find the value of the session cookie in your browser after logging in to Advent of Code."
  echo "Optionally, you can create a .env file in the root directory of the repository with the following content:"
  echo "  AOC_SESSION=<your session cookie>"
}

# Exit immediately if a command exits with a non-zero status
set -e

# Export environment variables from the .env file
export "$(xargs <.env)"

if [ -z "$AOC_SESSION" ]; then
  echo "AOC_SESSION environment variable is not set"
  exit 1
fi

# Parse command line arguments
ARGS=()
FORCE=false

while [[ $# -gt 0 ]]; do
  case $1 in
    --help|-h)
      print_usage
      exit 0
      ;;
    --force|-f)
      FORCE=true
      shift
      ;;
    --*|-*)
      echo "Unknown option: $1"
      exit 1
      ;;
    *)
      ARGS+=("$1")
      shift
      ;;
  esac
done

# Check if the required arguments are provided
if [ ${#ARGS[@]} -lt 2 ]; then
  print_usage
  exit 1
fi

YEAR=${ARGS[0]}
if ! [[ "$YEAR" =~ ^[0-9]{4}$ ]]; then
  echo "Invalid year: $YEAR"
  exit 1
fi

DAY=${ARGS[1]}
if ! [[ "$DAY" =~ ^[0-9]+$ ]]; then
  echo "Invalid day: $DAY"
  exit 1
fi

# Check if the input file already exists
if [ -f "input/${YEAR}/${DAY}.txt" ] && [ "$FORCE" != true ]; then
  echo "File already exists: input/${YEAR}/${DAY}.txt"
  echo "Use --force or -f option to force fetch the input."
  exit 0
fi

mkdir -p "input/${YEAR}/"

response_code=$(curl -s "https://adventofcode.com/${YEAR}/day/${DAY}/input" \
  -H "cookie: session=$AOC_SESSION" \
  -o "input/${YEAR}/${DAY}.txt" \
  -w "%{response_code}")
if [ $response_code != "200" ]; then
  echo "Failed to fetch input for day ${DAY} of ${YEAR}, response code: $response_code"
  echo "Make sure the session cookie is correct and has not expired"
  exit 1
fi

# Print success message
echo "Fetched input for day ${DAY} of ${YEAR} to input/${YEAR}/${DAY}.txt"