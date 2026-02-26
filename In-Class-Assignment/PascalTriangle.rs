impl Solution 
{
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>>
     {

        let n:usize = num_rows as usize;
        let mut res: Vec<Vec<i32>> = vec![];


        for i in 0..n
        {

            let mut row: Vec<i32> = vec![1; i + 1];
            for j in 1..i
            {
                // row[j]
                row[j] = res[i-1][j] + res[i-1][j-1];
            }
            res.push(row);
        }
        res
    }

}
