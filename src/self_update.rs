use self_update::cargo_crate_version;

pub fn update() -> Result<(), Box<dyn (::std::error::Error)>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("leonovk")
        .repo_name("minicode")
        .bin_name("minicode")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;

    println!("Update status: `{}`!", status.version());
    Ok(())
}
