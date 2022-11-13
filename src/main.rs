mod command_handler;

use std::{
    fmt::Debug,
    io::{stdin, stdout, Error, Write},
};

use command_handler::get_command_handler;

#[derive(Debug)]
pub struct Editor {
    text: Vec<String>,
}

impl Editor {
    fn index_out_of_bounds_check(&self, index: usize) -> Result<(), String> {
        if self.text.len() <= index {
            Err("given Paragraph does not exist".to_string())
        } else {
            Ok(())
        }
    }

    fn append_text(&mut self, paragraph: String) {
        self.text.push(paragraph);
    }

    fn append_text_at(&mut self, index: usize, paragraph: String) -> Result<(), String> {
        self.index_out_of_bounds_check(index)?;
        self.text.insert(index, paragraph);
        Ok(())
    }

    fn delete_paragraph(&mut self, index: usize) -> Result<(), String> {
        self.index_out_of_bounds_check(index)?;
        self.text.remove(index);
        Ok(())
    }

    fn replace_paragraph(&mut self, index: usize, new_paragraph: String) -> Result<(), String> {
        self.index_out_of_bounds_check(index)?;
        self.text[index] = new_paragraph;
        Ok(())
    }
}

pub struct UserInput {
    tokens: Vec<String>,
}

fn is_exit(user_input: &UserInput) -> bool {
    user_input
        .tokens
        .first()
        .unwrap_or(&String::new())
        .to_lowercase()
        == "exit".to_string()
}

fn get_user_input(buffer: &mut String, prompt: Option<&str>) -> Result<UserInput, std::io::Error> {
    print!("{}> ", prompt.unwrap_or(""));
    stdout().flush()?;
    stdin().read_line(buffer)?;
    Ok(UserInput {
        tokens: buffer
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),
    })
}

fn get_user_input_text(buffer: &mut String, prompt: Option<&str>) -> Result<String, Error> {
    print!("{}> ", prompt.unwrap_or(""));
    stdout().flush()?;
    stdin().read_line(buffer)?;
    Ok(buffer.trim().to_string())
}
#[derive(Debug, PartialEq, Eq)]
pub enum FormatMode {
    Raw,
    Fix(usize),
}
fn main() -> Result<(), Error> {
    let mut user_input = get_user_input(&mut String::new(), None)?;

    let mut editor = Editor { text: Vec::new() };
    let mut format_mode = FormatMode::Raw;
    while !is_exit(&user_input) {
        get_command_handler(&user_input, &format_mode)
            .and_then(|handler| handler.handle(&user_input, &mut editor, &mut |f| format_mode = f))
            .err()
            .and_then(|err| {
                println!("{}", err);
                Some(())
            });

        user_input = get_user_input(&mut String::new(), None)?;
    }
    Ok(())
}
