use crate::matrix::{MatrixTrait, Matrix};

type GF2Matrix = Matrix<u8>;

impl GF2Matrix {
    

    fn is_echelon(&self) -> bool{
        unimplemented!()
    }

    fn rank_non_echelon_form(&self) -> usize{
        if self.is_echelon(){
            return self.rank()
        }
        else {
            let ech_form = self.echelon_form();
            return ech_form.rank();   
        }

    }

    fn kernel_non_echelon_form(&self)-> Vec<Vec<u8>>{
        if self.is_echelon(){
            return self.kernel();
        }
        else {
            let ech_form = self.echelon_form();
            return ech_form.kernel();
        }
    }

}

impl MatrixTrait<u8> for GF2Matrix  {

    fn get_pivot(row: &Vec<u8>) -> Option<usize> {
        row.iter().position(|&x| x == 1)
    }

    fn nrows(&self) -> usize {
        self.elements.len()
    }

    fn ncols(&self) -> usize {
        if !self.elements.is_empty() {
            self.elements[0].len()
        } else {
            0
        }
    }

    fn rank(&self) -> usize {
        let mut count = 0;
        let mut pivot_columns = std::collections::HashSet::new();

        for i in 0..self.nrows() {
            let p = GF2Matrix::get_pivot(&self.elements[i]);
            if let Some(col) = p {
                if pivot_columns.insert(col) {
                    count += 1;
                }
            }
        }
        count
    }

    fn echelon_form(&self) -> Self {
        unimplemented!()
    }

    fn kernel(&self) -> Vec<Vec<u8>> {
        let rows = self.nrows();
        let cols = self.ncols();

        let mut pivots = std::collections::HashMap::new();
        let mut kernel_base: Vec<Vec<u8>> = Vec::new();
        let mut free_columns: Vec<usize> = Vec::new();
        let mut row_index = 0;

        for j in 0..cols {
            if row_index < rows && self.elements[row_index][j] == 1 {
                pivots.insert(j, row_index);
                row_index += 1;
            } else {
                free_columns.push(j);
            }
        }

        for &free_col in &free_columns {
            let mut kernel_vector = vec![0; cols];
            kernel_vector[free_col] = 1;

            for (&p_index, &p_row) in &pivots {
                let mut sum = 0;

                for col in (0..cols).rev() {
                    if col != p_index {
                        sum = sum ^ (self.elements[p_row][col] * kernel_vector[col]);
                    }
                }

                kernel_vector[p_index] = sum;
            }

            kernel_base.push(kernel_vector);
        }

        kernel_base
    }
    
}