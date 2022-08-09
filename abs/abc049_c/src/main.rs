use proconio::input;

fn main() {
    input! {
        s: String,
    };

    println!("{}", solve(s));
}

fn solve(s: String) -> String {
    let mut i = s.len();
    while i > 0 {
        if i >= 7 && &s[i - 7..i] == "dreamer" {
            i -= 7;
        } else if i >= 6 && &s[i - 6..i] == "eraser" {
            i -= 6;
        } else if i >= 5 && &s[i - 5..i] == "dream" ||  &s[i - 5..i] == "erase" {
            i -= 5;
        } else {
            return  String::from("NO")
        }
    }
    String::from("YES")
}

#[test]
fn sample1() {
    assert_eq!(
        solve(String::from("erasedream")),
        String::from("YES")
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(String::from("dreameraser")),
        String::from("YES")
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(String::from("dreamerer")),
        String::from("NO")
    );
}
