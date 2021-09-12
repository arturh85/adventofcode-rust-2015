# Advent of Code 2015 in [Rust](https://www.rust-lang.org/)

```
AOC 2015
Day 1 - Part 1 : 232
        generator: 400ns,
        runner: 17.9µs

Day 1 - Part 1 - alt1 : 232
        generator: 200ns,
        runner: 18.6µs

Day 1 - Part 2 : 1783
        generator: 100ns,
        runner: 5.5µs

Day 2 - Part 1 : 1586300
        generator: 169.2µs,
        runner: 500ns

Day 2 - Part 2 : 3737498
        generator: 148.9µs,
        runner: 700ns

Day 3 - Part 1 : 2565
        generator: 800ns,
        runner: 492µs

Day 3 - Part 2 : 2639
        generator: 200ns,
        runner: 388.2µs
	
Day 4 - Part 1 : 346386
	generator: 200ns,
	runner: 55.2363ms

Day 4 - Part 2 : 9958218
	generator: 100ns,
	runner: 1.3883396s
```

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