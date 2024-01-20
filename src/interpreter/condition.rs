use crate::opcode::ValueType;
use crate::opcode::ValueType::*;
use std::collections::HashMap;

pub fn condition(
    key: &String,
    true_or_false: &bool,
    target_value: &i64,
    storage: &HashMap<&String, ValueType>,
) -> bool {
    let value = storage.get(key);

    match value {
        Some(s) => match s {
            Int(i) => {
                if *true_or_false {
                    return i == target_value;
                } else {
                    return i != target_value;
                }
            }
            Line(_s) => panic!("condition - wrong type value"),
        },
        None => panic!("condition - not value"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_condition_true_int() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        let result = condition(&String::from("test_key"), &true, &10, &map);
        assert_eq!(result, true);
    }

    #[test]
    fn test_condition_false_int() {
        let mut map: HashMap<&String, ValueType> = HashMap::new();
        let binding = String::from("test_key");
        map.insert(&binding, Int(10));
        let result = condition(&String::from("test_key"), &false, &5, &map);
        assert_eq!(result, true);
    }
}
