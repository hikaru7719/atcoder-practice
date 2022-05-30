use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    
    let mut cnt: Vec<Vec<i32>> = vec![vec![0;10];10];
    for ss in s.into_iter() {
        let chars: Vec<char> = ss.chars().collect();
        for (j,cc) in  chars.into_iter().enumerate(){
            let i = (cc as i32 -48) as usize;
            cnt[i][j] +=1; 
        }
    }

    let mut ans = 100000;
    for i in 0..10 {
        let mut mx = 0;
        for j in 0..10 {
            mx = std::cmp::max(mx, 10 * (cnt[i][j] -1) + j as i32);
        }
        ans = std::cmp::min(ans, mx);
    }
    println!("{}", ans);
}
