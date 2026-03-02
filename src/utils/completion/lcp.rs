pub fn longest_prefix_by_char_match(items: &Vec<&str>) -> String {
   if items.is_empty() {
      return String::new();
   }
   let smallest_word_len = items.iter().map(|w| w.len()).min();
   let mut res = String::new();
   if let Some(l) = smallest_word_len {
      for i in 0..l {
         let first_item_chars = items[0].as_bytes()[i];
         for item in items {
            if item.as_bytes()[i] != first_item_chars {
               return res;
            }
         }
         res.push(char::from(first_item_chars))
      }
   }

   return res;
}