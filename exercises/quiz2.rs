// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// Execute `rustlings hint quiz2` or use the `hint` watch subcommand for a hint.



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    input.trim().to_string()
}

fn to_uppercase(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_uppercase()
}

fn append(input: &str, times:usize) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let mut newWord=input.to_string();
    for number in (0..times).rev() {
        newWord.push_str("bar");
    }
         newWord
}

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String,Command)>) ->  Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String>  = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            match command {
             Command::Uppercase    =>output.push(to_uppercase(string)) ,
             Command::Append(x)    =>output.push(append(string,*x)) ,
             Command::Trim    =>output.push(trim_me(string)) ,
             
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we have to import to have `transformer` in scope?
    use my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
