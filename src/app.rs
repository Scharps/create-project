use crate::prelude::*;
use crate::software::{
    CSharpProjectBuilder, RustProjectBuilder, SoftwareBuildResult, SoftwareProjectBuilder,
};
use clap::Parser;
use colored::Colorize;

const SOFTWARE_DIR: &str = "software";

pub struct App {
    name: String,
    repo_visibility: Visibility,
    software_builder: Option<Box<dyn SoftwareProjectBuilder>>,
}

enum Visibility {
    Public,
    Private,
}

impl App {
    pub fn run(&self) {
        self.setup_directory();
        if let Some(software_builder) = &self.software_builder {
            if let Err(err) = App::set_up_software_project(software_builder.as_ref()) {
                println!("{err}");
                // println!("Reverting creation of project directory");
                // TODO: Revert creation of project directory
            }
        }
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

        if let Some(software_builder) = &self.software_builder {
            std::fs::write(".gitignore", software_builder.ignore_str()).unwrap();
        }

        println!("Creating docs directory...");
        std::fs::create_dir("docs").unwrap();
    }

    fn set_up_software_project(
        software_builder: &dyn SoftwareProjectBuilder,
    ) -> SoftwareBuildResult {
        println!("Creating software project folder...");
        std::fs::create_dir(SOFTWARE_DIR).unwrap();
        std::env::set_current_dir(SOFTWARE_DIR).unwrap();
        let res = software_builder.create();
        std::env::set_current_dir("..").unwrap();
        res
    }

    fn create_github_repo(&self) -> String {
        println!("Creating GitHub Repo...");
        let visibility = match self.repo_visibility {
            Visibility::Public => "--public",
            Visibility::Private => "--private",
        };
        let res = gh!("repo", "create", visibility, &self.name);
        let res = res.stdout;
        String::from_utf8(res).unwrap().trim_end().to_string()
    }

    fn commit_and_push(&self, github: String) {
        println!("Pushing to repository...");
        git!("remote", "add", "origin", &format!("{}.git", github));
        git!("add", ".");
        git!("branch", "-m", "main");
        git!("commit", "-m", "Project initialisation");
        git!("push", "-u", "origin", "main");
    }
}

#[derive(Parser, Debug)]
#[command(name = "Project Builder")]
#[command(author = "Samuel J.")]
#[command(about = "Creates starter project structure.")]
#[command(version = "v0.1.0")]
#[command(propagate_version = true)]
pub struct Args {
    #[clap(short, long, group = "language")]
    csharp_project: Option<String>,
    #[clap(short, long, group = "language")]
    rust_project: Option<String>,
    #[clap(short, long, required = true)]
    name: String,
    #[clap(short, long, default_value_t = false)]
    public: bool,
}

impl From<Args> for App {
    fn from(args: Args) -> Self {
        let software_project_builder: Option<Box<dyn SoftwareProjectBuilder>> =
            match (args.csharp_project, args.rust_project) {
                (Some(cs_template), _) => Some(Box::new(CSharpProjectBuilder {
                    name: args.name.clone(),
                    template: cs_template,
                })),
                (_, Some(rs_template)) => Some(Box::new(RustProjectBuilder {
                    name: args.name.clone(),
                    template: rs_template,
                })),
                _ => None,
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
