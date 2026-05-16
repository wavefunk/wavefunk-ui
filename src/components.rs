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

impl askama::FastWritable for TrustedHtml<'_> {
    #[inline]
    fn write_into(&self, dest: &mut dyn fmt::Write, _: &dyn askama::Values) -> askama::Result<()> {
        Ok(dest.write_str(self.html)?)
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

    pub fn class_name(&self) -> String {
        let numeric = if self.numeric { "num" } else { "" };
        let strong = if self.strong { " strong" } else { "" };
        let muted = if self.muted { " muted" } else { "" };
        format!("{numeric}{strong}{muted}")
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

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/modal.html")]
pub struct Modal<'a> {
    pub title: &'a str,
    pub body_html: TrustedHtml<'a>,
    pub footer_html: Option<TrustedHtml<'a>>,
    pub open: bool,
}

impl<'a> Modal<'a> {
    pub const fn new(title: &'a str, body_html: TrustedHtml<'a>) -> Self {
        Self {
            title,
            body_html,
            footer_html: None,
            open: false,
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

    pub fn overlay_class(&self) -> &'static str {
        if self.open {
            "wf-overlay is-open"
        } else {
            "wf-overlay"
        }
    }

    pub fn modal_class(&self) -> &'static str {
        if self.open {
            "wf-modal is-open"
        } else {
            "wf-modal"
        }
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

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "components/minibuffer.html")]
pub struct Minibuffer<'a> {
    pub prompt: &'a str,
    pub message: Option<&'a str>,
    pub kind: Option<FeedbackKind>,
    pub time: Option<&'a str>,
}

impl<'a> Minibuffer<'a> {
    pub const fn new() -> Self {
        Self {
            prompt: ">",
            message: None,
            kind: None,
            time: None,
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
        assert!(drawer.contains(r#"class="wf-drawer is-open left""#));
        assert!(skeleton.contains(r#"class="wf-skeleton title""#));
        assert!(spinner.contains(r#"class="wf-spinner lg""#));
        assert!(minibuffer.contains(r#"class="wf-minibuffer""#));
        assert!(minibuffer.contains("data-wf-echo"));
        assert!(!minibuffer.contains("Queued <job>"));
    }
}
