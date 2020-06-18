use std::collections::VecDeque;

pub fn schriver_algorithm<F>(base_size: usize, f: &F) -> (Vec<bool>, f64)
where
    F: Fn(&[bool]) -> f64,
{
    let n = base_size;
    let mut coeffs: Vec<f64> = vec![1.0];
    let mut bs: Vec<Vec<f64>> = vec![calc_b(f, &(0..n).collect::<Vec<usize>>())];
    let mut ords: Vec<Vec<usize>> = vec![(0..n).collect()];
    let mut x = bs[0].clone();
    loop {
        let positive: Vec<usize> = (0..n).filter(|i| x[*i] > 0.0).collect();
        let (distance, before) = calc_distance(n, &ords, &positive);
        let unreachable: Vec<_> = (0..n).map(|v| distance[v] == n + 1).collect();
        if (0..n).all(|ind| x[ind] >= 0.0 || unreachable[ind]) {
            let optimal_value = f(&unreachable);
            return (unreachable, optimal_value);
        }
        let mut farest = n;
        for i in 0..n {
            if !unreachable[i] && x[i] < 0.0 && (farest == n || distance[farest] <= distance[i]) {
                farest = i;
            }
        }
        let before_farest = before[farest];
        let mut most_differ_i = 0;
        let mut diff = 0;
        let mut before_farest_ind = 0;
        let mut farest_ind = 0;
        for (i, ord) in ords.iter().enumerate() {
            let mut bfi = 0;
            while ord[bfi] != before_farest {
                bfi += 1;
            }
            let mut fi = 0;
            while ord[fi] != farest {
                fi += 1;
            }
            if bfi < fi && diff <= fi - bfi {
                diff = fi - bfi;
                most_differ_i = i;
                farest_ind = fi;
                before_farest_ind = bfi;
            }
        }
        let mut epsilon = -x[farest];

        let mut diff = vec![0.0; n];

        diff[farest] += epsilon;
        diff[before_farest] -= epsilon;
        let mut new_coeffs = vec![];
        let mut new_bs = vec![];
        let mut new_ords = vec![];
        let now_b = calc_b(f, &ords[most_differ_i]);
        let mut delta = 0.0;
        for i in (before_farest_ind + 1..=farest_ind).rev() {
            let mut moved_ord = ords[most_differ_i].clone();
            for j in (before_farest_ind..i).rev() {
                moved_ord.swap(j, j + 1);
            }
            let new_b = calc_b(f, &moved_ord);
            let increased_node = ords[most_differ_i][i];

            if (new_b[increased_node] - now_b[increased_node]).abs() < 1e-10 {
                delta = -1.0;
                ords[most_differ_i] = moved_ord;
                break;
            }
            let coeff = diff[increased_node] / (new_b[increased_node] - now_b[increased_node]);
            for j in before_farest_ind..=i {
                let decreased_node = ords[most_differ_i][j];
                //print!("{}-> ", decreased_node);
                diff[decreased_node] -= coeff * (new_b[decreased_node] - now_b[decreased_node]);
            }
            new_bs.push(new_b);
            new_coeffs.push(coeff);
            new_ords.push(moved_ord);
            delta += coeff;
        }

        if delta < -0.5 {
            continue;
        }

        if delta > coeffs[most_differ_i] {
            for c in new_coeffs.iter_mut() {
                *c *= coeffs[most_differ_i] / delta;
            }
            epsilon *= coeffs[most_differ_i] / delta;
            delta = coeffs[most_differ_i];
        }
        coeffs[most_differ_i] -= delta;
        bs.extend(new_bs);
        coeffs.extend(new_coeffs);
        ords.extend(new_ords);
        x[before_farest] -= epsilon;
        x[farest] += epsilon;
        squash(&mut coeffs, &mut bs, &mut ords);
    }
}

fn remove_zero(coeffs: &mut Vec<f64>, bs: &mut Vec<Vec<f64>>, ords: &mut Vec<Vec<usize>>) {
    let mut nonzero_coeffs = vec![];
    let mut nonzero_bs = vec![];
    let mut nonzero_ords = vec![];

    for i in 0..coeffs.len() {
        let coeff = coeffs[i];
        if coeff.abs() < 1e-10 {
            continue;
        }
        nonzero_coeffs.push(coeff);
        nonzero_bs.push(bs[i].clone());
        nonzero_ords.push(ords[i].clone());
    }
    *coeffs = nonzero_coeffs;
    *bs = nonzero_bs;
    *ords = nonzero_ords;
}

fn squash(coeffs: &mut Vec<f64>, bs: &mut Vec<Vec<f64>>, ords: &mut Vec<Vec<usize>>) {
    remove_zero(coeffs, bs, ords);
    if bs.is_empty() {
        return;
    }

    let n = bs[0].len();
    let m = bs.len();
    let mut basis = vec![vec![0.0; m]; n];

    //Transpose
    for (i, row) in basis.iter_mut().enumerate() {
        for (j, elem) in row.iter_mut().enumerate() {
            *elem = bs[j][i];
        }
    }

    let mut rank = 0;
    for column in 0..m {
        let mut row = rank;
        while row < n && basis[row][column].abs() < 1e-10 {
            row += 1;
        }
        if row < n {
            basis.swap(rank, row);
            bs.swap(column, rank);
            ords.swap(column, rank);
            coeffs.swap(column, rank);
            basis.iter_mut().for_each(|row| row.swap(column, rank));
            swipe(&mut basis, rank);
            rank += 1;
        } else {
            let mut delta = coeffs[column];
            let mut pivot = column;
            for r in 0..rank {
                if basis[r][column] >= 0.0 {
                    continue;
                }
                if delta > -coeffs[r] / basis[r][column] {
                    delta = -coeffs[r] / basis[r][column];
                    pivot = r;
                }
            }
            if pivot != column {
                bs.swap(column, pivot);
                ords.swap(column, pivot);
                coeffs.swap(column, pivot);
                basis.iter_mut().for_each(|row| row.swap(column, pivot));
                swipe(&mut basis, pivot);
            }

            for r in 0..rank {
                coeffs[r] += coeffs[column] * basis[r][column];
            }
            coeffs[column] = 0.0;
        }
    }
    remove_zero(coeffs, bs, ords);
}

fn swipe(matrix: &mut Vec<Vec<f64>>, pivot: usize) {
    let ratio = matrix[pivot][pivot];
    matrix[pivot].iter_mut().for_each(|elem| *elem /= ratio);
    for r in 0..matrix.len() {
        if r != pivot {
            for c in 0..matrix[0].len() {
                if c != pivot {
                    matrix[r][c] -= matrix[pivot][c] * matrix[r][pivot];
                }
            }
            matrix[r][pivot] = 0.0;
        }
    }
}

fn calc_distance(n: usize, bs: &[Vec<usize>], positive: &[usize]) -> (Vec<usize>, Vec<usize>) {
    let mut distance: Vec<usize> = vec![n + 1; n];
    let mut before: Vec<usize> = vec![n + 1; n];
    for p in positive {
        distance[*p] = 0;
    }

    let mut edges = vec![vec![]; n];
    for b in bs {
        for i in 0..n - 1 {
            for j in i + 1..n {
                edges[b[i]].push(b[j]);
            }
        }
    }
    let mut q: VecDeque<usize> = positive.to_vec().into();
    while !q.is_empty() {
        let from = *q.back().unwrap();
        q.pop_back();
        for &to in edges[from].iter() {
            if distance[to] > distance[from] + 1
                || distance[to] == distance[from] + 1 && before[to] < from
            {
                before[to] = from;
                distance[to] = distance[from] + 1;
                q.push_back(to);
            }
        }
    }
    (distance, before)
}

fn calc_b<F>(f: F, order: &[usize]) -> Vec<f64>
where
    F: Fn(&[bool]) -> f64,
{
    let mut b = vec![0.0; order.len()];
    let mut xs = vec![false; order.len()];
    let mut last_value = f(&xs);
    for &x in order {
        xs[x] = true;
        let new_value = f(&xs);
        b[x] += new_value - last_value;
        last_value = new_value;
    }
    b
}
