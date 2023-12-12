mod kurv;
mod common;

use {
    kurv::Kurv,
    anyhow::Result,
    log::info
};

fn main() -> Result<()> {
    info!("Starting Kurv");

    let mut kurv = Kurv::new();
    kurv.run();

    Ok(())
}
