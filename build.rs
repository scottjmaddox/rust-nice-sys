extern crate pkg_config;

fn main() {
    let lib = match pkg_config::Config::new()
        .print_system_libs(false)
        .find("nice")
    {
        Ok(lib) => lib,
        Err(e) => {
            println!("run pkg_config fail: {:?}", e);
            std::process::exit(1);
        }
    };

    for include in lib.include_paths.iter() {
        println!("cargo:include={}", include.display());
    }
}
