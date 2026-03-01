use std::process::Command;

use crate::{
    models::app_entry::AppEntry,
    system::{
        command::{run, run_no_output, LunaraError, LunaraResult},
        window_manager_controller::WindowManagerController,
    },
};

pub struct BspwmWMController {
    last_focused_desktop: Option<String>,
}

impl WindowManagerController for BspwmWMController {
    fn new(_app_handle: tauri::AppHandle) -> Self {
        return BspwmWMController {
            last_focused_desktop: None,
        };
    }

    fn is_running(&mut self, app: &AppEntry) -> bool {
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

    fn focus_lunara(&mut self) -> LunaraResult<()> {
        self.last_focused_desktop = BspwmWMController::get_current_dekstop();
        run_no_output("bspc", &["desktop", "-f", "^1"])
    }

    fn focus_last_app(&mut self) -> LunaraResult<()> {
        if let Some(d) = &self.last_focused_desktop {
            run_no_output("bspc", &["desktop", "-f", d.as_str()])
        } else {
            Ok(())
        }
    }

    fn launch_or_focus(&mut self, app: &AppEntry) -> LunaraResult<()> {
        if self.is_running(app) {
            let _ = BspwmWMController::focus_app(app);
            return Ok(());
        }

        if let (Some(wm_class), Some(desk)) =
            (app.wm_class.as_deref(), app.bspwm_desktop.as_deref())
        {
            let del_class = format!("{wm_class}:*:*");
            run_no_output("bspc", &["rule", "-r", del_class.as_str()])?;
            let rule = format!("desktop={desk} follow=on");
            run_no_output("bspc", &["rule", "-a", wm_class, "-o", rule.as_str()])?;
        }

        self.run_app(app)?;
        let _ = BspwmWMController::focus_app(app)?;
        Ok(())
    }
}

impl BspwmWMController {
    fn get_current_dekstop() -> Option<String> {
        run("bspc", &["query", "--desktops", "-d", "focused", "--names"])
            .ok()
            .map(|v| v.trim().to_string())
    }

    fn focus_app(app: &AppEntry) -> LunaraResult<bool> {
        let Some(did) = app.bspwm_desktop.as_ref() else {
            return Ok(false);
        };
        run_no_output("bspc", &["desktop", "-f", did])?;
        Ok(true)
    }

    fn run_app(&mut self, app: &AppEntry) -> LunaraResult<()> {
        if self.is_running(app) {
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
}
