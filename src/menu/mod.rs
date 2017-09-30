use idle;
use rtfm::Threshold;
use core::result::Result;
use config::Display;

pub mod feed;

#[derive(Debug)]
pub struct Next;
#[derive(Debug)]
pub struct Exit;

pub type MenuResult = Result<Next, Exit>;

pub trait Menu {
    fn label(&self) -> &'static str;
    fn run(&mut self, _t: &mut Threshold, r: &mut idle::Resources) -> MenuResult;
}

pub struct MainMenu {
    feed: feed::FeedMenu
}

impl MainMenu {
    pub fn new() -> MainMenu {
        MainMenu {
            feed: feed::FeedMenu::new(),
        }
    }
}

impl Menu for MainMenu {
    fn run(&mut self, t: &mut Threshold, r: &mut idle::Resources) -> MenuResult {
        run_items(&mut [&mut self.feed], t, r)
    }

    fn label(&self) -> &'static str {
        "Main"
    }
}

fn run_items(items: &mut [&mut Menu], t: &mut Threshold, r: &mut idle::Resources) -> MenuResult {
    r.ENCODER.set_current(0);
    r.ENCODER.set_limit(items.len() as u16);
    loop {
        let current: &mut Menu = items[r.ENCODER.current() as usize];

        if r.CONTROLS.state().button {
            Display::new(r.SCREEN).clear();
            current.run(t, r)?;
        } else {
            let mut lcd = Display::new(r.SCREEN);
            lcd.position(0, 0);
            lcd.print(current.label());
        }
    }
}
