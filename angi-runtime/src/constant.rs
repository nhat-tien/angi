use crate::{error::VmError, value::Value};

#[derive(Debug)]
pub enum ConstantValue {
    Int(i64),
    String(String),
}

impl std::fmt::Display for ConstantValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstantValue::Int(int) => int.fmt(f),
            ConstantValue::String(str) => str.fmt(f)
        }
    }
}

impl ConstantValue {
   pub fn to_value(&self) -> Value {
        match self {
            ConstantValue::Int(int) => Value::Int(*int),
            ConstantValue::String(str) => Value::String(str.clone())
        }
    }

    pub fn to_string(&self) -> Result<String, VmError>{
        match self {
            ConstantValue::String(str) => Ok(str.clone()),
            _ => Err(VmError::UnexpectedError { message: "constant not string".into() })
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::VmError;
    use crate::value::Value;

    #[test]
    fn test_display_for_constantvalue() {
        let c1 = ConstantValue::Int(42);
        assert_eq!(format!("{}", c1), "42");

        let c2 = ConstantValue::String("abc".into());
        assert_eq!(format!("{}", c2), "abc");
    }

    #[test]
    fn test_to_value() {
        let c1 = ConstantValue::Int(7);
        let v1 = c1.to_value();
        assert!(matches!(v1, Value::Int(7)));

        let c2 = ConstantValue::String("xyz".into());
        let v2 = c2.to_value();
        assert!(matches!(v2, Value::String(s) if s == "xyz"));
    }

    #[test]
    fn test_to_string_success() {
        let c = ConstantValue::String("hello".into());
        let result = c.to_string().unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_to_string_fail() {
        let c = ConstantValue::Int(10);
        let result = c.to_string();
        match result {
            Err(VmError::UnexpectedError { message }) => {
                assert!(message.contains("constant not string"));
            }
            _ => panic!("Expected VmError::UnexpectedError"),
        }
    }
}
