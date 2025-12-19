use std::collections::{HashMap, HashSet};

fn load_boxes<'a>(s: &'a str) -> Vec<(&'a str, Vec<i64>)> {
    let boxes: Vec<(&str, Vec<i64>)> = s.lines()
        .map(|l| {
            let nums = l.split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (l, nums)
        }).collect();

    boxes
}

fn load_distances<'a>(boxes: &'a Vec<(&'a str, Vec<i64>)>)
        -> Vec<(i64, &'a str, &'a str)> {
    let mut by_dist: Vec<(i64, &str, &str)> = Vec::new();
    for i in 0..(boxes.len()-1) {
        for j in i+1..boxes.len() {
            let dx = boxes[j].1[0] - boxes[i].1[0];
            let dy = boxes[j].1[1] - boxes[i].1[1];
            let dz = boxes[j].1[2] - boxes[i].1[2];
            let dist = dx * dx + dy * dy + dz * dz;
            by_dist.push((dist, boxes[i].0, boxes[j].0));
        }
    }
    by_dist.sort_unstable();

    by_dist
}

fn add_connection<'a>(connections: &mut HashMap<&'a str, HashSet<&'a str>>,
                    dist: (i64, &'a str, &'a str)) {
    // take a distance and two points and put the two points in the connections
    //  hash map. Join connections if needed
    connections.entry(dist.1).or_insert_with(|| {
        let mut set = HashSet::new();
        set.insert(dist.1);
        set
    }).insert(dist.2);
    connections.entry(dist.2).or_insert_with(|| {
        let mut set = HashSet::new();
        set.insert(dist.2);
        set
    }).insert(dist.1);
}

fn get_circuits<'a>(connections: &HashMap<&'a str, HashSet<&'a str>>) -> 
                Vec<HashSet<&'a str>> {
    let mut circuits: Vec<HashSet<&str>> = Vec::new();
    let mut in_circuit: HashSet<&str> = HashSet::new();
    for &key in connections.keys() {
        if !in_circuit.contains(key) {
            let mut circuit: HashSet<&str> = HashSet::new();
            let mut visited: HashSet<&str> = HashSet::new();
            let mut to_visit: Vec<&str> = connections[key].iter()
                .copied().collect();
            circuit.extend(connections[key].iter().copied());
            visited.insert(key);
            while to_visit.len() > 0 {
                if let Some(cur_box) = to_visit.pop() {
                    if !visited.contains(cur_box) {
                        circuit.extend(connections[cur_box].iter().copied());
                        visited.insert(cur_box);
                        for &neighbor in &connections[cur_box] {
                            if !visited.contains(neighbor) {
                                to_visit.push(neighbor);
                            }
                        }
                    }
                }
            }
            in_circuit.extend(circuit.iter().copied());
            circuits.push(circuit);
        }
    }

    circuits
}

fn add_circuit<'a>(circuits: &mut Vec<HashSet<&'a str>>,
                connections: &mut HashMap<&'a str, HashSet<&'a str>>,
                dist: (i64, &'a str, &'a str)) {
    // will need to know which were already in a circuit
    let mut b1_in_circuit = false;
    let mut b2_in_circuit = false;
    if connections.contains_key(dist.1) {
        b1_in_circuit = true;
    }
    if connections.contains_key(dist.2) {
        b2_in_circuit = true;
    }
    
    // update the connections
    connections.entry(dist.1).or_insert_with(|| {
        let mut set = HashSet::new();
        set.insert(dist.1);
        set
    }).insert(dist.2);
    connections.entry(dist.2).or_insert_with(|| {
        let mut set = HashSet::new();
        set.insert(dist.2);
        set
    }).insert(dist.1);
    
    // now add box1 to circuits
    if ! b1_in_circuit && ! b2_in_circuit {
        let mut circuit: HashSet<&str> = HashSet::new();
        circuit.extend(connections[dist.1].iter().copied());
        circuits.push(circuit);
    } else if b1_in_circuit && ! b2_in_circuit {
        for cur_circuit in circuits {
            if cur_circuit.contains(dist.1) {
                cur_circuit.extend(connections[dist.2].iter().copied());
                break;
            }
        }
    } else if b1_in_circuit && b2_in_circuit {
        for (i, cur_circuit) in circuits.iter().enumerate() {
            if cur_circuit.contains(dist.1) {
                if cur_circuit.contains(dist.2) {
                    break; // both boxes already in circuit
                }
                let mut circuit: HashSet<&str> = HashSet::new();
                circuit.extend(cur_circuit.iter().copied());
                for (j, to_circuit) in circuits.iter().enumerate() {
                    if to_circuit.contains(dist.2) {
                        circuit.extend(to_circuit.iter().copied());
                        if j > i {
                            circuits.remove(j);
                            circuits.remove(i);
                        } else {
                            circuits.remove(i);
                            circuits.remove(j);
                        }
                        circuits.push(circuit);
                        break;
                    }
                }
                break;
            }
        }
    } else {
        // b2 in a circuit, but not b1
        for cur_circuit in circuits {
            if cur_circuit.contains(dist.2) {
                cur_circuit.extend(connections[dist.1].iter().copied());
                break;
            }
        }
    }
}

pub fn part1_and2(s: &str, n: usize) -> (i64, i64) {
    let boxes = load_boxes(s);
    let dists = load_distances(&boxes);
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for i in 0..n {
        add_connection(&mut connections, dists[i]);
    }
    
    let mut circuits = get_circuits(&connections);
    circuits.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
    
    let mut total = 1;
    for i in 0..3 {
        total *= circuits[i].len() as i64;
    }

    // going to assume that more than the number of connections made in part
    //  1 will be needed to fully connect
    let mut i = n-1;
    let num_boxes = boxes.len();
    while circuits[0].len() < num_boxes {
        i += 1;
        add_circuit(&mut circuits, &mut connections, dists[i]);
    }
    let x1 = dists[i].1.split(',').next()
        .unwrap().parse::<i64>().unwrap();
    let x2 = dists[i].2.split(',').next()
        .unwrap().parse::<i64>().unwrap();
    (total, x1 * x2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_boxes() {
        let s = "162,817,812
57,618,57
906,360,560";
        let test_boxes: Vec<(&str, Vec<i64>)> = vec![
            ("162,817,812", vec![162,817,812]),
            ("57,618,57", vec![57,618,57]),
            ("906,360,560", vec![906,360,560]),];
        let boxes = load_boxes(&s);
        assert_eq!(boxes, test_boxes);
    }

    #[test]
    fn test_load_distances() {
        let test_boxes: Vec<(&str, Vec<i64>)> = vec![
            ("162,817,812", vec![162,817,812]),
            ("57,618,57", vec![57,618,57]),
            ("906,360,560", vec![906,360,560]),];
        let test_dists = vec![
            (620651, "162,817,812", "57,618,57"),
            (825889, "162,817,812", "906,360,560"),
            (1040374, "57,618,57", "906,360,560")
        ];
        let by_dist = load_distances(&test_boxes);
        assert_eq!(test_dists[0].0, by_dist[0].0);
        assert_eq!(by_dist[0].1, test_dists[0].1);
        assert_eq!(test_dists[1].0, by_dist[1].0);
        assert_eq!(by_dist[1].1, test_dists[1].1);
        assert_eq!(test_dists[2].0, by_dist[2].0);
        assert_eq!(by_dist[2].1, test_dists[2].1);
    }

    #[test]
    fn test_part1_and2() {
        let s = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        
        assert_eq!(part1_and2(&s, 10), (40, 25272));
    }
}