// Credits to: https://github.dev/YS-L/csvlens/blob/main/src/util/events.rs

use std::time::{Duration,Instant};

use crossterm::event::{Event as CrossTermEvent, KeyEvent, KeyEventKind, poll, read};

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    tick_rate: Duration,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: Duration,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        Events {
            tick_rate: config.tick_rate,
        }
    }

    pub fn next(&self) -> std::io::Result<Event<KeyEvent>> {
        let now = Instant::now();
        match poll(self.tick_rate) {
            Ok(true) => match read()? {
                CrossTermEvent::Key(event) if event.kind == KeyEventKind::Press => {
                    Ok(Event::Input(event))
                }
                _ => {
                    let time_spent = now.elapsed();
                    let rest = self.tick_rate.saturating_sub(time_spent);

                    Self { tick_rate: rest }.next()
                }
            },
            Ok(false) => Ok(Event::Tick),
            Err(_) => todo!(),
        }
    }
}
