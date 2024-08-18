use std::{
    io::{self, Stdout, Write},
    process::Command,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, poll, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};

use crate::errors::{AppError, AppResult};

use super::ModManager;

pub struct Terminal<'a> {
    mod_manager: &'a mut ModManager,
    selected_index: usize,
}

impl<'a> Terminal<'a> {
    pub fn new(mod_manager: &'a mut ModManager) -> Self {
        Terminal {
            mod_manager,
            selected_index: 0,
        }
    }

    pub fn run(&mut self) -> AppResult<()> {
        let mut stdout = io::stdout();

        execute!(stdout, cursor::SavePosition)?;
        execute!(stdout, terminal::EnterAlternateScreen)?;

        terminal::enable_raw_mode()?;

        self.main_loop(&mut stdout)?;

        terminal::disable_raw_mode()?;

        execute!(stdout, terminal::LeaveAlternateScreen)?;
        execute!(stdout, cursor::RestorePosition)?;

        Ok(())
    }

    fn render(&self, stdout: &mut Stdout) -> AppResult<()> {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        execute!(stdout, crossterm::cursor::Hide)?;

        let mut top_offset = 0;

        execute!(
            stdout,
            SetForegroundColor(Color::Cyan),
            cursor::MoveTo(0, top_offset),
            Print("Arma 3 Mod Manager CLI"),
            SetForegroundColor(Color::Reset)
        )?;

        top_offset += 2;

        execute!(
            stdout,
            cursor::MoveTo(0, top_offset),
            SetForegroundColor(Color::Cyan),
            Print("Navigate with <WASD>, <HJKL> or <ARROW KEYS>"),
            SetForegroundColor(Color::Reset)
        )?;

        top_offset += 1;

        execute!(
            stdout,
            cursor::MoveTo(0, top_offset),
            SetForegroundColor(Color::Cyan),
            Print("Enable Mod: <SPACE> | Enable All: <CTRL> + <SPACE>"),
            SetForegroundColor(Color::Reset)
        )?;

        top_offset += 1;

        execute!(
            stdout,
            cursor::MoveTo(0, top_offset),
            SetForegroundColor(Color::Cyan),
            Print("Refresh Mods: R"),
            SetForegroundColor(Color::Reset)
        )?;

        top_offset += 1;
        execute!(
            stdout,
            cursor::MoveTo(0, top_offset),
            SetForegroundColor(Color::Cyan),
            Print("Start Game: P"),
            SetForegroundColor(Color::Reset)
        )?;

        top_offset += 1;
        execute!(
            stdout,
            cursor::MoveTo(0, top_offset),
            SetForegroundColor(Color::Cyan),
            Print("Quit Manager: ESC"),
            SetForegroundColor(Color::Reset)
        )?;

        top_offset += 2;

        let enabled_mods = self.mod_manager.loaded_mods.filter(|m| m.enabled).len();
        let total_mods = self.mod_manager.loaded_mods.all_items().len();

        let page_number = self.mod_manager.loaded_mods.current_page + 1;
        let total_pages = self.mod_manager.loaded_mods.total_pages();

        execute!(
            stdout,
            cursor::MoveTo(0, top_offset),
            SetForegroundColor(Color::White),
            Print(&format!(
                "Mods: {}/{}{}Page: {}/{}",
                enabled_mods,
                total_mods,
                " ".repeat(25),
                page_number,
                total_pages
            )),
            SetForegroundColor(Color::Reset),
        )?;

        top_offset += 2;

        for (i, m) in self
            .mod_manager
            .loaded_mods
            .current_page_items()
            .iter()
            .enumerate()
        {
            let mut str: String = String::new();

            let cursor = if i == self.selected_index {
                " > "
            } else {
                "   "
            };

            execute!(
                stdout,
                cursor::MoveTo(0, top_offset),
                SetForegroundColor(Color::Red),
                Print(cursor),
                SetForegroundColor(Color::Reset)
            )?;

            let mut color = Color::Grey;

            if m.enabled {
                color = Color::White;
                str += "[X]";
            } else {
                str += "[ ]";
            }

            str += &format!(" {}", m.name);

            execute!(
                stdout,
                cursor::MoveTo(3, top_offset),
                SetForegroundColor(color),
                Print(str),
                SetForegroundColor(Color::Reset)
            )?;
            top_offset += 1;
        }

        // Show pagination direction

        if page_number < total_pages {
            execute!(stdout, cursor::MoveTo(23, top_offset), Print("-->"),)?;
        }

        if page_number > 1 {
            execute!(stdout, cursor::MoveTo(3, top_offset), Print("<--"),)?;
        }

        top_offset += 1;

        execute!(
            stdout,
            cursor::MoveTo(0, top_offset),
            Print("For more information visit: github.com/viktorholk/arma3-mod-manager-cli")
        )?;

        stdout.flush()?;

        Ok(())
    }

    fn main_loop(&mut self, stdout: &mut Stdout) -> AppResult<()> {
        self.render(stdout)?;
        stdout.flush()?;

        loop {
            if poll(Duration::from_millis(1000))? {
                match event::read()? {
                    Event::Key(event) => match event.code {
                        KeyCode::Char('w') | KeyCode::Char('k') | KeyCode::Up => {
                            if self.selected_index > 0 {
                                self.selected_index -= 1;
                            }
                        }
                        KeyCode::Char('s') | KeyCode::Char('j') | KeyCode::Down => {
                            let length = self.mod_manager.loaded_mods.current_page_items().len();

                            if self.selected_index < length - 1 {
                                self.selected_index += 1;
                            }
                        }

                        KeyCode::Char('a') | KeyCode::Char('h') | KeyCode::Left => {
                            self.mod_manager.loaded_mods.prev_page();
                            self.selected_index = 0;
                        }

                        KeyCode::Char('d') | KeyCode::Char('l') | KeyCode::Right => {
                            self.mod_manager.loaded_mods.next_page();
                            self.selected_index = 0;
                        }

                        KeyCode::Char(' ') if event.modifiers == KeyModifiers::CONTROL => {
                            let value = if self
                                .mod_manager
                                .loaded_mods
                                .all_items()
                                .iter()
                                .all(|m| m.enabled)
                            {
                                false
                            } else {
                                true
                            };

                            self.mod_manager
                                .loaded_mods
                                .all_items_mut()
                                .iter_mut()
                                .for_each(|m| m.enabled = value);
                        }

                        KeyCode::Char(' ') => {
                            let current_page = self.mod_manager.loaded_mods.current_page;
                            let page_size = self.mod_manager.loaded_mods.page_size;
                            let index = self.selected_index + (current_page * page_size);

                            let selected_mod =
                                &mut self.mod_manager.loaded_mods.all_items_mut()[index];
                            selected_mod.enabled = !selected_mod.enabled;
                        }

                        KeyCode::Char('r') => {
                            self.mod_manager.refresh_mods()?;
                        }
                        KeyCode::Char('p') => {
                            self.start_game()?;
                        }

                        KeyCode::Esc => break,

                        _ => continue,
                    },

                    _ => continue,
                }
                self.render(stdout)?;
                stdout.flush()?;
            }
        }

        Ok(())
    }

    fn start_game(&mut self) -> AppResult<()> {
        let enabled_mods = self.mod_manager.loaded_mods.filter(|m| m.enabled);
        let game_path = self.mod_manager.config.get_game_path();
        let workshop_path = self.mod_manager.config.get_workshop_path();

        let game_app_path = game_path.join("arma3.app");
        let game_app_path_str = game_app_path.to_string_lossy().to_string();

        if !game_app_path.exists() {
            return Err(AppError::InvalidPath(game_app_path_str.to_owned()));
        }

        let mut command = Command::new("open");

        command.args(["-a", &game_app_path_str]);

        // Remove existing symlinks from the game directory
        super::file_handler::remove_dir_symlinks(game_path)?;

        if !enabled_mods.is_empty() {
            let mod_paths = enabled_mods
                .iter()
                .map(|m| m.get_path(workshop_path))
                .collect::<Vec<_>>();

            super::file_handler::create_sym_links(game_path, mod_paths)?;

            // Save the enabled mods so it loads next time
            self.mod_manager
                .config
                .update_mods(enabled_mods.iter().map(|m| m.id).collect());
            self.mod_manager.config.save()?;

            // Build args
            command.args([
                "--args",
                &format!(
                    "-mod={}",
                    enabled_mods
                        .iter()
                        .map(|m| m.id.to_string())
                        .collect::<Vec<String>>()
                        .join(";")
                ),
            ]);
        }

        command.output()?;

        Ok(())
    }
}
