use crate::matrix::{MatrixTrait, Matrix};

pub type GF2Matrix = Matrix<u8>;

impl MatrixTrait<u8> for GF2Matrix {
    

    fn is_reduced_echelon(&self) -> bool{
        let nrows = self.nrows();
        let mut old_piv = 0;
        let mut found_zero_row = false;
        for i in 0..nrows{
            let piv = GF2Matrix::get_pivot(&self.elements[i]);
            match piv {
                None => {
                    found_zero_row = true;
                    continue;
                }
                Some(piv) => {
                    if piv < old_piv || found_zero_row == true {
                        return false;
                    }
                    for j in 0..nrows {
                        if j != i && self.elements[j][piv] != 0 {
                            return false;
                        }
                    }
                    old_piv = piv;

                }
            }
        }
        return true;
    }

    fn rank(&self) -> usize{
        if self.is_reduced_echelon(){
            return self.rank_echelon_form()
        }
        else {
            let (ech_form, _) = self.echelon_form();
            return ech_form.rank_echelon_form();   
        }

    }

    fn kernel(&self)-> Vec<Vec<u8>>{
        if self.is_reduced_echelon(){
            println!("{:?}", self.elements);
            return self.kernel_echelon_form();
        }
        else {
            let (ech_form, _) = self.echelon_form();
            println!("{:?}", ech_form);
            return ech_form.kernel_echelon_form();
        }
    }

    fn echelon_form(&self) -> (Self, Vec<(usize, usize)>) {
        let mut m_copy = self.clone();
        let rows = m_copy.nrows();
        let cols = m_copy.ncols();
        let mut operations: Vec<(usize, usize)> = Vec::new();
        let mut lead = 0;

        for r in 0..rows {
            if lead >= cols {
                break;
            }
            let mut i = r;
            while m_copy.elements[i][lead] == 0 {
                i += 1;
                if i == rows {
                    i = r;
                    lead += 1;
                    if lead == cols {
                        return (m_copy, operations);
                    }
                }
            }
            m_copy.swap_rows(r, i);
            if r != i {
                operations.push((r, i));
                operations.push((i, r));
                operations.push((r, i));
            }
            for i in 0..rows {
                if i != r && m_copy.elements[i][lead] == 1 {
                    for j in 0..cols {
                        m_copy.elements[i][j] = (m_copy.elements[i][j] + m_copy.elements[r][j]) % 2;
                    }
                    operations.push((i, r));
                }
            }
            lead += 1;
        }

        (m_copy, operations)
    }

    fn get_pivot(row: &Vec<u8>) -> Option<usize> {
        row.iter().position(|&x| x == 1)
    }

    fn image(&self) -> Vec<Vec<u8>>{
        let mat = if !self.is_reduced_echelon() {
            let (m, _) = self.echelon_form();
            m
        } else {
            self.clone()
        };
        let mut image_base: Vec<Vec<u8>> = Vec::new();
        for i in 0..mat.nrows() {
            let row = mat.elements[i].clone();
            let piv = GF2Matrix::get_pivot(&row);
            if !piv.is_none() {
                image_base.push(row);
            }
        }
        image_base
    }


}

impl GF2Matrix  {

    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.elements.swap(row1, row2);
    }

    fn rank_echelon_form(&self) -> usize {
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


    fn kernel_echelon_form(&self) -> Vec<Vec<u8>> {
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