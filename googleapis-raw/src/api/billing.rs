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
//! Generated file from `google/api/billing.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct Billing {
    // message fields
    pub consumer_destinations: ::protobuf::RepeatedField<Billing_BillingDestination>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Billing {
    fn default() -> &'a Billing {
        <Billing as ::protobuf::Message>::default_instance()
    }
}

impl Billing {
    pub fn new() -> Billing {
        ::std::default::Default::default()
    }

    // repeated .google.api.Billing.BillingDestination consumer_destinations = 8;


    pub fn get_consumer_destinations(&self) -> &[Billing_BillingDestination] {
        &self.consumer_destinations
    }
    pub fn clear_consumer_destinations(&mut self) {
        self.consumer_destinations.clear();
    }

    // Param is passed by value, moved
    pub fn set_consumer_destinations(&mut self, v: ::protobuf::RepeatedField<Billing_BillingDestination>) {
        self.consumer_destinations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_consumer_destinations(&mut self) -> &mut ::protobuf::RepeatedField<Billing_BillingDestination> {
        &mut self.consumer_destinations
    }

    // Take field
    pub fn take_consumer_destinations(&mut self) -> ::protobuf::RepeatedField<Billing_BillingDestination> {
        ::std::mem::replace(&mut self.consumer_destinations, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Billing {
    fn is_initialized(&self) -> bool {
        for v in &self.consumer_destinations {
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
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.consumer_destinations)?;
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
        for value in &self.consumer_destinations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.consumer_destinations {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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

    fn new() -> Billing {
        Billing::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Billing_BillingDestination>>(
                "consumer_destinations",
                |m: &Billing| { &m.consumer_destinations },
                |m: &mut Billing| { &mut m.consumer_destinations },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Billing>(
                "Billing",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Billing {
        static instance: ::protobuf::rt::LazyV2<Billing> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Billing::new)
    }
}

impl ::protobuf::Clear for Billing {
    fn clear(&mut self) {
        self.consumer_destinations.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Billing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Billing {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Billing_BillingDestination {
    // message fields
    pub monitored_resource: ::std::string::String,
    pub metrics: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Billing_BillingDestination {
    fn default() -> &'a Billing_BillingDestination {
        <Billing_BillingDestination as ::protobuf::Message>::default_instance()
    }
}

impl Billing_BillingDestination {
    pub fn new() -> Billing_BillingDestination {
        ::std::default::Default::default()
    }

    // string monitored_resource = 1;


    pub fn get_monitored_resource(&self) -> &str {
        &self.monitored_resource
    }
    pub fn clear_monitored_resource(&mut self) {
        self.monitored_resource.clear();
    }

    // Param is passed by value, moved
    pub fn set_monitored_resource(&mut self, v: ::std::string::String) {
        self.monitored_resource = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_monitored_resource(&mut self) -> &mut ::std::string::String {
        &mut self.monitored_resource
    }

    // Take field
    pub fn take_monitored_resource(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.monitored_resource, ::std::string::String::new())
    }

    // repeated string metrics = 2;


    pub fn get_metrics(&self) -> &[::std::string::String] {
        &self.metrics
    }
    pub fn clear_metrics(&mut self) {
        self.metrics.clear();
    }

    // Param is passed by value, moved
    pub fn set_metrics(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.metrics = v;
    }

    // Mutable pointer to the field.
    pub fn mut_metrics(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.metrics
    }

    // Take field
    pub fn take_metrics(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.metrics, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Billing_BillingDestination {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.monitored_resource)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.metrics)?;
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
        if !self.monitored_resource.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.monitored_resource);
        }
        for value in &self.metrics {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.monitored_resource.is_empty() {
            os.write_string(1, &self.monitored_resource)?;
        }
        for v in &self.metrics {
            os.write_string(2, &v)?;
        };
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

    fn new() -> Billing_BillingDestination {
        Billing_BillingDestination::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "monitored_resource",
                |m: &Billing_BillingDestination| { &m.monitored_resource },
                |m: &mut Billing_BillingDestination| { &mut m.monitored_resource },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "metrics",
                |m: &Billing_BillingDestination| { &m.metrics },
                |m: &mut Billing_BillingDestination| { &mut m.metrics },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Billing_BillingDestination>(
                "Billing.BillingDestination",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Billing_BillingDestination {
        static instance: ::protobuf::rt::LazyV2<Billing_BillingDestination> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Billing_BillingDestination::new)
    }
}

impl ::protobuf::Clear for Billing_BillingDestination {
    fn clear(&mut self) {
        self.monitored_resource.clear();
        self.metrics.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Billing_BillingDestination {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Billing_BillingDestination {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18google/api/billing.proto\x12\ngoogle.api\x1a\x17google/api/metric.\
    proto\"\xc5\x01\n\x07Billing\x12[\n\x15consumer_destinations\x18\x08\x20\
    \x03(\x0b2&.google.api.Billing.BillingDestinationR\x14consumerDestinatio\
    ns\x1a]\n\x12BillingDestination\x12-\n\x12monitored_resource\x18\x01\x20\
    \x01(\tR\x11monitoredResource\x12\x18\n\x07metrics\x18\x02\x20\x03(\tR\
    \x07metricsBn\n\x0ecom.google.apiB\x0cBillingProtoP\x01ZEgoogle.golang.o\
    rg/genproto/googleapis/api/serviceconfig;serviceconfig\xa2\x02\x04GAPIJ\
    \xff\x11\n\x06\x12\x04\x0f\0B\x01\n\xbe\x04\n\x01\x0c\x12\x03\x0f\0\x122\
    \xb3\x04\x20Copyright\x202019\x20Google\x20LLC.\n\n\x20Licensed\x20under\
    \x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\");\
    \n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compli\
    ance\x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\
    \x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.\
    org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\
    \x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distrib\
    uted\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\
    \x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\
    \x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\
    \x20License\x20for\x20the\x20specific\x20language\x20governing\x20permis\
    sions\x20and\n\x20limitations\x20under\x20the\x20License.\n\n\n\x08\n\
    \x01\x02\x12\x03\x11\0\x13\n\t\n\x02\x03\0\x12\x03\x13\0!\n\x08\n\x01\
    \x08\x12\x03\x15\0\\\n\t\n\x02\x08\x0b\x12\x03\x15\0\\\n\x08\n\x01\x08\
    \x12\x03\x16\0\"\n\t\n\x02\x08\n\x12\x03\x16\0\"\n\x08\n\x01\x08\x12\x03\
    \x17\0-\n\t\n\x02\x08\x08\x12\x03\x17\0-\n\x08\n\x01\x08\x12\x03\x18\0'\
    \n\t\n\x02\x08\x01\x12\x03\x18\0'\n\x08\n\x01\x08\x12\x03\x19\0\"\n\t\n\
    \x02\x08$\x12\x03\x19\0\"\n\xb3\x05\n\x02\x04\0\x12\x040\0B\x01\x1a\xa6\
    \x05\x20Billing\x20related\x20configuration\x20of\x20the\x20service.\n\n\
    \x20The\x20following\x20example\x20shows\x20how\x20to\x20configure\x20mo\
    nitored\x20resources\x20and\x20metrics\n\x20for\x20billing:\n\n\x20\x20\
    \x20\x20\x20monitored_resources:\n\x20\x20\x20\x20\x20-\x20type:\x20libr\
    ary.googleapis.com/branch\n\x20\x20\x20\x20\x20\x20\x20labels:\n\x20\x20\
    \x20\x20\x20\x20\x20-\x20key:\x20/city\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20description:\x20The\x20city\x20where\x20the\x20library\x20branch\x20\
    is\x20located\x20in.\n\x20\x20\x20\x20\x20\x20\x20-\x20key:\x20/name\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20description:\x20The\x20name\x20of\
    \x20the\x20branch.\n\x20\x20\x20\x20\x20metrics:\n\x20\x20\x20\x20\x20-\
    \x20name:\x20library.googleapis.com/book/borrowed_count\n\x20\x20\x20\
    \x20\x20\x20\x20metric_kind:\x20DELTA\n\x20\x20\x20\x20\x20\x20\x20value\
    _type:\x20INT64\n\x20\x20\x20\x20\x20billing:\n\x20\x20\x20\x20\x20\x20\
    \x20consumer_destinations:\n\x20\x20\x20\x20\x20\x20\x20-\x20monitored_r\
    esource:\x20library.googleapis.com/branch\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20metrics:\n\x20\x20\x20\x20\x20\x20\x20\x20\x20-\x20library.googl\
    eapis.com/book/borrowed_count\n\n\n\n\x03\x04\0\x01\x12\x030\x08\x0f\nx\
    \n\x04\x04\0\x03\0\x12\x043\x02;\x03\x1aj\x20Configuration\x20of\x20a\
    \x20specific\x20billing\x20destination\x20(Currently\x20only\x20support\
    \n\x20bill\x20against\x20consumer\x20project).\n\n\x0c\n\x05\x04\0\x03\0\
    \x01\x12\x033\n\x1c\n\x9a\x01\n\x06\x04\0\x03\0\x02\0\x12\x036\x04\"\x1a\
    \x8a\x01\x20The\x20monitored\x20resource\x20type.\x20The\x20type\x20must\
    \x20be\x20defined\x20in\n\x20[Service.monitored_resources][google.api.Se\
    rvice.monitored_resources]\x20section.\n\n\x0e\n\x07\x04\0\x03\0\x02\0\
    \x05\x12\x036\x04\n\n\x0e\n\x07\x04\0\x03\0\x02\0\x01\x12\x036\x0b\x1d\n\
    \x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x036\x20!\n\xa2\x01\n\x06\x04\0\x03\
    \0\x02\x01\x12\x03:\x04\x20\x1a\x92\x01\x20Names\x20of\x20the\x20metrics\
    \x20to\x20report\x20to\x20this\x20billing\x20destination.\n\x20Each\x20n\
    ame\x20must\x20be\x20defined\x20in\x20[Service.metrics][google.api.Servi\
    ce.metrics]\x20section.\n\n\x0e\n\x07\x04\0\x03\0\x02\x01\x04\x12\x03:\
    \x04\x0c\n\x0e\n\x07\x04\0\x03\0\x02\x01\x05\x12\x03:\r\x13\n\x0e\n\x07\
    \x04\0\x03\0\x02\x01\x01\x12\x03:\x14\x1b\n\x0e\n\x07\x04\0\x03\0\x02\
    \x01\x03\x12\x03:\x1e\x1f\n\x81\x02\n\x04\x04\0\x02\0\x12\x03A\x028\x1a\
    \xf3\x01\x20Billing\x20configurations\x20for\x20sending\x20metrics\x20to\
    \x20the\x20consumer\x20project.\n\x20There\x20can\x20be\x20multiple\x20c\
    onsumer\x20destinations\x20per\x20service,\x20each\x20one\x20must\x20hav\
    e\n\x20a\x20different\x20monitored\x20resource\x20type.\x20A\x20metric\
    \x20can\x20be\x20used\x20in\x20at\x20most\n\x20one\x20consumer\x20destin\
    ation.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03A\x02\n\n\x0c\n\x05\x04\0\
    \x02\0\x06\x12\x03A\x0b\x1d\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03A\x1e3\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03A67b\x06proto3\
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
