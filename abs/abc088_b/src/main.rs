use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    };

    println!("{}", solve(a));
}

fn solve(a: Vec<i32>) -> i32 {
    let mut a = Vec::clone(&a);
    a.sort();

    let mut alice = 0;
    let mut bob = 0;
    for (i, a) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += a;
        } else {
            bob += a;
        }
    }
    (alice - bob).abs()
}

#[test]
fn sample1() {
    assert_eq!(
        solve(vec![3, 1]),
        2
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(vec![2, 7, 4]),
        5
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(vec![20, 18, 2, 18]),
        18
    );
}
