use async_trait::async_trait;
use std::{error, pin::Pin};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
pub type AsyncTraitReturn<'async_trait, T> = Pin<
    Box<
        (dyn std::future::Future<
            Output = (dyn std::future::Future<
                Output = std::result::Result<
                    T,
                    Box<(dyn std::error::Error + 'static)>,
                >,
            > + 'static),
        > + Send
             + 'async_trait),
    >,
>;
