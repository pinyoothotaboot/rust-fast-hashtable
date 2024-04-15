
#[cfg(test)]
mod tests {
    use crate::adapter::idict::{IDict};
    use crate::libs::interface::Dict;

    #[test]
    fn test_initial_IDict_with_data_passed() {
        let dict : IDict<String> = IDict::<String>::new();
        assert_eq!(dict.length(),0);
    }

    #[test]
    fn test_set_with_data_passed() {
        let mut dict : IDict<String> = IDict::<String>::new();
        let key : usize = dict.length() / 2;
        let value = String::from("How are you");
        let do_set = dict.set(&key, value);
        assert_eq!(do_set,Ok(true));
    }

    #[test]
    fn test_get_with_data_passed() {
        let mut dict : IDict<String> = IDict::<String>::new();
        let key : usize = dict.length() / 2;
        let value = String::from("How are you");
        let do_set = dict.set(&key, value.clone());
        let result = dict.get(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value)));
    }

    #[test]
    fn test_length_with_data_passed() {
        let mut dict : IDict<String> = IDict::<String>::new();
        let key : usize = dict.length() / 2;
        let key2 : usize = dict.length() / 2 + 10;
        let value = String::from("How are you");
        let do_set1 = dict.set(&key, value.clone());
        let do_set2 = dict.set(&key2, value.clone());
        let size = dict.length();
        assert_eq!(do_set1, Ok(true));
        assert_eq!(do_set2, Ok(true));
        assert_eq!(size,2);
    }

    #[test]
    fn test_length_with_duplicate_key_failed() {
        let mut dict : IDict<String> = IDict::<String>::new();
        let key : usize = dict.length() / 2;
        let value = String::from("How are you");
        let do_set1 = dict.set(&key, value.clone());
        let do_set2 = dict.set(&key, value.clone());
        let size = dict.length();
        assert_eq!(do_set1, Ok(true));
        assert_eq!(do_set2, Err("The key already seted"));
        assert_eq!(size,1);
    }

    #[test]
    fn test_delete_with_data_passed() {
        let mut dict : IDict<String> = IDict::<String>::new();
        let key : usize = dict.length() / 2;
        let value = String::from("How are you");
        let do_set = dict.set(&key, value.clone());
        let deleted = dict.delete(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted, Ok(Some(value)));
    }

    #[test]
    fn test_delete_with_duplicate_key_failed() {
        let mut dict : IDict<String> = IDict::<String>::new();
        let key : usize = dict.length() / 2;
        let value = String::from("How are you");
        let do_set = dict.set(&key, value.clone());
        let deleted = dict.delete(&key);
        let deleted2 = dict.delete(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Ok(Some(value.clone())));
        assert_eq!(deleted2,Err("The nodes of index is empty!."));
    }

    #[test]
    fn test_update_with_data_passed() {
        let mut dict : IDict<String> = IDict::<String>::new();
        let key : usize = dict.length() / 2;
        let value = String::from("How are you");
        let new_value = String::from("I'm fine");
        let do_set = dict.set(&key, value.clone());
        let result = dict.get(&key);
        let do_updated = dict.update(&key, new_value.clone());
        let result2 = dict.get(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value)));
        assert_eq!(do_updated, Ok(true));
        assert_eq!(result2,Ok(Some(new_value)));
    }
    
    #[test]
    fn test_clear_with_data_passed() {
        let mut dict : IDict<String> = IDict::<String>::new();
        let key : usize = dict.length() / 2;
        let value = String::from("How are you");
        let do_set = dict.set(&key, value.clone());
        let result = dict.get(&key);
        let len1 = dict.length();
        let do_clear = dict.clear();
        let len2 = dict.length();
        let result1 = dict.get(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(value.clone())));
        assert_eq!(len1, 1);
        assert_eq!(do_clear,Ok(true));
        assert_eq!(result1,Err("The first node is empty!."));
        assert_eq!(len2, 0);
    }
}