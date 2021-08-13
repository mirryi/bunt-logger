use bunt_logger::{debug, error, info, trace, warn, ColorChoice, Level};

fn main() {
    bunt_logger::with()
        .level(Level::Trace)
        .stderr(ColorChoice::Always);

    error!("{$red+bold}A red and bold error message!{/$}");
    warn!("{$yellow}A yellow warning message!{/$}");
    info!("{$green}A green info message!{/$}");
    debug!("{$cyan}A cyan debug message!{/$}");
    trace!("{$white+dimmed}A white and dimmed trace message!{/$}");
}
