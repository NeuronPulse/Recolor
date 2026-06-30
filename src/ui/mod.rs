mod generate;
mod library;
pub mod piano_roll;
mod sidebar;
mod tracks;
mod top_bar;

pub use generate::render_generate;
pub use library::render_library;
pub use sidebar::render_sidebar;
pub use tracks::{render_tracks, TrackView};
pub use top_bar::render_top_bar;
