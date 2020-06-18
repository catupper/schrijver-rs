use proconio::input;

use submodular_optimization::*;

fn main() {
    input! {
        (n, m): (usize, usize),
        edges: [(usize, usize, f64);m],
    }
    let mut g = Graph::new(n);
    for (from, to, cost) in edges {
        g.add_edge(from, to, cost);
    }
    let (cut, ans) = schriver_algorithm(n, &|nodes| g.cut(nodes, 0, n - 1));
    println!("Cut: {:?}", cut);
    println!("Cost: {:?}", ans);

    let (cut, ans) = naive_cut(&g);
    println!("Cut: {:?}", cut);
    println!("Cost: {:?}", ans);
}
