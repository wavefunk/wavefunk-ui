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
