advent of code 2022
===================

https://adventofcode.com/2022

### timing

- comparing to these numbers isn't necessarily useful
- normalize your timing to day 1 part 1 and compare
- these timings are very non-scientific (sample size 1)

```shell
$ find -maxdepth 1 -type d -name 'day*' -not -name day00 | sort | xargs --replace bash -xc 'python {}/part1.py {}/input.txt; python {}/part2.py {}/input.txt'
+ python day01/part1.py day01/input.txt
68787
> 756 μs
+ python day01/part2.py day01/input.txt
198041
> 789 μs
+ python day02/part1.py day02/input.txt
13268
> 1485 μs
+ python day02/part2.py day02/input.txt
508
> 1533 μs
+ python day03/part1.py day03/input.txt
8123
> 907 μs
+ python day03/part2.py day03/input.txt
2620
> 625 μs
+ python day04/part1.py day04/input.txt
651
> 1722 μs
+ python day04/part2.py day04/input.txt
956
> 1634 μs
```
