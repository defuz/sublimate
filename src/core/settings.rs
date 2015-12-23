use std::marker::PhantomData;
use std::fs::File;
use std::io::{Read, BufReader};
use std::iter::Peekable;

pub use rustc_serialize::json::Json as Settings;
pub use rustc_serialize::json::Array as SettingsArray;
pub use rustc_serialize::json::Object as SettingsObject;

use plist::{PlistEvent, ParserError};
use plist::xml::StreamingParser;

pub trait FromSettings : Sized {
    fn from_settings(settings: Settings) -> Self;
}

pub trait ParseSettings : Sized {
    type Error;
    fn parse_settings(settings: Settings) -> Result<Self, Self::Error>;
}

pub trait FromPlist : Sized {
    fn from_plist(plist: &mut Plist) -> Self;
}

pub trait ParsePlist : Sized {
    type Error;
    fn parse_plist(plist: &mut Plist) -> Result<Self, Self::Error>;
}

pub enum PlistError {
    ExpectString,
    ExpectArray,
    ExpectDictionary,
    ExpectDocumentStart,
    ExpectDocumentEnd,
    Parse(ParserError)
}

impl From<ParserError> for PlistError {
    fn from(error: ParserError) -> PlistError {
        PlistError::Parse(error)
    }
}

pub struct Plist {
    parser: Peekable<StreamingParser<BufReader<File>>>
}

impl Plist {
    pub fn new(file: BufReader<File>) -> Plist {
        Plist { parser: StreamingParser::new(file).peekable() }
    }

    fn next(&mut self) -> Result<PlistEvent, PlistError> {
        match self.parser.next() {
            Some(r) => Ok(try!(r)),
            None => Err(PlistError::Parse(ParserError::UnexpectedEof))
        }
    }

    pub fn parse_string(&mut self) -> Result<String, PlistError> {
        match try!(self.next()) {
            PlistEvent::StringValue(s) => Ok(s),
            _ => Err(PlistError::ExpectString)
        }
    }

    pub fn parse_array_start(&mut self) -> Result<(), PlistError> {
        match try!(self.next()) {
            PlistEvent::StartArray(..) => Ok(()),
            _ => Err(PlistError::ExpectArray)
        }
    }

    pub fn parse_array_continue(&mut self) -> Result<bool, PlistError> {
        Ok(match self.parser.peek() {
            Some(&Ok(PlistEvent::EndArray)) => {
                self.next();
                false
            },
            _ => true
        })
    }

    pub fn parse_dict_start(&mut self) -> Result<(), PlistError> {
        match try!(self.next()) {
            PlistEvent::StartDictionary(..) => Ok(()),
            _ => Err(PlistError::ExpectDictionary)
        }
    }

    pub fn parse_dict_continue(&mut self) -> Result<bool, PlistError> {
        Ok(match self.parser.peek() {
            Some(&Ok(PlistEvent::EndDictionary)) => {
                self.next();
                false
            },
            _ => true
        })
    }

    pub fn parse_document_start(&mut self) -> Result<(), PlistError> {
        match try!(self.next()) {
            PlistEvent::StartPlist => Ok(()),
            _ => Err(PlistError::ExpectDocumentStart)
        }
    }

    pub fn parse_document_end(&mut self) -> Result<(), PlistError> {
        match try!(self.next()) {
            PlistEvent::EndPlist => {
                if let None = self.parser.next() {
                    Ok(())
                } else {
                    Err(PlistError::Parse(ParserError::InvalidData))
                }

            },
            _ => Err(PlistError::ExpectDocumentEnd)
        }
    }
}
