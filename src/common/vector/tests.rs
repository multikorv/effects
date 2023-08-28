use crate::common::vector::Vec2;

#[test]
fn given_right_hand_side_is_smaller_when_subtracti_then_result_should_be_2() {
    let result: Vec2 = Vec2::new(4.0, 4.0) - Vec2::new(2.0, 2.0);
    assert_eq!(result, Vec2::new(2.0, 2.0))
}

#[test]
fn given_right_hand_side_is_larger_when_subtract_then_result_should_be_negative_2() {
    let result: Vec2 = Vec2::new(4.0, 4.0) - Vec2::new(6.0, 6.0);
    assert_eq!(result, Vec2::new(-2.0, -2.0))
}