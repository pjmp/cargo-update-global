use anyhow::Result;

mod cli;
mod ops;

#[tokio::main]
async fn main() -> Result<()> {
    let app = cli::new().get_matches();

    let cmd = match app.subcommand {
        None => {
            let _ = cli::new().print_long_help();
            std::process::exit(1);
        }
        Some(sub_cmd) => sub_cmd.matches,
    };

    // if we have more that two flags, we need to change this
    if cmd.is_present("list") || !cmd.is_present("list") && !cmd.is_present("update") {
        let container = ops::get_upgradable_crates().await?;
        ops::pretty_print_stats(container);
    }

    if cmd.is_present("update") {
        ops::update_upgradable_crates().await?;
    }

    Ok(())
}
