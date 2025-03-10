#[cfg(feature = "rcgen-certs")]
use rcgen::RcgenError;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[cfg(feature = "rcgen-certs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rcgen-certs")))]
    #[error("invalid CA")]
    Tls(#[from] RcgenError),
    #[error("network error")]
    Network(#[from] hyper::Error),
    #[error("unable to decode body")]
    Decode,
    #[error("unknown error")]
    Unknown,
}
