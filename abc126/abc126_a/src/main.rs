use proconio::input;

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: String,
    };
    println!("{}", solve(k, s));
}

fn solve(k: usize, s: String) -> String {
    let mut s: Vec<char> = s.chars().collect();
    s[k - 1] = s[k - 1].to_ascii_lowercase();
    s.iter().collect()
}

#[test]
fn sample1() {
    assert_eq!(
        solve(1, String::from("ABC")),
        String::from("aBC")
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(3, String::from("CABA")),
        String::from("CAbA")
    );
}
