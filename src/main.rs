mod prelude;
mod util;

use crate::prelude::*;

fn main() -> Result<(), Error> {
    let start = std::time::Instant::now();
    let args = Opt::try_parse()?;
    let files: Vec<_> = glob(&args.input)?
        .filter_map(Result::ok)
        .filter(|p| p.is_file())
        .collect();
    let total_count = files.len();
    let mut processed_count: usize = 0;
    let base_path = std::fs::canonicalize(glob_base(&args.input)?)?;

    println!("[ i ] Using base path as: `{}`.", base_path.display());

    for file in files.iter() {
        let abs_path = std::fs::canonicalize(file)?;

        println!("[ i ] Proessing: `{}`...", abs_path.display());

        let start_path_str = abs_path.display().to_string();
        let output: PathBuf;

        let mut processed_iteration = |output: &PathBuf| {
            println!(
                "[ {:.03} ] Processed: `{}` => `{}`",
                start.elapsed().as_secs_f64(),
                start_path_str,
                output.display()
            );
            processed_count += 1;
        };

        let dest = &base_path.join(
            abs_path
                .file_name()
                .ok_or(anyhow!("Expected a filename for `{}`", start_path_str))?,
        );

        // NOTE: Dry run won't do the duplicate logic, but the duplicate logic is something that's
        // well defined and can be tested separately.
        if args.dry_run {
            println!(
                "[ i ] Dry run - Processed: {} => {}",
                &abs_path.display(),
                dest.display()
            );
            processed_count += 1;
            continue;
        }

        let res = std::fs::rename(&abs_path, dest);

        match res {
            Ok(()) => processed_iteration(dest),
            Err(e) => {
                use DuplicateAction as DA;

                match args.on_duplicate {
                    DA::Abort => return Err(anyhow!(e)),
                    DA::Rename => {
                        // TODO: Let the user define their own renaming strategy.
                        output = handle_duplicate_renaming(&base_path, &args.duplicate_format)?;
                        processed_iteration(&output);
                        continue;
                    }
                    DA::Skip => {
                        continue;
                    }
                }
            }
        }
    }

    println!(
        "Processed {}/{} ({:.02}%) files in {:.03} seconds.",
        processed_count,
        processed_count,
        processed_count.checked_div(total_count).unwrap_or(0) * 100,
        start.elapsed().as_secs_f64()
    );

    Ok(())
}
