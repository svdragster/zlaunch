//! Rendering for AI items.

use crate::ai::AiItem;
use crate::ui::theme::theme;
use gpui::{Div, Stateful, div, prelude::*, svg};

use super::{item_container, render_action_indicator, render_text_content};

/// Render an AI item with brain icon and query.
pub fn render_ai(item: &AiItem, selected: bool, row: usize) -> Stateful<Div> {
    let mut container =
        item_container(row, selected)
            .child(render_ai_icon())
            .child(render_text_content(
                &item.name,
                Some(item.description()),
                selected,
            ));

    if selected {
        container = container.child(render_action_indicator("Ask"));
    }

    container
}

/// Render the AI brain icon.
fn render_ai_icon() -> Div {
    let t = theme();
    let size = t.icon_size;

    let icon_container = div()
        .w(size)
        .h(size)
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .bg(t.icon_placeholder_background)
        .rounded_sm();

    icon_container.child(
        svg()
            .path(crate::assets::PhosphorIcon::Brain.path())
            .size_4()
            .text_color(t.icon_placeholder_color),
    )
}
