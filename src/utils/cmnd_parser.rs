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
/*stage22- to change this parser from working for only arguments to working for executables (token #0)*/
pub fn handle(cmnd: &UserInput) -> Vec<String> {
    let curr_arg_buffer = cmnd.raw.trim().chars();
    let mut curr_arg = String::new();

    /*a flag*/
    let mut in_quotes = false;
    let mut in_double_quotes = false;

    /*complete set of args*/
    let mut args: Vec<String> = Vec::new();

    /*stage 18- double quotes*/
    let double_quotes = '\"';

    /*stage19 - escaped*/
    let mut escaped = false;
    let slash = '\\';

    /*3 diff behaviours
    Case1 - c = '\'' -controls the quote mode and is not added in o/p
    Case2 - c = ' ' and not in quotes -ends arg if is_quotes = false
    Case3 - c = any other char - append it to curr_arg*/
    /*stage 19 to 22-backslash handling -'\' is not a state trigger, it is  one-shot and is not persistent
    -if escaped = true-push the nxt char as it is, if false, then proceed normally
    -correct order - back_slash-> single_quotes, double_quotes-> space splitting-> literal char
    -reason for this order -effect of these rules on parsing(scope of influencing)
    -Rules that change interpretation must run before rules that consume characters -here \ changes interpret. and quotes and spaces consume chars*/
    for c in curr_arg_buffer {
        if c == slash {
            if !in_double_quotes && !in_quotes {
                escaped = true;
                continue;
            } else if in_quotes && !in_double_quotes {
                curr_arg.push(c);
                continue;
            } else if in_double_quotes && !in_quotes {
                escaped = true;
                continue;
            }
        }
        if escaped {
            if in_double_quotes {
                if c == double_quotes || c == slash {
                    curr_arg.push(c);
                    continue;
                } else {
                    // Invalid escape → keep backslash literal
                    curr_arg.push('\\');
                    curr_arg.push(c);
                }
            }
            //for single or outside of quotes
            curr_arg.push(c);
            escaped = false;
            continue;
        }
        if c == double_quotes && !in_quotes {
            /*toggles only if not in ''*/
            in_double_quotes = !in_double_quotes;
            continue;
        }
        if c == '\'' && !in_double_quotes {
            /*toggling the quote mode -no storing of ' in o/p
            -toggles only if not in "" */
            in_quotes = !in_quotes;
            /*adding continue helps as it skips the toggling char as soon as it is matched
            control chars must short circuit processing (skip the entire loop on matching)*/
            continue;
        }
        /*handling of special ' ' that are inside the ''
        c = ' ' and not in quotes or double_quotes (outside quotes) -only 1 state can be active a t a time
        c = '\'' and in double quotes*/
        else if c == ' ' && (!in_quotes && !in_double_quotes) {
            /*split the main cmnd and args*/
            if !(&curr_arg.is_empty()) {
                /*this line -args.push(curr_arg) takes the ownership of curr_arg and the condition in if becomes invalid.*/
                args.push(curr_arg);

                /*to prevent this condition, a new val is assigned to curr_arg -this is emptying buffer
                this also leads to end the current arg*/
                curr_arg = String::new();
            }
            continue;
        } else {
            /*every other char -for double quotes, if ' is in "", then consider it as a char
            -rules for pushing literal chars -
            skip ' if it toggles in_quotes and same for "
            before skipping the loop by using continue, the quotes used to leak into this loop and were added to args */
            curr_arg.push(c);
        }
    }
    if !curr_arg.is_empty() {
        args.push(curr_arg);
    }
    args
}
