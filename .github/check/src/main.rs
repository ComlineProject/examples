//! Compiles every example with core's compiler, and fails on a reference or an
//! import it refuses (`reference::*`, `import::*`). Other errors are listed and
//! let through: see where they are matched.
//!
//! An example is a directory with a `meta.json`; each `<name>.ids` in it is a
//! schema of one package, in the namespace `<name>` (see the README).

use std::{fs, path::Path, process::ExitCode};

use comline_core::package::build::PackageSources;

fn compile(schemas: &[(String, String)]) -> Result<(), String> {
    let mut sources = PackageSources::new();
    for (name, text) in schemas {
        sources = sources.schema([name.clone()], text.clone());
    }
    sources.compile().map(|_| ()).map_err(|e| format!("{e:?}"))
}

fn refused(error: &str) -> bool {
    error.contains("reference::") || error.contains("import::")
}

/// Cases the rule must refuse, and one it must accept: if the compiler under
/// test doesn't enforce the rule, the examples passing says nothing.
fn controls() -> Result<(), String> {
    let types = "struct User {\n    id: u32\n}\n".to_string();
    let with = |api: &str| {
        vec![("types".to_string(), types.clone()), ("api".to_string(), api.to_string())]
    };
    let refused_cases = [
        (
            "use package::types::User\n\nstruct Account {\n    owner: types::User\n}\n",
            "reference::qualifier-not-in-scope",
        ),
        (
            "use package::types\n\nstruct Account {\n    owner: User\n}\n",
            "reference::not-in-scope",
        ),
    ];
    for (api, code) in refused_cases {
        match compile(&with(api)) {
            Err(e) if e.contains(code) => println!("control: refused with {code}"),
            Err(e) => return Err(format!("control refused, but not with {code}:\n{e}")),
            Ok(()) => return Err(format!("control compiled, expected {code}:\n{api}")),
        }
    }
    let accepted = "use package::types\n\nstruct Account {\n    owner: types::User\n}\n";
    compile(&with(accepted)).map_err(|e| format!("control refused, expected to compile:\n{e}"))?;
    println!("control: accepted");
    Ok(())
}

fn main() -> ExitCode {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    if let Err(e) = controls() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    let mut dirs: Vec<_> = fs::read_dir(&root)
        .expect("read the examples directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.join("meta.json").is_file())
        .collect();
    dirs.sort();

    let mut failed = false;
    for dir in &dirs {
        let mut schemas = Vec::new();
        let mut files: Vec<_> = fs::read_dir(dir)
            .expect("read an example")
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "ids"))
            .collect();
        files.sort();
        for file in files {
            let name = Path::new(&file).file_stem().unwrap().to_string_lossy().into_owned();
            schemas.push((name, fs::read_to_string(&file).expect("read a schema")));
        }
        let name = dir.file_name().unwrap().to_string_lossy();
        match compile(&schemas) {
            Ok(()) => println!("{name}: compiles"),
            Err(e) if refused(&e) => {
                println!("{name}: REFUSED\n{e}");
                failed = true;
            }
            // The compiler's default settings aren't the playground's: a
            // `string` with no maximum (`bounds::unbounded`) and `@framing`
            // (`settings::forbidden`) are refused here and not there. They
            // are not what this checks, so they are listed and let through.
            Err(e) => {
                let codes: Vec<&str> = e
                    .split('[')
                    .skip(1)
                    .filter_map(|rest| rest.split(']').next())
                    .filter(|code| code.contains("::"))
                    .collect();
                println!("{name}: no reference or import error; other errors, not checked here: {codes:?}");
            }
        }
    }
    println!("{} examples", dirs.len());
    if failed { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
