use std::{io::{BufReader, BufWriter, Read, Write}};

use serde::{Serialize, de::{DeserializeOwned}};

use crate::error::FcResult;

pub struct JsonStream {}

impl JsonStream {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait FiniteStreamHandler {
    fn read_all<Target>(self: &mut Self, source: &mut (dyn Read))
    -> FcResult<Target>
    where Target: DeserializeOwned ;
    fn write_all<Source>(self: &mut Self,
        writer: &mut (dyn Write),
        source: &Source)
    -> FcResult<()>
    where Source: ?Sized + Serialize;
}

impl FiniteStreamHandler for JsonStream {
    fn read_all<Target>(self: &mut Self, reader: &mut (dyn Read))
    -> FcResult<Target>
    where Target: DeserializeOwned {
        Ok(serde_json::from_reader(
            BufReader::new(reader)
        )?)
    }

    fn write_all<Source>(self: &mut Self,
        writer: &mut (dyn Write),
        source: &Source)
    -> FcResult<()>
    where Source: ?Sized + Serialize {
        Ok(serde_json::to_writer_pretty(
            BufWriter::new(writer),
            source
        )?)
    }
}