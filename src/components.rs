use askama::Template;
use std::fmt;

#[derive(Clone, Copy, Debug)]
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

impl askama::filters::HtmlSafe for TrustedHtml<'_> {}

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
}
