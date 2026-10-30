use orbclient::Color;

// Bun OS Aphrody palette (same values as Orbitals defaults and orbdatas orbital.toml).
pub const BAR_COLOR: Color = Color::rgba(0x11, 0x13, 0x1D, 224);
pub const BAR_HIGHLIGHT_COLOR: Color = Color::rgba(0x2E, 0x3F, 0x95, 224);
pub const TEXT_COLOR: Color = Color::rgb(0xE1, 0xE3, 0xF2);
pub const TEXT_HIGHLIGHT_COLOR: Color = Color::rgb(0xA9, 0xB8, 0xFF);

/// Bun OS default terminal: Rio when the rioterm package is installed, COSMIC Terminal otherwise.
pub fn terminal() -> String {
    for (path, exec) in [("/usr/bin/rio", "rio"), ("/usr/bin/cosmic-term", "cosmic-term")] {
        if std::path::Path::new(path).exists() {
            return exec.to_string();
        }
    }
    "cosmic-term".to_string()
}
