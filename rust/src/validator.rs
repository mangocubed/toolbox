use validator::ValidationErrors;

pub type ValidationResult<T = ()> = Result<T, ValidationErrors>;

pub trait OrValidationErrors<T> {
    fn or_validation_errors(self) -> ValidationResult<T>;
}

impl<T> OrValidationErrors<T> for Option<T> {
    fn or_validation_errors(self) -> ValidationResult<T> {
        self.ok_or_else(Default::default)
    }
}

impl<T, E> OrValidationErrors<T> for Result<T, E> {
    fn or_validation_errors(self) -> ValidationResult<T> {
        self.map_err(|_| Default::default())
    }
}
