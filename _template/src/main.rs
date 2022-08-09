use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        s: String,
    };

    let ans = solve(a, b, c, s);
    println!("{} {}", ans.0, ans.1);
}

fn solve(a: u32, b: u32, c: u32, s: String) -> (u32, String) {
    (a + b + c, s)
}

#[test]
fn sample1() {
    assert_eq!(
        solve(1, 2, 3, String::from("test")),
        (6, String::from("test"))
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(72, 128, 256, String::from("myonmyon")),
        (456, String::from("myonmyon"))
    );
}
