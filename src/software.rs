use std::process::Output;
use crate::prelude::*;

pub type SoftwareBuildResult = std::result::Result<(), String>;

pub trait SoftwareProjectBuilder {
    fn create(&self) -> SoftwareBuildResult;
    fn ignore_str(&self) -> &'static str;
}

pub struct RustProjectBuilder {
    pub name: String,
    pub template: String,
}

impl SoftwareProjectBuilder for RustProjectBuilder {
    fn create(&self) -> SoftwareBuildResult {
        let project_type = format!("--{}", self.template);
        let res = cargo!("new", project_type.as_str(), self.name.as_str());
        check_command_result(res)
    }

    fn ignore_str(&self) -> &'static str {
        "target"
    }
}

fn check_command_result(output: Output) -> SoftwareBuildResult {
    if !output.status.success() {
        return Err(String::from_utf8(output.stderr).unwrap())
    }
    Ok(())
}

pub struct CSharpProjectBuilder {
    pub name: String,
    pub template: String,
}

impl SoftwareProjectBuilder for CSharpProjectBuilder {
    fn create(&self) -> SoftwareBuildResult {
        let output = dotnet!("new", self.template.as_str(), "-o", self.name.as_str());
        check_command_result(output)        
    }

    fn ignore_str(&self) -> &'static str {
        "bin\nobj"
    }
}
