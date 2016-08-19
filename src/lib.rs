// For pest parser generation
#![recursion_limit = "1024"]

extern crate ansi_term;
extern crate duk;
#[macro_use]
extern crate error_chain;
extern crate glob;
#[macro_use]
extern crate log;
extern crate ordered_float;
#[macro_use]
extern crate pest;
extern crate protobuf;
extern crate rmp;
extern crate serde;
extern crate serde_cbor;
extern crate serde_hjson;
extern crate serde_json;
extern crate serde_protobuf;
extern crate serde_yaml;
extern crate xdg_basedir;
extern crate yaml_rust;

pub mod config;
pub mod error;
pub mod proto_index;
pub mod query;
pub mod value;

pub fn run_query<I, O>(query: &query::Query, source: I, mut sink: O) -> error::Result<()>
    where I: value::Source,
          O: value::Sink
{
    let query_context = query::Context::new();

    let mut results = try!(query.evaluate(&query_context, source));

    while let Some(result) = try!(value::Source::read(&mut results)) {
        try!(sink.write(result));
    }

    Ok(())
}
