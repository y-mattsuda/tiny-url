use std::marker::PhantomData;

use derive_new::new;

use crate::persistence::mysql::Db;

pub mod health_check;
pub mod url;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _marker: PhantomData<T>,
}
