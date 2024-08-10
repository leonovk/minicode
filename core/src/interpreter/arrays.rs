use crate::opcode::OpCodeResultType;
use crate::opcode::OpCodeResultType::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn push<'a>(
    key: &'a String,
    value: &'a String,
    target: &mut HashMap<&'a String, ValueType>,
) -> Result<OpCodeResultType, String> {
    let second_value = match target.get(value) {
        Some(some) => some.clone(),
        None => match value.parse::<f64>() {
            Ok(i) => Int(i),
            Err(_) => Line(value.to_string()),
        },
    };

    let first_value: &mut Vec<ValueType> = match target.get_mut(key) {
        Some(some) => match some {
            Arr(a) => a,
            _ => return Err("not is array".to_string()),
        },
        None => return Err("not is array".to_string()),
    };

    first_value.push(second_value);

    Ok(Empty)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn test_push_to_array() {
        let mut target: HashMap<&String, ValueType> = HashMap::new();
        let key = String::from("key");
        let value = String::from("123");
        let arr = Vec::<ValueType>::new();

        target.insert(&key, Arr(arr));

        let result = push(&key, &value, &mut target);

        match result {
            Ok(r) => match r {
                OpCodeResultType::Empty => {}
                _ => panic!("not expected result! 1"),
            },
            Err(_e) => panic!("not expected result! 2"),
        }

        let arr = match target.get(&key) {
            Some(ValueType::Arr(arr)) => arr,
            _ => panic!("Expected an array"),
        };

        assert_eq!(arr.len(), 1);

        match arr[0] {
            Int(i) => assert_eq!(i, 123.0),
            _ => panic!("Expected an integer"),
        }
    }
}
