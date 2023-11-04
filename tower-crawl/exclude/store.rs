//! Cache for the retrieved and parsed [`Robots`]`.txt` files.

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use http::{StatusCode, Uri};
use robotxt::url::Url;
use robotxt::{AccessResult, Robots, ALL_UAS};

/// Status of the [`Robots`] for the requested [`Uri`].
#[derive(Debug, Clone)]
pub(crate) enum Status {
    /// The requested [`Uri`] is not valid for the retrieval e.g.
    /// relative address or unexpected protocol.
    Unknown,
    /// The [`Robots`] location is locked, the future is expected to
    /// retrieve the `robots.txt` file or to cancel the lock.
    Locking(Uri),
    /// The [`Robots`] location is locked, the future is expected to
    /// wait until the `robots.txt` file is retrieved.
    Waiting,
    /// The [`Robots`] is parsed and ready to be used.
    Ready(Robots),
}

#[derive(Debug)]
struct StoreInner {
    pub user_agent: String,
    pub cache: Mutex<HashMap<String, Status>>,
}

/// Cache for the retrieved and parsed [`Robots`]`.txt` files.
#[derive(Debug, Clone)]
pub struct Store {
    inner: Arc<StoreInner>,
}

impl Store {
    /// Creates a new [`Store`] with the given `User-Agent`.
    pub fn new(user_agent: &str) -> Self {
        let inner = Arc::new(StoreInner {
            user_agent: user_agent.to_string(),
            cache: Mutex::new(HashMap::new()),
        });

        Self { inner }
    }

    /// Creates a new [`Store`] with the [`ALL_UAS`] as a `User-Agent`.
    pub fn global() -> Self {
        static STORE: OnceLock<Store> = OnceLock::new();
        STORE.get_or_init(|| Store::new(ALL_UAS)).clone()
    }

    /// Returns the applied `User-Agent`.
    pub fn user_agent(&self) -> &str {
        self.inner.user_agent.as_str()
    }

    /// Attempts to retrieve the [`Robots`] file, locks the [`Status`] otherwise.
    pub(crate) fn access(&self, addr: &Uri) -> Status {
        let host = match addr.host() {
            None => return Status::Unknown,
            Some(host) => host,
        };

        let mut cache = self.inner.cache.lock().expect("should not be held");
        let status = match cache.entry(host.to_owned()) {
            Entry::Vacant(x) => match Self::create_uri(addr) {
                Some(robots) => x.insert(Status::Locking(robots)).clone(),
                None => Status::Unknown,
            },
            Entry::Occupied(mut x) => match x.get() {
                Status::Locking { .. } => x.insert(Status::Waiting).clone(),
                status => status.clone(),
            },
        };

        status
    }

    /// Parses the given `buf` into the [`Robots`] as per specification.
    ///
    /// See [`AccessResult`].
    pub(crate) fn store(&self, addr: &Uri, code: StatusCode, buf: Option<&[u8]>) -> Option<Robots> {
        let host = addr.host()?;

        let access = match code {
            x if x.is_informational() || x.is_success() => {
                AccessResult::Successful(buf.unwrap_or_default())
            }

            // Assumes, that the client has followed at least five consecutive redirects
            // as per specification and failed to retrieve the `robots.txt` file.
            x if x.is_redirection() => AccessResult::Redirect,
            x if x.is_client_error() => AccessResult::Unavailable,
            // Google makes an exception for a `TOO_MANY_REQUESTS` status code.
            x if x.is_server_error() || x == StatusCode::TOO_MANY_REQUESTS => {
                AccessResult::Unreachable
            }

            // Treat any protocol error as an error in the 400-499 range.
            _ => AccessResult::Unavailable,
        };

        let robots = Robots::from_access(access, self.user_agent());
        let mut cache = self.inner.cache.lock().expect("should not be held");
        let status = cache.get_mut(host).expect("should be present and locked");
        *status = Status::Ready(robots.clone());

        Some(robots)
    }

    /// Unlocks the host if the currently locked.
    ///
    /// # Panics
    ///
    /// Expects the current status to be [`Status::Locking`], panics otherwise.
    pub(crate) fn cancel(&self, path: &Uri) {
        let host = path.host().expect("should be a valid address");
        let mut cache = self.inner.cache.lock().expect("should not be held");
        let status = cache.remove(host).expect("should be present and locked");
        debug_assert!(matches!(status, Status::Locking(_)));
    }

    /// Returns the expected path to the `robots.txt` file as the [`http::Uri`].
    ///
    /// # Note
    ///
    /// Returns `None` if the [`Uri`] is not an absolute `http`/`https` address.
    /// See [`robotxt::create_url`].
    pub(crate) fn create_uri(path: &Uri) -> Option<Uri> {
        // TODO: How do I remove username/password with re-parsing?
        let url = Url::parse(&path.to_string()).ok();
        url.and_then(|url| robotxt::create_url(&url).ok())
            .and_then(|url| url.to_string().parse::<Uri>().ok())
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::global()
    }
}
