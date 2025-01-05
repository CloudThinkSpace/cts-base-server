use std::future::Future;
use crate::request::CtsFile;
use anyhow::Result;

pub mod local_multipart;
pub mod oss_multipart;


pub trait CtsOssParse {
    fn parse(&mut self) -> impl Future<Output=Result<Vec<CtsFile>>>;
}

pub trait CtsLocalParse {
    fn parse(&mut self) -> impl Future<Output=Result<Vec<CtsFile>>>;
}