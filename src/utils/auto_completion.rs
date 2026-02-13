use crate::utils::path::{find_completions, find_executable, is_executable};
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
/*ingredients for stage 29-
pub enum CompletionType {
   List,
}
pub trait Candidate {
    // Required methods
    fn display(&self) -> &str;
    fn replacement(&self) -> &str;
}
pub struct Pair {
    pub display: String,
    pub replacement: String,
}
pub trait Completer {
    type Candidate: Candidate;

    // Provided methods
    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>)> { ... }
    fn update(
        &self,
        line: &mut LineBuffer,
        start: usize,
        elected: &str,
        cl: &mut Changeset,
    ) { ... }
}
pub enum CompletionType {
   List, Circular, Fuzzy
}
impl Builder{
   pub fn new() -> Self //Returns a Config builder.
   pub fn completion_type(self, completion_type: CompletionType) -> Self //Set completion_type.
   pub fn completion_show_all_if_ambiguous(
    self,
    completion_show_all_if_ambiguous: bool,
) -> Self //Choose whether or not to show all alternatives immediately when using list completion,By default, a second tab is needed.
   pub fn build(self) -> Config //Builds a Config with the settings specified so far.
}
impl Config {
   pub fn builder() -> Builder //Returns a Config builder.
   pub fn completion_type(&self) -> CompletionType //Completion behaviour.
   pub fn completion_show_all_if_ambiguous(&self) -> bool //Directly show all alternatives when using list completion, By default, they are not, a second tab is needed
}
impl<H: Helper> Editor<H, DefaultHistory>{
   pub fn new() -> Result<Self> //Create an editor with the default configuration
   pub fn with_config(config: Config) -> Result<Self> //Create an editor with a specific configuration.
   pub fn set_helper(&mut self, helper: Option<H>) //Register a callback function to be called for tab-completion or to show hints to the user at the right of the prompt.
   pub fn helper_mut(&mut self) -> Option<&mut H> //Return a mutable reference to the helper.
   pub fn helper(&self) -> Option<&H> //Return an immutable reference to the helper.
}
impl<H: Helper, I: History> Configurer for Editor<H, I>{
   fn config_mut(&mut self) -> &mut Config //Config accessor.
}*/
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
      /*CORE LOGIC-
      1match-> Autocomplete on first TAB press
      many matches->  Ring Bell on 1st TAB and then list all the matches*/
      let builtins = ["echo", "exit"];

      /*start of the concerned word
      -line[..pos]=line before the cursor(currently typed line)
      -rfind -to return Option(idx) of the last space just before the cursor to find the word to replace
      -map(|i|i+1) = takes the idx returned in Some(idx) and +1 to find the char next to last space(the first char of the word to be autocompleted)
      -if no such idx exists -start from 0
      */
      let start = line[0..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);

      // Increment tab count and get current value
      let tab_cnt = self.tab_cnt.get() + 1; //.get -> returns copy of current val
      self.tab_cnt.set(tab_cnt);
      //Cell.set -> updates the old val with the curr val, drops the old val and nothing is returned

      let prefix = &line[start..pos];
      let mut vec_to_be_returned: Vec<Pair> = vec![];

      /*all the matches in all possible cases*/
      /*1.matches with incomplete cmnd*/
      let list_paths = find_completions(&prefix);
      let mut file_names: Vec<_> = list_paths
         .iter()
         .filter_map(|f| f.file_name().and_then(|n| n.to_str()))
         .collect();
      file_names.sort();
      let count_files = file_names.len();

      if count_files > 1 {
         if tab_cnt == 1 {
            print!("\x07");
            io::stdout().flush().unwrap();
         }
         if tab_cnt == 2 {
            let file_list_as_string = file_names.join("  ");
            println!("\n{}", file_list_as_string);
            print!("$ {}", prefix);
            io::stdout().flush().unwrap();
            vec_to_be_returned.clear();
            self.tab_cnt.set(0); // Reset for next command
         }
      } else if count_files == 1 && tab_cnt == 1 {
         vec_to_be_returned.push(Pair {
            display: file_names[0].to_string(),
            replacement: format!("{} ", file_names[0].to_string()),
         })
      }

      /*2. matches with builtins*/
      let mut matched_builtins = vec![];
      for cmnd in builtins {
         if cmnd.starts_with(prefix) {
            matched_builtins.push(cmnd);
         }
      }
      let count_builtins = matched_builtins.len();
      if count_builtins > 1 {
         if tab_cnt == 1 {
            print!("\x07");
            io::stdout().flush().unwrap();
         }
         if tab_cnt == 2 {
            let matched_builtins_as_string = matched_builtins.join("  ");
            println!("\n{}", matched_builtins_as_string);
            print!("$ {}", prefix);
            io::stdout().flush().unwrap();
            vec_to_be_returned.clear();
            self.tab_cnt.set(0); // Reset for next command
         }
      } else if count_builtins == 1 && tab_cnt == 1 {
         vec_to_be_returned.push(Pair {
            display: matched_builtins[0].to_string(),
            replacement: format!("{} ", matched_builtins[0].to_string()),
         })
      }

      /*3. match with executables*/
      let mut matched_executable = vec![];
      let complete_executable_path = find_executable(&prefix);
      if let Some(p) = complete_executable_path {
         if p.is_file() && is_executable(&p) {
            matched_executable.push(p);
         }
      }
      let matched_executable_as_string: Vec<&str> = matched_executable
         .iter()
         .filter_map(|n| n.file_name().and_then(|f| f.to_str()))
         .collect();
      let count_executables = matched_executable_as_string.len();
      if count_executables > 1 {
         if tab_cnt == 1 {
            print!("\x07");
            io::stdout().flush().unwrap();
         }
         if tab_cnt == 2 {
            println!("\n{}", matched_executable_as_string.join("  "));
            print!("$ {}", prefix);
            io::stdout().flush().unwrap();
            vec_to_be_returned.clear();
            self.tab_cnt.set(0); // Reset for next command
         }
      } else if count_executables == 1 && tab_cnt == 1 {
         vec_to_be_returned.push(Pair {
            display: matched_executable_as_string[0].to_string(),
            replacement: format!("{} ", matched_executable_as_string[0].to_string()),
         })
      }
      Ok((start, vec_to_be_returned))
   }
}
fn autocomplete(prefix:&str, tab_cnt: Cell<usize>, matches: Vec<&str>) {
   // let tab_cnt = tab_cnt.get();
   let matches_len = matches.len();
   let mut vec_to_be_returned = vec![];
   if matches_len > 1 {
      if tab_cnt.get() == 1 {
         print!("\x07");
         io::stdout().flush().unwrap();
      }
      if tab_cnt.get() == 2 {
         println!("\n{}", matches.join(" "));
         print!("$ {}", prefix);
         io::stdout().flush().unwrap();
         vec_to_be_returned.clear();
         tab_cnt.set(0); // Reset for next command
      }
   } else if matches_len == 1 && tab_cnt.get() == 1 {
      vec_to_be_returned.push(Pair {
         display: matches[0].to_string(),
         replacement: format!("{}", matches[0].to_string()),
      })
   }
}
