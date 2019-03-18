// Widget related properties.

pub use self::font_icon::*;
pub use self::image::*;
pub use self::text::*;
pub use self::text_selection::*;
pub use self::water_mark::*;

mod font_icon;
mod image;
mod text;
mod text_selection;
mod water_mark;

#[cfg(test)]
mod tests;
