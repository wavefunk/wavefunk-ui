use crate::{
    assets,
    components::{Avatar, HtmlAttr, MenuItem, MenuItemKind},
    html,
};
use askama::Template;

pub const DEFAULT_PROFILE_LOGOUT_CONFIRM: &str = "Log out of this session?";

pub const DEFAULT_PROFILE_LOGOUT_ATTRS: [HtmlAttr<'static>; 2] = [
    HtmlAttr::hx_post("/logout"),
    HtmlAttr::hx_confirm(DEFAULT_PROFILE_LOGOUT_CONFIRM),
];

pub const DEFAULT_PROFILE_MENU_ITEMS: [MenuItem<'static>; 3] = [
    MenuItem::link("Settings", "/settings"),
    MenuItem::separator(),
    MenuItem::button("Logout")
        .danger()
        .with_attrs(&DEFAULT_PROFILE_LOGOUT_ATTRS),
];

#[derive(Debug, Template)]
#[non_exhaustive]
#[template(path = "layouts/sidebar_profile.html")]
pub struct SidebarProfile<'a> {
    pub name: Option<&'a str>,
    pub email: Option<&'a str>,
    pub avatar: Option<Avatar<'a>>,
    pub menu_items: &'a [MenuItem<'a>],
}

impl<'a> SidebarProfile<'a> {
    pub const fn new() -> Self {
        Self {
            name: None,
            email: None,
            avatar: None,
            menu_items: &DEFAULT_PROFILE_MENU_ITEMS,
        }
    }

    pub const fn with_name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub const fn with_email(mut self, email: &'a str) -> Self {
        self.email = Some(email);
        self
    }

    pub const fn with_avatar(mut self, avatar: Avatar<'a>) -> Self {
        self.avatar = Some(avatar);
        self
    }

    pub const fn with_menu_items(mut self, menu_items: &'a [MenuItem<'a>]) -> Self {
        self.menu_items = menu_items;
        self
    }

    pub fn display_label(&self) -> &str {
        self.name.or(self.email).unwrap_or("Profile")
    }

    pub const fn secondary_email(&self) -> Option<&'a str> {
        if self.name.is_some() {
            self.email
        } else {
            None
        }
    }
}

impl<'a> Default for SidebarProfile<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> askama::filters::HtmlSafe for SidebarProfile<'a> {}

#[derive(Debug, Template)]
#[template(path = "layouts/app_shell.html")]
pub struct AppShell<'a> {
    pub title: &'a str,
    pub app_name: &'a str,
    pub mode: &'a str,
    pub density_class: &'a str,
    pub asset_base_path: &'a str,
    pub nav_html: &'a str,
    pub actions_html: &'a str,
    pub content_html: &'a str,
    pub profile: Option<SidebarProfile<'a>>,
    pub status_left: &'a str,
    pub status_right: &'a str,
}

impl<'a> AppShell<'a> {
    pub const fn new(title: &'a str, app_name: &'a str, content_html: &'a str) -> Self {
        Self {
            title,
            app_name,
            mode: "dark",
            density_class: "density-dense",
            asset_base_path: assets::DEFAULT_BASE_PATH,
            nav_html: "",
            actions_html: "",
            content_html,
            profile: None,
            status_left: app_name,
            status_right: "",
        }
    }

    pub const fn with_mode(mut self, mode: &'a str) -> Self {
        self.mode = mode;
        self
    }

    pub const fn light(self) -> Self {
        self.with_mode("light")
    }

    pub const fn dark(self) -> Self {
        self.with_mode("dark")
    }

    pub const fn dense(mut self) -> Self {
        self.density_class = "density-dense";
        self
    }

    pub const fn default_density(mut self) -> Self {
        self.density_class = "";
        self
    }

    pub const fn with_asset_base_path(mut self, asset_base_path: &'a str) -> Self {
        self.asset_base_path = asset_base_path;
        self
    }

    pub const fn with_nav(mut self, nav_html: &'a str) -> Self {
        self.nav_html = nav_html;
        self
    }

    pub const fn with_actions(mut self, actions_html: &'a str) -> Self {
        self.actions_html = actions_html;
        self
    }

    pub const fn with_profile(mut self, profile: SidebarProfile<'a>) -> Self {
        self.profile = Some(profile);
        self
    }

    pub const fn with_status(mut self, status_left: &'a str, status_right: &'a str) -> Self {
        self.status_left = status_left;
        self.status_right = status_right;
        self
    }

    pub fn stylesheet_link(&self) -> String {
        html::stylesheet_link(self.asset_base_path)
    }

    pub fn htmx_script_link(&self) -> String {
        html::htmx_script_link(self.asset_base_path)
    }

    pub fn script_link(&self) -> String {
        html::script_link(self.asset_base_path)
    }
}

impl<'a> askama::filters::HtmlSafe for AppShell<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::{HtmlAttr, MenuItem};

    #[test]
    fn app_shell_builders_render_variants() {
        let html = AppShell::new("Title", "Wave Funk", "<section>Content</section>")
            .light()
            .default_density()
            .with_nav(r#"<a class="wf-nav-item" href="/">Home</a>"#)
            .with_actions(r#"<button class="wf-btn">Save</button>"#)
            .with_status("Ready", "v0.1")
            .render()
            .unwrap();

        assert!(html.contains(r#"data-mode="light""#));
        assert!(html.contains(r#"class="wf-app ""#));
        assert!(html.contains(r#"<a class="wf-nav-item" href="/">Home</a>"#));
        assert!(html.contains(r#"<button class="wf-btn">Save</button>"#));
        assert!(html.contains(">Ready<"));
        assert!(html.contains(">v0.1<"));
    }

    #[test]
    fn app_shell_omits_sidebar_profile_until_supplied() {
        let html = AppShell::new("Title", "Wave Funk", "<section>Content</section>")
            .render()
            .unwrap();

        assert!(!html.contains("wf-sidebar-profile"));
        assert!(!html.contains("hx-post=\"/logout\""));
        assert!(!html.contains("Log out of this session?"));
    }

    #[test]
    fn app_shell_renders_default_sidebar_profile_menu_after_nav() {
        let html = AppShell::new("Title", "Wave Funk", "<section>Content</section>")
            .with_nav(r#"<a class="wf-nav-item" href="/">Home</a>"#)
            .with_profile(
                SidebarProfile::new()
                    .with_name("Sandeep Nambiar")
                    .with_email("sandeep@wavefunk.test"),
            )
            .render()
            .unwrap();

        let nav_position = html.find(r#"id="app-nav""#).unwrap();
        let profile_position = html.find("wf-sidebar-profile").unwrap();

        assert!(profile_position > nav_position);
        assert!(html.contains(">Sandeep Nambiar<"));
        assert!(html.contains(">sandeep@wavefunk.test<"));
        assert!(html.contains(r#"href="/settings""#));
        assert!(html.contains(">Settings<"));
        assert!(html.contains(r#"hx-post="/logout""#));
        assert!(html.contains(r#"hx-confirm="Log out of this session?""#));
        assert!(html.contains(">Logout<"));
    }

    #[test]
    fn sidebar_profile_label_falls_back_and_menu_items_can_be_overridden() {
        let logout_attrs = [
            HtmlAttr::hx_post("/sessions/current/delete"),
            HtmlAttr::hx_confirm("Really log out?"),
        ];
        let custom_menu = [
            MenuItem::link("Account", "/account"),
            MenuItem::button("Sign out")
                .danger()
                .with_attrs(&logout_attrs),
        ];

        let email_only = AppShell::new("Title", "Wave Funk", "<section>Content</section>")
            .with_profile(
                SidebarProfile::new()
                    .with_email("team@example.test")
                    .with_menu_items(&custom_menu),
            )
            .render()
            .unwrap();

        assert_eq!(email_only.matches("team@example.test").count(), 1);
        assert!(email_only.contains(">Account<"));
        assert!(email_only.contains(r#"href="/account""#));
        assert!(email_only.contains(">Sign out<"));
        assert!(email_only.contains(r#"hx-post="/sessions/current/delete""#));
        assert!(email_only.contains(r#"hx-confirm="Really log out?""#));
        assert!(!email_only.contains(r#"href="/settings""#));
        assert!(!email_only.contains(r#"hx-post="/logout""#));

        let anonymous = AppShell::new("Title", "Wave Funk", "<section>Content</section>")
            .with_profile(SidebarProfile::new())
            .render()
            .unwrap();

        assert!(anonymous.contains(">Profile<"));
    }
}
