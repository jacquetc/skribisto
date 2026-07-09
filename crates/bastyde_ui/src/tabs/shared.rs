//! Shared building blocks for the editor tabs: the quiet writing-editor style,
//! the centered/capped writing column, and small pane builders (writing column,
//! synopsis box, title field) reused across the per-`sub_role` layouts.

use std::rc::Rc;

use bastyde::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use bastyde::tokens::{BorderRole, CornerRadius, SurfaceRole};
use bastyde::widgets::rich_text::{EditorHandle, RichTextEditor, ScrollPolicy};
use bastyde::widgets::{
    Expand, FixedSize, GroupHeader, HStack, MaxSize, MenuItem, MenuList, Padding, Panel, RectWidget,
    Spacer, TextInput, VStack, ZStack,
};

/// A caret-aware "split scene" action for a writing editor's context menu:
/// invoked with the event context and the current caret offset.
pub type SplitFn = Rc<dyn Fn(&mut EventContext, usize)>;

use super::{ContentTab, TitleField};

/// How much narrower (px, total across both margins) the synopsis column is than
/// the main writing column, so it reads as the subordinate pane.
pub const SYNOPSIS_WIDTH_INSET: f32 = 48.0;

/// Minimum height (in lines) of the main writing editor when the scene tab
/// scrolls as one flowing page: a short scene still presents a page-sized
/// writing surface rather than collapsing to its few lines of text.
pub const MAIN_MIN_LINES: u32 = 20;

/// The centered, max-width main writing column for `doc`, wired so user edits
/// flip the tab's dirty flag via `on_change`.
///
/// **Flowing layout:** the editor is *intrinsic*-sized (`min_lines`, no
/// `max_lines` cap → grows to its content) and its own scroll bar is suppressed
/// (`AlwaysOff`); the scene tab's outer `ScrollArea` scrolls the whole page.
/// [`CenterColumnFlowing`] gives it a bounded width (so it wraps at the column
/// cap) but takes its intrinsic height (so the page grows with the prose).
pub fn writing_column(
    doc: &TextDocument,
    column_width: &Signal<f32>,
    on_change: impl Fn() + 'static,
    split: Option<SplitFn>,
) -> CenterColumnFlowing {
    let mut editor = RichTextEditor::editor(doc.clone())
        .style(WritingEditorStyle)
        .on_change(on_change)
        .content_padding_symmetric(8.0, 12.0)
        .min_lines(MAIN_MIN_LINES)
        .v_scroll_policy(ScrollPolicy::AlwaysOff);
    if let Some(split) = split {
        // Replace the built-in menu with the standard editing actions (rebuilt
        // through the editor handle) plus "Split scene" at the caret.
        let handle = editor.handle();
        let cursor = editor.cursor_position_signal();
        editor = editor.context_menu(move |_pt, _ctx| {
            Some(Box::new(scene_editor_menu(
                handle.clone(),
                cursor.clone(),
                split.clone(),
            )))
        });
    }
    CenterColumnFlowing::new(bati!(
        MaxSize::width(column_width.get()) {
            max_width: column_width.clone()
            Expand::horizontal { child: editor }
        }
    ))
}

/// The scene editor's right-click menu: Cut / Copy / Paste / Paste Unformatted /
/// Select All (via the editor handle) plus **Split scene** at the caret.
fn scene_editor_menu(handle: EditorHandle, cursor: Signal<usize>, split: SplitFn) -> MenuList {
    let cut = handle.clone();
    let copy = handle.clone();
    let paste = handle.clone();
    let paste_plain = handle.clone();
    let select = handle;
    MenuList::new()
        .item(MenuItem::new(tr!(menu_cut())).on_activate_fn(move |ctx| cut.cut(ctx)))
        .item(MenuItem::new(tr!(menu_copy())).on_activate_fn(move |ctx| copy.copy(ctx)))
        .item(MenuItem::new(tr!(menu_paste())).on_activate_fn(move |ctx| paste.paste(ctx)))
        .item(
            MenuItem::new(tr!(menu_paste_unformatted()))
                .on_activate_fn(move |ctx| paste_plain.paste_unformatted(ctx)),
        )
        .separator()
        .item(MenuItem::new(tr!(menu_select_all())).on_activate_fn(move |_ctx| select.select_all()))
        .separator()
        .item(MenuItem::new(tr!(split_scene())).on_activate_fn(move |ctx| split(ctx, cursor.get())))
}

/// The bordered synopsis editor box (caller sizes/centres it). User edits flip
/// the tab's dirty flag via `on_change`.
pub fn synopsis_editor(doc: &TextDocument, on_change: impl Fn() + 'static) -> impl Widget {
    bati!(
        Panel {
            background: SurfaceRole::Content
            border_color: BorderRole::Default
            border_width: 1.0
            corner_radius: 6.0
            RichTextEditor::editor(doc.clone()) {
                style: WritingEditorStyle
                on_change: on_change
                content_padding_symmetric: 6.0, 30.0
                min_lines: 1
                max_lines: 6
                v_scroll_policy: ScrollPolicy::Auto
                text_color: TextRole::Secondary
            }
        }
    )
}

/// A one-line title input bound to `field.value`.
pub fn title_input(field: &TitleField, placeholder: impl Into<LocalizedString>) -> impl Widget {
    TextInput::new(field.value.clone()).placeholder(placeholder)
}

/// "Synopsis" header + the centered, capped synopsis editor (a touch narrower
/// than the main column so it reads as subordinate).
pub fn synopsis_section(
    doc: &TextDocument,
    column_width: &Signal<f32>,
    on_change: impl Fn() + 'static,
) -> impl Widget {
    let synopsis_width = column_width.map(|w| (w - SYNOPSIS_WIDTH_INSET).max(0.0));
    bati!(
        VStack {
            spacing: 5.0
            GroupHeader::new(tr!(synopsis())) {
                style: TextStyleRole::SmallBold
                color: TextRole::Secondary
            }
            HStack {
                Spacer
                MaxSize::width(synopsis_width.get()) {
                    max_width: synopsis_width.clone()
                    Expand::horizontal {
                        child: synopsis_editor(doc, on_change)
                    }
                }
                Spacer {
                    min_length: 4.0
                }
            }
        }
    )
}

/// "Text" header + the centered, capped main writing column. The column is
/// intrinsic-height (see [`writing_column`]) so the section grows with the prose
/// and the tab's outer `ScrollArea` scrolls it.
pub fn writing_section(
    doc: &TextDocument,
    column_width: &Signal<f32>,
    on_change: impl Fn() + 'static,
) -> impl Widget {
    VStack::new()
        .spacing(5.0)
        .child(
            GroupHeader::new(tr!(text_heading()))
                .style(TextStyleRole::SmallBold)
                .color(TextRole::Secondary),
        )
        .child(writing_column(doc, column_width, on_change, None))
}

/// A flat, edge-to-edge Content-surface backdrop wrapping the tab body (the
/// `TabWidget` doesn't paint a content background).
pub fn tab_backdrop(body: impl Widget + 'static) -> Box<dyn Widget> {
    Box::new(bati!(
        Panel {
            background: SurfaceRole::Content
            corner_radius: 0.0
            padding: 0.0
            child: Expand { child: body }
        }
    ))
}

/// A fixed vertical gap.
pub fn vspace(height: f32) -> impl Widget {
    bati!(FixedSize { height: height })
}

/// Center `child` horizontally, capped at the live writing-column width.
pub fn centered(child: impl Widget + 'static, column_width: &Signal<f32>) -> impl Widget {
    bati!(
        HStack {
            Spacer
            MaxSize::width(column_width.get()) {
                max_width: column_width.clone()
                child: child
            }
            Spacer {
                min_length: 4.0
            }
        }
    )
}

/// The shared **Synopsis** view for the folder container tabs: the folder's
/// title (and subtitle, for a Book) above its synopsis editor. Folder tabs each
/// own a `SegmentedControl`; this is the segment they all share.
pub fn folder_synopsis_pane(tab: &ContentTab) -> impl Widget {
    let mut col = VStack::new().spacing(8.0).child(vspace(12.0));
    if let Some(t) = &tab.title {
        col = col.child(centered(
            title_input(t, tr!(placeholder_title())),
            &tab.column_width,
        ));
    }
    if let Some(st) = &tab.subtitle {
        col = col.child(centered(
            title_input(st, tr!(placeholder_subtitle())),
            &tab.column_width,
        ));
    }
    if let Some(s) = &tab.synopsis {
        col = col.child(vspace(4.0)).child(synopsis_section(
            &s.doc,
            &tab.column_width,
            tab.mark_dirty_fn(),
        ));
    }
    col
}

/// Editor chrome with a **constant** border instead of the default recipe's
/// focus-aware one — a writing editor is almost always focused, so the accent
/// focus ring would read as a permanent frame. Keeps a quiet, edge-to-edge box.
#[derive(Debug, Default, Clone, Copy)]
pub struct WritingEditorStyle;

impl RichTextEditorStyle for WritingEditorStyle {
    fn make_body(&self, cfg: &RichTextEditorStyleConfig, ctx: &mut BuildContext) -> WidgetId {
        if cfg.is_read_only {
            return match cfg.content_padding {
                Some((t, r, b, l)) => ctx.add(Padding::new(t, r, b, l).child_id(cfg.viewport)),
                None => cfg.viewport,
            };
        }
        let bg = ctx.add(
            RectWidget::new()
                .background(SurfaceRole::Content)
                .border_color(BorderRole::Default)
                .border_width(0.0)
                .corner_radius(CornerRadius::uniform(6.0)),
        );
        let (pt, pr, pb, pl) = cfg.content_padding.unwrap_or((8.0, 12.0, 8.0, 12.0));
        let padded = ctx.add(Padding::new(pt, pr, pb, pl).child_id(cfg.viewport));
        ctx.add(ZStack::new().add_child(bg).add_child(padded))
    }
}

/// Fills the available width and centers its single child **horizontally**, but
/// takes the child's **intrinsic height** instead of filling — for a flowing
/// page inside an outer `ScrollArea`. It proposes a *bounded* width (so a
/// width-capped wrapping child wraps at its cap rather than overflowing) and an
/// *unspecified* height (so the child reports its natural content height), then
/// centers the child horizontally.
#[derive(Debug)]
pub struct CenterColumnFlowing {
    child_id: Option<WidgetId>,
    pending: Option<Box<dyn Widget>>,
}

impl CenterColumnFlowing {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self {
            child_id: None,
            pending: Some(Box::new(child)),
        }
    }
}

impl Widget for CenterColumnFlowing {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(w) = self.pending.take() {
            self.child_id = Some(ctx.add_boxed(w));
        }
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        let width = proposal.resolve(0.0, 0.0).width;
        let height = self
            .child_id
            .and_then(|id| ctx.child_size(id, SizeProposal::with_width(width)))
            .map(|s| s.height)
            .unwrap_or(0.0);
        Size::new(width, height).into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            let size = ctx
                .child_size(child.id, SizeProposal::with_width(bounds.width))
                .unwrap_or_else(|| bounds.size());
            let dx = ((bounds.width - size.width) / 2.0).max(0.0);
            child.origin = Point::new(bounds.x + dx, bounds.y);
            child.size = size;
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}
