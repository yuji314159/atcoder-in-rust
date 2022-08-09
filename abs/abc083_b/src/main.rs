use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    };

    println!("{}", solve(n, a, b));
}

fn solve(n: u32, a: u32, b: u32) -> u32 {
    let mut ans = 0;
    for i in 1..=n {
        let x = i % 10 + i / 10 % 10 + i / 100 % 10 + i / 1000 % 10 + i / 10000 % 10;
        if a <= x && x <= b {
            ans += i
        }
    }
    ans
}

#[test]
fn sample1() {
    assert_eq!(
        solve(20, 2, 5),
        84
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(10, 1, 2),
        13
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(100, 4, 16),
        4554
    );
}
