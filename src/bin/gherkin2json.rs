use std::io;
use std::process::ExitCode;

use rs_gherkin_feature2json::stdin2feature2json2stdout_default;

fn io_main() -> impl FnMut() -> Result<(), io::Error> {
    stdin2feature2json2stdout_default()
}

fn sub() -> Result<(), io::Error> {
    io_main()()
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
