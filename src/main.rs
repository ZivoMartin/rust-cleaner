use anyhow::{Context, Result, bail};
use std::{
    env::args,
    fs::remove_dir_all,
    io::stdin,
    path::{Path, PathBuf},
};

#[derive(Debug)]
enum Mode {
    Dry,
    Remove,
}

struct Remover {
    mode: Mode,
    log: bool,
    confirm: bool,
}

impl Default for Remover {
    fn default() -> Self {
        Remover {
            mode: Mode::Remove,
            log: true,
            confirm: true,
        }
    }
}

impl Remover {
    fn confirm(&self, path: &Path) -> Result<bool> {
        loop {
            println!("Removing {path:?} ? (y/n)  ");
            let mut response = String::new();
            stdin().read_line(&mut response)?;
            match &response as &str {
                "y" | "yes" => break Ok(true),
                "n" | "no" => break Ok(false),
                _ => (),
            }
        }
    }

    fn log(&self, l: &str) {
        if self.log {
            println!("{l}");
        }
    }

    fn remove(&self, path: &Path) -> Result<()> {
        match self.mode {
            Mode::Dry => self.log(&format!("Would like to remove {}", path.display())),
            Mode::Remove => {
                if self.confirm {
                    if !self.confirm(path)? {
                        self.log(&format!("Skipping {path:?}"));
                        return Ok(());
                    }
                }
                remove_dir_all(path)?;
                self.log(&format!("Removing {path:?}"));
            }
        }
        Ok(())
    }
}

fn scan(path: &Path, remover: &Remover) -> Result<()> {
    let items = path
        .read_dir()
        .context("Failed to read into the given path")?
        .collect::<Vec<_>>();

    let mut target_path: Option<PathBuf> = None;
    let mut is_toml = false;

    for item in items {
        let item = match item {
            Ok(i) => i,
            _ => continue,
        };

        let item_path = item.path();

        let item_name = item.file_name();
        let item_name = match item_name.to_str() {
            Some(name) => name,
            _ => continue,
        };

        match (item_name, is_toml, &target_path) {
            ("Cargo.toml", _, None) => is_toml = true,
            ("Cargo.toml", _, Some(target_path)) => remover.remove(&target_path)?,
            ("target", true, _) => remover.remove(&item_path)?,
            ("target", false, _) => target_path = Some(item.path()),
            _ if item_path.is_dir() => scan(&item_path, remover)?,
            _ => (),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut path = None;
    let mut args_iter = args();
    args_iter.next();

    let mut remover = Remover::default();

    while let Some(arg) = args_iter.next() {
        match &arg as &str {
            "--dry-run" => {
                remover.mode = Mode::Dry;
                remover.log = true;
            }
            "--silent-run" => remover.log = false,
            "--no-confirm" => remover.confirm = false,
            _ => path = Some(PathBuf::from(arg)),
        }
    }
    let path = match path {
        Some(path) => path,
        None => bail!("You forgot to provide a path"),
    };

    if !path.exists() {
        bail!("Dir {path:?} does not exist");
    }
    if !path.is_dir() {
        bail!("Path {path:?} is not a directory");
    }

    scan(&path, &remover)?;

    Ok(())
}
