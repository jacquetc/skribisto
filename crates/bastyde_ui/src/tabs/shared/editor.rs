//! Low-level editor primitives shared by the per-combination tabs: the quiet
//! writing-editor style, the centered/capped writing column, the synopsis box,
//! the title field, and the live typography plumbing. Composite pane renders
//! (heading form, dual-pane prose, folder synopsis) live in [`super::panes`].

use std::rc::Rc;

use bastyde::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::text::EditorTypographyDefaults;
use bastyde::text_document::TextDocument;
use bastyde::tokens::{BorderRole, CornerRadius, SurfaceRole};
use bastyde::widgets::rich_text::{EditorHandle, RichTextEditor, ScrollPolicy};
use bastyde::widgets::{
    Expand, FixedSize, GroupHeader, HStack, MaxSize, MenuItem, MenuList, Padding, Panel,
    RectWidget, Spacer, TextInput, VStack, ZStack,
};

use crate::tabs::TitleField;
use crate::view_models::EditorTypography;

/// A caret-aware "split scene" action for a writing editor's context menu:
/// invoked with the event context and the current caret offset.
pub type SplitFn = Rc<dyn Fn(&mut EventContext, usize)>;

/// How much narrower (px, total across both margins) the synopsis column is than
/// the main writing column, so it reads as the subordinate pane.
pub const SYNOPSIS_WIDTH_INSET: f32 = 48.0;

/// Minimum height (in lines) of the main writing editor when the scene tab
/// scrolls as one flowing page: a short scene still presents a page-sized
/// writing surface rather than collapsing to its few lines of text.
pub const MAIN_MIN_LINES: u32 = 10;

/// Minimum height for a *subordinate* prose editor inside a stream — a chapter
/// heading's own prose in a Full Part / Full Book view. A chapter folder always has a
/// `SceneText` field (the matrix gives it one), so at `MAIN_MIN_LINES` a Full Book
/// would show an empty ten-line box under every chapter heading. One line, growing
/// with its content, keeps the manuscript readable.
pub const HEADING_PROSE_MIN_LINES: u32 = 1;

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
    typo: &EditorTypography,
    min_lines: u32,
    on_change: impl Fn() + 'static,
    split: Option<SplitFn>,
) -> CenterColumnFlowing {
    let mut editor = RichTextEditor::editor(doc.clone())
        .style(WritingEditorStyle)
        .on_change(on_change)
        .content_padding_symmetric(8.0, 12.0)
        .min_lines(min_lines)
        .v_scroll_policy(ScrollPolicy::AlwaysOff)
        .typography_defaults(typo_defaults(typo))
        .zoom(typo.size.get());
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
            Expand::horizontal {
                child: TypographyBoundEditor::new(editor, typo.clone())
            }
        }
    ))
}

/// A writing editor's right-click menu: Cut / Copy / Paste / Paste Unformatted /
/// Select All (via the editor handle) plus **Split scene** at the caret. Offered on
/// both of a row's surfaces — which text the split cuts is implicit in which editor
/// was right-clicked (see `StreamViewModel::split_row`).
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

/// The bordered synopsis editor box (caller sizes/centres it). User edits flip the
/// tab's dirty flag via `on_change`. `split` adds the caret-aware "Split scene" action
/// to its context menu — in a Full Synopsis stream the synopsis is the text being cut.
pub fn synopsis_editor(
    doc: &TextDocument,
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
    split: Option<SplitFn>,
) -> impl Widget {
    let mut editor = RichTextEditor::editor(doc.clone())
        .style(WritingEditorStyle)
        .on_change(on_change)
        .content_padding_symmetric(6.0, 30.0)
        .min_lines(1)
        .max_lines(6)
        .v_scroll_policy(ScrollPolicy::Auto)
        .text_color(TextRole::Secondary)
        .typography_defaults(typo_defaults(typo))
        .zoom(typo.size.get());
    if let Some(split) = split {
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
    bati!(
        Panel {
            background: SurfaceRole::Content
            border_color: BorderRole::Default
            border_width: 1.0
            corner_radius: 6.0
            child: TypographyBoundEditor::new(editor, typo.clone())
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
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
) -> impl Widget {
    bati!(
        VStack {
            spacing: 5.0
            GroupHeader::new(tr!(synopsis())) {
                style: TextStyleRole::SmallBold
                color: TextRole::Secondary
            }
            child: synopsis_column(doc, column_width, typo, on_change, Option::None)
        }
    )
}

/// The centered, capped synopsis editor without the "Synopsis" caption — for a stream
/// row, where the caption would repeat on every row and the segment already says it.
pub fn synopsis_column(
    doc: &TextDocument,
    column_width: &Signal<f32>,
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
    split: Option<SplitFn>,
) -> impl Widget {
    let synopsis_width = column_width.map(|w| (w - SYNOPSIS_WIDTH_INSET).max(0.0));
    bati!(
        HStack {
            Spacer
            MaxSize::width(synopsis_width.get()) {
                max_width: synopsis_width.clone()
                Expand::horizontal {
                    child: synopsis_editor(doc, typo, on_change, split)
                }
            }
            Spacer {
                min_length: 4.0
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
    typo: &EditorTypography,
    on_change: impl Fn() + 'static,
) -> impl Widget {
    VStack::new()
        .spacing(5.0)
        .child(
            GroupHeader::new(tr!(text_heading()))
                .style(TextStyleRole::SmallBold)
                .color(TextRole::Secondary),
        )
        .child(writing_column(
            doc,
            column_width,
            typo,
            MAIN_MIN_LINES,
            on_change,
            None,
        ))
}

/// A flat, edge-to-edge Content-surface backdrop wrapping the tab body (the
/// `TabWidget` doesn't paint a content background).
pub fn tab_backdrop(body: impl Widget + 'static) -> Box<dyn Widget> {
    Box::new(bati!(
        Panel {
            background: SurfaceRole::Content
            corner_radius: 0.0
            padding: 0.0
            child: Expand {
                child: body
            }
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

/// The framework's non-destructive default typography from a settings bundle's
/// *current* values (font family / line height / first-line indent). Size is
/// applied separately as editor zoom.
fn typo_defaults(typo: &EditorTypography) -> EditorTypographyDefaults {
    EditorTypographyDefaults {
        font_family: Some(typo.font_family.get()),
        line_height: typo.line_height.get(),
        first_line_indent: typo.first_line_indent.get(),
        paragraph_spacing_before: typo.para_spacing_before.get(),
        paragraph_spacing_after: typo.para_spacing_after.get(),
    }
}

/// Push `typo`'s current values onto a live editor: font / line-height / indent
/// as non-destructive defaults, size as zoom. Idempotent — called on mount and
/// on every settings change.
fn push_typography(handle: &EditorHandle, typo: &EditorTypography) {
    handle.set_typography_defaults(typo_defaults(typo));
    handle.set_zoom_level(typo.size.get());
}

/// Wraps a `RichTextEditor`, keeping its per-editor-type typography live for the
/// life of the tab. Initial values are already baked onto `editor` by the caller
/// (`typography_defaults` + `zoom`); this registers one `ctx.effect` per settings
/// field so a preference edit re-pushes the whole bundle through the editor
/// handle to every open tab. Four *separate* effects rather than one combined
/// `zip` signal — `zip`/`zip3` build a *derived* signal, which panics on
/// `.observe()`; the `SettingsStore` signals are mutable, so per-field effects
/// are safe.
struct TypographyBoundEditor {
    editor: Option<RichTextEditor>,
    typo: EditorTypography,
    child_id: Option<WidgetId>,
}

impl TypographyBoundEditor {
    fn new(editor: RichTextEditor, typo: EditorTypography) -> Self {
        Self {
            editor: Some(editor),
            typo,
            child_id: None,
        }
    }
}

impl std::fmt::Debug for TypographyBoundEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypographyBoundEditor")
            .finish_non_exhaustive()
    }
}

impl Widget for TypographyBoundEditor {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let editor = self
            .editor
            .take()
            .expect("TypographyBoundEditor built once");
        let handle = editor.handle();
        let id = ctx.add(editor);
        self.child_id = Some(id);
        // Any one field changing re-pushes the whole bundle (font/line/indent +
        // zoom) to the live editor. Separate effects — a combined `zip` signal is
        // derived and would panic on observe.
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.font_family, move |_| push_typography(&h, &t));
        }
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.size, move |_| push_typography(&h, &t));
        }
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.line_height, move |_| push_typography(&h, &t));
        }
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.first_line_indent, move |_| {
                push_typography(&h, &t)
            });
        }
        {
            let (h, t) = (handle.clone(), self.typo.clone());
            ctx.effect(&self.typo.para_spacing_before, move |_| {
                push_typography(&h, &t)
            });
        }
        {
            let (h, t) = (handle, self.typo.clone());
            ctx.effect(&self.typo.para_spacing_after, move |_| {
                push_typography(&h, &t)
            });
        }
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child_id
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
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
