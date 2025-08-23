use crate::data::{get_projects, Project};
use stylist::{yew::styled_component, Style};
use yew::prelude::*;

#[styled_component(ProjectsView)]
pub fn projects_view() -> Html {
    let projects = get_projects();

    let style = Style::new(css!(
        r#"
        .projects-view {
            max-width: 1200px;
            margin: 0 auto;
            padding: 40px 20px;
        }

        .projects-header {
            text-align: center;
            margin-bottom: 40px;
        }

        .projects-header h1 {
            font-size: 2.5rem;
            margin-bottom: 10px;
            background: linear-gradient(45deg, #3b82f6, #8b5cf6);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }

        .projects-header p {
            font-size: 1.125rem;
            color: #9ca3af;
            font-weight: 300;
        }

        .projects-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 30px;
        }

        .project-card {
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            border-radius: 16px;
            overflow: hidden;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
            border: 1px solid rgba(255, 255, 255, 0.1);
            transition: all 0.3s ease;
            display: flex;
            flex-direction: column;
        }

        .project-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
            border-color: rgba(96, 165, 250, 0.3);
        }

        .project-image {
            height: 200px;
            background: linear-gradient(135deg, #3b82f6, #8b5cf6);
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-size: 3rem;
        }

        .project-content {
            padding: 30px;
            display: flex;
            flex-direction: column;
            gap: 20px;
            flex: 1;
        }

        .project-title {
            font-size: 1.5rem;
            font-weight: 600;
            color: white;
            margin: 0;
        }

        .project-description {
            color: #d1d5db;
            line-height: 1.7;
            font-size: 1rem;
            margin: 0;
            flex: 1;
        }

        .project-date {
            color: #9ca3af;
            font-size: 0.875rem;
            font-weight: 400;
            margin-bottom: 10px;
            display: flex;
            align-items: center;
            gap: 6px;
        }

        .project-date i {
            font-size: 0.75rem;
        }

        .tech-tags {
            display: flex;
            flex-wrap: wrap;
            gap: 10px;
            margin: 10px 0;
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
        }

        .project-links {
            display: flex;
            gap: 16px;
            margin-top: auto;
            padding-top: 20px;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
        }

        .project-link {
            display: flex;
            align-items: center;
            gap: 8px;
            color: #60a5fa;
            text-decoration: none;
            font-weight: 500;
            transition: all 0.2s;
            padding: 8px 16px;
            border: 1px solid rgba(96, 165, 250, 0.3);
            border-radius: 8px;
            background: rgba(96, 165, 250, 0.05);
        }

        .project-link:hover {
            color: white;
            background: rgba(96, 165, 250, 0.15);
            border-color: rgba(96, 165, 250, 0.5);
            transform: translateY(-2px);
        }

        .project-link i {
            font-size: 1rem;
        }

        @media (max-width: 768px) {
            .projects-grid {
                grid-template-columns: 1fr;
                gap: 24px;
            }

            .projects-header h1 {
                font-size: 2rem;
            }

            .project-content {
                padding: 24px;
            }
        }
        "#
    ))
    .expect("Failed to create style");

    html! {
        <div class={style}>
            <div class="projects-view">
                <div class="projects-header">
                    <h1>{"Projects"}</h1>
                    <p>{"A showcase of my work and contributions"}</p>
                </div>

                <div class="projects-grid">
                    { for projects.iter().map(|project| render_project(project)) }
                </div>
            </div>
        </div>
    }
}

fn render_project(project: &Project) -> Html {
    html! {
        <div class="project-card">
            <div class="project-image">
                if let Some(_image) = &project.image {
                    // TODO: Add section for image previews
                    <i class="fas fa-code"></i>
                } else {
                    <i class="fas fa-code"></i>
                }
            </div>

            <div class="project-content">
                <h3 class="project-title">{&project.name}</h3>
                if let Some(date) = &project.date {
                    <div class="project-date">
                        <i class="far fa-calendar"></i>
                        {date}
                    </div>
                }
                <p class="project-description">{&project.description}</p>

                <div class="tech-tags">
                    { for project.technologies.iter().map(|tech| {
                        html! { <span class="tech-tag">{tech}</span> }
                    }) }
                </div>

                <div class="project-links">
                    if let Some(website) = &project.website {
                        <a
                            href={website.clone()}
                            target="_blank"
                            rel="noopener noreferrer"
                            class="project-link"
                        >
                            <i class="fas fa-external-link-alt"></i>
                            {"Website"}
                        </a>
                    }

                    if let Some(github) = &project.github {
                        <a
                            href={github.clone()}
                            target="_blank"
                            rel="noopener noreferrer"
                            class="project-link"
                        >
                            <i class="fab fa-github"></i>
                            {"Code"}
                        </a>
                    }
                </div>
            </div>
        </div>
    }
}
