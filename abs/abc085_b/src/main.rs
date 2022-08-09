use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i32; n],
    };

    println!("{}", solve(d));
}

fn solve(d: Vec<i32>) -> usize {
    let mut d = Vec::clone(&d);
    d.sort();
    d.dedup();
    d.len()
}

#[test]
fn sample1() {
    assert_eq!(
        solve(vec![10, 8, 8, 6]),
        3
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(vec![15, 15, 15]),
        1
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(vec![50, 30, 50, 100, 50, 80, 30]),
        4
    );
}
