use crate::Route;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

#[styled_component(NotFoundView)]
pub fn not_found_view() -> Html {
    let navigator = use_navigator().unwrap();

    let on_home_click = {
        let navigator = navigator.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            navigator.push(&Route::About);
        })
    };

    let style = Style::new(css!(
        r#"
        .not-found-view {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 80vh;
            text-align: center;
            padding: 40px 20px;
        }

        .error-code {
            font-size: 8rem;
            font-weight: 700;
            background: linear-gradient(45deg, #3b82f6, #8b5cf6);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            margin-bottom: 20px;
            line-height: 1;
        }

        .error-title {
            font-size: 2.5rem;
            color: white;
            margin-bottom: 16px;
            font-weight: 600;
        }

        .error-message {
            font-size: 1.25rem;
            color: #9ca3af;
            margin-bottom: 40px;
            max-width: 500px;
            line-height: 1.6;
        }

        .error-container {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            padding: 60px 40px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }

        .home-button {
            display: inline-flex;
            align-items: center;
            gap: 12px;
            background: linear-gradient(45deg, #3b82f6, #8b5cf6);
            color: white;
            border: none;
            border-radius: 8px;
            padding: 14px 28px;
            font-size: 1rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
            text-decoration: none;
            letter-spacing: 0.025em;
        }

        .home-button:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(59, 130, 246, 0.4);
        }

        .home-button:active {
            transform: translateY(0);
        }

        .home-button i {
            font-size: 1.125rem;
        }

        .glitch {
            position: relative;
            animation: glitch 2s ease-in-out infinite;
        }

        @keyframes glitch {
            0%, 100% {
                transform: translate(0);
            }
            20% {
                transform: translate(-1px, 1px);
            }
            40% {
                transform: translate(-1px, -1px);
            }
            60% {
                transform: translate(1px, 1px);
            }
            80% {
                transform: translate(1px, -1px);
            }
        }

        @media (max-width: 768px) {
            .error-code {
                font-size: 6rem;
            }

            .error-title {
                font-size: 2rem;
            }

            .error-message {
                font-size: 1.125rem;
            }

            .error-container {
                padding: 40px 24px;
            }
        }
        "#
    ))
    .expect("Failed to create style");

    html! {
        <div class={style}>
            <div class="not-found-view">
                <div class="error-container">
                    <div class="error-code glitch">{"404"}</div>
                    <h1 class="error-title">{"Page Not Found"}</h1>
                    <p class="error-message">
                        {"The page you're looking for doesn't exist or has been moved. Let's get you back on track."}
                    </p>
                    <button class="home-button" onclick={on_home_click}>
                        <i class="fas fa-home"></i>
                        {"Back to Home"}
                    </button>
                </div>
            </div>
        </div>
    }
}
