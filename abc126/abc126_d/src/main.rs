use proconio::input;

fn main() {
    input! {
        n: usize,
        uvw: [(usize, usize, u32); n - 1],
    };
    let ans = solve(n, uvw);
    for c in ans {
        println!("{}", c);
    }
}

fn solve(n: usize, uvw: Vec<(usize, usize, u32)>) -> Vec<u32> {
    let mut edges: Vec<Vec<(usize, u32)>> = vec![vec![]; n];
    for (u, v, w) in uvw {
        edges[u - 1].push((v - 1, w));
        edges[v - 1].push((u - 1, w));
    }

    let mut ans = vec![None; n];
    ans[0] = Some(0);
    dfs(0, &mut ans, &edges);
    ans.iter().map(|x| x.unwrap_or(99)).collect()
}

fn dfs(u: usize, ans: &mut Vec<Option<u32>>, edges: &Vec<Vec<(usize, u32)>>) -> () {
    for &(v, w) in &edges[u] {
        if ans[v] != None {
            continue;
        }
        ans[v] = Some((ans[u].unwrap() + w) % 2);
        dfs(v, ans, edges);
    }
}

#[test]
fn sample1() {
    assert_eq!(
        solve(3, vec![(1, 2, 2), (2, 3, 1)]),
        vec![0, 0, 1]
    );
}

#[test]
fn sample2() {
    assert_eq!(
        solve(5, vec![(2, 5, 2), (2, 3, 10), (1, 3, 8), (3, 4, 2)]),
        // vec![1, 0, 1, 0, 1]
        vec![0, 0, 0, 0, 0]
    );
}

#[test]
fn my1() {
    assert_eq!(
        solve(1, vec![]),
        vec![0]
    );
}
