#!/usr/bin/env bash

for (( i = 1; i <= 24; i++ )); do
  cargo aoc -d $i >> timings.txt
  cargo aoc flamegraph -d $i
  mv "target/aoc/aoc-autobench/flamegraph.svg" "flamegraph$i.svg"
done

exit 0