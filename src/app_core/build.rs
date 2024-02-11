use std::env;
fn main() {
    let network = env::var("DFX_NETWORK");

    if network.is_ok() {
        if network.unwrap() == "local" {
            println!("cargo:rustc-cfg=network=\"local\"")
        } else {
            println!("cargo:rustc-cfg=network=\"ic\"")
        }
    } else {
        println!("cargo:rustc-cfg=network=\"local\"")
    }
    
}