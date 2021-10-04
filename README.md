# Advent of Code 2015 in [Rust](https://www.rust-lang.org/)

- [Docs](https://arturh85.github.io/adventofcode-rust-2015/adventofcode_rust_2015/)
- [Timings](https://arturh85.github.io/adventofcode-rust-2015/timings.txt)



# Setup

see https://github.com/gobanos/cargo-aoc

## Install `cargo aoc`

Boot a terminal and run `cargo install cargo-aoc`

If you installed `cargo-aoc` in a previous year, it may be out of date now and this
will cause build errors because of a version mismatch if you use the latest version of the `aoc-runner` and `aoc-runner-derive` below. 
If that's the case, then update it with `cargo install cargo-aoc --force`.

## Setting up the CLI

You will need to find your session token for the AoC in order for cargo-aoc to work. Thankfully, finding your token is easy since it is stored in your Browser's cookies. Open up the devtools of your browser, and then :

* Firefox: "Storage" tab, Cookies, and copy the "Value" field of the `session` cookie.
* Google Chrome / Chromium: "Application" tab, Cookies, and copy the "Value" field of the `session` cookie.

Once you have it, simply run : `cargo aoc credentials -s {token}`

You're now ready to start coding !

NOTE: If for some reason your token has changed, dont forget to change it back.

`cargo aoc credentials` will show the currently stored user token


## Running the latest Problem

`cargo aoc`

## Running a specific Day

`cargo aoc -d 2`