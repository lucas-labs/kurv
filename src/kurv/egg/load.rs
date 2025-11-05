use {
    super::Egg,
    anyhow::{Context, Result, anyhow},
    log::debug,
    serde::Deserialize,
    std::{fs::File, path::PathBuf},
};

impl Egg {
    pub fn load(path: PathBuf) -> Result<Egg> {
        if !path.exists() {
            debug!(".kurv file not found, starting fresh (searched in {})", path.display());
            debug!("you can set KURV_HOME to change the directory");
            return Err(anyhow!(format!("file {} not found", path.display())));
        }

        let rdr = File::open(&path)
            .with_context(|| format!("failed to open egg file: {}", path.display()))?;

        let mut egg: Egg = Egg::deserialize(serde_saphyr::from_reader(rdr))
            .context(format!("failed to parse egg file: {}", path.display()))?;

        // remove id if it has one, so that it doesn't collide with an existing egg
        // the server will assign an ID automatically when spawning.
        egg.id = None;

        Ok(egg)
    }
}
