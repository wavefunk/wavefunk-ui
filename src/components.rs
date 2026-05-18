use crate::assets;
use askama::Template;
use std::fmt::{self, Write as _};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HtmlAttr<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

impl<'a> HtmlAttr<'a> {
    pub const fn new(name: &'a str, value: &'a str) -> Self {
        Self { name, value }
    }

    pub const fn hx_get(value: &'a str) -> Self {
        Self::new("hx-get", value)
    }

    pub const fn hx_post(value: &'a str) -> Self {
        Self::new("hx-post", value)
    }

    pub const fn hx_put(value: &'a str) -> Self {
        Self::new("hx-put", value)
    }

    pub const fn hx_patch(value: &'a str) -> Self {
        Self::new("hx-patch", value)
    }

    pub const fn hx_delete(value: &'a str) -> Self {
        Self::new("hx-delete", value)
    }

    pub const fn hx_target(value: &'a str) -> Self {
        Self::new("hx-target", value)
    }

    pub const fn hx_swap(value: &'a str) -> Self {
        Self::new("hx-swap", value)
    }

    pub const fn hx_trigger(value: &'a str) -> Self {
        Self::new("hx-trigger", value)
    }

    pub const fn hx_confirm(value: &'a str) -> Self {
        Self::new("hx-confirm", value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrustedHtml<'a> {
    html: &'a str,
}

impl<'a> TrustedHtml<'a> {
    pub const fn new(html: &'a str) -> Self {
        Self { html }
    }

    pub const fn as_str(self) -> &'a str {
        self.html
    }
}

impl fmt::Display for TrustedHtml<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.html)
    }
}

impl askama::FastWritable for TrustedHtml<'_> {
    #[inline]
    fn write_into(&self, dest: &mut dyn fmt::Write, _: &dyn askama::Values) -> askama::Result<()> {
        Ok(dest.write_str(self.html)?)
    }
}

impl askama::filters::HtmlSafe for TrustedHtml<'_> {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustedHtmlBuf {
    html: String,
}

impl TrustedHtmlBuf {
    pub fn new(html: impl Into<String>) -> Self {
        Self { html: html.into() }
    }

    pub fn from_trusted(html: TrustedHtml<'_>) -> Self {
        Self::new(html.as_str())
    }

    pub fn as_str(&self) -> &str {
        &self.html
    }
}

impl From<TrustedHtml<'_>> for TrustedHtmlBuf {
    fn from(value: TrustedHtml<'_>) -> Self {
        Self::from_trusted(value)
    }
}

impl From<String> for TrustedHtmlBuf {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for TrustedHtmlBuf {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for TrustedHtmlBuf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.html)
    }
}

impl askama::FastWritable for TrustedHtmlBuf {
    #[inline]
    fn write_into(&self, dest: &mut dyn fmt::Write, _: &dyn askama::Values) -> askama::Result<()> {
        Ok(dest.write_str(&self.html)?)
    }
}

impl askama::filters::HtmlSafe for TrustedHtmlBuf {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonVariant {
    Default,
    Primary,
    Ghost,
    Danger,
}

impl ButtonVariant {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Primary => " primary",
            Self::Ghost => " ghost",
            Self::Danger => " danger",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonSize {
    Default,
    Small,
    Large,
}

impl ButtonSize {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Small => " sm",
            Self::Large => " lg",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/button.html")]
pub struct Button<'a> {
    pub label: &'a str,
    pub href: Option<&'a str>,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub attrs: &'a [HtmlAttr<'a>],
    pub disabled: bool,
    pub button_type: &'a str,
}

impl<'a> Button<'a> {
    pub const fn new(label: &'a str) -> Self {
        Self {
            label,
            href: None,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            attrs: &[],
            disabled: false,
            button_type: "button",
        }
    }

    pub const fn primary(label: &'a str) -> Self {
        Self {
            variant: ButtonVariant::Primary,
            ..Self::new(label)
        }
    }

    pub const fn link(label: &'a str, href: &'a str) -> Self {
        Self {
            href: Some(href),
            ..Self::new(label)
        }
    }

    pub const fn with_href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    pub const fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn with_button_type(mut self, button_type: &'a str) -> Self {
        self.button_type = button_type;
        self
    }

    pub fn class_name(&self) -> String {
        format!("wf-btn{}{}", self.variant.class(), self.size.class())
    }
}

impl<'a> askama::filters::HtmlSafe for Button<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeedbackKind {
    Info,
    Ok,
    Warn,
    Error,
}

impl FeedbackKind {
    fn class(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Ok => "ok",
            Self::Warn => "warn",
            Self::Error => "err",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/alert.html")]
pub struct Alert<'a> {
    pub kind: FeedbackKind,
    pub title: Option<&'a str>,
    pub message: &'a str,
}

impl<'a> Alert<'a> {
    pub const fn new(kind: FeedbackKind, message: &'a str) -> Self {
        Self {
            kind,
            title: None,
            message,
        }
    }

    pub const fn with_title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn class_name(&self) -> String {
        format!("wf-alert {}", self.kind.class())
    }
}

impl<'a> askama::filters::HtmlSafe for Alert<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/tag.html")]
pub struct Tag<'a> {
    pub kind: Option<FeedbackKind>,
    pub label: &'a str,
    pub dot: bool,
}

impl<'a> Tag<'a> {
    pub const fn new(label: &'a str) -> Self {
        Self {
            kind: None,
            label,
            dot: false,
        }
    }

    pub const fn status(kind: FeedbackKind, label: &'a str) -> Self {
        Self {
            kind: Some(kind),
            label,
            dot: true,
        }
    }

    pub const fn with_kind(mut self, kind: FeedbackKind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub const fn with_dot(mut self) -> Self {
        self.dot = true;
        self
    }

    pub fn class_name(&self) -> String {
        match self.kind {
            Some(kind) => format!("wf-tag {}", kind.class()),
            None => "wf-tag".to_owned(),
        }
    }
}

impl<'a> askama::filters::HtmlSafe for Tag<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldState {
    Default,
    Error,
    Success,
}

impl FieldState {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Error => " is-error",
            Self::Success => " is-success",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/field.html")]
pub struct Field<'a> {
    pub label: &'a str,
    pub control_html: TrustedHtml<'a>,
    pub hint: Option<&'a str>,
    pub state: FieldState,
}

impl<'a> Field<'a> {
    pub const fn new(label: &'a str, control_html: TrustedHtml<'a>) -> Self {
        Self {
            label,
            control_html,
            hint: None,
            state: FieldState::Default,
        }
    }

    pub const fn with_hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    pub const fn with_state(mut self, state: FieldState) -> Self {
        self.state = state;
        self
    }

    pub fn class_name(&self) -> String {
        format!("wf-field{}", self.state.class())
    }
}

impl<'a> askama::filters::HtmlSafe for Field<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/form.html")]
pub struct Form<'a> {
    pub body_html: TrustedHtml<'a>,
    pub action: Option<&'a str>,
    pub method: &'a str,
    pub enctype: Option<&'a str>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> Form<'a> {
    pub const fn new(body_html: TrustedHtml<'a>) -> Self {
        Self {
            body_html,
            action: None,
            method: "post",
            enctype: None,
            attrs: &[],
        }
    }

    pub const fn with_action(mut self, action: &'a str) -> Self {
        self.action = Some(action);
        self
    }

    pub const fn with_method(mut self, method: &'a str) -> Self {
        self.method = method;
        self
    }

    pub const fn with_enctype(mut self, enctype: &'a str) -> Self {
        self.enctype = Some(enctype);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for Form<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/form_section.html")]
pub struct FormSection<'a> {
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub description: Option<&'a str>,
    pub actions_html: Option<TrustedHtml<'a>>,
}

impl<'a> FormSection<'a> {
    pub const fn new(title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            body_html,
            description: None,
            actions_html: None,
        }
    }

    pub const fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn with_actions(mut self, actions_html: TrustedHtml<'a>) -> Self {
        self.actions_html = Some(actions_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for FormSection<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/form_actions.html")]
pub struct FormActions<'a> {
    pub primary_html: TrustedHtml<'a>,
    pub secondary_html: Option<TrustedHtml<'a>>,
}

impl<'a> FormActions<'a> {
    pub const fn new(primary_html: TrustedHtml<'a>) -> Self {
        Self {
            primary_html,
            secondary_html: None,
        }
    }

    pub const fn with_secondary(mut self, secondary_html: TrustedHtml<'a>) -> Self {
        self.secondary_html = Some(secondary_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for FormActions<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/object_fieldset.html")]
pub struct ObjectFieldset<'a> {
    pub legend: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub description: Option<&'a str>,
    pub actions_html: Option<TrustedHtml<'a>>,
}

impl<'a> ObjectFieldset<'a> {
    pub const fn new(legend: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            legend,
            body_html,
            description: None,
            actions_html: None,
        }
    }

    pub const fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn with_actions(mut self, actions_html: TrustedHtml<'a>) -> Self {
        self.actions_html = Some(actions_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for ObjectFieldset<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/repeatable_array.html")]
pub struct RepeatableArray<'a> {
    pub label: &'a str,
    pub items_html: TrustedHtml<'a>,
    pub description: Option<&'a str>,
    pub action_html: Option<TrustedHtml<'a>>,
}

impl<'a> RepeatableArray<'a> {
    pub const fn new(label: &'a str, items_html: TrustedHtml<'a>) -> Self {
        Self {
            label,
            items_html,
            description: None,
            action_html: None,
        }
    }

    pub const fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn with_action(mut self, action_html: TrustedHtml<'a>) -> Self {
        self.action_html = Some(action_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for RepeatableArray<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/repeatable_item.html")]
pub struct RepeatableItem<'a> {
    pub label: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub actions_html: Option<TrustedHtml<'a>>,
}

impl<'a> RepeatableItem<'a> {
    pub const fn new(label: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            label,
            body_html,
            actions_html: None,
        }
    }

    pub const fn with_actions(mut self, actions_html: TrustedHtml<'a>) -> Self {
        self.actions_html = Some(actions_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for RepeatableItem<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/current_upload.html")]
pub struct CurrentUpload<'a> {
    pub label: &'a str,
    pub href: &'a str,
    pub filename: &'a str,
    pub meta: Option<&'a str>,
    pub thumbnail_html: Option<TrustedHtml<'a>>,
    pub actions_html: Option<TrustedHtml<'a>>,
}

impl<'a> CurrentUpload<'a> {
    pub const fn new(label: &'a str, href: &'a str, filename: &'a str) -> Self {
        Self {
            label,
            href,
            filename,
            meta: None,
            thumbnail_html: None,
            actions_html: None,
        }
    }

    pub const fn with_meta(mut self, meta: &'a str) -> Self {
        self.meta = Some(meta);
        self
    }

    pub const fn with_thumbnail(mut self, thumbnail_html: TrustedHtml<'a>) -> Self {
        self.thumbnail_html = Some(thumbnail_html);
        self
    }

    pub const fn with_actions(mut self, actions_html: TrustedHtml<'a>) -> Self {
        self.actions_html = Some(actions_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for CurrentUpload<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/reference_select.html")]
pub struct ReferenceSelect<'a> {
    pub label: &'a str,
    pub select_html: TrustedHtml<'a>,
    pub hint: Option<&'a str>,
}

impl<'a> ReferenceSelect<'a> {
    pub const fn new(label: &'a str, select_html: TrustedHtml<'a>) -> Self {
        Self {
            label,
            select_html,
            hint: None,
        }
    }

    pub const fn with_hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for ReferenceSelect<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/markdown_textarea.html")]
pub struct MarkdownTextarea<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
    pub placeholder: Option<&'a str>,
    pub rows: u16,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> MarkdownTextarea<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
            value: None,
            placeholder: None,
            rows: 6,
            attrs: &[],
        }
    }

    pub const fn with_value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    pub const fn with_placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub const fn with_rows(mut self, rows: u16) -> Self {
        self.rows = rows;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for MarkdownTextarea<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/rich_text_host.html")]
pub struct RichTextHost<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub value: Option<&'a str>,
    pub toolbar_html: Option<TrustedHtml<'a>>,
    pub body_html: Option<TrustedHtml<'a>>,
}

impl<'a> RichTextHost<'a> {
    pub const fn new(id: &'a str, name: &'a str) -> Self {
        Self {
            id,
            name,
            value: None,
            toolbar_html: None,
            body_html: None,
        }
    }

    pub const fn with_value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    pub const fn with_toolbar(mut self, toolbar_html: TrustedHtml<'a>) -> Self {
        self.toolbar_html = Some(toolbar_html);
        self
    }

    pub const fn with_body(mut self, body_html: TrustedHtml<'a>) -> Self {
        self.body_html = Some(body_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for RichTextHost<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/button_group.html")]
pub struct ButtonGroup<'a> {
    pub buttons: &'a [Button<'a>],
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> ButtonGroup<'a> {
    pub const fn new(buttons: &'a [Button<'a>]) -> Self {
        Self {
            buttons,
            attrs: &[],
        }
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for ButtonGroup<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/split_button.html")]
pub struct SplitButton<'a> {
    pub action: Button<'a>,
    pub menu: Button<'a>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> SplitButton<'a> {
    pub const fn new(action: Button<'a>, menu: Button<'a>) -> Self {
        Self {
            action,
            menu,
            attrs: &[],
        }
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for SplitButton<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/icon_button.html")]
pub struct IconButton<'a> {
    pub icon: TrustedHtml<'a>,
    pub label: &'a str,
    pub href: Option<&'a str>,
    pub variant: ButtonVariant,
    pub attrs: &'a [HtmlAttr<'a>],
    pub disabled: bool,
    pub button_type: &'a str,
}

impl<'a> IconButton<'a> {
    pub const fn new(icon: TrustedHtml<'a>, label: &'a str) -> Self {
        Self {
            icon,
            label,
            href: None,
            variant: ButtonVariant::Default,
            attrs: &[],
            disabled: false,
            button_type: "button",
        }
    }

    pub const fn with_href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    pub const fn with_variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn with_button_type(mut self, button_type: &'a str) -> Self {
        self.button_type = button_type;
        self
    }

    pub fn class_name(&self) -> String {
        format!("wf-icon-btn{}", self.variant.class())
    }
}

impl<'a> askama::filters::HtmlSafe for IconButton<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlSize {
    Default,
    Small,
}

impl ControlSize {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Small => " sm",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/input.html")]
pub struct Input<'a> {
    pub name: &'a str,
    pub input_type: &'a str,
    pub value: Option<&'a str>,
    pub placeholder: Option<&'a str>,
    pub size: ControlSize,
    pub attrs: &'a [HtmlAttr<'a>],
    pub disabled: bool,
    pub required: bool,
}

impl<'a> Input<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
            input_type: "text",
            value: None,
            placeholder: None,
            size: ControlSize::Default,
            attrs: &[],
            disabled: false,
            required: false,
        }
    }

    pub const fn email(name: &'a str) -> Self {
        Self {
            input_type: "email",
            ..Self::new(name)
        }
    }

    pub const fn url(name: &'a str) -> Self {
        Self {
            input_type: "url",
            ..Self::new(name)
        }
    }

    pub const fn search(name: &'a str) -> Self {
        Self {
            input_type: "search",
            ..Self::new(name)
        }
    }

    pub const fn with_type(mut self, input_type: &'a str) -> Self {
        self.input_type = input_type;
        self
    }

    pub const fn with_value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    pub const fn with_placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub const fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn class_name(&self) -> String {
        format!("wf-input{}", self.size.class())
    }
}

impl<'a> askama::filters::HtmlSafe for Input<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/textarea.html")]
pub struct Textarea<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
    pub placeholder: Option<&'a str>,
    pub rows: Option<u16>,
    pub attrs: &'a [HtmlAttr<'a>],
    pub disabled: bool,
    pub required: bool,
}

impl<'a> Textarea<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
            value: None,
            placeholder: None,
            rows: None,
            attrs: &[],
            disabled: false,
            required: false,
        }
    }

    pub const fn with_value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    pub const fn with_placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub const fn with_rows(mut self, rows: u16) -> Self {
        self.rows = Some(rows);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for Textarea<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SelectOption<'a> {
    pub value: &'a str,
    pub label: &'a str,
    pub selected: bool,
    pub disabled: bool,
}

impl<'a> SelectOption<'a> {
    pub const fn new(value: &'a str, label: &'a str) -> Self {
        Self {
            value,
            label,
            selected: false,
            disabled: false,
        }
    }

    pub const fn selected(mut self) -> Self {
        self.selected = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/select.html")]
pub struct Select<'a> {
    pub name: &'a str,
    pub options: &'a [SelectOption<'a>],
    pub size: ControlSize,
    pub attrs: &'a [HtmlAttr<'a>],
    pub disabled: bool,
    pub required: bool,
}

impl<'a> Select<'a> {
    pub const fn new(name: &'a str, options: &'a [SelectOption<'a>]) -> Self {
        Self {
            name,
            options,
            size: ControlSize::Default,
            attrs: &[],
            disabled: false,
            required: false,
        }
    }

    pub const fn with_size(mut self, size: ControlSize) -> Self {
        self.size = size;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }

    pub fn class_name(&self) -> String {
        format!("wf-select{}", self.size.class())
    }
}

impl<'a> askama::filters::HtmlSafe for Select<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/input_group.html")]
pub struct InputGroup<'a> {
    pub control_html: TrustedHtml<'a>,
    pub prefix: Option<&'a str>,
    pub suffix: Option<&'a str>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> InputGroup<'a> {
    pub const fn new(control_html: TrustedHtml<'a>) -> Self {
        Self {
            control_html,
            prefix: None,
            suffix: None,
            attrs: &[],
        }
    }

    pub const fn with_prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub const fn with_suffix(mut self, suffix: &'a str) -> Self {
        self.suffix = Some(suffix);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for InputGroup<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckKind {
    Checkbox,
    Radio,
}

impl CheckKind {
    fn input_type(self) -> &'static str {
        match self {
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/check_row.html")]
pub struct CheckRow<'a> {
    pub kind: CheckKind,
    pub name: &'a str,
    pub value: &'a str,
    pub label: &'a str,
    pub attrs: &'a [HtmlAttr<'a>],
    pub checked: bool,
    pub disabled: bool,
}

impl<'a> CheckRow<'a> {
    pub const fn checkbox(name: &'a str, value: &'a str, label: &'a str) -> Self {
        Self {
            kind: CheckKind::Checkbox,
            name,
            value,
            label,
            attrs: &[],
            checked: false,
            disabled: false,
        }
    }

    pub const fn radio(name: &'a str, value: &'a str, label: &'a str) -> Self {
        Self {
            kind: CheckKind::Radio,
            ..Self::checkbox(name, value, label)
        }
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn input_type(&self) -> &'static str {
        self.kind.input_type()
    }
}

impl<'a> askama::filters::HtmlSafe for CheckRow<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/switch.html")]
pub struct Switch<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub attrs: &'a [HtmlAttr<'a>],
    pub checked: bool,
    pub disabled: bool,
}

impl<'a> Switch<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
            value: "on",
            attrs: &[],
            checked: false,
            disabled: false,
        }
    }

    pub const fn with_value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for Switch<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/range.html")]
pub struct Range<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
    pub min: Option<&'a str>,
    pub max: Option<&'a str>,
    pub step: Option<&'a str>,
    pub attrs: &'a [HtmlAttr<'a>],
    pub disabled: bool,
}

impl<'a> Range<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
            value: None,
            min: None,
            max: None,
            step: None,
            attrs: &[],
            disabled: false,
        }
    }

    pub const fn with_value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    pub const fn with_bounds(mut self, min: &'a str, max: &'a str) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    pub const fn with_step(mut self, step: &'a str) -> Self {
        self.step = Some(step);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for Range<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/dropzone.html")]
pub struct Dropzone<'a> {
    pub name: &'a str,
    pub title: &'a str,
    pub hint: Option<&'a str>,
    pub accept: Option<&'a str>,
    pub attrs: &'a [HtmlAttr<'a>],
    pub multiple: bool,
    pub disabled: bool,
    pub dragover: bool,
}

impl<'a> Dropzone<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
            title: "Drop files or click",
            hint: None,
            accept: None,
            attrs: &[],
            multiple: false,
            disabled: false,
            dragover: false,
        }
    }

    pub const fn with_title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub const fn with_hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    pub const fn with_accept(mut self, accept: &'a str) -> Self {
        self.accept = Some(accept);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn dragover(mut self) -> Self {
        self.dragover = true;
        self
    }

    pub fn class_name(&self) -> String {
        let dragover = if self.dragover { " is-dragover" } else { "" };
        let disabled = if self.disabled { " is-disabled" } else { "" };
        format!("wf-dropzone{dragover}{disabled}")
    }
}

impl<'a> askama::filters::HtmlSafe for Dropzone<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/panel.html")]
pub struct Panel<'a> {
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub action_html: Option<TrustedHtml<'a>>,
    pub danger: bool,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> Panel<'a> {
    pub const fn new(title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            body_html,
            action_html: None,
            danger: false,
            attrs: &[],
        }
    }

    pub const fn with_action(mut self, action_html: TrustedHtml<'a>) -> Self {
        self.action_html = Some(action_html);
        self
    }

    pub const fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.danger {
            "wf-panel is-danger"
        } else {
            "wf-panel"
        }
    }
}

impl<'a> askama::filters::HtmlSafe for Panel<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/form_panel.html")]
pub struct FormPanel<'a> {
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub subtitle: Option<&'a str>,
    pub actions_html: Option<TrustedHtml<'a>>,
    pub meta_html: Option<TrustedHtml<'a>>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> FormPanel<'a> {
    pub const fn new(title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            body_html,
            subtitle: None,
            actions_html: None,
            meta_html: None,
            attrs: &[],
        }
    }

    pub const fn with_subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub const fn with_actions(mut self, actions_html: TrustedHtml<'a>) -> Self {
        self.actions_html = Some(actions_html);
        self
    }

    pub const fn with_meta(mut self, meta_html: TrustedHtml<'a>) -> Self {
        self.meta_html = Some(meta_html);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for FormPanel<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/split_shell.html")]
pub struct SplitShell<'a> {
    pub content_html: TrustedHtml<'a>,
    pub visual_html: Option<TrustedHtml<'a>>,
    pub top_html: Option<TrustedHtml<'a>>,
    pub footer_html: Option<TrustedHtml<'a>>,
    pub mode: Option<&'a str>,
    pub mode_locked: bool,
    pub asset_base_path: &'a str,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> SplitShell<'a> {
    pub const fn new(content_html: TrustedHtml<'a>) -> Self {
        Self {
            content_html,
            visual_html: None,
            top_html: None,
            footer_html: None,
            mode: None,
            mode_locked: false,
            asset_base_path: assets::DEFAULT_BASE_PATH,
            attrs: &[],
        }
    }

    pub const fn with_visual(mut self, visual_html: TrustedHtml<'a>) -> Self {
        self.visual_html = Some(visual_html);
        self
    }

    pub const fn with_top(mut self, top_html: TrustedHtml<'a>) -> Self {
        self.top_html = Some(top_html);
        self
    }

    pub const fn with_footer(mut self, footer_html: TrustedHtml<'a>) -> Self {
        self.footer_html = Some(footer_html);
        self
    }

    pub const fn with_mode(mut self, mode: &'a str) -> Self {
        self.mode = Some(mode);
        self
    }

    pub const fn mode_locked(mut self) -> Self {
        self.mode_locked = true;
        self
    }

    pub const fn with_asset_base_path(mut self, asset_base_path: &'a str) -> Self {
        self.asset_base_path = asset_base_path;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for SplitShell<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/settings_section.html")]
pub struct SettingsSection<'a> {
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub description: Option<&'a str>,
    pub action_html: Option<TrustedHtml<'a>>,
    pub danger: bool,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> SettingsSection<'a> {
    pub const fn new(title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            body_html,
            description: None,
            action_html: None,
            danger: false,
            attrs: &[],
        }
    }

    pub const fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn with_action(mut self, action_html: TrustedHtml<'a>) -> Self {
        self.action_html = Some(action_html);
        self
    }

    pub const fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.danger {
            "wf-panel wf-settings-section is-danger"
        } else {
            "wf-panel wf-settings-section"
        }
    }
}

impl<'a> askama::filters::HtmlSafe for SettingsSection<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/inline_form_row.html")]
pub struct InlineFormRow<'a> {
    pub label: &'a str,
    pub control_html: TrustedHtml<'a>,
    pub hint: Option<&'a str>,
    pub action_html: Option<TrustedHtml<'a>>,
}

impl<'a> InlineFormRow<'a> {
    pub const fn new(label: &'a str, control_html: TrustedHtml<'a>) -> Self {
        Self {
            label,
            control_html,
            hint: None,
            action_html: None,
        }
    }

    pub const fn with_hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    pub const fn with_action(mut self, action_html: TrustedHtml<'a>) -> Self {
        self.action_html = Some(action_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for InlineFormRow<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/copyable_value.html")]
pub struct CopyableValue<'a> {
    pub label: &'a str,
    pub id: &'a str,
    pub value: &'a str,
    pub button_label: &'a str,
    pub secret: bool,
}

impl<'a> CopyableValue<'a> {
    pub const fn new(label: &'a str, id: &'a str, value: &'a str) -> Self {
        Self {
            label,
            id,
            value,
            button_label: "Copy",
            secret: false,
        }
    }

    pub const fn with_button_label(mut self, button_label: &'a str) -> Self {
        self.button_label = button_label;
        self
    }

    pub const fn secret(mut self) -> Self {
        self.secret = true;
        self
    }

    pub fn value_class(&self) -> &'static str {
        if self.secret {
            "wf-copyable-value is-secret"
        } else {
            "wf-copyable-value"
        }
    }
}

impl<'a> askama::filters::HtmlSafe for CopyableValue<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/secret_value.html")]
pub struct SecretValue<'a> {
    pub label: &'a str,
    pub id: &'a str,
    pub value: &'a str,
    pub button_label: &'a str,
    pub revealed: bool,
    pub copy_raw_value: bool,
    pub warning: Option<&'a str>,
    pub help_html: Option<TrustedHtml<'a>>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> SecretValue<'a> {
    pub const fn new(label: &'a str, id: &'a str, value: &'a str) -> Self {
        Self {
            label,
            id,
            value,
            button_label: "Copy",
            revealed: false,
            copy_raw_value: false,
            warning: None,
            help_html: None,
            attrs: &[],
        }
    }

    pub const fn revealed(mut self) -> Self {
        self.revealed = true;
        self
    }

    pub const fn copy_raw_value(mut self) -> Self {
        self.copy_raw_value = true;
        self
    }

    pub const fn with_button_label(mut self, button_label: &'a str) -> Self {
        self.button_label = button_label;
        self
    }

    pub const fn with_warning(mut self, warning: &'a str) -> Self {
        self.warning = Some(warning);
        self
    }

    pub const fn with_help(mut self, help_html: TrustedHtml<'a>) -> Self {
        self.help_html = Some(help_html);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn display_value(&self) -> &str {
        if self.revealed {
            self.value
        } else {
            "********"
        }
    }

    pub fn value_class(&self) -> &'static str {
        if self.revealed {
            "wf-secret-code is-revealed"
        } else {
            "wf-secret-code is-masked"
        }
    }
}

impl<'a> askama::filters::HtmlSafe for SecretValue<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChecklistItem<'a> {
    pub label: &'a str,
    pub description: Option<&'a str>,
    pub kind: FeedbackKind,
    pub status_label: Option<&'a str>,
    pub icon_html: Option<TrustedHtml<'a>>,
}

impl<'a> ChecklistItem<'a> {
    pub const fn new(label: &'a str, kind: FeedbackKind) -> Self {
        Self {
            label,
            description: None,
            kind,
            status_label: None,
            icon_html: None,
        }
    }

    pub const fn info(label: &'a str) -> Self {
        Self::new(label, FeedbackKind::Info)
    }

    pub const fn ok(label: &'a str) -> Self {
        Self::new(label, FeedbackKind::Ok)
    }

    pub const fn warn(label: &'a str) -> Self {
        Self::new(label, FeedbackKind::Warn)
    }

    pub const fn error(label: &'a str) -> Self {
        Self::new(label, FeedbackKind::Error)
    }

    pub const fn with_description(mut self, description: &'a str) -> Self {
        self.description = Some(description);
        self
    }

    pub const fn with_status_label(mut self, status_label: &'a str) -> Self {
        self.status_label = Some(status_label);
        self
    }

    pub const fn with_icon(mut self, icon_html: TrustedHtml<'a>) -> Self {
        self.icon_html = Some(icon_html);
        self
    }

    pub fn class_name(&self) -> &'static str {
        match self.kind {
            FeedbackKind::Info => "wf-checklist-item is-info",
            FeedbackKind::Ok => "wf-checklist-item is-ok",
            FeedbackKind::Warn => "wf-checklist-item is-warn",
            FeedbackKind::Error => "wf-checklist-item is-err",
        }
    }

    pub fn status_text(&self) -> &'a str {
        self.status_label.unwrap_or(match self.kind {
            FeedbackKind::Info => "info",
            FeedbackKind::Ok => "ok",
            FeedbackKind::Warn => "warn",
            FeedbackKind::Error => "error",
        })
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/checklist.html")]
pub struct Checklist<'a> {
    pub items: &'a [ChecklistItem<'a>],
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> Checklist<'a> {
    pub const fn new(items: &'a [ChecklistItem<'a>]) -> Self {
        Self { items, attrs: &[] }
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for Checklist<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/code_grid.html")]
pub struct CodeGrid<'a> {
    pub codes: &'a [&'a str],
    pub label: Option<&'a str>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> CodeGrid<'a> {
    pub const fn new(codes: &'a [&'a str]) -> Self {
        Self {
            codes,
            label: None,
            attrs: &[],
        }
    }

    pub const fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for CodeGrid<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CredentialStatusItem<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub kind: FeedbackKind,
    pub status_label: &'a str,
}

impl<'a> CredentialStatusItem<'a> {
    pub const fn new(
        label: &'a str,
        value: &'a str,
        kind: FeedbackKind,
        status_label: &'a str,
    ) -> Self {
        Self {
            label,
            value,
            kind,
            status_label,
        }
    }

    pub const fn ok(label: &'a str, value: &'a str) -> Self {
        Self::new(label, value, FeedbackKind::Ok, "ok")
    }

    pub const fn warn(label: &'a str, value: &'a str) -> Self {
        Self::new(label, value, FeedbackKind::Warn, "warn")
    }

    pub const fn error(label: &'a str, value: &'a str) -> Self {
        Self::new(label, value, FeedbackKind::Error, "error")
    }

    pub const fn info(label: &'a str, value: &'a str) -> Self {
        Self::new(label, value, FeedbackKind::Info, "info")
    }

    pub fn kind_class(&self) -> String {
        format!("wf-tag {}", self.kind.class())
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/credential_status_list.html")]
pub struct CredentialStatusList<'a> {
    pub items: &'a [CredentialStatusItem<'a>],
}

impl<'a> CredentialStatusList<'a> {
    pub const fn new(items: &'a [CredentialStatusItem<'a>]) -> Self {
        Self { items }
    }
}

impl<'a> askama::filters::HtmlSafe for CredentialStatusList<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/confirm_action.html")]
pub struct ConfirmAction<'a> {
    pub label: &'a str,
    pub action: &'a str,
    pub method: &'a str,
    pub message: Option<&'a str>,
    pub confirm: Option<&'a str>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> ConfirmAction<'a> {
    pub const fn new(label: &'a str, action: &'a str) -> Self {
        Self {
            label,
            action,
            method: "post",
            message: None,
            confirm: None,
            attrs: &[],
        }
    }

    pub const fn with_method(mut self, method: &'a str) -> Self {
        self.method = method;
        self
    }

    pub const fn with_message(mut self, message: &'a str) -> Self {
        self.message = Some(message);
        self
    }

    pub const fn with_confirm(mut self, confirm: &'a str) -> Self {
        self.confirm = Some(confirm);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for ConfirmAction<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/card.html")]
pub struct Card<'a> {
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub kicker: Option<&'a str>,
    pub foot_html: Option<TrustedHtml<'a>>,
    pub raised: bool,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> Card<'a> {
    pub const fn new(title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            body_html,
            kicker: None,
            foot_html: None,
            raised: false,
            attrs: &[],
        }
    }

    pub const fn with_kicker(mut self, kicker: &'a str) -> Self {
        self.kicker = Some(kicker);
        self
    }

    pub const fn with_foot(mut self, foot_html: TrustedHtml<'a>) -> Self {
        self.foot_html = Some(foot_html);
        self
    }

    pub const fn raised(mut self) -> Self {
        self.raised = true;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.raised {
            "wf-card is-raised"
        } else {
            "wf-card"
        }
    }
}

impl<'a> askama::filters::HtmlSafe for Card<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BadgeKind {
    Default,
    Muted,
    Error,
}

impl BadgeKind {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Muted => " muted",
            Self::Error => " err",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/badge.html")]
pub struct Badge<'a> {
    pub label: &'a str,
    pub kind: BadgeKind,
}

impl<'a> Badge<'a> {
    pub const fn new(label: &'a str) -> Self {
        Self {
            label,
            kind: BadgeKind::Default,
        }
    }

    pub const fn muted(label: &'a str) -> Self {
        Self {
            kind: BadgeKind::Muted,
            ..Self::new(label)
        }
    }

    pub const fn error(label: &'a str) -> Self {
        Self {
            kind: BadgeKind::Error,
            ..Self::new(label)
        }
    }

    pub fn class_name(&self) -> String {
        format!("wf-badge{}", self.kind.class())
    }
}

impl<'a> askama::filters::HtmlSafe for Badge<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AvatarSize {
    Default,
    Small,
    Large,
    ExtraLarge,
}

impl AvatarSize {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Small => " sm",
            Self::Large => " lg",
            Self::ExtraLarge => " xl",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/avatar.html")]
pub struct Avatar<'a> {
    pub initials: &'a str,
    pub image_src: Option<&'a str>,
    pub size: AvatarSize,
    pub accent: bool,
}

impl<'a> Avatar<'a> {
    pub const fn new(initials: &'a str) -> Self {
        Self {
            initials,
            image_src: None,
            size: AvatarSize::Default,
            accent: false,
        }
    }

    pub const fn with_image(mut self, image_src: &'a str) -> Self {
        self.image_src = Some(image_src);
        self
    }

    pub const fn with_size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub const fn accent(mut self) -> Self {
        self.accent = true;
        self
    }

    pub fn class_name(&self) -> String {
        let accent = if self.accent { " accent" } else { "" };
        format!("wf-avatar{}{}", self.size.class(), accent)
    }
}

impl<'a> askama::filters::HtmlSafe for Avatar<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/avatar_group.html")]
pub struct AvatarGroup<'a> {
    pub avatars: &'a [Avatar<'a>],
}

impl<'a> AvatarGroup<'a> {
    pub const fn new(avatars: &'a [Avatar<'a>]) -> Self {
        Self { avatars }
    }
}

impl<'a> askama::filters::HtmlSafe for AvatarGroup<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/user_button.html")]
pub struct UserButton<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub avatar: Avatar<'a>,
    pub compact: bool,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> UserButton<'a> {
    pub const fn new(name: &'a str, email: &'a str, avatar: Avatar<'a>) -> Self {
        Self {
            name,
            email,
            avatar,
            compact: false,
            attrs: &[],
        }
    }

    pub const fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.compact {
            "wf-user compact"
        } else {
            "wf-user"
        }
    }
}

impl<'a> askama::filters::HtmlSafe for UserButton<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/wordmark.html")]
pub struct Wordmark<'a> {
    pub name: &'a str,
    pub mark_html: Option<TrustedHtml<'a>>,
}

impl<'a> Wordmark<'a> {
    pub const fn new(name: &'a str) -> Self {
        Self {
            name,
            mark_html: None,
        }
    }

    pub const fn with_mark(mut self, mark_html: TrustedHtml<'a>) -> Self {
        self.mark_html = Some(mark_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for Wordmark<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeltaKind {
    Neutral,
    Up,
    Down,
}

impl DeltaKind {
    fn class(self) -> &'static str {
        match self {
            Self::Neutral => "",
            Self::Up => " up",
            Self::Down => " down",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Stat<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub unit: Option<&'a str>,
    pub delta: Option<&'a str>,
    pub delta_kind: DeltaKind,
    pub foot: Option<&'a str>,
}

impl<'a> Stat<'a> {
    pub const fn new(label: &'a str, value: &'a str) -> Self {
        Self {
            label,
            value,
            unit: None,
            delta: None,
            delta_kind: DeltaKind::Neutral,
            foot: None,
        }
    }

    pub const fn with_unit(mut self, unit: &'a str) -> Self {
        self.unit = Some(unit);
        self
    }

    pub const fn with_delta(mut self, delta: &'a str, kind: DeltaKind) -> Self {
        self.delta = Some(delta);
        self.delta_kind = kind;
        self
    }

    pub const fn with_foot(mut self, foot: &'a str) -> Self {
        self.foot = Some(foot);
        self
    }

    pub fn delta_class(&self) -> String {
        format!("wf-stat-delta{}", self.delta_kind.class())
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/stat_row.html")]
pub struct StatRow<'a> {
    pub stats: &'a [Stat<'a>],
}

impl<'a> StatRow<'a> {
    pub const fn new(stats: &'a [Stat<'a>]) -> Self {
        Self { stats }
    }
}

impl<'a> askama::filters::HtmlSafe for StatRow<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BreadcrumbItem<'a> {
    pub label: &'a str,
    pub href: Option<&'a str>,
    pub current: bool,
}

impl<'a> BreadcrumbItem<'a> {
    pub const fn link(label: &'a str, href: &'a str) -> Self {
        Self {
            label,
            href: Some(href),
            current: false,
        }
    }

    pub const fn current(label: &'a str) -> Self {
        Self {
            label,
            href: None,
            current: true,
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/breadcrumbs.html")]
pub struct Breadcrumbs<'a> {
    pub items: &'a [BreadcrumbItem<'a>],
}

impl<'a> Breadcrumbs<'a> {
    pub const fn new(items: &'a [BreadcrumbItem<'a>]) -> Self {
        Self { items }
    }
}

impl<'a> askama::filters::HtmlSafe for Breadcrumbs<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/page_header.html")]
pub struct PageHeader<'a> {
    pub title: &'a str,
    pub subtitle: Option<&'a str>,
    pub back_href: Option<&'a str>,
    pub back_label: &'a str,
    pub meta_html: Option<TrustedHtml<'a>>,
    pub primary_html: Option<TrustedHtml<'a>>,
    pub secondary_html: Option<TrustedHtml<'a>>,
}

impl<'a> PageHeader<'a> {
    pub const fn new(title: &'a str) -> Self {
        Self {
            title,
            subtitle: None,
            back_href: None,
            back_label: "Back",
            meta_html: None,
            primary_html: None,
            secondary_html: None,
        }
    }

    pub const fn with_subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub const fn with_back(mut self, href: &'a str, label: &'a str) -> Self {
        self.back_href = Some(href);
        self.back_label = label;
        self
    }

    pub const fn with_meta(mut self, meta_html: TrustedHtml<'a>) -> Self {
        self.meta_html = Some(meta_html);
        self
    }

    pub const fn with_primary(mut self, primary_html: TrustedHtml<'a>) -> Self {
        self.primary_html = Some(primary_html);
        self
    }

    pub const fn with_secondary(mut self, secondary_html: TrustedHtml<'a>) -> Self {
        self.secondary_html = Some(secondary_html);
        self
    }

    pub const fn has_actions(&self) -> bool {
        self.primary_html.is_some() || self.secondary_html.is_some()
    }
}

impl<'a> askama::filters::HtmlSafe for PageHeader<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TabItem<'a> {
    pub label: &'a str,
    pub href: &'a str,
    pub active: bool,
}

impl<'a> TabItem<'a> {
    pub const fn link(label: &'a str, href: &'a str) -> Self {
        Self {
            label,
            href,
            active: false,
        }
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/tabs.html")]
pub struct Tabs<'a> {
    pub items: &'a [TabItem<'a>],
}

impl<'a> Tabs<'a> {
    pub const fn new(items: &'a [TabItem<'a>]) -> Self {
        Self { items }
    }
}

impl<'a> askama::filters::HtmlSafe for Tabs<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentOption<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub active: bool,
}

impl<'a> SegmentOption<'a> {
    pub const fn new(label: &'a str, value: &'a str) -> Self {
        Self {
            label,
            value,
            active: false,
        }
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/segmented_control.html")]
pub struct SegmentedControl<'a> {
    pub options: &'a [SegmentOption<'a>],
    pub small: bool,
}

impl<'a> SegmentedControl<'a> {
    pub const fn new(options: &'a [SegmentOption<'a>]) -> Self {
        Self {
            options,
            small: false,
        }
    }

    pub const fn small(mut self) -> Self {
        self.small = true;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.small { "wf-seg sm" } else { "wf-seg" }
    }
}

impl<'a> askama::filters::HtmlSafe for SegmentedControl<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageLink<'a> {
    pub label: &'a str,
    pub href: Option<&'a str>,
    pub active: bool,
    pub disabled: bool,
    pub ellipsis: bool,
}

impl<'a> PageLink<'a> {
    pub const fn link(label: &'a str, href: &'a str) -> Self {
        Self {
            label,
            href: Some(href),
            active: false,
            disabled: false,
            ellipsis: false,
        }
    }

    pub const fn disabled(label: &'a str) -> Self {
        Self {
            label,
            href: None,
            active: false,
            disabled: true,
            ellipsis: false,
        }
    }

    pub const fn ellipsis() -> Self {
        Self {
            label: "...",
            href: None,
            active: false,
            disabled: false,
            ellipsis: true,
        }
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/pagination.html")]
pub struct Pagination<'a> {
    pub pages: &'a [PageLink<'a>],
}

impl<'a> Pagination<'a> {
    pub const fn new(pages: &'a [PageLink<'a>]) -> Self {
        Self { pages }
    }
}

impl<'a> askama::filters::HtmlSafe for Pagination<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StepState {
    Upcoming,
    Active,
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StepItem<'a> {
    pub label: &'a str,
    pub href: Option<&'a str>,
    pub state: StepState,
}

impl<'a> StepItem<'a> {
    pub const fn new(label: &'a str) -> Self {
        Self {
            label,
            href: None,
            state: StepState::Upcoming,
        }
    }

    pub const fn with_href(mut self, href: &'a str) -> Self {
        self.href = Some(href);
        self
    }

    pub const fn active(mut self) -> Self {
        self.state = StepState::Active;
        self
    }

    pub const fn done(mut self) -> Self {
        self.state = StepState::Done;
        self
    }

    pub fn class_name(&self) -> &'static str {
        match self.state {
            StepState::Upcoming => "wf-step",
            StepState::Active => "wf-step is-active",
            StepState::Done => "wf-step is-done",
        }
    }

    pub fn is_active(&self) -> bool {
        self.state == StepState::Active
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/stepper.html")]
pub struct Stepper<'a> {
    pub steps: &'a [StepItem<'a>],
}

impl<'a> Stepper<'a> {
    pub const fn new(steps: &'a [StepItem<'a>]) -> Self {
        Self { steps }
    }
}

impl<'a> askama::filters::HtmlSafe for Stepper<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccordionItem<'a> {
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub open: bool,
}

impl<'a> AccordionItem<'a> {
    pub const fn new(title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            body_html,
            open: false,
        }
    }

    pub const fn open(mut self) -> Self {
        self.open = true;
        self
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/accordion.html")]
pub struct Accordion<'a> {
    pub items: &'a [AccordionItem<'a>],
}

impl<'a> Accordion<'a> {
    pub const fn new(items: &'a [AccordionItem<'a>]) -> Self {
        Self { items }
    }
}

impl<'a> askama::filters::HtmlSafe for Accordion<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FaqItem<'a> {
    pub question: &'a str,
    pub answer_html: TrustedHtml<'a>,
}

impl<'a> FaqItem<'a> {
    pub const fn new(question: &'a str, answer_html: TrustedHtml<'a>) -> Self {
        Self {
            question,
            answer_html,
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/faq.html")]
pub struct Faq<'a> {
    pub items: &'a [FaqItem<'a>],
}

impl<'a> Faq<'a> {
    pub const fn new(items: &'a [FaqItem<'a>]) -> Self {
        Self { items }
    }
}

impl<'a> askama::filters::HtmlSafe for Faq<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/nav_section.html")]
pub struct NavSection<'a> {
    pub label: &'a str,
}

impl<'a> NavSection<'a> {
    pub const fn new(label: &'a str) -> Self {
        Self { label }
    }
}

impl<'a> askama::filters::HtmlSafe for NavSection<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/nav_item.html")]
pub struct NavItem<'a> {
    pub label: &'a str,
    pub href: &'a str,
    pub count: Option<&'a str>,
    pub active: bool,
}

impl<'a> NavItem<'a> {
    pub const fn new(label: &'a str, href: &'a str) -> Self {
        Self {
            label,
            href,
            count: None,
            active: false,
        }
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }

    pub const fn with_count(mut self, count: &'a str) -> Self {
        self.count = Some(count);
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.active {
            "wf-nav-item is-active"
        } else {
            "wf-nav-item"
        }
    }
}

impl<'a> askama::filters::HtmlSafe for NavItem<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContextSwitcherItem<'a> {
    pub label: &'a str,
    pub href: &'a str,
    pub meta: Option<&'a str>,
    pub badge_html: Option<TrustedHtml<'a>>,
    pub active: bool,
    pub disabled: bool,
}

impl<'a> ContextSwitcherItem<'a> {
    pub const fn link(label: &'a str, href: &'a str) -> Self {
        Self {
            label,
            href,
            meta: None,
            badge_html: None,
            active: false,
            disabled: false,
        }
    }

    pub const fn with_meta(mut self, meta: &'a str) -> Self {
        self.meta = Some(meta);
        self
    }

    pub const fn with_badge(mut self, badge_html: TrustedHtml<'a>) -> Self {
        self.badge_html = Some(badge_html);
        self
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn class_name(&self) -> &'static str {
        match (self.active, self.disabled) {
            (true, true) => "wf-context-switcher-item is-active is-disabled",
            (true, false) => "wf-context-switcher-item is-active",
            (false, true) => "wf-context-switcher-item is-disabled",
            (false, false) => "wf-context-switcher-item",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/context_switcher.html")]
pub struct ContextSwitcher<'a> {
    pub label: &'a str,
    pub current: &'a str,
    pub items: &'a [ContextSwitcherItem<'a>],
    pub meta_html: Option<TrustedHtml<'a>>,
    pub open: bool,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> ContextSwitcher<'a> {
    pub const fn new(
        label: &'a str,
        current: &'a str,
        items: &'a [ContextSwitcherItem<'a>],
    ) -> Self {
        Self {
            label,
            current,
            items,
            meta_html: None,
            open: false,
            attrs: &[],
        }
    }

    pub const fn with_meta(mut self, meta_html: TrustedHtml<'a>) -> Self {
        self.meta_html = Some(meta_html);
        self
    }

    pub const fn open(mut self) -> Self {
        self.open = true;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for ContextSwitcher<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SidenavItem<'a> {
    pub label: &'a str,
    pub href: &'a str,
    pub badge: Option<&'a str>,
    pub coming_soon: Option<&'a str>,
    pub active: bool,
    pub muted: bool,
    pub disabled: bool,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> SidenavItem<'a> {
    pub const fn link(label: &'a str, href: &'a str) -> Self {
        Self {
            label,
            href,
            badge: None,
            coming_soon: None,
            active: false,
            muted: false,
            disabled: false,
            attrs: &[],
        }
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }

    pub const fn muted(mut self) -> Self {
        self.muted = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn with_badge(mut self, badge: &'a str) -> Self {
        self.badge = Some(badge);
        self
    }

    pub const fn with_coming_soon(mut self, coming_soon: &'a str) -> Self {
        self.coming_soon = Some(coming_soon);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub fn class_name(&self) -> &'static str {
        match (self.active, self.muted, self.disabled) {
            (true, true, true) => "wf-sidenav-item is-active is-muted is-disabled",
            (true, true, false) => "wf-sidenav-item is-active is-muted",
            (true, false, true) => "wf-sidenav-item is-active is-disabled",
            (true, false, false) => "wf-sidenav-item is-active",
            (false, true, true) => "wf-sidenav-item is-muted is-disabled",
            (false, true, false) => "wf-sidenav-item is-muted",
            (false, false, true) => "wf-sidenav-item is-disabled",
            (false, false, false) => "wf-sidenav-item",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SidenavSection<'a> {
    pub label: &'a str,
    pub items: &'a [SidenavItem<'a>],
}

impl<'a> SidenavSection<'a> {
    pub const fn new(label: &'a str, items: &'a [SidenavItem<'a>]) -> Self {
        Self { label, items }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/sidenav.html")]
pub struct Sidenav<'a> {
    pub sections: &'a [SidenavSection<'a>],
    pub attrs: &'a [HtmlAttr<'a>],
    pub landmark: bool,
}

impl<'a> Sidenav<'a> {
    pub const fn new(sections: &'a [SidenavSection<'a>]) -> Self {
        Self {
            sections,
            attrs: &[],
            landmark: true,
        }
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn embedded(mut self) -> Self {
        self.landmark = false;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for Sidenav<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/topbar.html")]
pub struct Topbar<'a> {
    pub breadcrumbs_html: TrustedHtml<'a>,
    pub actions_html: TrustedHtml<'a>,
}

impl<'a> Topbar<'a> {
    pub const fn new(breadcrumbs_html: TrustedHtml<'a>, actions_html: TrustedHtml<'a>) -> Self {
        Self {
            breadcrumbs_html,
            actions_html,
        }
    }
}

impl<'a> askama::filters::HtmlSafe for Topbar<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/statusbar.html")]
pub struct Statusbar<'a> {
    pub left: &'a str,
    pub right: &'a str,
}

impl<'a> Statusbar<'a> {
    pub const fn new(left: &'a str, right: &'a str) -> Self {
        Self { left, right }
    }
}

impl<'a> askama::filters::HtmlSafe for Statusbar<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/empty_state.html")]
pub struct EmptyState<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub glyph_html: Option<TrustedHtml<'a>>,
    pub actions_html: Option<TrustedHtml<'a>>,
    pub bordered: bool,
    pub dense: bool,
}

impl<'a> EmptyState<'a> {
    pub const fn new(title: &'a str, body: &'a str) -> Self {
        Self {
            title,
            body,
            glyph_html: None,
            actions_html: None,
            bordered: false,
            dense: false,
        }
    }

    pub const fn with_glyph(mut self, glyph_html: TrustedHtml<'a>) -> Self {
        self.glyph_html = Some(glyph_html);
        self
    }

    pub const fn with_actions(mut self, actions_html: TrustedHtml<'a>) -> Self {
        self.actions_html = Some(actions_html);
        self
    }

    pub const fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }

    pub const fn dense(mut self) -> Self {
        self.dense = true;
        self
    }

    pub fn class_name(&self) -> String {
        let bordered = if self.bordered { " bordered" } else { "" };
        let dense = if self.dense { " dense" } else { "" };
        format!("wf-empty{bordered}{dense}")
    }
}

impl<'a> askama::filters::HtmlSafe for EmptyState<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl SortDirection {
    fn arrow(self) -> &'static str {
        match self {
            Self::Ascending => "^",
            Self::Descending => "v",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableColumnWidth {
    Auto,
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
    Id,
    Checkbox,
    Action,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableHeader<'a> {
    pub label: &'a str,
    pub numeric: bool,
}

impl<'a> TableHeader<'a> {
    pub const fn new(label: &'a str) -> Self {
        Self {
            label,
            numeric: false,
        }
    }

    pub const fn numeric(label: &'a str) -> Self {
        Self {
            label,
            numeric: true,
        }
    }

    pub fn class_name(&self) -> &'static str {
        if self.numeric { "num" } else { "" }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableCell<'a> {
    pub text: &'a str,
    pub numeric: bool,
    pub strong: bool,
    pub muted: bool,
}

impl<'a> TableCell<'a> {
    pub const fn new(text: &'a str) -> Self {
        Self {
            text,
            numeric: false,
            strong: false,
            muted: false,
        }
    }

    pub const fn numeric(text: &'a str) -> Self {
        Self {
            numeric: true,
            ..Self::new(text)
        }
    }

    pub const fn strong(text: &'a str) -> Self {
        Self {
            strong: true,
            ..Self::new(text)
        }
    }

    pub const fn muted(text: &'a str) -> Self {
        Self {
            muted: true,
            ..Self::new(text)
        }
    }

    pub fn class_name(&self) -> &'static str {
        match (self.numeric, self.strong, self.muted) {
            (false, false, false) => "",
            (true, false, false) => "num",
            (false, true, false) => "strong",
            (false, false, true) => "muted",
            (true, true, false) => "num strong",
            (true, false, true) => "num muted",
            (false, true, true) => "strong muted",
            (true, true, true) => "num strong muted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TableRow<'a> {
    pub cells: &'a [TableCell<'a>],
    pub selected: bool,
}

impl<'a> TableRow<'a> {
    pub const fn new(cells: &'a [TableCell<'a>]) -> Self {
        Self {
            cells,
            selected: false,
        }
    }

    pub const fn selected(mut self) -> Self {
        self.selected = true;
        self
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/table.html")]
pub struct Table<'a> {
    pub headers: &'a [TableHeader<'a>],
    pub rows: &'a [TableRow<'a>],
    pub flush: bool,
    pub interactive: bool,
    pub sticky: bool,
    pub pin_last: bool,
}

impl<'a> Table<'a> {
    pub const fn new(headers: &'a [TableHeader<'a>], rows: &'a [TableRow<'a>]) -> Self {
        Self {
            headers,
            rows,
            flush: false,
            interactive: false,
            sticky: false,
            pin_last: false,
        }
    }

    pub const fn flush(mut self) -> Self {
        self.flush = true;
        self
    }

    pub const fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    pub const fn sticky(mut self) -> Self {
        self.sticky = true;
        self
    }

    pub const fn pin_last(mut self) -> Self {
        self.pin_last = true;
        self
    }

    pub fn class_name(&self) -> String {
        let flush = if self.flush { " flush" } else { "" };
        let interactive = if self.interactive {
            " is-interactive"
        } else {
            ""
        };
        let sticky = if self.sticky { " sticky" } else { "" };
        let pin_last = if self.pin_last { " pin-last" } else { "" };
        format!("wf-table{flush}{interactive}{sticky}{pin_last}")
    }
}

impl<'a> askama::filters::HtmlSafe for Table<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataTableHeader<'a> {
    pub label: &'a str,
    pub numeric: bool,
    pub sort_key: Option<&'a str>,
    pub sort_direction: Option<SortDirection>,
    pub width: TableColumnWidth,
}

impl<'a> DataTableHeader<'a> {
    pub const fn new(label: &'a str) -> Self {
        Self {
            label,
            numeric: false,
            sort_key: None,
            sort_direction: None,
            width: TableColumnWidth::Auto,
        }
    }

    pub const fn numeric(label: &'a str) -> Self {
        Self {
            numeric: true,
            ..Self::new(label)
        }
    }

    pub const fn sort(label: &'a str, sort_key: &'a str) -> Self {
        Self {
            sort_key: Some(sort_key),
            ..Self::new(label)
        }
    }

    pub const fn sorted(label: &'a str, sort_key: &'a str, direction: SortDirection) -> Self {
        Self {
            sort_direction: Some(direction),
            ..Self::sort(label, sort_key)
        }
    }

    pub const fn sortable(mut self, sort_key: &'a str, direction: SortDirection) -> Self {
        self.sort_key = Some(sort_key);
        self.sort_direction = Some(direction);
        self
    }

    pub const fn with_width(mut self, width: TableColumnWidth) -> Self {
        self.width = width;
        self
    }

    pub const fn action_column(mut self) -> Self {
        self.width = TableColumnWidth::Action;
        self
    }

    pub fn class_name(&self) -> &'static str {
        match (self.width, self.numeric) {
            (TableColumnWidth::Auto, false) => "",
            (TableColumnWidth::Auto, true) => "num",
            (TableColumnWidth::ExtraSmall, false) => "wf-col-xs",
            (TableColumnWidth::ExtraSmall, true) => "wf-col-xs num",
            (TableColumnWidth::Small, false) => "wf-col-sm",
            (TableColumnWidth::Small, true) => "wf-col-sm num",
            (TableColumnWidth::Medium, false) => "wf-col-md",
            (TableColumnWidth::Medium, true) => "wf-col-md num",
            (TableColumnWidth::Large, false) => "wf-col-lg",
            (TableColumnWidth::Large, true) => "wf-col-lg num",
            (TableColumnWidth::ExtraLarge, false) => "wf-col-xl",
            (TableColumnWidth::ExtraLarge, true) => "wf-col-xl num",
            (TableColumnWidth::Id, false) => "wf-col-id",
            (TableColumnWidth::Id, true) => "wf-col-id num",
            (TableColumnWidth::Checkbox, false) => "wf-col-chk",
            (TableColumnWidth::Checkbox, true) => "wf-col-chk num",
            (TableColumnWidth::Action, false) => "wf-col-act",
            (TableColumnWidth::Action, true) => "wf-col-act num",
        }
    }

    pub fn sort_arrow(&self) -> &'static str {
        self.sort_direction.map(SortDirection::arrow).unwrap_or("-")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataTableCell<'a> {
    pub text: &'a str,
    pub html: Option<TrustedHtml<'a>>,
    pub numeric: bool,
    pub strong: bool,
    pub muted: bool,
}

impl<'a> DataTableCell<'a> {
    pub const fn new(text: &'a str) -> Self {
        Self {
            text,
            html: None,
            numeric: false,
            strong: false,
            muted: false,
        }
    }

    pub const fn numeric(text: &'a str) -> Self {
        Self {
            numeric: true,
            ..Self::new(text)
        }
    }

    pub const fn strong(text: &'a str) -> Self {
        Self {
            strong: true,
            ..Self::new(text)
        }
    }

    pub const fn muted(text: &'a str) -> Self {
        Self {
            muted: true,
            ..Self::new(text)
        }
    }

    pub const fn html(html: TrustedHtml<'a>) -> Self {
        Self {
            text: "",
            html: Some(html),
            numeric: false,
            strong: false,
            muted: false,
        }
    }

    pub fn class_name(&self) -> &'static str {
        match (self.numeric, self.strong, self.muted) {
            (false, false, false) => "",
            (true, false, false) => "num",
            (false, true, false) => "strong",
            (false, false, true) => "muted",
            (true, true, false) => "num strong",
            (true, false, true) => "num muted",
            (false, true, true) => "strong muted",
            (true, true, true) => "num strong muted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataTableRow<'a> {
    pub cells: &'a [DataTableCell<'a>],
    pub selected: bool,
}

impl<'a> DataTableRow<'a> {
    pub const fn new(cells: &'a [DataTableCell<'a>]) -> Self {
        Self {
            cells,
            selected: false,
        }
    }

    pub const fn selected(mut self) -> Self {
        self.selected = true;
        self
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/data_table.html")]
pub struct DataTable<'a> {
    pub headers: &'a [DataTableHeader<'a>],
    pub rows: &'a [DataTableRow<'a>],
    pub flush: bool,
    pub interactive: bool,
    pub sticky: bool,
    pub pin_last: bool,
}

impl<'a> DataTable<'a> {
    pub const fn new(headers: &'a [DataTableHeader<'a>], rows: &'a [DataTableRow<'a>]) -> Self {
        Self {
            headers,
            rows,
            flush: false,
            interactive: false,
            sticky: false,
            pin_last: false,
        }
    }

    pub const fn flush(mut self) -> Self {
        self.flush = true;
        self
    }

    pub const fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    pub const fn sticky(mut self) -> Self {
        self.sticky = true;
        self
    }

    pub const fn pin_last(mut self) -> Self {
        self.pin_last = true;
        self
    }

    pub fn class_name(&self) -> String {
        let flush = if self.flush { " flush" } else { "" };
        let interactive = if self.interactive {
            " is-interactive"
        } else {
            ""
        };
        let sticky = if self.sticky { " sticky" } else { "" };
        let pin_last = if self.pin_last { " pin-last" } else { "" };
        format!("wf-table{flush}{interactive}{sticky}{pin_last}")
    }
}

impl<'a> askama::filters::HtmlSafe for DataTable<'a> {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedDataTableCell {
    pub text: String,
    pub html: Option<TrustedHtmlBuf>,
    pub numeric: bool,
    pub strong: bool,
    pub muted: bool,
}

impl OwnedDataTableCell {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            html: None,
            numeric: false,
            strong: false,
            muted: false,
        }
    }

    pub fn numeric(text: impl Into<String>) -> Self {
        Self {
            numeric: true,
            ..Self::new(text)
        }
    }

    pub fn strong(text: impl Into<String>) -> Self {
        Self {
            strong: true,
            ..Self::new(text)
        }
    }

    pub fn muted(text: impl Into<String>) -> Self {
        Self {
            muted: true,
            ..Self::new(text)
        }
    }

    pub fn html(html: impl Into<TrustedHtmlBuf>) -> Self {
        Self {
            text: String::new(),
            html: Some(html.into()),
            numeric: false,
            strong: false,
            muted: false,
        }
    }

    pub fn class_name(&self) -> &'static str {
        match (self.numeric, self.strong, self.muted) {
            (false, false, false) => "",
            (true, false, false) => "num",
            (false, true, false) => "strong",
            (false, false, true) => "muted",
            (true, true, false) => "num strong",
            (true, false, true) => "num muted",
            (false, true, true) => "strong muted",
            (true, true, true) => "num strong muted",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedDataTableRow {
    pub cells: Vec<OwnedDataTableCell>,
    pub selected: bool,
}

impl OwnedDataTableRow {
    pub fn new(cells: impl Into<Vec<OwnedDataTableCell>>) -> Self {
        Self {
            cells: cells.into(),
            selected: false,
        }
    }

    pub fn selected(mut self) -> Self {
        self.selected = true;
        self
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/owned_data_table.html")]
pub struct OwnedDataTable<'a> {
    pub headers: &'a [DataTableHeader<'a>],
    pub rows: Vec<OwnedDataTableRow>,
    pub flush: bool,
    pub interactive: bool,
    pub sticky: bool,
    pub pin_last: bool,
}

impl<'a> OwnedDataTable<'a> {
    pub fn new(
        headers: &'a [DataTableHeader<'a>],
        rows: impl Into<Vec<OwnedDataTableRow>>,
    ) -> Self {
        Self {
            headers,
            rows: rows.into(),
            flush: false,
            interactive: false,
            sticky: false,
            pin_last: false,
        }
    }

    pub fn flush(mut self) -> Self {
        self.flush = true;
        self
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    pub fn sticky(mut self) -> Self {
        self.sticky = true;
        self
    }

    pub fn pin_last(mut self) -> Self {
        self.pin_last = true;
        self
    }

    pub fn class_name(&self) -> String {
        let flush = if self.flush { " flush" } else { "" };
        let interactive = if self.interactive {
            " is-interactive"
        } else {
            ""
        };
        let sticky = if self.sticky { " sticky" } else { "" };
        let pin_last = if self.pin_last { " pin-last" } else { "" };
        format!("wf-table{flush}{interactive}{sticky}{pin_last}")
    }
}

impl askama::filters::HtmlSafe for OwnedDataTable<'_> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/filter_bar.html")]
pub struct FilterBar<'a> {
    pub controls_html: TrustedHtml<'a>,
    pub actions_html: Option<TrustedHtml<'a>>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> FilterBar<'a> {
    pub const fn new(controls_html: TrustedHtml<'a>) -> Self {
        Self {
            controls_html,
            actions_html: None,
            attrs: &[],
        }
    }

    pub const fn with_actions(mut self, actions_html: TrustedHtml<'a>) -> Self {
        self.actions_html = Some(actions_html);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for FilterBar<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/bulk_action_bar.html")]
pub struct BulkActionBar<'a> {
    pub count_label: &'a str,
    pub actions_html: TrustedHtml<'a>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> BulkActionBar<'a> {
    pub const fn new(count_label: &'a str, actions_html: TrustedHtml<'a>) -> Self {
        Self {
            count_label,
            actions_html,
            attrs: &[],
        }
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for BulkActionBar<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/table_footer.html")]
pub struct TableFooter<'a> {
    pub content_html: TrustedHtml<'a>,
    pub actions_html: Option<TrustedHtml<'a>>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> TableFooter<'a> {
    pub const fn new(content_html: TrustedHtml<'a>) -> Self {
        Self {
            content_html,
            actions_html: None,
            attrs: &[],
        }
    }

    pub const fn with_actions(mut self, actions_html: TrustedHtml<'a>) -> Self {
        self.actions_html = Some(actions_html);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for TableFooter<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/row_select.html")]
pub struct RowSelect<'a> {
    pub name: &'a str,
    pub value: &'a str,
    pub label: &'a str,
    pub checked: bool,
    pub disabled: bool,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> RowSelect<'a> {
    pub const fn new(name: &'a str, value: &'a str, label: &'a str) -> Self {
        Self {
            name,
            value,
            label,
            checked: false,
            disabled: false,
            attrs: &[],
        }
    }

    pub const fn checked(mut self) -> Self {
        self.checked = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for RowSelect<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/table_wrap.html")]
pub struct TableWrap<'a> {
    pub table_html: TrustedHtml<'a>,
    pub filterbar_html: Option<TrustedHtml<'a>>,
    pub filterbar_component_html: Option<TrustedHtml<'a>>,
    pub bulk_count: Option<&'a str>,
    pub bulk_actions_html: Option<TrustedHtml<'a>>,
    pub bulkbar_component_html: Option<TrustedHtml<'a>>,
    pub footer_html: Option<TrustedHtml<'a>>,
    pub footer_component_html: Option<TrustedHtml<'a>>,
}

impl<'a> TableWrap<'a> {
    pub const fn new(table_html: TrustedHtml<'a>) -> Self {
        Self {
            table_html,
            filterbar_html: None,
            filterbar_component_html: None,
            bulk_count: None,
            bulk_actions_html: None,
            bulkbar_component_html: None,
            footer_html: None,
            footer_component_html: None,
        }
    }

    pub const fn with_filterbar(mut self, filterbar_html: TrustedHtml<'a>) -> Self {
        self.filterbar_html = Some(filterbar_html);
        self
    }

    pub const fn with_filterbar_component(mut self, filterbar_html: TrustedHtml<'a>) -> Self {
        self.filterbar_component_html = Some(filterbar_html);
        self
    }

    pub const fn with_bulkbar(
        mut self,
        bulk_count: &'a str,
        bulk_actions_html: TrustedHtml<'a>,
    ) -> Self {
        self.bulk_count = Some(bulk_count);
        self.bulk_actions_html = Some(bulk_actions_html);
        self
    }

    pub const fn with_bulkbar_component(mut self, bulkbar_html: TrustedHtml<'a>) -> Self {
        self.bulkbar_component_html = Some(bulkbar_html);
        self
    }

    pub const fn with_footer(mut self, footer_html: TrustedHtml<'a>) -> Self {
        self.footer_html = Some(footer_html);
        self
    }

    pub const fn with_footer_component(mut self, footer_html: TrustedHtml<'a>) -> Self {
        self.footer_component_html = Some(footer_html);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for TableWrap<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DefinitionItem<'a> {
    pub term: &'a str,
    pub description: &'a str,
}

impl<'a> DefinitionItem<'a> {
    pub const fn new(term: &'a str, description: &'a str) -> Self {
        Self { term, description }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/definition_list.html")]
pub struct DefinitionList<'a> {
    pub items: &'a [DefinitionItem<'a>],
    pub flush: bool,
}

impl<'a> DefinitionList<'a> {
    pub const fn new(items: &'a [DefinitionItem<'a>]) -> Self {
        Self {
            items,
            flush: false,
        }
    }

    pub const fn flush(mut self) -> Self {
        self.flush = true;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.flush { "wf-dl flush" } else { "wf-dl" }
    }
}

impl<'a> askama::filters::HtmlSafe for DefinitionList<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RankRow<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub percent: u8,
}

impl<'a> RankRow<'a> {
    pub const fn new(label: &'a str, value: &'a str, percent: u8) -> Self {
        Self {
            label,
            value,
            percent,
        }
    }

    pub fn bounded_percent(&self) -> u8 {
        self.percent.min(100)
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/rank_list.html")]
pub struct RankList<'a> {
    pub rows: &'a [RankRow<'a>],
}

impl<'a> RankList<'a> {
    pub const fn new(rows: &'a [RankRow<'a>]) -> Self {
        Self { rows }
    }
}

impl<'a> askama::filters::HtmlSafe for RankList<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FeedRow<'a> {
    pub time: &'a str,
    pub kicker: &'a str,
    pub text: &'a str,
}

impl<'a> FeedRow<'a> {
    pub const fn new(time: &'a str, kicker: &'a str, text: &'a str) -> Self {
        Self { time, kicker, text }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/feed.html")]
pub struct Feed<'a> {
    pub rows: &'a [FeedRow<'a>],
}

impl<'a> Feed<'a> {
    pub const fn new(rows: &'a [FeedRow<'a>]) -> Self {
        Self { rows }
    }
}

impl<'a> askama::filters::HtmlSafe for Feed<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimelineItem<'a> {
    pub time: &'a str,
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub active: bool,
}

impl<'a> TimelineItem<'a> {
    pub const fn new(time: &'a str, title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            time,
            title,
            body_html,
            active: false,
        }
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.active {
            "wf-timeline-item is-active"
        } else {
            "wf-timeline-item"
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/timeline.html")]
pub struct Timeline<'a> {
    pub items: &'a [TimelineItem<'a>],
}

impl<'a> Timeline<'a> {
    pub const fn new(items: &'a [TimelineItem<'a>]) -> Self {
        Self { items }
    }
}

impl<'a> askama::filters::HtmlSafe for Timeline<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TreeItemKind {
    Folder,
    File,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TreeItem<'a> {
    pub kind: TreeItemKind,
    pub label: &'a str,
    pub active: bool,
    pub collapsed: bool,
    pub children_html: Option<TrustedHtml<'a>>,
}

impl<'a> TreeItem<'a> {
    pub const fn folder(label: &'a str) -> Self {
        Self {
            kind: TreeItemKind::Folder,
            label,
            active: false,
            collapsed: false,
            children_html: None,
        }
    }

    pub const fn file(label: &'a str) -> Self {
        Self {
            kind: TreeItemKind::File,
            label,
            active: false,
            collapsed: false,
            children_html: None,
        }
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }

    pub const fn collapsed(mut self) -> Self {
        self.collapsed = true;
        self
    }

    pub const fn with_children(mut self, children_html: TrustedHtml<'a>) -> Self {
        self.children_html = Some(children_html);
        self
    }

    pub fn item_class(&self) -> &'static str {
        if self.collapsed { "is-collapsed" } else { "" }
    }

    pub fn label_class(&self) -> &'static str {
        match (self.kind, self.active) {
            (TreeItemKind::Folder, _) => "tree-folder",
            (TreeItemKind::File, true) => "tree-file is-active",
            (TreeItemKind::File, false) => "tree-file",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/tree_view.html")]
pub struct TreeView<'a> {
    pub items: &'a [TreeItem<'a>],
    pub nested: bool,
}

impl<'a> TreeView<'a> {
    pub const fn new(items: &'a [TreeItem<'a>]) -> Self {
        Self {
            items,
            nested: false,
        }
    }

    pub const fn nested(mut self) -> Self {
        self.nested = true;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.nested { "" } else { "wf-tree" }
    }
}

impl<'a> askama::filters::HtmlSafe for TreeView<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/framed.html")]
pub struct Framed<'a> {
    pub content_html: TrustedHtml<'a>,
    pub dense: bool,
    pub dashed: bool,
}

impl<'a> Framed<'a> {
    pub const fn new(content_html: TrustedHtml<'a>) -> Self {
        Self {
            content_html,
            dense: false,
            dashed: false,
        }
    }

    pub const fn dense(mut self) -> Self {
        self.dense = true;
        self
    }

    pub const fn dashed(mut self) -> Self {
        self.dashed = true;
        self
    }

    pub fn class_name(&self) -> String {
        let dense = if self.dense { " dense" } else { "" };
        let dashed = if self.dashed { " dashed" } else { "" };
        format!("wf-framed{dense}{dashed}")
    }
}

impl<'a> askama::filters::HtmlSafe for Framed<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/grid.html")]
pub struct Grid<'a> {
    pub content_html: TrustedHtml<'a>,
    pub columns: u8,
}

impl<'a> Grid<'a> {
    pub const fn new(content_html: TrustedHtml<'a>) -> Self {
        Self {
            content_html,
            columns: 2,
        }
    }

    pub const fn with_columns(mut self, columns: u8) -> Self {
        self.columns = columns;
        self
    }

    pub fn class_name(&self) -> String {
        format!("wf-grid cols-{}", self.columns)
    }
}

impl<'a> askama::filters::HtmlSafe for Grid<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/split.html")]
pub struct Split<'a> {
    pub content_html: TrustedHtml<'a>,
    pub vertical: bool,
}

impl<'a> Split<'a> {
    pub const fn new(content_html: TrustedHtml<'a>) -> Self {
        Self {
            content_html,
            vertical: false,
        }
    }

    pub const fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.vertical {
            "wf-split vertical"
        } else {
            "wf-split"
        }
    }
}

impl<'a> askama::filters::HtmlSafe for Split<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/callout.html")]
pub struct Callout<'a> {
    pub kind: FeedbackKind,
    pub title: Option<&'a str>,
    pub body_html: TrustedHtml<'a>,
}

impl<'a> Callout<'a> {
    pub const fn new(kind: FeedbackKind, body_html: TrustedHtml<'a>) -> Self {
        Self {
            kind,
            title: None,
            body_html,
        }
    }

    pub const fn with_title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn class_name(&self) -> String {
        format!("wf-callout {}", self.kind.class())
    }
}

impl<'a> askama::filters::HtmlSafe for Callout<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/toast.html")]
pub struct Toast<'a> {
    pub kind: FeedbackKind,
    pub message: &'a str,
}

impl<'a> Toast<'a> {
    pub const fn new(kind: FeedbackKind, message: &'a str) -> Self {
        Self { kind, message }
    }

    pub fn class_name(&self) -> String {
        format!("wf-toast {}", self.kind.class())
    }
}

impl<'a> askama::filters::HtmlSafe for Toast<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/toast_host.html")]
pub struct ToastHost<'a> {
    pub id: &'a str,
}

impl<'a> ToastHost<'a> {
    pub const fn new() -> Self {
        Self { id: "toast-host" }
    }

    pub const fn with_id(mut self, id: &'a str) -> Self {
        self.id = id;
        self
    }
}

impl<'a> Default for ToastHost<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> askama::filters::HtmlSafe for ToastHost<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/tooltip.html")]
pub struct Tooltip<'a> {
    pub tip: &'a str,
    pub content_html: TrustedHtml<'a>,
}

impl<'a> Tooltip<'a> {
    pub const fn new(tip: &'a str, content_html: TrustedHtml<'a>) -> Self {
        Self { tip, content_html }
    }
}

impl<'a> askama::filters::HtmlSafe for Tooltip<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuItemKind {
    Button,
    Link,
    Separator,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MenuItem<'a> {
    pub kind: MenuItemKind,
    pub label: &'a str,
    pub href: Option<&'a str>,
    pub danger: bool,
    pub disabled: bool,
    pub kbd: Option<&'a str>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> MenuItem<'a> {
    pub const fn button(label: &'a str) -> Self {
        Self {
            kind: MenuItemKind::Button,
            label,
            href: None,
            danger: false,
            disabled: false,
            kbd: None,
            attrs: &[],
        }
    }

    pub const fn link(label: &'a str, href: &'a str) -> Self {
        Self {
            kind: MenuItemKind::Link,
            href: Some(href),
            ..Self::button(label)
        }
    }

    pub const fn separator() -> Self {
        Self {
            kind: MenuItemKind::Separator,
            label: "",
            href: None,
            danger: false,
            disabled: false,
            kbd: None,
            attrs: &[],
        }
    }

    pub const fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn with_kbd(mut self, kbd: &'a str) -> Self {
        self.kbd = Some(kbd);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.danger {
            "wf-menu-item danger"
        } else {
            "wf-menu-item"
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/menu.html")]
pub struct Menu<'a> {
    pub items: &'a [MenuItem<'a>],
}

impl<'a> Menu<'a> {
    pub const fn new(items: &'a [MenuItem<'a>]) -> Self {
        Self { items }
    }
}

impl<'a> askama::filters::HtmlSafe for Menu<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/popover.html")]
pub struct Popover<'a> {
    pub trigger_html: TrustedHtml<'a>,
    pub body_html: TrustedHtml<'a>,
    pub heading: Option<&'a str>,
    pub side: &'a str,
    pub open: bool,
}

impl<'a> Popover<'a> {
    pub const fn new(trigger_html: TrustedHtml<'a>, body_html: TrustedHtml<'a>) -> Self {
        Self {
            trigger_html,
            body_html,
            heading: None,
            side: "bottom",
            open: false,
        }
    }

    pub const fn with_heading(mut self, heading: &'a str) -> Self {
        self.heading = Some(heading);
        self
    }

    pub const fn with_side(mut self, side: &'a str) -> Self {
        self.side = side;
        self
    }

    pub const fn open(mut self) -> Self {
        self.open = true;
        self
    }

    pub fn popover_class(&self) -> &'static str {
        if self.open {
            "wf-popover is-open"
        } else {
            "wf-popover"
        }
    }
}

impl<'a> askama::filters::HtmlSafe for Popover<'a> {}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ModalSize {
    #[default]
    Default,
    Large,
}

impl ModalSize {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Large => " wf-modal--lg",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/modal.html")]
pub struct Modal<'a> {
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub footer_html: Option<TrustedHtml<'a>>,
    pub open: bool,
    pub size: ModalSize,
}

impl<'a> Modal<'a> {
    pub const fn new(title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            body_html,
            footer_html: None,
            open: false,
            size: ModalSize::Default,
        }
    }

    pub const fn with_footer(mut self, footer_html: TrustedHtml<'a>) -> Self {
        self.footer_html = Some(footer_html);
        self
    }

    pub const fn open(mut self) -> Self {
        self.open = true;
        self
    }

    pub const fn with_size(mut self, size: ModalSize) -> Self {
        self.size = size;
        self
    }

    pub const fn large(self) -> Self {
        self.with_size(ModalSize::Large)
    }

    pub fn overlay_class(&self) -> &'static str {
        if self.open {
            "wf-overlay is-open"
        } else {
            "wf-overlay"
        }
    }

    pub fn modal_class(&self) -> String {
        let open = if self.open { " is-open" } else { "" };
        let size = self.size.class();
        format!("wf-modal{open}{size}")
    }
}

impl<'a> Default for Modal<'a> {
    fn default() -> Self {
        Self::new("", TrustedHtml::new(""))
    }
}

impl<'a> askama::filters::HtmlSafe for Modal<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/drawer.html")]
pub struct Drawer<'a> {
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub footer_html: Option<TrustedHtml<'a>>,
    pub open: bool,
    pub left: bool,
}

impl<'a> Drawer<'a> {
    pub const fn new(title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            body_html,
            footer_html: None,
            open: false,
            left: false,
        }
    }

    pub const fn with_footer(mut self, footer_html: TrustedHtml<'a>) -> Self {
        self.footer_html = Some(footer_html);
        self
    }

    pub const fn open(mut self) -> Self {
        self.open = true;
        self
    }

    pub const fn left(mut self) -> Self {
        self.left = true;
        self
    }

    pub fn overlay_class(&self) -> &'static str {
        if self.open {
            "wf-overlay is-open"
        } else {
            "wf-overlay"
        }
    }

    pub fn drawer_class(&self) -> String {
        let open = if self.open { " is-open" } else { "" };
        let left = if self.left { " left" } else { "" };
        format!("wf-drawer{open}{left}")
    }
}

impl<'a> askama::filters::HtmlSafe for Drawer<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/progress.html")]
pub struct Progress {
    pub value: Option<u8>,
}

impl Progress {
    pub const fn new(value: u8) -> Self {
        Self { value: Some(value) }
    }

    pub const fn indeterminate() -> Self {
        Self { value: None }
    }

    pub fn class_name(&self) -> &'static str {
        if self.value.is_some() {
            "wf-progress"
        } else {
            "wf-progress indeterminate"
        }
    }

    pub fn bounded_value(&self) -> u8 {
        self.value.unwrap_or(0).min(100)
    }
}

impl askama::filters::HtmlSafe for Progress {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeterColor {
    Accent,
    Ok,
    Warn,
    Error,
    Info,
}

impl MeterColor {
    fn css_var(self) -> &'static str {
        match self {
            Self::Accent => "var(--accent)",
            Self::Ok => "var(--ok)",
            Self::Warn => "var(--warn)",
            Self::Error => "var(--err)",
            Self::Info => "var(--info)",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/meter.html")]
pub struct Meter {
    pub value: u8,
    pub width_px: Option<u16>,
    pub height_px: Option<u16>,
    pub color: Option<MeterColor>,
}

impl Meter {
    pub const fn new(value: u8) -> Self {
        Self {
            value,
            width_px: None,
            height_px: None,
            color: None,
        }
    }

    pub const fn with_size_px(mut self, width_px: u16, height_px: u16) -> Self {
        self.width_px = Some(width_px);
        self.height_px = Some(height_px);
        self
    }

    pub const fn with_color(mut self, color: MeterColor) -> Self {
        self.color = Some(color);
        self
    }

    pub fn style(&self) -> String {
        let mut style = String::with_capacity(72);
        let _ = write!(&mut style, "--meter: {}%", self.value.min(100));
        if let Some(width) = self.width_px {
            let _ = write!(&mut style, "; --meter-w: {width}px");
        }
        if let Some(height) = self.height_px {
            let _ = write!(&mut style, "; --meter-h: {height}px");
        }
        if let Some(color) = self.color {
            style.push_str("; --meter-c: ");
            style.push_str(color.css_var());
        }
        style
    }
}

impl askama::filters::HtmlSafe for Meter {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/code_block.html")]
pub struct CodeBlock<'a> {
    pub code: &'a str,
    pub language: Option<&'a str>,
    pub label: Option<&'a str>,
    pub copy_target_id: Option<&'a str>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> CodeBlock<'a> {
    pub const fn new(code: &'a str) -> Self {
        Self {
            code,
            language: None,
            label: None,
            copy_target_id: None,
            attrs: &[],
        }
    }

    pub const fn with_language(mut self, language: &'a str) -> Self {
        self.language = Some(language);
        self
    }

    pub const fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub const fn with_copy_target(mut self, copy_target_id: &'a str) -> Self {
        self.copy_target_id = Some(copy_target_id);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for CodeBlock<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnippetTab<'a> {
    pub label: &'a str,
    pub code: &'a str,
    pub language: Option<&'a str>,
    pub active: bool,
}

impl<'a> SnippetTab<'a> {
    pub const fn new(label: &'a str, code: &'a str) -> Self {
        Self {
            label,
            code,
            language: None,
            active: false,
        }
    }

    pub const fn with_language(mut self, language: &'a str) -> Self {
        self.language = Some(language);
        self
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }

    pub fn tab_class(&self) -> &'static str {
        if self.active {
            "wf-snippet-tab is-active"
        } else {
            "wf-snippet-tab"
        }
    }

    pub fn panel_class(&self) -> &'static str {
        if self.active {
            "wf-snippet-panel is-active"
        } else {
            "wf-snippet-panel"
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/snippet_tabs.html")]
pub struct SnippetTabs<'a> {
    pub id: &'a str,
    pub tabs: &'a [SnippetTab<'a>],
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> SnippetTabs<'a> {
    pub const fn new(id: &'a str, tabs: &'a [SnippetTab<'a>]) -> Self {
        Self {
            id,
            tabs,
            attrs: &[],
        }
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for SnippetTabs<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/strength_meter.html")]
pub struct StrengthMeter<'a> {
    pub value: u8,
    pub max: u8,
    pub text: &'a str,
    pub label: Option<&'a str>,
    pub kind: Option<FeedbackKind>,
    pub live: bool,
}

impl<'a> StrengthMeter<'a> {
    pub const fn new(value: u8, max: u8, text: &'a str) -> Self {
        Self {
            value,
            max,
            text,
            label: None,
            kind: None,
            live: false,
        }
    }

    pub const fn with_label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub const fn with_feedback(mut self, feedback: FeedbackKind) -> Self {
        self.kind = Some(feedback);
        self
    }

    pub const fn live(mut self) -> Self {
        self.live = true;
        self
    }

    pub fn class_name(&self) -> &'static str {
        match self.kind {
            Some(FeedbackKind::Info) => "wf-strength-meter is-info",
            Some(FeedbackKind::Ok) => "wf-strength-meter is-ok",
            Some(FeedbackKind::Warn) => "wf-strength-meter is-warn",
            Some(FeedbackKind::Error) => "wf-strength-meter is-err",
            None => "wf-strength-meter",
        }
    }

    pub fn bounded_value(&self) -> u8 {
        self.value.min(self.max)
    }

    pub fn percentage(&self) -> u8 {
        if self.max == 0 {
            0
        } else {
            ((u16::from(self.bounded_value()) * 100) / u16::from(self.max)) as u8
        }
    }

    pub fn style(&self) -> String {
        format!("--strength: {}%", self.percentage())
    }
}

impl<'a> askama::filters::HtmlSafe for StrengthMeter<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/kbd.html")]
pub struct Kbd<'a> {
    pub label: &'a str,
}

impl<'a> Kbd<'a> {
    pub const fn new(label: &'a str) -> Self {
        Self { label }
    }
}

impl<'a> askama::filters::HtmlSafe for Kbd<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkeletonKind {
    Line,
    Title,
    Block,
}

impl SkeletonKind {
    fn class(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Title => "title",
            Self::Block => "block",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/skeleton.html")]
pub struct Skeleton {
    pub kind: SkeletonKind,
}

impl Skeleton {
    pub const fn line() -> Self {
        Self {
            kind: SkeletonKind::Line,
        }
    }

    pub const fn title() -> Self {
        Self {
            kind: SkeletonKind::Title,
        }
    }

    pub const fn block() -> Self {
        Self {
            kind: SkeletonKind::Block,
        }
    }

    pub fn class_name(&self) -> String {
        format!("wf-skeleton {}", self.kind.class())
    }
}

impl askama::filters::HtmlSafe for Skeleton {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/spinner.html")]
pub struct Spinner {
    pub large: bool,
}

impl Spinner {
    pub const fn new() -> Self {
        Self { large: false }
    }

    pub const fn large() -> Self {
        Self { large: true }
    }

    pub fn class_name(&self) -> &'static str {
        if self.large {
            "wf-spinner lg"
        } else {
            "wf-spinner"
        }
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl askama::filters::HtmlSafe for Spinner {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelineSegmentKind {
    Default,
    Chevron,
    Flag,
    Buffer,
    Mode,
    Position,
    Progress,
}

impl ModelineSegmentKind {
    fn class(self) -> &'static str {
        match self {
            Self::Default => "wf-ml-seg",
            Self::Chevron => "wf-ml-seg wf-ml-chevron",
            Self::Flag => "wf-ml-seg wf-ml-flag",
            Self::Buffer => "wf-ml-seg wf-ml-buffer",
            Self::Mode => "wf-ml-seg wf-ml-mode",
            Self::Position => "wf-ml-seg wf-ml-pos",
            Self::Progress => "wf-ml-seg wf-ml-progress",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/modeline_segment.html")]
pub struct ModelineSegment<'a> {
    pub label: &'a str,
    pub kind: ModelineSegmentKind,
    pub state: Option<FeedbackKind>,
    pub href: Option<&'a str>,
    pub button: bool,
    pub button_type: &'a str,
    pub active: bool,
    pub kbd: Option<&'a str>,
    pub html: Option<TrustedHtml<'a>>,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> ModelineSegment<'a> {
    pub const fn text(label: &'a str) -> Self {
        Self {
            label,
            kind: ModelineSegmentKind::Default,
            state: None,
            href: None,
            button: false,
            button_type: "button",
            active: false,
            kbd: None,
            html: None,
            attrs: &[],
        }
    }

    pub const fn chevron(label: &'a str) -> Self {
        Self {
            kind: ModelineSegmentKind::Chevron,
            ..Self::text(label)
        }
    }

    pub const fn flag(label: &'a str) -> Self {
        Self {
            kind: ModelineSegmentKind::Flag,
            ..Self::text(label)
        }
    }

    pub const fn buffer(label: &'a str) -> Self {
        Self {
            kind: ModelineSegmentKind::Buffer,
            ..Self::text(label)
        }
    }

    pub const fn mode(label: &'a str) -> Self {
        Self {
            kind: ModelineSegmentKind::Mode,
            ..Self::text(label)
        }
    }

    pub const fn position(label: &'a str) -> Self {
        Self {
            kind: ModelineSegmentKind::Position,
            ..Self::text(label)
        }
    }

    pub const fn progress(label: &'a str) -> Self {
        Self {
            kind: ModelineSegmentKind::Progress,
            ..Self::text(label)
        }
    }

    pub const fn link(label: &'a str, href: &'a str) -> Self {
        Self {
            href: Some(href),
            ..Self::text(label)
        }
    }

    pub const fn button(label: &'a str) -> Self {
        Self {
            button: true,
            ..Self::text(label)
        }
    }

    pub const fn with_feedback(mut self, feedback: FeedbackKind) -> Self {
        self.state = Some(feedback);
        self
    }

    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }

    pub const fn with_kbd(mut self, kbd: &'a str) -> Self {
        self.kbd = Some(kbd);
        self
    }

    pub const fn with_html(mut self, html: TrustedHtml<'a>) -> Self {
        self.html = Some(html);
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }

    pub const fn with_button_type(mut self, button_type: &'a str) -> Self {
        self.button_type = button_type;
        self
    }

    pub fn class_name(&self) -> String {
        let mut class = String::from(self.kind.class());
        if self.href.is_some() || self.button || !self.attrs.is_empty() {
            class.push_str(" is-interactive");
        }
        if self.active {
            class.push_str(" is-active");
        }
        if let Some(kind) = self.state {
            class.push_str(" is-");
            class.push_str(kind.class());
        }
        class
    }
}

impl<'a> askama::filters::HtmlSafe for ModelineSegment<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/modeline.html")]
pub struct Modeline<'a> {
    pub left_segments: &'a [ModelineSegment<'a>],
    pub right_segments: &'a [ModelineSegment<'a>],
    pub fill: bool,
    pub attrs: &'a [HtmlAttr<'a>],
}

impl<'a> Modeline<'a> {
    pub const fn new(left_segments: &'a [ModelineSegment<'a>]) -> Self {
        Self {
            left_segments,
            right_segments: &[],
            fill: true,
            attrs: &[],
        }
    }

    pub const fn with_right(mut self, right_segments: &'a [ModelineSegment<'a>]) -> Self {
        self.right_segments = right_segments;
        self
    }

    pub const fn without_fill(mut self) -> Self {
        self.fill = false;
        self
    }

    pub const fn with_attrs(mut self, attrs: &'a [HtmlAttr<'a>]) -> Self {
        self.attrs = attrs;
        self
    }
}

impl<'a> askama::filters::HtmlSafe for Modeline<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MinibufferHistoryRow<'a> {
    pub time: &'a str,
    pub message: &'a str,
    pub kind: Option<FeedbackKind>,
}

impl<'a> MinibufferHistoryRow<'a> {
    pub const fn new(time: &'a str, message: &'a str) -> Self {
        Self {
            time,
            message,
            kind: None,
        }
    }

    pub const fn with_feedback(mut self, feedback: FeedbackKind) -> Self {
        self.kind = Some(feedback);
        self
    }

    pub fn class_name(&self) -> &'static str {
        match self.kind {
            Some(FeedbackKind::Info) => "row is-info",
            Some(FeedbackKind::Ok) => "row is-ok",
            Some(FeedbackKind::Warn) => "row is-warn",
            Some(FeedbackKind::Error) => "row is-err",
            None => "row",
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/minibuffer.html")]
pub struct Minibuffer<'a> {
    pub prompt: &'a str,
    pub message: Option<&'a str>,
    pub kind: Option<FeedbackKind>,
    pub time: Option<&'a str>,
    pub history: &'a [MinibufferHistoryRow<'a>],
}

impl<'a> Minibuffer<'a> {
    pub const fn new() -> Self {
        Self {
            prompt: ">",
            message: None,
            kind: None,
            time: None,
            history: &[],
        }
    }

    pub const fn with_prompt(mut self, prompt: &'a str) -> Self {
        self.prompt = prompt;
        self
    }

    pub const fn with_message(mut self, kind: FeedbackKind, message: &'a str) -> Self {
        self.kind = Some(kind);
        self.message = Some(message);
        self
    }

    pub const fn with_time(mut self, time: &'a str) -> Self {
        self.time = Some(time);
        self
    }

    pub const fn with_history(mut self, history: &'a [MinibufferHistoryRow<'a>]) -> Self {
        self.history = history;
        self
    }

    pub const fn has_history(&self) -> bool {
        !self.history.is_empty()
    }

    pub fn message_class(&self) -> String {
        match self.kind {
            Some(kind) if self.message.is_some() => {
                format!("wf-minibuffer-msg is-visible is-{}", kind.class())
            }
            _ => "wf-minibuffer-msg".to_owned(),
        }
    }
}

impl<'a> Default for Minibuffer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> askama::filters::HtmlSafe for Minibuffer<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/minibuffer_echo.html")]
pub struct MinibufferEcho<'a> {
    pub kind: FeedbackKind,
    pub message: &'a str,
}

impl<'a> MinibufferEcho<'a> {
    pub const fn new(kind: FeedbackKind, message: &'a str) -> Self {
        Self { kind, message }
    }

    pub const fn info(message: &'a str) -> Self {
        Self::new(FeedbackKind::Info, message)
    }

    pub const fn ok(message: &'a str) -> Self {
        Self::new(FeedbackKind::Ok, message)
    }

    pub const fn warn(message: &'a str) -> Self {
        Self::new(FeedbackKind::Warn, message)
    }

    pub const fn error(message: &'a str) -> Self {
        Self::new(FeedbackKind::Error, message)
    }

    pub fn kind_class(&self) -> &'static str {
        self.kind.class()
    }
}

impl<'a> askama::filters::HtmlSafe for MinibufferEcho<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FeatureItem<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> FeatureItem<'a> {
    pub const fn new(title: &'a str, body: &'a str) -> Self {
        Self { title, body }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/feature_grid.html")]
pub struct FeatureGrid<'a> {
    pub items: &'a [FeatureItem<'a>],
}

impl<'a> FeatureGrid<'a> {
    pub const fn new(items: &'a [FeatureItem<'a>]) -> Self {
        Self { items }
    }
}

impl<'a> askama::filters::HtmlSafe for FeatureGrid<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MarketingStep<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

impl<'a> MarketingStep<'a> {
    pub const fn new(title: &'a str, body: &'a str) -> Self {
        Self { title, body }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/marketing_step_grid.html")]
pub struct MarketingStepGrid<'a> {
    pub steps: &'a [MarketingStep<'a>],
}

impl<'a> MarketingStepGrid<'a> {
    pub const fn new(steps: &'a [MarketingStep<'a>]) -> Self {
        Self { steps }
    }
}

impl<'a> askama::filters::HtmlSafe for MarketingStepGrid<'a> {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PricingPlan<'a> {
    pub name: &'a str,
    pub price: &'a str,
    pub unit: Option<&'a str>,
    pub blurb: Option<&'a str>,
    pub featured: bool,
}

impl<'a> PricingPlan<'a> {
    pub const fn new(name: &'a str, price: &'a str) -> Self {
        Self {
            name,
            price,
            unit: None,
            blurb: None,
            featured: false,
        }
    }

    pub const fn with_unit(mut self, unit: &'a str) -> Self {
        self.unit = Some(unit);
        self
    }

    pub const fn with_blurb(mut self, blurb: &'a str) -> Self {
        self.blurb = Some(blurb);
        self
    }

    pub const fn featured(mut self) -> Self {
        self.featured = true;
        self
    }

    pub fn class_name(&self) -> &'static str {
        if self.featured {
            "wf-plan is-featured"
        } else {
            "wf-plan"
        }
    }
}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/pricing_plans.html")]
pub struct PricingPlans<'a> {
    pub plans: &'a [PricingPlan<'a>],
}

impl<'a> PricingPlans<'a> {
    pub const fn new(plans: &'a [PricingPlan<'a>]) -> Self {
        Self { plans }
    }
}

impl<'a> askama::filters::HtmlSafe for PricingPlans<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/testimonial.html")]
pub struct Testimonial<'a> {
    pub quote_html: TrustedHtml<'a>,
    pub name: &'a str,
    pub role: &'a str,
}

impl<'a> Testimonial<'a> {
    pub const fn new(quote_html: TrustedHtml<'a>, name: &'a str, role: &'a str) -> Self {
        Self {
            quote_html,
            name,
            role,
        }
    }
}

impl<'a> askama::filters::HtmlSafe for Testimonial<'a> {}

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/marketing_section.html")]
pub struct MarketingSection<'a> {
    pub title: &'a str,
    pub content_html: TrustedHtml<'a>,
    pub kicker: Option<&'a str>,
    pub subtitle: Option<&'a str>,
}

impl<'a> MarketingSection<'a> {
    pub const fn new(title: &'a str, content_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            content_html,
            kicker: None,
            subtitle: None,
        }
    }

    pub const fn with_kicker(mut self, kicker: &'a str) -> Self {
        self.kicker = Some(kicker);
        self
    }

    pub const fn with_subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }
}

impl<'a> askama::filters::HtmlSafe for MarketingSection<'a> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_button_with_htmx_attrs() {
        let attrs = [HtmlAttr::hx_post("/save?next=<home>")];
        let html = Button::primary("Save").with_attrs(&attrs).render().unwrap();

        assert!(html.contains(r#"class="wf-btn primary""#));
        assert!(html.contains(r#"hx-post="/save?next="#));
        assert!(!html.contains(r#"hx-post="/save?next=<home>""#));
    }

    #[test]
    fn field_escapes_copy_and_renders_trusted_control_html() {
        let html = Field::new(
            "Email <required>",
            TrustedHtml::new(r#"<input class="wf-input" name="email">"#),
        )
        .with_hint("Use <work> address")
        .render()
        .unwrap();

        assert!(html.contains("Email"));
        assert!(!html.contains("Email <required>"));
        assert!(html.contains(r#"<input class="wf-input" name="email">"#));
        assert!(html.contains("Use"));
        assert!(!html.contains("Use <work> address"));
    }

    #[test]
    fn trusted_html_writes_without_formatter_allocation() {
        let mut html = String::new();

        askama::FastWritable::write_into(
            &TrustedHtml::new("<strong>Ready</strong>"),
            &mut html,
            askama::NO_VALUES,
        )
        .unwrap();

        assert_eq!(html, "<strong>Ready</strong>");
    }

    #[derive(Template)]
    #[template(source = "{{ button }}", ext = "html")]
    struct NestedButton<'a> {
        button: Button<'a>,
    }

    #[test]
    fn nested_components_render_as_html() {
        let html = NestedButton {
            button: Button::primary("Save"),
        }
        .render()
        .unwrap();

        assert!(html.contains("<button"));
        assert!(!html.contains("&lt;button"));
    }

    #[test]
    fn action_primitives_render_wave_funk_markup() {
        let attrs = [HtmlAttr::hx_post("/actions/archive")];
        let buttons = [
            Button::new("Left"),
            Button::primary("Archive").with_attrs(&attrs),
        ];

        let group_html = ButtonGroup::new(&buttons).render().unwrap();
        let split_html = SplitButton::new(Button::primary("Run"), Button::new("More"))
            .render()
            .unwrap();
        let icon_html = IconButton::new(TrustedHtml::new("&times;"), "Close")
            .with_variant(ButtonVariant::Ghost)
            .render()
            .unwrap();

        assert!(group_html.contains(r#"class="wf-btn-group""#));
        assert!(group_html.contains(r#"hx-post="/actions/archive""#));
        assert!(split_html.contains(r#"class="wf-btn-split""#));
        assert!(split_html.contains(r#"class="wf-btn caret""#));
        assert!(icon_html.contains(r#"class="wf-icon-btn ghost""#));
        assert!(icon_html.contains(r#"aria-label="Close""#));
        assert!(icon_html.contains("&times;"));
    }

    #[test]
    fn text_form_primitives_escape_copy_and_attrs() {
        let attrs = [HtmlAttr::hx_get("/validate/email")];
        let input_html = Input::email("email")
            .with_value("sandeep<wavefunk>")
            .with_placeholder("Email <address>")
            .with_attrs(&attrs)
            .render()
            .unwrap();
        let textarea_html = Textarea::new("notes")
            .with_value("Hello <team>")
            .with_placeholder("Notes <optional>")
            .render()
            .unwrap();
        let options = [
            SelectOption::new("starter", "Starter"),
            SelectOption::new("pro", "Pro <team>").selected(),
        ];
        let select_html = Select::new("plan", &options).render().unwrap();

        assert!(input_html.contains(r#"class="wf-input""#));
        assert!(input_html.contains(r#"type="email""#));
        assert!(input_html.contains(r#"hx-get="/validate/email""#));
        assert!(!input_html.contains("sandeep<wavefunk>"));
        assert!(!input_html.contains("Email <address>"));
        assert!(textarea_html.contains(r#"class="wf-textarea""#));
        assert!(!textarea_html.contains("Hello <team>"));
        assert!(select_html.contains(r#"class="wf-select""#));
        assert!(select_html.contains(r#"value="pro" selected"#));
        assert!(!select_html.contains("Pro <team>"));
    }

    #[test]
    fn grouped_choice_and_range_primitives_render_expected_classes() {
        let input_html = Input::url("site_url").render().unwrap();
        let group_html = InputGroup::new(TrustedHtml::new(&input_html))
            .with_prefix("https://")
            .with_suffix(".wavefunk.test")
            .render()
            .unwrap();
        let checkbox_html = CheckRow::checkbox("terms", "yes", "Accept <terms>")
            .checked()
            .render()
            .unwrap();
        let radio_html = CheckRow::radio("plan", "pro", "Pro").render().unwrap();
        let switch_html = Switch::new("enabled").checked().render().unwrap();
        let range_html = Range::new("volume")
            .with_bounds("0", "100")
            .with_value("50")
            .render()
            .unwrap();
        let field_html = Field::new("URL", TrustedHtml::new(&group_html))
            .with_state(FieldState::Success)
            .render()
            .unwrap();

        assert!(group_html.contains(r#"class="wf-input-group""#));
        assert!(group_html.contains(r#"class="wf-input-addon">https://"#));
        assert!(checkbox_html.contains(r#"class="wf-check-row""#));
        assert!(checkbox_html.contains(r#"type="checkbox""#));
        assert!(checkbox_html.contains("checked"));
        assert!(!checkbox_html.contains("Accept <terms>"));
        assert!(radio_html.contains(r#"type="radio""#));
        assert!(switch_html.contains(r#"class="wf-switch""#));
        assert!(switch_html.contains("checked"));
        assert!(range_html.contains(r#"class="wf-range""#));
        assert!(range_html.contains(r#"min="0""#));
        assert!(field_html.contains(r#"class="wf-field is-success""#));
    }

    #[test]
    fn layout_navigation_and_data_primitives_render_expected_markup() {
        let panel = Panel::new("Deployments", TrustedHtml::new("<p>Ready</p>"))
            .with_action(TrustedHtml::new(
                r#"<a class="wf-panel-link" href="/all">All</a>"#,
            ))
            .render()
            .unwrap();
        let card = Card::new("Project <alpha>", TrustedHtml::new("<p>Live</p>"))
            .with_kicker("Status")
            .raised()
            .render()
            .unwrap();
        let stats = [Stat::new("Requests", "42").with_unit("rpm")];
        let stat_row = StatRow::new(&stats).render().unwrap();
        let badge = Badge::muted("beta").render().unwrap();
        let avatar = Avatar::new("SN").accent().render().unwrap();
        let crumbs = [
            BreadcrumbItem::link("Projects", "/projects"),
            BreadcrumbItem::current("Wavefunk <UI>"),
        ];
        let breadcrumbs = Breadcrumbs::new(&crumbs).render().unwrap();
        let tabs = [
            TabItem::link("Overview", "/").active(),
            TabItem::link("Settings", "/settings"),
        ];
        let tab_html = Tabs::new(&tabs).render().unwrap();
        let segments = [
            SegmentOption::new("List", "list").active(),
            SegmentOption::new("Grid", "grid"),
        ];
        let seg_html = SegmentedControl::new(&segments).render().unwrap();
        let pages = [
            PageLink::link("1", "/page/1").active(),
            PageLink::ellipsis(),
            PageLink::disabled("Next"),
        ];
        let pagination = Pagination::new(&pages).render().unwrap();
        let nav_section = NavSection::new("Workspace").render().unwrap();
        let nav_item = NavItem::new("Dashboard", "/").active().with_count("3");
        let topbar = Topbar::new(TrustedHtml::new(&breadcrumbs), TrustedHtml::new(&badge))
            .render()
            .unwrap();
        let statusbar = Statusbar::new("Connected", "v0.1").render().unwrap();
        let empty = EmptyState::new("No hooks", "Create a hook to start.")
            .with_glyph(TrustedHtml::new("&empty;"))
            .bordered()
            .render()
            .unwrap();
        let table_headers = [TableHeader::new("Name"), TableHeader::numeric("Runs")];
        let table_cells = [TableCell::strong("Build <main>"), TableCell::numeric("12")];
        let table_rows = [TableRow::new(&table_cells).selected()];
        let table = Table::new(&table_headers, &table_rows)
            .interactive()
            .render()
            .unwrap();
        let dl_items = [DefinitionItem::new("Runtime", "Rust <stable>")];
        let dl = DefinitionList::new(&dl_items).render().unwrap();
        let grid = Grid::new(TrustedHtml::new(&card))
            .with_columns(2)
            .render()
            .unwrap();
        let split = Split::new(TrustedHtml::new(&panel))
            .vertical()
            .render()
            .unwrap();

        assert!(panel.contains(r#"class="wf-panel""#));
        assert!(card.contains(r#"class="wf-card is-raised""#));
        assert!(!card.contains("Project <alpha>"));
        assert!(stat_row.contains(r#"class="wf-stat-row""#));
        assert!(badge.contains(r#"class="wf-badge muted""#));
        assert!(avatar.contains(r#"class="wf-avatar accent""#));
        assert!(breadcrumbs.contains(r#"class="wf-crumbs""#));
        assert!(!breadcrumbs.contains("Wavefunk <UI>"));
        assert!(tab_html.contains(r#"class="wf-tabs""#));
        assert!(seg_html.contains(r#"class="wf-seg""#));
        assert!(pagination.contains(r#"class="wf-pagination""#));
        assert!(nav_section.contains(r#"class="wf-nav-section""#));
        assert!(
            nav_item
                .render()
                .unwrap()
                .contains(r#"class="wf-nav-item is-active""#)
        );
        assert!(topbar.contains(r#"class="wf-topbar""#));
        assert!(statusbar.contains(r#"class="wf-statusbar wf-hair""#));
        assert!(empty.contains(r#"class="wf-empty bordered""#));
        assert!(table.contains(r#"class="wf-table is-interactive""#));
        assert!(!table.contains("Build <main>"));
        assert!(dl.contains(r#"class="wf-dl""#));
        assert!(!dl.contains("Rust <stable>"));
        assert!(grid.contains(r#"class="wf-grid cols-2""#));
        assert!(split.contains(r#"class="wf-split vertical""#));
    }

    #[test]
    fn page_header_supports_title_meta_back_and_action_slots() {
        let primary = Button::primary("Create").render().unwrap();
        let secondary = Button::new("Export").render().unwrap();
        let header = PageHeader::new("Deployments <prod>")
            .with_subtitle("Filtered by team <ops>")
            .with_back("/settings", "Settings")
            .with_meta(TrustedHtml::new(
                r#"<span class="wf-badge muted">12</span>"#,
            ))
            .with_primary(TrustedHtml::new(&primary))
            .with_secondary(TrustedHtml::new(&secondary))
            .render()
            .unwrap();

        assert!(header.contains(r#"class="wf-pageheader""#));
        assert!(header.contains(r#"class="wf-pageheader-main""#));
        assert!(header.contains(r#"<a class="wf-backlink" href="/settings">"#));
        assert!(header.contains(">Settings<"));
        assert!(header.contains(r#"class="wf-pagetitle""#));
        assert!(!header.contains("Deployments <prod>"));
        assert!(header.contains(r#"class="wf-pageheader-subtitle""#));
        assert!(!header.contains("Filtered by team <ops>"));
        assert!(header.contains(r#"<span class="wf-badge muted">12</span>"#));
        assert!(header.contains(r#"class="wf-pageheader-actions""#));
        assert!(header.contains(">Create<"));
        assert!(header.contains(">Export<"));
    }

    #[test]
    fn feedback_overlay_and_loading_primitives_render_expected_markup() {
        let callout = Callout::new(FeedbackKind::Warn, TrustedHtml::new("<p>Heads up</p>"))
            .with_title("Warning")
            .render()
            .unwrap();
        let toast = Toast::new(FeedbackKind::Ok, "Saved <now>")
            .render()
            .unwrap();
        let toast_host = ToastHost::new().render().unwrap();
        let tooltip = Tooltip::new("Copy id", TrustedHtml::new(r#"<button>copy</button>"#))
            .render()
            .unwrap();
        let menu_items = [
            MenuItem::button("Open"),
            MenuItem::link("Settings", "/settings"),
            MenuItem::separator(),
            MenuItem::button("Delete").danger(),
        ];
        let menu = Menu::new(&menu_items).render().unwrap();
        let popover = Popover::new(
            TrustedHtml::new(r#"<button data-popover-toggle>Open</button>"#),
            TrustedHtml::new(&menu),
        )
        .with_heading("Menu")
        .open()
        .render()
        .unwrap();
        let modal = Modal::new("Confirm", TrustedHtml::new("<p>Continue?</p>"))
            .with_footer(TrustedHtml::new(
                r#"<button class="wf-btn primary">Confirm</button>"#,
            ))
            .open()
            .render()
            .unwrap();
        let drawer = Drawer::new("Details", TrustedHtml::new("<p>Side sheet</p>"))
            .left()
            .open()
            .render()
            .unwrap();
        let skeleton = Skeleton::title().render().unwrap();
        let spinner = Spinner::large().render().unwrap();
        let minibuffer = Minibuffer::new()
            .with_message(FeedbackKind::Info, "Queued <job>")
            .with_time("09:41")
            .render()
            .unwrap();
        let minibuffer_echo = MinibufferEcho::warn("Queued <job>").render().unwrap();

        assert!(callout.contains(r#"class="wf-callout warn""#));
        assert!(toast.contains(r#"class="wf-toast ok""#));
        assert!(!toast.contains("Saved <now>"));
        assert!(toast_host.contains(r#"class="wf-toast-host""#));
        assert!(tooltip.contains(r#"class="wf-tooltip""#));
        assert!(tooltip.contains(r#"data-tip="Copy id""#));
        assert!(menu.contains(r#"class="wf-menu""#));
        assert!(menu.contains(r#"class="wf-menu-sep""#));
        assert!(popover.contains(r#"class="wf-popover is-open""#));
        assert!(modal.contains(r#"class="wf-modal is-open""#));
        assert!(modal.contains(r#"class="wf-overlay is-open""#));
        assert!(modal.contains(r#"data-wf-dismiss="overlay""#));
        assert!(drawer.contains(r#"class="wf-drawer is-open left""#));
        assert!(drawer.contains(r#"data-wf-dismiss="overlay""#));
        assert!(skeleton.contains(r#"class="wf-skeleton title""#));
        assert!(spinner.contains(r#"class="wf-spinner lg""#));
        assert!(minibuffer.contains(r#"class="wf-minibuffer""#));
        assert!(minibuffer.contains("data-wf-echo"));
        assert!(!minibuffer.contains("Queued <job>"));
        assert!(minibuffer_echo.contains(r#"hidden"#));
        assert!(minibuffer_echo.contains(r#"data-wf-echo-kind="warn""#));
        assert!(minibuffer_echo.contains(r#"data-wf-echo-message="Queued "#));
        assert!(!minibuffer_echo.contains("Queued <job>"));
    }

    #[test]
    fn modal_size_and_spacing_utilities_cover_large_overlay_layouts() {
        let modal = Modal::new("Edit record", TrustedHtml::new("<p>Large form</p>"))
            .large()
            .open()
            .render()
            .unwrap();
        let components_css = include_str!("../static/wavefunk/css/04-components.css");
        let utilities_css = include_str!("../static/wavefunk/css/05-utilities.css");

        assert!(modal.contains(r#"class="wf-modal is-open wf-modal--lg""#));
        assert!(components_css.contains(".wf-modal--lg"));
        assert!(utilities_css.contains(".wf-mb-1 { margin-bottom: 4px; }"));
        assert!(utilities_css.contains(".wf-mb-8 { margin-bottom: 32px; }"));
        assert!(utilities_css.contains(".wf-ml-2 { margin-left: 8px; }"));
    }

    #[test]
    fn form_composition_and_dropzone_components_render_expected_markup() {
        let input_html = Input::email("email")
            .with_placeholder("you@example.test")
            .render()
            .unwrap();
        let field_html = Field::new("Email", TrustedHtml::new(&input_html))
            .with_hint("Use <work> address")
            .render()
            .unwrap();
        let actions_html = FormActions::new(TrustedHtml::new(
            r#"<button class="wf-btn primary">Save</button>"#,
        ))
        .with_secondary(TrustedHtml::new(
            r#"<button class="wf-btn">Cancel</button>"#,
        ))
        .render()
        .unwrap();
        let section_html = FormSection::new("Profile <setup>", TrustedHtml::new(&field_html))
            .with_description("Shown to teammates <public>")
            .render()
            .unwrap();
        let attrs = [HtmlAttr::hx_post("/profile")];
        let form_html = Form::new(TrustedHtml::new(&section_html))
            .with_action("/profile/save?next=<home>")
            .with_method("post")
            .with_attrs(&attrs)
            .render()
            .unwrap();
        let dropzone_attrs = [HtmlAttr::new("data-intent", "avatar <upload>")];
        let dropzone_html = Dropzone::new("avatar")
            .with_title("Drop avatar <image>")
            .with_hint("PNG or JPG <2MB>")
            .with_accept("image/png,image/jpeg")
            .with_attrs(&dropzone_attrs)
            .multiple()
            .disabled()
            .dragover()
            .render()
            .unwrap();

        assert!(actions_html.contains(r#"class="wf-form-actions""#));
        assert!(section_html.contains(r#"class="wf-form-section""#));
        assert!(!section_html.contains("Profile <setup>"));
        assert!(!section_html.contains("Shown to teammates <public>"));
        assert!(form_html.contains(r#"<form class="wf-form""#));
        assert!(form_html.contains(r#"method="post""#));
        assert!(form_html.contains(r#"hx-post="/profile""#));
        assert!(!form_html.contains(r#"action="/profile/save?next=<home>""#));
        assert!(dropzone_html.contains(r#"class="wf-dropzone is-dragover is-disabled""#));
        assert!(dropzone_html.contains("data-upload-zone"));
        assert!(dropzone_html.contains(r#"type="file""#));
        assert!(dropzone_html.contains(r#"multiple"#));
        assert!(dropzone_html.contains(r#"disabled"#));
        assert!(dropzone_html.contains(r#"accept="image/png,image/jpeg""#));
        assert!(dropzone_html.contains(r#"data-intent="avatar "#));
        assert!(!dropzone_html.contains(r#"data-intent="avatar <upload>""#));
        assert!(!dropzone_html.contains("Drop avatar <image>"));
        assert!(!dropzone_html.contains("PNG or JPG <2MB>"));
    }

    #[test]
    fn generated_form_building_blocks_render_generic_schema_shapes() {
        let title = Input::new("title").render().unwrap();
        let title_field = Field::new("Title", TrustedHtml::new(&title))
            .render()
            .unwrap();
        let object = ObjectFieldset::new("Metadata", TrustedHtml::new(&title_field))
            .with_description("Nested object fields")
            .render()
            .unwrap();
        let item = RepeatableItem::new("Link 1", TrustedHtml::new(&title_field))
            .with_actions(TrustedHtml::new(
                r#"<button class="wf-btn sm">Remove</button>"#,
            ))
            .render()
            .unwrap();
        let array = RepeatableArray::new("Links", TrustedHtml::new(&item))
            .with_description("Zero or more external links.")
            .with_action(TrustedHtml::new(
                r#"<button class="wf-btn sm">Add link</button>"#,
            ))
            .render()
            .unwrap();
        let upload = CurrentUpload::new("Hero image", "/media/hero.jpg", "hero.jpg")
            .with_meta("1200x630 JPG")
            .with_thumbnail(TrustedHtml::new(r#"<img src="/media/hero.jpg" alt="">"#))
            .render()
            .unwrap();
        let options = [
            SelectOption::new("home", "Home"),
            SelectOption::new("about", "About").selected(),
        ];
        let select = Select::new("related_page", &options).render().unwrap();
        let reference = ReferenceSelect::new("Related page", TrustedHtml::new(&select))
            .with_hint("Search and choose another record.")
            .render()
            .unwrap();
        let markdown = MarkdownTextarea::new("body")
            .with_value("# Hello")
            .with_rows(8)
            .render()
            .unwrap();
        let richtext = RichTextHost::new("body-editor", "body_html")
            .with_value("<p>Hello</p>")
            .with_toolbar(TrustedHtml::new(
                r#"<button class="wf-btn sm">Bold</button>"#,
            ))
            .render()
            .unwrap();

        assert!(object.contains(r#"<fieldset class="wf-object-fieldset">"#));
        assert!(object.contains(r#"<legend class="wf-object-legend">Metadata</legend>"#));
        assert!(array.contains(r#"class="wf-repeatable-array""#));
        assert!(array.contains(r#"class="wf-repeatable-item""#));
        assert!(upload.contains(r#"class="wf-current-upload""#));
        assert!(upload.contains(r#"<a href="/media/hero.jpg">hero.jpg</a>"#));
        assert!(reference.contains(r#"class="wf-reference-select""#));
        assert!(markdown.contains(r#"class="wf-textarea wf-markdown-textarea""#));
        assert!(markdown.contains("data-wf-markdown"));
        assert!(richtext.contains(r#"class="wf-richtext""#));
        assert!(richtext.contains("data-wf-richtext"));
        assert!(richtext.contains(r#"data-wf-richtext-modal-host"#));
    }

    #[test]
    fn table_workflow_components_support_sorting_actions_and_chrome() {
        let _source_compatible_header = TableHeader {
            label: "Legacy",
            numeric: false,
        };
        let _source_compatible_cell = TableCell {
            text: "Legacy",
            numeric: false,
            strong: false,
            muted: false,
        };
        let headers = [
            DataTableHeader::new("Name").sortable("name", SortDirection::Ascending),
            DataTableHeader::numeric("Runs").with_width(TableColumnWidth::Small),
            DataTableHeader::new("Actions").action_column(),
        ];
        let actions = IconButton::new(TrustedHtml::new("&times;"), "Stop")
            .with_variant(ButtonVariant::Danger)
            .render()
            .unwrap();
        let row_cells = [
            DataTableCell::strong("Build <main>"),
            DataTableCell::numeric("12"),
            DataTableCell::html(TrustedHtml::new(&actions)),
        ];
        let rows = [DataTableRow::new(&row_cells).selected()];
        let filter_html = Input::new("q")
            .with_size(ControlSize::Small)
            .with_placeholder("Search")
            .render()
            .unwrap();
        let bulk_html = Button::new("Delete").render().unwrap();
        let table_html = DataTable::new(&headers, &rows)
            .interactive()
            .sticky()
            .pin_last()
            .render()
            .unwrap();
        let wrap_html = TableWrap::new(TrustedHtml::new(&table_html))
            .with_filterbar(TrustedHtml::new(&filter_html))
            .with_bulkbar("1 selected", TrustedHtml::new(&bulk_html))
            .with_footer(TrustedHtml::new("Showing 1-1 of 1"))
            .render()
            .unwrap();

        assert!(table_html.contains(r#"class="wf-sort-h is-active""#));
        assert!(table_html.contains(r#"data-sort-key="name""#));
        assert!(table_html.contains(r#"class="wf-sort-arrow">^"#));
        assert!(table_html.contains(r#"class="wf-col-sm num""#));
        assert!(table_html.contains(r#"class="wf-col-act""#));
        assert!(table_html.contains("&times;"));
        assert!(!table_html.contains("Build <main>"));
        assert!(wrap_html.contains(r#"class="wf-tablewrap""#));
        assert!(wrap_html.contains(r#"class="wf-filterbar""#));
        assert!(wrap_html.contains(r#"class="wf-bulkbar""#));
        assert!(wrap_html.contains(r#"class="wf-tablefoot""#));
    }

    #[test]
    fn owned_data_table_accepts_dynamic_strings_and_trusted_html_cells() {
        let headers = [
            DataTableHeader::new("Name").sortable("name", SortDirection::Ascending),
            DataTableHeader::numeric("Runs").with_width(TableColumnWidth::Small),
            DataTableHeader::new("Actions").action_column(),
        ];
        let rows = [("Build <main>".to_owned(), 12)]
            .into_iter()
            .map(|(name, runs)| {
                OwnedDataTableRow::new([
                    OwnedDataTableCell::strong(name),
                    OwnedDataTableCell::numeric(runs.to_string()),
                    OwnedDataTableCell::html(TrustedHtmlBuf::new(
                        r#"<button class="wf-icon-btn danger" type="button">Stop</button>"#,
                    )),
                ])
                .selected()
            })
            .collect::<Vec<_>>();

        let html = OwnedDataTable::new(&headers, rows)
            .interactive()
            .render()
            .unwrap();

        assert!(html.contains(r#"class="wf-table is-interactive""#));
        assert!(html.contains(r#"class="is-selected" aria-selected="true""#));
        assert!(html.contains(r#"class="wf-col-sm num""#));
        assert!(html.contains(r#"class="wf-col-act""#));
        assert!(!html.contains("Build <main>"));
        assert!(html.contains("Build "));
        assert!(html.contains(r#"<button class="wf-icon-btn danger" type="button">Stop</button>"#));
    }

    #[test]
    fn resource_table_chrome_components_compose_filter_bulk_footer_and_selection() {
        let filter_input = Input::search("q")
            .with_placeholder("Search resources")
            .with_size(ControlSize::Small)
            .render()
            .unwrap();
        let filter_action = Button::new("Refresh").render().unwrap();
        let filterbar = FilterBar::new(TrustedHtml::new(&filter_input))
            .with_actions(TrustedHtml::new(&filter_action))
            .render()
            .unwrap();
        let bulk_action = Button::new("Delete")
            .with_variant(ButtonVariant::Danger)
            .render()
            .unwrap();
        let bulkbar = BulkActionBar::new("2 selected", TrustedHtml::new(&bulk_action))
            .render()
            .unwrap();
        let footer_action = Pagination::new(&[
            PageLink::link("1", "/page/1").active(),
            PageLink::link("2", "/page/2"),
        ])
        .render()
        .unwrap();
        let footer = TableFooter::new(TrustedHtml::new("Showing 1-2 of 8"))
            .with_actions(TrustedHtml::new(&footer_action))
            .render()
            .unwrap();
        let selector = RowSelect::new("selected", "build", "Select Build")
            .checked()
            .render()
            .unwrap();
        let headers = [
            DataTableHeader::new("").with_width(TableColumnWidth::Checkbox),
            DataTableHeader::sorted("Name", "name", SortDirection::Ascending),
        ];
        let cells = [
            DataTableCell::html(TrustedHtml::new(&selector)),
            DataTableCell::strong("Build"),
        ];
        let rows = [DataTableRow::new(&cells).selected()];
        let table = DataTable::new(&headers, &rows)
            .interactive()
            .render()
            .unwrap();
        let wrap = TableWrap::new(TrustedHtml::new(&table))
            .with_filterbar_component(TrustedHtml::new(&filterbar))
            .with_bulkbar_component(TrustedHtml::new(&bulkbar))
            .with_footer_component(TrustedHtml::new(&footer))
            .render()
            .unwrap();

        assert!(filterbar.contains(r#"class="wf-filterbar""#));
        assert!(filterbar.contains(r#"class="wf-filterbar-actions""#));
        assert!(bulkbar.contains(r#"class="wf-bulkbar""#));
        assert!(bulkbar.contains(r#"class="wf-sel-count">2 selected"#));
        assert!(footer.contains(r#"class="wf-tablefoot""#));
        assert!(footer.contains(r#"class="wf-tablefoot-actions""#));
        assert!(selector.contains(r#"class="wf-check wf-rowselect""#));
        assert!(selector.contains(r#"aria-label="Select Build""#));
        assert!(selector.contains("checked"));
        assert!(table.contains(r#"data-sort-key="name""#));
        assert!(wrap.matches(r#"class="wf-filterbar""#).count() == 1);
        assert!(wrap.matches(r#"class="wf-bulkbar""#).count() == 1);
        assert!(wrap.matches(r#"class="wf-tablefoot""#).count() == 1);
    }

    #[test]
    fn progress_stepper_and_disclosure_components_render_expected_markup() {
        let progress = Progress::new(60).render().unwrap();
        let indeterminate = Progress::indeterminate().render().unwrap();
        let meter = Meter::new(75)
            .with_size_px(96, 6)
            .with_color(MeterColor::Ok)
            .render()
            .unwrap();
        let kbd = Kbd::new("Ctrl <K>").render().unwrap();
        let steps = [
            StepItem::new("Account").done(),
            StepItem::new("Profile <public>")
                .active()
                .with_href("/profile"),
            StepItem::new("Invite"),
        ];
        let stepper = Stepper::new(&steps).render().unwrap();
        let accordion_items = [
            AccordionItem::new("What is <UI>?", TrustedHtml::new("<p>Typed</p>")).open(),
            AccordionItem::new("Can it htmx?", TrustedHtml::new("<p>Yes</p>")),
        ];
        let accordion = Accordion::new(&accordion_items).render().unwrap();
        let faq_items = [FaqItem::new(
            "Why typed?",
            TrustedHtml::new("<p>To preserve semver.</p>"),
        )];
        let faq = Faq::new(&faq_items).render().unwrap();

        assert!(progress.contains(r#"class="wf-progress""#));
        assert!(progress.contains(r#"style="--progress: 60%""#));
        assert!(indeterminate.contains(r#"class="wf-progress indeterminate""#));
        assert!(meter.contains(
            r#"style="--meter: 75%; --meter-w: 96px; --meter-h: 6px; --meter-c: var(--ok)""#
        ));
        assert!(kbd.contains(r#"class="wf-kbd""#));
        assert!(!kbd.contains("Ctrl <K>"));
        assert!(stepper.contains(r#"class="wf-step is-done""#));
        assert!(stepper.contains(r#"aria-current="step""#));
        assert!(!stepper.contains("Profile <public>"));
        assert!(accordion.contains(r#"class="wf-accordion""#));
        assert!(accordion.contains(r#"<details class="wf-accordion-item" open>"#));
        assert!(!accordion.contains("What is <UI>?"));
        assert!(faq.contains(r#"class="wf-faq""#));
        assert!(faq.contains("<p>To preserve semver.</p>"));
    }

    #[test]
    fn identity_brand_and_operational_components_render_expected_markup() {
        let avatars = [
            Avatar::new("SN").with_image("/avatar.png").accent(),
            Avatar::new("WF").with_size(AvatarSize::Small),
        ];
        let avatar_group = AvatarGroup::new(&avatars).render().unwrap();
        let full_user = UserButton::new("Wave Funk", "team@example.test", Avatar::new("WF"))
            .render()
            .unwrap();
        let user = UserButton::new(
            "Sandeep <Nambiar>",
            "sandeep@example.test",
            Avatar::new("SN"),
        )
        .compact()
        .render()
        .unwrap();
        let wordmark = Wordmark::new("Wave <Funk>")
            .with_mark(TrustedHtml::new(r#"<svg class="wf-mark"></svg>"#))
            .render()
            .unwrap();
        let ranks = [RankRow::new("Builds <main>", "42", 72)];
        let rank_list = RankList::new(&ranks).render().unwrap();
        let feed_rows = [FeedRow::new("09:41", "Deploy <prod>", "Released <v1>")];
        let feed = Feed::new(&feed_rows).render().unwrap();
        let timeline_items =
            [
                TimelineItem::new("09:42", "Queued <job>", TrustedHtml::new("<p>Pending</p>"))
                    .active(),
            ];
        let timeline = Timeline::new(&timeline_items).render().unwrap();
        let tree_children = [TreeItem::file("components.rs").active()];
        let tree_child_html = TreeView::new(&tree_children).nested().render().unwrap();
        let tree_items = [TreeItem::folder("src <root>")
            .collapsed()
            .with_children(TrustedHtml::new(&tree_child_html))];
        let tree = TreeView::new(&tree_items).render().unwrap();
        let framed = Framed::new(TrustedHtml::new("<code>cargo test</code>"))
            .dense()
            .dashed()
            .render()
            .unwrap();

        assert!(avatar_group.contains(r#"class="wf-avatar-group""#));
        assert!(avatar_group.contains(r#"<img src="/avatar.png" alt="SN">"#));
        assert!(full_user.contains(r#"class="wf-user""#));
        assert!(!full_user.contains(r#"class="wf-user compact""#));
        assert!(user.contains(r#"class="wf-user compact""#));
        assert!(!user.contains("Sandeep <Nambiar>"));
        assert!(wordmark.contains(r#"class="wf-wordmark""#));
        assert!(wordmark.contains(r#"<svg class="wf-mark"></svg>"#));
        assert!(!wordmark.contains("Wave <Funk>"));
        assert!(rank_list.contains(r#"class="wf-rank""#));
        assert!(rank_list.contains(r#"style="width: 72%""#));
        assert!(!rank_list.contains("Builds <main>"));
        assert!(feed.contains(r#"class="wf-feed""#));
        assert!(!feed.contains("Deploy <prod>"));
        assert!(!feed.contains("Released <v1>"));
        assert!(timeline.contains(r#"class="wf-timeline-item is-active""#));
        assert!(!timeline.contains("Queued <job>"));
        assert!(tree.contains(r#"class="wf-tree""#));
        assert!(tree.contains(r#"class="is-collapsed""#));
        assert!(!tree.contains("src <root>"));
        assert!(framed.contains(r#"class="wf-framed dense dashed""#));
    }

    #[test]
    fn settings_and_admin_workflow_primitives_render_generic_markup() {
        let input = Input::email("email").render().unwrap();
        let save = Button::primary("Save")
            .with_button_type("submit")
            .render()
            .unwrap();
        let row = InlineFormRow::new("Notification email", TrustedHtml::new(&input))
            .with_hint("Used for account notices <private>")
            .with_action(TrustedHtml::new(&save))
            .render()
            .unwrap();
        let copy = CopyableValue::new("Webhook URL", "webhook-url", "https://example.test/hook")
            .with_button_label("Copy URL")
            .render()
            .unwrap();
        let statuses = [
            CredentialStatusItem::ok("Mail", "Configured"),
            CredentialStatusItem::warn("Backups", "Rotation due"),
        ];
        let status_list = CredentialStatusList::new(&statuses).render().unwrap();
        let confirm = ConfirmAction::new("Delete workspace", "/settings/delete")
            .with_message("This cannot be undone.")
            .with_confirm("Delete this workspace?")
            .render()
            .unwrap();
        let section_body = format!("{row}{copy}{status_list}{confirm}");
        let section = SettingsSection::new("Workspace settings", TrustedHtml::new(&section_body))
            .with_description("Operational settings for this app.")
            .danger()
            .render()
            .unwrap();

        assert!(row.contains(r#"class="wf-inline-form-row""#));
        assert!(!row.contains("account notices <private>"));
        assert!(copy.contains(r#"class="wf-copyable""#));
        assert!(copy.contains(r#"id="webhook-url""#));
        assert!(copy.contains(r##"data-wf-copy="#webhook-url""##));
        assert!(copy.contains(">Copy URL<"));
        assert!(status_list.contains(r#"class="wf-credential-list""#));
        assert!(status_list.contains(r#"class="wf-tag ok""#));
        assert!(status_list.contains(r#"class="wf-tag warn""#));
        assert!(confirm.contains(
            r#"<form class="wf-confirm-action" action="/settings/delete" method="post">"#
        ));
        assert!(confirm.contains(r#"hx-confirm="Delete this workspace?""#));
        assert!(confirm.contains(r#"class="wf-btn danger""#));
        assert!(section.contains(r#"class="wf-panel wf-settings-section is-danger""#));
        assert!(section.contains("Operational settings for this app."));
    }

    #[test]
    fn split_shell_and_form_panel_render_generic_setup_surfaces() {
        let actions = Button::primary("Continue").render().unwrap();
        let panel = FormPanel::new(
            "Setup <workspace>",
            TrustedHtml::new(r#"<form class="wf-form">Fields</form>"#),
        )
        .with_subtitle("Use generic product copy <only>")
        .with_actions(TrustedHtml::new(&actions))
        .render()
        .unwrap();
        let attrs = [HtmlAttr::new("data-surface", "setup <flow>")];
        let shell = SplitShell::new(TrustedHtml::new(&panel))
            .with_top(TrustedHtml::new(
                r#"<a class="wf-btn ghost" href="/">Back</a>"#,
            ))
            .with_visual(TrustedHtml::new(r#"<pre aria-label="preview">wave</pre>"#))
            .with_footer(TrustedHtml::new(r#"<div class="wf-statusbar">Ready</div>"#))
            .with_mode("light")
            .mode_locked()
            .with_asset_base_path("/assets/wavefunk")
            .with_attrs(&attrs)
            .render()
            .unwrap();

        assert!(panel.contains(r#"class="wf-form-panel""#));
        assert!(!panel.contains("Setup <workspace>"));
        assert!(!panel.contains("generic product copy <only>"));
        assert!(panel.contains(r#"<form class="wf-form">Fields</form>"#));
        assert!(shell.contains(r#"class="wf-split-shell""#));
        assert!(shell.contains(r#"data-mode="light""#));
        assert!(shell.contains(r#"data-mode-locked"#));
        assert!(shell.contains(r#"data-wf-asset-base="/assets/wavefunk""#));
        assert!(shell.contains(r#"data-surface="setup "#));
        assert!(!shell.contains(r#"data-surface="setup <flow>""#));
        assert!(shell.contains(r#"<pre aria-label="preview">wave</pre>"#));
    }

    #[test]
    fn modeline_minibuffer_history_context_switcher_and_sidenav_are_generic() {
        let toggle_attrs = [HtmlAttr::new("data-mode-toggle", "")];
        let left = [
            ModelineSegment::chevron("WF"),
            ModelineSegment::buffer("workspace.rs"),
            ModelineSegment::button("Mode").with_attrs(&toggle_attrs),
        ];
        let right = [
            ModelineSegment::position("L12:C4"),
            ModelineSegment::text("Ready").with_feedback(FeedbackKind::Ok),
        ];
        let modeline = Modeline::new(&left).with_right(&right).render().unwrap();
        let history =
            [MinibufferHistoryRow::new("09:41", "Saved <draft>").with_feedback(FeedbackKind::Ok)];
        let minibuffer = Minibuffer::new()
            .with_prompt("wf")
            .with_message(FeedbackKind::Info, "Queued <job>")
            .with_history(&history)
            .render()
            .unwrap();
        let switcher_items = [
            ContextSwitcherItem::link("Production <east>", "/contexts/prod")
                .with_meta("3 apps")
                .active(),
            ContextSwitcherItem::link("Sandbox", "/contexts/sandbox")
                .with_badge(TrustedHtml::new(r#"<span class="wf-tag">test</span>"#)),
        ];
        let switcher = ContextSwitcher::new("Workspace", "Production", &switcher_items)
            .with_meta(TrustedHtml::new(r#"<span class="wf-tag ok">live</span>"#))
            .open()
            .render()
            .unwrap();
        let side_items = [
            SidenavItem::link("Overview", "/overview").active(),
            SidenavItem::link("Reports <beta>", "/reports")
                .muted()
                .with_badge("Soon"),
            SidenavItem::link("Billing", "/billing")
                .disabled()
                .with_coming_soon("coming soon"),
        ];
        let side_sections = [SidenavSection::new("Manage <workspace>", &side_items)];
        let sidenav = Sidenav::new(&side_sections).render().unwrap();
        let embedded_sidenav = Sidenav::new(&side_sections).embedded().render().unwrap();

        assert!(modeline.contains(r#"class="wf-modeline""#));
        assert!(modeline.contains(r#"class="wf-ml-seg wf-ml-chevron""#));
        assert!(modeline.contains(r#"data-mode-toggle="""#));
        assert!(modeline.contains(r#"class="wf-ml-seg wf-ml-pos""#));
        assert!(modeline.contains(r#"class="wf-ml-seg is-ok""#));
        assert!(modeline.contains(r#"class="wf-ml-fill""#));
        assert!(minibuffer.contains(r#"class="wf-minibuffer-history""#));
        assert!(!minibuffer.contains("Queued <job>"));
        assert!(!minibuffer.contains("Saved <draft>"));
        assert!(switcher.contains(r#"class="wf-context-switcher""#));
        assert!(switcher.contains(r#"<details class="wf-context-switcher" open>"#));
        assert!(!switcher.contains("Production <east>"));
        assert!(switcher.contains(r#"<span class="wf-tag">test</span>"#));
        assert!(sidenav.contains(r#"class="wf-sidenav""#));
        assert!(sidenav.contains(r#"class="wf-sidenav-item is-active""#));
        assert!(sidenav.contains(r#"class="wf-sidenav-item is-muted""#));
        assert!(sidenav.contains(r#"aria-disabled="true""#));
        assert!(!sidenav.contains("Manage <workspace>"));
        assert!(!sidenav.contains("Reports <beta>"));
        assert!(sidenav.contains(r#"<nav class="wf-sidenav""#));
        assert!(embedded_sidenav.contains(r#"<div class="wf-sidenav""#));
        assert!(!embedded_sidenav.contains(r#"<nav class="wf-sidenav""#));
    }

    #[test]
    fn secret_checklist_code_grid_snippets_and_strength_meter_are_product_neutral() {
        let secret = SecretValue::new("Recovery token", "recovery-token", "tok_<secret>")
            .with_warning("Shown once <store it>")
            .with_help(TrustedHtml::new(
                "<strong>Store this value securely.</strong>",
            ))
            .render()
            .unwrap();
        let checklist_items = [
            ChecklistItem::ok("DNS configured <edge>").with_description("Records verified."),
            ChecklistItem::warn("Webhook retry").with_status_label("review"),
        ];
        let checklist = Checklist::new(&checklist_items).render().unwrap();
        let codes = ["ABCD-EFGH", "IJKL<MNOP>"];
        let code_grid = CodeGrid::new(&codes)
            .with_label("One-time codes <backup>")
            .render()
            .unwrap();
        let block = CodeBlock::new("cargo add wavefunk-ui <latest>")
            .with_label("Install")
            .with_language("shell")
            .with_copy_target("install-command")
            .render()
            .unwrap();
        let tabs = [
            SnippetTab::new("Rust", r#"let value = "<typed>";"#)
                .with_language("rust")
                .active(),
            SnippetTab::new("Shell", "cargo test").with_language("shell"),
        ];
        let snippets = SnippetTabs::new("quickstart", &tabs).render().unwrap();
        let strength = StrengthMeter::new(3, 4, "Strong <enough>")
            .with_label("Key strength")
            .with_feedback(FeedbackKind::Ok)
            .live()
            .render()
            .unwrap();

        assert!(secret.contains(r#"class="wf-secret-value""#));
        assert!(secret.contains(r##"data-wf-copy="#recovery-token""##));
        assert!(!secret.contains("data-wf-copy-value"));
        assert!(secret.contains("********"));
        assert!(!secret.contains("tok_<secret>"));
        assert!(!secret.contains("Shown once <store it>"));
        assert!(secret.contains("<strong>Store this value securely.</strong>"));
        let copyable_masked = SecretValue::new("Raw token", "raw-token", "raw-secret")
            .copy_raw_value()
            .render()
            .unwrap();
        assert!(copyable_masked.contains(r#"data-wf-copy-value="raw-secret""#));
        assert!(checklist.contains(r#"class="wf-checklist""#));
        assert!(checklist.contains(r#"class="wf-checklist-item is-ok""#));
        assert!(checklist.contains(r#"class="wf-checklist-item is-warn""#));
        assert!(!checklist.contains("DNS configured <edge>"));
        assert!(code_grid.contains(r#"class="wf-code-grid""#));
        assert!(!code_grid.contains("IJKL<MNOP>"));
        assert!(!code_grid.contains("One-time codes <backup>"));
        assert!(block.contains(r#"class="wf-code-block""#));
        assert!(block.contains(r#"data-language="shell""#));
        assert!(block.contains(r##"data-wf-copy="#install-command""##));
        assert!(!block.contains("wavefunk-ui <latest>"));
        assert!(snippets.contains(r#"class="wf-snippet-tabs""#));
        assert!(snippets.contains(r#"role="tablist""#));
        assert!(snippets.contains(r##"data-wf-snippet-tab="#quickstart-panel-1""##));
        assert!(snippets.contains(r#"aria-controls="quickstart-panel-1""#));
        assert!(snippets.contains(r#"id="quickstart-panel-2""#));
        assert!(snippets.contains(r#"hidden"#));
        assert!(!snippets.contains(r#"let value = "<typed>";"#));
        assert!(strength.contains(r#"class="wf-strength-meter is-ok""#));
        assert!(strength.contains(r#"role="progressbar""#));
        assert!(strength.contains(r#"aria-valuenow="3""#));
        assert!(strength.contains(r#"aria-valuemax="4""#));
        assert!(strength.contains(r#"style="--strength: 75%""#));
        assert!(!strength.contains("Strong <enough>"));
    }

    #[test]
    fn marketing_primitives_render_stable_typed_sections() {
        let features = [
            FeatureItem::new("Typed <APIs>", "No struct literal churn."),
            FeatureItem::new("Embedded assets", "Self-contained binaries."),
        ];
        let feature_grid = FeatureGrid::new(&features).render().unwrap();
        let steps = [
            MarketingStep::new("Install", "Add the crate."),
            MarketingStep::new("Render", "Use Askama templates."),
        ];
        let step_grid = MarketingStepGrid::new(&steps).render().unwrap();
        let plans = [
            PricingPlan::new("Starter", "$9")
                .with_blurb("For small teams.")
                .featured(),
            PricingPlan::new("Scale", "$29"),
        ];
        let pricing = PricingPlans::new(&plans).render().unwrap();
        let testimonial = Testimonial::new(
            TrustedHtml::new("<p>Fast to wire.</p>"),
            "Operator <one>",
            "Founder",
        )
        .render()
        .unwrap();
        let section = MarketingSection::new("Component <system>", TrustedHtml::new(&feature_grid))
            .with_kicker("Wave Funk")
            .with_subtitle("Typed primitives for Rust apps.")
            .render()
            .unwrap();

        assert!(feature_grid.contains(r#"class="mk-features""#));
        assert!(!feature_grid.contains("Typed <APIs>"));
        assert!(step_grid.contains(r#"class="mk-steps""#));
        assert!(pricing.contains(r#"class="wf-plans""#));
        assert!(pricing.contains(r#"class="wf-plan is-featured""#));
        assert!(testimonial.contains(r#"class="wf-testimonial""#));
        assert!(!testimonial.contains("Operator <one>"));
        assert!(section.contains(r#"class="mk-sect""#));
        assert!(!section.contains("Component <system>"));
    }
}
