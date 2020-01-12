pub struct Entities{
    pub number1:f64,
    pub number2:f64,
    pub operation:String,
}


pub fn add(calculate:&Entities)->f64{
    calculate.number2 + calculate.number2
}

pub fn sub(calculate:&Entities)->f64{
    calculate.number1 - calculate.number2
}
pub fn mult(calculate:&Entities)->f64{
    calculate.number1 * calculate.number2
}
pub fn div(calculate:&Entities)->f64{
    calculate.number1 / calculate.number2
}

fn main() {
    let event1 = Entities{number1:34.0,number2:32.0,operation:String::from("Addition")};

    if event1.operation == "Addition"{
        let result = add(&event1);
        println!("The result is {}",result);
    }


    else if event1.operation == "Subtraction"{
        let result = sub(&event1);
        println!("The result is {}",result);
    }

    else if event1.operation == "Multiplication"{
        let result = mult(&event1);
        println!("The result is {}",result);
    }
    
    else if event1.operation == "Division"{
        let result = div(&event1);
        println!("The result is {}",result);
    }
    else{
        println!("Invalid input");
    }
}
