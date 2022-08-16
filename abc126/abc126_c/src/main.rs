use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    println!("{:.12}", solve(n, k));
}

fn solve(n: usize, k: usize) -> f64 {
    let mut ans = 0.0;
    for i in 1..=n {
        let mut x = i;
        let mut q = 1.0;
        while x < k {
            x *= 2;
            q /= 2.0;
        }
        // println!("{} {}", x, q);
        ans += q / n as f64;
    }
    ans
}

#[test]
fn sample1() {
    assert!(
        (solve(3, 10) - 0.145833333333).abs() < 1e-9
    );
}

#[test]
fn sample2() {
    assert!(
        (solve(100000, 5) - 0.999973749998).abs() < 1e-9
    );
}
