use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    };

    println!("{}", solve(a, b, c, x));
}

fn solve(a: u32, b: u32, c: u32, x: u32) -> u32 {
    let mut ans = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if 500 * i + 100 * j + 50 * k == x {
                    ans += 1;
                }
            }
        }
    }
    ans
}

#[test]
fn sample1() {
    assert_eq!(
        solve(2, 2, 2, 100),
        2
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(5, 1, 0, 150),
        0
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(30, 40, 50, 6000),
        213
    );
}
