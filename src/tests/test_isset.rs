
#[cfg(test)]
mod tests {
    use crate::adapter::iset::{ISet};

    #[test]
    fn test_initial_sdict_with_data_passed() {
        let set : ISet<String> = ISet::<String>::new();
        assert_eq!(set.length(),0);
    }

    #[test]
    fn test_set_with_data_passed() {
        let mut set : ISet<String> = ISet::<String>::new();
        let key = set.length() / 2;
        let do_set = set.add(&key);
        assert_eq!(do_set,Ok(true));
    }

   
    #[test]
    fn test_get_with_data_passed() {
        let mut set : ISet<String> = ISet::<String>::new();
        let key = set.length() / 2;
        let do_set = set.add(&key);
        let result = set.get(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(key)));
    }

    #[test]
    fn test_length_with_data_passed() {
        let mut set : ISet<String> = ISet::<String>::new();
        let key = set.length() / 2;
        let key2 = set.length() / 2 + 10;
        let do_set1 = set.add(&key);
        let do_set2 = set.add(&key2);
        let size = set.length();
        assert_eq!(do_set1, Ok(true));
        assert_eq!(do_set2, Ok(true));
        assert_eq!(size,2);
    }

    #[test]
    fn test_length_with_duplicate_key_failed() {
        let mut set : ISet<String> = ISet::<String>::new();
        let key = set.length() / 2;
        let do_set1 = set.add(&key);
        let do_set2 = set.add(&key);
        let size = set.length();
        assert_eq!(do_set1, Ok(true));
        assert_eq!(do_set2, Err("The key already seted"));
        assert_eq!(size,1);
    }

    #[test]
    fn test_delete_with_data_passed() {
        let mut set : ISet<String> = ISet::<String>::new();
        let key = set.length() / 2;
        let do_set = set.add(&key);
        let deleted = set.delete(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted, Ok(Some(key)));
    }

    #[test]
    fn test_delete_with_duplicate_key_failed() {
        let mut set : ISet<String> = ISet::<String>::new();
        let key = set.length() / 2;
        let do_set = set.add(&key);
        let deleted = set.delete(&key);
        let deleted2 = set.delete(&key);
        assert_eq!(do_set, Ok(true));
        assert_eq!(deleted,Ok(Some(key)));
        assert_eq!(deleted2,Err("The nodes of index is empty!."));
    }

    #[test]
    fn test_update_with_data_passed() {
        let mut set : ISet<String> = ISet::<String>::new();
        let key = set.length() / 2;
        let do_set = set.add(&key);
        let result = set.get(&key);
        let do_updated = set.update(&key, &(key + 10));
        let result2 = set.get(&(key + 10));
        assert_eq!(do_set, Ok(true));
        assert_eq!(result,Ok(Some(key)));
        assert_eq!(do_updated, Ok(true));
        assert_eq!(result2,Ok(Some(key + 10)));
    }

    #[test]
    fn test_clear_with_data_passed() {
        let mut set : ISet<String> = ISet::<String>::new();
        let key = set.length() / 2;
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