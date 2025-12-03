from time_it import time_it

#filename = "test03.txt"
filename = "input03.txt"

with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    total = 0
    for l in ls:
        batteries = [int(x) for x in l]
        tens = batteries[-2]
        tens_index = len(batteries)-2
        ones = batteries[-1]

        for i in range(len(batteries)-3, -1, -1):
            if tens <= batteries[i]:
                tens = batteries[i]
                tens_index = i
        i = len(batteries)-2
        while i > tens_index:
            if ones < batteries[i]:
                ones = batteries[i]
            i -= 1
        total += tens * 10 + ones
    print(f"Part1 is {total}")

part1(ls)

@time_it
def part2(ls):
    total = 0
    for l in ls:
        batteries = [int(x) for x in l]
        indexes = list(range(len(batteries)-12, len(batteries)))
        last_max = -1
        for i in range(12):
            j = indexes[i]
            while j > last_max:
                if batteries[indexes[i]] <= batteries[j]:
                    indexes[i] = j
                j -= 1
            last_max = indexes[i]
        sub_total = 0
        for p in range(12):
            battery = batteries[indexes[p]]
            sub_total += battery * 10**(11-p)
        total += sub_total
    print(f"Part2 is {total}")

part2(ls)
