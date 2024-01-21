use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn create<'a>(key: &'a String, value: &ValueType, target: &mut HashMap<&'a String, ValueType>) {
    match value {
        Int(int) => target.insert(key, Int(*int)),
        Line(str) => match target.get(str) {
            Some(some) => target.insert(key, some.clone()),
            None => match complex_assignments_value(str, target) {
                Some(some_value) => target.insert(key, some_value),
                None => target.insert(key, value.clone()),
            },
        },
    };
}

fn complex_assignments_value<'a>(
    expression: &String,
    target: &HashMap<&'a String, ValueType>,
) -> Option<ValueType> {
    let values: Vec<&str> = expression.split_whitespace().collect();

    if values.len() == 2 {
        match target.get(&values[0].to_string()) {
            Some(some) => match some {
                Int(_i) => None,
                Line(str) => match values[1].to_string().parse::<i64>() {
                    Ok(parsed) => Some(Line(str.chars().nth(parsed as usize).unwrap().to_string())),
                    Err(_) => match target.get(&values[1].to_string()) {
                        Some(some_second) => match some_second {
                            Int(sec_int) => Some(Line(
                                str.chars().nth(*sec_int as usize).unwrap().to_string(),
                            )),
                            Line(_l) => None,
                        },
                        None => None,
                    },
                },
            },
            None => None,
        }
    } else if values.len() == 1 {
        match target.get(&values[0].to_string()) {
            Some(s) => match s {
                Int(int) => Some(Int(*int)),
                Line(str) => Some(Line(str.to_string())),
            },
            None => None,
        }
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn test_basic_create_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        create(&binding, &Int(10), &mut map);
        let result = map.get(&binding);
        assert_eq!(result, Some(&Int(10)));
    }

    #[test]
    fn test_basic_create_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        create(&binding, &Line("b 2".to_string()), &mut map);
        let result = map.get(&binding);
        assert_eq!(result, Some(&Line("b 2".to_string())));
    }

    #[test]
    fn test_basic_create_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        create(&binding, &Line("qwert".to_string()), &mut map);
        let result = map.get(&binding);
        assert_eq!(result, Some(&Line("qwert".to_string())));
    }

    #[test]
    fn test_not_basic_create_one() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let new_key = String::from("new_key");

        map.insert(&old_key, Int(10));

        create(&new_key, &Line("old_key".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Int(10)));
        assert_eq!(result_2, Some(&Int(10)));
    }

    #[test]
    fn test_not_basic_create_two() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let new_key = String::from("new_key");

        map.insert(&old_key, Line("line".to_string()));

        create(&new_key, &Line("old_key".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Line("line".to_string())));
        assert_eq!(result_2, Some(&Line("line".to_string())));
    }

    #[test]
    fn test_not_basic_create_three() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let new_key = String::from("new_key");

        map.insert(&old_key, Line("line".to_string()));

        create(&new_key, &Line("old_key 1".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Line("line".to_string())));
        assert_eq!(result_2, Some(&Line("i".to_string())));
    }

    #[test]
    fn test_not_basic_create_four() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let old_key = String::from("old_key");
        let old_key_2 = String::from("old_key_2");
        let new_key = String::from("new_key");

        map.insert(&old_key, Line("line".to_string()));
        map.insert(&old_key_2, Int(1));

        create(&new_key, &Line("old_key old_key_2".to_string()), &mut map);

        let result_1 = map.get(&old_key);
        let result_2 = map.get(&new_key);

        assert_eq!(result_1, Some(&Line("line".to_string())));
        assert_eq!(result_2, Some(&Line("i".to_string())));
    }
}
