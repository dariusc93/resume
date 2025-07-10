use crate::data::get_terms;
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[styled_component(TermsView)]
pub fn terms_view() -> Html {
    let terms = get_terms();

    let style = Style::new(css!(
        r#"
        .terms-view {
            max-width: 800px;
            margin: 0 auto;
            padding: 40px 20px;
        }

        .terms-container {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            padding: 40px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }

        .terms-header {
            text-align: center;
            margin-bottom: 40px;
            padding-bottom: 30px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
        }

        .terms-title {
            font-size: 2.5rem;
            font-weight: 700;
            margin-bottom: 10px;
            background: linear-gradient(45deg, #3b82f6, #8b5cf6);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }

        .effective-date {
            font-size: 1rem;
            color: #9ca3af;
            font-weight: 300;
        }

        .terms-section {
            margin-bottom: 32px;
        }

        .terms-section:last-child {
            margin-bottom: 0;
        }

        .section-title {
            font-size: 1.25rem;
            font-weight: 600;
            color: #e5e7eb;
            margin-bottom: 12px;
        }

        .section-content {
            font-size: 1rem;
            line-height: 1.8;
            color: #d1d5db;
            text-align: justify;
        }

        @media (max-width: 768px) {
            .terms-container {
                padding: 30px 20px;
            }

            .terms-title {
                font-size: 2rem;
            }

            .section-title {
                font-size: 1.125rem;
            }

            .section-content {
                font-size: 0.938rem;
            }
        }
        "#
    ))
    .expect("Failed to create style");

    html! {
        <div class={style}>
            <div class="terms-view">
                <div class="terms-container">
                    <div class="terms-header">
                        <h1 class="terms-title">{&terms.title}</h1>
                        <p class="effective-date">{format!("Effective Date: {}", &terms.effective_date)}</p>
                    </div>

                    <div class="terms-content">
                        {for terms.sections.iter().map(|section| {
                            html! {
                                <div class="terms-section">
                                    <h2 class="section-title">{&section.title}</h2>
                                    <p class="section-content">{&section.content}</p>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </div>
        </div>
    }
}