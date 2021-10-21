#!/usr/bin/env bash
sh -c 'echo 0 >/proc/sys/kernel/perf_event_paranoid'
for (( i = 1; i <= 24; i++ )); do
  echo "Day $i"
  cargo aoc -d $i >> timings.txt
  cargo aoc flamegraph -d $i
  if test -f "target/aoc/aoc-autobench/flamegraph.svg"; then
    mv "target/aoc/aoc-autobench/flamegraph.svg" "flamegraph$i.svg"
  fi
done

exit 0