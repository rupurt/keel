use serde::Serialize;

#[derive(Serialize)]
struct TestFm {
    z: String,
    a: String,
    b: u32,
}

#[test]
fn test_serde_yaml_determinism() {
    let fm = TestFm {
        z: "last".to_string(),
        a: "first".to_string(),
        b: 42,
    };
    let s = serde_yaml::to_string(&fm).unwrap();
    // Struct field order should be preserved
    assert_eq!(s, "z: last
a: first
b: 42
");
}
