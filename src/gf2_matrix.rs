use crate::matrix::MatrixTraid;

struct GF2Matrix{
    elements: Vec<Vec<u8>>
}

impl GF2Matrix {
    fn new(elements: Vec<Vec<u8>>) -> Self{
        Self { elements}
    }

    fn is_echelon(&self) -> bool{
        unimplemented!()
    }

    fn rank_echelon_form(&self) -> usize{
        if self.is_echelon(){
            return self.rank()
        }
        else {
            let ech_form = self.echelon_form();
            return ech_form.rank();   
        }

    }

    fn kernel_echelon_form(&self)-> Vec<Vec<u8>>{
        if self.is_echelon(){
            return self.kernel();
        }
        else {
            let ech_form = self.echelon_form();
            return ech_form.kernel();
        }
    }
}

impl MatrixTraid<u8> for GF2Matrix  {

    fn rank(&self)-> usize {
        unimplemented!()
    }

    fn echelon_form(&self) -> Self {
        unimplemented!()
    }

    fn kernel(&self) -> Vec<Vec<u8>> {
        unimplemented!()
    }
    
}