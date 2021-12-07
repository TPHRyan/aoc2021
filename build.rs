use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let solution_dir = Path::new(&manifest_dir)
        .join("src")
        .join("aoc")
        .join("solution");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir_path = Path::new(&out_dir);

    let solution_modules = fetch_solution_modules(solution_dir.as_path(), "day_")?;
    fs::write(
        out_dir_path.join("solution_modules.in"),
        generate_solution_module_declarations(&solution_modules)?,
    )?;
    fs::write(
        out_dir_path.join("solutions_implemented.in"),
        generate_solution_fn_getter(&solution_modules)?,
    )?;
    Ok(())
}

fn fetch_solution_modules(
    solution_path: &Path,
    module_prefix: &str,
) -> std::io::Result<Vec<(String, String)>> {
    let mut solution_modules: Vec<(String, String)> = Vec::new();
    for entry_result in fs::read_dir(solution_path)? {
        let entry = entry_result?;
        let file_name = String::from(entry.file_name().to_str().unwrap());
        let entry_path = entry.path();
        let file_path = String::from(entry_path.to_str().unwrap());
        if entry_path.is_file() && file_name.starts_with(module_prefix) {
            println!("cargo:rerun-if-changed={}", file_path);
            solution_modules.push((file_name.replace(".rs", ""), file_path));
        }
    }
    Ok(solution_modules)
}

fn generate_solution_module_declarations(
    solution_modules: &Vec<(String, String)>,
) -> std::io::Result<String> {
    let declaration_vec: Vec<String> = solution_modules
        .iter()
        .map(|(module_name, module_path)| {
            format!("#[path = \"{}\"]\nmod {};", module_path, module_name)
        })
        .collect();
    Ok(format!("{}\n", declaration_vec.join("\n")))
}

fn generate_solution_fn_getter(
    solution_modules: &Vec<(String, String)>,
) -> std::io::Result<String> {
    let signature = "|day, part|";
    let day_arms: Vec<String> = solution_modules
        .iter()
        .map(|(day_mod, _)| {
            let day_number = day_mod.replace("day_", "");
            format!(
                "\t\t{} => match part {{\n{}\n\t\t}},",
                day_number,
                format!(
                    "\t\t\t1 => Some({}::solve_part_1),\n\t\t\t2 => Some({}::solve_part_2),\n\t\t\t_ => None,",
                    day_mod, day_mod
                )
            )
        })
        .collect();
    let body = format!(
        "\tmatch day {{\n{}\n\t}}",
        format!("{}\n\t\t_ => None,", day_arms.join("\n"))
    );
    Ok(format!("{} {{\n{}\n}}", signature, body))
}
