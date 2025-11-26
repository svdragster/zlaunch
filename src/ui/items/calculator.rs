//! Calculator item rendering.
//!
//! Renders the calculator result as a special "bigger" item at the top of the list.

use crate::items::CalculatorItem;
use crate::ui::theme::theme;
use gpui::{Div, ElementId, SharedString, Stateful, div, hsla, prelude::*, px};

use super::base::render_action_indicator;

/// Render a calculator item with special styling.
///
/// The calculator item is 1.5x the height of normal items and displays:
/// - A custom "=" icon
/// - The expression as a muted title
/// - The result (or error) with "= " prefix in larger text
pub fn render_calculator(calc: &CalculatorItem, selected: bool, row: usize) -> Stateful<Div> {
    let t = theme();

    let bg_color = if selected {
        t.item_background_selected
    } else {
        t.item_background
    };

    // 1.5x vertical padding for bigger item
    let padding_y = t.item_padding_y * 1.5;

    let mut container = div()
        .id(ElementId::NamedInteger("calc-item".into(), row as u64))
        .mx(t.item_margin_x)
        .my(t.item_margin_y)
        .px(t.item_padding_x)
        .py(padding_y)
        .bg(bg_color)
        .rounded(t.item_border_radius)
        .overflow_hidden()
        .relative()
        .flex()
        .flex_row()
        .items_center()
        .gap_2();

    // Add icon
    container = container.child(render_calculator_icon());

    // Add text content
    container = container.child(render_calculator_content(calc, selected));

    // Add action indicator when selected
    if selected {
        container = container.child(render_action_indicator("Copy"));
    }

    container
}

/// Render the calculator icon (an "=" in a colored circle).
fn render_calculator_icon() -> Div {
    let t = theme();
    let size = t.icon_size;

    // Blue-ish accent color for the calculator
    let icon_bg = hsla(210.0 / 360.0, 0.6, 0.5, 0.15);
    let icon_color = hsla(210.0 / 360.0, 0.7, 0.7, 1.0);

    div()
        .w(size)
        .h(size)
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_center()
        .bg(icon_bg)
        .rounded_sm()
        .child(
            div()
                .text_sm()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(icon_color)
                .child(SharedString::from("=")),
        )
}

/// Render the calculator text content (expression + result).
fn render_calculator_content(calc: &CalculatorItem, selected: bool) -> Div {
    let t = theme();

    // Expression as muted smaller text
    let expression_element = div()
        .w_full()
        .text_xs()
        .text_color(t.item_description_color)
        .whitespace_nowrap()
        .overflow_hidden()
        .text_ellipsis()
        .child(SharedString::from(calc.expression.clone()));

    // Result with "= " prefix, larger text
    let result_color = if calc.is_error {
        // Error color: orange/red-ish
        hsla(15.0 / 360.0, 0.7, 0.6, 1.0)
    } else {
        t.item_title_color
    };

    let result_text = format!("= {}", calc.display_result);
    let result_element = div()
        .w_full()
        .text_base() // Slightly larger than normal text_sm
        .font_weight(gpui::FontWeight::MEDIUM)
        .text_color(result_color)
        .whitespace_nowrap()
        .overflow_hidden()
        .text_ellipsis()
        .child(SharedString::from(result_text));

    let max_width = t.max_text_width(selected);

    // Content height is 1.5x normal to accommodate larger result text
    let content_height = t.item_content_height * 1.25;

    div()
        .h(content_height)
        .max_w(max_width)
        .flex()
        .flex_col()
        .justify_center()
        .overflow_hidden()
        .gap(px(2.0))
        .child(expression_element)
        .child(result_element)
}
