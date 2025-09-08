//! Build script for `GraphBit` Node.js bindings
//!
//! This build script sets up the necessary configuration for building
//! Node.js native addons using napi-rs.

extern crate napi_build;

fn main() {
    napi_build::setup();
}
