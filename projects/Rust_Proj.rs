// Define a struct to store the input elements
struct Calculator {
    first_number: f64,
    second_number: f64,
    operator: char,
    result: f64,
}

// Define a method to perform calculation
impl Calculator {
    fn calculate(&mut self) {
        // Perform calculation based on the operator
        let res;
        match self.operator {
            '+' => res = self.first_number + self.second_number,
            '-' => res = self.first_number - self.second_number,
            '*' => res = self.first_number * self.second_number,
            '/' => res = self.first_number / self.second_number,
            _ => panic!("Invalid operator"),
        }

        // Display the result
        self.result = res;
        println!("The result is {}", self.result);
    }
}

fn main(){
    // Create an instance of Calculator with some values
    let mut calc = Calculator {
        first_number: 10.0,
        second_number: 5.0,
        operator: '+',
        result: 0.0,
    };
    
    // Call the calculate method
    calc.calculate();
}