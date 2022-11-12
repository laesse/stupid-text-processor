use crate::{get_user_input_text, Editor, UserInput};

pub trait Command {
    fn handle(&self, user_input: &UserInput, editor: &mut Editor);
}

struct AddCommand;

impl Command for AddCommand {
    fn handle(&self, user_input: &UserInput, editor: &mut Editor) {
        println!("add {:?}", user_input.tokens);

        let mut buffer = String::new();
        let user_text =
            get_user_input_text(&mut buffer, Some("text to insert")).unwrap_or(String::new());

        if user_input.tokens.len() < 2 {
            editor.append_text(user_text);
        } else {
            let index = user_input.tokens[1].parse::<usize>().unwrap();
            editor.append_text_at(index, user_text);
        }
    }
}

struct DummyCommand;

impl Command for DummyCommand {
    fn handle(&self, user_input: &UserInput, editor: &mut Editor) {
        println!("dummy {:?}", user_input.tokens);

        editor.append_text("this is a dummy paragraph text".to_string());
    }
}
struct ReplaceCommand;

impl Command for ReplaceCommand {
    fn handle(&self, user_input: &UserInput, editor: &mut Editor) {
        println!("replace {:?}", user_input.tokens);

        let mut buffer = String::new();
        let search_text =
            get_user_input_text(&mut buffer, Some("text to search")).unwrap_or(String::new());

        let mut buffer = String::new();
        let replace_text =
            get_user_input_text(&mut buffer, Some("text to replace")).unwrap_or(String::new());

        if user_input.tokens.len() < 2 {
            let new_paragarph = match editor.text.last() {
                Some(last) => last.replace(&search_text, &replace_text),
                None => String::new(),
            };
            editor.replace_paragraph(editor.text.len() - 1, new_paragarph);
        } else {
            let index = user_input.tokens[1].parse::<usize>().unwrap();
            let new_paragraph = match editor.text.get(index) {
                Some(last) => last.replace(&search_text, &replace_text),
                None => String::new(),
            };
            editor.replace_paragraph(index, new_paragraph);
        }
    }
}

struct PrintCommand;

impl Command for PrintCommand {
    fn handle(&self, _user_input: &UserInput, editor: &mut Editor) {
        for (i, paragraph) in editor.text.iter().enumerate() {
            println!("{}: {}", i, paragraph)
        }
    }
}
struct DelCommand;

impl Command for DelCommand {
    fn handle(&self, user_input: &UserInput, editor: &mut Editor) {
        println!("del {:?}", user_input.tokens);

        if user_input.tokens.len() < 2 {
            if !editor.text.is_empty() {
                editor.delete_paragraph(editor.text.len() - 1);
            }
        } else {
            let index = user_input.tokens[1].parse::<usize>().unwrap();
            editor.delete_paragraph(index);
        }
    }
}

pub fn get_command_handler(user_input: &UserInput) -> Result<Box<dyn Command>, String> {
    let empty_string = &String::new();

    match user_input
        .tokens
        .first()
        .unwrap_or(empty_string)
        .to_lowercase()
        .as_str()
    {
        "add" => Ok(Box::new(AddCommand)),
        "del" => Ok(Box::new(DelCommand)),
        "dummy" => Ok(Box::new(DummyCommand)),
        "replace" => Ok(Box::new(ReplaceCommand)),
        "print" => Ok(Box::new(PrintCommand)),
        &_ => Err("command invalid".to_string()),
    }
}
