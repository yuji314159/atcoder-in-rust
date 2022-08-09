use proconio::input;

fn main() {
    input! {
        s: String,
    };

    println!("{}", solve(s));
}

fn solve(s: String) -> usize {
    s.chars().filter(|&c| c == '1').count()
}

#[test]
fn sample1() {
    assert_eq!(
        solve(String::from("101")),
        2
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(String::from("000")),
        0
    );
}
