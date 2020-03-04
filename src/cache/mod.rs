use hyper::Body;
use hyper::Request;
use hyper::Response;

pub mod error;
pub struct MemoryCache {}

impl Cache for MemoryCache {
    fn put(&self, _key: Request<Body>, _res: Response<Body>) -> Result<u32, error::CacheError> {
        return Err(error::CacheError::new("testing"));
    }

    fn get(&self, _key: Request<Body>) -> Result<Response<Body>, error::CacheError> {
        return Err(error::CacheError::new("testing"));
    }
}

pub trait Cache {
    fn put(&self, key: Request<Body>, res: Response<Body>) -> Result<u32, error::CacheError>;
    fn get(&self, key: Request<Body>) -> Result<Response<Body>, error::CacheError>;
}
