fn main() {
    match lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .always_use_colors()
        .force_build(true)
        .emit_rerun_directives(true)
        .emit_comments(true)
        .emit_report(true)
        .log_quiet()
        .process() {
            Ok(_) => {},
            Err(_) => println!("Error: Could not generate parser"),
        }
}
