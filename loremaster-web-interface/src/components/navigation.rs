use super::icon::{
    SvgIcon, ARCHIVE_SVG_HTML, BOOK_SVG_HTML, CLOCK_SVG_HTML, FEATHER_SVG_HTML,
    HELP_CIRCLE_SVG_HTML, LAYOUT_SVG_HTML, LOGIN_SVG_HTML, USER_PLUS_SVG_HTML, USER_SVG_HTML,
};

pub mod side_nav_bar;
pub mod tab;
pub mod top_nav_bar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationLink<'a> {
    pub html_id: &'a str,
    pub html_href: &'a str,
    pub display_text: &'a str,
    pub svg_html: SvgIcon,
}

pub fn get_home_link() -> NavigationLink<'static> {
    NavigationLink {
        html_id: "index-link",
        html_href: "/",
        display_text: "Loremaster",
        svg_html: "",
    }
}

pub fn get_navigation_links() -> Vec<NavigationLink<'static>> {
    vec![
        NavigationLink {
            html_id: "you-link",
            html_href: "/you/",
            display_text: "You",
            svg_html: USER_SVG_HTML,
        },
        NavigationLink {
            html_id: "chronicle-link",
            html_href: "/chronicle/",
            display_text: "Chronicle",
            svg_html: FEATHER_SVG_HTML,
        },
        NavigationLink {
            html_id: "lore-link",
            html_href: "/lore/",
            display_text: "Lore",
            svg_html: BOOK_SVG_HTML,
        },
        NavigationLink {
            html_id: "timeline-link",
            html_href: "/timeline/",
            display_text: "Timeline",
            svg_html: CLOCK_SVG_HTML,
        },
        NavigationLink {
            html_id: "ownership-link",
            html_href: "/ownership/",
            display_text: "Ownership",
            svg_html: ARCHIVE_SVG_HTML,
        },
        NavigationLink {
            html_id: "registration-link",
            html_href: "/registration/",
            display_text: "Registration",
            svg_html: USER_PLUS_SVG_HTML,
        },
        NavigationLink {
            html_id: "login-link",
            html_href: "/login/",
            display_text: "Login",
            svg_html: LOGIN_SVG_HTML,
        },
        NavigationLink {
            html_id: "about-link",
            html_href: "/about/",
            display_text: "About",
            svg_html: HELP_CIRCLE_SVG_HTML,
        },
    ]
}

pub fn get_navigation_link_by_name(name: String) -> Option<NavigationLink<'static>> {
    get_navigation_links()
        .iter()
        .find(|link| link.display_text == name)
        .map(|link| link.to_owned())
}

pub const DESIGN_SYSTEM_LINK: NavigationLink = NavigationLink {
    html_id: "design-system-link",
    html_href: "/design-system/",
    display_text: "Design System",
    svg_html: LAYOUT_SVG_HTML,
};
