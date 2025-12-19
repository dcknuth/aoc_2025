struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut a = self.find(a);
        let mut b = self.find(b);

        if a != b {
            if self.size[a] < self.size[b] {
                std::mem::swap(&mut a, &mut b);
            }
            self.parent[b] = a;
            self.size[a] += self.size[b];
        }
    }

    fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    fn all_component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = Vec::new();
        for i in 0..self.parent.len() {
            if self.find(i) == i {
                sizes.push(self.size[i]);
            }
        }
        sizes
    }
}

fn load_boxes(s: &str) -> (Vec<[i64; 3]>, Vec<&str>) {
    let mut coords = Vec::new();
    let mut labels = Vec::new();

    for line in s.lines() {
        let nums: Vec<i64> = line.split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        coords.push([nums[0], nums[1], nums[2]]);
        labels.push(line);
    }

    (coords, labels)
}

fn load_distances(coords: &Vec<[i64; 3]>) -> Vec<(i64, usize, usize)> {
    let n = coords.len();
    let mut dists = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n - 1 {
        for j in i + 1..n {
            let dx = coords[j][0] - coords[i][0];
            let dy = coords[j][1] - coords[i][1];
            let dz = coords[j][2] - coords[i][2];
            let dist = dx * dx + dy * dy + dz * dz;
            dists.push((dist, i, j));
        }
    }

    dists.sort_unstable();
    dists
}

pub fn part1_and2(s: &str, n: usize) -> (i64, i64) {
    let (coords, labels) = load_boxes(s);
    let dists = load_distances(&coords);

    let num_boxes = coords.len();
    let mut dsu = DSU::new(num_boxes);

    // Part 1: union the first n edges
    for i in 0..n {
        let (_, a, b) = dists[i];
        dsu.union(a, b);
    }

    // Compute the sizes of all components
    let mut sizes = dsu.all_component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let total = (sizes[0] * sizes[1] * sizes[2]) as i64;

    // Part 2: continue unioning until fully connected
    let mut i = n - 1;
    while dsu.component_size(0) < num_boxes {
        i += 1;
        let (_, a, b) = dists[i];
        dsu.union(a, b);
    }

    // Extract x-coordinates from coords instead of splitting strings
    let x1 = coords[dists[i].1][0];
    let x2 = coords[dists[i].2][0];

    (total, x1 * x2)
}


#[cfg(test)]
mod tests {
    use super::*;

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