use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };

    println!("{}", solve(a, b));
}

fn solve(a: u32, b: u32) -> String {
    if a * b % 2 == 0 {
        String::from("Even")
    } else {
        String::from("Odd")
    }
}

#[test]
fn sample1() {
    assert_eq!(
        solve(3, 4),
        String::from("Even")
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(1, 21),
        String::from("Odd")
    );
}
