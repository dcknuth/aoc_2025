from time_it import time_it
from itertools import product

#filename = "test10.txt"
filename = "input10.txt"


with open(filename) as f:
    ls = f.read().strip().split('\n')

@time_it
def part1(ls):
    # load file
    light_targets = []
    button_lists = []
    jolt_lists = []
    for l in ls:
        parts = list(l.split())
        l_target = []
        for c in parts[0][1:-1]:
            if c == '.':
                l_target.append(False)
            else:
                l_target.append(True)
        light_targets.append(l_target)
        j_list = []
        for jolts in parts[-1][1:-1].split(','):
            j_list.append(int(jolts))
        jolt_lists.append(j_list)
        b_list = []
        for button in parts[1:-1]:
            b_list.append([int(x) for x in button[1:-1].split(',')])
        button_lists.append(b_list)
    
    # try brute-force fit
    b_presses = 0
    for m in range(len(light_targets)):
        found = False
        try_n = 1
        while not found:
            for to_try in product(button_lists[m], repeat=try_n):
                cur_state = [False for x in range(len(light_targets[m]))]
                for b in to_try:
                    for i in b:
                        cur_state[i] = not cur_state[i]
                if cur_state == light_targets[m]:
                    found = True
                    b_presses += try_n
                    print(f"Found {to_try}")
                    break
            if found == True:
                break
            try_n += 1
    
    print(f"Part1 is {b_presses}")

part1(ls)

@time_it
def part2(ls):
    # load file
    light_targets = []
    button_lists = []
    jolt_lists = []
    for l in ls:
        parts = list(l.split())
        l_target = []
        for c in parts[0][1:-1]:
            if c == '.':
                l_target.append(False)
            else:
                l_target.append(True)
        light_targets.append(l_target)
        j_list = []
        for jolts in parts[-1][1:-1].split(','):
            j_list.append(int(jolts))
        jolt_lists.append(j_list)
        b_list = []
        for button in parts[1:-1]:
            b_list.append([int(x) for x in button[1:-1].split(',')])
        button_lists.append(b_list)
    
    # brute force will not work here as even the example has too many presses
    # we can turn the buttons into a set of equations with the jolts as
    #  totals for each position. Try pulp
    import pulp
    total = 0
    for m in range(len(jolt_lists)):
        prob = pulp.LpProblem("MinimizeSum", pulp.LpMinimize)
        var_names = [chr(ord('a')+i) for i in range(len(button_lists[m]))]
        vars = dict()
        for name in var_names:
            vars[name] = pulp.LpVariable(name, lowBound=0, cat="Integer")
        
        # add pulp objective
        prob += sum(vars.values())
        # Loop constraints
        for i, c in enumerate(jolt_lists[m]):
            var_ids = []
            for j in range(len(button_lists[m])):
                if i in button_lists[m][j]:
                    var_ids.append(var_names[j])
            prob += pulp.lpSum([vars[id] for id in var_ids]) == c, f"lt_{i}"

        # pulp solve
        prob.solve(pulp.PULP_CBC_CMD(msg=0)) # msg=0 turns off printing
        total += pulp.value(prob.objective)
        
    print(f"Part 2 is {total}")

part2(ls)
