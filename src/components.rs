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
}
