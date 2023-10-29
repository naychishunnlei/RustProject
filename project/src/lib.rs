pub mod math_set {
    use std::io::BufRead;
    use std::error::Error;
    use std::fs::File;
    use std::path::Path;
    use std::io::BufReader;

    #[derive(Debug, Clone, PartialEq)]
    pub struct MathSet {
        pub elements: Vec<i32>,
    }

    impl MathSet {
        pub fn new(elements: Vec<i32>) -> MathSet {
            MathSet { elements }
        }

        pub fn union(&self, other: &MathSet) -> MathSet {
            let mut union = self.elements.clone();

            for element in &other.elements {
                if !union.contains(element) {
                    union.push(*element);
                }
            }

            MathSet { elements: union }
        }

        pub fn intersection(&self, other: &MathSet) -> MathSet {
            let mut intersect = Vec::new();

            for element in &self.elements {
                if other.elements.contains(element) {
                    intersect.push(*element);
                }
            }

            MathSet { elements: intersect }
        }

        pub fn difference(&self, other: &MathSet) -> MathSet {
            let mut difference = Vec::new();

            for element in &self.elements {
                if !other.elements.contains(element) {
                    difference.push(*element);
                }
            }

            MathSet { elements: difference }
        }
    }    

    pub fn read_sets_from_csv(filename: &str) -> Result<Vec<MathSet>, Box<dyn Error>> {
        let path = Path::new(filename);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut sets = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let elements: Vec<i32> = line
                .split(',')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect();
            let math_set = MathSet::new(elements);
            sets.push(math_set);
        }

        Ok(sets)
    }

    #[test]
    fn test_set_operations() {
        // Define some example sets
        let set1 = MathSet { elements: vec![1, 2, 3] };
        let set2 = MathSet { elements: vec![3, 4, 5] };

        // Test union
        let union_result = set1.union(&set2);
        let expected_union = MathSet { elements: vec![1, 2, 3, 4, 5] };
        assert_eq!(union_result, expected_union);

        // Test intersection
        let intersection_result = set1.intersection(&set2);
        let expected_intersection = MathSet { elements: vec![3] };
        assert_eq!(intersection_result, expected_intersection);

        // Test difference
        let difference_result = set1.difference(&set2);
        let expected_difference = MathSet { elements: vec![1, 2] };
        assert_eq!(difference_result, expected_difference);
    }
}

pub mod matrix  {
    use std::io::BufRead;
    use std::error::Error;
    use std::fs::File;
    use std::path::Path;
    use std::io::BufReader;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Matrix {
        pub data: Vec<Vec<i32>>,
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Matrix {
            Matrix {
                data: vec![vec![0; cols]; rows],
            }
        }

        pub fn add(&self, other: &Matrix) -> Result<Matrix, &'static str> {
            if self.data.len() != other.data.len() || self.data[0].len() != other.data[0].len() {
                return Err("Matrices must have the same dimensions for addition.");
            }

            let mut result = Matrix::new(self.data.len(), self.data[0].len());

            for i in 0..self.data.len() {
                for j in 0..self.data[0].len() {
                    result.data[i][j] = self.data[i][j] + other.data[i][j];
                }
            }

            Ok(result)
        }

        pub fn subtract(&self, other: &Matrix) -> Result<Matrix, &'static str> {
            if self.data.len() != other.data.len() || self.data[0].len() != other.data[0].len() {
                return Err("Matrices must have the same dimensions for subtraction.");
            }

            let mut result = Matrix::new(self.data.len(), self.data[0].len());

            for i in 0..self.data.len() {
                for j in 0..self.data[0].len() {
                    result.data[i][j] = self.data[i][j] - other.data[i][j];
                }
            }

            Ok(result)
        }

        pub fn multiply(&self, other: &Matrix) -> Result<Matrix, &'static str> {
            if self.data[0].len() != other.data.len() {
                return Err("Invalid dimensions for matrix multiplication.");
            }

            let mut result = Matrix::new(self.data.len(), other.data[0].len());

            for i in 0..self.data.len() {
                for j in 0..other.data[0].len() {
                    let mut multiply = 0;
                    for k in 0..other.data[0].len() {
                        multiply += self.data[i][k] * other.data[k][j];
                    }
                    result.data[i][j] = multiply;
                }
            }
            Ok(result)
        }

        pub fn scalar_multiply(&self, scalar: i32) -> Matrix {
            let mut result = Matrix::new(self.data.len(), self.data[0].len());

            for i in 0..self.data.len() {
                for j in 0..self.data[0].len() {
                    result.data[i][j] = self.data[i][j] * scalar;
                }
            }

            result
        }
    }

    pub fn read_matrices_from_csv(filename: &str) -> Result<Vec<Matrix>, Box<dyn Error>> {
        let path = Path::new(filename);
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut matrices = Vec::new();
        let mut current_matrix = Matrix::new(0, 0);

        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                matrices.push(current_matrix);
                current_matrix = Matrix::new(0, 0);
            } else {
                let row: Vec<i32> = line
                    .split(',')
                    .map(|s| s.trim().parse::<i32>().unwrap())
                    .collect();
                current_matrix.data.push(row);
            }
        }

        matrices.push(current_matrix); // Add the last matrix
        Ok(matrices)
    }

    pub fn print_matrix(matrix: &Matrix) {
        for row in &matrix.data {
            println!("{:?}", row);
        }
    }

    #[test]
    fn test_matrix_operations() {
        let matrix1 = Matrix { data: vec![vec![1, 2], vec![3, 4]] };
        let matrix2 = Matrix { data: vec![vec![5, 6], vec![7, 8]] };
        let scalar = 2;

        // Test addition
        let addition_result = matrix1.add(&matrix2);
        let expected_addition = Matrix { data: vec![vec![6, 8], vec![10, 12]] };
        assert_eq!(addition_result, Ok(expected_addition));

        // Test subtraction
        let subtraction_result = matrix1.subtract(&matrix2);
        let expected_subtraction = Matrix { data: vec![vec![-4, -4], vec![-4, -4]] };
        assert_eq!(subtraction_result, Ok(expected_subtraction));

        // Test multiplication
        let multiplication_result = matrix1.multiply(&matrix2);
        let expected_multiplication = Matrix { data: vec![vec![19, 22], vec![43, 50]] };
        assert_eq!(multiplication_result, Ok(expected_multiplication));

        // Test scalar multiplication
        let scalar_result = matrix1.scalar_multiply(scalar);
        let expected_scalar_result = Matrix { data: vec![vec![2, 4], vec![6, 8]] };
        assert_eq!(scalar_result, expected_scalar_result);
    }
}

pub mod vector {
    use std::io;
    use std::error::Error;
    use std::fs::File;
    use std::io::BufRead;
    use std::path::Path;
    use std::str::FromStr;

    #[derive(Debug, PartialEq, Clone)]
    pub struct Vector {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    impl Vector {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Vector { x, y, z }
        }

        pub fn add(&self, other: &Vector) -> Vector {
            Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
        }

        pub fn subtract(&self, other: &Vector) -> Vector {
            Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
        }

        pub fn scalar_multiply(&self, scalar: f64) -> Vector {
            Vector::new(self.x * scalar, self.y * scalar, self.z * scalar)
        }

        pub fn dot_product(&self, other: &Vector) -> f64 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }

        pub fn cross_product(&self, other: &Vector) -> Vector {
            let x = self.y * other.z - self.z * other.y;
            let y = self.z * other.x - self.x * other.z;
            let z = self.x * other.y - self.y * other.x;
            Vector::new(x, y, z)
        }

        pub fn magnitude(&self) -> f64 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }
    }

    pub fn read_csv_data(filename: &str) -> Result<Vec<Vector>, Box<dyn Error>> {
        let path = Path::new(filename);
        let file = File::open(&path)?;

        let mut vectors = Vec::new();

        for line in io::BufReader::new(file).lines() {
            let line = line?;
            let components: Vec<f64> = line
                .split(',')
                .map(|s| f64::from_str(s.trim()).unwrap())
                .collect();

            if components.len() == 3 {
                vectors.push(Vector::new(components[0], components[1], components[2]));
            }
        }

        Ok(vectors)
    }

    #[test]
    fn test_vector_operations() {
        let vector1 = Vector::new(2.0, 3.0, 4.0);
        let vector2 = Vector::new(5.0, 6.0, 7.0);

        let add_result = vector1.add(&vector2);
        assert_eq!(add_result, Vector::new(7.0, 9.0, 11.0));

        let subtract_result = vector1.subtract(&vector2);
        assert_eq!(subtract_result, Vector::new(-3.0, -3.0, -3.0));

        let scalar = 0.5;
        let scalar_result = vector1.scalar_multiply(scalar);
        assert_eq!(scalar_result, Vector::new(1.0, 1.5, 2.0));

        let dot_product_result = vector1.dot_product(&vector2);
        assert_eq!(dot_product_result, 56.0);

        let cross_product_result = vector1.cross_product(&vector2);
        assert_eq!(cross_product_result, Vector::new(-3.0, 6.0, -3.0));

        let magnitude_result1 = vector1.magnitude();
        assert_eq!(magnitude_result1, 5.385164807134504);

        let magnitude_result2 = vector2.magnitude();
        assert_eq!(magnitude_result2, 10.488088481701515);
    }
}

pub mod logic {
    use csv::ReaderBuilder;
    use std::error::Error;

    pub struct LogicGate {
        pub inputs: Vec<bool>,
    }

    impl LogicGate {
        pub fn new(inputs: Vec<bool>) -> Result<LogicGate, &'static str> {
            let bool_inputs = inputs.len();

            if bool_inputs < 2 {
                return Err("LogicGate requires at least two inputs.");
            }

            Ok(LogicGate { inputs })
        }

        // Function to perform AND operation
        pub fn and(&self) -> bool {
            self.inputs.iter().all(|&x| x)
        }

        // Function to perform OR operation
        pub fn or(&self) -> bool {
            self.inputs.iter().any(|&x| x)
        }

        // Function to perform NOT operation on all inputs
        pub fn not(&self) -> Vec<bool> {
            self.inputs.iter().map(|&x| !x).collect()
        }

        // Function to perform NAND operation
        pub fn nand(&self) -> bool {
            !self.and()
        }

        // Function to perform NOR operation
        pub fn nor(&self) -> bool {
            !self.or()
        }

        // Function to perform XOR operation
        pub fn xor(&self) -> bool {
            self.inputs.iter().fold(false, |acc, &x| acc ^ x)
        }

        // Function to perform XNOR operation
        pub fn xnor(&self) -> bool {
            !self.xor()
        }
    }

    pub fn read_inputs_from_csv(filename: &str) -> Result<Vec<bool>, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new().has_headers(true).from_path(filename)?;
    
        let mut inputs = Vec::new();
    
        for record in reader.records() {
            let record = record?;
            let input: bool = record[0].parse()?;
            inputs.push(input);
        }
    
        Ok(inputs)
    }

    #[test]
    fn test_logic_gate_operations() {
        // Test case with valid inputs
        let valid_inputs = vec![true, false, true, true, false];
        let gate = LogicGate::new(valid_inputs.clone());
        assert!(gate.is_ok());

        let gate = gate.unwrap();
        assert_eq!(gate.and(), false);
        assert_eq!(gate.or(), true);
        assert_eq!(gate.not(), vec![false, true, false, false, true]);
        assert_eq!(gate.nand(), true);
        assert_eq!(gate.nor(), false);
        assert_eq!(gate.xor(), true);
        assert_eq!(gate.xnor(), false);

        // Test case with empty inputs
        let empty_inputs = vec![];
        let gate = LogicGate::new(empty_inputs);
        assert!(gate.is_err(), "LogicGate creation should fail with empty inputs");
    }
}

pub mod complex {
    use csv::ReaderBuilder;
    use std::error::Error;

    #[warn(non_snake_case)]
    #[derive(Debug, Clone)]
    pub struct Complex {
        pub real: f32,
        pub imaginary: f32,
    }

    impl Complex {
        pub fn new(real: f32, imaginary: f32) -> Complex {
            Complex { real, imaginary }
        }

        pub fn add(&self, other: &Complex) -> Complex {
            Complex::new(self.real + other.real, self.imaginary + other.imaginary)
        }

        pub fn subtract(&self, other: &Complex) -> Complex {
            Complex::new(self.real - other.real, self.imaginary - other.imaginary)
        }

        pub fn multiply(&self, other: &Complex) -> Complex {
            let real_part = self.real * other.real - self.imaginary * other.imaginary;
            let imaginary_part = self.real * other.imaginary + self.imaginary * other.real;
            Complex::new(real_part, imaginary_part)
        }

        pub fn divide(&self, other: &Complex) -> Result<Complex, &'static str> {
            if other.real == 0.0 && other.imaginary == 0.0 {
                return Err("Division by zero is not allowed.");
            }

            let denominator = other.real * other.real + other.imaginary * other.imaginary;

            let real_part = (self.real * other.real + self.imaginary * other.imaginary) / denominator;
            let imaginary_part = (self.imaginary * other.real - self.real * other.imaginary) / denominator;

            Ok(Complex::new(real_part, imaginary_part))
        }
    }
    pub fn read_complex_numbers_from_csv(filename: &str) -> Result<Vec<Complex>, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new().has_headers(true).from_path(filename)?;
    
        let mut complex_numbers = Vec::new();
    
        for record in reader.records() {
            let record = record?;
            let real: f32 = record[0].parse()?;
            let imaginary: f32 = record[1].parse()?;
            complex_numbers.push(Complex::new(real, imaginary));
        }
    
        Ok(complex_numbers)
    }

    #[test]
    fn test_complex_operations() {
        // Define some complex numbers for testing
        let complex1 = Complex::new(1.0, 2.0);
        let complex2 = Complex::new(3.0, 4.0);

        // Test addition
        let addition_result = complex1.add(&complex2);
        assert_eq!(addition_result.real, 4.0);
        assert_eq!(addition_result.imaginary, 6.0);

        // Test subtraction
        let subtraction_result = complex2.subtract(&complex1);
        assert_eq!(subtraction_result.real, 2.0);
        assert_eq!(subtraction_result.imaginary, 2.0);

        // Test multiplication
        let multiplication_result = complex1.multiply(&complex2);
        assert_eq!(multiplication_result.real, -5.0);
        assert_eq!(multiplication_result.imaginary, 10.0);

        // Test division
        let division_result = complex1.divide(&complex2);
        assert_eq!(division_result.clone().unwrap().real, 0.44, "{epsilon}", epsilon = 0.01);
        assert_eq!(division_result.clone().unwrap().imaginary, 0.08, "{epsilon}", epsilon = 0.01);
    }
}
