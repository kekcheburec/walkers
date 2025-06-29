//! Extra functionalities that can be used with the map.

mod image;
mod labeled_symbol;
mod places;
pub use crate::tiles::Texture;
pub use image::Image;
pub use labeled_symbol::{
    LabeledSymbol, LabeledSymbolGroup, LabeledSymbolGroupStyle, LabeledSymbolStyle, Symbol,
};
pub use places::{Group, GroupedPlaces, Place, Places};
