// sometimes we need to express why an operation fails, for this we have Result
// 2 variants:
// Ok(value) indicates operation success, and wraps the value in a tuple struct
// Err(why) indicates failure, and wraps the reason in a tuple struct
mod checked {
    // mathematical errors we want to catch
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            // this op will fail, so let's return reason for failure as a Result
            Err(MathError::DivisionByZero)
        } else {
            // this op is valid
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }
}

// op(x, y) === sqrt(ln(x / y))
fn op(x: f64, y: f64) -> f64 {
    // this is a 3 level match pyramid
    match checked::div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match checked::ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match checked::sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

fn main() {
    // will this fail??
    println!("{}", op(1.0, 10.0));
}