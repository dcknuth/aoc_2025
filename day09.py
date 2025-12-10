from time_it import time_it
import networkx as nx

#filename = "test09.txt"
filename = "input09.txt"


with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    max_area = 0
    m = []
    for point in ls:
        x, y = map(int, point.split(','))
        m.append((x,y))
    for i, (x1, y1) in enumerate(m[:len(m)-1]):
        for x2, y2 in m[i+1:]:
            if (abs(x2-x1)+1) * (abs(y2-y1)+1) > max_area:
                max_area = (abs(x2-x1)+1) * (abs(y2-y1)+1)
    
    print(f"Part1 is {max_area}")

part1(ls)


def does_cross(b, l):
    # This assumes all bounding lines are every-other direction and no shared
    #  corners to contain sub-regions. Also, no one-apart lines
    # set if line is horizontal or vertical
    # line too low
    if max(l[1], l[3]) <= min(b[1], b[3]):
        return(False)
    # line too high
    if min(l[1], l[3]) >= max(b[1], b[3]):
        return(False)
    # line too left
    if max(l[0], l[2]) <= min(b[0], b[2]):
        return(False)
    # line too right
    if min(l[0], l[2]) >= max(b[0], b[2]):
        return(False)
    # any other line will now intersect
    return(True)

from itertools import combinations
@time_it
def part2(ls):
    max_area = 0
    m = []
    bounding_lines = []
    last_x = -1
    last_y = -1
    for point in ls:
        x, y = map(int, point.split(','))
        m.append((x,y))
        if last_x > -1:
            bounding_lines.append((last_x, last_y, x, y))
        last_x = x
        last_y = y
    # put in wrap line
    x, y = map(int, ls[0].split(','))
    bounding_lines.append((last_x, last_y, x, y))
    for (x1, y1), (x2, y2) in combinations(m, 2):
        area = (abs(x2-x1)+1) * (abs(y2-y1)+1)
        if area > max_area:
            # find if any of the box's line segments cross one of the
            #  bounding box's line segments
            crosses = False
            for l in bounding_lines:
                if does_cross((x1, y1, x2, y2), l):
                    crosses = True
                    break
            if not crosses:
                max_area = area
    
    print(f"Part2 is {max_area}")

part2(ls)
