use clap::Parser;
use colored::Colorize;

pub struct App {
    args: Args,
}

impl App {
    pub fn new() -> Self {
        Self {
            args: Args::parse(),
        }
    }

    pub fn run(&self) {
        self.setup_directory();
        self.setup_src();
        let repo = self.create_github_repo();
        self.commit_and_push(repo);
    }
    fn get_ignore_str<'a>(&self) -> &'a str {
        match (self.args.rust, self.args.csharp) {
            (_, true) => "bin\nobj",
            (_, _) => "target", // default is rust case
        }
    }
    fn setup_directory(&self) {
        println!("Creating project directory...");
        std::fs::create_dir(&self.args.name).unwrap();

        std::env::set_current_dir(&self.args.name).unwrap();
        if std::fs::File::create("README.md").is_err() {
            println!("{}: Unable to create README.md.", "Warning".yellow());
        }

        std::process::Command::new("git")
            .args(["init"])
            .output()
            .unwrap();

        std::fs::write(".gitignore", self.get_ignore_str()).unwrap();

        println!("Creating docs directory...");
        std::fs::create_dir("docs").unwrap();
    }

    fn setup_src(&self) {
        std::fs::create_dir("src").unwrap();
        std::env::set_current_dir("src").unwrap();
        if self.args.csharp {
            println!("Creating C# project...");
            std::process::Command::new("dotnet")
                .args(["new", "console"])
                .args(["-o", &self.args.name])
                .output()
                .unwrap();
        } else {
            println!("Creating Rust project...");
            std::process::Command::new("cargo")
                .args(["new", &self.args.name])
                .output()
                .unwrap();
        }
        std::env::set_current_dir("..").unwrap();
    }

    fn create_github_repo(&self) -> String {
        println!("Creating GitHub Repo...");
        let visibility = match self.args.public {
            true => "--public",
            false => "--private",
        };
        let res = std::process::Command::new("gh")
            .args(["repo"])
            .args(["create"])
            .args([visibility])
            .args([&self.args.name])
            .output()
            .unwrap()
            .stdout;

        String::from_utf8(res).unwrap().trim_end().to_string()
    }

    fn commit_and_push(&self, github: String) {
        println!("Pushing to repository...");
        std::process::Command::new("git")
            .args(["remote"])
            .args(["add", "origin", &format!("{}.git", github)])
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["add", "*"])
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["branch"])
            .args(["-m", "main"])
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit"])
            .args(["-m", "\"Project initialisation\""])
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["push"])
            .args(["-u", "origin", "main"])
            .output()
            .unwrap();
    }
}

#[derive(Parser, Debug)]
#[command(name = "Project Builder")]
#[command(author = "Samuel J.")]
#[command(about = "Creates starter project structure.")]
struct Args {
    #[clap(short, long, group = "language")]
    csharp: bool,
    #[clap(short, long, group = "language")]
    rust: bool,
    #[clap(short, long, required = true)]
    name: String,
    #[clap(short, long, default_value_t = false)]
    public: bool,
}
