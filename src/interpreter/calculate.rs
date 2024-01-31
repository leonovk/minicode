use super::opcode_result_type::*;
use crate::opcode::OperationType;
use crate::opcode::OperationType::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn calculate<'a>(
    key: &'a String,
    o_type: &OperationType,
    value: &ValueType,
    target: &mut HashMap<&'a String, ValueType>,
) -> Result<OpCodeResultType, String> {
    let old_value = match target.get(key) {
        Some(s) => match s {
            Int(int) => int,
            _ => return Err("wrong type for calculate".to_string()),
        },
        None => return Err("Value not found".to_string()),
    };

    let operational_meaning = match value {
        Int(int) => int,
        Line(str) => match target.get(str) {
            Some(some) => match some {
                Int(link_int) => link_int,
                _ => return Err("wrong type for calculate".to_string()),
            },
            None => return Err("wrong type for calculate".to_string()),
        },
        Arr(_arr) => return Err("wrong type for calculate".to_string()),
    };

    target.insert(
        key,
        calculate_new_value(old_value, operational_meaning, o_type),
    );

    Ok(OpCodeResultType::Empty)
}

fn calculate_new_value(old_value: &f64, oper_value: &f64, o_type: &OperationType) -> ValueType {
    let new_value = match o_type {
        Increment => old_value + oper_value,
        Decrement => old_value - oper_value,
        Multiplication => old_value * oper_value,
        Division => old_value / oper_value,
    };

    Int(new_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_increment_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10.0));
        let _ = calculate(&binding, &OperationType::Increment, &Int(5.0), &mut map);
        assert_eq!(map.get(&String::from("test_key")), Some(&Int(15.0)));
    }

    #[test]
    fn test_calculate_increment_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10.0));
        map.insert(&binding_2, Int(5.0));
        let _ = calculate(
            &binding,
            &OperationType::Increment,
            &Line("test_key_2".to_string()),
            &mut map,
        );
        assert_eq!(map.get(&String::from("test_key")), Some(&Int(15.0)));
    }

    #[test]
    fn test_calculate_decrement_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10.0));
        let _ = calculate(&binding, &OperationType::Decrement, &Int(3.0), &mut map);
        assert_eq!(map.get(&String::from("test_key")), Some(&Int(7.0)));
    }

    #[test]
    fn test_calculate_decrement_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10.0));
        map.insert(&binding_2, Int(5.0));
        let _ = calculate(
            &binding,
            &OperationType::Decrement,
            &Line("test_key_2".to_string()),
            &mut map,
        );
        assert_eq!(map.get(&String::from("test_key")), Some(&Int(5.0)));
    }

    #[test]
    fn test_calculate_multy_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10.0));
        let _ = calculate(
            &binding,
            &OperationType::Multiplication,
            &Int(3.0),
            &mut map,
        );
        assert_eq!(map.get(&String::from("test_key")), Some(&Int(30.0)));
    }

    #[test]
    fn test_calculate_multy_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10.0));
        map.insert(&binding_2, Int(5.0));
        let _ = calculate(
            &binding,
            &OperationType::Multiplication,
            &Line("test_key_2".to_string()),
            &mut map,
        );
        assert_eq!(map.get(&String::from("test_key")), Some(&Int(50.0)));
    }

    #[test]
    fn test_calculate_divi_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10.0));
        let _ = calculate(&binding, &OperationType::Division, &Int(3.0), &mut map);
        assert_eq!(
            map.get(&String::from("test_key")),
            Some(&Int(3.3333333333333335))
        );
    }

    #[test]
    fn test_calculate_divi_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10.0));
        map.insert(&binding_2, Int(5.0));
        let _ = calculate(
            &binding,
            &OperationType::Division,
            &Line("test_key_2".to_string()),
            &mut map,
        );
        assert_eq!(map.get(&String::from("test_key")), Some(&Int(2.0)));
    }
}
