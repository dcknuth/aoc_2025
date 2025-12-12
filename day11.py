from time_it import time_it
import networkx as nx

#filename = "test11.txt"
#filename = "test11-2.txt"
filename = "input11.txt"


with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    g = nx.DiGraph()
    for l in ls:
        nodes = l.split()
        from_node = nodes[0][:-1]
        for to_node in nodes[1:]:
            g.add_edge(from_node, to_node)
    paths = list(nx.all_simple_paths(g, source='you', target='out'))
    total = len(paths)
    print(f"Part 1 is {total}")

part1(ls)

from collections import Counter
@time_it
def part2(ls):
    total = 0
    cur_nodes = Counter()
    connections = dict()
    for l in ls:
        nodes = l.split()
        from_node = nodes[0][:-1]
        connections[from_node] = nodes[1:]
    
    # going to assume no loops
    cur_nodes['svr:0:0'] = 1
    while len(cur_nodes) > 0:
        new_nodes = Counter()
        for k in cur_nodes.keys():
            node, seen_dac, seen_fft = k.split(':')
            if node == 'out':
                if seen_dac == '1' and seen_fft == '1':
                    total += cur_nodes[k]
            else:
                for to_node in connections[node]:
                    if to_node == 'dac':
                        cur_dac = '1'
                    else:
                        cur_dac = seen_dac
                    if to_node == 'fft':
                        cur_fft = '1'
                    else:
                        cur_fft = seen_fft
                    new_node = f"{to_node}:{cur_dac}:{cur_fft}"
                    new_nodes[new_node] += cur_nodes[k]
        cur_nodes = new_nodes
        
    print(f"Part 2 is {total}")

part2(ls)
