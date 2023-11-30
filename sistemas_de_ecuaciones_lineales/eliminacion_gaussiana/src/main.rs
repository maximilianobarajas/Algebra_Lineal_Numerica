fn main() {
    //Ejemplo a resolver en la rutina usual
    // 2x + 3y - z = 1
    // 4x + y + 2z = 2
    // 3x + 2y + 3z = 3
    let mut augmented_matrix = vec![
        vec![2.0, 3.0, -1.0, 1.0],
        vec![4.0, 1.0, 2.0, 2.0],
        vec![3.0, 2.0, 3.0, 3.0],
    ];
    //Llamamos a la función elmimnación Gaussiana
    gaussian_elimination(&mut augmented_matrix);
    //Mostramos la forma escalonada
    println!("Forma reducida escalonada:");
    display_matrix(&augmented_matrix);
    //Aplicamos la substitución hacía atrás
    let solution = backward_substitution(&augmented_matrix);
    //Mostramos la solución
    println!("Solución:");
    for i in 0..solution.len() {
        println!("x[{}]: {:.9}", i + 1, solution[i]);
    }
}
fn gaussian_elimination(matrix: &mut Vec<Vec<f64>>) {
    let n = matrix.len();
    for i in 0..n {
        let divisor = matrix[i][i];
        for j in 0..=n {
            matrix[i][j] /= divisor;
        }
        for k in 0..n {
            if k != i {
                let factor = matrix[k][i];
                for j in 0..=n {
                    matrix[k][j] -= factor * matrix[i][j];
                }
            }
        }
    }
}

fn backward_substitution(matrix: &Vec<Vec<f64>>) -> Vec<f64> {
    let n = matrix.len();
    let mut solution = vec![0.0; n];

    for i in (0..n).rev() {
        solution[i] = matrix[i][n];
        for j in (i + 1)..n {
            solution[i] -= matrix[i][j] * solution[j];
        }
    }

    solution
}

fn display_matrix(matrix: &Vec<Vec<f64>>) {
    for row in matrix {
        for &elem in row {
            print!("{:.4} ", elem);
        }
        println!();
    }
}


