use std::collections::{HashMap, HashSet};

pub fn is_isomorphic(s: String, t: String) -> bool {
    let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());

    let mut map: HashMap<u8, u8> = HashMap::new();
    
    for i in 0..s_bytes.len() {
        let v = map.entry(s_bytes[i]).or_insert(t_bytes[i]);
        if *v != t_bytes[i] {
            return false;
        }
    }

    map.values().collect::<HashSet<_>>().len() == map.keys().collect::<HashSet<_>>().len()
}


mod tests {
    use super::is_isomorphic;
    #[test]
    fn test_is_isomorphic() {
        let s = String::from("egg");
        let t = String::from("add");
        let expected = true;
        let actual = is_isomorphic(s, t);
        assert_eq!(actual, expected);
    }
}
