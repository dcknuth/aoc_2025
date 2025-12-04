from time_it import time_it

#filename = "test04.txt"
filename = "input04.txt"

with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    total = 0
    m = []
    for l in ls:
        row = [c for c in l]
        m.append(row)
    for y, row in enumerate(m):
        for x, c in enumerate(row):
            if c == '@':
                np = 0
                # up left
                if y > 0 and x > 0:
                    if m[y-1][x-1] == '@':
                        np += 1
                # up
                if y > 0:
                    if m[y-1][x] == '@':
                        np += 1
                # up right
                if y > 0 and x < len(row)-1:
                    if m[y-1][x+1] == '@':
                        np += 1
                # left
                if x > 0:
                    if m[y][x-1] == '@':
                        np += 1
                # right
                if x < len(row)-1:
                    if m[y][x+1] == '@':
                        np += 1
                # left down
                if y < len(m)-1 and x > 0:
                    if m[y+1][x-1] == '@':
                        np += 1
                # down
                if y < len(m)-1:
                    if m[y+1][x] == '@':
                        np += 1
                # down right
                if y < len(m)-1 and x < len(row)-1:
                    if m[y+1][x+1] == '@':
                        np += 1
                if np < 4:
                    total += 1
    print(f"Part1 is {total}")

part1(ls)

def remove_rolls(m):
    total = 0
    for y, row in enumerate(m):
        for x, c in enumerate(row):
            if c == '@':
                total += 1
                np = 0
                # up left
                if y > 0 and x > 0:
                    if m[y-1][x-1] == '@':
                        np += 1
                # up
                if y > 0:
                    if m[y-1][x] == '@':
                        np += 1
                # up right
                if y > 0 and x < len(row)-1:
                    if m[y-1][x+1] == '@':
                        np += 1
                # left
                if x > 0:
                    if m[y][x-1] == '@':
                        np += 1
                # right
                if x < len(row)-1:
                    if m[y][x+1] == '@':
                        np += 1
                # left down
                if y < len(m)-1 and x > 0:
                    if m[y+1][x-1] == '@':
                        np += 1
                # down
                if y < len(m)-1:
                    if m[y+1][x] == '@':
                        np += 1
                # down right
                if y < len(m)-1 and x < len(row)-1:
                    if m[y+1][x+1] == '@':
                        np += 1
                if np < 4:
                    m[y][x] = '.'
    return(total)

@time_it
def part2(ls):
    m = []
    start_rolls = 0
    for l in ls:
        row = [c for c in l]
        m.append(row)
        start_rolls += l.count('@')
    cur_count = remove_rolls(m)
    last_count = cur_count +1
    while cur_count != last_count:
        last_count = cur_count
        cur_count = remove_rolls(m)

    print(f"Part2 is {start_rolls-cur_count}")

part2(ls)
