// handles cli arg parsing

const HELP: &str = "\
App

USAGE:
  app [OPTIONS] --number NUMBER [INPUT]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --number NUMBER       Sets a number
  --opt-number NUMBER   Sets an optional number
  --width WIDTH         Sets width [default: 10]
  --output PATH         Sets an output path

ARGS:
  <INPUT>
";

#[derive(Debug, Clone)]
pub struct AppArgs {
    pub port: Option<u32>,
    pub dir: Option<std::path::PathBuf>,
    pub configpath: Option<std::path::PathBuf>,
}

pub fn parse() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        // gets config path
        configpath: pargs.opt_value_from_os_str("-c", parse_path)?,

        // get port and dir
        port: pargs.opt_free_from_str()?,
        dir: pargs.opt_free_from_str()?,
    };

    Ok(args)
}

// parses string into pathbufcx
fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}
