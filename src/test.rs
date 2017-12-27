use parse_float_radix;

#[test]
fn test1() {
    let result = parse_float_radix("42.1", 8).unwrap();
    let expected = 34.125;
    assert_eq!(result, expected);
}
