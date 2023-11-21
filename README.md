# Project Create

## About 

Intended to be a simple command tool to create project directories, boilerplate, and git repository.

## Requirements

* Authorised GitHub CLI in path.
* Dotnet in path.
* Rust's Cargo in path.

## Usage 

### Example 1

`create-project --name ProjectName -c webapi`

This will create a project structure shown below, initialise a git repository, create a private GitHub repository ([Username]/ProjectName), and push the initial project structure to GitHub.

```
ProjectName
├── README.md
├── .gitignore
└── software
    └── ProjectName
        ├── appsettings.Development.json
        ├── appsettings.json
        ├── Program.cs
        ├── ProjectName.csproj
        ├── ProjectName.http
        └── Properties
            └── launchSettings.json
```


### Example 2

`create-project --name rust_project -r bin --public`

This will create a binary Rust project under the name "rust_project", initialise a git repository, create a **public** GitHub repository ([Username]/rust_project), and push the initial project structure to GitHub.

## Version

### 0.1.0