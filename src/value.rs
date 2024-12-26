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
            Literal::None => panic!("unreachable"),
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
}
