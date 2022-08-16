use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    println!("{}", solve(a, b));
}

fn solve(a: usize, b: usize) -> usize {
    if a <= 5 {
        0
    } else if a <= 12 {
        b / 2
    } else {
        b
    }
}

#[test]
fn sample1() {
    assert_eq!(
        solve(30, 100),
        100
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(12, 100),
        50
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(0, 100),
        0
    );
}
