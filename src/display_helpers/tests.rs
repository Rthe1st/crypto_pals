use display_helpers;

#[test]
fn test_clean_u8_vec_to_string(){
    let binary = vec![71, 111, 32, 104, 111, 109, 101, 33];
    assert_eq!(display_helpers::u8_vec_to_string(&binary), "Go home!");
}