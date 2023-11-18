use clap::Parser;
use colored::Colorize;

macro_rules! exec_command {
    ($comm: expr, $($arg:expr),+ ) => {
        std::process::Command::new($comm)
            .args([$($arg),*])
            .output()
            .unwrap()
    };
}

macro_rules! git {
    ($($arg:expr),+) => {
        exec_command!("git", $($arg),+)
    };
}

macro_rules! gh {
    ($($arg:expr),+) => {
        exec_command!("gh", $($arg),+)
    };
}

macro_rules! cargo {
    ($($arg:expr),+) => {
        exec_command!("cargo", $($arg),+)
    };
}

macro_rules! dotnet {
    ($($arg:expr),+) => {
        exec_command!("dotnet", $($arg),+)
    };
}

pub struct App {
    name: String,
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

struct RustProjectBuilder {
    name: String,
    lib: bool,
}

impl SoftwareProjectBuilder for RustProjectBuilder {
    fn create(&self) {
        let project_type = match self.lib {
            true => "--lib",
            false => "--bin",
        };
        cargo!("new", project_type, self.name.as_str());
    }

    fn ignore_str(&self) -> &'static str {
        "target"
    }
}

struct CSharpProjectBuilder {
    name: String,
    lib: bool,
}

impl SoftwareProjectBuilder for CSharpProjectBuilder {
    fn create(&self) {
        let project_type = match self.lib {
            true => "classlib",
            false => "console",
        };
        dotnet!("new", project_type, "-o", self.name.as_str());
    }

    fn ignore_str(&self) -> &'static str {
        "bin\nobj"
    }
}

impl App {
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

        git!("init");

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
        let res = gh!("repo", "create", visibility, &self.name).stdout;
        String::from_utf8(res).unwrap().trim_end().to_string()
    }

    fn commit_and_push(&self, github: String) {
        println!("Pushing to repository...");
        git!("remote", "add", "origin", &format!("{}.git", github));
        git!("add", ".");
        git!("branch", "-m", "main");
        git!("commit", "-m", "Project initialisation");
        git!("push");
    }
}

#[derive(Parser, Debug)]
#[command(name = "Project Builder")]
#[command(author = "Samuel J.")]
#[command(about = "Creates starter project structure.")]
#[command(version = "v0.0.1")]
#[command(propagate_version = true)]
pub struct Args {
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

impl From<Args> for App {
    fn from(args: Args) -> Self {
        let software_project_builder: Box<dyn SoftwareProjectBuilder> =
            match (args.rust, args.csharp) {
                (_, true) => Box::new(CSharpProjectBuilder {
                    name: args.name.clone(),
                    lib: args.lib,
                }),
                (_, _) => Box::new(RustProjectBuilder {
                    name: args.name.clone(),
                    lib: args.lib,
                }),
            };
        Self {
            name: args.name,
            software_builder: software_project_builder,
            repo_visibility: match args.public {
                true => Visibility::Public,
                false => Visibility::Private,
            },
        }
    }
}
