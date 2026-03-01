use std::{
    ffi::OsStr,
    fmt::{Debug, Display},
    process::Command,
};

#[derive(Debug)]
pub enum LunaraError {
    MissingCmd,
    CommandFailed(String),
}

pub type LunaraResult<T> = std::result::Result<T, LunaraError>;

pub fn run<C, I, A>(cmd: C, args: I) -> LunaraResult<String>
where
    C: AsRef<OsStr> + Display + Copy,
    I: IntoIterator<Item = A> + Debug + Copy,
    A: AsRef<OsStr>,
{
    let out = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| LunaraError::CommandFailed(format!("failed to run {cmd}: {e}")))?;

    if !out.status.success() {
        return Err(LunaraError::CommandFailed(format!(
            "{cmd} {:?} exited with {:?}. stderr={}",
            args,
            out.status.code(),
            String::from_utf8_lossy(&out.stderr)
        )));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

pub fn run_optional<C, I, A>(cmd: C, args: I) -> Option<String>
where
    C: AsRef<std::ffi::OsStr> + std::fmt::Display + Copy,
    I: IntoIterator<Item = A> + std::fmt::Debug + Copy,
    A: AsRef<std::ffi::OsStr>,
{
    match run(cmd, args) {
        Ok(output) => Some(output),
        Err(e) => {
            eprintln!("Command failed: {:?}", e);
            None
        }
    }
}

pub fn run_no_output<C, I, A>(cmd: C, args: I) -> LunaraResult<()>
where
    C: AsRef<OsStr> + Display + Copy,
    I: IntoIterator<Item = A> + Debug + Copy,
    A: AsRef<OsStr>,
{
    match run(cmd, args) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
