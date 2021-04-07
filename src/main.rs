#[deny(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    unconditional_recursion,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Env Config Structs 
mod config {
	pub use ::config::ConfigError;
	use serde::Deserialize;

	#[derive(Deserialize)]
	pub struct Config {
		pub server_addr: String,
		pub pg: deadpool_postgres::Config,
	}

	impl Config {
		pub fn from_env() -> Result<Self, ConfigError> {
			let mut cfg = ::config::Config::new();
			cfg.merge(::config::Environment::new())?;
			cfg.try_into()
		}
	}
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Structs
mod models {
    #[derive(debug, serialize, deserialize)]
    struct packet {
        x_deg: i64,
        y_deg: i64,
        dist: i64,
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// Wrappers for complicated OCV functions
mod cv_wrapper {
    use opencv::{
        prelude::*,
        videoio,
        highgui,
        imgproc,
        Result,
    };

    //TODO: Make this real...
    pub fn hsv_process() -> i32 {
        2 + 2
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
/// MAIN
