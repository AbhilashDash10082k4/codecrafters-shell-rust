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
/*-iter-iterator -> refs to elems -> returns &&'static str
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

      /*start of the concerned word
      -line[..pos]=line before the cursor(currently typed line)
      -rfind -to return Option(idx) of the last space just before the cursor to ifnd the word to replace
      -map(|i|i+1) = takes the idx returned in Some(idx) and +1 to find the char next to last space(the first char of the word to be autocompleted)
      -if no such idx exists -start from 0
      */
      let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);

      let prefix = &line[start..pos];

      /*builtin completion */
      // 🔑 New completion cycle → reset TAB state
      self.last_was_tab.set(false);

      // 2️⃣ Collect ALL matches (names only)
      let mut matches: Vec<String> = Vec::new();

      // builtins
      for b in builtins {
         if b.starts_with(prefix) {
            matches.push(b.to_string());
         }
      }

      // executables
      matches.extend(
         find_completions(prefix)
            .into_iter()
            .filter_map(|p| p.file_name()?.to_str().map(|s| s.to_string())),
      );

      // normalize
      matches.sort();
      matches.dedup();

      // 3️⃣ EXACTLY ONE MATCH → autocomplete immediately
      if matches.len() == 1 {
         let m = matches.pop().unwrap();
         let replacement = format!("{m} ");

         return Ok((
            start,
            vec![Pair {
               display: m,
               replacement,
            }],
         ));
      }

      // 4️⃣ MULTIPLE MATCHES → double TAB logic
      if matches.len() > 1 {
         // MULTIPLE MATCHES → print immediately (NO bell, NO second TAB)
if matches.len() > 1 {
    println!("{}", matches.join("  "));
    print!("$ {}", prefix);
    std::io::stdout().flush().unwrap();
    self.last_was_tab.set(false);
    return Ok((start, vec![]));
}
      }

      // 5️⃣ No matches
      Ok((start, vec![]))
   }
}
