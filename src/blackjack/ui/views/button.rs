use std::sync::{Arc, RwLock};

use cursive::align::{HAlign, VAlign};
use cursive::direction::Direction;
use cursive::event::{Callback, Event, EventResult, MouseButton, MouseEvent};
use cursive::theme::ColorStyle;
use cursive::traits::With;
use cursive::view::{SizeConstraint, View};
use cursive::{Cursive, Printer, Rect, Vec2, XY};
use unicode_width::UnicodeWidthStr;

pub struct Button {
    label: String,
    style: ColorStyle,
    on_click: Callback,
    last_size: Vec2,
    size: Option<(XY<SizeConstraint>)>,
}

impl Button {
    pub fn new<F>(label: String, style: ColorStyle, cb: F) -> Button
    where
        F: 'static + Fn(&mut Cursive),
    {
        Button {
            label,
            style,
            on_click: Callback::from_fn(cb),
            last_size: Vec2::zero(),
            size: None,
        }
    }

    pub fn set_size(
        self,
        width: SizeConstraint,
        height: SizeConstraint,
    ) -> Self {
        self.with(|s| {
            let size: XY<SizeConstraint> = (width, height).into();
            s.size = Some(size);
        })
    }
}

// see https://github.com/gyscos/Cursive/blob/master/src/views/button.rs
// for more information on implementation
impl View for Button {
    fn draw(&self, printer: &Printer) {
        let label = self.label.as_str();

        let x_offset = HAlign::Center.get_offset(label.width(), printer.size.x);
        // right now the button just assumes there is a single line of text - this will be a problem if there are multiple
        let y_offset = VAlign::Center.get_offset(1, printer.size.y);

        printer.with_color(self.style, |printer| {
            printer.print((x_offset, y_offset), &self.label);

            // if there is size we know that there should be a box wrapping it
            if self.size.is_some() {
                printer.print_box((0, 0), printer.size, false);
            }
        });
    }

    // consume the left mouse click and ignore everything else
    fn on_event(&mut self, event: Event) -> EventResult {
        //        let width = self.label.width();
        //        let self_offset = HAlign::Center.get_offset(width, self.last_size.x);

        // @todo verify this still works correctly when a button doesn't have a box

        match event {
            Event::Mouse {
                position,
                offset,
                event: MouseEvent::Release(MouseButton::Left),
            } => {
                if position.fits_in_rect(offset, self.last_size) {
                    return EventResult::Consumed(Some(self.on_click.clone()));
                }
                EventResult::Ignored
            }
            _ => EventResult::Ignored,
        }
    }

    fn take_focus(&mut self, _source: Direction) -> bool {
        true
    }

    fn layout(&mut self, size: Vec2) {
        self.last_size = size;
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        // if there is a size set, the buttons should be boxed... calculate the size available
        // https://github.com/gyscos/Cursive/blob/master/src/views/box_view.rs#L211
        if let Some(size) = self.size {
            let req = size.zip_map(constraint, SizeConstraint::available);

            let desired_size = Vec2::from((1, 1));

            let res =
                size.zip_map(desired_size.zip(req), SizeConstraint::result);

            return res;
        }
        Vec2::from((1, 1))
    }
}
