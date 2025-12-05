from time_it import time_it

#filename = "test05.txt"
filename = "input05.txt"

with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    total = 0
    ranges = dict()
    start_ids = 0
    for i, l in enumerate(ls):
        if l == "":
            start_ids = i+1
            break
        ss = list(map(int, l.split('-')))
        start = int(ss[0])
        stop = int(ss[1])
        ranges[(start, stop)] = True
    for l in ls[start_ids:]:
        id = int(l)
        for k in ranges.keys():
            if id >= k[0] and id <= k[1]:
                total += 1
                break
    
    print(f"Part1 is {total}")

part1(ls)


@time_it
def part2(ls):
    ranges = dict()
    start_ids = 0
    for i, l in enumerate(ls):
        if l == "":
            start_ids = i+1
            break
        ss = list(map(int, l.split('-')))
        start = int(ss[0])
        stop = int(ss[1])
        ranges[(start, stop)] = (stop - start) + 1

    def mergeRange(r1, r2):
        # not overlapping
        if r1[1] < r2[0] or r1[0] > r2[1]:
            return()
        # r1 inside r2
        if r1[0] >= r2[0] and r1[1] <= r2[1]:
            return(r2)
        # r2 inside r1
        if r2[0] >= r1[0] and r2[1] <= r1[1]:
            return(r1)
        # r1 lower overlapping range
        if r1[0] <= r2[0] and r2[1] >= r1[1]:
            return((r1[0], r2[1]))
        # r1 upper overlapping range
        if r2[0] <= r1[0] and r1[1] >= r2[1]:
            return((r2[0], r1[1]))
        print("Should not get to here in mergeRange")
        return()

    done = False
    while not done:
        done = True
        found = False
        delete_list = []
        loop_keys = list(ranges.keys())
        for i, k1 in enumerate(loop_keys[:-1]):
            if found:
                break
            for k2 in loop_keys[i+1:]:
                merged = mergeRange(k1, k2)
                if merged != ():
                    found = True
                    done = False
                    if merged != k1:
                        delete_list.append(k1)
                    if merged != k2:
                        delete_list.append(k2)
                    ranges[merged] = ((merged[1] - merged[0]) + 1)
                    break
        for range in delete_list:
            del ranges[range]
    
    total = sum(ranges.values())
    print(f"Part2 is {total}")

part2(ls)
