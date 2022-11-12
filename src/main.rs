mod command_handler;

use std::{
    error::Error,
    fmt::Debug,
    io::{stdin, stdout, Write},
};

use command_handler::get_command_handler;

#[derive(Debug)]
pub struct Editor {
    text: Vec<String>,
}

impl Editor {
    fn append_text(&mut self, paragraph: String) {
        self.text.push(paragraph);
    }
    fn append_text_at(&mut self, index: usize, paragraph: String) {
        self.text.insert(index, paragraph);
    }
    fn delete_paragraph(&mut self, index: usize) {
        self.text.remove(index);
    }
    fn replace_paragraph(&mut self, index: usize, new_paragraph_text: String) {
        self.text[index] = new_paragraph_text;
    }
}

pub struct UserInput {
    tokens: Vec<String>,
}

fn is_exit(user_input: &UserInput) -> bool {
    let empty_string = &String::new();
    user_input
        .tokens
        .first()
        .unwrap_or(empty_string)
        .to_lowercase()
        == "exit".to_string()
}

fn get_user_input(buffer: &mut String, prompt: Option<&str>) -> Result<UserInput, std::io::Error> {
    print!("{}> ", prompt.unwrap_or(""));
    stdout().flush()?;
    match stdin().read_line(buffer) {
        Ok(_n) => Ok(UserInput {
            tokens: buffer
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>(),
        }),
        Err(error) => Err(error),
    }
}

fn get_user_input_text(
    buffer: &mut String,
    prompt: Option<&str>,
) -> Result<String, std::io::Error> {
    print!("{}> ", prompt.unwrap_or(""));
    stdout().flush()?;
    match stdin().read_line(buffer) {
        Ok(_n) => Ok(buffer.trim().to_string()),
        Err(error) => Err(error),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    let mut user_input = get_user_input(&mut buffer, None)?;

    let mut editor = Editor { text: Vec::new() };

    while !is_exit(&user_input) {
        match get_command_handler(&user_input) {
            Ok(handler) => {
                handler.handle(&user_input, &mut editor);
                println!("{:?}", editor);
            }
            Err(e) => {
                println!("{}", e);
            }
        };

        let mut buffer = String::new();
        user_input = get_user_input(&mut buffer, None)?;
    }
    Ok(())
}
