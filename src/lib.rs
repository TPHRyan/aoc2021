mod aoc;
mod common;
mod prompt;
mod sub;

use std::env;

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
    let challenge = aoc::get_challenge(aoc::prompt_select_challenge_day()?)?;
    println!("Loaded challenge for day {}.", challenge.day);
    let challenge_part: u8 = if challenge.parts > 1 {
        aoc::prompt_select_challenge_part(challenge.parts)?
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
