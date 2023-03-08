use super::icon::{
    ARCHIVE_SVG_HTML, BOOK_SVG_HTML, CLOCK_SVG_HTML, FEATHER_SVG_HTML, HELP_CIRCLE_SVG_HTML,
    LOGIN_SVG_HTML, USER_PLUS_SVG_HTML, USER_SVG_HTML,
};

pub mod side_nav_bar;
pub mod top_nav_bar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationLink {
    pub html_id: String,
    pub html_href: String,
    pub display_text: String,
    pub svg_html: &'static str,
}

pub fn get_home_link() -> NavigationLink {
    NavigationLink {
        html_id: String::from("index-link"),
        html_href: String::from("/"),
        display_text: String::from("Loremaster"),
        svg_html: "",
    }
}

pub fn get_navigation_links() -> Vec<NavigationLink> {
    return vec![
        NavigationLink {
            html_id: String::from("you-link"),
            html_href: String::from("/you/"),
            display_text: String::from("You"),
            svg_html: USER_SVG_HTML,
        },
        NavigationLink {
            html_id: String::from("chronicle-link"),
            html_href: String::from("/chronicle/"),
            display_text: String::from("Chronicle"),
            svg_html: FEATHER_SVG_HTML,
        },
        NavigationLink {
            html_id: String::from("lore-link"),
            html_href: String::from("/lore/"),
            display_text: String::from("Lore"),
            svg_html: BOOK_SVG_HTML,
        },
        NavigationLink {
            html_id: String::from("timeline-link"),
            html_href: String::from("/timeline/"),
            display_text: String::from("Timeline"),
            svg_html: CLOCK_SVG_HTML,
        },
        NavigationLink {
            html_id: String::from("ownership-link"),
            html_href: String::from("/ownership/"),
            display_text: String::from("Ownership"),
            svg_html: ARCHIVE_SVG_HTML,
        },
        NavigationLink {
            html_id: String::from("registration-link"),
            html_href: String::from("/registration/"),
            display_text: String::from("Registration"),
            svg_html: USER_PLUS_SVG_HTML,
        },
        NavigationLink {
            html_id: String::from("login-link"),
            html_href: String::from("/login/"),
            display_text: String::from("Login"),
            svg_html: LOGIN_SVG_HTML,
        },
        NavigationLink {
            html_id: String::from("about-link"),
            html_href: String::from("/about/"),
            display_text: String::from("About"),
            svg_html: HELP_CIRCLE_SVG_HTML,
        },
    ];
}

pub fn get_navigation_link_by_name(name: String) -> Option<NavigationLink> {
    match get_navigation_links()
        .iter()
        .find(|link| link.display_text == name)
    {
        Some(link) => Some(link.to_owned()),
        None => None,
    }
}
