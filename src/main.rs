mod kurv;
mod common;

use {
    kurv::Kurv,
    anyhow::Result,
};

fn main() -> Result<()> {
    let mut kurv = Kurv::new();
    kurv.run();

    Ok(())
}
