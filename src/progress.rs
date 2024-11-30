use std::{
    collections::HashSet,
};

#[derive(Default)]
pub struct RuntimeErrors {
    pub no_permissions: HashSet<String>,
    pub file_not_found: HashSet<String>,
    pub unknown_error: HashSet<String>,
    pub abort: bool,
}