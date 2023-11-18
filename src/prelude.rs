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

pub(crate) use cargo;
pub(crate) use dotnet;
pub(crate) use exec_command;
pub(crate) use gh;
pub(crate) use git;
