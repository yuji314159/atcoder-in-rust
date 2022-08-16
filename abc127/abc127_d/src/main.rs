use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        bc: [(usize, usize); m],
    };
    println!("{}", solve(n, m, a, bc));
}

fn solve(n: usize, _m: usize, a: Vec<usize>, mut bc: Vec<(usize, usize)>) -> u64 {
    // for (b, c) in bc {
    //     for _ in 0..b {
    //         a.push(c);
    //     }
    // }
    for a in a {
        bc.push((1, a));
    }
    // println!("{:?}", bc);
    bc.sort_by_key(|bc| bc.1);
    // println!("{:?}", bc);

    let mut ans: u64 = 0;
    let mut cnt = 0;
    for &(b, c) in bc.iter().rev() {
        ans += c as u64 * b.min(n - cnt) as u64;
        cnt += b;
        if cnt >= n {
            break;
        }
    }
    ans
}

#[test]
fn sample1() {
    assert_eq!(
        solve(3, 2, vec![5, 1, 4], vec![(2, 3), (1, 5)]),
        14
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(10, 3, vec![1, 8, 5, 7, 100, 4, 52, 33, 13, 5], vec![(3, 10), (4, 30), (1, 4)]),
        338
    );
}

#[test]
fn sample3() {
    assert_eq!(
        solve(3, 2, vec![100, 100, 100], vec![(3, 99), (3, 99)]),
        300
    );
}

#[test]
fn sample4() {
    assert_eq!(
        solve(11, 3, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![(3, 1000000000), (4, 1000000000), (3, 1000000000)]),
        10000000001
    );
}
