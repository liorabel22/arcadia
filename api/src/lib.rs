use std::ops::Deref;
use crate::env::Env;

pub mod env;
pub mod api_doc;
pub mod handlers;
pub mod periodic_tasks;
pub mod routes;
pub mod services;
pub mod tracker;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum OpenSignups {
    Disabled,
    Enabled,
}

impl From<bool> for OpenSignups {
    fn from(value: bool) -> Self {
        if value {
            OpenSignups::Enabled
        } else {
            OpenSignups::Disabled
        }
    }
}

#[derive(Clone)]
pub struct Arcadia {
    pub pool: sqlx::PgPool,
    env: Env,
}

impl Deref for Arcadia {
    type Target = Env;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

impl Arcadia {
    pub fn new(pool: sqlx::PgPool, env: Env) -> Self {
        Self {pool, env}
    }
    #[inline]
    pub fn is_open_signups(&self) -> bool {
        Into::<OpenSignups>::into(self.env.open_signups) == OpenSignups::Enabled
    }
}
