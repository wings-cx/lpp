use std::error::Error;

use lalrpop::Configuration;

fn main() -> Result<(), Box<dyn Error>> {
    Configuration::new()
        .always_use_colors()
        .log_debug()
        .process_current_dir()?;

    Ok(())
}
