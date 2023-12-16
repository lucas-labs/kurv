//! List Command
//! Prints the list of eggs registered in kurv and their status as a table

use {
    anyhow::Result,
    pico_args::Arguments,
};

pub fn run(
    args: &mut Arguments
) -> Result<()> {
    // print the list of eggs

    println!("kurv 0.1.0");
    println!("A simple process manager.");
    println!();
    println!("List of eggs:");
    println!();
    println!("| Name | Status |");
    println!("| ---- | ------ |");
    println!("| egg1 | active |");
    println!("| egg2 | active |");

    Ok(())
}
