from time_it import time_it

#filename = "test02.txt"
filename = "input02.txt"

with open(filename) as f:
    ls = f.read().strip()

@time_it
def part1(ls):
    total = 0
    for s in ls.split(','):
        start, stop = map(int, s.split('-'))
        for id in range(start, stop+1):
            id_str = str(id)
            if id_str[:len(id_str)//2] == id_str[len(id_str)//2:]:
                total += id

    print(f"Part1 is {total}")

part1(ls)

@time_it
def part2(ls):
    total = 0
    for s in ls.split(','):
        start, stop = map(int, s.split('-'))
        for id in range(start, stop+1):
            id_str = str(id)
            for l in range(1, (len(id_str)//2)+1):
                if len(id_str) % l == 0:
                    found = True
                    for i in range(l, len(id_str), l):
                        if id_str[:l] != id_str[i:i+l]:
                            found = False
                            break
                    if found:
                        total += id
                        break
    print(f"Part2 is {total}")

part2(ls)
