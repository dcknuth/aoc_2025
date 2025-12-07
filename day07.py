from time_it import time_it
from collections import Counter, defaultdict

#filename = "test07.txt"
filename = "input07.txt"

with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    splits = Counter()
    m = [list(l) for l in ls]
    # first row
    i = m[0].index('S')
    m[0][i] = '|'
    # all rows
    for i, row in enumerate(m[:-1]):
        indices = [j for j, c in enumerate(row) if c == '|']
        for k in indices:
            if m[i+1][k] == '^':
                splits[(i+1, k)] += 1
                if k == 0 or k == len(row) - 1:
                    print(f"We have a boundary problem at y: {i+i} {k=}")
                elif m[i+1][k-1] == '^':
                    print(f"We have a collision prob at i: {i+1} k: {k-1}")
                elif m[i+1][k+1] == '^':
                    print(f"We have a collision prob at i: {i+1} k: {k+1}")
                else:
                    m[i+1][k-1] = '|'
                    m[i+1][k+1] = '|'
            else:
                m[i+1][k] = '|'
    
    print(f"Part1 is {len(splits)}")

part1(ls)

@time_it
def part2(ls):
    m = [list(l) for l in ls]
    i = m[0].index('S')
    timelines = defaultdict(int)
    timelines[(0, i)] = 1
    for i in range(len(m)-1):
        new_timelines = defaultdict(int)
        for key in timelines.keys():
            cur_y, cur_x = key
            if m[cur_y+1][cur_x] == '^':
                new_timelines[(cur_y+1, cur_x-1)] += timelines[key]
                new_timelines[(cur_y+1, cur_x+1)] += timelines[key]
                # we know from part 1 that we will not have any bounds or
                #  collision issues
            else:
                new_timelines[(cur_y+1, cur_x)] += timelines[key]
        timelines = new_timelines
    print(f"Part2 is {sum(timelines.values())}")

part2(ls)
