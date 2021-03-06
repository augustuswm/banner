use error::BannerError;
use flag::{Flag, FlagPath};
use store::ThreadedStore;
use user::User;

pub type FlagStore = ThreadedStore<FlagPath, Flag, Error = BannerError>;
pub type PathStore = ThreadedStore<String, FlagPath, Error = BannerError>;
pub type UserStore = ThreadedStore<String, User, Error = BannerError>;

pub struct AppState {
    flag_store: Box<FlagStore>,
    path_store: Box<PathStore>,
    user_store: Box<UserStore>,
}

impl AppState {
    pub fn new<F, P, U>(flag_store: F, path_store: P, user_store: U) -> AppState
    where
        F: ThreadedStore<FlagPath, Flag, Error = BannerError> + 'static,
        P: ThreadedStore<String, FlagPath, Error = BannerError> + 'static,
        U: ThreadedStore<String, User, Error = BannerError> + 'static,
    {
        AppState {
            flag_store: Box::new(flag_store),
            path_store: Box::new(path_store),
            user_store: Box::new(user_store),
        }
    }

    pub fn flags(&self) -> &Box<ThreadedStore<FlagPath, Flag, Error = BannerError>> {
        &self.flag_store
    }

    pub fn paths(&self) -> &Box<ThreadedStore<String, FlagPath, Error = BannerError>> {
        &self.path_store
    }

    pub fn users(&self) -> &Box<ThreadedStore<String, User, Error = BannerError>> {
        &self.user_store
    }
}
