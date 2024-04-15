
#[cfg(test)]
mod tests {
    use crate::adapter::sset::{SSet};
    use crate::libs::interface::Set;

    #[test]
    fn test_initial_sdict_with_data_passed() {
        let set : SSet<String> = SSet::<String>::new();
        assert_eq!(set.length(),0);
    }

    #[test]
    fn test_set_with_data_passed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        assert_eq!(do_set,Ok(true));
    }

    #[test]
    fn test_set_with_empty_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("").as_bytes().to_vec();
        let do_set = set.add(&key);
        assert_eq!(do_set, Err("Not support keyword"));
    }

    #[test]
    fn test_set_with_less_2_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("H").as_bytes().to_vec();
        let do_set = set.add(&key);
        assert_eq!(do_set, Err("Not support keyword"));
    }

    #[test]
    fn test_set_with_more_11_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("ABCDEFGHIJKLMN").as_bytes().to_vec();
        let do_set = set.add(&key);
        assert_eq!(do_set, Err("Not support keyword"));
    }

    #[test]
    fn test_get_with_data_passed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(key)));
    }

    #[test]
    fn test_get_with_empty_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&"".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Err("Not support keyword"));
    }

    #[test]
    fn test_get_with_invalid_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&"MYNAME".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Err("The first node is empty!."));
    }

    #[test]
    fn test_get_with_less_2_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&"M".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Err("Not support keyword"));
    }

    #[test]
    fn test_get_with_more_11_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&"ABCDEFGHIJKLMN".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Err("Not support keyword"));
    }

    #[test]
    fn test_length_with_data_passed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let key2 = String::from("HI").as_bytes().to_vec();
        let do_set1 = set.add(&key);
        let do_set2 = set.add(&key2);
        let size = set.length();
        assert_eq!(do_set1, Ok(true));
        assert_eq!(do_set2, Ok(true));
        assert_eq!(size,2);
    }

    #[test]
    fn test_length_with_duplicate_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set1 = set.add(&key);
        let do_set2 = set.add(&key);
        let size = set.length();
        assert_eq!(do_set1, Ok(true));
        assert_eq!(do_set2, Err("The key already seted"));
        assert_eq!(size,1);
    }

    #[test]
    fn test_delete_with_data_passed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let deleted = set.delete(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted, Ok(Some(key)));
    }

    #[test]
    fn test_delete_with_empty_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let deleted = set.delete(&"".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Err("Not support keyword"));
    }

    #[test]
    fn test_delete_with_invalid_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let deleted = set.delete(&"HH".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Err("The nodes of index is empty!."));
    }

    #[test]
    fn test_delete_with_less_2_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let deleted = set.delete(&"H".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Err("Not support keyword"));
    }

    #[test]
    fn test_delete_with_more_11_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let deleted = set.delete(&"ABCDEFGHIJKLMN".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Err("Not support keyword"));
    }

    #[test]
    fn test_delete_with_duplicate_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let deleted = set.delete(&key);
        let deleted2 = set.delete(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Ok(Some(key)));
        assert_eq!(deleted2,Err("The nodes of index is empty!."));
    }

    #[test]
    fn test_update_with_data_passed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&key);
        let do_updated = set.update(&key, &"HOW ARE YOU".to_string().as_bytes().to_vec());
        let result2 = set.get(&"HOW ARE YOU".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(key)));
        assert_eq!(do_updated, Ok(true));
        assert_eq!(result2,Ok(Some("HOW ARE YOU".to_string().as_bytes().to_vec())));
    }

    #[test]
    fn test_update_with_empty_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&key);
        let do_updated = set.update(&"".to_string().as_bytes().to_vec(), &"HOW ARE YOU".to_string().as_bytes().to_vec());
        let result2 = set.get(&"HOW ARE YOU".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(key)));
        assert_eq!(do_updated, Ok(true));
        assert_eq!(result2,Ok(Some("HOW ARE YOU".to_string().as_bytes().to_vec())));
    }

    #[test]
    fn test_update_with_invalid_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&key);
        let do_updated = set.update(&"ABCD".to_string().as_bytes().to_vec(), &"HOW ARE YOU".to_string().as_bytes().to_vec());
        let result2 = set.get(&"HOW ARE YOU".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(key)));
        assert_eq!(do_updated, Ok(true));
        assert_eq!(result2,Ok(Some("HOW ARE YOU".to_string().as_bytes().to_vec())));
    }

    #[test]
    fn test_update_with_less_2_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&key);
        let do_updated = set.update(&"A".to_string().as_bytes().to_vec(), &"HOW ARE YOU".to_string().as_bytes().to_vec());
        let result2 = set.get(&"HOW ARE YOU".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(key)));
        assert_eq!(do_updated, Ok(true));
        assert_eq!(result2,Ok(Some("HOW ARE YOU".to_string().as_bytes().to_vec())));
    }

    #[test]
    fn test_update_with_more_11_key_failed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&key);
        let do_updated = set.update(&"ABCDEFGHIJKLMN".to_string().as_bytes().to_vec(), &"HOW ARE YOU".to_string().as_bytes().to_vec());
        let result2 = set.get(&"HOW ARE YOU".to_string().as_bytes().to_vec());
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(key)));
        assert_eq!(do_updated, Ok(true));
        assert_eq!(result2,Ok(Some("HOW ARE YOU".to_string().as_bytes().to_vec())));
    }

    #[test]
    fn test_clear_with_data_passed() {
        let mut set : SSet<String> = SSet::<String>::new();
        let key = String::from("HELLO").as_bytes().to_vec();
        let do_set = set.add(&key);
        let result = set.get(&key);
        let len1 = set.length();
        let do_clear = set.clear();
        let len2 = set.length();
        let result1 = set.get(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(key)));
        assert_eq!(len1, 1);
        assert_eq!(do_clear,Ok(true));
        assert_eq!(result1,Err("The first node is empty!."));
        assert_eq!(len2, 0);
    }
}