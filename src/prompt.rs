use crate::{Error, Result};
use std::io::{self, Write};
use std::str::FromStr;

macro_rules! invalid_number_msg {
    () => {
        "Please enter a valid number between {} and {}."
    };
}

pub fn prompt(message: &str) -> Result<String> {
    let mut answer = String::new();
    println!("{}", message);
    print!("> ");
    io::stdout().flush()?;
    let mut prompt_result: io::Result<usize> = io::stdin().read_line(&mut answer);
    while prompt_result.is_err() {
        println!("Sorry, I didn't quite catch that! Try again.");
        print!("> ");
        prompt_result = io::stdin().read_line(&mut answer);
    }
    match prompt_result {
        Ok(_) => Ok(answer),
        Err(err) => panic!("{}", err),
    }
}

pub fn prompt_select_bounded_number(message: &str, min: u8, max: u8) -> Result<u8> {
    let selection_str = prompt(message)?;
    let parsed_number_result = u8::from_str(selection_str.trim());
    if let Err(_) = parsed_number_result {
        println!(invalid_number_msg!(), min, max);
        return prompt_select_bounded_number(message, min, max);
    }
    let parsed_number = parsed_number_result.unwrap();
    if parsed_number < min || parsed_number > max {
        println!(invalid_number_msg!(), min, max);
        return prompt_select_bounded_number(message, min, max);
    }
    return Ok(parsed_number);
}

pub fn prompt_select_from_numbered_list<'a, T>(
    message: &str,
    options: &'a Vec<T>,
    captions: &Vec<&str>,
) -> Result<&'a T> {
    let mut prompt_text = String::from(message);

    for (i, caption) in captions.iter().enumerate() {
        prompt_text = format!("{}\n{}) {}", prompt_text, i + 1, caption);
    }

    let action = prompt_select_bounded_number(&prompt_text, 1, options.len() as u8)?;
    match options.get(action as usize - 1) {
        Some(option) => Ok(option),
        None => Err(Box::new(Error::new(&format!("Invalid choice {}!", action)))),
    }
}
