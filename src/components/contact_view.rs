use crate::data::{ContactForm, Profile};
use stylist::{yew::styled_component, Style};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use std::time::Duration;
use crate::BASE_URL;

// Set to false by default for the time being, however this might be set through some compile time flag in the future
// to enable the contact form, or maybe by runtime if the api is down.
const ENABLE_CONTACT_FORM: bool = false;

#[derive(Debug, Clone, PartialEq)]
enum SubmissionState {
    Idle,
    Submitting,
    Success,
    Error(String),
}

#[styled_component(ContactView)]
pub fn contact_view() -> Html {
    let form_data = use_state(ContactForm::default);
    let submission_state = use_state(|| SubmissionState::Idle);
    
    let profile_str = include_str!("../datafile/profile.json");
    let profile: Profile = serde_json::from_str(profile_str).expect("valid json");

    let on_name_change = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_form = (*form_data).clone();
            new_form.name = input.value();
            form_data.set(new_form);
        })
    };

    let on_email_change = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_form = (*form_data).clone();
            new_form.email = input.value();
            form_data.set(new_form);
        })
    };

    let on_phone_change = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_form = (*form_data).clone();
            new_form.phone = input.value();
            form_data.set(new_form);
        })
    };

    let on_message_change = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut new_form = (*form_data).clone();
            new_form.message = input.value();
            form_data.set(new_form);
        })
    };

    let on_submit = {
        let form_data = form_data.clone();
        let submission_state = submission_state.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Don't submit if already submitting
            if matches!(*submission_state, SubmissionState::Submitting) {
                return;
            }
            
            let form_data = form_data.clone();
            let submission_state = submission_state.clone();
            
            submission_state.set(SubmissionState::Submitting);
            
            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest::Client::new();
                let request = client
                    .post(format!("{BASE_URL}/v1/contact"))
                    .json(&*form_data)
                    .timeout(Duration::from_secs(10));
                
                match request.send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            submission_state.set(SubmissionState::Success);
                            form_data.set(ContactForm::default());
                            
                            // Reset to idle after 3 seconds
                            let submission_state = submission_state.clone();
                            gloo::timers::callback::Timeout::new(3000, move || {
                                submission_state.set(SubmissionState::Idle);
                            }).forget();
                        } else {
                            let status = response.status();
                            let error_msg = match response.json::<serde_json::Value>().await {
                                Ok(json) => {
                                    let str = serde_json::to_string(&json).unwrap();
                                    format!("Server error: {status} | {str}", )
                                },
                                Err(_) => {
                                    format!("Server error: {status}")
                                }
                            };
                            submission_state.set(SubmissionState::Error(error_msg));
                        }
                    }
                    Err(err) => {
                        let error_msg = if err.is_timeout() {
                            "Request timed out. Please try again.".to_string()
                        } else if err.is_request() {
                            "Unable to send request. Please check your connection or try again later.".to_string()
                        } else {
                            "Network error. Please try again later.".to_string()
                        };
                        submission_state.set(SubmissionState::Error(error_msg));
                    }
                }
            });
        })
    };

    let style = Style::new(css!(
        r#"
        .contact-view {
            max-width: 700px;
            margin: 0 auto;
            padding: 40px 20px;
        }

        .contact-header {
            text-align: center;
            margin-bottom: 40px;
        }

        .contact-header h1 {
            font-size: 2.5rem;
            margin-bottom: 10px;
            background: linear-gradient(45deg, #3b82f6, #8b5cf6);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }

        .contact-header p {
            font-size: 1.125rem;
            color: #9ca3af;
            font-weight: 300;
        }

        .contact-form-container {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            padding: 40px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }

        .form-grid {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 24px;
            margin-bottom: 0;
        }

        .form-group {
            display: flex;
            flex-direction: column;
            margin-bottom: 24px;
        }

        .form-group:last-of-type {
            margin-bottom: 0;
        }

        .form-label {
            font-size: 0.875rem;
            font-weight: 500;
            color: #e5e7eb;
            margin-bottom: 8px;
            letter-spacing: 0.025em;
        }

        .form-input,
        .form-textarea {
            background: rgba(255, 255, 255, 0.08);
            border: 1px solid rgba(255, 255, 255, 0.15);
            border-radius: 8px;
            padding: 12px 16px;
            color: white;
            font-size: 1rem;
            transition: all 0.2s;
            font-family: inherit;
        }

        .form-input:focus,
        .form-textarea:focus {
            outline: none;
            background: rgba(255, 255, 255, 0.1);
            border-color: #60a5fa;
            box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.1);
        }

        .form-input::placeholder,
        .form-textarea::placeholder {
            color: #6b7280;
        }

        .form-textarea {
            min-height: 150px;
            resize: vertical;
        }

        .submit-btn {
            width: 100%;
            background: linear-gradient(45deg, #3b82f6, #8b5cf6);
            color: white;
            border: none;
            border-radius: 8px;
            padding: 14px 24px;
            font-size: 1rem;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
            margin-top: 32px;
            letter-spacing: 0.025em;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 12px;
            min-height: 52px;
        }

        .submit-btn:hover:not(:disabled) {
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(59, 130, 246, 0.4);
        }

        .submit-btn:active {
            transform: translateY(0);
        }

        .submit-btn:disabled {
            opacity: 0.7;
            cursor: not-allowed;
            transform: none;
        }

        .submit-btn.success {
            background: linear-gradient(45deg, #10b981, #34d399);
        }

        .submit-btn.error {
            background: linear-gradient(45deg, #ef4444, #f87171);
        }

        .spinner {
            width: 20px;
            height: 20px;
            border: 2px solid transparent;
            border-top: 2px solid currentColor;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }

        @keyframes spin {
            to {
                transform: rotate(360deg);
            }
        }

        .success-message {
            background: rgba(16, 185, 129, 0.1);
            border: 1px solid rgba(16, 185, 129, 0.3);
            color: #34d399;
            padding: 12px 16px;
            border-radius: 8px;
            margin-top: 16px;
            text-align: center;
            font-size: 0.875rem;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
        }

        .error-message {
            background: rgba(239, 68, 68, 0.1);
            border: 1px solid rgba(239, 68, 68, 0.3);
            color: #f87171;
            padding: 12px 16px;
            border-radius: 8px;
            margin-top: 16px;
            text-align: center;
            font-size: 0.875rem;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
        }

        .contact-info {
            margin-top: 40px;
            text-align: center;
            padding-top: 40px;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
        }

        .contact-info h3 {
            color: #a78bfa;
            font-size: 1.125rem;
            margin-bottom: 20px;
            font-weight: 600;
        }

        .contact-methods {
            display: flex;
            justify-content: center;
            gap: 40px;
            flex-wrap: wrap;
        }

        .contact-method {
            display: flex;
            align-items: center;
            gap: 12px;
            color: #d1d5db;
            text-decoration: none;
            transition: color 0.2s;
        }

        .contact-method:hover {
            color: #60a5fa;
        }

        .contact-method i {
            font-size: 1.25rem;
            color: #60a5fa;
        }

        .contact-info-container {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            padding: 40px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.1);
            text-align: center;
        }

        .contact-info-intro {
            font-size: 1.125rem;
            line-height: 1.8;
            color: #e5e7eb;
            margin-bottom: 40px;
            font-weight: 300;
        }

        .contact-methods-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 30px;
            margin-top: 40px;
        }

        .contact-method-card {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 16px;
            padding: 30px;
            background: rgba(255, 255, 255, 0.03);
            border-radius: 12px;
            border: 1px solid rgba(255, 255, 255, 0.08);
            transition: all 0.2s;
        }

        .contact-method-card:hover {
            background: rgba(255, 255, 255, 0.06);
            border-color: rgba(96, 165, 250, 0.3);
            transform: translateY(-2px);
        }

        .contact-method-icon {
            width: 60px;
            height: 60px;
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.75rem;
            color: white;
        }

        .contact-method-icon.email {
            background: linear-gradient(135deg, #8b5cf6, #a78bfa);
        }

        .contact-method-icon.linkedin {
            background: linear-gradient(135deg, #0ea5e9, #38bdf8);
        }

        .contact-method-icon.github {
            background: linear-gradient(135deg, #6b7280, #9ca3af);
        }

        .contact-method-label {
            font-size: 1.125rem;
            font-weight: 600;
            color: white;
            margin-bottom: 4px;
        }

        .contact-method-value {
            font-size: 1rem;
            color: #9ca3af;
        }

        .contact-method-link {
            text-decoration: none;
            color: inherit;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 16px;
            width: 100%;
        }

        @media (max-width: 768px) {
            .contact-form-container,
            .contact-info-container {
                padding: 30px 20px;
            }

            .form-grid {
                grid-template-columns: 1fr;
            }

            .contact-header h1 {
                font-size: 2rem;
            }

            .contact-methods {
                flex-direction: column;
                gap: 20px;
                align-items: center;
            }

            .contact-methods-grid {
                grid-template-columns: 1fr;
            }
        }
        "#
    ))
    .expect("Failed to create style");

    html! {
        <div class={style}>
            <div class="contact-view">
                <div class="contact-header">
                    <h1>{"Get In Touch"}</h1>
                    <p>{
                        if ENABLE_CONTACT_FORM {
                            "Let's discuss your next project or opportunity"
                        } else {
                            "I'd love to hear from you! Reach out through any of the channels below."
                        }
                    }</p>
                </div>

                {if ENABLE_CONTACT_FORM {
                    html! {
                        <div class="contact-form-container">
                            <form onsubmit={on_submit}>
                                <div class="form-grid">
                                    <div class="form-group">
                                        <label class="form-label" for="name">{"Name *"}</label>
                                        <input
                                            id="name"
                                            type="text"
                                            class="form-input"
                                            placeholder="John Doe"
                                            value={form_data.name.clone()}
                                            onchange={on_name_change}
                                            required=true
                                        />
                                    </div>

                                    <div class="form-group">
                                        <label class="form-label" for="email">{"Email *"}</label>
                                        <input
                                            id="email"
                                            type="email"
                                            class="form-input"
                                            placeholder="john.doe@example.com"
                                            value={form_data.email.clone()}
                                            onchange={on_email_change}
                                            required=true
                                        />
                                    </div>
                                </div>

                                <div class="form-group">
                                    <label class="form-label" for="phone">{"Phone Number"}</label>
                                    <input
                                        id="phone"
                                        type="tel"
                                        class="form-input"
                                        placeholder="+1 (555) 123-4567"
                                        value={form_data.phone.clone()}
                                        onchange={on_phone_change}
                                    />
                                </div>

                                <div class="form-group">
                                    <label class="form-label" for="message">{"Message *"}</label>
                                    <textarea
                                        id="message"
                                        class="form-textarea"
                                        placeholder="Tell me about your project or inquiry..."
                                        value={form_data.message.clone()}
                                        onchange={on_message_change}
                                        required=true
                                    />
                                </div>

                                <button 
                                    type="submit" 
                                    class={match *submission_state {
                                        SubmissionState::Success => "submit-btn success",
                                        SubmissionState::Error(_) => "submit-btn error",
                                        _ => "submit-btn"
                                    }}
                                    disabled={matches!(*submission_state, SubmissionState::Submitting)}
                                >
                                    {match &*submission_state {
                                        SubmissionState::Idle => html! {
                                            <>
                                                <i class="fas fa-paper-plane"></i>
                                                {"Send Message"}
                                            </>
                                        },
                                        SubmissionState::Submitting => html! {
                                            <>
                                                <div class="spinner"></div>
                                                {"Sending..."}
                                            </>
                                        },
                                        SubmissionState::Success => html! {
                                            <>
                                                <i class="fas fa-check"></i>
                                                {"Message Sent!"}
                                            </>
                                        },
                                        SubmissionState::Error(_) => html! {
                                            <>
                                                <i class="fas fa-exclamation-triangle"></i>
                                                {"Try Again"}
                                            </>
                                        }
                                    }}
                                </button>

                                {match &*submission_state {
                                    SubmissionState::Success => html! {
                                        <div class="success-message">
                                            <i class="fas fa-check-circle"></i>
                                            {"Thank you for your message! I'll get back to you soon."}
                                        </div>
                                    },
                                    SubmissionState::Error(msg) => html! {
                                        <div class="error-message">
                                            <i class="fas fa-exclamation-circle"></i>
                                            {msg}
                                        </div>
                                    },
                                    _ => html! {}
                                }}
                            </form>
                        </div>
                    }
                } else {
                    html! {
                        <div class="contact-info-container">
                            <div class="contact-info-intro">
                                <p>{"Whether you have a project in mind, want to collaborate, or just want to say hello, I'm always open to connecting with fellow developers and interesting people."}</p>
                            </div>

                            <div class="contact-methods-grid">
                                {if let Some(email) = &profile.email {
                                    html! {
                                        <div class="contact-method-card">
                                            <a href={format!("mailto:{}", email)} class="contact-method-link">
                                                <div class="contact-method-icon email">
                                                    <i class="fas fa-envelope"></i>
                                                </div>
                                                <div class="contact-method-label">{"Email"}</div>
                                                <div class="contact-method-value">{email}</div>
                                            </a>
                                        </div>
                                    }
                                } else { html! {} }}

                                {if let Some(linkedin) = &profile.linkedin {
                                    html! {
                                        <div class="contact-method-card">
                                            <a href={linkedin.clone()} target="_blank" rel="noopener noreferrer" class="contact-method-link">
                                                <div class="contact-method-icon linkedin">
                                                    <i class="fab fa-linkedin"></i>
                                                </div>
                                                <div class="contact-method-label">{"LinkedIn"}</div>
                                                <div class="contact-method-value">{"Connect with me"}</div>
                                            </a>
                                        </div>
                                    }
                                } else { html! {} }}

                                {if let Some(github) = &profile.github {
                                    html! {
                                        <div class="contact-method-card">
                                            <a href={github.clone()} target="_blank" rel="noopener noreferrer" class="contact-method-link">
                                                <div class="contact-method-icon github">
                                                    <i class="fab fa-github"></i>
                                                </div>
                                                <div class="contact-method-label">{"GitHub"}</div>
                                                <div class="contact-method-value">{"Check out my code"}</div>
                                            </a>
                                        </div>
                                    }
                                } else { html! {} }}
                            </div>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
