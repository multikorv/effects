use std::time::Duration;

use crate::common::time::Time;

#[test]
fn given_tick_is_called_when_checking_elapsed_time_then_time_should_be_updated() {
    let mut timer: Time = Time::new();

    let elapsed1 = timer.elapsed;

    std::thread::sleep(Duration::from_micros(1));
    timer.tick();

    let elapsed2 = timer.elapsed;

    assert!(elapsed1 < elapsed2)
}

#[test]
fn given_tick_is_called_once_when_checking_delta_time_then_delta_time_should_be_same_as_passed() {
    let mut timer: Time = Time::new();

    std::thread::sleep(Duration::from_micros(1));
    timer.tick();

    let elapsed = timer.elapsed;

    assert_eq!(timer.delta, elapsed)
}

#[test]
fn given_tick_is_called_twice_when_checking_delta_time_then_delta_time_should_be_smaller_than_passed() {
    let mut timer: Time = Time::new();

    std::thread::sleep(Duration::from_micros(1));
    timer.tick();

    std::thread::sleep(Duration::from_micros(1));
    timer.tick();

    assert!(
        timer.delta < timer.elapsed,
        "Expected delta time {} microseconds to be lower than elapsed {} microseconds",
        timer.delta.as_micros(),
        timer.elapsed.as_micros()
    )
}