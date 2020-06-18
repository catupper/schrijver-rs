use std::fs::File;
use std::io::prelude::*;

fn main() {
    make_output("input/01.txt", 20);
    make_output("input/02.txt", 20);
    make_output("input/03.txt", 20);
    make_output("input/04.txt", 20);
    make_output("input/05.txt", 20);
    make_output("input/06.txt", 20);
    make_output("input/07.txt", 20);
}

fn make_output(path: &str, n: usize) {
    let mut file = File::create(path).unwrap();
    let mut ans: Vec<_> = (0..n).map(|_| rand::random::<bool>()).collect();
    ans[0] = true;
    ans[n - 1] = false;
    let m = n * (n - 1) / 2;
    file.write_all(format!("{} {}\n", n, m).as_bytes()).unwrap();

    for from in 0..n {
        for to in from + 1..n {
            let cost = if ans[from] == ans[to] {
                rand::random::<u32>() % 1000
            } else {
                rand::random::<u32>() % 50
            };
            file.write_all(format!("{} {} {}\n", from, to, cost).as_bytes())
                .unwrap();
        }
    }
}
