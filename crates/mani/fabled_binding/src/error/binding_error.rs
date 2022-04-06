use mlua::Error;
use std::fmt::Formatter;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[non_exhaustive]
pub enum LuaBindingError {
    #[error("Syntax while parsing lua source code \n Message : {message:?}, incomplete input : {incomplete_input:?}")]
    SyntaxError {
        message: String,

        incomplete_input: bool,
    },

    #[error("Builtin operation is performed on incompatible types. This includes invoking on wrong type (such as calling or indexing nil) \n Message : {0}")]
    RuntimeError(String),

    #[error("The allocator hasn't returned the request memory. Out of memory \n Message : {0}")]
    MemoryError(String),

    #[error("Potentially unsafe action in safe mode. Use new_unsafe method \n Message : {0}")]
    SafetyError(String),

    #[error(
        "Lua state was not created by the engine and doesn't have a custom allocator attached"
    )]
    MemoryLimitNotAvailable,

    #[error("Error shouldn't happen in the current lua version 5.4")]
    MainThreadNotAvailable,

    #[error("Mutable callback has triggered Lua code that has called the same mutable callback again. \n Mutable callback can only be borrowed mutably once")]
    RecursiveMutCallback,

    #[error("Internal Rust code panic resumed previously was recovered and returned again")]
    PreviouslyResumedPanic,

    #[error("Callback or a userdata method has been called, but the callback or userdata has been destructed")]
    CallbackDestructed,

    #[error("Not enough stack space to place arguments to lua function or return values from callback \n Try to refactoring out mega function to reduce huge number of arguments or huge number of return values")]
    StackError,

    #[error("To many arguments to Function::bind internal")]
    BindError,

    #[error("Rust value could not be converted to the expected Lua value \n From : {from:?} To : {to:?} \n Message : {message:?}")]
    ToLuaConversionError {
        from: &'static str,

        to: &'static str,

        message: Option<String>,
    },

    #[error("Lua value could not be converted to the expected Rust type \n From : {from:?} To : {to:?} \n Message : {message:?}")]
    FromLuaConversionError {
        from: &'static str,

        to: &'static str,

        message: Option<String>,
    },

    #[error("Thread::resume internal was called on a inactive coroutine \n A coroutine is inactive if its main function return or an error has occurred inside the coroutine")]
    CoroutineInactive,

    #[error("A AnyUserData is not the expected type in a borrow \n Error can happen when manually using AnyUserData or implementing meta-method for binary operators")]
    UserDataTypeMismatch,

    #[error("A AnyUserData borrow failed because it has been destructed")]
    UserDataDestructed,

    #[error("A AnyUserData immutable borrow failed because it is already borrowed mutably")]
    UserDataBorrowError,

    #[error("A AnyUserData mutable borrow failed because it is already borrowed")]
    UserDataBorrowMutError,

    #[error("A MetaMethod operation is restricted \n Message : {0}")]
    MetaMethodRestricted(String),

    #[error("A MetaMethod has invalid type \n Method : {method:?} Type Name : {type_name:?} \n Message : {message:?}")]
    MetaMethodTypeError {
        method: String,
        type_name: &'static str,
        message: Option<String>,
    },

    #[error("A RegistryKey produced from a different Lua state was used")]
    MismatchedRegistryKey,

    #[error("A Rust callback returned Err raising the contained Error as a Lua error \n Cause : {cause:?} TraceBack : {traceback:?}")]
    CallbackError {
        traceback: String,
        cause: Arc<mlua::Error>,
    },

    #[error("{0}")]
    ExternalError(Arc<dyn std::error::Error + Send + Sync>),
}

impl From<mlua::Error> for LuaBindingError {
    fn from(err: Error) -> Self {
        unsafe {
            match err {
                Error::SyntaxError {
                    message,
                    incomplete_input,
                } => Self::SyntaxError {
                    message,
                    incomplete_input,
                },
                Error::RuntimeError(runtime_msg) => Self::RuntimeError(runtime_msg),
                Error::MemoryError(memory_msg) => Self::MemoryError(memory_msg),
                Error::SafetyError(safety_msg) => Self::SafetyError(safety_msg),
                Error::MemoryLimitNotAvailable => Self::MemoryLimitNotAvailable,
                Error::MainThreadNotAvailable => Self::MainThreadNotAvailable,
                Error::RecursiveMutCallback => Self::RecursiveMutCallback,
                Error::CallbackDestructed => Self::CallbackDestructed,
                Error::StackError => Self::StackError,
                Error::BindError => Self::BindError,
                Error::ToLuaConversionError { from, to, message } => {
                    Self::ToLuaConversionError { from, to, message }
                }
                Error::FromLuaConversionError { from, to, message } => {
                    Self::FromLuaConversionError { from, to, message }
                }
                Error::CoroutineInactive => Self::CoroutineInactive,
                Error::UserDataTypeMismatch => Self::UserDataTypeMismatch,
                Error::UserDataDestructed => Self::UserDataDestructed,
                Error::UserDataBorrowError => Self::UserDataBorrowError,
                Error::UserDataBorrowMutError => Self::UserDataBorrowMutError,
                Error::MetaMethodRestricted(meta_msg) => Self::MetaMethodRestricted(meta_msg),
                Error::MetaMethodTypeError {
                    method,
                    type_name,
                    message,
                } => Self::MetaMethodTypeError {
                    method,
                    type_name,
                    message,
                },
                Error::MismatchedRegistryKey => Self::MismatchedRegistryKey,
                Error::CallbackError { traceback, cause } => {
                    Self::CallbackError { traceback, cause }
                }
                Error::PreviouslyResumedPanic => Self::PreviouslyResumedPanic,
                Error::ExternalError(ext_err) => Self::ExternalError(ext_err),
                _ => std::hint::unreachable_unchecked(),
            }
        }
    }
}
