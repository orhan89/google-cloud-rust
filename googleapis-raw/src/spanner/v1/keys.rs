// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/spanner/v1/keys.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct KeyRange {
    // message oneof groups
    pub start_key_type: ::std::option::Option<KeyRange_oneof_start_key_type>,
    pub end_key_type: ::std::option::Option<KeyRange_oneof_end_key_type>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a KeyRange {
    fn default() -> &'a KeyRange {
        <KeyRange as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum KeyRange_oneof_start_key_type {
    start_closed(::protobuf::well_known_types::ListValue),
    start_open(::protobuf::well_known_types::ListValue),
}

#[derive(Clone,PartialEq,Debug)]
pub enum KeyRange_oneof_end_key_type {
    end_closed(::protobuf::well_known_types::ListValue),
    end_open(::protobuf::well_known_types::ListValue),
}

impl KeyRange {
    pub fn new() -> KeyRange {
        ::std::default::Default::default()
    }

    // .google.protobuf.ListValue start_closed = 1;


    pub fn get_start_closed(&self) -> &::protobuf::well_known_types::ListValue {
        match self.start_key_type {
            ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_closed(ref v)) => v,
            _ => <::protobuf::well_known_types::ListValue as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_start_closed(&mut self) {
        self.start_key_type = ::std::option::Option::None;
    }

    pub fn has_start_closed(&self) -> bool {
        match self.start_key_type {
            ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_closed(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_start_closed(&mut self, v: ::protobuf::well_known_types::ListValue) {
        self.start_key_type = ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_closed(v))
    }

    // Mutable pointer to the field.
    pub fn mut_start_closed(&mut self) -> &mut ::protobuf::well_known_types::ListValue {
        if let ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_closed(_)) = self.start_key_type {
        } else {
            self.start_key_type = ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_closed(::protobuf::well_known_types::ListValue::new()));
        }
        match self.start_key_type {
            ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_closed(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_start_closed(&mut self) -> ::protobuf::well_known_types::ListValue {
        if self.has_start_closed() {
            match self.start_key_type.take() {
                ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_closed(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::ListValue::new()
        }
    }

    // .google.protobuf.ListValue start_open = 2;


    pub fn get_start_open(&self) -> &::protobuf::well_known_types::ListValue {
        match self.start_key_type {
            ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_open(ref v)) => v,
            _ => <::protobuf::well_known_types::ListValue as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_start_open(&mut self) {
        self.start_key_type = ::std::option::Option::None;
    }

    pub fn has_start_open(&self) -> bool {
        match self.start_key_type {
            ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_open(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_start_open(&mut self, v: ::protobuf::well_known_types::ListValue) {
        self.start_key_type = ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_open(v))
    }

    // Mutable pointer to the field.
    pub fn mut_start_open(&mut self) -> &mut ::protobuf::well_known_types::ListValue {
        if let ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_open(_)) = self.start_key_type {
        } else {
            self.start_key_type = ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_open(::protobuf::well_known_types::ListValue::new()));
        }
        match self.start_key_type {
            ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_open(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_start_open(&mut self) -> ::protobuf::well_known_types::ListValue {
        if self.has_start_open() {
            match self.start_key_type.take() {
                ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_open(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::ListValue::new()
        }
    }

    // .google.protobuf.ListValue end_closed = 3;


    pub fn get_end_closed(&self) -> &::protobuf::well_known_types::ListValue {
        match self.end_key_type {
            ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_closed(ref v)) => v,
            _ => <::protobuf::well_known_types::ListValue as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_end_closed(&mut self) {
        self.end_key_type = ::std::option::Option::None;
    }

    pub fn has_end_closed(&self) -> bool {
        match self.end_key_type {
            ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_closed(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_closed(&mut self, v: ::protobuf::well_known_types::ListValue) {
        self.end_key_type = ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_closed(v))
    }

    // Mutable pointer to the field.
    pub fn mut_end_closed(&mut self) -> &mut ::protobuf::well_known_types::ListValue {
        if let ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_closed(_)) = self.end_key_type {
        } else {
            self.end_key_type = ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_closed(::protobuf::well_known_types::ListValue::new()));
        }
        match self.end_key_type {
            ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_closed(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_closed(&mut self) -> ::protobuf::well_known_types::ListValue {
        if self.has_end_closed() {
            match self.end_key_type.take() {
                ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_closed(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::ListValue::new()
        }
    }

    // .google.protobuf.ListValue end_open = 4;


    pub fn get_end_open(&self) -> &::protobuf::well_known_types::ListValue {
        match self.end_key_type {
            ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_open(ref v)) => v,
            _ => <::protobuf::well_known_types::ListValue as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_end_open(&mut self) {
        self.end_key_type = ::std::option::Option::None;
    }

    pub fn has_end_open(&self) -> bool {
        match self.end_key_type {
            ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_open(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_open(&mut self, v: ::protobuf::well_known_types::ListValue) {
        self.end_key_type = ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_open(v))
    }

    // Mutable pointer to the field.
    pub fn mut_end_open(&mut self) -> &mut ::protobuf::well_known_types::ListValue {
        if let ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_open(_)) = self.end_key_type {
        } else {
            self.end_key_type = ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_open(::protobuf::well_known_types::ListValue::new()));
        }
        match self.end_key_type {
            ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_open(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_open(&mut self) -> ::protobuf::well_known_types::ListValue {
        if self.has_end_open() {
            match self.end_key_type.take() {
                ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_open(v)) => v,
                _ => panic!(),
            }
        } else {
            ::protobuf::well_known_types::ListValue::new()
        }
    }
}

impl ::protobuf::Message for KeyRange {
    fn is_initialized(&self) -> bool {
        if let Some(KeyRange_oneof_start_key_type::start_closed(ref v)) = self.start_key_type {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(KeyRange_oneof_start_key_type::start_open(ref v)) = self.start_key_type {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(KeyRange_oneof_end_key_type::end_closed(ref v)) = self.end_key_type {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(KeyRange_oneof_end_key_type::end_open(ref v)) = self.end_key_type {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.start_key_type = ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_closed(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.start_key_type = ::std::option::Option::Some(KeyRange_oneof_start_key_type::start_open(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.end_key_type = ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_closed(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.end_key_type = ::std::option::Option::Some(KeyRange_oneof_end_key_type::end_open(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.start_key_type {
            match v {
                &KeyRange_oneof_start_key_type::start_closed(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &KeyRange_oneof_start_key_type::start_open(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.end_key_type {
            match v {
                &KeyRange_oneof_end_key_type::end_closed(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &KeyRange_oneof_end_key_type::end_open(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.start_key_type {
            match v {
                &KeyRange_oneof_start_key_type::start_closed(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &KeyRange_oneof_start_key_type::start_open(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        if let ::std::option::Option::Some(ref v) = self.end_key_type {
            match v {
                &KeyRange_oneof_end_key_type::end_closed(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &KeyRange_oneof_end_key_type::end_open(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> KeyRange {
        KeyRange::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::protobuf::well_known_types::ListValue>(
                "start_closed",
                KeyRange::has_start_closed,
                KeyRange::get_start_closed,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::protobuf::well_known_types::ListValue>(
                "start_open",
                KeyRange::has_start_open,
                KeyRange::get_start_open,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::protobuf::well_known_types::ListValue>(
                "end_closed",
                KeyRange::has_end_closed,
                KeyRange::get_end_closed,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ::protobuf::well_known_types::ListValue>(
                "end_open",
                KeyRange::has_end_open,
                KeyRange::get_end_open,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<KeyRange>(
                "KeyRange",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static KeyRange {
        static instance: ::protobuf::rt::LazyV2<KeyRange> = ::protobuf::rt::LazyV2::INIT;
        instance.get(KeyRange::new)
    }
}

impl ::protobuf::Clear for KeyRange {
    fn clear(&mut self) {
        self.start_key_type = ::std::option::Option::None;
        self.start_key_type = ::std::option::Option::None;
        self.end_key_type = ::std::option::Option::None;
        self.end_key_type = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyRange {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KeySet {
    // message fields
    pub keys: ::protobuf::RepeatedField<::protobuf::well_known_types::ListValue>,
    pub ranges: ::protobuf::RepeatedField<KeyRange>,
    pub all: bool,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a KeySet {
    fn default() -> &'a KeySet {
        <KeySet as ::protobuf::Message>::default_instance()
    }
}

impl KeySet {
    pub fn new() -> KeySet {
        ::std::default::Default::default()
    }

    // repeated .google.protobuf.ListValue keys = 1;


    pub fn get_keys(&self) -> &[::protobuf::well_known_types::ListValue] {
        &self.keys
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::protobuf::well_known_types::ListValue>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::protobuf::well_known_types::ListValue> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::protobuf::well_known_types::ListValue> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    // repeated .google.spanner.v1.KeyRange ranges = 2;


    pub fn get_ranges(&self) -> &[KeyRange] {
        &self.ranges
    }
    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::protobuf::RepeatedField<KeyRange>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges(&mut self) -> &mut ::protobuf::RepeatedField<KeyRange> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::protobuf::RepeatedField<KeyRange> {
        ::std::mem::replace(&mut self.ranges, ::protobuf::RepeatedField::new())
    }

    // bool all = 3;


    pub fn get_all(&self) -> bool {
        self.all
    }
    pub fn clear_all(&mut self) {
        self.all = false;
    }

    // Param is passed by value, moved
    pub fn set_all(&mut self, v: bool) {
        self.all = v;
    }
}

impl ::protobuf::Message for KeySet {
    fn is_initialized(&self) -> bool {
        for v in &self.keys {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ranges {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ranges)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.all = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.ranges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.all != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.ranges {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.all != false {
            os.write_bool(3, self.all)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> KeySet {
        KeySet::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::ListValue>>(
                "keys",
                |m: &KeySet| { &m.keys },
                |m: &mut KeySet| { &mut m.keys },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyRange>>(
                "ranges",
                |m: &KeySet| { &m.ranges },
                |m: &mut KeySet| { &mut m.ranges },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "all",
                |m: &KeySet| { &m.all },
                |m: &mut KeySet| { &mut m.all },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<KeySet>(
                "KeySet",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static KeySet {
        static instance: ::protobuf::rt::LazyV2<KeySet> = ::protobuf::rt::LazyV2::INIT;
        instance.get(KeySet::new)
    }
}

impl ::protobuf::Clear for KeySet {
    fn clear(&mut self) {
        self.keys.clear();
        self.ranges.clear();
        self.all = false;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeySet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeySet {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cgoogle/spanner/v1/keys.proto\x12\x11google.spanner.v1\x1a\x1cgoogl\
    e/protobuf/struct.proto\x1a\x1cgoogle/api/annotations.proto\"\xa0\x02\n\
    \x08KeyRange\x12?\n\x0cstart_closed\x18\x01\x20\x01(\x0b2\x1a.google.pro\
    tobuf.ListValueH\0R\x0bstartClosed\x12;\n\nstart_open\x18\x02\x20\x01(\
    \x0b2\x1a.google.protobuf.ListValueH\0R\tstartOpen\x12;\n\nend_closed\
    \x18\x03\x20\x01(\x0b2\x1a.google.protobuf.ListValueH\x01R\tendClosed\
    \x127\n\x08end_open\x18\x04\x20\x01(\x0b2\x1a.google.protobuf.ListValueH\
    \x01R\x07endOpenB\x10\n\x0estart_key_typeB\x0e\n\x0cend_key_type\"\x7f\n\
    \x06KeySet\x12.\n\x04keys\x18\x01\x20\x03(\x0b2\x1a.google.protobuf.List\
    ValueR\x04keys\x123\n\x06ranges\x18\x02\x20\x03(\x0b2\x1b.google.spanner\
    .v1.KeyRangeR\x06ranges\x12\x10\n\x03all\x18\x03\x20\x01(\x08R\x03allB\
    \x92\x01\n\x15com.google.spanner.v1B\tKeysProtoP\x01Z8google.golang.org/\
    genproto/googleapis/spanner/v1;spanner\xaa\x02\x17Google.Cloud.Spanner.V\
    1\xca\x02\x17Google\\Cloud\\Spanner\\V1J\xf4+\n\x07\x12\x05\x0f\0\xa3\
    \x01\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\xb3\x04\x20Copyright\x20\
    2019\x20Google\x20LLC.\n\n\x20Licensed\x20under\x20the\x20Apache\x20Lice\
    nse,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\
    \x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20\
    License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\
    \x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\
    \n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\
    \x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\n\x08\n\x01\x02\x12\x03\x11\0\
    \x1a\n\t\n\x02\x03\0\x12\x03\x13\0&\n\t\n\x02\x03\x01\x12\x03\x14\0&\n\
    \x08\n\x01\x08\x12\x03\x16\04\n\t\n\x02\x08%\x12\x03\x16\04\n\x08\n\x01\
    \x08\x12\x03\x17\0O\n\t\n\x02\x08\x0b\x12\x03\x17\0O\n\x08\n\x01\x08\x12\
    \x03\x18\0\"\n\t\n\x02\x08\n\x12\x03\x18\0\"\n\x08\n\x01\x08\x12\x03\x19\
    \0*\n\t\n\x02\x08\x08\x12\x03\x19\0*\n\x08\n\x01\x08\x12\x03\x1a\0.\n\t\
    \n\x02\x08\x01\x12\x03\x1a\0.\n\x08\n\x01\x08\x12\x03\x1b\04\n\t\n\x02\
    \x08)\x12\x03\x1b\04\n\xb5\x15\n\x02\x04\0\x12\x05u\0\x8b\x01\x01\x1a\
    \xa7\x15\x20KeyRange\x20represents\x20a\x20range\x20of\x20rows\x20in\x20\
    a\x20table\x20or\x20index.\n\n\x20A\x20range\x20has\x20a\x20start\x20key\
    \x20and\x20an\x20end\x20key.\x20These\x20keys\x20can\x20be\x20open\x20or\
    \n\x20closed,\x20indicating\x20if\x20the\x20range\x20includes\x20rows\
    \x20with\x20that\x20key.\n\n\x20Keys\x20are\x20represented\x20by\x20list\
    s,\x20where\x20the\x20ith\x20value\x20in\x20the\x20list\n\x20corresponds\
    \x20to\x20the\x20ith\x20component\x20of\x20the\x20table\x20or\x20index\
    \x20primary\x20key.\n\x20Individual\x20values\x20are\x20encoded\x20as\
    \x20described\n\x20[here][google.spanner.v1.TypeCode].\n\n\x20For\x20exa\
    mple,\x20consider\x20the\x20following\x20table\x20definition:\n\n\x20\
    \x20\x20\x20\x20CREATE\x20TABLE\x20UserEvents\x20(\n\x20\x20\x20\x20\x20\
    \x20\x20UserName\x20STRING(MAX),\n\x20\x20\x20\x20\x20\x20\x20EventDate\
    \x20STRING(10)\n\x20\x20\x20\x20\x20)\x20PRIMARY\x20KEY(UserName,\x20Eve\
    ntDate);\n\n\x20The\x20following\x20keys\x20name\x20rows\x20in\x20this\
    \x20table:\n\n\x20\x20\x20\x20\x20[\"Bob\",\x20\"2014-09-23\"]\n\x20\x20\
    \x20\x20\x20[\"Alfred\",\x20\"2015-06-12\"]\n\n\x20Since\x20the\x20`User\
    Events`\x20table's\x20`PRIMARY\x20KEY`\x20clause\x20names\x20two\n\x20co\
    lumns,\x20each\x20`UserEvents`\x20key\x20has\x20two\x20elements;\x20the\
    \x20first\x20is\x20the\n\x20`UserName`,\x20and\x20the\x20second\x20is\
    \x20the\x20`EventDate`.\n\n\x20Key\x20ranges\x20with\x20multiple\x20comp\
    onents\x20are\x20interpreted\n\x20lexicographically\x20by\x20component\
    \x20using\x20the\x20table\x20or\x20index\x20key's\x20declared\n\x20sort\
    \x20order.\x20For\x20example,\x20the\x20following\x20range\x20returns\
    \x20all\x20events\x20for\n\x20user\x20`\"Bob\"`\x20that\x20occurred\x20i\
    n\x20the\x20year\x202015:\n\n\x20\x20\x20\x20\x20\"start_closed\":\x20[\
    \"Bob\",\x20\"2015-01-01\"]\n\x20\x20\x20\x20\x20\"end_closed\":\x20[\"B\
    ob\",\x20\"2015-12-31\"]\n\n\x20Start\x20and\x20end\x20keys\x20can\x20om\
    it\x20trailing\x20key\x20components.\x20This\x20affects\x20the\n\x20incl\
    usion\x20and\x20exclusion\x20of\x20rows\x20that\x20exactly\x20match\x20t\
    he\x20provided\x20key\n\x20components:\x20if\x20the\x20key\x20is\x20clos\
    ed,\x20then\x20rows\x20that\x20exactly\x20match\x20the\n\x20provided\x20\
    components\x20are\x20included;\x20if\x20the\x20key\x20is\x20open,\x20the\
    n\x20rows\n\x20that\x20exactly\x20match\x20are\x20not\x20included.\n\n\
    \x20For\x20example,\x20the\x20following\x20range\x20includes\x20all\x20e\
    vents\x20for\x20`\"Bob\"`\x20that\n\x20occurred\x20during\x20and\x20afte\
    r\x20the\x20year\x202000:\n\n\x20\x20\x20\x20\x20\"start_closed\":\x20[\
    \"Bob\",\x20\"2000-01-01\"]\n\x20\x20\x20\x20\x20\"end_closed\":\x20[\"B\
    ob\"]\n\n\x20The\x20next\x20example\x20retrieves\x20all\x20events\x20for\
    \x20`\"Bob\"`:\n\n\x20\x20\x20\x20\x20\"start_closed\":\x20[\"Bob\"]\n\
    \x20\x20\x20\x20\x20\"end_closed\":\x20[\"Bob\"]\n\n\x20To\x20retrieve\
    \x20events\x20before\x20the\x20year\x202000:\n\n\x20\x20\x20\x20\x20\"st\
    art_closed\":\x20[\"Bob\"]\n\x20\x20\x20\x20\x20\"end_open\":\x20[\"Bob\
    \",\x20\"2000-01-01\"]\n\n\x20The\x20following\x20range\x20includes\x20a\
    ll\x20rows\x20in\x20the\x20table:\n\n\x20\x20\x20\x20\x20\"start_closed\
    \":\x20[]\n\x20\x20\x20\x20\x20\"end_closed\":\x20[]\n\n\x20This\x20rang\
    e\x20returns\x20all\x20users\x20whose\x20`UserName`\x20begins\x20with\
    \x20any\n\x20character\x20from\x20A\x20to\x20C:\n\n\x20\x20\x20\x20\x20\
    \"start_closed\":\x20[\"A\"]\n\x20\x20\x20\x20\x20\"end_open\":\x20[\"D\
    \"]\n\n\x20This\x20range\x20returns\x20all\x20users\x20whose\x20`UserNam\
    e`\x20begins\x20with\x20B:\n\n\x20\x20\x20\x20\x20\"start_closed\":\x20[\
    \"B\"]\n\x20\x20\x20\x20\x20\"end_open\":\x20[\"C\"]\n\n\x20Key\x20range\
    s\x20honor\x20column\x20sort\x20order.\x20For\x20example,\x20suppose\x20\
    a\x20table\x20is\n\x20defined\x20as\x20follows:\n\n\x20\x20\x20\x20\x20C\
    REATE\x20TABLE\x20DescendingSortedTable\x20{\n\x20\x20\x20\x20\x20\x20\
    \x20Key\x20INT64,\n\x20\x20\x20\x20\x20\x20\x20...\n\x20\x20\x20\x20\x20\
    )\x20PRIMARY\x20KEY(Key\x20DESC);\n\n\x20The\x20following\x20range\x20re\
    trieves\x20all\x20rows\x20with\x20key\x20values\x20between\x201\n\x20and\
    \x20100\x20inclusive:\n\n\x20\x20\x20\x20\x20\"start_closed\":\x20[\"100\
    \"]\n\x20\x20\x20\x20\x20\"end_closed\":\x20[\"1\"]\n\n\x20Note\x20that\
    \x20100\x20is\x20passed\x20as\x20the\x20start,\x20and\x201\x20is\x20pass\
    ed\x20as\x20the\x20end,\n\x20because\x20`Key`\x20is\x20a\x20descending\
    \x20column\x20in\x20the\x20schema.\n\n\n\n\x03\x04\0\x01\x12\x03u\x08\
    \x10\nP\n\x04\x04\0\x08\0\x12\x04w\x02\x7f\x03\x1aB\x20The\x20start\x20k\
    ey\x20must\x20be\x20provided.\x20It\x20can\x20be\x20either\x20closed\x20\
    or\x20open.\n\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03w\x08\x16\n\x93\x01\n\
    \x04\x04\0\x02\0\x12\x03z\x04/\x1a\x85\x01\x20If\x20the\x20start\x20is\
    \x20closed,\x20then\x20the\x20range\x20includes\x20all\x20rows\x20whose\
    \n\x20first\x20`len(start_closed)`\x20key\x20columns\x20exactly\x20match\
    \x20`start_closed`.\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03z\x04\x1d\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03z\x1e*\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\
    z-.\n\x88\x01\n\x04\x04\0\x02\x01\x12\x03~\x04-\x1a{\x20If\x20the\x20sta\
    rt\x20is\x20open,\x20then\x20the\x20range\x20excludes\x20rows\x20whose\
    \x20first\n\x20`len(start_open)`\x20key\x20columns\x20exactly\x20match\
    \x20`start_open`.\n\n\x0c\n\x05\x04\0\x02\x01\x06\x12\x03~\x04\x1d\n\x0c\
    \n\x05\x04\0\x02\x01\x01\x12\x03~\x1e(\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03~+,\nP\n\x04\x04\0\x08\x01\x12\x06\x82\x01\x02\x8a\x01\x03\x1a@\x20T\
    he\x20end\x20key\x20must\x20be\x20provided.\x20It\x20can\x20be\x20either\
    \x20closed\x20or\x20open.\n\n\r\n\x05\x04\0\x08\x01\x01\x12\x04\x82\x01\
    \x08\x14\n\x8d\x01\n\x04\x04\0\x02\x02\x12\x04\x85\x01\x04-\x1a\x7f\x20I\
    f\x20the\x20end\x20is\x20closed,\x20then\x20the\x20range\x20includes\x20\
    all\x20rows\x20whose\n\x20first\x20`len(end_closed)`\x20key\x20columns\
    \x20exactly\x20match\x20`end_closed`.\n\n\r\n\x05\x04\0\x02\x02\x06\x12\
    \x04\x85\x01\x04\x1d\n\r\n\x05\x04\0\x02\x02\x01\x12\x04\x85\x01\x1e(\n\
    \r\n\x05\x04\0\x02\x02\x03\x12\x04\x85\x01+,\n\x83\x01\n\x04\x04\0\x02\
    \x03\x12\x04\x89\x01\x04+\x1au\x20If\x20the\x20end\x20is\x20open,\x20the\
    n\x20the\x20range\x20excludes\x20rows\x20whose\x20first\n\x20`len(end_op\
    en)`\x20key\x20columns\x20exactly\x20match\x20`end_open`.\n\n\r\n\x05\
    \x04\0\x02\x03\x06\x12\x04\x89\x01\x04\x1d\n\r\n\x05\x04\0\x02\x03\x01\
    \x12\x04\x89\x01\x1e&\n\r\n\x05\x04\0\x02\x03\x03\x12\x04\x89\x01)*\n\
    \x86\x03\n\x02\x04\x01\x12\x06\x94\x01\0\xa3\x01\x01\x1a\xf7\x02\x20`Key\
    Set`\x20defines\x20a\x20collection\x20of\x20Cloud\x20Spanner\x20keys\x20\
    and/or\x20key\x20ranges.\x20All\n\x20the\x20keys\x20are\x20expected\x20t\
    o\x20be\x20in\x20the\x20same\x20table\x20or\x20index.\x20The\x20keys\x20\
    need\n\x20not\x20be\x20sorted\x20in\x20any\x20particular\x20way.\n\n\x20\
    If\x20the\x20same\x20key\x20is\x20specified\x20multiple\x20times\x20in\
    \x20the\x20set\x20(for\x20example\n\x20if\x20two\x20ranges,\x20two\x20ke\
    ys,\x20or\x20a\x20key\x20and\x20a\x20range\x20overlap),\x20Cloud\x20Span\
    ner\n\x20behaves\x20as\x20if\x20the\x20key\x20were\x20only\x20specified\
    \x20once.\n\n\x0b\n\x03\x04\x01\x01\x12\x04\x94\x01\x08\x0e\n\x8a\x02\n\
    \x04\x04\x01\x02\0\x12\x04\x99\x01\x02.\x1a\xfb\x01\x20A\x20list\x20of\
    \x20specific\x20keys.\x20Entries\x20in\x20`keys`\x20should\x20have\x20ex\
    actly\x20as\n\x20many\x20elements\x20as\x20there\x20are\x20columns\x20in\
    \x20the\x20primary\x20or\x20index\x20key\n\x20with\x20which\x20this\x20`\
    KeySet`\x20is\x20used.\x20\x20Individual\x20key\x20values\x20are\n\x20en\
    coded\x20as\x20described\x20[here][google.spanner.v1.TypeCode].\n\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04\x99\x01\x02\n\n\r\n\x05\x04\x01\x02\0\x06\
    \x12\x04\x99\x01\x0b$\n\r\n\x05\x04\x01\x02\0\x01\x12\x04\x99\x01%)\n\r\
    \n\x05\x04\x01\x02\0\x03\x12\x04\x99\x01,-\n\x86\x01\n\x04\x04\x01\x02\
    \x01\x12\x04\x9d\x01\x02\x1f\x1ax\x20A\x20list\x20of\x20key\x20ranges.\
    \x20See\x20[KeyRange][google.spanner.v1.KeyRange]\x20for\x20more\x20info\
    rmation\x20about\n\x20key\x20range\x20specifications.\n\n\r\n\x05\x04\
    \x01\x02\x01\x04\x12\x04\x9d\x01\x02\n\n\r\n\x05\x04\x01\x02\x01\x06\x12\
    \x04\x9d\x01\x0b\x13\n\r\n\x05\x04\x01\x02\x01\x01\x12\x04\x9d\x01\x14\
    \x1a\n\r\n\x05\x04\x01\x02\x01\x03\x12\x04\x9d\x01\x1d\x1e\n\xce\x01\n\
    \x04\x04\x01\x02\x02\x12\x04\xa2\x01\x02\x0f\x1a\xbf\x01\x20For\x20conve\
    nience\x20`all`\x20can\x20be\x20set\x20to\x20`true`\x20to\x20indicate\
    \x20that\x20this\n\x20`KeySet`\x20matches\x20all\x20keys\x20in\x20the\
    \x20table\x20or\x20index.\x20Note\x20that\x20any\x20keys\n\x20specified\
    \x20in\x20`keys`\x20or\x20`ranges`\x20are\x20only\x20yielded\x20once.\n\
    \n\r\n\x05\x04\x01\x02\x02\x05\x12\x04\xa2\x01\x02\x06\n\r\n\x05\x04\x01\
    \x02\x02\x01\x12\x04\xa2\x01\x07\n\n\r\n\x05\x04\x01\x02\x02\x03\x12\x04\
    \xa2\x01\r\x0eb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
