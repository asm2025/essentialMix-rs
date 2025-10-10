use crossterm::{
    ExecutableCommand, cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use dialoguer::{Select, theme::ColorfulTheme};
use std::{
    io::{Write, stdin, stdout},
    str::FromStr,
    thread,
};
use tokio::sync::mpsc::{self, error::TryRecvError};

use crate::{Result, error::emError};

#[derive(Debug)]
pub struct KeyListener {
    rx: mpsc::Receiver<KeyEvent>,
    _handle: thread::JoinHandle<()>,
}

impl KeyListener {
    pub fn new() -> Result<Self> {
        Self::bounded(1)
    }

    pub fn bounded(buffer_size: usize) -> Result<Self> {
        let (tx, rx) = mpsc::channel(buffer_size);

        let handle = thread::spawn(move || {
            if enable_raw_mode().is_err() {
                return;
            }

            loop {
                if let Ok(Event::Key(key)) = event::read() {
                    if !key.is_press() {
                        continue;
                    }

                    if tx.blocking_send(key).is_err() {
                        break;
                    }
                }
            }

            let _ = disable_raw_mode();
        });

        Ok(KeyListener {
            rx,
            _handle: handle,
        })
    }

    pub fn receiver(&self) -> &mpsc::Receiver<KeyEvent> {
        &self.rx
    }

    pub async fn recv(&mut self) -> Option<KeyEvent> {
        self.rx.recv().await
    }

    pub fn try_recv(&mut self) -> std::result::Result<KeyEvent, TryRecvError> {
        self.rx.try_recv()
    }
}

impl Drop for KeyListener {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

pub fn clear_screen() -> Result<()> {
    let mut stdout = stdout();
    stdout
        .execute(Clear(ClearType::All))?
        .execute(cursor::MoveTo(0, 0))?;
    Ok(())
}

pub fn display_menu(items: &[&str], prompt: Option<&str>) -> Result<usize> {
    clear_screen()?;

    let prompt = match prompt {
        Some(s) if !s.is_empty() => s,
        _ => "Please select an option",
    };
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .default(0)
        .interact()
        .map_err(|e| emError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    Ok(if selection == items.len() - 1 {
        0
    } else {
        selection + 1
    })
}

pub fn get(prompt: Option<&str>) -> Result<String> {
    print_prompt(prompt);

    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    if !buffer.is_empty() {
        // Remove the trailing newlines
        buffer.pop();
    }

    Ok(buffer)
}

pub fn get_str(prompt: Option<&str>) -> Result<String> {
    let input = get(prompt)?;

    if input.is_empty() {
        return Err(emError::NoInput);
    }

    Ok(input)
}

pub fn get_char(prompt: Option<&str>) -> Result<char> {
    print_prompt(prompt);
    // Enable raw mode to read single characters
    enable_raw_mode()?;

    let result = loop {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Char(c) => break Ok(c),
                KeyCode::Esc | KeyCode::Enter => break Err(emError::NoInput),
                _ => continue,
            }
        }
    };

    // Disable raw mode before returning
    disable_raw_mode()?;
    result
}

pub fn get_numeric<T: FromStr>(prompt: Option<&str>) -> Result<T>
where
    <T as FromStr>::Err: std::fmt::Display,
    T::Err: std::error::Error + 'static,
{
    let input = get_str(prompt)?;
    let n = input.parse::<T>().map_err(|e| {
        emError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            e.to_string(),
        ))
    })?;
    Ok(n)
}

pub fn get_password(prompt: Option<&str>) -> Result<String> {
    print_prompt(prompt);

    let input = rpassword::read_password()?;
    Ok(input)
}

pub fn get_password_str(prompt: Option<&str>) -> Result<String> {
    let input = get_password(prompt)?;

    if input.is_empty() {
        return Err(emError::NoInput);
    }

    Ok(input)
}

pub fn confirm(prompt: Option<&str>) -> Result<bool> {
    let input = get_char(prompt)?;

    match input {
        'y' | 'Y' => Ok(true),
        _ => Err(emError::NoInput),
    }
}

pub fn pause() {
    println!("Press any key to continue...");
    let _ = get_char(None);
}

fn print_prompt(prompt: Option<&str>) {
    if let Some(p) = prompt {
        if !p.is_empty() {
            print!("{} ", p);
            stdout().flush().expect("Failed to flush stdout");
        }
    }
}
