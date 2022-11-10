fn main() {
    println!("Hello, world!");
}

struct Solution{}


impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        fn remove_island(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
            if !(0..grid.len()).contains(&i) || !(0..grid[0].len()).contains(&j) || grid[i][j] != '1' {
                return
            }
            grid[i][j] = '0';
            remove_island(grid, i+1, j);
            remove_island(grid, i-1, j);
            remove_island(grid, i, j+1);
            remove_island(grid, i, j-1);
        }
        let mut count = 0;
        for i in (0..grid.len()) {
            for j in (0..grid[0].len()) {
                if grid[i][j] == '1' {
                    count += 1;
                    remove_island(&mut grid, i, j);
                }
            }
        }
        count
    }
}
