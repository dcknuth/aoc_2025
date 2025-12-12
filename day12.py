from time_it import time_it

#filename = "test12.txt"
filename = "input12.txt"
GNUM = 6
GSIZE = 3

with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    presents = []
    present_areas = []
    l = 0
    for i in range(GNUM):
        l += 1
        present = []
        area = 0
        for j in range(GSIZE):
            row = [{'#':1, '.':0}[c] for c in ls[l]]
            present.append(row)
            l += 1
            area += sum(row)
        presents.append(present)
        present_areas.append(area)
        l += 1
    # now l should be at the start of the trees
    trees = []
    for line in range(l, len(ls)):
        j = ls[line].split()
        x, y = map(int, j[0][:-1].split('x'))
        tree_list = [int(n) for n in j[1:]]
        trees.append(([x,y], tree_list))
    # input loaded
    
    total = 0
    trees_xtra_room = []
    t_room_as_percent = []
    for (x, y), t_list in trees:
        t_area = x * y
        p_area = 0
        for i, p in enumerate(t_list):
            p_area += p * present_areas[i]
        trees_xtra_room.append(t_area - p_area)
        percent_area_free = (t_area - p_area)/t_area
        t_room_as_percent.append(percent_area_free)
        if percent_area_free > 0.15:
            total += 1
    xtra_room_sorted = sorted(trees_xtra_room, reverse=True)
    t_room_as_percent.sort(reverse=True)
    #print(xtra_room_sorted)
    #print(t_room_as_percent)

    # can't really believe this approach worked
    print(f"Part 1 is {total}")

part1(ls)
