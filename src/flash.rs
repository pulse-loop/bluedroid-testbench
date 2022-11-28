use std::path::Path;

use log::info;

pub fn flash(library_path: String) -> Result<(), anyhow::Error> {
    let path = Path::new(&library_path);

    if !path.exists() {
        return Err(anyhow::anyhow!("Path does not exist."));
    }

    info!(
        "Trying to flash target device with example from \"{}\".",
        path.canonicalize().unwrap().display()
    );

    let mut command = std::process::Command::new("cargo")
        .arg("espflash")
        .arg("flash")
        .arg("--example")
        .arg("server")
        .arg("--release")
        .arg("--baud")
        .arg("921600")
        .current_dir(path)
        .spawn()?;

    let status = command.wait()?;
    if status.success() {
        info!("Flashed target device successfully.");
        Ok(())
    } else {
        Err(anyhow::anyhow!("Flashing failed."))
    }
}
