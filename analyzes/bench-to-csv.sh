#!/bin/bash

echo "function,implementation,size,time"
while read line
do
    name=$(cut -d ',' -f 1 <<<$line)
    time=$(cut -d ',' -f 2 <<<$line | cut -d ' ' -f 3-4)
    func=$(cut -d '/' -f 1 <<<$name)
    variant=$(cut -d '/' -f 2 <<<$name)
    n=$(cut -d '/' -f 3 <<<$name)
    echo $func,$variant,$n,$time
    #done< <(cat benchmarks.txt | grep -B1 'time:' | tr -d '\n' | sed -E 's/\s+time:\s+/,/g;s/--/\n/g;$a\')
done< <(grep --line-buffered -B1 'time:' | tr -d '\n' | sed -E 's/\s+time:\s+/,/g;s/--/\n/g;$a\')
