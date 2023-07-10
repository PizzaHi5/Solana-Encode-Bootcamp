// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

// I AM DONE

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(i64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value > 0 { //Some(value > 0) == Some(0) {
            Ok(PositiveNonzeroInteger(value))
        } else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Err(CreationError::Negative)
        }
        //match value {
        //    Err(PositiveNonzeroInteger(x)) => Err(CreationError::Negative),
        //    Ok(PositiveNonzeroInteger(x)) => Ok(PositiveNonzeroInteger(x))
        //}
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
