use rustyline::{
    Context, Helper,
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
};
use std::env;
use walkdir::WalkDir;
pub struct TabCompleter;
impl Helper for TabCompleter {}
impl Validator for TabCompleter {}
impl Highlighter for TabCompleter {}
impl Hinter for TabCompleter {
    type Hint = String;
}
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
        let mut vec_pair: Vec<Pair> = vec![];

        let prefix = &line[start..pos];

        /*-iter-iterator -> refs to elems -> returns &&'static str
        -.filter(|b| b.starts_with(prefix) = filters out from the array the matching strings- uses auto-deref(*b) -> returns Iterator<Item = &&str>
        -.map(|b| Pair {..} = takes this matched word and gives to 2 behaviours defined in Pair struct) -does auto-deref (*b) ->returns impl Iterator<Item = Pair>
        */
        let matches = builtins
            .iter()
            .filter(|b| b.starts_with(prefix))
            .map(|b| Pair {
                display: b.to_string(),
                replacement: format!("{b} "),
            })
            .collect();
        vec_pair = matches;

        /*autocompletion for executables -cmnd given ->somehow compare this cmnd with an existing file -> if maximum match -> autocomplete*/
        /*compare the file with the last elem of prefix after splitting/using components*/
        let path = env::var("PATH").ok();
        match path {
            Some(p) => {
                for entry in p.split(":") {
                    for f in WalkDir::new(entry).into_iter().filter_map(|e| e.ok()) {
                        // file_name_iter.push(f);
                        if let Some(file) = f.file_name().to_str() {
                            if file.starts_with(prefix) {
                                vec_pair.push(Pair {
                                    display: file.to_string(),
                                    replacement: format!("{file} "),
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Ok((start, vec_pair))
    }
}
