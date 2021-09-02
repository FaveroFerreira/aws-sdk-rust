// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    BadRequestException(crate::error::BadRequestException),
    ConflictException(crate::error::ConflictException),
    InternalFailureException(crate::error::InternalFailureException),
    LimitExceededException(crate::error::LimitExceededException),
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalFailureException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::StartMedicalStreamTranscriptionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartMedicalStreamTranscriptionError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartMedicalStreamTranscriptionErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
                crate::error::StartMedicalStreamTranscriptionErrorKind::BadRequestException(inner) => Error::BadRequestException(inner),
                crate::error::StartMedicalStreamTranscriptionErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
                crate::error::StartMedicalStreamTranscriptionErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::StartMedicalStreamTranscriptionErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::StartMedicalStreamTranscriptionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<smithy_http::result::SdkError<crate::error::StartStreamTranscriptionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: smithy_http::result::SdkError<crate::error::StartStreamTranscriptionError, R>,
    ) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::StartStreamTranscriptionErrorKind::ServiceUnavailableException(
                    inner,
                ) => Error::ServiceUnavailableException(inner),
                crate::error::StartStreamTranscriptionErrorKind::BadRequestException(inner) => {
                    Error::BadRequestException(inner)
                }
                crate::error::StartStreamTranscriptionErrorKind::InternalFailureException(
                    inner,
                ) => Error::InternalFailureException(inner),
                crate::error::StartStreamTranscriptionErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::StartStreamTranscriptionErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::StartStreamTranscriptionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}