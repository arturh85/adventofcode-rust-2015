#!/usr/bin/env bash

AOC_YEAR=2015
TERM_TIMEOUT=5m
KILL_TIMEOUT=6m

sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'

function aoc () {
  nl=$'\n'
  echo "Running Day $1 Part $2"
  cargo aoc -d "$1" -p "$2" | grep -v "AOC $AOC_YEAR" | sed "s/Day $1 - Part $2/### Result/g" | sed 's/,/ /g' | sed 's/generator/'"\\${nl}"'- generator/g' | sed 's/runner/'"\\${nl}"'- runner/g' > "times-$1-$2.md"
  if test -s "times-$1-$2.md"; then
    if [ "$2" == "1" ]; then
      head "src/day$1.rs" -n 1 | sed 's/\/\/! # /# 📅 /g' >> times.md
      echo "- [Solution Source](https://github.com/arturh85/adventofcode-rust-$AOC_YEAR/blob/master/src/day$1.rs)" >> times.md
    fi
    echo "## Part $2" >> times.md
    cat "times-$1-$2.md" >> times.md
    echo "Generating flamegraph for Day $1 Part $2"
    time timeout -k $KILL_TIMEOUT $TERM_TIMEOUT cargo aoc flamegraph -d "$1" -p "$2" > /dev/null 2>&1
    if test -f "target/aoc/aoc-autobench/flamegraph.svg"; then
      mv "target/aoc/aoc-autobench/flamegraph.svg" "flamegraph-day$1-$2.svg"
      echo "### [Flamegraph](flamegraph-day$1-$2.svg):" >> times.md
      echo "![Flamegraph Day $1 Part $2](./flamegraph-day$1-$2.svg)" >> times.md
    else
      echo "- 🤯 Flamegraph failed to generate" >> times.md
    fi
  fi
}

echo "# Execution times for Advent of Code $AOC_YEAR" > times.md
echo "- 🔖 [Github Repository](https://github.com/arturh85/adventofcode-rust-$AOC_YEAR)" >> times.md
echo "- 🚀 Benchmarked using [Github Actions](https://github.com/features/actions)" >> times.md
for (( i = 1; i <= 24; i++ )); do
  aoc "$i" 1
  aoc "$i" 2
done

exit 0