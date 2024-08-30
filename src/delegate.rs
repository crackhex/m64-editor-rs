use crate::{AppState, OPEN_FILE, SET_OUTPUT_TEXT};
use druid::{commands, AppDelegate, Command, DelegateCtx, Env, Event, Handled, LensExt, Target, WindowId};
use druid_shell::RawMods::Ctrl;
use druid_shell::{HotKey, KbKey};

pub struct Delegate {
    main_window: Option<WindowId>,
}

impl Delegate {
    pub fn new() -> Self {
        Self {
            main_window: None,
        }
    }

    fn show_main(&mut self, ctx: &mut DelegateCtx) {
        match self.main_window {
            Some(id) => {
                ctx.submit_command(commands::SHOW_WINDOW.to(id));
            }
            None => {

            }
        }
    }

    fn close_all_windows(&mut self, ctx: &mut DelegateCtx) {
        ctx.submit_command(commands::CLOSE_ALL_WINDOWS);
        self.main_window = None;
    }

}

impl AppDelegate<String> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut String,
        _env: &Env,
    ) -> Handled {
        if let Some(_) = cmd.get(SET_OUTPUT_TEXT) {
            println!("Show window");
            self.show_main(ctx);
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(commands::CLOSE_ALL_WINDOWS) {
            println!("Show window");
            self.close_all_windows(ctx);
            return Handled::Yes;
        }
        Handled::No
    }
}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        ctx: &mut DelegateCtx,
        _window_id: WindowId,
        event: Event,
        _data: &mut AppState,
        _env: &Env,
    ) -> Option<Event> {

        if let Event::KeyDown(key_event) = &event {
            println!("Key down: {:?}", key_event.key);
            if key_event.key == KbKey::Escape {
                ctx.submit_command(commands::CLOSE_ALL_WINDOWS);
                return None;
            }
            let open_file = HotKey::new(Ctrl, KbKey::Character("o".to_string()));
            if open_file.matches(key_event) {
                println!("Open file");
                ctx.submit_command(OPEN_FILE);
                return None;
            }
        }

        Some(event)
    }
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if let Some(info) = cmd.get(SET_OUTPUT_TEXT) {
            data.output_m64 = info.path().to_str().unwrap().to_string();
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(commands::SHOW_WINDOW) {
            println!("Show window");
            self.show_main(ctx);
            return Handled::Yes;
        }
        if let Some(_) = cmd.get(commands::CLOSE_ALL_WINDOWS) {
            println!("Show window");
            self.close_all_windows(ctx);
            return Handled::Yes;
        }
        Handled::No

    }
}