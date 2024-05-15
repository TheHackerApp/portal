use async_graphql::Object;

/// Represents and error in the input of a mutation
#[derive(Debug)]
pub struct UserError {
    /// The path to the input field that caused the error
    field: &'static [&'static str],
    /// The error message
    message: String,
}

// Cannot derive SimpleObject due to bug causing lifetime issues with unnecessary referencing
// for `UserError.field`
#[Object(shareable)]
/// Represents and error in the input of a mutation
impl UserError {
    /// The path to the input field that caused the error
    #[inline(always)]
    async fn field(&self) -> &'static [&'static str] {
        self.field
    }

    /// The error message
    #[inline(always)]
    async fn message(&self) -> &String {
        &self.message
    }
}

impl UserError {
    /// Create a new user error
    pub fn new(field: &'static [&'static str], message: impl ToString) -> Self {
        let message = message.to_string();
        Self { field, message }
    }
}

/// Create mutation results with user errors
macro_rules! results {
    (
        $(
            $( #[$outer:meta] )*
            $name:ident {
                $( #[$inner:meta] )*
                $field:ident : $type:ty $(,)?
            }
        )*
    ) => {
        $(
            $( #[$outer] )*
            #[derive(Debug, async_graphql::SimpleObject)]
            struct $name {
                $( #[$inner] )*
                $field: Option<$type>,
                /// Errors that may have occurred while processing the action
                user_errors: Vec<$crate::mutation::UserError>
            }

            impl From<$type> for $name {
                fn from(value: $type) -> Self {
                    Self {
                        $field: Some(value),
                        user_errors: Vec::with_capacity(0),
                    }
                }
            }

            impl From<$crate::mutation::UserError> for $name {
                fn from(user_error: $crate::mutation::UserError) -> Self {
                    Self {
                        $field: None,
                        user_errors: vec![user_error],
                    }
                }
            }

            impl From<Vec<$crate::mutation::UserError>> for $name {
                fn from(user_errors: Vec<$crate::mutation::UserError>) -> Self {
                    Self {
                        $field: None,
                        user_errors,
                    }
                }
            }
        )*
    };
}

pub(crate) use results;
