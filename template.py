from time_it import time_it

filename = "test00.txt"
#filename = "input00.txt"

# Read in file. There might be a day or two where this needs to be changed
with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    # Example of just counting the length of the input
    ans1 = len(ls)
    print(f"Part1 is {ans1}")

part1(ls)

@time_it
def part2(ls):
    # Example of: sum of line index times length of line
    ans2 = 0
    for i, l in enumerate(ls):
        ans2 += i * len(l)
    print(f"Part2 is {ans2}")

#part2(ls)
