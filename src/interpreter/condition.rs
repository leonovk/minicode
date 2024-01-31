use crate::opcode::ComparisonOperators;
use crate::opcode::ComparisonOperators::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;
use super::opcode_result_type::*;

pub fn condition(
    key: &String,
    operator: &ComparisonOperators,
    target_value: &ValueType,
    storage: &HashMap<&String, ValueType>,
) -> Result<OpCodeResultType, String> {
    let key_string = Line(key.to_string());
    let first_value = storage.get(key).unwrap_or(&key_string);

    let result = match target_value {
        Int(int) => condition_result(first_value, &Int(*int), operator),
        Line(str) => match storage.get(str) {
            Some(some) => condition_result(first_value, some, operator),
            None => condition_result(first_value, &Line(str.to_string()), operator),
        },
        Arr(_arr) => return Err("you can't compare arrays".to_string()),
    };

    match result {
        Ok(b) => Ok(OpCodeResultType::Bool(b)),
        Err(e) => Err(e)
    }
}

fn condition_result(first: &ValueType, second: &ValueType, operator: &ComparisonOperators) -> Result<bool, String> {
    if type_are_different(first, second) {
        return Err("You cannot compare values of different types, or arrays!".to_string());
    }

    let result = match operator {
        Equals => first == second,
        NotEquals => first != second,
        More => first > second,
        Less => first < second,
    };

    Ok(result)
}

fn type_are_different(v1: &ValueType, v2: &ValueType) -> bool {
    if let ValueType::Arr(_) = v1 {
        return true;
    } else if let ValueType::Arr(_) = v2 {
        return true;
    }

    match (v1, v2) {
        (ValueType::Int(_), ValueType::Line(_)) => true,
        (ValueType::Line(_), ValueType::Int(_)) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn test_condition_int_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10.0));
        let result = condition(&String::from("test_key"), &Equals, &Int(10.0), &map).unwrap();

        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_one")
        };
    }

    #[test]
    fn test_condition_int_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10.0));
        let result = condition(&String::from("test_key"), &Equals, &Int(5.0), &map).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_int_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10.0));
        let result = condition(&String::from("test_key"), &NotEquals, &Int(10.0), &map).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_int_four() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10.0));
        let result = condition(&String::from("test_key"), &NotEquals, &Int(5.0), &map).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    #[should_panic]
    fn test_condition_int_and_line() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10.0));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("5".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Line("10".to_string()));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("10".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Line("10".to_string()));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("5".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Line("10".to_string()));
        let result = condition(
            &String::from("test_key"),
            &NotEquals,
            &Line("10".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_four() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Line("10".to_string()));
        let result = condition(
            &String::from("test_key"),
            &NotEquals,
            &Line("5".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_key_int_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10.0));
        map.insert(&binding_2, Int(10.0));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key_2".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_key_int_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10.0));
        map.insert(&binding_2, Int(10.0));
        let result = condition(
            &String::from("test_key"),
            &NotEquals,
            &Line("test_key_2".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_key_int_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10.0));
        map.insert(&binding_2, Int(5.0));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key_2".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_key_int_four() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10.0));
        map.insert(&binding_2, Int(5.0));
        let result = condition(
            &String::from("test_key"),
            &NotEquals,
            &Line("test_key_2".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_key_str_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Line("10".to_string()));
        map.insert(&binding_2, Line("10".to_string()));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key_2".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_key_str_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Line("10".to_string()));
        map.insert(&binding_2, Line("10".to_string()));
        let result = condition(
            &String::from("test_key"),
            &NotEquals,
            &Line("test_key_2".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_key_str_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Line("10".to_string()));
        map.insert(&binding_2, Line("1dsa".to_string()));
        let result = condition(
            &String::from("test_key"),
            &NotEquals,
            &Line("test_key_2".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_line_key_str_four() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Line("10".to_string()));
        map.insert(&binding_2, Line("1dsa".to_string()));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key_2".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_where_key_not_in_hash_one() {
        let map: HashMap<&String, ValueType> = HashMap::new();
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key_2".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, false),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_where_key_not_in_hash_two() {
        let map: HashMap<&String, ValueType> = HashMap::new();
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_where_key_not_in_hash_three() {
        let map: HashMap<&String, ValueType> = HashMap::new();
        let result = condition(
            &String::from("test_keydsa"),
            &More,
            &Line("test_key".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }

    #[test]
    fn test_condition_where_key_not_in_hash_four() {
        let map: HashMap<&String, ValueType> = HashMap::new();
        let result = condition(
            &String::from("test"),
            &Less,
            &Line("test_key".to_string()),
            &map,
        ).unwrap();
        match result {
            OpCodeResultType::Bool(b) => assert_eq!(b, true),
            _ => panic!("test_condition_int_two")
        };
    }
}
