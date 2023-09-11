use std::fmt::{Debug, Display};

pub trait Error<E>: Debug + Display + From<E>
where
    E: std::error::Error + 'static,
{
}
