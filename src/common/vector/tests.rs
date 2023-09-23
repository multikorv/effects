use crate::common::vector::{Vec2, self};

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

#[test]
fn given_vectors_are_one_away_from_center_in_opposite_direction_when_measuring_distance_then_distance_should_be_2() {
    let result = vector::distance(&Vec2::new(1.0, 0.0), &Vec2::new(-1.0, 0.0));
    assert_eq!(result, 1.0)
}

#[test]
fn given_vectors_are_equal_when_measuring_distance_then_distance_should_be_0() {
    let result = vector::distance(&Vec2::new(3.0, 3.0), &Vec2::new(3.0, 3.0));
    assert_eq!(result, 0.0)
}