//! flameview — initial placeholder API (will be replaced by milestones).
pub fn add_one(x: i32) -> i32 {
    x.wrapping_add(1)
}

pub mod arena;
pub use arena::{FlameTree, Node, NodeId};
pub mod input;
pub mod loader;
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
/// Parse collapsed stacks from any buffered reader.
pub use loader::collapsed::load_stream;
mod summarize;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adds_one() {
        assert_eq!(add_one(41), 42);
    }
}
