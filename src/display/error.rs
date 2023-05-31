/// Enumeration of possible display errors
#[derive(Debug, Clone, Copy)]
pub enum DisplayError {

    /// Happens when a display manager is not supported.
    NotSupported,

    /// Happens when no display server is found.
    NoDisplayServer,

    /// Happens when trying to resize a [Display] outside of allowed boundaries.
    SizeError,

    /// Happens when trying get hardware screen details failed.
    ScreenDetailError,


    /// Happens when trying to add the same [KEventReceiver] twice to a [KWindow].
    ReceiverAlreadyExists,

    /// Happens when trying to remove a [KEventReceiver] not added to a [KWindow].
    ReceiverNotFound,

}