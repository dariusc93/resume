use crate::data::{get_resume, Education, Experience, SkillCategory};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[styled_component(ResumeView)]
pub fn resume_view() -> Html {
    let resume = get_resume();

    let style = Style::new(css!(
        r#"
        .resume-view {
            max-width: 1000px;
            margin: 0 auto;
            padding: 40px 20px;
        }

        .resume-header {
            text-align: center;
            margin-bottom: 40px;
        }

        .resume-header h1 {
            font-size: 2.5rem;
            margin-bottom: 10px;
            background: linear-gradient(45deg, #3b82f6, #8b5cf6);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }

        .resume-header p {
            font-size: 1.125rem;
            color: #9ca3af;
            font-weight: 300;
        }

        .resume-sections {
            display: grid;
            gap: 30px;
        }

        .resume-section {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            padding: 32px;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.1);
        }

        .section-header {
            display: flex;
            align-items: center;
            gap: 16px;
            margin-bottom: 28px;
            padding-bottom: 16px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
        }

        .section-icon {
            width: 48px;
            height: 48px;
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.5rem;
            color: white;
        }

        .section-icon.blue {
            background: linear-gradient(135deg, #3b82f6, #60a5fa);
        }

        .section-icon.green {
            background: linear-gradient(135deg, #10b981, #34d399);
        }

        .section-icon.purple {
            background: linear-gradient(135deg, #8b5cf6, #a78bfa);
        }

        .section-title {
            font-size: 1.75rem;
            font-weight: 600;
            color: white;
            margin: 0;
        }

        .experience-item,
        .education-item {
            margin-bottom: 28px;
            padding-bottom: 28px;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }

        .experience-item:last-child,
        .education-item:last-child {
            margin-bottom: 0;
            padding-bottom: 0;
            border-bottom: none;
        }

        .item-header {
            display: flex;
            justify-content: space-between;
            align-items: flex-start;
            margin-bottom: 8px;
            flex-wrap: wrap;
            gap: 12px;
        }

        .item-title {
            font-size: 1.25rem;
            font-weight: 600;
            color: white;
            margin: 0;
        }

        .item-duration {
            font-size: 0.875rem;
            color: #9ca3af;
            background: rgba(255, 255, 255, 0.05);
            padding: 4px 12px;
            border-radius: 20px;
            border: 1px solid rgba(255, 255, 255, 0.1);
        }

        .item-company {
            color: #60a5fa;
            font-weight: 500;
            margin-bottom: 12px;
            font-size: 1.125rem;
        }

        .item-description {
            color: #d1d5db;
            line-height: 1.7;
            margin-bottom: 16px;
        }

        .tech-tags {
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
        }

        .tech-tag {
            background: rgba(59, 130, 246, 0.1);
            border: 1px solid rgba(59, 130, 246, 0.3);
            color: #93c5fd;
            padding: 6px 14px;
            border-radius: 20px;
            font-size: 0.875rem;
            transition: all 0.2s;
        }

        .tech-tag:hover {
            background: rgba(59, 130, 246, 0.2);
            border-color: rgba(59, 130, 246, 0.5);
            transform: translateY(-2px);
        }

        .skills-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 24px;
        }

        .skill-category h3 {
            font-size: 1.125rem;
            font-weight: 600;
            color: #a78bfa;
            margin-bottom: 16px;
        }

        @media (max-width: 768px) {
            .resume-header h1 {
                font-size: 2rem;
            }

            .resume-section {
                padding: 24px;
            }

            .item-header {
                flex-direction: column;
                align-items: flex-start;
            }

            .skills-grid {
                grid-template-columns: 1fr;
            }
        }
        "#
    ))
    .expect("Failed to create style");

    html! {
        <div class={style}>
            <div class="resume-view">
                <div class="resume-header">
                    <h1>{"Resume"}</h1>
                    <p>{"Professional experience and qualifications"}</p>
                </div>

                <div class="resume-sections">
                    // Experience Section
                    <div class="resume-section">
                        <div class="section-header">
                            <div class="section-icon blue">
                                <i class="fas fa-briefcase"></i>
                            </div>
                            <h2 class="section-title">{"Experience"}</h2>
                        </div>

                        <div class="experiences">
                            { for resume.experience.iter().map(|exp| render_experience(exp)) }
                        </div>
                    </div>

                    // Education Section
                    <div class="resume-section">
                        <div class="section-header">
                            <div class="section-icon green">
                                <i class="fas fa-graduation-cap"></i>
                            </div>
                            <h2 class="section-title">{"Education"}</h2>
                        </div>

                        <div class="education-items">
                            { for resume.education.iter().map(|edu| render_education(edu)) }
                        </div>
                    </div>

                    // Skills Section
                    <div class="resume-section">
                        <div class="section-header">
                            <div class="section-icon purple">
                                <i class="fas fa-cog"></i>
                            </div>
                            <h2 class="section-title">{"Skills"}</h2>
                        </div>

                        <div class="skills-grid">
                            { for resume.skills.iter().map(|category| render_skill_category(category)) }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn render_experience(exp: &Experience) -> Html {
    html! {
        <div class="experience-item">
            <div class="item-header">
                <h3 class="item-title">{&exp.position}</h3>
                <span class="item-duration">{&exp.duration}</span>
            </div>
            <p class="item-company">{&exp.company}</p>
            <p class="item-description">{&exp.description}</p>
            <div class="tech-tags">
                { for exp.technologies.iter().map(|tech| {
                    html! { <span class="tech-tag">{tech}</span> }
                }) }
            </div>
        </div>
    }
}

fn render_education(edu: &Education) -> Html {
    html! {
        <div class="education-item">
            <div class="item-header">
                <h3 class="item-title">{&edu.degree}</h3>
                if let Some(year) = edu.year.as_ref() {
                    <span class="item-duration">{year}</span>
                }
            </div>
            <p class="item-company">{&edu.school}</p>
            <p class="item-description">{&edu.details}</p>
        </div>
    }
}

fn render_skill_category(category: &SkillCategory) -> Html {
    html! {
        <div class="skill-category">
            <h3>{&category.name}</h3>
            <div class="tech-tags">
                { for category.items.iter().map(|skill| {
                    html! { <span class="tech-tag">{skill}</span> }
                }) }
            </div>
        </div>
    }
}
