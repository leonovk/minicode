use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn condition(
    key: &String,
    true_or_false: &bool,
    target_value: &ValueType,
    storage: &HashMap<&String, ValueType>,
) -> bool {
    let value = storage.get(key);

    match value {
        Some(s) => match s {
            Int(first_integer) => match target_value {
                Int(second_integer) => {
                    condition_result(first_integer, second_integer, true_or_false)
                }
                Line(second_str) => match storage.get(second_str) {
                    Some(second_str_value) => match second_str_value {
                        Int(second_str_value_int) => {
                            condition_result(first_integer, second_str_value_int, true_or_false)
                        }
                        Line(_second_str_value_str) => {
                            panic!("You can't compare a string with a number")
                        }
                    },
                    None => panic!("You can't compare a string with a number"),
                },
            },
            Line(first_str) => match target_value {
                Int(_it) => panic!("You can't compare a string with a number"),
                Line(second_str) => match storage.get(second_str) {
                    Some(second_str_value) => match second_str_value {
                        Int(_second_str_value_int) => {
                            panic!("You can't compare a string with a number")
                        }
                        Line(second_str_value_str) => {
                            condition_result(first_str, second_str_value_str, true_or_false)
                        }
                    },
                    None => condition_result(first_str, second_str, true_or_false),
                },
            },
        },
        None => match target_value {
            Int(_second_int) => panic!("You can't compare a string with a number"),
            Line(second_line) => match storage.get(second_line) {
                Some(s) => match s {
                    Int(_second_value_int) => panic!("You can't compare a string with a number"),
                    Line(second_value_str) => {
                        condition_result(key, second_value_str, true_or_false)
                    }
                },
                None => condition_result(key, second_line, true_or_false),
            },
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
        let result = condition(&String::from("test_key"), &true, &Line("test_key_2".to_string()), &map);
        assert_eq!(result, false);
    }

    #[test]
    fn test_condition_where_key_not_in_hash_two() {
        let map: HashMap<&String, ValueType> = HashMap::new();
        let result = condition(&String::from("test_key"), &true, &Line("test_key".to_string()), &map);
        assert_eq!(result, true);
    }
}
