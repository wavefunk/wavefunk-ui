use crate::{assets, html};
use askama::Template;

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
}
