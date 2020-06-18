pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cost: f64,
}

pub struct Graph {
    n: usize,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self { n, edges: vec![] }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn cut(&self, nodes: &[bool], s: usize, t: usize) -> f64 {
        let mut cost = 0.0;
        if !nodes[s] {
            cost += 1e10;
        }
        if nodes[t] {
            cost += 1e10;
        }
        for e in &self.edges {
            if nodes[e.from] ^ nodes[e.to] {
                cost += e.cost;
            }
        }
        cost
    }
    pub fn add_edge(&mut self, from: usize, to: usize, cost: f64) {
        self.edges.push(Edge { from, to, cost });
    }
}

pub fn naive_cut(g: &Graph) -> (Vec<bool>, f64) {
    let mut ans = 1e10;
    let mut arg = vec![];
    for bit in 0..(1 << g.size()) {
        let v = (0..g.size())
            .map(|i| (bit >> i) % 2 == 1)
            .collect::<Vec<_>>();
        let tmp = g.cut(&v, 0, g.size() - 1);
        //println!("{:?}", v);
        //println!("{:?}", tmp);
        if tmp < ans {
            ans = tmp;
            arg = v;
        }
    }
    (arg, ans)
}
