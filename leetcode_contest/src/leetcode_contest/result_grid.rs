struct Solution;
impl Solution {
    #[allow(dead_code)]
    pub fn result_grid(image: Vec<Vec<i32>>, threshold: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![Vec::<i32>::new(); image[0].len()]; image.len()];
        let row_len = image.len();
        let col_len = image[0].len();
        for r in 1..row_len - 1 {
            for c in 1..col_len - 1 {
                let n1 = image[r-1][c-1];
                let n2 = image[r-1][c];
                let n3 = image[r-1][c+1];
                let n4 = image[r][c-1];
                let n5 = image[r][c];
                let n6 = image[r][c+1];
                let n7 = image[r+1][c-1];
                let n8 = image[r+1][c];
                let n9 = image[r+1][c+1];
                if (n1-n2).abs()>threshold || (n1-n4).abs()>threshold || (n2-n3).abs()>threshold || (n2-n5).abs()>threshold || (n3-n6).abs()>threshold || (n4-n5).abs()>threshold || (n4-n7).abs()>threshold || (n5-n6).abs()>threshold || (n5-n8).abs()>threshold || (n6-n9).abs()>threshold || (n7-n8).abs()>threshold || (n8-n9).abs()>threshold  {
                    continue;
                }
                let total = (n1+n2+n3+n4+n5+n6+n7+n8+n9)/9;
                ret[r - 1][c - 1].push(total);
                ret[r - 1][c].push(total);
                ret[r - 1][c + 1].push(total);
                ret[r][c - 1].push(total);
                ret[r][c].push(total);
                ret[r][c + 1].push(total);
                ret[r + 1][c - 1].push(total);
                ret[r + 1][c].push(total);
                ret[r + 1][c + 1].push(total);
            }
        }
        let mut ans = image;
        for r in 0..row_len {
            for c in 0..col_len {
                let len = ret[r][c].len();
                if len != 0 {
                    ans[r][c] = ret[r][c].iter().sum::<i32>() / len as i32;
                }
            }
        }
        ans
    }
}
