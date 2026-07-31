use edge_application_observer::StdObserveFactory;

#[test]
fn test_std_observe_factory_is_constructible_happy() {
    let f = StdObserveFactory;
    assert_eq!(std::mem::size_of_val(&f), 0, "StdObserveFactory is ZST");
}

#[test]
fn test_std_observe_factory_two_instances_are_distinct_error() {
    let a = StdObserveFactory;
    let b = StdObserveFactory;
    assert_eq!(std::mem::size_of_val(&a), 0, "both instances are ZST");
    assert_eq!(std::mem::size_of_val(&b), 0, "both instances are ZST");
}

#[test]
fn test_std_observe_factory_unit_struct_zero_size_edge() {
    assert_eq!(std::mem::size_of::<StdObserveFactory>(), 0);
}
