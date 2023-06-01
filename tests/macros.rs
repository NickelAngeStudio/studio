
/// Macro used as shortcut to test correct error received on [Result].
/// 
/// Reference(s)
/// <https://stackoverflow.com/questions/53124930/how-do-you-test-for-a-specific-rust-error/53124931#53124931>
#[macro_export]
macro_rules! assert_err {
    ($expression:expr, $error:pat) => {
        if let Err($error) = $expression  {
            // Nothing because error is expected.  
        } else {
            panic!("Expected Err(`{:?}`) but got Ok() instead!", stringify!($error));    
        }
    }
}

/// Macro used as shortcut to test and retrieve [Result] value and panic on Err.
/// 
/// Can take a value to evaluate if got 2 parameters.
#[macro_export]
macro_rules! assert_ok {
    ($expression:expr, $control:expr) => {
        match $expression {
            // Return value of Ok
            Ok(value) => assert_eq!(value, $control, "Ok({:?}) received when Ok({:?}) was expected instead!", $control, value),
            Err(err) => panic!("Received Err(`{:?}`) when Ok() was expected instead!", err),
        }
    };

    ($expression:expr) => {
        match $expression {
            // Return value of Ok
            Ok(value) => value,
            Err(err) => panic!("Received Err(`{:?}`) when Ok() was expected instead!", err),
        }
    };
    
}