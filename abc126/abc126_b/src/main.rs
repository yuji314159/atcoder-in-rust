use proconio::input;

fn main() {
    input! {
        s: String,
    };
    println!("{}", solve(s));
}

fn solve(s: String) -> String {
    let s = s.parse::<usize>().unwrap();
    let (a, b) = (s / 100, s % 100);
    match (1 <= a && a <= 12, 1 <= b && b <= 12) {
        (true, true) => String::from("AMBIGUOUS"),
        (true, false) => String::from("MMYY"),
        (false, true) => String::from("YYMM"),
        (false, false) => String::from("NA"),
    }
}

#[test]
fn sample1() {
    assert_eq!(
        solve(String::from("1905")),
        String::from("YYMM")
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(String::from("0112")),
        String::from("AMBIGUOUS")
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(String::from("1700")),
        String::from("NA")
    );
}
