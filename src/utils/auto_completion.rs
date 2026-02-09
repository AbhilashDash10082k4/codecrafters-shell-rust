use std::cell::Cell;
use std::io::{self, Write};

use crate::utils::path::{find_completions, find_executable, is_executable};
use rustyline::{
   Context, Helper,
   completion::{Completer, Pair},
   highlight::Highlighter,
   hint::Hinter,
   validate::Validator,
};

/*tabcompleter lives across calls*/
pub struct TabCompleter {
   pub tab_cnt: Cell<usize>,
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
refs -KeyCode,
-usize -start idx for replacement of text(an entire word), after a word it is the idx after the last space
-usize coz-it is big enough to index any obj in memory on this machine
    -for (32,64) bit machine-(u32,u64)
    - safe for memory operations
-Pair- display(suggestions to user),replace(actual replacement of og text -includes trailing space)
-separation of concerns-
    -Rustyline-(cursor pos, terminbal cntrl, i/p buffer)
    -Me- (completion logic, replacement text)
-Automcomplete -buffer replacement
-builtins -[&static str;2]->
static= lifetime==of entire program -data which references to data type which has lifetime of entire code
    -exists in binary, not on heap or stack
    -compile time is allocated
    -no lifetime annotations reqd, no dangling refs
-iter-iterator -> refs to elems -> returns &&'static str
-.filter(|b| b.starts_with(prefix) = filters out from the array the matching strings- uses auto-deref(*b) -> returns Iterator<Item = &&str>
-.map(|b| Pair {..} = takes this matched word and gives to 2 behaviours defined in Pair struct) -does auto-deref (*b) ->returns impl Iterator<Item = Pair>
*/
impl Completer for TabCompleter {
   type Candidate = Pair;
   fn complete(
      &self,
      line: &str,
      pos: usize,
      _ctx: &Context<'_>,
   ) -> rustyline::Result<(usize, Vec<Pair>)> {
      let builtins = ["echo", "exit"];

      // Increment tab count and get current value
      let tab_cnt = self.tab_cnt.get() + 1;
      self.tab_cnt.set(tab_cnt); //Cell.set -> updates the old val with the curr val, drops the old val and nothing is returned

      /*press bell*/
      if tab_cnt == 1 {
         print!("\x07");
         io::stdout().flush().unwrap();
      }

      /*start of the concerned word
      -line[..pos]=line before the cursor(currently typed line)
      -rfind -to return Option(idx) of the last space just before the cursor to find the word to replace
      -map(|i|i+1) = takes the idx returned in Some(idx) and +1 to find the char next to last space(the first char of the word to be autocompleted)
      -if no such idx exists -start from 0
      */
      let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);

      let prefix = &line[start..pos];
      let mut vec_to_be_returned: Vec<Pair> = vec![];

      /*compare the file with the last elem of prefix after splitting/using components*/
      let list_paths = find_completions(&prefix); //gives sorted list of file paths
      if tab_cnt == 2 {
         if !list_paths.is_empty() {
            let mut file_names: Vec<_> = list_paths
               .iter()
               .filter_map(|f| f.file_name().and_then(|n| n.to_str())) //filter_map => returns only Some(&str) vals
               .collect();
            file_names.sort(); // Sort alphabetically
            let file_list_as_string = file_names.join("  ");
            // println!("\n{}", file_list_as_string);
            vec_to_be_returned.clear(); // Don't show autocompletion suggestions when listing
            self.tab_cnt.set(0); // Reset for next command
         }
      }
      // builtins-for complete commands
      let matches: Vec<Pair> = builtins
         .iter()
         .filter(|b| b.starts_with(prefix))
         .map(|b| Pair {
            display: b.to_string(),
            replacement: format!("{b} "),
         })
         .collect();
      vec_to_be_returned = matches;

      //autocompletion for executable (for complete commands)-
      let complete_executable_path = find_executable(&prefix);
      if let Some(p) = complete_executable_path {
         if p.is_file() && is_executable(&p) {
            if let Some(path_to_display) = p.to_str() {
               vec_to_be_returned.push(Pair {
                  display: path_to_display.to_string(),
                  replacement: format!("{path_to_display} "),
               });
            }
         }
      }

      Ok((start, vec_to_be_returned))
   }
}
