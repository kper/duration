use crate::diff::diff as work;

#[test]
fn test_simple_calculation() {
    let work_input = "09:00-15:00".to_string();
    let pause_input = "".to_string();
    
    let diff = work(work_input, pause_input);

    assert_eq!(6.0, diff.unwrap().get_hours(), "Hours doesn't match");
}

#[test]
fn test_simple_calculation_with_break() {
    let work_input = "09:00-15:00".to_string();
    let pause_input = "13:00-14:00".to_string();
    
    let diff = work(work_input, pause_input);

    assert_eq!(5.0, diff.unwrap().get_hours(), "Hours doesn't match");
}

#[test]
fn test_simple_calculation_with_multiple_breaks() {
    let work_input = "09:00-15:00".to_string();
    let pause_input = "13:00-14:00;14:00-15:00".to_string();

    let diff = work(work_input, pause_input);

    assert_eq!(4.0, diff.unwrap().get_hours(), "Hours doesn't match");
}