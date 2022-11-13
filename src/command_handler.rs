use std::collections::{BTreeSet, HashMap};

use crate::{get_user_input_text, Editor, FormatMode, UserInput};

pub trait Command {
    fn handle(
        &self,
        user_input: &UserInput,
        editor: &mut Editor,
        format_mode_changer: &mut dyn FnMut(FormatMode) -> (),
    ) -> Result<(), String>;
}

struct AddCommand;

impl Command for AddCommand {
    fn handle(
        &self,
        user_input: &UserInput,
        editor: &mut Editor,
        _: &mut dyn FnMut(FormatMode) -> (),
    ) -> Result<(), String> {
        println!("add {:?}", user_input.tokens);

        let user_text = get_user_input_text(&mut String::new(), Some("text to insert"))
            .unwrap_or(String::new());

        if user_input.tokens.len() < 2 {
            editor.append_text(user_text);
            Ok(())
        } else {
            let index = user_input.tokens[1]
                .parse::<usize>()
                .map_err(|_| "invalid index value given")?;
            editor.append_text_at(index, user_text)
        }
    }
}

struct DummyCommand;

impl Command for DummyCommand {
    fn handle(
        &self,
        user_input: &UserInput,
        editor: &mut Editor,
        _: &mut dyn FnMut(FormatMode) -> (),
    ) -> Result<(), String> {
        println!("dummy {:?}", user_input.tokens);
        let dummy_paragraph = "this is a dummy paragraph text".to_string();
        if user_input.tokens.len() < 2 {
            editor.append_text(dummy_paragraph);
            Ok(())
        } else {
            let index = user_input.tokens[1]
                .parse::<usize>()
                .map_err(|_| "invalid index value given")?;
            editor.append_text_at(index, dummy_paragraph)
        }
    }
}
struct IndexCommand;

impl Command for IndexCommand {
    fn handle(
        &self,
        user_input: &UserInput,
        editor: &mut Editor,
        _: &mut dyn FnMut(FormatMode) -> (),
    ) -> Result<(), String> {
        println!("index {:?}", user_input.tokens);
        let mut index: HashMap<&str, BTreeSet<usize>> = HashMap::new();
        for (i, paragraph) in editor.text.iter().enumerate() {
            for word in paragraph.split_ascii_whitespace() {
                index
                    .entry(word)
                    .and_modify(|e| {
                        e.insert(i);
                    })
                    .or_insert(BTreeSet::from([i]));
            }
        }
        for (word, ocurrances) in index {
            if ocurrances.len() > 3 {
                println!(
                    "{}: {}",
                    word,
                    ocurrances
                        .iter()
                        .fold(String::new(), |res, occurance| res
                            + &format!("{}, ", occurance))
                        .trim_end_matches(", ")
                );
            }
        }

        Ok(())
    }
}
struct ReplaceCommand;

impl Command for ReplaceCommand {
    fn handle(
        &self,
        user_input: &UserInput,
        editor: &mut Editor,
        _: &mut dyn FnMut(FormatMode) -> (),
    ) -> Result<(), String> {
        println!("replace {:?}", user_input.tokens);

        if editor.text.is_empty() {
            return Err("nothing to replace".into());
        }

        let search_text = get_user_input_text(&mut String::new(), Some("text to search"))
            .unwrap_or(String::new());

        let replace_text = get_user_input_text(&mut String::new(), Some("text to replace"))
            .unwrap_or(String::new());

        let index = if user_input.tokens.len() < 2 {
            editor.text.len() - 1
        } else {
            user_input.tokens[1]
                .parse::<usize>()
                .map_err(|_| "invalid index value given")?
        };

        let new_paragraph = editor
            .text
            .get(index)
            .and_then(|last| Some(last.replace(&search_text, &replace_text)))
            .unwrap_or(String::new());

        editor.replace_paragraph(index, new_paragraph)
    }
}

struct PrintCommand;

impl Command for PrintCommand {
    fn handle(
        &self,
        _user_input: &UserInput,
        editor: &mut Editor,
        _: &mut dyn FnMut(FormatMode) -> (),
    ) -> Result<(), String> {
        for (i, paragraph) in editor.text.iter().enumerate() {
            println!("{}: {}", i, paragraph)
        }
        Ok(())
    }
}
struct PrintFixCommand {
    line_width: usize,
}

impl Command for PrintFixCommand {
    fn handle(
        &self,
        _user_input: &UserInput,
        editor: &mut Editor,
        _: &mut dyn FnMut(FormatMode) -> (),
    ) -> Result<(), String> {
        for paragraph in editor.text.iter() {
            let mut paragraph = paragraph.clone();
            while paragraph.len() > self.line_width {
                let line_lenght = paragraph[0..self.line_width + 1]
                    .rfind(" ")
                    .unwrap_or(self.line_width);

                println!("{}", paragraph[0..line_lenght].to_string());
                paragraph = paragraph[line_lenght..paragraph.len()].trim().into();
            }
            println!("{}", paragraph);
        }
        Ok(())
    }
}
struct FormatCommand;

impl Command for FormatCommand {
    fn handle(
        &self,
        user_input: &UserInput,
        _: &mut Editor,
        format_mode_changer: &mut dyn FnMut(FormatMode) -> (),
    ) -> Result<(), String> {
        if user_input.tokens.len() == 2 && user_input.tokens[1].to_lowercase() == "raw" {
            format_mode_changer(FormatMode::Raw);
            Ok(())
        } else if user_input.tokens.len() >= 3 && user_input.tokens[1].to_lowercase() == "fix" {
            let line_witdh = user_input.tokens[2]
                .parse::<usize>()
                .map_err(|_| "invalid line width value given")?;
            format_mode_changer(FormatMode::Fix(line_witdh));
            Ok(())
        } else {
            Err("no valid formating mode given".into())
        }
    }
}
struct DelCommand;

impl Command for DelCommand {
    fn handle(
        &self,
        user_input: &UserInput,
        editor: &mut Editor,
        _: &mut dyn FnMut(FormatMode) -> (),
    ) -> Result<(), String> {
        println!("del {:?}", user_input.tokens);

        if editor.text.is_empty() {
            return Err("nothing to delete".into());
        }

        let index = if user_input.tokens.len() < 2 {
            editor.text.len()
        } else {
            user_input.tokens[1]
                .parse::<usize>()
                .map_err(|_| "invalid index value given")?
        };
        editor.delete_paragraph(index)
    }
}

pub fn get_command_handler(
    user_input: &UserInput,
    format_mode: &FormatMode,
) -> Result<Box<dyn Command>, String> {
    match user_input
        .tokens
        .first()
        .unwrap_or(&String::new())
        .to_lowercase()
        .as_str()
    {
        "add" => Ok(Box::new(AddCommand)),
        "del" => Ok(Box::new(DelCommand)),
        "dummy" => Ok(Box::new(DummyCommand)),
        "replace" => Ok(Box::new(ReplaceCommand)),
        "print" => Ok(match format_mode {
            FormatMode::Raw => Box::new(PrintCommand),
            FormatMode::Fix(line_width) => Box::new(PrintFixCommand {
                line_width: line_width.clone(),
            }),
        }),
        "index" => Ok(Box::new(IndexCommand)),
        "format" => Ok(Box::new(FormatCommand)),
        &_ => Err("command invalid".into()),
    }
}
