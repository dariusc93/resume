use crate::{data::Profile, Route};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub profile: Profile,
    pub current_route: Route,
    pub is_mobile: bool,
    pub is_open: bool,
    pub on_navigate: Callback<()>,
}

#[styled_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let _navigator = use_navigator().unwrap();

    // Get the current route directly from the router hook
    let current_route = use_route::<Route>().unwrap_or(Route::About);

    let style = Style::new(css!(
        r#"
        .sidebar-footer {
            padding: 24px;
            flex-shrink: 0;
            border-top: 1px solid rgba(255, 255, 255, 0.08);
            margin-top: auto;
        }

        .social-links {
            text-align: center;
        }

        .social-links p {
            color: #9ca3af;
            font-size: 0.875rem;
            margin-bottom: 16px;
            font-weight: 500;
            letter-spacing: 0.025em;
        }

        .social-icons {
            display: flex;
            justify-content: center;
            gap: 16px;
        }

        .social-icon {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 44px;
            height: 44px;
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 12px;
            color: #9ca3af;
            text-decoration: none;
            transition: all 0.2s ease;
            backdrop-filter: blur(10px);
        }

        .social-icon:hover {
            background: rgba(255, 255, 255, 0.1);
            border-color: rgba(96, 165, 250, 0.5);
            color: #60a5fa;
            transform: translateY(-2px);
            box-shadow: 0 4px 12px rgba(96, 165, 250, 0.15);
        }

        .social-icon i {
            font-size: 1.125rem;
        }

        .social-icon.linkedin:hover {
            border-color: rgba(14, 165, 233, 0.5);
            color: #0ea5e9;
            box-shadow: 0 4px 12px rgba(14, 165, 233, 0.15);
        }

        .social-icon.github:hover {
            border-color: rgba(107, 114, 128, 0.5);
            color: #e5e7eb;
            box-shadow: 0 4px 12px rgba(107, 114, 128, 0.15);
        }

        .social-icon.email:hover {
            border-color: rgba(139, 92, 246, 0.5);
            color: #a78bfa;
            box-shadow: 0 4px 12px rgba(139, 92, 246, 0.15);
        }
        "#
    ))
    .expect("Failed to create style");

    let sidebar_class = if props.is_mobile {
        if props.is_open {
            "sidebar sidebar-mobile open"
        } else {
            "sidebar sidebar-mobile"
        }
    } else {
        "sidebar sidebar-desktop"
    };

    let initials = props
        .profile
        .name
        .split_whitespace()
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_uppercase();

    html! {
        <div class={style}>
            <div class={sidebar_class}>
                <div class="sidebar-profile">
                    <div class="profile-info">
                        <div class="profile-avatar">
                            {initials}
                        </div>
                        <div class="profile-details">
                            <h2>{&props.profile.name}</h2>
                            <p>{&props.profile.title}</p>
                        </div>
                    </div>
                </div>

                <nav class="sidebar-nav">
                    <Link<Route>
                        to={Route::About}
                        classes={if current_route == Route::About { "nav-item active" } else { "nav-item" }}
                    >
                        <i class="fas fa-info-circle"></i>
                        {"About"}
                    </Link<Route>>

                    <Link<Route>
                        to={Route::Resume}
                        classes={if current_route == Route::Resume { "nav-item active" } else { "nav-item" }}
                    >
                        <i class="fas fa-user"></i>
                        {"Resume"}
                    </Link<Route>>

                    <Link<Route>
                        to={Route::Projects}
                        classes={if current_route == Route::Projects { "nav-item active" } else { "nav-item" }}
                    >
                        <i class="fas fa-folder"></i>
                        {"Projects"}
                    </Link<Route>>

                    <Link<Route>
                        to={Route::Contact}
                        classes={if current_route == Route::Contact { "nav-item active" } else { "nav-item" }}
                    >
                        <i class="fas fa-envelope"></i>
                        {"Contact"}
                    </Link<Route>>
                </nav>

                if props.profile.hide_get_in_touch() {
                    <div class="sidebar-footer">
                        <div class="social-links">
                            <p>{"Get in touch"}</p>
                            <div class="social-icons">
                                if let Some(linkedin) = props.profile.linkedin.as_ref() {
                                    <a href={linkedin.clone()} target="_blank" rel="noopener noreferrer" class="social-icon linkedin">
                                        <i class="fab fa-linkedin"></i>
                                    </a>
                                }
                                if let Some(github) = props.profile.github.as_ref() {
                                    <a href={github.clone()} target="_blank" rel="noopener noreferrer" class="social-icon github">
                                        <i class="fab fa-github"></i>
                                    </a>
                                }
                                if let Some(email) = props.profile.email.as_ref() {
                                    <a href={format!("mailto:{email}")} target="_blank" rel="noopener noreferrer" class="social-icon email">
                                        <i class="fas fa-envelope"></i>
                                    </a>
                                }
                            </div>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}
