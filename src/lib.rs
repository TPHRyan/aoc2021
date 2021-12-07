mod aoc;
mod common;
mod sub;

use std::env;
use std::io;
use std::io::Write;
use std::str::FromStr;

use common::AppParams;
use common::Error;
use common::Result;

macro_rules! invalid_number_msg {
    () => {
        "Please enter a valid number between {} and {}."
    };
}

pub fn run(app_params: AppParams) -> Result<()> {
    let challenge = aoc::get_challenge(prompt_select_challenge_day()?)?;
    println!("Loaded challenge for day {}.", challenge.day);
    let challenge_part: u8 = if challenge.parts > 1 {
        prompt_select_challenge_part(challenge.parts)?
    } else {
        1
    };

    let challenge_solution = aoc::get_challenge_solution(challenge.day, challenge_part);

    let use_option_3 = challenge_solution.is_some();
    let action_str = prompt(&format!(
        "{}\n{}\n{}\n{}",
        "What do you want to do?",
        "1) Show challenge example data",
        "2) Show challenge puzzle data",
        if use_option_3 {
            "3) Run challenge solution"
        } else {
            ""
        }
    ))?;

    match action_str.trim() {
        "1" => println!("Challenge example data:\n{}", challenge.example_data),
        "2" => println!("Challenge data:\n{}", challenge.data),
        "3" => {
            return match challenge_solution {
                Some(solution) => solution.run(challenge, app_params),
                None => Err(Box::new(Error::new(&format!(
                    "Invalid choice {}!",
                    action_str
                )))),
            }
        }
        _ => {
            return Err(Box::new(Error::new(&format!(
                "Invalid choice {}!",
                action_str
            ))))
        }
    }
    Ok(())
}

fn prompt_select_challenge_day() -> Result<u8> {
    return prompt_select_bounded_number("Please select a challenge. (1-25)", 1, 25);
}

fn prompt_select_challenge_part(num_parts: u8) -> Result<u8> {
    return prompt_select_bounded_number(
        &format!("Which challenge part? (1-{})", num_parts),
        1,
        num_parts,
    );
}

fn prompt_select_bounded_number(message: &str, min: u8, max: u8) -> Result<u8> {
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

fn prompt(message: &str) -> Result<String> {
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

pub fn parse_args(mut args: env::Args) -> Result<AppParams> {
    let program_name = args.next().unwrap();
    let mut use_example_data = false;

    while let Some(next_arg) = args.next() {
        match next_arg.as_str() {
            "--use-example-data" => use_example_data = true,
            _ => {}
        }
    }

    Ok(AppParams {
        program_name,
        use_example_data,
    })
}
