mod aoc;
mod common;
mod prompt;
mod sub;

use std::env::Args;
use std::iter::Peekable;
use std::str::FromStr;

use common::AppParams;
use common::Error;
use common::Result;
use prompt::prompt_select_from_numbered_list;

enum ChallengeOption {
    DisplayExampleData,
    DisplayRealData,
    RunSolution,
}

pub fn run(app_params: AppParams) -> Result<()> {
    let challenge_day: u8 = match app_params.challenge_day {
        Some(day) => day as u8,
        None => aoc::prompt_select_challenge_day()?,
    };
    let challenge = aoc::get_challenge(challenge_day)?;
    println!("Loaded challenge for day {}.", challenge.day);
    let challenge_part: u8 = if challenge.parts > 1 {
        match app_params.challenge_part {
            Some(part) => part as u8,
            None => aoc::prompt_select_challenge_part(challenge.parts)?,
        }
    } else {
        1
    };

    let challenge_solution = aoc::get_challenge_solution(challenge.day, challenge_part);

    let mut options: Vec<ChallengeOption> = vec![
        ChallengeOption::DisplayExampleData,
        ChallengeOption::DisplayRealData,
    ];
    let mut captions: Vec<&str> = vec!["Show challenge example data", "Show challenge puzzle data"];
    if challenge_solution.is_some() {
        options.insert(0, ChallengeOption::RunSolution);
        captions.insert(0, "Run challenge solution");
    }
    let challenge_option =
        prompt_select_from_numbered_list("What do you want to do?", &options, &captions)?;

    match challenge_option {
        ChallengeOption::DisplayExampleData => {
            println!("Challenge example data:\n{}", challenge.example_data)
        }
        ChallengeOption::DisplayRealData => println!("Challenge data:\n{}", challenge.data),
        ChallengeOption::RunSolution => {
            return match challenge_solution {
                Some(solution) => solution.run(challenge, app_params),
                None => Err(Box::new(Error::new(
                    "Runtime error: Challenge solution not found!",
                ))),
            }
        }
    }
    Ok(())
}

pub fn parse_args(args: Args) -> Result<AppParams> {
    let mut args = args.peekable();
    let program_name = args.next().unwrap();

    let mut use_example_data = false;
    let mut challenge_day: Option<u32> = None;
    let mut challenge_part: Option<u32> = None;

    while let Some(next_arg) = args.peek() {
        match next_arg.as_str() {
            "--day" => challenge_day = Some(consume_u32_option(&mut args)?),
            "--part" => challenge_part = Some(consume_u32_option(&mut args)?),
            "--use-example-data" => use_example_data = consume_boolean_option(&mut args),
            s => return Err(Box::new(Error::new(&format!("Unrecognized option {}!", s)))),
        };
    }

    Ok(AppParams {
        program_name,
        use_example_data,
        challenge_day,
        challenge_part,
    })
}

fn consume_boolean_option(args: &mut Peekable<Args>) -> bool {
    args.next();
    true
}

fn consume_string_option(args: &mut Peekable<Args>) -> String {
    args.next();
    match args.next() {
        Some(s) => s,
        None => String::new(),
    }
}

fn consume_u32_option(args: &mut Peekable<Args>) -> Result<u32> {
    let str_val = consume_string_option(args);
    let u32_val = u32::from_str(&str_val)?;
    Ok(u32_val)
}
