use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };

    println!("{}", solve(a));
}

fn solve(a: Vec<u32>) -> u32 {
    a.iter().map(|a| a.trailing_zeros()).min().unwrap()
}

#[test]
fn sample1() {
    assert_eq!(
        solve(vec![8, 12, 40]),
        2
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(vec![5, 6, 8, 10]),
        0
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(vec![382253568, 723152896, 37802240, 379425024, 404894720, 471526144]),
        8
    );
}
