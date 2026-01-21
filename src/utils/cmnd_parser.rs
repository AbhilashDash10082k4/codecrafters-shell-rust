use crate::commands::command::UserInput;
/*Parsing problems are solved by state machines, not string tricks
raw str to vec str but by not splitting based on whitespaces
smart splitting is done considering spaces within quotes chars*/

/*handling quotes
-prints the entire string between ''.
-empty '' are ignored
-two words placed side by side are concatenated
M1-
if cmd.raw.starts_with("'") && cmd.raw.ends_with("'") {
    println!("{}", cmd.raw);
} -This wont work coz this needs context and not characters
-This is a tokenizer
*/
/*stage15 approach
curr_arg_buffer, args, in_quotes
-react to white spaces, build args, store chars
*/
pub fn handle(cmnd: &UserInput) -> Vec<String> {
    let curr_arg_buffer = cmnd.raw.trim().chars();
    let mut curr_arg = String::new();
    
    /*a flag*/
    let mut in_quotes = false;
    let mut in_double_quotes = false;
    
    /*complete set of args*/
    let mut args: Vec<String> = Vec::new();
    
    /*stage 18- double quotes*/
    let double_quotes= '\"';

    for c in curr_arg_buffer {
        /*3 diff behaviours
        Case1 - c = '\'' -controls the quote mode and is not added in o/p
        Case2 - c = ' ' and not in quotes -ends arg if is_quotes = false
        Case3 - c = any other char - append it to curr_arg*/
        if c == '\'' && !in_double_quotes{
            /*toggling the quote mode -no storing of ' in o/p
            -toggles only if not in "" */
            in_quotes = !in_quotes;
        }
        if c == double_quotes && !in_quotes{
            /*toggles only if not in ''*/
            in_double_quotes = !in_double_quotes;
        }
        /*handling of special ' ' that are inside the '' 
        c = ' ' and not in quotes or double_quotes (outside quotes)
        c = '\'' and in double quotes*/ 
        else if c == ' ' && (!in_quotes && !in_double_quotes) {
            /*split the main cmnd and args*/
            if !(&curr_arg.is_empty()) {
                /*this line -args.push(curr_arg) takes the ownership of curr_arg and the condition in if becomes invalid.*/
                args.push(curr_arg);

                /*to prevent this condition, a new val is assigned to curr_arg -this is emptying buffer*/
                curr_arg = String::new();
            }
        } else {
            /*every other char -for double quotes, if ' is in "", then consider it as a char */
            curr_arg.push(c);
        }
    }
    if !curr_arg.is_empty() {
        args.push(curr_arg);
    }
    args
}
