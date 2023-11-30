

fn main() {
    //Matriz a factorizar
    let matrix = vec![
        vec![2.0, 3.0, -1.0],
        vec![4.0, 1.0, 2.0],
        vec![3.0, 2.0, 3.0],
    ];

    //Llamamos a la función de factorización
    let (lower, upper) = lu_factorization(&matrix);

    //Mostramos las matrices
    println!("Matriz Original:");
    display_matrix(&matrix);

    println!("Matriz triangular inferior:");
    display_matrix(&lower);

    println!("Matriz triangular superior:");
    display_matrix(&upper);
}

fn lu_factorization(matrix: &Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let n = matrix.len();
    let mut lower = vec![vec![0.0; n]; n];
    let mut upper = vec![vec![0.0; n]; n];

    for i in 0..n {
        // Upper Triangular Matrix
        for j in i..n {
            let sum: f64 = (0..i).map(|k| lower[i][k] * upper[k][j]).sum();
            upper[i][j] = matrix[i][j] - sum;
        }

        // Lower Triangular Matrix
        for j in i..n {
            if i == j {
                lower[i][i] = 1.0;
            } else {
                let sum: f64 = (0..i).map(|k| lower[j][k] * upper[k][i]).sum();
                lower[j][i] = (matrix[j][i] - sum) / upper[i][i];
            }
        }
    }

    (lower, upper)
}


fn display_matrix(matrix: &Vec<Vec<f64>>) {
    for row in matrix {
        for &elem in row {
            print!("{:.4} ", elem);
        }
        println!();
    }
}

