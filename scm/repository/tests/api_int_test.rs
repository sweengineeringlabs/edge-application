//! Layer-level coverage for `api/repository/dto/*.rs` request/response types.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_repository::{
    AlwaysMatchSpec, CountByResponse, MatchingEntitiesResponse, MatchingEntityResponse, Page,
    RepositoryCountResponse, RepositoryDeleteResponse, RepositoryExistsResponse,
    RepositoryFindResponse, RepositoryIdRequest, RepositoryListPageRequest,
    RepositoryListPageResponse, RepositoryListRequest, RepositoryListResponse,
    RepositorySaveRequest, Spec, SpecMatchesRequest, SpecMatchesResponse, SpecRequest,
};

/// @covers: CountByResponse
#[test]
fn test_count_by_response_holds_count_happy() {
    let r = CountByResponse { count: 3 };
    assert_eq!(r.count, 3);
}

/// @covers: MatchingEntitiesResponse
#[test]
fn test_matching_entities_response_holds_items_happy() {
    let r = MatchingEntitiesResponse {
        items: vec![1, 2, 3],
    };
    assert_eq!(r.items, vec![1, 2, 3]);
}

/// @covers: MatchingEntitiesResponse
#[test]
fn test_matching_entities_response_empty_items_edge() {
    let r: MatchingEntitiesResponse<u32> = MatchingEntitiesResponse { items: vec![] };
    assert!(r.items.is_empty());
}

/// @covers: MatchingEntityResponse
#[test]
fn test_matching_entity_response_holds_some_happy() {
    let r = MatchingEntityResponse {
        entity: Some(42u32),
    };
    assert_eq!(r.entity, Some(42));
}

/// @covers: MatchingEntityResponse
#[test]
fn test_matching_entity_response_none_edge() {
    let r: MatchingEntityResponse<u32> = MatchingEntityResponse { entity: None };
    assert_eq!(r.entity, None);
}

/// @covers: RepositoryCountResponse
#[test]
fn test_repository_count_response_holds_count_happy() {
    let r = RepositoryCountResponse { count: 7 };
    assert_eq!(r.count, 7);
}

/// @covers: RepositoryDeleteResponse
#[test]
fn test_repository_delete_response_removed_true_happy() {
    let r = RepositoryDeleteResponse { removed: true };
    assert!(r.removed);
}

/// @covers: RepositoryDeleteResponse
#[test]
fn test_repository_delete_response_removed_false_error() {
    let r = RepositoryDeleteResponse { removed: false };
    assert!(!r.removed);
}

/// @covers: RepositoryExistsResponse
#[test]
fn test_repository_exists_response_exists_true_happy() {
    let r = RepositoryExistsResponse { exists: true };
    assert!(r.exists);
}

/// @covers: RepositoryExistsResponse
#[test]
fn test_repository_exists_response_exists_false_error() {
    let r = RepositoryExistsResponse { exists: false };
    assert!(!r.exists);
}

/// @covers: RepositoryFindResponse
#[test]
fn test_repository_find_response_holds_entity_happy() {
    let r = RepositoryFindResponse {
        entity: Some("hello".to_string()),
    };
    assert_eq!(r.entity.as_deref(), Some("hello"));
}

/// @covers: RepositoryFindResponse
#[test]
fn test_repository_find_response_none_edge() {
    let r: RepositoryFindResponse<String> = RepositoryFindResponse { entity: None };
    assert!(r.entity.is_none());
}

/// @covers: RepositoryIdRequest
#[test]
fn test_repository_id_request_holds_id_happy() {
    let id = 42u32;
    let r = RepositoryIdRequest { id: &id };
    assert_eq!(*r.id, 42);
}

/// @covers: RepositoryListPageRequest
#[test]
fn test_repository_list_page_request_holds_offset_and_limit_happy() {
    let r = RepositoryListPageRequest {
        offset: 5,
        limit: 10,
    };
    assert_eq!(r.offset, 5);
    assert_eq!(r.limit, 10);
}

/// @covers: RepositoryListPageRequest
#[test]
fn test_repository_list_page_request_zero_limit_edge() {
    let r = RepositoryListPageRequest {
        offset: 0,
        limit: 0,
    };
    assert_eq!(r.limit, 0);
}

/// @covers: RepositoryListPageResponse
#[test]
fn test_repository_list_page_response_holds_page_happy() {
    let r = RepositoryListPageResponse {
        page: Page::new(vec![1, 2], 5, 0, 2),
    };
    assert_eq!(r.page.items, vec![1, 2]);
    assert!(r.page.has_more());
}

/// @covers: RepositoryListRequest
#[test]
fn test_repository_list_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<RepositoryListRequest>(), 0);
    let _ = RepositoryListRequest;
}

/// @covers: RepositoryListResponse
#[test]
fn test_repository_list_response_holds_items_happy() {
    let r = RepositoryListResponse {
        items: vec!["a", "b"],
    };
    assert_eq!(r.items, vec!["a", "b"]);
}

/// @covers: RepositoryListResponse
#[test]
fn test_repository_list_response_empty_edge() {
    let r: RepositoryListResponse<u32> = RepositoryListResponse { items: vec![] };
    assert!(r.items.is_empty());
}

/// @covers: RepositorySaveRequest
#[test]
fn test_repository_save_request_holds_id_and_entity_happy() {
    let r = RepositorySaveRequest {
        id: 1u32,
        entity: "value".to_string(),
    };
    assert_eq!(r.id, 1);
    assert_eq!(r.entity, "value");
}

/// @covers: SpecMatchesRequest
#[test]
fn test_spec_matches_request_holds_entity_happy() {
    let entity = 9u32;
    let r = SpecMatchesRequest { entity: &entity };
    assert_eq!(*r.entity, 9);
}

/// @covers: SpecMatchesResponse
#[test]
fn test_spec_matches_response_true_happy() {
    let r = SpecMatchesResponse { matches: true };
    assert!(r.matches);
}

/// @covers: SpecMatchesResponse
#[test]
fn test_spec_matches_response_false_error() {
    let r = SpecMatchesResponse { matches: false };
    assert!(!r.matches);
}

/// @covers: SpecRequest
#[test]
fn test_spec_request_holds_boxed_spec_happy() {
    let req = SpecRequest {
        spec: Box::new(AlwaysMatchSpec::new()),
    };
    let result = req
        .spec
        .matches(SpecMatchesRequest { entity: &1u32 })
        .unwrap();
    assert!(result.matches);
}

/// @covers: AlwaysMatchSpec
#[test]
fn test_always_match_spec_matches_any_entity_happy() {
    let result = AlwaysMatchSpec::new()
        .matches(SpecMatchesRequest {
            entity: &"anything",
        })
        .unwrap();
    assert!(result.matches);
}
