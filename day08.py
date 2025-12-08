from time_it import time_it
import networkx as nx

#filename = "test08.txt"
filename = "input08.txt"
NUM = 1000
TOP_N = 3

with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    boxes = []
    for l in ls:
        coords = tuple(map(int, l.split(',')))
        boxes.append(coords)
    distances = dict()
    for i, b1 in enumerate(boxes[:-1]):
        for b2 in boxes[i+1:]:
            distance = ((b1[0]-b2[0])**2 + (b1[1]-b2[1])**2 +
                        (b1[2]-b2[2])**2)**0.5
            distances[tuple(sorted([b1, b2]))] = distance
    boxes_by_d = list(distances.items())
    boxes_by_d.sort(key=lambda x: x[1], reverse=True)
    
    # make a graph and connect NUM edges
    g = nx.Graph()
    for b in boxes:
        g.add_node(b)
    for i in range(NUM):
        cur_pair = boxes_by_d.pop()
        (b1, b2), dist = cur_pair
        g.add_edge(b1, b2)

    # get sub-graphs, sort and multiply
    sub_graphs = list(nx.connected_components(g))
    sub_graphs.sort(key=len, reverse=True)
    total = 1
    for i in range(TOP_N):
        total *= len(sub_graphs[i])
    
    print(f"Part1 is {total}")

part1(ls)

@time_it
def part2(ls):
    boxes = []
    for l in ls:
        coords = tuple(map(int, l.split(',')))
        boxes.append(coords)
    distances = dict()
    for i, b1 in enumerate(boxes[:-1]):
        for b2 in boxes[i+1:]:
            distance = ((b1[0]-b2[0])**2 + (b1[1]-b2[1])**2 +
                        (b1[2]-b2[2])**2)**0.5
            distances[tuple(sorted([b1, b2]))] = distance
    boxes_by_d = list(distances.items())
    boxes_by_d.sort(key=lambda x: x[1], reverse=True)
    
    # make a graph and connect until fully connected
    g = nx.Graph()
    for b in boxes:
        g.add_node(b)
    fully_connected = False
    ans_p2 = 0
    while not fully_connected:
        cur_pair = boxes_by_d.pop()
        (b1, b2), dist = cur_pair
        g.add_edge(b1, b2)
        component_nodes = nx.node_connected_component(g, b1)
        if len(component_nodes) == len(boxes):
            fully_connected = True
            ans_p2 = b1[0] * b2[0]

    print(f"Part2 is {ans_p2}")

part2(ls)
