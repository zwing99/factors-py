use rstest::rstest;

#[rstest]
#[case::factor_7(7, vec![1, 7])]
#[case::factor_9(9, vec![1, 3, 9])]
#[case::factor_12(12, vec![1, 2, 3, 4, 6, 12])]
fn test_factor(#[case] number: u128, #[case] expected: Vec<u128>) {
    let result = super::_factors(number);
    assert_eq!(result.0, expected);
}
