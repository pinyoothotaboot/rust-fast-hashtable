
#[cfg(test)]
mod tests {
    use crate::adapter::sdict::{SDict};
    use crate::libs::interface::Dict;

    #[test]
    fn test_initial_sdict_with_data_passed() {
        let dict : SDict<String> = SDict::<String>::new();
        assert_eq!(dict.length(),0);
    }

    #[test]
    fn test_set_with_data_passed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key, value);
        assert_eq!(do_set,Ok(true));
    }

    #[test]
    fn test_set_with_empty_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("");
        let value = String::from("How are you");
        let do_set = dict.set(key, value);
        assert_eq!(do_set, Err("Not support keyword"));
    }

    #[test]
    fn test_set_with_less_2_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("H");
        let value = String::from("How are you");
        let do_set = dict.set(key, value);
        assert_eq!(do_set, Err("Not support keyword"));
    }

    #[test]
    fn test_set_with_more_11_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("ABCDEFGHIJKLMN");
        let value = String::from("How are you");
        let do_set = dict.set(key, value);
        assert_eq!(do_set, Err("Not support keyword"));
    }

    #[test]
    fn test_get_with_data_passed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get(key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value)));
    }

    #[test]
    fn test_get_with_empty_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get("".to_string());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Err("Not support keyword"));
    }

    #[test]
    fn test_get_with_invalid_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get("MYNAME".to_string());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Err("The first node is empty!."));
    }

    #[test]
    fn test_get_with_less_2_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get("M".to_string());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Err("Not support keyword"));
    }

    #[test]
    fn test_get_with_more_11_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get("ABCDEFGHIJKLMN".to_string());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Err("Not support keyword"));
    }

    #[test]
    fn test_length_with_data_passed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let key2 = String::from("HI");
        let value = String::from("How are you");
        let do_set1 = dict.set(key.clone(), value.clone());
        let do_set2 = dict.set(key2.clone(), value.clone());
        let size = dict.length();
        assert_eq!(do_set1, Ok(true));
        assert_eq!(do_set2, Ok(true));
        assert_eq!(size,2);
    }

    #[test]
    fn test_length_with_duplicate_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set1 = dict.set(key.clone(), value.clone());
        let do_set2 = dict.set(key.clone(), value.clone());
        let size = dict.length();
        assert_eq!(do_set1, Ok(true));
        assert_eq!(do_set2, Err("The key already seted"));
        assert_eq!(size,1);
    }

    #[test]
    fn test_delete_with_data_passed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let deleted = dict.delete(key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted, Ok(Some(value)));
    }

    #[test]
    fn test_delete_with_empty_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let deleted = dict.delete("".to_string());
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Err("Not support keyword"));
    }

    #[test]
    fn test_delete_with_invalid_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let deleted = dict.delete("HH".to_string());
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Err("The nodes of index is empty!."));
    }

    #[test]
    fn test_delete_with_less_2_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let deleted = dict.delete("H".to_string());
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Err("Not support keyword"));
    }

    #[test]
    fn test_delete_with_more_11_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let deleted = dict.delete("ABCDEFGHIJKLMN".to_string());
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Err("Not support keyword"));
    }

    #[test]
    fn test_delete_with_duplicate_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let deleted = dict.delete(key.clone());
        let deleted2 = dict.delete(key.clone());
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Ok(Some(value.clone())));
        assert_eq!(deleted2,Err("The nodes of index is empty!."));
    }

    #[test]
    fn test_update_with_data_passed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let new_value = String::from("I'm fine");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get(key.clone());
        let do_updated = dict.update(key.clone(), new_value.clone());
        let result2 = dict.get(key.clone());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value)));
        assert_eq!(do_updated, Ok(true));
        assert_eq!(result2,Ok(Some(new_value)));
    }

    #[test]
    fn test_update_with_empty_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let new_value = String::from("I'm fine");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get(key.clone());
        let do_updated = dict.update("".to_string(), new_value.clone());
        let result2 = dict.get(key.clone());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value.clone())));
        assert_eq!(do_updated, Err("Not support keyword"));
        assert_eq!(result2,Ok(Some(value)));
    }

    #[test]
    fn test_update_with_invalid_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let new_value = String::from("I'm fine");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get(key.clone());
        let do_updated = dict.update("ABCD".to_string(), new_value.clone());
        let result2 = dict.get(key.clone());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value.clone())));
        assert_eq!(do_updated, Err("Not found node matched key!."));
        assert_eq!(result2,Ok(Some(value)));
    }

    #[test]
    fn test_update_with_less_2_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let new_value = String::from("I'm fine");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get(key.clone());
        let do_updated = dict.update("A".to_string(), new_value.clone());
        let result2 = dict.get(key.clone());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value.clone())));
        assert_eq!(do_updated, Err("Not support keyword"));
        assert_eq!(result2,Ok(Some(value)));
    }

    #[test]
    fn test_update_with_more_11_key_failed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let new_value = String::from("I'm fine");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get(key.clone());
        let do_updated = dict.update("ABCDEFGHIJKLMN".to_string(), new_value.clone());
        let result2 = dict.get(key.clone());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value.clone())));
        assert_eq!(do_updated, Err("Not support keyword"));
        assert_eq!(result2,Ok(Some(value)));
    }

    #[test]
    fn test_clear_with_data_passed() {
        let mut dict : SDict<String> = SDict::<String>::new();
        let key = String::from("HELLO");
        let value = String::from("How are you");
        let do_set = dict.set(key.clone(), value.clone());
        let result = dict.get(key.clone());
        let len1 = dict.length();
        let do_clear = dict.clear();
        let len2 = dict.length();
        let result1 = dict.get(key.clone());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value.clone())));
        assert_eq!(len1, 1);
        assert_eq!(do_clear,Ok(true));
        assert_eq!(result1,Err("The first node is empty!."));
        assert_eq!(len2, 0);
    }
}