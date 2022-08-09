use proconio::input;

fn main() {
    input! {
        n: usize,
        txy: [(i32, i32, i32); n]
    };

    println!("{}", solve(txy));
}

fn solve(txy: Vec<(i32, i32, i32)>) -> String {
    let (mut t, mut x, mut y) = (0, 0, 0);
    for txy in txy.iter() {
        let len = (txy.1 - x).abs() + (txy.2 - y).abs();
        if len <= txy.0 - t && len % 2 == (txy.0 - t) % 2 {
            // (t, x, y) = *txy;
            t = txy.0;
            x = txy.1;
            y = txy.2;
        } else {
            return String::from("No");
        }
    }
    String::from("Yes")
}

#[test]
fn sample1() {
    assert_eq!(
        solve(vec![(3, 1, 2), (6, 1, 1)]),
        String::from("Yes")
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(vec![(1, 100, 100)]),
        String::from("No")
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(vec![(5, 1, 1), (100, 1, 1)]),
        String::from("No")
    );
}
