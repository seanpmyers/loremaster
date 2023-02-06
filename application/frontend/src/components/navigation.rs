use sycamore::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationLink {
    pub html_id: String,
    pub html_href: String,
    pub display_text: String,
}

#[component(NavigationLinks<G>)]
pub fn navigation_links() -> View<G> {
    let app_name_class: &str = "loremaster-text";
    let a_class: &str = "loremaster-text";
    let li_class: &str = "hover-border-bottom";

    let nav_classes: &str = "top-nav";
    let nav_ul_classes: &str = "";
    let nav_li_classes: &str = "";

    let links: Signal<Vec<NavigationLink>> = Signal::new(get_navigation_links());

    view! {
        nav(class=nav_classes) {
            ul(class=nav_ul_classes) {
                Indexed(IndexedProps{
                    iterable:links.handle(),
                    template: move |link| view! {
                        li(class=nav_li_classes) {
                            a(class=a_class, id=link.html_id, href=link.html_href) { (link.display_text) }
                        }
                    }
                })
            }
        }
    }
}

pub fn get_navigation_links() -> Vec<NavigationLink> {
    return vec![
        NavigationLink {
            html_id: String::from("index-link"),
            html_href: String::from("/"),
            display_text: String::from("Loremaster"),
        },
        NavigationLink {
            html_id: String::from("about-link"),
            html_href: String::from("/about/"),
            display_text: String::from("About"),
        },
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
            html_id: String::from("timeline-link"),
            html_href: String::from("/timeline/"),
            display_text: String::from("Timeline"),
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
    ];
}
