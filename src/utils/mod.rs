
const USED_TECHNOLOGIES: [&'static str; 39] = [
    // Programming Languages
    "Rust",
    "Typescript/Javascript",
    "Python",
    "Ruby",
    "Haskell",
    "Elixir",
    "Prolog",
    "Java",
    "Kotlin",
    "C++",
    "C#",
    "VHDL",

    // DevOps Tools
    "Docker",
    "Kubernetes",
    "Terraform",
    "Helm",
    "Azure",
    "Azure DevOps",
    "Github Project Management",
    "GitOps",
    "CI/CD",
    "DevOps",
    "Prometheus",
    "Grafana",

    // Databases and Messaging
    "PostgreSQL",
    "Redis",
    "RabbitMQ",
    "Minio",

    // Web Technologies
    "React",
    "Redux",
    "Node.js",
    "OpenAPI",
    "GraphQL",
    "gRPC",

    // Operating Systems
    "Linux",
    "NixOS",
    "Arch Linux",

    // Miscellaneous
    "Git",
    "LLM Application",
];

pub fn used_tech_tags() -> String {
    USED_TECHNOLOGIES.into_iter().map(|tech| {
        markup::new! {
            span[class="text-sm italic"] {
                { tech }
            }
        }.to_string()
    }).collect::<Vec<_>>().join("")
}