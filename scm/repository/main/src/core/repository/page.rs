//! Inherent impl for [`Page`].

use crate::api::Page;

impl<T> Page<T> {
    /// Creates a new `Page` with the given items, total count, offset, and limit.
    pub fn new(items: Vec<T>, total: usize, offset: usize, limit: usize) -> Self {
        Self {
            items,
            total,
            offset,
            limit,
        }
    }

    /// Returns `true` if there are more items beyond this page.
    pub fn has_more(&self) -> bool {
        self.offset + self.items.len() < self.total
    }

    /// Returns the offset to use for the next page, or `None` if this is the last page.
    pub fn next_offset(&self) -> Option<usize> {
        self.has_more().then_some(self.offset + self.items.len())
    }
}
