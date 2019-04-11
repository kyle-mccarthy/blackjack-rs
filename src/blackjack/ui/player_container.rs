use cursive::theme::{BaseColor, Color, ColorStyle, Style};
use cursive::utils::markup::StyledString;
use cursive::view::{Margins, SizeConstraint, View, ViewWrapper};
use cursive::views::{
    BoxView, Button, IdView, LinearLayout, PaddedView, Panel, TextView,
};
use cursive::Cursive;

#[allow(dead_code)]
pub struct PlayerContainer {
    width: usize,
    height: usize,
    pub element: IdView<Panel<BoxView<PaddedView<LinearLayout>>>>,
}

impl PlayerContainer {
    pub fn build(width: usize, height: usize) -> Self {
        let mut inner = LinearLayout::horizontal();

        let available_width = width.saturating_sub(2);
        let left_col_width = available_width * 2 / 3;
        let right_col_width = available_width / 3;

        let left = Self::build_col(
            "player_container_left_col",
            left_col_width,
            Margins::new(0, 1, 0, 0),
            TextView::new("@todo"),
        );

        inner.add_child(left);

        let right = Self::build_col(
            "player_container_right_col",
            right_col_width,
            Margins::new(1, 0, 0, 0),
            LinearLayout::vertical()
                .child(TextView::new("@todo"))
                .child(Self::build_button("hit_btn", "Hit", |a| {})),
        );

        inner.add_child(right);

        let wrapper = IdView::new(
            "player_container",
            Panel::new(BoxView::new(
                SizeConstraint::Fixed(width),
                SizeConstraint::Fixed(height),
                PaddedView::new(Margins::from((1, 1, 0, 0)), inner),
            )),
        );

        PlayerContainer {
            width,
            height,
            //            element: Rc::new(RefCell::new(wrapper)),
            element: wrapper,
        }
    }

    #[allow(dead_code)]
    fn build_col<V: View>(
        id: &'static str,
        width: usize,
        margin: Margins,
        el: V,
    ) -> BoxView<PaddedView<IdView<V>>> {
        BoxView::with_fixed_width(
            width,
            PaddedView::new(margin, IdView::new(id, el)),
        )
    }

    #[allow(dead_code)]
    fn build_button<F>(
        id: &'static str,
        label: &'static str,
        cb: F,
    ) -> IdView<BoxView<Panel<TextView>>>
    where
        F: 'static + Fn(&mut Cursive),
    {
        let color_style = ColorStyle::new(
            Color::Light(BaseColor::Black),
            Color::Dark(BaseColor::Green),
        );

        let style = Style::from(color_style);
        let t = TextView::new(StyledString::styled(
            format!(" <{}> ", label),
            style,
        ))
        .center();
        IdView::new(
            id,
            BoxView::new(
                SizeConstraint::Full,
                SizeConstraint::Fixed(3),
                Panel::new(t),
            ),
        )
    }
}

impl ViewWrapper for PlayerContainer {
    type V = Panel<BoxView<PaddedView<LinearLayout>>>;

    fn with_view<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Self::V) -> R,
    {
        self.element.with_view(f)
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Self::V) -> R,
    {
        self.element.with_view_mut(f)
    }
}
