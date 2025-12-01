from time_it import time_it

#filename = "test01.txt"
filename = "input01.txt"

# Read in file. There might be a day or two where this needs to be changed
with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    total = 0
    cur_dial = 50
    for l in ls:
        if l[0] == "L":
            cur_dial -= int(l[1:])
            cur_dial %= 100
        else:
            cur_dial += int(l[1:])
            cur_dial %= 100
        if cur_dial == 0:
            total += 1
    print(f"Part1 is {total}")

part1(ls)

@time_it
def part2(ls):
    total = 0
    cur_dial = 50
    for l in ls:
        if l[0] == "L":
            for i in range(int(l[1:])):
                cur_dial -= 1
                cur_dial %= 100
                if cur_dial == 0:
                    total += 1
        else:
            for i in range(int(l[1:])):
                cur_dial += 1
                cur_dial %= 100
                if cur_dial == 0:
                    total += 1
    print(f"Part2 is {total}")

part2(ls)
