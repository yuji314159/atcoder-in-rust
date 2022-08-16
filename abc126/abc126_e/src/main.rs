use proconio::input;
use petgraph::unionfind::UnionFind;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(usize, usize, usize); m],
    };
    println!("{}", solve(n, xyz));
}

fn solve(n: usize, xyz: Vec<(usize, usize, usize)>) -> usize {
    let mut uf = UnionFind::new(n);
    for (x, y, _z) in xyz {
        uf.union(x - 1, y - 1);
    }
    (0..n).map(|i| (uf.find(i) == i) as usize).sum::<usize>()
}

#[test]
fn sample1() {
    assert_eq!(
        solve(3, vec![(1, 2, 1)]),
        2
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(6, vec![(1, 2, 1), (2, 3, 2), (1, 3, 3), (4, 5, 4), (5, 6, 5)]),
        2
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(100000, vec![(1, 100000, 100)]),
        99999
    );
}
