pub fn can_construct(ransom_note: String, magazine: String) -> bool {
  if ransom_note.len() > magazine.len() {
    return false;
  }

  let mut map = [0; 26];
  let mut flags = 0;
  let idx = |c: char| c as usize - 'a' as usize;

  ransom_note.chars().for_each(|c| {
      let i = idx(c);
      map[i] += 1;
      flags |= 1 << i; // set flag
  });

  for c in magazine.chars() {
      let i = idx(c);
      map[i] -= 1;
      if map[i] == 0 {
          flags ^= 1 << i;  // drop flag
          if flags == 0 {
              return true;
          }
      }
  }
  false
}


#[cfg(test)]
mod tests {
    use super::can_construct;
    #[test]
    fn test_can_construct__false() {
        let ransom_note = String::from("a")
        let magazine = String::from("b")
        let mut actual = can_construct(nums);
        let expected = false;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_can_construct__true() {
        let ransom_note = String::from("aa")
        let magazine = String::from("aab")
        let mut actual = can_construct(nums);
        let expected = true;

        assert_eq!(actual, expected);
    }
}
