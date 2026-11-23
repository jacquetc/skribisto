// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A press on the empty page around a writing editor puts the caret in it: on
//! the line at that height beside the text, at the end of the text under it.
//!
//! These build the real `writing_column` and press it through the real pointer
//! router, because what is being proven is the hit test's verdict: a press on a
//! background reaches the editor, and a press on anything else does not.

use super::*;
use teksilo::core::widget_tree::WidgetTree;
use teksilo::text_document::{MoveMode, MoveOperation};
use teksilo::widgets::ScrollArea;

const WINDOW_W: f32 = 900.0;
const WINDOW_H: f32 = 700.0;
/// Narrower than the window, so the column has a margin on each side.
const COLUMN_W: f32 = 400.0;

/// The scene's own defaults: the gaps between lines and between paragraphs are
/// part of what a press beside the text has to resolve.
fn test_typo() -> EditorTypography {
    typo_with_spacing(0.0)
}

fn typo_with_spacing(para_spacing_after: f32) -> EditorTypography {
    EditorTypography {
        font_family: Signal::new("Literata".to_string()),
        size: Signal::new(1.0),
        line_height: Signal::new(crate::settings_keys::SCENE_LINE_HEIGHT_DEFAULT),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(para_spacing_after),
        justify_new_text: Signal::new(false),
        size_range: crate::settings::TypographySizeRange::default(),
    }
}

/// Three short paragraphs, each on one line at `COLUMN_W`.
fn three_paragraphs() -> TextDocument {
    let doc = TextDocument::new();
    let _ = doc.set_djot_sync("First line.\n\nSecond line.\n\nThird line.\n");
    doc
}

/// The cursor position where the paragraph `index` (0-based) starts, and where
/// it ends.
fn paragraph_span(doc: &TextDocument, index: usize) -> (usize, usize) {
    let c = doc.cursor();
    c.move_position(MoveOperation::Start, MoveMode::MoveAnchor, 1);
    for _ in 0..index {
        c.move_position(MoveOperation::NextBlock, MoveMode::MoveAnchor, 1);
    }
    let start = c.position();
    c.move_position(MoveOperation::EndOfBlock, MoveMode::MoveAnchor, 1);
    (start, c.position())
}

fn binding() -> crate::shared::ViewStateBinding {
    crate::shared::ViewStateBinding {
        initial: crate::shared::ViewState::default(),
        ports: Rc::new(crate::shared::ViewStatePorts::default()),
        page_editor: Rc::new(RefCell::new(None)),
        page_caret: Rc::new(RefCell::new(None)),
    }
}

fn column(
    doc: &TextDocument,
    view_state: &crate::shared::ViewStateBinding,
    typo: &EditorTypography,
) -> impl Widget {
    writing_column(
        doc,
        &Signal::new(COLUMN_W),
        typo,
        MAIN_MIN_LINES,
        || {},
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(view_state.clone()),
        None,
        None,
        None,
        None,
        false,
        None,
        false,
        None,
        None,
        tr!(editor_text_name()),
        None,
    )
}

/// The page the app builds around a writing column, reduced to what a press
/// meets: a `ScrollArea` stretched to the viewport holding a `PageTail`, and a
/// button under the column standing in for whatever else a page carries.
struct Page {
    tree: WidgetTree,
    handle: EditorHandle,
    scroll_y: Signal<f32>,
    max_scroll_y: Signal<f32>,
}

fn page(doc: &TextDocument, pressed: Rc<Cell<bool>>) -> Page {
    page_with(doc, pressed, &test_typo())
}

fn page_with(doc: &TextDocument, pressed: Rc<Cell<bool>>, typo: &EditorTypography) -> Page {
    let vs = binding();
    let content = VStack::new()
        .spacing(0.0)
        .child(column(doc, &vs, typo))
        .child(Button::new(lit!("A control")).on_activate_fn(move |_| pressed.set(true)));
    let area = ScrollArea::new()
        .widget_resizable(true)
        .smooth_scrolling(false);
    let scroll_y = area.scroll_y_signal().clone();
    let max_scroll_y = area.max_scroll_y_signal().clone();
    let area = area.child(PageTail::new(vs.page_caret.clone(), content).bottom_space(28.0));
    let mut tree = WidgetTree::new();
    tree.add(area);
    // Twice: the editor shapes its text at the first paint, and the page only
    // measures the text's real height on the layout after it.
    for _ in 0..2 {
        tree.layout(SizeProposal::exact(WINDOW_W, WINDOW_H));
        let _ = tree.render();
    }
    let handle = vs
        .page_editor
        .borrow()
        .clone()
        .expect("the column publishes its editor");
    Page {
        tree,
        handle,
        scroll_y,
        max_scroll_y,
    }
}

/// The window-space middle of the line holding `offset`.
fn line_middle(handle: &EditorHandle, offset: usize) -> f32 {
    let r = handle.offset_rect(offset).expect("the editor has a layout");
    r.y + r.height / 2.0
}

/// Where the column sits: its left and right edges in window space.
fn column_edges(handle: &EditorHandle, doc: &TextDocument) -> (f32, f32) {
    // The start of a line is the column's left edge (plus its content padding),
    // and the column is centred, so the right edge mirrors it.
    let (start, _) = paragraph_span(doc, 0);
    let left = handle.offset_rect(start).expect("a layout").x;
    (left, WINDOW_W - left)
}

fn click(tree: &mut WidgetTree, at: Point) {
    tree.pointer_down_button(at, PointerButton::Primary);
    tree.pointer_up_button(at, PointerButton::Primary);
    let _ = tree.render();
}

#[test]
fn a_click_left_of_a_line_puts_the_caret_at_its_start() {
    let doc = three_paragraphs();
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let (second_start, _) = paragraph_span(&doc, 1);
    let (left, _) = column_edges(&handle, &doc);
    assert!(
        left > 100.0,
        "the column must leave a margin to press: {left}"
    );

    click(
        &mut tree,
        Point::new(left / 2.0, line_middle(&handle, second_start)),
    );

    assert_eq!(handle.selection(), (second_start, second_start));
    assert!(
        handle.focused_signal().get(),
        "the editor takes the keyboard, so the caret shows and typing goes there"
    );
}

#[test]
fn a_click_right_of_a_line_puts_the_caret_at_its_end() {
    let doc = three_paragraphs();
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let (second_start, second_end) = paragraph_span(&doc, 1);
    let (_, right) = column_edges(&handle, &doc);

    click(
        &mut tree,
        Point::new((right + WINDOW_W) / 2.0, line_middle(&handle, second_start)),
    );

    assert_eq!(handle.selection(), (second_end, second_end));
    assert!(handle.focused_signal().get());
}

/// The editor is ten lines tall however short its text, and the margin beside
/// it is as tall: beside the empty lines is past the last one.
#[test]
fn a_click_in_the_margin_below_the_last_line_puts_the_caret_at_the_end() {
    let doc = three_paragraphs();
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let end = crate::shared::binder_ops::end_of(&doc);
    let last = handle.offset_rect(end).expect("a layout");
    let (left, _) = column_edges(&handle, &doc);

    click(
        &mut tree,
        Point::new(left / 2.0, last.bottom() + 2.0 * last.height),
    );

    assert_eq!(handle.selection(), (end, end));
    assert!(handle.focused_signal().get());
}

#[test]
fn a_click_under_the_page_puts_the_caret_at_the_end_of_the_text() {
    let doc = three_paragraphs();
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let end = crate::shared::binder_ops::end_of(&doc);

    click(&mut tree, Point::new(WINDOW_W / 2.0, WINDOW_H - 20.0));

    assert_eq!(handle.selection(), (end, end));
    assert!(handle.focused_signal().get());
}

/// The space kept under the content is the page's too, not a spacer of its own
/// that a press would land on and stop.
#[test]
fn a_click_in_the_space_kept_under_the_content_reaches_the_editor() {
    let doc = TextDocument::new();
    let _ = doc.set_djot_sync(&"A paragraph long enough to take a line.\n\n".repeat(40));
    let Page {
        mut tree,
        handle,
        scroll_y,
        max_scroll_y,
    } = page(&doc, Rc::new(Cell::new(false)));
    let end = crate::shared::binder_ops::end_of(&doc);
    // Scrolled to the bottom: the content is taller than the window, so the only
    // space under it is the kept one.
    let max = max_scroll_y.get();
    assert!(
        max > 28.0,
        "the page must scroll for this to mean anything: {max}"
    );
    scroll_y.set(max);
    tree.layout(SizeProposal::exact(WINDOW_W, WINDOW_H));
    let _ = tree.render();

    click(&mut tree, Point::new(WINDOW_W / 2.0, WINDOW_H - 10.0));

    assert_eq!(handle.selection(), (end, end));
}

/// A press on a control in the page is the control's, never the caret's.
#[test]
fn a_click_on_a_control_on_the_page_does_not_move_the_caret() {
    let doc = three_paragraphs();
    let pressed = Rc::new(Cell::new(false));
    let Page {
        mut tree, handle, ..
    } = page(&doc, pressed.clone());
    handle.select_range(3, 3);
    let control = tree.find_by_label("A control").expect("the control");

    tree.click(control);

    assert!(pressed.get(), "the control still activates");
    assert_eq!(handle.selection(), (3, 3), "the caret stays where it was");
}

/// A press on the text is the editor's own: the page handler must not put the
/// caret somewhere else first, or in addition.
#[test]
fn a_click_on_the_text_is_left_to_the_editor() {
    let doc = three_paragraphs();
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let (second_start, second_end) = paragraph_span(&doc, 1);
    let a = handle.offset_rect(second_start).expect("a layout");
    let b = handle.offset_rect(second_end).expect("a layout");
    let inside = Point::new((a.x + b.x) / 2.0, a.y + a.height / 2.0);
    let expected = handle.offset_at_point(inside).expect("a point on the text");

    click(&mut tree, inside);

    assert_eq!(handle.selection(), (expected, expected));
}

#[test]
fn a_finger_tap_beside_a_line_puts_the_caret_there_on_release() {
    let doc = three_paragraphs();
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let (second_start, second_end) = paragraph_span(&doc, 1);
    let (_, right) = column_edges(&handle, &doc);
    let at = Point::new((right + WINDOW_W) / 2.0, line_middle(&handle, second_start));
    handle.select_range(0, 0);

    let finger = tree.new_contact();
    tree.touch_down(finger, at);
    assert_eq!(
        handle.selection(),
        (0, 0),
        "a landing finger has not chosen yet: it may be starting to scroll"
    );
    tree.touch_up(finger, at);
    let _ = tree.render();

    assert_eq!(handle.selection(), (second_end, second_end));
    assert!(handle.focused_signal().get());
}

#[test]
fn a_finger_tap_under_the_page_puts_the_caret_at_the_end() {
    let doc = three_paragraphs();
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let end = crate::shared::binder_ops::end_of(&doc);
    handle.select_range(0, 0);

    let at = Point::new(WINDOW_W / 2.0, WINDOW_H - 20.0);
    let finger = tree.new_contact();
    tree.touch_down(finger, at);
    tree.touch_up(finger, at);
    let _ = tree.render();

    assert_eq!(handle.selection(), (end, end));
    assert!(handle.focused_signal().get());
}

/// A finger that lands on the margin and slides is scrolling the page.
#[test]
fn a_finger_sliding_from_the_margin_does_not_move_the_caret() {
    let doc = three_paragraphs();
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let (second_start, _) = paragraph_span(&doc, 1);
    let (left, _) = column_edges(&handle, &doc);
    let from = Point::new(left / 2.0, line_middle(&handle, second_start));
    let to = Point::new(from.x, from.y + 120.0);
    handle.select_range(0, 0);

    let finger = tree.new_contact();
    tree.touch_down(finger, from);
    tree.touch_move(finger, Point::new(from.x, from.y + 60.0));
    tree.touch_move(finger, to);
    tree.touch_up(finger, to);
    let _ = tree.render();

    assert_eq!(handle.selection(), (0, 0));
    assert!(!handle.focused_signal().get());
}

/// A right-click in the margin opens no caret anywhere: only the primary button
/// chooses a place.
#[test]
fn a_secondary_click_in_the_margin_leaves_the_caret() {
    let doc = three_paragraphs();
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let (second_start, _) = paragraph_span(&doc, 1);
    let (left, _) = column_edges(&handle, &doc);
    handle.select_range(0, 0);

    let at = Point::new(left / 2.0, line_middle(&handle, second_start));
    tree.pointer_down_button(at, PointerButton::Secondary);
    tree.pointer_up_button(at, PointerButton::Secondary);

    assert_eq!(handle.selection(), (0, 0));
}

/// Left of a wrapped line of a paragraph is the start of that line, drawn there.
///
/// Each wrapped line keeps its trailing space, so the end of one line and the
/// start of the next are the same position in the text and only the caret's
/// affinity says which line it is drawn on. Placed with `select_range` it took
/// the default one and was drawn at the end of the line above: so this asserts
/// where the caret is *drawn*, not which offset it holds.
#[test]
fn a_click_left_of_a_wrapped_line_puts_the_caret_at_its_start() {
    let doc = TextDocument::new();
    let _ = doc.set_djot_sync(&"A paragraph long enough to wrap onto several lines. ".repeat(6));
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let lines = wrapped_lines(&handle, &doc);
    let (left, _) = column_edges(&handle, &doc);
    let text_left = handle.offset_rect(0).expect("a layout").x;

    for y in lines.into_iter().skip(1) {
        handle.select_range(0, 0);
        tree.advance_time(std::time::Duration::from_secs(1));
        let height = handle.offset_rect(0).expect("a layout").height;
        click(&mut tree, Point::new(left / 2.0, y + height / 2.0));
        let caret = handle.caret_window_rect().expect("a caret");
        assert!(
            (caret.y - y).abs() < 0.5,
            "the caret is drawn on the line pressed beside ({y}), not at {}",
            caret.y
        );
        assert!(
            (caret.x - text_left).abs() < 1.0,
            "the caret is drawn at the start of the line ({text_left}), not at {}",
            caret.x
        );
    }
}

/// The y every line of `doc` is drawn at, top to bottom.
fn wrapped_lines(handle: &EditorHandle, doc: &TextDocument) -> Vec<f32> {
    let end = crate::shared::binder_ops::end_of(doc);
    let mut lines = vec![handle.offset_rect(0).expect("a layout").y];
    for offset in 0..=end {
        let y = handle.offset_rect(offset).expect("a layout").y;
        if lines.last().is_some_and(|last| y > last + 0.5) {
            lines.push(y);
        }
    }
    assert!(lines.len() >= 3, "the paragraph must wrap: {lines:?}");
    lines
}

/// Right of a wrapped line is its end, drawn on that line, not the start of the
/// line under it, which is the same position in the text.
#[test]
fn a_click_right_of_a_wrapped_line_puts_the_caret_at_its_end() {
    let doc = TextDocument::new();
    let _ = doc.set_djot_sync(&"A paragraph long enough to wrap onto several lines. ".repeat(6));
    let Page {
        mut tree, handle, ..
    } = page(&doc, Rc::new(Cell::new(false)));
    let mut lines = wrapped_lines(&handle, &doc);
    // Every line but the last, whose end is the end of the text.
    lines.pop();
    let (_, right) = column_edges(&handle, &doc);

    for y in lines {
        handle.select_range(0, 0);
        tree.advance_time(std::time::Duration::from_secs(1));
        let height = handle.offset_rect(0).expect("a layout").height;
        click(
            &mut tree,
            Point::new((right + WINDOW_W) / 2.0, y + height / 2.0),
        );
        let caret = handle.caret_window_rect().expect("a caret");
        assert!(
            (caret.y - y).abs() < 0.5,
            "the caret is drawn on the line that was pressed beside ({y}), not at {}",
            caret.y
        );
        let (at, _) = handle.selection();
        let next = handle.offset_rect(at + 1).expect("a layout");
        assert!(
            next.y > y + 0.5,
            "the caret is at the end of its line: the next position is on the line below"
        );
    }
}
