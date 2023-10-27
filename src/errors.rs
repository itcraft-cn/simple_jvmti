#![allow(non_snake_case, non_camel_case_types, dead_code)]

use crate::jvmti_sys::jvmtiError;
use thiserror::Error;

pub type JvmtiResult<T> = Result<T, JvmtiError>;

#[derive(Debug, Error)]
pub enum JvmtiError {
    #[error("Invalid Thread")]
    JvmtiErrorInvalidThread = 10,
    #[error("Invalid Thread Group")]
    JvmtiErrorInvalidThreadGroup = 11,
    #[error("Invalid Priority")]
    JvmtiErrorInvalidPriority = 12,
    #[error("Thread Not Suspended")]
    JvmtiErrorThreadNotSuspended = 13,
    #[error("Thread Suspended")]
    JvmtiErrorThreadSuspended = 14,
    #[error("Thread Not Alive")]
    JvmtiErrorThreadNotAlive = 15,
    #[error("Invalid Object")]
    JvmtiErrorInvalidObject = 20,
    #[error("Invalid Class")]
    JvmtiErrorInvalidClass = 21,
    #[error("Class Not Prepared")]
    JvmtiErrorClassNotPrepared = 22,
    #[error("Invalid Methodid")]
    JvmtiErrorInvalidMethodid = 23,
    #[error("Invalid Location")]
    JvmtiErrorInvalidLocation = 24,
    #[error("Invalid Fieldid")]
    JvmtiErrorInvalidFieldid = 25,
    #[error("No More Frames")]
    JvmtiErrorNoMoreFrames = 31,
    #[error("Opaque Frame")]
    JvmtiErrorOpaqueFrame = 32,
    #[error("Type Mismatch")]
    JvmtiErrorTypeMismatch = 34,
    #[error("Invalid Slot")]
    JvmtiErrorInvalidSlot = 35,
    #[error("Duplicate")]
    JvmtiErrorDuplicate = 40,
    #[error("Not Found")]
    JvmtiErrorNotFound = 41,
    #[error("Invalid Monitor")]
    JvmtiErrorInvalidMonitor = 50,
    #[error("Not Monitor Owner")]
    JvmtiErrorNotMonitorOwner = 51,
    #[error("Interrupt")]
    JvmtiErrorInterrupt = 52,
    #[error("Invalid Class Format")]
    JvmtiErrorInvalidClassFormat = 60,
    #[error("Circular Class Definition")]
    JvmtiErrorCircularClassDefinition = 61,
    #[error("Fails Verification")]
    JvmtiErrorFailsVerification = 62,
    #[error("Unsupported Redefinition Method Added")]
    JvmtiErrorUnsupportedRedefinitionMethodAdded = 63,
    #[error("Unsupported Redefinition Schema Changed")]
    JvmtiErrorUnsupportedRedefinitionSchemaChanged = 64,
    #[error("Invalid Typestate")]
    JvmtiErrorInvalidTypestate = 65,
    #[error("Unsupported Redefinition Hierarchy Changed")]
    JvmtiErrorUnsupportedRedefinitionHierarchyChanged = 66,
    #[error("Unsupported Redefinition Method Deleted")]
    JvmtiErrorUnsupportedRedefinitionMethodDeleted = 67,
    #[error("Unsupported Version")]
    JvmtiErrorUnsupportedVersion = 68,
    #[error("Names Dont Match")]
    JvmtiErrorNamesDontMatch = 69,
    #[error("Unsupported Redefinition Class Modifiers Changed")]
    JvmtiErrorUnsupportedRedefinitionClassModifiersChanged = 70,
    #[error("Unsupported Redefinition Method Modifiers Changed")]
    JvmtiErrorUnsupportedRedefinitionMethodModifiersChanged = 71,
    #[error("Unmodifiable Class")]
    JvmtiErrorUnmodifiableClass = 79,
    #[error("Not Available")]
    JvmtiErrorNotAvailable = 98,
    #[error("Must Possess Capability")]
    JvmtiErrorMustPossessCapability = 99,
    #[error("Null Pointer")]
    JvmtiErrorNullPointer = 100,
    #[error("Absent Information")]
    JvmtiErrorAbsentInformation = 101,
    #[error("Invalid Event Type")]
    JvmtiErrorInvalidEventType = 102,
    #[error("Illegal Argument")]
    JvmtiErrorIllegalArgument = 103,
    #[error("Native Method")]
    JvmtiErrorNativeMethod = 104,
    #[error("Class Loader Unsupported")]
    JvmtiErrorClassLoaderUnsupported = 106,
    #[error("Out Of Memory")]
    JvmtiErrorOutOfMemory = 110,
    #[error("Access Denied")]
    JvmtiErrorAccessDenied = 111,
    #[error("Wrong Phase")]
    JvmtiErrorWrongPhase = 112,
    #[error("Internal")]
    JvmtiErrorInternal = 113,
    #[error("Unattached Thread")]
    JvmtiErrorUnattachedThread = 115,
    #[error("Invalid Environment")]
    JvmtiErrorInvalidEnvironment = 116,
}

impl From<&jvmtiError> for JvmtiError {
    fn from(error: &jvmtiError) -> Self {
        match error {
            jvmtiError::JVMTI_ERROR_NONE => { panic!("This should not happen!") }
            jvmtiError::JVMTI_ERROR_INVALID_THREAD => { JvmtiError::JvmtiErrorInvalidThread }
            jvmtiError::JVMTI_ERROR_INVALID_THREAD_GROUP => { JvmtiError::JvmtiErrorInvalidThreadGroup }
            jvmtiError::JVMTI_ERROR_INVALID_PRIORITY => { JvmtiError::JvmtiErrorInvalidPriority }
            jvmtiError::JVMTI_ERROR_THREAD_NOT_SUSPENDED => { JvmtiError::JvmtiErrorThreadNotSuspended }
            jvmtiError::JVMTI_ERROR_THREAD_SUSPENDED => { JvmtiError::JvmtiErrorThreadSuspended }
            jvmtiError::JVMTI_ERROR_THREAD_NOT_ALIVE => { JvmtiError::JvmtiErrorThreadNotAlive }
            jvmtiError::JVMTI_ERROR_INVALID_OBJECT => { JvmtiError::JvmtiErrorInvalidObject }
            jvmtiError::JVMTI_ERROR_INVALID_CLASS => { JvmtiError::JvmtiErrorInvalidClass }
            jvmtiError::JVMTI_ERROR_CLASS_NOT_PREPARED => { JvmtiError::JvmtiErrorClassNotPrepared }
            jvmtiError::JVMTI_ERROR_INVALID_METHODID => { JvmtiError::JvmtiErrorInvalidMethodid }
            jvmtiError::JVMTI_ERROR_INVALID_LOCATION => { JvmtiError::JvmtiErrorInvalidLocation }
            jvmtiError::JVMTI_ERROR_INVALID_FIELDID => { JvmtiError::JvmtiErrorInvalidFieldid }
            jvmtiError::JVMTI_ERROR_NO_MORE_FRAMES => { JvmtiError::JvmtiErrorNoMoreFrames }
            jvmtiError::JVMTI_ERROR_OPAQUE_FRAME => { JvmtiError::JvmtiErrorOpaqueFrame }
            jvmtiError::JVMTI_ERROR_TYPE_MISMATCH => { JvmtiError::JvmtiErrorTypeMismatch }
            jvmtiError::JVMTI_ERROR_INVALID_SLOT => { JvmtiError::JvmtiErrorInvalidSlot }
            jvmtiError::JVMTI_ERROR_DUPLICATE => { JvmtiError::JvmtiErrorDuplicate }
            jvmtiError::JVMTI_ERROR_NOT_FOUND => { JvmtiError::JvmtiErrorNotFound }
            jvmtiError::JVMTI_ERROR_INVALID_MONITOR => { JvmtiError::JvmtiErrorInvalidMonitor }
            jvmtiError::JVMTI_ERROR_NOT_MONITOR_OWNER => { JvmtiError::JvmtiErrorNotMonitorOwner }
            jvmtiError::JVMTI_ERROR_INTERRUPT => { JvmtiError::JvmtiErrorInterrupt }
            jvmtiError::JVMTI_ERROR_INVALID_CLASS_FORMAT => { JvmtiError::JvmtiErrorInvalidClassFormat }
            jvmtiError::JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION => { JvmtiError::JvmtiErrorCircularClassDefinition }
            jvmtiError::JVMTI_ERROR_FAILS_VERIFICATION => { JvmtiError::JvmtiErrorFailsVerification }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED => { JvmtiError::JvmtiErrorUnsupportedRedefinitionMethodAdded }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED => { JvmtiError::JvmtiErrorUnsupportedRedefinitionSchemaChanged }
            jvmtiError::JVMTI_ERROR_INVALID_TYPESTATE => { JvmtiError::JvmtiErrorInvalidTypestate }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED => { JvmtiError::JvmtiErrorUnsupportedRedefinitionHierarchyChanged }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED => { JvmtiError::JvmtiErrorUnsupportedRedefinitionMethodDeleted }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_VERSION => { JvmtiError::JvmtiErrorUnsupportedVersion }
            jvmtiError::JVMTI_ERROR_NAMES_DONT_MATCH => { JvmtiError::JvmtiErrorNamesDontMatch }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED => { JvmtiError::JvmtiErrorUnsupportedRedefinitionClassModifiersChanged }
            jvmtiError::JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED => { JvmtiError::JvmtiErrorUnsupportedRedefinitionMethodModifiersChanged }
            jvmtiError::JVMTI_ERROR_UNMODIFIABLE_CLASS => { JvmtiError::JvmtiErrorUnmodifiableClass }
            jvmtiError::JVMTI_ERROR_NOT_AVAILABLE => { JvmtiError::JvmtiErrorNotAvailable }
            jvmtiError::JVMTI_ERROR_MUST_POSSESS_CAPABILITY => { JvmtiError::JvmtiErrorMustPossessCapability }
            jvmtiError::JVMTI_ERROR_NULL_POINTER => { JvmtiError::JvmtiErrorNullPointer }
            jvmtiError::JVMTI_ERROR_ABSENT_INFORMATION => { JvmtiError::JvmtiErrorAbsentInformation }
            jvmtiError::JVMTI_ERROR_INVALID_EVENT_TYPE => { JvmtiError::JvmtiErrorInvalidEventType }
            jvmtiError::JVMTI_ERROR_ILLEGAL_ARGUMENT => { JvmtiError::JvmtiErrorIllegalArgument }
            jvmtiError::JVMTI_ERROR_NATIVE_METHOD => { JvmtiError::JvmtiErrorNativeMethod }
            jvmtiError::JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED => { JvmtiError::JvmtiErrorClassLoaderUnsupported }
            jvmtiError::JVMTI_ERROR_OUT_OF_MEMORY => { JvmtiError::JvmtiErrorOutOfMemory }
            jvmtiError::JVMTI_ERROR_ACCESS_DENIED => { JvmtiError::JvmtiErrorAccessDenied }
            jvmtiError::JVMTI_ERROR_WRONG_PHASE => { JvmtiError::JvmtiErrorWrongPhase }
            jvmtiError::JVMTI_ERROR_INTERNAL => { JvmtiError::JvmtiErrorInternal }
            jvmtiError::JVMTI_ERROR_UNATTACHED_THREAD => { JvmtiError::JvmtiErrorUnattachedThread }
            jvmtiError::JVMTI_ERROR_INVALID_ENVIRONMENT => { JvmtiError::JvmtiErrorInvalidEnvironment }
        }
    }
}