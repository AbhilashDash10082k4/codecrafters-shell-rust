use crate::utils::completion::lcp::longest_prefix_by_char_match;
use rustyline::completion::Pair;
use std::{io::{self, Write},cell::Cell};

pub fn autocomplete(prefix: &str, tab_cnt: &Cell<usize>, matches: Vec<&str>) -> Vec<Pair> {
   //Cell.set -> updates the old val with the curr val, drops the old val and nothing is returned
   let matches_len = matches.len();
   if matches_len == 0 {
      return Vec::new();
   }
   let mut vec_to_be_returned = vec![];
   if matches_len == 1 {
      vec_to_be_returned.push(Pair {
         display: matches[0].to_string(),
         replacement: format!("{} ", matches[0].to_string()),
      });
   }

   let lcp = longest_prefix_by_char_match(&matches);
   if matches_len > 1 {
      if lcp.len() > prefix.len() {
         //tab is making progress
         vec_to_be_returned.push(Pair {
            display: lcp.to_string(),
            replacement: format!("{}", lcp),
         });
         tab_cnt.set(0);
      } else if lcp.len() == prefix.len() {
         //tab is making no progress- continue with bell press
         if tab_cnt.get() == 1 {
            print!("\x07");
            io::stdout().flush().unwrap()
         } else if tab_cnt.get() == 2 {
            println!("\n{}", matches.join("  "));
            println!("$ {}", prefix);
            tab_cnt.set(0);
         }
      }
   }
   vec_to_be_returned
}
