#!/usr/bin/env bash

for (( i = 1; i <= 24; i++ )); do
  cargo aoc -d $i | grep -v AOC >> timings.txt
  cargo aoc bench -d $i
done