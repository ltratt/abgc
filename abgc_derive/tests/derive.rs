use abgc::Gc;
use abgc_derive::GcLayout;

#[test]
fn test_derive() {
    #[derive(GcLayout)]
    struct S(u64);

    let s = Gc::new(S(42));
    assert_eq!(s.0, 42);
}
