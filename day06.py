from time_it import time_it
from collections import defaultdict

#filename = "test06.txt"
filename = "input06.txt"

with open(filename) as f:
    ls = f.read().split('\n')

@time_it
def part1(ls):
    total = 0
    eqs = defaultdict(list)
    for i, e in enumerate(ls):
        for i, v in enumerate(e.split()):
            eqs[i].append(v.strip())
    for l in eqs.values():
        if l[-1] == '+':
            total += sum(map(int, l[:-1]))
        else:
            prod_total = 1
            for i in l[:-1]:
                prod_total *= int(i)
            total += prod_total
    
    print(f"Part1 is {total}")

part1(ls)

@time_it
def part2(ls):
    total = 0
    num_lists = defaultdict(list)
    eq_num = 0
    i = 0
    while i < len(ls[0]):
        found = False
        num_str = ''
        for l in range(len(ls)-1):
            if ls[l][i].isdigit():
                found = True
                num_str = num_str + ls[l][i]
        if found:
            num_lists[eq_num].append(int(num_str))
        else:
            eq_num += 1
        i += 1
    # due to a \n at the end   v  is 2 instead of 1
    for x, op in enumerate(ls[-2].split()):
        if op == '+':
            total += sum(num_lists[x])
        else:
            prod_total = 1
            for j in num_lists[x]:
                prod_total *= j
            total += prod_total
    print(f"Part2 is {total}")

part2(ls)
