use crate::prelude::*;

pub trait SoftwareProjectBuilder {
    fn create(&self);
    fn ignore_str(&self) -> &'static str;
}

pub struct RustProjectBuilder {
    pub name: String,
    pub lib: bool,
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

pub struct CSharpProjectBuilder {
    pub name: String,
    pub lib: bool,
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
