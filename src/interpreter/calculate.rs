use crate::opcode::OperationType;
use crate::opcode::OperationType::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn calculate<'a>(
    key: &'a String,
    o_type: &OperationType,
    value: &i64,
    target: &mut HashMap<&'a String, ValueType>,
) {
    let old_value = match target.get(key) {
        Some(s) => match s {
            Int(i) => i,
            Line(_s) => panic!("wrong type for calculate"),
        },
        None => panic!("not value for calculate"),
    };

    let new_value = match o_type {
        Increment => old_value + value,
        Decrement => old_value - value,
    };

    target.insert(key, Int(new_value));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_increment() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        calculate(&binding, &OperationType::Increment, &5, &mut map);
        assert_eq!(map.get(&String::from("test_key")), Some(&Int(15)));
    }

    #[test]
    fn test_calculate_decrement() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        calculate(&binding, &OperationType::Decrement, &3, &mut map);
        assert_eq!(map.get(&String::from("test_key")), Some(&Int(7)));
    }
}
