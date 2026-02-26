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

// pub fn run(cmd: &str, args: &[&str]) -> LunaraResult<String> {
//     let out = Command::new(cmd)
//         .args(args)
//         .output()
//         .map_err(|e| LunaraError::CommandFailed(format!("failed to run {cmd}: {e}")))?;
//
//     if !out.status.success() {
//         return Err(LunaraError::CommandFailed(format!(
//             "{cmd} {:?} exited with {:?}. stderr={}",
//             args,
//             out.status.code(),
//             String::from_utf8_lossy(&out.stderr)
//         )));
//     }
//
//     Ok(String::from_utf8_lossy(&out.stdout).to_string())
// }
//
pub fn run_no_output<C, I, A>(cmd: C, args: I) -> Result<(), LunaraError>
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
    Ok(())
}
