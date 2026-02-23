extern crate winres;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

//keep runtime assets in the build output and configure platform specific icons
fn main() {
    assets_to_target();
    windows_icon();
    macos_icon();
}

//this build step only applies when compiling on windows
fn windows_icon() {
    if !cfg!(target_os = "windows") {
        return;
    }
    //embed the windows app icon for the .exe, rerun build.rs if the file changes
    println!("cargo:rerun-if-changed=assets/game/icon.ico");
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/game/icon.ico");
    res.compile().unwrap();
}

//this build step only applies when compiling on macOS
fn macos_icon() {
    if !cfg!(target_os = "macos") {
        return;
    }
    println!("cargo:rerun-if-changed=assets/game/icon.icns");
}

fn assets_to_target() {
    //resolve the project root so paths work in any build context
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let assets_dir = manifest_dir.join("assets");
    //if the assets folder is missing, skip copying
    if !assets_dir.exists() {
        return;
    }

    //track all asset files for rebuilds when any change
    rerun_if_changed(&assets_dir);

    //use the build profile to match cargo's output layout
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_assets_dir = manifest_dir.join("target").join(profile).join("assets");
    //copy assets so relative paths work when running from target
    copy_directory(&assets_dir, &target_assets_dir)
        .unwrap_or_else(|error| panic!("Failed to copy assets: {}", error));
}

fn rerun_if_changed(dir: &Path) {
    //recursively emit cargo rebuild hints for every asset
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            //subfolders so nested assets are tracked too
            if path.is_dir() {
                rerun_if_changed(&path);
            } else if let Some(path_str) = path.to_str() {
                //cargo watches file path
                println!("cargo:rerun-if-changed={}", path_str);
            }
        }
    }
}

//recursively copy the directory, creating the destination (if needed)
fn copy_directory(src: &Path, dst: &Path) -> std::io::Result<()> {
    //create the destination tree
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    //"mirror" the assets directory tree into target output
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        //subfolders
        if src_path.is_dir() {
            copy_directory(&src_path, &dst_path)?;
        } else {
            //overwrite any stale file
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
