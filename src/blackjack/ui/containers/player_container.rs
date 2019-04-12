use std::sync::{Arc, RwLock};

use crossbeam_channel::Sender;
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::view::{Identifiable, Margins, SizeConstraint, View, ViewWrapper};
use cursive::views::{
    BoxView, IdView, LinearLayout, PaddedView, Panel, TextView,
};
use cursive::Cursive;

use crate::blackjack::game::Action;
use crate::blackjack::player::Player;
use crate::blackjack::ui::utils::{cb, ViewId};
use crate::blackjack::ui::views::{Button, PlayerView};

#[allow(dead_code)]
pub struct PlayerContainer {
    width: usize,
    height: usize,
    pub element: IdView<Panel<BoxView<PaddedView<LinearLayout>>>>,
}

impl PlayerContainer {
    pub fn build(
        width: usize,
        height: usize,
        tx: Sender<Action>,
        player: Arc<RwLock<Player>>,
    ) -> Self {
        let mut inner = LinearLayout::horizontal();

        let available_width = width.saturating_sub(2);
        let left_col_width = available_width * 2 / 3;
        let right_col_width = available_width / 3;

        let left = Self::build_col(
            ViewId::PlayerContainerLeftColumn,
            left_col_width,
            Margins::new(0, 1, 0, 0),
            TextView::new("@todo"),
        );

        inner.add_child(left);

        let player_clone = player.clone();
        let player_clone2 = player.clone();

        let right = Self::build_col(
            ViewId::PlayerContainerRightColumn,
            right_col_width,
            Margins::new(1, 0, 0, 0),
            LinearLayout::vertical()
                .child(
                    TextView::new(format!(
                        "Bankroll: {}",
                        player.read().unwrap().get_available_funds()
                    ))
                    .with_id(ViewId::PlayerBankroll.to_string()),
                )
                .child(Self::build_button(
                    ViewId::HitButton,
                    "Hit",
                    cb(tx.clone(), move |t, s| {
                        let player_clone = player_clone.clone();
                        t.send(Action::AddFundsToPlayerBankroll(
                            player_clone,
                            10,
                        ))
                        .unwrap();
                    }),
                ))
                .child(Self::build_button(
                    ViewId::StayButton,
                    "stay",
                    cb(tx.clone(), move |s, t| {
                        info!("Trying to add funds");
                    }),
                )),
        );

        inner.add_child(right);

        let wrapper = Panel::new(BoxView::new(
            SizeConstraint::Fixed(width),
            SizeConstraint::Fixed(height),
            PaddedView::new(Margins::from((1, 1, 0, 0)), inner),
        ))
        .title(player.read().unwrap().get_name().clone())
        .with_id(ViewId::PlayerContainer.to_string());

        PlayerContainer {
            width,
            height,
            element: wrapper,
        }
    }

    #[allow(dead_code)]
    fn build_col<V: View>(
        id: ViewId,
        width: usize,
        margin: Margins,
        el: V,
    ) -> BoxView<PaddedView<IdView<V>>> {
        BoxView::with_fixed_width(
            width,
            PaddedView::new(margin, IdView::new(id.to_string(), el)),
        )
    }

    #[allow(dead_code)]
    fn build_button<F>(id: ViewId, label: &'static str, cb: F) -> IdView<Button>
    where
        F: 'static + Fn(&mut Cursive),
    {
        let styles = ColorStyle::new(
            Color::Light(BaseColor::Black),
            Color::Dark(BaseColor::Green),
        );
        Button::new(format!(" <{}> ", label), styles, cb)
            .set_size(SizeConstraint::Full, SizeConstraint::Fixed(3))
            .with_id(id.to_string())
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
