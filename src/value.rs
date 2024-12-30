use crate::token::Literal;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    True,
    False,
    Num(f64),
    Str(String),
}
impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Nil => "niks".to_string(),
            Value::True => "wellus".to_string(),
            Value::False => "nietus".to_string(),
            Value::Num(num) => num.to_string(),
            Value::Str(str) => str.to_string(),
        }
    }

    pub fn from_lit(lit: &Literal) -> Self {
        match lit {
            Literal::None => panic!("Unreachable."),
            Literal::Str(str) => Value::Str(str.clone()),
            Literal::Num(num) => Value::Num(*num),
            Literal::True => Value::True,
            Literal::False => Value::False,
            Literal::Nil => Value::Nil,
        }
    }

    pub fn from_bool(is_true: bool) -> Value {
        if is_true {
            return Value::True;
        }
        Value::False
    }

    pub fn is_true(&self) -> bool {
        match self {
            Value::False | Value::Nil => false,
            _ => true,
        }
    }

    pub fn is_equal(value1: &Value, value2: &Value) -> bool {
        match (value1, value2) {
            (Value::Nil, Value::Nil) => true,
            (Value::True, Value::True) => true,
            (Value::False, Value::False) => true,
            (Value::Num(num1), Value::Num(num2)) => num1 == num2,
            (Value::Str(str1), Value::Str(str2)) => str1 == str2,
            _ => false,
        }
    }
}
