use librustdesk::*;

#[cfg(not(target_os = "macos"))]
fn main() {}

#[cfg(target_os = "macos")]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // global_init seeds the baked-in server/relay/key defaults. The root service
    // process needs them too: it writes plists and answers IPC option queries,
    // and without this it would fall back to the public network's key.
    if !crate::common::global_init() {
        eprintln!("Global initialization failed.");
        return;
    }
    crate::common::load_custom_client();
    if args.len() > 1 && args[1] == "--write-plists" {
        if let Err(e) = librustdesk::platform::write_plists() {
            eprintln!("Failed to write plists: {}", e);
            std::process::exit(1);
        }
        std::process::exit(0);
    }
    hbb_common::init_log(false, "service");
    crate::start_os_service();
}
