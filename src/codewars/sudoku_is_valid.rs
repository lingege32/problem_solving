struct Sudoku{
    data: Vec<Vec<u32>>,
}


impl Sudoku{
    fn is_valid(&self) -> bool {
        let data  = &self.data;
        let len = data.len();
        if data.iter().any(|v| v.len() != len) {
            return false;
        }
        let step = (0..len).find(|&x| x*x == len).unwrap();
        for line_idx in 0..len {
            let mut tmp = vec![0; len+1];
            for other_idx in 0..len {
                let val = data[line_idx][other_idx] as usize;
                if tmp[val] == 0 {
                    tmp[val] = 1;
                }
                else {
                    return false;
                }
            }
        }
        for line_idx in 0..len {
            let mut tmp = vec![0; len+1];
            for other_idx in 0..len {
                let val = data[other_idx][line_idx] as usize;
                if tmp[val] == 0 {
                    tmp[val] = 1;
                }
                else {
                    return false;
                }
            }
        }
        for col_begin in (0..len).step_by(step) {
            for row_begin in (0..len).step_by(step) {
                let mut tmp = vec![0; len+1];
                for c_idx in (0..step).map(|x| x+col_begin) {
                    for r_idx in (0..step).map(|x| x+row_begin) {
                        let val = data[c_idx][r_idx] as usize;
                        if tmp[val] == 0 {
                            tmp[val] = 1;
                        }
                        else{
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}
#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku{
        data: vec![
                vec![7,8,4, 1,5,9, 3,2,6],
                vec![5,3,9, 6,7,2, 8,4,1],
                vec![6,1,2, 4,3,8, 7,5,9],

                vec![9,2,8, 7,1,5, 4,6,3],
                vec![3,5,7, 8,4,6, 1,9,2],
                vec![4,6,1, 9,2,3, 5,8,7],
                
                vec![8,7,6, 3,9,4, 2,1,5],
                vec![2,4,3, 5,6,1, 9,7,8],
                vec![1,9,5, 2,8,7, 6,3,4]
            ]
    };
    
    let good_sudoku_2 = Sudoku{
        data: vec![
                vec![1, 4,  2, 3],
                vec![3, 2,  4, 1],
        
                vec![4, 1,  3, 2],
                vec![2, 3,  1, 4],
            ]
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku{
        data: vec![
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],

                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
                vec![1,2,3, 4,5,6, 7,8,9],
            ]
    };
    
    let bad_sudoku_2 = Sudoku{
        data: vec![
                vec![1,2,3,4,5],
                vec![1,2,3,4],
                vec![1,2,3,4],
                vec![1],
            ]
    };
    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}