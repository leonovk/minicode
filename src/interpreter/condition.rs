use crate::opcode::ComparisonOperators;
use crate::opcode::ComparisonOperators::*;
use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn condition(
    key: &String,
    operator: &ComparisonOperators,
    target_value: &ValueType,
    storage: &HashMap<&String, ValueType>,
) -> bool {
    let key_string = Line(key.to_string());
    let first_value = storage.get(key).unwrap_or(&key_string);

    match target_value {
        Int(int) => condition_result(first_value, &Int(*int), operator),
        Line(str) => match storage.get(str) {
            Some(some) => condition_result(first_value, some, operator),
            None => condition_result(first_value, &Line(str.to_string()), operator),
        },
    }
}

fn condition_result(first: &ValueType, second: &ValueType, operator: &ComparisonOperators) -> bool {
    if type_are_different(first, second) {
        panic!("You cannot compare values of different types!");
    }

    match operator {
        Equals => first == second,
        NotEquals => first != second,
        More => first > second,
        Less => first < second,
    }
}

fn type_are_different(v1: &ValueType, v2: &ValueType) -> bool {
    match (v1, v2) {
        (ValueType::Int(_), ValueType::Line(_)) => true,
        (ValueType::Line(_), ValueType::Int(_)) => true,
        _ => false,
    }
}

#[test]
fn test_condition_result() {
    let c = condition_result(&Int(32), &Int(1), &Equals);
    assert_eq!(c, false);
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
        map.insert(&binding, Int(10));
        let result = condition(&String::from("test_key"), &Equals, &Int(10), &map);
        assert_eq!(result, true);
    }

    #[test]
    fn test_condition_int_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        let result = condition(&String::from("test_key"), &Equals, &Int(5), &map);
        assert_eq!(result, false);
    }

    #[test]
    fn test_condition_int_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        let result = condition(&String::from("test_key"), &NotEquals, &Int(10), &map);
        assert_eq!(result, false);
    }

    #[test]
    fn test_condition_int_four() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        let result = condition(&String::from("test_key"), &NotEquals, &Int(5), &map);
        assert_eq!(result, true);
    }

    #[test]
    #[should_panic]
    fn test_condition_int_and_line() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("5".to_string()),
            &map,
        );
        assert_eq!(result, false);
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
        );
        assert_eq!(result, true);
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
        );
        assert_eq!(result, false);
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
        );
        assert_eq!(result, false);
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
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test_condition_line_key_int_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10));
        map.insert(&binding_2, Int(10));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key_2".to_string()),
            &map,
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test_condition_line_key_int_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10));
        map.insert(&binding_2, Int(10));
        let result = condition(
            &String::from("test_key"),
            &NotEquals,
            &Line("test_key_2".to_string()),
            &map,
        );
        assert_eq!(result, false);
    }

    #[test]
    fn test_condition_line_key_int_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10));
        map.insert(&binding_2, Int(5));
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key_2".to_string()),
            &map,
        );
        assert_eq!(result, false);
    }

    #[test]
    fn test_condition_line_key_int_four() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        let binding_2 = String::from("test_key_2");
        map.insert(&binding, Int(10));
        map.insert(&binding_2, Int(5));
        let result = condition(
            &String::from("test_key"),
            &NotEquals,
            &Line("test_key_2".to_string()),
            &map,
        );
        assert_eq!(result, true);
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
        );
        assert_eq!(result, true);
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
        );
        assert_eq!(result, false);
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
        );
        assert_eq!(result, true);
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
        );
        assert_eq!(result, false);
    }

    #[test]
    fn test_condition_where_key_not_in_hash_one() {
        let map: HashMap<&String, ValueType> = HashMap::new();
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key_2".to_string()),
            &map,
        );
        assert_eq!(result, false);
    }

    #[test]
    fn test_condition_where_key_not_in_hash_two() {
        let map: HashMap<&String, ValueType> = HashMap::new();
        let result = condition(
            &String::from("test_key"),
            &Equals,
            &Line("test_key".to_string()),
            &map,
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test_condition_where_key_not_in_hash_three() {
        let map: HashMap<&String, ValueType> = HashMap::new();
        let result = condition(
            &String::from("test_keydsa"),
            &More,
            &Line("test_key".to_string()),
            &map,
        );
        assert_eq!(result, true);
    }

    #[test]
    fn test_condition_where_key_not_in_hash_four() {
        let map: HashMap<&String, ValueType> = HashMap::new();
        let result = condition(
            &String::from("test"),
            &Less,
            &Line("test_key".to_string()),
            &map,
        );
        assert_eq!(result, true);
    }
}
