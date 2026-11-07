//! Trindle Help: the help window, started as `trindle-help [PAGE[#SECTION] [ERROR TEXT]]`.

mod help;
mod host;

use trindle_toolkit as toolkit;

#[used]
#[link_section = ".init_array"]
static TOOLKIT_HOST: extern "C" fn() = host::set;

fn main() -> std::process::ExitCode {
    // SAFETY: a setting of the C allocator, made before anything else runs.
    unsafe { libc::mallopt(libc::M_ARENA_MAX, 2) };
    let args: Vec<String> = std::env::args().collect();
    std::process::ExitCode::from(help::main(args) as u8)
}
