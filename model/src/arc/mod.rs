pub mod tag;
pub mod item;
pub mod volume;

pub mod prelude {
    #[doc(hidden)]
    pub use crate::prelude::{*,
        Tag as ModelTag,
        Item as ModelItem,
        Volume as ModelVolume,
    };

    #[doc(hidden)]
    pub use super::tag::Tag;

    #[doc(hidden)]
    pub use super::item::Item;

    #[doc(hidden)]
    pub use super::volume::Volume;
}