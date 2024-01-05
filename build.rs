use std::{env, io};

// https://github.com/mxre/winres/blob/master/example/build.rs

fn main() -> io::Result<()> {
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        if std::env::var("PROFILE").unwrap() == "release" {
            winres::WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("resource/app_icon/app_icon150.ico")
            .set_manifest_file("manifest.xml")
            .compile()?;
        }
    }
    Ok(())
}
