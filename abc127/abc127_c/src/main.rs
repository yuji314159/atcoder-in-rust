use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); m]
    };
    println!("{}", solve(n, m, lr));
}

fn solve(n: usize, _m: usize, lr: Vec<(usize, usize)>) -> usize {
    let (mut a, mut b) = (0, n);
    for (l, r) in lr {
        a = a.max(l - 1);
        b = b.min(r);
    }
    b.saturating_sub(a)

}

#[test]
fn sample1() {
    assert_eq!(
        solve(4, 2, vec![(1, 3), (2, 4)]),
        2
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(10, 3, vec![(3, 6), (5, 7), (6, 9)]),
        1   
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(100000, 1, vec![(1, 100000)]),
        100000
    );
}

#[test]
fn my1() {
    assert_eq!(
        solve(5, 2, vec![(1, 2), (4, 5)]),
        0
    );
}
