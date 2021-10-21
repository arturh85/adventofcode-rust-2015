#!/usr/bin/env bash
sudo sh -c 'echo -1 >/proc/sys/kernel/perf_event_paranoid'
for (( i = 1; i <= 24; i++ )); do
  echo "Day $i"
  cargo aoc -d $i >> timings.txt
  timeout 3m "cargo aoc flamegraph -d $i > /dev/null 2>&1 "
  if test -f "target/aoc/aoc-autobench/flamegraph.svg"; then
    mv "target/aoc/aoc-autobench/flamegraph.svg" "flamegraph$i.svg"
  fi
done

exit 0