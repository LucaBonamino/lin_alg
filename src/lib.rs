mod matrix;
mod gf2_matrix;

#[cfg(test)]
mod tests {
    use crate::matrix::MatrixTrait;

    use super::*;

    #[test]
    fn instantiate_matrix() {
        let mat = gf2_matrix::GF2Matrix::new(vec![vec![1,0], vec![1,1]]);
        let el = mat.elements;
        assert_eq!(el, vec![vec![1,0], vec![1,1]]);
    }

    #[test]
    fn  echelon_form_matrix() {
        let mat = gf2_matrix::GF2Matrix::new(vec![vec![1,0], vec![1,1]]);
        let (ech_form, _) = mat.echelon_form();     
        assert_eq!(ech_form.elements, vec![vec![1,0], vec![0,1]]);
   
    }

    #[test]
    fn echelon_form_ops() {
        let mat = gf2_matrix::GF2Matrix::new(vec![vec![1,0], vec![1,1]]);
        let (_, ops) = mat.echelon_form();
        assert_eq!(ops, vec![(1,0)])
    }
    
}
