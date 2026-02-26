use std::process::Command;

use crate::{
    models::app_entry::AppEntry,
    system::command::{run_no_output, LunaraError, LunaraResult},
};

pub fn is_running(app: &AppEntry) -> bool {
    let Some(wc) = app.wm_class.as_ref() else {
        return false;
    };

    let out = Command::new("xdotool")
        .args(["search", "--class", wc])
        .output();

    match out {
        Ok(o) => o.status.success() && !o.stdout.is_empty(),
        Err(_) => false,
    }
}

pub fn focus_lunara() -> LunaraResult<()> {
    run_no_output("bspc", &["desktop", "-f", "^1"])
}

pub fn focus_app(app: &AppEntry) -> LunaraResult<bool> {
    let Some(did) = app.bspwm_desktop.as_ref() else {
        return Ok(false);
    };
    run_no_output("bspc", &["desktop", "-f", did])?;
    Ok(true)
}

pub fn run_app(app: &AppEntry) -> Result<(), LunaraError> {
    if is_running(app) {
        return Ok(());
    }
    if app.cmd.is_empty() {
        return Err(LunaraError::MissingCmd);
    }
    let program = &app.cmd[0];
    let args = &app.cmd[1..];

    Command::new(program)
        .args(args)
        .spawn()
        .map(|_| ())
        .map_err(|e| LunaraError::CommandFailed(format!("spawn failed: {e}")))
}

pub fn launch_or_focus(app: &AppEntry) -> Result<(), LunaraError> {
    if is_running(app) {
        let _ = focus_app(app);
        return Ok(());
    }

    if let (Some(wm_class), Some(desk)) = (app.wm_class.as_deref(), app.bspwm_desktop.as_deref()) {
        let del_class = format!("{wm_class}:*:*");
        run_no_output("bspc", &["rule", "-r", del_class.as_str()])?;
        let rule = format!("desktop={desk} follow=on");
        run_no_output("bspc", &["rule", "-a", wm_class, "-o", rule.as_str()])?;
    }

    let _ = run_app(app);
    let _ = focus_app(app);
    Ok(())
}
