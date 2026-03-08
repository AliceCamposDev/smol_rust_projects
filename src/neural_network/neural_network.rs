// use rand::prelude::*;
// pub struct Matrix {
//     pub rows: usize,
//     pub cols: usize,
//     pub data: Vec<f64>,
// }

// impl Matrix {
//     pub fn add(&self, other: &Matrix) -> Matrix {
//         if self.rows != other.rows || self.cols != other.cols {
//             panic!("Matrices must have the same dimensions for addition");
//         }
        
//         let mut result = Matrix {
//             rows: self.rows,
//             cols: self.cols,
//             data: vec![0.0; self.rows * self.cols],
//         };
//         for i in 0..self.data.len() {
//             result.data[i] = self.data[i] + other.data[i];
//         }
//         result
//     }
// }

// pub fn random(rows: usize, cols: usize) -> Matrix {
//     let mut buffer: Vec<f64> = Vec::<f64>::with_capacity(rows * cols);
//     let mut rng = rand::rng();
//     for _ in 0..rows * cols {
//         let num: f64 =rng.random_range(0.0..1.0);
//         buffer.push(num);
//     }
//     Matrix {
//         rows,
//         cols,
//         data: buffer,
//     }
// }



pub fn neural_network_fn() {}
