use crate::options::Game;

#[cfg(windows)]
pub fn patch_game(game: &Game) {
    println!("Patching game...");
    // TODO: Patch game by copying a file or by injecting a DLL?
}

#[cfg(linux)]
pub fn patch_game(game: &Game) {
    println!("Patching game...");
}

#[cfg(macos)]
pub fn patch_game(game: &Game) {
    println!("Patching game...");
}

#[cfg(windows)]
pub fn enable_proxy(game: &Game) {
    println!("Enabling proxy...");
}

#[cfg(linux)]
pub fn enable_proxy(game: &Game) {
    println!("Enabling proxy...");
}

#[cfg(macos)]
pub fn enable_proxy(game: &Game) {
    println!("Enabling proxy...");
}
