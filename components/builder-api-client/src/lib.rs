// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate habitat_http_client as hab_http;
#[macro_use]
extern crate hyper;
extern crate hyper_openssl;
#[macro_use]
extern crate log;

pub mod error;
pub use error::{Error, Result};

use std::path::Path;

use hab_http::ApiClient;
use hyper::client::{Body, IntoUrl, RequestBuilder};
use hyper::header::{Authorization, Bearer};

pub struct Client(ApiClient);

impl Client {
    pub fn new<U>(
        api_url: U,
        product: &str,
        version: &str,
        fs_root_path: Option<&Path>,
    ) -> Result<Self>
    where
        U: IntoUrl,
    {
        Ok(Client(
            ApiClient::new(api_url, product, version, fs_root_path)
                .map_err(Error::HabitatHttpClient)?,
        ))
    }

    /// Create a job.
    ///
    /// # Failures
    ///
    /// * Remote API Server is not available
    ///
    /// # Panics
    ///
    /// * Authorization token was not set on client
    pub fn create_job(&self, project_name: &str, token: &str) -> Result<()> {
        Ok(())
    }

    fn add_authz<'a>(&'a self, rb: RequestBuilder<'a>, token: &str) -> RequestBuilder {
        rb.header(Authorization(Bearer { token: token.to_string() }))
    }
}
