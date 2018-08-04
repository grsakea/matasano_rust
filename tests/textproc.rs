extern crate hello_world;

#[test]
fn test_hamming() {
    let text1 = "this is a test".as_bytes().to_vec();
    let text2 = "wokka wokka!!!".as_bytes().to_vec();
    assert_eq!(hello_world::textproc::hamming_distance(&text1, &text2), 37);
}

