use crate::data::get_about;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[styled_component(AboutView)]
pub fn about_view() -> Html {
    let about = get_about();

    let style = Style::new(css!(
        r#"
        .about-view {
            max-width: 900px;
            margin: 0 auto;
            padding: 40px 20px;
        }

        .about-header {
            text-align: center;
            margin-bottom: 40px;
        }

        .about-header h1 {
            font-size: 2.5rem;
            margin-bottom: 10px;
            background: linear-gradient(45deg, #3b82f6, #8b5cf6);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }

        .about-container {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            padding: 40px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }

        .about-intro {
            font-size: 1.125rem;
            line-height: 1.8;
            color: #e5e7eb;
            margin-bottom: 40px;
            text-align: center;
            font-weight: 300;
        }

        .about-sections {
            display: grid;
            gap: 30px;
            margin-bottom: 40px;
        }

        .about-section h3 {
            color: #60a5fa;
            font-size: 1.25rem;
            margin-bottom: 12px;
            font-weight: 600;
        }

        .about-section p {
            color: #d1d5db;
            line-height: 1.7;
            font-size: 1rem;
        }

        .about-bottom {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 40px;
            margin-top: 40px;
            padding-top: 40px;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
        }

        .about-interests h3,
        .about-values h3 {
            color: #a78bfa;
            font-size: 1.25rem;
            margin-bottom: 20px;
            font-weight: 600;
        }

        .interests-grid {
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
        }

        .interest-tag {
            background: rgba(59, 130, 246, 0.1);
            border: 1px solid rgba(59, 130, 246, 0.3);
            color: #93c5fd;
            padding: 8px 16px;
            border-radius: 20px;
            font-size: 0.875rem;
            transition: all 0.2s;
        }

        .interest-tag:hover {
            background: rgba(59, 130, 246, 0.2);
            border-color: rgba(59, 130, 246, 0.5);
            transform: translateY(-2px);
        }

        .values-list {
            list-style: none;
            display: grid;
            gap: 12px;
        }

        .values-list li {
            color: #d1d5db;
            padding-left: 24px;
            position: relative;
            line-height: 1.6;
            font-size: 0.95rem;
        }

        .values-list li::before {
            content: "▸";
            position: absolute;
            left: 0;
            color: #a78bfa;
            font-weight: bold;
        }

        @media (max-width: 768px) {
            .about-container {
                padding: 30px 20px;
            }

            .about-header h1 {
                font-size: 2rem;
            }

            .about-bottom {
                grid-template-columns: 1fr;
                gap: 30px;
            }
        }
        "#
    ))
    .expect("Failed to create style");

    html! {
        <div class={style}>
            <div class="about-view">
                <div class="about-header">
                    <h1>{"About Me"}</h1>
                </div>

                <div class="about-container">
                    <div class="about-intro">
                        <p>{&about.intro}</p>
                    </div>

                    <div class="about-sections">
                        {for about.sections.iter().map(|section| {
                            html! {
                                <div class="about-section">
                                    <h3>{&section.title}</h3>
                                    <p>{&section.content}</p>
                                </div>
                            }
                        })}
                    </div>

                    <div class="about-bottom">
                        if !about.interests.is_empty() {
                            <div class="about-interests">
                                <h3>{"Areas of Interest"}</h3>
                                <div class="interests-grid">
                                    {for about.interests.iter().map(|interest| {
                                        html! {
                                            <span class="interest-tag">{interest}</span>
                                        }
                                    })}
                                </div>
                            </div>
                        }

                        if !about.values.is_empty() {
                            <div class="about-values">
                                <h3>{"Core Values"}</h3>
                                <ul class="values-list">
                                    {for about.values.iter().map(|value| {
                                        html! {
                                            <li>{value}</li>
                                        }
                                    })}
                                </ul>
                            </div>
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
