use std::collections::{HashMap, HashSet};

fn load_boxes(s: &String) -> Vec<Vec<i64>> {
    let boxes: Vec<Vec<i64>> = s.lines()
        .map(|l| l.split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()).collect();

    boxes
}

fn load_distances(boxes: Vec<Vec<i64>>) -> Vec<(f64, Vec<i64>, Vec<i64>)> {
    let mut by_dist: Vec<(f64, Vec<i64>, Vec<i64>)> = Vec::new();
    for i in 0..(boxes.len()-1) {
        for j in i+1..boxes.len() {
            let dx = boxes[j][0] - boxes[i][0];
            let dy = boxes[j][1] - boxes[i][1];
            let dz = boxes[j][2] - boxes[i][2];
            let dist = ((dx * dx + dy * dy + dz * dz) as f64).sqrt();
            by_dist.push((dist, boxes[i].clone(), boxes[j].clone()));
        } // TODO: remove clones later and see if it makes a difference
    }
    by_dist.sort_by(|a, b| a.0.total_cmp(&b.0));

    by_dist
}

fn add_connection(connections: &mut HashMap<String, HashSet<String>>,
                    dist: (f64, String, String)) {
    // take a distance and two points and put the two points in the connections
    //  hash map. Join connections if needed
    connections.entry(dist.1.clone()).or_insert_with(|| {
        let mut set = HashSet::new();
        set.insert(dist.1.clone());
        set
    }).insert(dist.2.clone());
    connections.entry(dist.2.clone()).or_insert_with(|| {
        let mut set = HashSet::new();
        set.insert(dist.2.clone());
        set
    }).insert(dist.1.clone());
}

fn get_circuits(connections: &HashMap<String, HashSet<String>>) -> 
                Vec<HashSet<String>> {
    let mut circuits: Vec<HashSet<String>> = Vec::new();
    let mut in_circuit: HashSet<String> = HashSet::new();
    for key in connections.keys() {
        if !in_circuit.contains(key) {
            let mut circuit: HashSet<String> = HashSet::new();
            let mut visited: HashSet<String> = HashSet::new();
            let mut to_visit: Vec<String> = connections[key].iter()
                .map(|s| s.clone()).collect();
            circuit.extend(connections[key].clone());
            visited.insert(key.clone());
            while to_visit.len() > 0 {
                if let Some(cur_box) = to_visit.pop() {
                    if !visited.contains(&cur_box) {
                        circuit.extend(connections[&cur_box].clone());
                        visited.insert(cur_box.clone());
                        for neighbor in connections[&cur_box].clone() {
                            if !visited.contains(&neighbor) {
                                to_visit.push(neighbor);
                            }
                        }
                    }
                }
            }
            circuits.push(circuit.clone());
            in_circuit.extend(circuit.clone());
        }
    }

    circuits
}

pub fn part1(s: &String, n: usize) -> i64 {
    let boxes = load_boxes(s);
    let dists = load_distances(boxes);
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for i in 0..n {
        let b1_str = format!("{},{},{}",
                                dists[i].1[0], dists[i].1[1], dists[i].1[2]);
        let b2_str = format!("{},{},{}",
                                dists[i].2[0], dists[i].2[1], dists[i].2[2]);
        add_connection(&mut connections, (dists[i].0, b1_str, b2_str));
    }
    let mut circuits = get_circuits(&connections);
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    let mut total = 1;
    for i in 0..3 {
        total *= circuits[i].len() as i64;
    }
    total
}

pub fn part2(s: &String, n: usize) -> i64 {
    let boxes = load_boxes(s);
    let dists = load_distances(boxes.clone());
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for i in 0..n {
        let b1_str = format!("{},{},{}",
                                dists[i].1[0], dists[i].1[1], dists[i].1[2]);
        let b2_str = format!("{},{},{}",
                                dists[i].2[0], dists[i].2[1], dists[i].2[2]);
        add_connection(&mut connections, (dists[i].0, b1_str, b2_str));
    }
    
    // going to assume that more than the number of connections made in part
    //  1 will be needed to fully connect
    let mut circuits = get_circuits(&connections);
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut i = n-1;
    let num_boxes = boxes.len();
    while circuits[0].len() < num_boxes {
        i += 1;
        let b1_str = format!("{},{},{}",
                                dists[i].1[0], dists[i].1[1], dists[i].1[2]);
        let b2_str = format!("{},{},{}",
                                dists[i].2[0], dists[i].2[1], dists[i].2[2]);
        add_connection(&mut connections, (dists[i].0, b1_str, b2_str));
        // this will be slow TODO, make an add one to replace get_circuits
        circuits = get_circuits(&connections);
    }

    dists[i].1[0] * dists[i].2[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_boxes() {
        let s = "162,817,812
57,618,57
906,360,560".to_string();
        let test_boxes = vec![[162,817,812],
                            [57,618,57],
                            [906,360,560],];
        let boxes = load_boxes(&s);
        assert_eq!(boxes, test_boxes);
    }

    #[test]
    fn test_load_distances() {
        let test_boxes: Vec<Vec<i64>> = vec![vec![162,817,812],
                            vec![57,618,57],
                            vec![906,360,560],];
        let test_dists = vec![
            (787.814, vec![162,817,812], vec![57,618,57]),
            (908.784, vec![162,817,812], vec![906,360,560]),
            (1019.987, vec![57,618,57], vec![906,360,560])
        ];
        let by_dist = load_distances(test_boxes);
        assert!((test_dists[0].0 - by_dist[0].0).abs() < 0.01);
        assert_eq!(by_dist[0].1, test_dists[0].1);
        assert!((test_dists[1].0 - by_dist[1].0).abs() < 0.01);
        assert_eq!(by_dist[1].1, test_dists[1].1);
        assert!((test_dists[2].0 - by_dist[2].0).abs() < 0.01);
        assert_eq!(by_dist[2].1, test_dists[2].1);
    }

    #[test]
    fn test_part1() {
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
425,690,689".to_string();
        
        assert_eq!(part1(&s, 10), 40);
    }

    #[test]
    fn test_part2() {
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
425,690,689".to_string();
        
        assert_eq!(part2(&s, 10), 25272);
    }
}