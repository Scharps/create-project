use clap::Parser;
use colored::Colorize;

pub struct App<'a> {
    name: &'a str,
    repo_visibility: Visibility,
    software_builder: Box<dyn SoftwareProjectBuilder>,
}

enum Visibility {
    Public,
    Private,
}

trait SoftwareProjectBuilder {
    fn create(&self);
    fn ignore_str(&self) -> &'static str;
}

struct RustProjectBuilder<'a> {
    name: &'a str,
    lib: bool,
}

impl<'a> SoftwareProjectBuilder for RustProjectBuilder<'a> {
    fn create(&self) {
        let project_type = match self.lib {
            true => "--lib",
            false => "--bin",
        };
        std::process::Command::new("cargo")
            .args(["new", project_type, self.name])
            .output()
            .unwrap();
    }

    fn ignore_str(&self) -> &'static str {
        "target"
    }
}

struct CSharpProjectBuilder<'a> {
    name: &'a str,
    lib: bool,
}

impl<'a> SoftwareProjectBuilder for CSharpProjectBuilder<'a> {
    fn create(&self) {
        let project_type = match self.lib {
            true => "classlib",
            false => "console",
        };
        std::process::Command::new("dotnet")
            .args(["new", project_type])
            .args(["-o", self.name])
            .output()
            .unwrap();
    }

    fn ignore_str(&self) -> &'static str {
        "bin\nobj"
    }
}

impl<'a> App<'a> {
    pub fn run(&self) {
        self.setup_directory();
        self.setup_src();
        let repo = self.create_github_repo();
        self.commit_and_push(repo);
    }

    fn setup_directory(&self) {
        println!("Creating project directory...");
        std::fs::create_dir(&self.name).unwrap();

        std::env::set_current_dir(&self.name).unwrap();
        if std::fs::File::create("README.md").is_err() {
            println!("{}: Unable to create README.md.", "Warning".yellow());
        }

        std::process::Command::new("git")
            .args(["init"])
            .output()
            .unwrap();

        std::fs::write(".gitignore", self.software_builder.ignore_str()).unwrap();

        println!("Creating docs directory...");
        std::fs::create_dir("docs").unwrap();
    }

    fn setup_src(&self) {
        std::fs::create_dir("src").unwrap();
        std::env::set_current_dir("src").unwrap();
        self.software_builder.create();
        std::env::set_current_dir("..").unwrap();
    }

    fn create_github_repo(&self) -> String {
        println!("Creating GitHub Repo...");
        let visibility = match self.repo_visibility {
            Visibility::Public => "--public",
            Visibility::Private => "--private",
        };
        let res = std::process::Command::new("gh")
            .args(["repo"])
            .args(["create"])
            .args([visibility])
            .args([&self.name])
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
#[command(version = "v0.0.1")]
#[command(propagate_version = true)]
struct Args {
    #[clap(short, long, group = "language")]
    csharp: bool,
    #[clap(short, long, group = "language")]
    rust: bool,
    #[clap(short, long, required = true)]
    name: String,
    #[clap(short, long, default_value_t = false)]
    public: bool,
    #[clap(short, long, default_value_t = false)]
    lib: bool,
}

impl<'a> From<Args> for App<'a> {
    fn from(args: Args) -> Self {
        let software_project_builder: Box<dyn SoftwareProjectBuilder> =
            match (args.rust, args.csharp) {
                (_, true) => Box::new(CSharpProjectBuilder {
                    name: args.name.as_str(),
                    lib: args.lib,
                }),
                (_, _) => Box::new(RustProjectBuilder {
                    name: args.name.as_str(),
                    lib: args.lib,
                }),
            };
        Self {
            name: args.name.as_str(),
            software_builder: software_project_builder,
            repo_visibility: match args.public {
                true => Visibility::Public,
                false => Visibility::Private,
            },
        }
    }
}
