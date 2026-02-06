use crate::utils::path::find_completions;
use rustyline::{
   Context, Helper,
   completion::{Completer, Pair},
   highlight::Highlighter,
   hint::Hinter,
   validate::Validator,
};
use std::cell::Cell;
use std::io::{self, Write};

/*tabcompleter lives across calls*/
pub struct TabCompleter {
   pub last_was_tab: Cell<bool>,
}
impl Helper for TabCompleter {}
impl Validator for TabCompleter {}
impl Highlighter for TabCompleter {}
impl Hinter for TabCompleter {
   type Hint = String;
}
/*stage29- keep a track of no. of TABs
-find all executables, print them with 2ble space
-do not autocomplete even after 2TAB clicks
refs -KeyCode, */
impl Completer for TabCompleter {
   type Candidate = Pair;
   fn complete(
      &self,
      line: &str,
      pos: usize,
      _ctx: &Context<'_>,
   ) -> rustyline::Result<(usize, Vec<Pair>)> {
      /*-usize -start idx for replacement of text(an entire word), after a word it is the idx after the last space
      -usize coz-it is big enough to index any obj in memory on this machine
          -for (32,64) bit machine-(u32,u64)
          - safe for memory operations
      -Pair- display(suggestions to user),replace(actual replacement of og text -includes trailing space)
      -separation of concerns-
          -Rustyline-(cursor pos, terminbal cntrl, i/p buffer)
          -Me- (completion logic, replacement text)
      -Automcomplete -buffer replacement*/
      /*builtins -[&static str;2]->
      static= lifetime==of entire program -data which references to data type which has lifetime of entire code
          -exists in binary, not on heap or stack
          -compile time is allocated
          -no lifetime annotations reqd, no dangling refs*/
      let builtins = ["echo", "exit"];

      /*start of the concerned word
      -line[..pos]=line before the cursor(currently typed line)
      -rfind -to return Option(idx) of the last space just before the cursor to ifnd the word to replace
      -map(|i|i+1) = takes the idx returned in Some(idx) and +1 to find the char next to last space(the first char of the word to be autocompleted)
      -if no such idx exists -start from 0
      */
      let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);

      /*value to be returned*/
      let mut pairs: Vec<Pair> = Vec::new();

      let prefix = &line[start..pos];

      /*-iter-iterator -> refs to elems -> returns &&'static str
      -.filter(|b| b.starts_with(prefix) = filters out from the array the matching strings- uses auto-deref(*b) -> returns Iterator<Item = &&str>
      -.map(|b| Pair {..} = takes this matched word and gives to 2 behaviours defined in Pair struct) -does auto-deref (*b) ->returns impl Iterator<Item = Pair>
      */
      /*builtin completion */
      for b in builtins {
        if b.starts_with(prefix) {
            pairs.push(Pair {
                display: b.to_string(),
                replacement: format!("{b} "),
            });
        }
    }

      /*autocompletion for executables -cmnd given ->somehow compare this cmnd with an existing file -> if maximum match -> autocomplete*/
      /*compare the file with the last elem of prefix after splitting/using components*/
      let mut list_paths: Vec<String> = find_completions(&prefix)
         .into_iter()
         .filter_map(|p| p.file_name()?.to_str().map(|s| s.to_string()))
         .collect();

      list_paths.sort();
      /*multiple tabs*/
      if list_paths.len() > 1 {
         if !self.last_was_tab.get() {
            print!("\x07"); // bell
            io::stdout().flush().unwrap();
            self.last_was_tab.set(true);
            return Ok((start, vec![]));
         } else {
            println!("\n{}", list_paths.join("  "));
            print!("$ {}", prefix);
            io::stdout().flush().unwrap();
            self.last_was_tab.set(false);
            return Ok((start, vec![]));
         }
      }
      //Single executable match → autocomplete (NO clone)
    if let Some(m) = list_paths.pop() {
        let replacement = format!("{m} ");
        pairs.push(Pair {
            display: m,
            replacement,
        });
    }
      self.last_was_tab.set(false);
      Ok((start, pairs))
   }
}
