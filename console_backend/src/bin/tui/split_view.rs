use tui::layout::{Constraint, Direction, Layout, Rect};

use crate::terminal::Frame;
use crate::Drawable;

pub struct SplitView<T, U> {
    pub a: T,
    pub b: U,
    direction: Direction,
    at: u16,
}

impl<T, U> SplitView<T, U> {
    pub fn new(a: T, b: U, direction: Direction, at: u16) -> Self {
        Self {
            a,
            b,
            direction,
            at,
        }
    }

    pub fn shift(&mut self) {
        self.at = (self.at + 10).min(100);
    }

    pub fn unshift(&mut self) {
        self.at = self.at.saturating_sub(10);
    }
}

impl<T, U> Drawable for SplitView<T, U>
where
    T: Drawable,
    U: Drawable,
{
    fn draw(&self, f: &mut Frame, rect: Rect) {
        let rects = Layout::default()
            .direction(self.direction.clone())
            .constraints([Constraint::Percentage(self.at), Constraint::Min(0)])
            .split(rect);
        self.a.draw(f, rects[0]);
        self.b.draw(f, rects[1]);
    }
}
