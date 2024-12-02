# 🎄 Advent of Code

My [Advent of Code](https://adventofcode.com/about) solutions, organized by year and day. 
While I aim for clean and reasonable approaches, my primary goal is to have fun, so not all solutions will be perfect or fully optimized.

Additionally, this repository includes a simple bash script to automate fetching the puzzle input for each day. To use it, follow these steps:

1. Ensure `curl` is installed.
2. Retrieve your session cookie from your browser after logging in to Advent of Code, and set its value as an environment variable named `AOC_SESSION` (you can use a `.env` file for this).
3. Grant the script execution permissions:
   ```bash
   chmod +x fetch_input.sh
   ```
4. Run the script with:
   ```bash
   ./fetch_input.sh <year> <day>
   ```
   
