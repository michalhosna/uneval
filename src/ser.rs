use crate::error::CoronaError;
use serde::ser;
use std::io::Write;

type SerResult = Result<(), CoronaError>;

pub struct Corona<W: Write> {
    writer: W,
    inside: bool,
}

impl<W: Write> Corona<W> {
    fn start_sub(&mut self) -> &mut Self {
        self.inside = true;
        self
    }

    fn serialize_item(&mut self, item: impl ser::Serialize) -> SerResult {
        if self.inside {
            write!(self.writer, ",")?;
        }
        self.inside = true;
        item.serialize(self)?;
        Ok(())
    }
}

impl<W: Write> ser::Serializer for &mut Corona<W> {
    type Ok = ();
    type Error = CoronaError;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> SerResult {
        write!(self.writer, "{}", v.to_string())?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> SerResult {
        write!(self.writer, "'{}'", v)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> SerResult {
        write!(self.writer, "\"{}\"", v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> SerResult {
        self.collect_seq(v)?;
        Ok(())
    }

    fn serialize_none(self) -> SerResult {
        write!(self.writer, "None")?;
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> SerResult
    where
        T: serde::Serialize,
    {
        write!(self.writer, "Some(")?;
        value.serialize(&mut *self)?;
        write!(self.writer, ")")?;
        Ok(())
    }

    fn serialize_unit(self) -> SerResult {
        write!(self.writer, "()")?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> SerResult {
        write!(self.writer, "{}()", name)?;
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> SerResult {
        write!(self.writer, "{}::{}()", name, variant)?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> SerResult
    where
        T: serde::Serialize,
    {
        write!(self.writer, "{}(", name)?;
        value.serialize(&mut *self)?;
        write!(self.writer, ")")?;
        Ok(())
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> SerResult
    where
        T: serde::Serialize,
    {
        write!(self.writer, "{}::{}(", name, variant)?;
        value.serialize(&mut *self)?;
        write!(self.writer, ")")?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        write!(self.writer, "vec![")?;
        Ok(self.start_sub())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        write!(self.writer, "(")?;
        Ok(self.start_sub())
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        write!(self.writer, "{}(", name)?;
        Ok(self.start_sub())
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        write!(self.writer, "{}::{}", name, variant)?;
        Ok(self.start_sub())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        write!(self.writer, "vec![")?;
        Ok(self.start_sub())
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        write!(self.writer, "{} {{", name)?;
        Ok(self.start_sub())
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        write!(self.writer, "{}::{} {{", name, variant)?;
        Ok(self.start_sub())
    }
}

impl<W: Write> ser::SerializeSeq for &mut Corona<W> {
    type Ok = ();
    type Error = CoronaError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> SerResult {
        write!(self.writer, "].into_iter().collect()")?;
        Ok(())
    }
}
impl<W: Write> ser::SerializeTuple for &mut Corona<W> {
    type Ok = ();
    type Error = CoronaError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> SerResult {
        write!(self.writer, ")")?;
        Ok(())
    }
}
impl<W: Write> ser::SerializeTupleStruct for &mut Corona<W> {
    type Ok = ();
    type Error = CoronaError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> SerResult {
        write!(self.writer, ")")?;
        Ok(())
    }
}
impl<W: Write> ser::SerializeTupleVariant for &mut Corona<W> {
    type Ok = ();
    type Error = CoronaError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> SerResult {
        write!(self.writer, ")")?;
        Ok(())
    }
}
impl<W: Write> ser::SerializeMap for &mut Corona<W> {
    type Ok = ();
    type Error = CoronaError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        // a little hack for code reuse
        self.serialize_item("(")?;
        key.serialize(&mut **self)?;
        write!(self.writer, ",")?;
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)?;
        write!(self.writer, ")")?;
        Ok(())
    }

    fn end(self) -> SerResult {
        write!(self.writer, "].into_iter().collect()")?;
        Ok(())
    }
}
impl<W: Write> ser::SerializeStruct for &mut Corona<W> {
    type Ok = ();
    type Error = CoronaError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serialize_item(format!("{}:", key))?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> SerResult {
        write!(self.writer, "}}")?;
        Ok(())
    }
}
impl<W: Write> ser::SerializeStructVariant for &mut Corona<W> {
    type Ok = ();
    type Error = CoronaError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.serialize_item(format!("{}:", key))?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> SerResult {
        write!(self.writer, "}}")?;
        Ok(())
    }
}