//! Integration tests for `Page<T>` — paginated result wrapper.

use edge_application_repository::Page;

/// @covers: Page::new — all fields stored correctly
#[test]
fn test_new_stores_all_fields_happy() {
    let page: Page<u32> = Page::new(vec![1, 2, 3], 10, 0, 3);
    assert_eq!(page.items, vec![1, 2, 3]);
    assert_eq!(page.total, 10);
    assert_eq!(page.offset, 0);
    assert_eq!(page.limit, 3);
}

/// @covers: Page::has_more — returns false when on the last page
#[test]
fn test_has_more_last_page_returns_false_error() {
    let page: Page<u32> = Page::new(vec![8, 9, 10], 3, 0, 10);
    assert!(!page.has_more());
}

/// @covers: has_more — empty items with non-zero total
#[test]
fn test_has_more_empty_items_nonzero_total_returns_true_edge() {
    let page: Page<u32> = Page::new(vec![], 3, 0, 2);
    assert!(page.has_more());
}

/// @covers: Page::next_offset — mid-page returns correct next offset
#[test]
fn test_next_offset_mid_page_returns_correct_offset_edge() {
    let page: Page<u32> = Page::new(vec![1, 2], 6, 0, 2);
    assert_eq!(page.next_offset(), Some(2));
}

/// @covers: next_offset — last page returns None
#[test]
fn test_next_offset_last_page_returns_none_happy() {
    let page: Page<u32> = Page::new(vec![5], 1, 0, 10);
    assert_eq!(page.next_offset(), None);
}
