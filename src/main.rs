use sha2::{Digest, Sha256};
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 && (args[1].ends_with(".rs")) {
        // called by binfmt, right? must set 'P'(preserve arg0) in binfmt config
        // somehow kernel doesn't always pass the absolute path
        binfmt_handler(&args[1..])
    } else {
        cli_handler()
    }
}

fn binfmt_handler(args: &[String]) -> std::io::Result<()> {
    let (head, actual_args) = args.split_at(2);
    let program_source_path = &head[0];
    let program_execution_name = &head[1];

    let real_program =
        compile_program_cached(program_source_path).expect("Failed to compile the program.");

    let real_program_result = Command::new(real_program)
        .arg0(program_execution_name)
        .args(actual_args)
        .exec();

    Err(real_program_result)
}

/// Compile the given source file.
///
/// The cache uses path as key.
fn compile_program_cached(program_source_path: &String) -> std::io::Result<String> {
    let hash = Sha256::new().chain_update(program_source_path).finalize();
    let hex_hash = base16ct::lower::encode_string(&hash);

    let mut cache = env::home_dir().unwrap_or(PathBuf::from("."));
    cache.push(".cache");
    cache.push("rs-run-binaries");
    cache.push(&hex_hash[0..2]);
    cache.push(&hex_hash[2..4]);

    // cache folder contains 3 levels, l1, l2, and binary(l3)
    // create the first 2 levels(directories)
    fs::create_dir_all(&cache)?;

    cache.push(&hex_hash[4..]);
    let cache_file_string: String = cache.to_string_lossy().into();

    if cache.is_file() {
        let cache_modified_time = fs::metadata(&cache)?.modified()?;
        let code_modified_time = fs::metadata(program_source_path)?.modified()?;

        if cache_modified_time < code_modified_time {
            // rebuild!
            compile_program(program_source_path, &cache_file_string)?;
        }
    } else {
        // build!
        compile_program(program_source_path, &cache_file_string)?;
    }

    Ok(cache_file_string)
}

fn compile_program(code: &String, binary: &String) -> std::io::Result<()> {
    let mut args: Vec<&str> = Vec::new();
    args.push("-o");
    args.push(binary.as_str());
    args.push(code.as_str());

    let compiler_result = Command::new("rustc").args(args).status()?;

    if compiler_result.success() {
        Ok(())
    } else {
        // FIXME: but how?
        panic!("`rustc' execution failed! Please check the compiler output above!")
    }
}

/// TODO: implement cli interface
fn cli_handler() -> std::io::Result<()> {
    let text = "RS-RUN NOTICE

Hi! The command line interface is yet to be implemented.

You can use the binfmt mode, by writing the config in `/etc/binfmt.d/rs-run.conf`:
    `:rs-run:E::rs::/path/to/rs-run:P`
and then restart `systemd-binfmt.service`.
Then you can execute the `.rs` file if it's given the execution permission.

To clean the cache, remove the folder `$HOME/.cache/rs-run-binaries`.
";
    print!("{}", text);
    Ok(())
}
