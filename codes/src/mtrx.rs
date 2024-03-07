use std::io;
use nalgebra::{DMatrix, Matrix2x3, Matrix3, Matrix4, Matrix5, Vector2, Vector3};

fn main() {
    println!("Are you doing 'mtrx' (matrix) or 'vec' (vector) things?");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    match choice.trim() {
        "mtrx" => handle_matrix(),
        "vec" => handle_vector(),
        _ => println!("Invalid choice"),
    }
}

fn handle_matrix() {
    println!("Enter the operation (rref, det, inv, add, sub, trans, gauss):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).unwrap();
    match operation.trim() {
        "rref" => {
            let matrix = read_matrix();
            println!("RREF: {:?}", matrix.rref());
        },
        "det" => {
            let matrix = read_matrix();
            println!("Determinant: {}", matrix.determinant());
        },
        "inv" => {
            let matrix = read_matrix();
            match matrix.try_inverse() {
                Some(inv) => println!("Inverse: {:?}", inv),
                None => println!("Matrix is not invertible"),
            }
        },
        "add" => {
            let matrix1 = read_matrix();
            let matrix2 = read_matrix();
            println!("Sum: {:?}", &matrix1 + &matrix2);
        },
        "sub" => {
            let matrix1 = read_matrix();
            let matrix2 = read_matrix();
            println!("Difference: {:?}", &matrix1 - &matrix2);
        },
        "trans" => {
            let matrix = read_matrix();
            println!("Transpose: {:?}", matrix.transpose());
        },
        "gauss" => {
            let matrix = read_matrix();
            println!("Gaussian Elimination: {:?}", matrix.rref());
        },
        _ => println!("Invalid operation"),
    }
}

fn handle_vector() {
    println!("Enter the operation (cross, dot):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).unwrap();
    match operation.trim() {
        "cross" => {
            let vector1 = read_vector();
            let vector2 = read_vector();
            println!("Cross Product: {:?}", vector1.cross(&vector2));
        },
        "dot" => {
            let vector1 = read_vector();
            let vector2 = read_vector();
            println!("Dot Product: {}", vector1.dot(&vector2));
        },
        _ => println!("Invalid operation"),
    }
}

fn read_matrix() -> DMatrix<f32> {
    println!("Enter matrix dimensions (e.g., 2x3):");
    let mut dimensions = String::new();
    io::stdin().read_line(&mut dimensions).unwrap();
    let dims: Vec<usize> = dimensions.trim().split('x').map(|d| d.parse().unwrap()).collect();
    let mut matrix_values = vec![0.0; dims[0] * dims[1]];
    for i in 0..dims[0] {
        for j in 0..dims[1] {
            println!("Enter value for ({}, {}):", i + 1, j + 1);
            let mut value = String::new();
            io::stdin().read_line(&mut value).unwrap();
            matrix_values[i * dims[1] + j] = value.trim().parse().unwrap();
        }
    }
    DMatrix::from_row_slice(dims[0], dims[1], &matrix_values)
}

fn read_vector() -> Vector3<f32> {
    let mut vector_values = vec![0.0; 3];
    for i in 0..3 {
        println!("Enter value for element {}:", i + 1);
        let mut value = String::new();
        io::stdin().read_line(&mut value).unwrap();
        vector_values[i] = value.trim().parse().unwrap();
    }
    Vector3::from_column_slice(&vector_values)
}
