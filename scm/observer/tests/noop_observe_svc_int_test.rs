use edge_application_observer::{NoopObserve, NOOP_OBSERVE_SVC};

// @covers NOOP_OBSERVE_SVC
#[test]
fn test_noop_observe_svc_key_non_empty_happy() {
    assert!(!NOOP_OBSERVE_SVC.is_empty());
}

// @covers NOOP_OBSERVE_SVC
#[test]
fn test_noop_observe_svc_key_has_edge_prefix_error() {
    assert!(NOOP_OBSERVE_SVC.starts_with("edge."));
}

// @covers NOOP_OBSERVE_SVC
#[test]
fn test_noop_observe_svc_key_stable_across_reads_edge() {
    assert_eq!(NOOP_OBSERVE_SVC, "edge.observe.noop");
}

#[test]
fn test_noop_observe_trait_accessible_via_svc_import_happy() {
    use edge_application_observer::StdObserveFactory;
    let c = StdObserveFactory::build_noop_counter();
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
}
