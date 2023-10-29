use std::io;
use project::math_set::*;
use project::matrix::*;
use project::vector::*;
use project::logic::*;
use project::complex::*;

fn main() {
    println!("Which Math Object do you want to work on? ");
    println!("1. Sets.");
    println!("2. Matrices.");
    println!("3. Vectors.");
    println!("4. Logic operations.");
    println!("5. Complex numbers");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line.");

    match choice.trim() {
        "1" => {
            let mut num_sets = String::new();
            println!("How many sets do you want to operate on?");
            io::stdin().read_line(&mut num_sets).expect("Failed to read line");
            
            let num_sets: usize = match num_sets.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input. Please provide a positive integer.");
                    return;
                }
            };
        
            if num_sets == 0 {
                eprintln!("Number of sets must be greater than 0.");
                return;
            }
        
            if let Ok(sets) = read_sets_from_csv("sets.csv") {
                if sets.len() < num_sets {
                    eprintln!("Not enough sets in the file to perform operations.");
                    return;
                }
        
                let sets_to_use = &sets[0..num_sets];
                for (index, sets) in sets_to_use.iter().enumerate() {
                    println!("Set {}: ", index+1);
                    println!("{:?}", sets);
                }
        
                let mut union_result = sets_to_use[0].clone();
                let mut intersection_result = sets_to_use[0].clone();
                let mut difference_result = sets_to_use[0].clone();
            
                for set_index in 1..num_sets {
                    let current_set = &sets_to_use[set_index];
            
                    // Union
                    union_result = union_result.union(current_set);
            
                    // Intersection
                    intersection_result = intersection_result.intersection(current_set);
            
                    // Difference
                    difference_result = difference_result.difference(current_set);
                }
        
                println!("Union: {:?}", union_result);
                println!("Intersection: {:?}", intersection_result);
                println!("Difference: {:?}", difference_result);
        
            } else {
                eprintln!("Error reading sets from CSV file.");
            }

        }

        "2" => {
            if let Ok(matrices) = read_matrices_from_csv("matrix.csv") {
                if matrices.len() < 2 {
                    eprintln!("Not enough matrices in the file to perform operations.");
                    return;
                }
        
                let matrix_vec: Vec<Matrix> = matrices.iter().map(|m| m.clone()).collect();
        
                let mut num_sets = String::new();
                println!("How many sets of matrices do you want to operate on?");
                io::stdin().read_line(&mut num_sets).expect("Failed to read line");
        
                let num_sets: usize = match num_sets.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Invalid input. Please provide a positive integer.");
                        return;
                    }
                };
        
                let mut scalar_input = String::new();
                println!("Enter the scalar value: ");
                io::stdin().read_line(&mut scalar_input).expect("Failed to read line");
        
                let scalar: i32 = match scalar_input.trim().parse() {
                    Ok(scalar) => scalar,
                    Err(_) => {
                        eprintln!("Invalid input for scalar. Please provide a valid number.");
                        return;
                    }
                };
        
                if num_sets == 0 || num_sets > matrix_vec.len() {
                    eprintln!("Invalid number of matrices.");
                    return;
                }
        
                let selected_matrices = &matrix_vec[0..num_sets];
        
                for (index, matrix) in selected_matrices.iter().enumerate() {
                    println!("Matrix {}:", index + 1);
                    print_matrix(matrix);
                }
        
                let mut result_addition = Matrix::new(matrices[0].data.len(), matrices[0].data[0].len());
        
                let mut result_subtraction = Matrix::new(matrices[0].data.len(), matrices[0].data[0].len());
        
                let mut result_multiplication = Matrix::new(matrices[0].data.len(), matrices[0].data[0].len());
        
                for set in 0..num_sets {
                    let matrix1 = &matrix_vec[set];
                    for (index, other_matrix) in matrix_vec.iter().enumerate().skip(set + 1).take(num_sets - set - 1) {
                        if let Ok(result) = matrix1.add(other_matrix) {
                            result_addition = result_addition.add(&result).unwrap(); // Assuming all matrices have the same dimensions
                        }
        
                        if let Ok(result) = matrix1.subtract(other_matrix) {
                            result_subtraction = result_subtraction.add(&result).unwrap(); // Assuming all matrices have the same dimensions
                        }
        
                        if let Ok(result) = matrix1.multiply(other_matrix) {
                            result_multiplication = result_multiplication.add(&result).unwrap(); // Assuming the matrices can be multiplied
                        }
        
                        let scalar_result = matrix1.scalar_multiply(scalar);
                        println!("Scalar multiplication of Matrix {}: {:?}", index + 1 , scalar_result);
            
                    }
                }
        
                // Print out the final result matrices
                println!("Result of Addition:");
                print_matrix(&result_addition);
        
                println!("Result of Subtraction:");
                print_matrix(&result_subtraction);
        
                println!("Result of Multiplication:");
                print_matrix(&result_multiplication);
        
            } else {
                eprintln!("Error reading matrices from CSV file.");
            }

        }

        "3" => {
            let mut num_vectors = String::new();

            println!("How many vectors do you want to operate on?");
            io::stdin().read_line(&mut num_vectors).expect("Failed to read line");

            let num_vectors: usize = match num_vectors.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input. Please provide a positive integer.");
                    return;
                }
            };

            let mut scalar_input = String::new();
            println!("Enter the scalar value: ");
            io::stdin().read_line(&mut scalar_input).expect("Failed to read line");

            let scalar: f64 = match scalar_input.trim().parse() {
                Ok(scalar) => scalar,
                Err(_) => {
                    eprintln!("Invalid input for scalar. Please provide a valid number.");
                    return;
                }
            };

            if num_vectors < 2 {
                eprintln!("Number of vectors must be at least 2.");
                return;
            }

            if let Ok(vectors) = read_csv_data("vectors.csv") {
                if vectors.len() < num_vectors {
                    eprintln!("Not enough vectors in the file to perform operations.");
                    return;
                }

                for (index, vector) in vectors.iter().take(num_vectors).enumerate() {
                    println!("Vector {}: {:?}", index + 1, vector);
                }

                let mut result_addition = vectors[0].clone();
                let mut result_subtraction = vectors[0].clone();
                let mut result_multiplication = vectors[0].clone();
                let mut result_dot_product = 0.0;
                let mut result_cross_product = vectors[0].clone();

                for other_vector in &vectors[1..num_vectors] {
                    result_addition = result_addition.add(other_vector);

                    result_subtraction = result_subtraction.subtract(other_vector);

                    result_multiplication = result_multiplication.add(other_vector);

                    result_dot_product += result_multiplication.dot_product(other_vector);

                    result_cross_product = result_cross_product.cross_product(other_vector);
                }

                println!("Result of Addition: {:?}", result_addition);
                println!("Result of Subtraction: {:?}", result_subtraction);
                println!("Result of Multiplication: {:?}", result_multiplication);
                println!("Dot Product: {:?}", result_dot_product);
                println!("Cross Product: {:?}", result_cross_product);

                for (index, vector) in vectors.iter().take(num_vectors).enumerate() {
                    let magnitude = vector.magnitude();
                    let scalar_multiply = vector.scalar_multiply(scalar);
                    println!("Magnitude of Vector {}: {}", index + 1, magnitude);
                    println!("Scalar multiplication of Vector {}: {:?}", index + 1 , scalar_multiply);
                }

            } else {
                eprintln!("Error reading vectors from CSV file.");
            }
        }

        "4" => {
            let mut bool_inputs = String::new();
            println!("How many boolean inputs do you want to operate on?");
            io::stdin().read_line(&mut bool_inputs).expect("Failed to read line");

            let bool_inputs: usize = match bool_inputs.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input. Please provide a positive integer.");
                    return;
                }
            };

            if bool_inputs == 0 {
                eprintln!("Number of inputs must be greater than 0.");
                return;
            }

            if let Ok(inputs) = read_inputs_from_csv("booleans.csv") {
                if inputs.len() < bool_inputs {
                    eprintln!("Not enough boolean inputs in the file to perform operations.");
                    return;
                }

                let inputs_to_use = &inputs[0..bool_inputs];
                println!("Inputs: {:?}", inputs_to_use);

                if let Ok(gate) = LogicGate::new(inputs_to_use.to_vec()) {
                    println!("AND: {}", gate.and());
                    println!("OR: {}", gate.or());
                    println!("NOT: {:?}", gate.not());
                    println!("NAND: {}", gate.nand());
                    println!("NOR: {}", gate.nor());
                    println!("XOR: {}", gate.xor());
                    println!("XNOR: {}", gate.xnor());
                } else {
                    eprintln!("Error creating LogicGate. Please check your inputs.");
                }
            } else {
                eprintln!("Error reading boolean inputs from CSV file.");
            }
                }

        "5" => {
            let mut num_complex_numbers = String::new();
            println!("How many complex numbers do you want to operate on?");
            io::stdin().read_line(&mut num_complex_numbers).expect("Failed to read line");

            let num_complex_numbers: usize = match num_complex_numbers.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid input. Please provide a positive integer.");
                    return;
                }
            };

            if num_complex_numbers == 0 {
                eprintln!("Number of complex numbers must be greater than 0.");
                return;
            }

            if let Ok(complex_numbers) = read_complex_numbers_from_csv("complex.csv") {
                if complex_numbers.len() < num_complex_numbers {
                    eprintln!("Not enough complex numbers in the file to perform operations.");
                    return;
                }

                let numbers_to_use = &complex_numbers[0..num_complex_numbers];
                for (index, number) in numbers_to_use.iter().enumerate() {
                    println!("Complex Number {}: {} + {}i", index + 1, number.real, number.imaginary);
                }

                let mut addition_result = numbers_to_use[0].clone();
                let mut subtraction_result = numbers_to_use[0].clone();
                let mut multiplication_result = numbers_to_use[0].clone();
                let mut division_result = numbers_to_use[0].clone();

                for num_index in 1..num_complex_numbers {
                    let current_number = &numbers_to_use[num_index];

                    // Addition
                    addition_result = addition_result.add(current_number);

                    // Subtraction
                    subtraction_result = subtraction_result.subtract(current_number);

                    // Multiplication
                    multiplication_result = multiplication_result.multiply(current_number);

                    // Division
                    match division_result.divide(current_number) {
                        Ok(result) => division_result = result,
                        Err(e) => {
                            eprintln!("Error during division: {}", e);
                            return;
                        }
                    }
                }

                println!("Addition Result: {} + {}i", addition_result.real, addition_result.imaginary);
                println!("Subtraction Result: {} + {}i", subtraction_result.real, subtraction_result.imaginary);
                println!("Multiplication Result: {} + {}i", multiplication_result.real, multiplication_result.imaginary);
                println!("Division Result: {} + {}i", division_result.real, division_result.imaginary);

            } else {
                eprintln!("Error reading complex numbers from CSV file.");
            }
                }

        _ => {
            eprintln!("Invalid choice. Please choose a valid choice.");
        }
    }
}