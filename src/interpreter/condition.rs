use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn condition(
    key: &String,
    true_or_false: &bool,
    target_value: &ValueType,
    storage: &HashMap<&String, ValueType>,
) -> bool {
    let key_string = Line(key.to_string());
    let first_value = storage.get(key).unwrap_or(&key_string);

    match target_value {
        Int(int) => condition_result(first_value, &Int(*int), true_or_false),
        Line(str) => match storage.get(str) {
            Some(some) => condition_result(first_value, some, true_or_false),
            None => condition_result(first_value, &Line(str.to_string()), true_or_false),
        },
    }
}

fn condition_result<T: std::cmp::PartialEq>(first: T, second: T, true_or_false: &bool) -> bool {
    if *true_or_false {
        return first == second;
    } else {
        return first != second;
    }
}

#[test]
fn test_condition_result() {
    let a = condition_result(1, 2, &true);
    assert_eq!(a, false);

    let b = condition_result(Int(1), Int(1), &true);
    assert_eq!(b, true);

    let c = condition_result(Line("dsa".to_string()), Int(1), &true);
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
        let result = condition(&String::from("test_key"), &true, &Int(10), &map);
        assert_eq!(result, true);
    }

    #[test]
    fn test_condition_int_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        let result = condition(&String::from("test_key"), &true, &Int(5), &map);
        assert_eq!(result, false);
    }

    #[test]
    fn test_condition_int_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        let result = condition(&String::from("test_key"), &false, &Int(10), &map);
        assert_eq!(result, false);
    }

    #[test]
    fn test_condition_int_four() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        let result = condition(&String::from("test_key"), &false, &Int(5), &map);
        assert_eq!(result, true);
    }

    #[test]
    fn test_condition_line_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Line("10".to_string()));
        let result = condition(
            &String::from("test_key"),
            &true,
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
            &true,
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
            &false,
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
            &false,
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
            &true,
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
            &false,
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
            &true,
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
            &false,
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
            &true,
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
            &false,
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
            &false,
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
            &true,
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
            &true,
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
            &true,
            &Line("test_key".to_string()),
            &map,
        );
        assert_eq!(result, true);
    }
}
