#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut numbers = vec![];
    for value in inputs {
        match value {
            CalculatorInput::Value(i) => numbers.push(*i),
            CalculatorInput::Add => {
                if numbers.len() < 2 {
                    return None;
                }
                let value_1 = numbers.pop().unwrap();
                let value_2 = numbers.pop().unwrap();
                let result = value_2 + value_1;
                numbers.push(result);
            }
            CalculatorInput::Subtract => {
                if numbers.len() < 2 {
                    return None;
                }
                let value_1 = numbers.pop().unwrap();
                let value_2 = numbers.pop().unwrap();
                let result = value_2 - value_1;
                numbers.push(result);
            }
            CalculatorInput::Multiply => {
                if numbers.len() < 2 {
                    return None;
                }
                let value_1 = numbers.pop().unwrap();
                let value_2 = numbers.pop().unwrap();
                let result = value_2 * value_1;
                numbers.push(result);
            }
            CalculatorInput::Divide => {
                if numbers.len() < 2 {
                    return None;
                }
                let value_1 = numbers.pop().unwrap();
                let value_2 = numbers.pop().unwrap();
                let result = value_2 / value_1;
                numbers.push(result);
            }
        }
    }
    if numbers.len() == 1 {
        numbers.pop()
    } else {
        None
    }
}
