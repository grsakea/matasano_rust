extern crate hello_world;

#[test]
fn test_hamming() {
    assert_eq!(hello_world::textproc::hamming_distance("this is a test", "wokka wokka!!!"), 37);
}

