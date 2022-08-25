use crossterm::Result;
use figlet_rs::FIGfont;
use std::io::stdout;
use std::{thread, time};
use std::time::Duration;

use crate::timechunk::TimeChunk;
use crate::timetext::TimeText;

pub struct Timer {
    time: TimeChunk,
}

impl Timer {
    pub fn new(hours: u64, minutes: u64, seconds: u64) -> Self {
        Timer {
            time: TimeChunk::from_hms(hours, minutes, seconds),
        }
    }

    pub fn countdown(&mut self) -> Result<()> {
        let stdo = stdout();

        let font_string = std::include_str!("../resources/colossal.flf");
        let thick_font = FIGfont::from_content(font_string).unwrap();
        let mut timetext = TimeText::new(String::from(""), thick_font);
        self.time.print_timetext(&stdo, &mut timetext)?;

        let start_time = time::Instant::now();
        let mut current_time = start_time;
        let mut elapsed = Duration::new(0, 0);
        while elapsed < self.time.duration {
            // figure out the next time to sleep until
            let next_wake = start_time + Duration::from_secs(elapsed.as_secs() + 1);
            let sleep_duration = next_wake - current_time;

            // let sleep_duration = Duration::milliseconds(250);
            thread::sleep(sleep_duration);

            current_time = time::Instant::now();
            elapsed = current_time - start_time;

            let print_time = if elapsed > self.time.duration { Duration::default() } else { self.time.duration - elapsed + Duration::new(1, 0) };
            TimeChunk::new(print_time).print_timetext(&stdo, &mut timetext)?;
        }

        // Swipe the terminal one last time, and move the cursor to back to the
        // beginning of the terminal window.
        TimeChunk::wipe_terminal(&stdo)?;
        Ok(())
    }
}