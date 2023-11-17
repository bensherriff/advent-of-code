[![Rust](https://github.com/bensherriff/advent-of-code/actions/workflows/rust.yml/badge.svg)](https://github.com/bensherriff/advent-of-code/actions/workflows/rust.yml)

# Building
`cargo build --release`

The executable will be located under `target/release/adventofcode.exe`

# Usage
Using remote inputs from [Advent of Code](https://adventofcode.com) requires an `.env` file set.
Copy `.env.TEMPLATE` to `.env` and insert a valid `SESSION` value. A session value can be retrieved by:
  1. Navigate to the advent of code website
  2. Sign in
  3. Open up the browser dev tools
  4. Copy the `session` application cookie value
  5. Paste the cookie value into the `.env`

Basic usage: `adventofcode <DAY>`

Specify the year: `adventofcode <DAY> --year <YEAR>`

Using a local file: `adventofcode <DAY> --local <file>`
