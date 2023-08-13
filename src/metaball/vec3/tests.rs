use crate::metaball::vec3::Vec3;

#[test]
fn given_right_hand_side_is_smaller_when_subtract_then_result_should_be_2() {
    let result: Vec3 = Vec3::new(4, 4, 4) - Vec3::new(2, 2, 2);
    assert_eq!(result, Vec3::new(2, 2, 2))
}

#[test]
fn given_right_hand_side_is_larger_when_subtract_then_result_should_be_negative_2() {
    let result: Vec3 = Vec3::new(4, 4, 4) - Vec3::new(6, 6, 6);
    assert_eq!(result, Vec3::new(-2, -2, -2))
}