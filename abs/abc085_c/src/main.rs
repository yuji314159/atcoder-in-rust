use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    };

   let ans = solve(n, y);
    println!("{} {} {}", ans.0, ans.1, ans.2);
}

fn solve(n: i32, y: i32) -> (i32, i32, i32) {
    for a in 0..=n {
        for b in 0..=n - a {
            let c = n - a - b;
            if a * 10000 + b * 5000 + c * 1000 == y {
                return (a, b, c)
            }
        }
    }
    (-1, -1, -1)
}

#[test]
fn sample1() {
    let actual = solve(9, 45000);
    assert_eq!(
        actual.0 * 10000 + actual.1 * 5000 + actual.2 * 1000,
        45000
    );
}

#[test]
fn sample2() {
    let actual = solve(20, 196000);
    assert_eq!(actual, (-1, -1, -1));
}

#[test]
fn sample3() {
    let actual = solve(1000, 1234000);
    assert_eq!(
        actual.0 * 10000 + actual.1 * 5000 + actual.2 * 1000,
        1234000
    );
}

#[test]
fn sample4() {
    let actual = solve(2000, 20000000);
    assert_eq!(
        actual.0 * 10000 + actual.1 * 5000 + actual.2 * 1000,
        20000000
    );
}
