use rustyline::{
    Context, Helper,
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
};
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
        let builtins = ["echo", "exit"];

        let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);

        let prefix = &line[start..pos];

        let matches = builtins
            .iter()
            .filter(|b| b.starts_with(prefix))
            .map(|b| Pair {
                display: b.to_string(),
                replacement: format!("{b} "),
            })
            .collect();

        Ok((start, matches))
    }
}
