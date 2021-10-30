#!/usr/bin/env bash
sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'

function aoc () {
   cargo aoc -d $1 -p $2 | grep -v "AOC 2015" | sed 's/generator/- generator/g' | sed 's/runner/- runner/g'  > times-$1-$2.md
   if test -s times-$1-$2.md; then
      echo "### Part $2" >> times.md
      cat times-$1-$2.md >> times.md

      time timeout -k 6m 5m cargo aoc flamegraph -d $1 -p $2 > /dev/null 2>&1
      if test -f "target/aoc/aoc-autobench/flamegraph.svg"; then
        mv "target/aoc/aoc-autobench/flamegraph.svg" "flamegraph$1-$2.svg"
        echo "- [Flamegraph Day $1 Part $2](flamegraph$1-$2.svg)" >> times.md
        echo "![Flamegraph Day $1 Part $2](flamegraph$1-$2.svg)" >> times.md
      fi
   fi
}

echo "# Github Action Execution times for Advent of Code 2015" >> times.md
for (( i = 1; i <= 24; i++ )); do
  echo "## 2015 Day $i" >> times.md
  aoc $i 1
  aoc $i 2
done

exit 0