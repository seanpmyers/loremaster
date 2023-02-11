pub mod side_nav_bar;
pub mod top_nav_bar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationLink {
    pub html_id: String,
    pub html_href: String,
    pub display_text: String,
}

pub fn get_home_link() -> NavigationLink {
    NavigationLink {
        html_id: String::from("index-link"),
        html_href: String::from("/"),
        display_text: String::from("Loremaster"),
    }
}

pub fn get_navigation_links() -> Vec<NavigationLink> {
    return vec![
        NavigationLink {
            html_id: String::from("you-link"),
            html_href: String::from("/you/"),
            display_text: String::from("You"),
        },
        NavigationLink {
            html_id: String::from("chronicle-link"),
            html_href: String::from("/chronicle/"),
            display_text: String::from("Chronicle"),
        },
        NavigationLink {
            html_id: String::from("lore-link"),
            html_href: String::from("/lore/"),
            display_text: String::from("Lore"),
        },
        NavigationLink {
            html_id: String::from("timeline-link"),
            html_href: String::from("/timeline/"),
            display_text: String::from("Timeline"),
        },
        NavigationLink {
            html_id: String::from("ownership-link"),
            html_href: String::from("/ownership/"),
            display_text: String::from("Ownership"),
        },
        NavigationLink {
            html_id: String::from("registration-link"),
            html_href: String::from("/registration/"),
            display_text: String::from("Registration"),
        },
        NavigationLink {
            html_id: String::from("login-link"),
            html_href: String::from("/login/"),
            display_text: String::from("Login"),
        },
        NavigationLink {
            html_id: String::from("about-link"),
            html_href: String::from("/about/"),
            display_text: String::from("About"),
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
