use proconio::input;

fn main() {
    input! {
        r: usize,
        d: usize,
        x: usize,
    };
    let ans = solve(r, d, x);
    for ans in ans {
        println!("{}", ans);
    }
}

fn solve(r: usize, d: usize, mut x: usize) -> Vec<usize> {
    let mut ans = vec![];
    for _ in 0..10 {
        x = r * x - d;
        ans.push(x);
    }
    ans
}

#[test]
fn sample1() {
    assert_eq!(
        solve(2, 10, 20),
        vec![30, 50, 90, 170, 330, 650, 1290, 2570, 5130, 10250]
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(4, 40, 60),
        vec![200, 760, 3000, 11960, 47800, 191160, 764600, 3058360, 12233400, 48933560]
    );
}
