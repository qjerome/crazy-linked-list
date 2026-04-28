#!/bin/bash

cargo bench 2>&1 | tee benchmarks.txt
# we post-process benchmarks
cat benchmarks.txt | ./bench-to-csv.sh | tee benchmarks.csv
