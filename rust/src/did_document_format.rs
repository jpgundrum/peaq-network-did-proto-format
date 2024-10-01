// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `did_document_format.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:document.VerificationMethod)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct VerificationMethod {
    // message fields
    // @@protoc_insertion_point(field:document.VerificationMethod.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:document.VerificationMethod.type)
    pub type_: ::protobuf::EnumOrUnknown<VerificationType>,
    // @@protoc_insertion_point(field:document.VerificationMethod.controller)
    pub controller: ::std::string::String,
    // @@protoc_insertion_point(field:document.VerificationMethod.publicKeyMultibase)
    pub publicKeyMultibase: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:document.VerificationMethod.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a VerificationMethod {
    fn default() -> &'a VerificationMethod {
        <VerificationMethod as ::protobuf::Message>::default_instance()
    }
}

impl VerificationMethod {
    pub fn new() -> VerificationMethod {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &VerificationMethod| { &m.id },
            |m: &mut VerificationMethod| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &VerificationMethod| { &m.type_ },
            |m: &mut VerificationMethod| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "controller",
            |m: &VerificationMethod| { &m.controller },
            |m: &mut VerificationMethod| { &mut m.controller },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "publicKeyMultibase",
            |m: &VerificationMethod| { &m.publicKeyMultibase },
            |m: &mut VerificationMethod| { &mut m.publicKeyMultibase },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<VerificationMethod>(
            "VerificationMethod",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for VerificationMethod {
    const NAME: &'static str = "VerificationMethod";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.id = is.read_string()?;
                },
                16 => {
                    self.type_ = is.read_enum_or_unknown()?;
                },
                26 => {
                    self.controller = is.read_string()?;
                },
                34 => {
                    self.publicKeyMultibase = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(VerificationType::Ed25519VerificationKey2020) {
            my_size += ::protobuf::rt::int32_size(2, self.type_.value());
        }
        if !self.controller.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.controller);
        }
        if !self.publicKeyMultibase.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.publicKeyMultibase);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(VerificationType::Ed25519VerificationKey2020) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        if !self.controller.is_empty() {
            os.write_string(3, &self.controller)?;
        }
        if !self.publicKeyMultibase.is_empty() {
            os.write_string(4, &self.publicKeyMultibase)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> VerificationMethod {
        VerificationMethod::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.type_ = ::protobuf::EnumOrUnknown::new(VerificationType::Ed25519VerificationKey2020);
        self.controller.clear();
        self.publicKeyMultibase.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static VerificationMethod {
        static instance: VerificationMethod = VerificationMethod {
            id: ::std::string::String::new(),
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            controller: ::std::string::String::new(),
            publicKeyMultibase: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for VerificationMethod {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("VerificationMethod").unwrap()).clone()
    }
}

impl ::std::fmt::Display for VerificationMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerificationMethod {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:document.Signature)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Signature {
    // message fields
    // @@protoc_insertion_point(field:document.Signature.type)
    pub type_: ::protobuf::EnumOrUnknown<VerificationType>,
    // @@protoc_insertion_point(field:document.Signature.issuer)
    pub issuer: ::std::string::String,
    // @@protoc_insertion_point(field:document.Signature.hash)
    pub hash: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:document.Signature.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Signature {
    fn default() -> &'a Signature {
        <Signature as ::protobuf::Message>::default_instance()
    }
}

impl Signature {
    pub fn new() -> Signature {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Signature| { &m.type_ },
            |m: &mut Signature| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "issuer",
            |m: &Signature| { &m.issuer },
            |m: &mut Signature| { &mut m.issuer },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hash",
            |m: &Signature| { &m.hash },
            |m: &mut Signature| { &mut m.hash },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Signature>(
            "Signature",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Signature {
    const NAME: &'static str = "Signature";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.type_ = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.issuer = is.read_string()?;
                },
                26 => {
                    self.hash = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.type_ != ::protobuf::EnumOrUnknown::new(VerificationType::Ed25519VerificationKey2020) {
            my_size += ::protobuf::rt::int32_size(1, self.type_.value());
        }
        if !self.issuer.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.issuer);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.type_ != ::protobuf::EnumOrUnknown::new(VerificationType::Ed25519VerificationKey2020) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        if !self.issuer.is_empty() {
            os.write_string(2, &self.issuer)?;
        }
        if !self.hash.is_empty() {
            os.write_string(3, &self.hash)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Signature {
        Signature::new()
    }

    fn clear(&mut self) {
        self.type_ = ::protobuf::EnumOrUnknown::new(VerificationType::Ed25519VerificationKey2020);
        self.issuer.clear();
        self.hash.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Signature {
        static instance: Signature = Signature {
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            issuer: ::std::string::String::new(),
            hash: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Signature {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Signature").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Signature {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:document.Service)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Service {
    // message fields
    // @@protoc_insertion_point(field:document.Service.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:document.Service.type)
    pub type_: ::std::string::String,
    // @@protoc_insertion_point(field:document.Service.serviceEndpoint)
    pub serviceEndpoint: ::std::string::String,
    // @@protoc_insertion_point(field:document.Service.data)
    pub data: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:document.Service.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Service {
    fn default() -> &'a Service {
        <Service as ::protobuf::Message>::default_instance()
    }
}

impl Service {
    pub fn new() -> Service {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Service| { &m.id },
            |m: &mut Service| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Service| { &m.type_ },
            |m: &mut Service| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "serviceEndpoint",
            |m: &Service| { &m.serviceEndpoint },
            |m: &mut Service| { &mut m.serviceEndpoint },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &Service| { &m.data },
            |m: &mut Service| { &mut m.data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Service>(
            "Service",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Service {
    const NAME: &'static str = "Service";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.id = is.read_string()?;
                },
                18 => {
                    self.type_ = is.read_string()?;
                },
                26 => {
                    self.serviceEndpoint = is.read_string()?;
                },
                34 => {
                    self.data = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.type_.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.type_);
        }
        if !self.serviceEndpoint.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.serviceEndpoint);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.type_.is_empty() {
            os.write_string(2, &self.type_)?;
        }
        if !self.serviceEndpoint.is_empty() {
            os.write_string(3, &self.serviceEndpoint)?;
        }
        if !self.data.is_empty() {
            os.write_string(4, &self.data)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Service {
        Service::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.type_.clear();
        self.serviceEndpoint.clear();
        self.data.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Service {
        static instance: Service = Service {
            id: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            serviceEndpoint: ::std::string::String::new(),
            data: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Service {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Service").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Service {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Service {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:document.Document)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Document {
    // message fields
    // @@protoc_insertion_point(field:document.Document.id)
    pub id: ::std::string::String,
    // @@protoc_insertion_point(field:document.Document.controller)
    pub controller: ::std::string::String,
    // @@protoc_insertion_point(field:document.Document.verificationMethods)
    pub verificationMethods: ::std::vec::Vec<VerificationMethod>,
    // @@protoc_insertion_point(field:document.Document.signature)
    pub signature: ::protobuf::MessageField<Signature>,
    // @@protoc_insertion_point(field:document.Document.services)
    pub services: ::std::vec::Vec<Service>,
    // @@protoc_insertion_point(field:document.Document.authentications)
    pub authentications: ::std::vec::Vec<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:document.Document.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Document {
    fn default() -> &'a Document {
        <Document as ::protobuf::Message>::default_instance()
    }
}

impl Document {
    pub fn new() -> Document {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Document| { &m.id },
            |m: &mut Document| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "controller",
            |m: &Document| { &m.controller },
            |m: &mut Document| { &mut m.controller },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "verificationMethods",
            |m: &Document| { &m.verificationMethods },
            |m: &mut Document| { &mut m.verificationMethods },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Signature>(
            "signature",
            |m: &Document| { &m.signature },
            |m: &mut Document| { &mut m.signature },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "services",
            |m: &Document| { &m.services },
            |m: &mut Document| { &mut m.services },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "authentications",
            |m: &Document| { &m.authentications },
            |m: &mut Document| { &mut m.authentications },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Document>(
            "Document",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Document {
    const NAME: &'static str = "Document";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.id = is.read_string()?;
                },
                18 => {
                    self.controller = is.read_string()?;
                },
                26 => {
                    self.verificationMethods.push(is.read_message()?);
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.signature)?;
                },
                42 => {
                    self.services.push(is.read_message()?);
                },
                50 => {
                    self.authentications.push(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.controller.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.controller);
        }
        for value in &self.verificationMethods {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.signature.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.services {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.authentications {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.controller.is_empty() {
            os.write_string(2, &self.controller)?;
        }
        for v in &self.verificationMethods {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if let Some(v) = self.signature.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        for v in &self.services {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        for v in &self.authentications {
            os.write_string(6, &v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Document {
        Document::new()
    }

    fn clear(&mut self) {
        self.id.clear();
        self.controller.clear();
        self.verificationMethods.clear();
        self.signature.clear();
        self.services.clear();
        self.authentications.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Document {
        static instance: Document = Document {
            id: ::std::string::String::new(),
            controller: ::std::string::String::new(),
            verificationMethods: ::std::vec::Vec::new(),
            signature: ::protobuf::MessageField::none(),
            services: ::std::vec::Vec::new(),
            authentications: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Document {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Document").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Document {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Document {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:document.VerificationType)
pub enum VerificationType {
    // @@protoc_insertion_point(enum_value:document.VerificationType.Ed25519VerificationKey2020)
    Ed25519VerificationKey2020 = 0,
    // @@protoc_insertion_point(enum_value:document.VerificationType.Sr25519VerificationKey2020)
    Sr25519VerificationKey2020 = 1,
}

impl ::protobuf::Enum for VerificationType {
    const NAME: &'static str = "VerificationType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<VerificationType> {
        match value {
            0 => ::std::option::Option::Some(VerificationType::Ed25519VerificationKey2020),
            1 => ::std::option::Option::Some(VerificationType::Sr25519VerificationKey2020),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<VerificationType> {
        match str {
            "Ed25519VerificationKey2020" => ::std::option::Option::Some(VerificationType::Ed25519VerificationKey2020),
            "Sr25519VerificationKey2020" => ::std::option::Option::Some(VerificationType::Sr25519VerificationKey2020),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [VerificationType] = &[
        VerificationType::Ed25519VerificationKey2020,
        VerificationType::Sr25519VerificationKey2020,
    ];
}

impl ::protobuf::EnumFull for VerificationType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("VerificationType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for VerificationType {
    fn default() -> Self {
        VerificationType::Ed25519VerificationKey2020
    }
}

impl VerificationType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<VerificationType>("VerificationType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19did_document_format.proto\x12\x08document\"\xa4\x01\n\x12Verificat\
    ionMethod\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02id\x12.\n\x04type\x18\
    \x02\x20\x01(\x0e2\x1a.document.VerificationTypeR\x04type\x12\x1e\n\ncon\
    troller\x18\x03\x20\x01(\tR\ncontroller\x12.\n\x12publicKeyMultibase\x18\
    \x04\x20\x01(\tR\x12publicKeyMultibase\"g\n\tSignature\x12.\n\x04type\
    \x18\x01\x20\x01(\x0e2\x1a.document.VerificationTypeR\x04type\x12\x16\n\
    \x06issuer\x18\x02\x20\x01(\tR\x06issuer\x12\x12\n\x04hash\x18\x03\x20\
    \x01(\tR\x04hash\"k\n\x07Service\x12\x0e\n\x02id\x18\x01\x20\x01(\tR\x02\
    id\x12\x12\n\x04type\x18\x02\x20\x01(\tR\x04type\x12(\n\x0fserviceEndpoi\
    nt\x18\x03\x20\x01(\tR\x0fserviceEndpoint\x12\x12\n\x04data\x18\x04\x20\
    \x01(\tR\x04data\"\x96\x02\n\x08Document\x12\x0e\n\x02id\x18\x01\x20\x01\
    (\tR\x02id\x12\x1e\n\ncontroller\x18\x02\x20\x01(\tR\ncontroller\x12N\n\
    \x13verificationMethods\x18\x03\x20\x03(\x0b2\x1c.document.VerificationM\
    ethodR\x13verificationMethods\x121\n\tsignature\x18\x04\x20\x01(\x0b2\
    \x13.document.SignatureR\tsignature\x12-\n\x08services\x18\x05\x20\x03(\
    \x0b2\x11.document.ServiceR\x08services\x12(\n\x0fauthentications\x18\
    \x06\x20\x03(\tR\x0fauthentications*R\n\x10VerificationType\x12\x1e\n\
    \x1aEd25519VerificationKey2020\x10\0\x12\x1e\n\x1aSr25519VerificationKey\
    2020\x10\x01BOZMgithub.com/peaqnetwork/peaq-network-did-proto-format/gol\
    ang/document;documentJ\xcc\t\n\x06\x12\x04\0\0%\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\0\x11\n\x08\n\x01\x08\x12\x03\
    \x03\0d\n\t\n\x02\x08\x0b\x12\x03\x03\0d\n\n\n\x02\x05\0\x12\x04\x05\0\
    \x08\x01\n\n\n\x03\x05\0\x01\x12\x03\x05\x05\x15\n\x0b\n\x04\x05\0\x02\0\
    \x12\x03\x06\x02!\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x06\x02\x1c\n\x0c\
    \n\x05\x05\0\x02\0\x02\x12\x03\x06\x1f\x20\n\x0b\n\x04\x05\0\x02\x01\x12\
    \x03\x07\x02!\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x07\x02\x1c\n\x0c\n\
    \x05\x05\0\x02\x01\x02\x12\x03\x07\x1f\x20\n\n\n\x02\x04\0\x12\x04\n\0\
    \x0f\x01\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x1a\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\x0b\x02\x10\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x0b\x02\x08\n\
    \x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0b\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x0b\x0e\x0f\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x02\x1c\n\x0c\
    \n\x05\x04\0\x02\x01\x06\x12\x03\x0c\x02\x12\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\x0c\x13\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0c\x1a\
    \x1b\n\x0b\n\x04\x04\0\x02\x02\x12\x03\r\x02\x18\n\x0c\n\x05\x04\0\x02\
    \x02\x05\x12\x03\r\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\r\t\x13\
    \n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\r\x16\x17\n\x0b\n\x04\x04\0\x02\
    \x03\x12\x03\x0e\x02\x20\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x0e\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0e\t\x1b\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03\x0e\x1e\x1f\n\n\n\x02\x04\x01\x12\x04\x11\0\x15\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x11\x08\x11\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x03\x12\x02\x1c\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x12\x02\x12\n\x0c\
    \n\x05\x04\x01\x02\0\x01\x12\x03\x12\x13\x17\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03\x12\x1a\x1b\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x13\x02\x14\
    \n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\x13\x02\x08\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x03\x13\t\x0f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\
    \x13\x12\x13\n\x0b\n\x04\x04\x01\x02\x02\x12\x03\x14\x02\x12\n\x0c\n\x05\
    \x04\x01\x02\x02\x05\x12\x03\x14\x02\x08\n\x0c\n\x05\x04\x01\x02\x02\x01\
    \x12\x03\x14\t\r\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x14\x10\x11\n\n\
    \n\x02\x04\x02\x12\x04\x17\0\x1c\x01\n\n\n\x03\x04\x02\x01\x12\x03\x17\
    \x08\x0f\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x18\x02\x10\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03\x18\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \x18\t\x0b\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x18\x0e\x0f\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03\x19\x02\x12\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\
    \x03\x19\x02\x08\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x19\t\r\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03\x19\x10\x11\n\x0b\n\x04\x04\x02\x02\x02\
    \x12\x03\x1a\x02\x1d\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\x1a\x02\x08\
    \n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x1a\t\x18\n\x0c\n\x05\x04\x02\
    \x02\x02\x03\x12\x03\x1a\x1b\x1c\n\x0b\n\x04\x04\x02\x02\x03\x12\x03\x1b\
    \x02\x12\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\x1b\x02\x08\n\x0c\n\x05\
    \x04\x02\x02\x03\x01\x12\x03\x1b\t\r\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\
    \x03\x1b\x10\x11\n\n\n\x02\x04\x03\x12\x04\x1e\0%\x01\n\n\n\x03\x04\x03\
    \x01\x12\x03\x1e\x08\x10\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x1f\x02\x10\n\
    \x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x1f\x02\x08\n\x0c\n\x05\x04\x03\x02\
    \0\x01\x12\x03\x1f\t\x0b\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x1f\x0e\
    \x0f\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\x20\x02\x18\n\x0c\n\x05\x04\x03\
    \x02\x01\x05\x12\x03\x20\x02\x08\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\
    \x20\t\x13\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x20\x16\x17\n\x0b\n\
    \x04\x04\x03\x02\x02\x12\x03!\x026\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\
    \x03!\x02\n\n\x0c\n\x05\x04\x03\x02\x02\x06\x12\x03!\x0b\x1d\n\x0c\n\x05\
    \x04\x03\x02\x02\x01\x12\x03!\x1e1\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\
    \x03!45\n\x0b\n\x04\x04\x03\x02\x03\x12\x03\"\x02\x1a\n\x0c\n\x05\x04\
    \x03\x02\x03\x06\x12\x03\"\x02\x0b\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\
    \x03\"\x0c\x15\n\x0c\n\x05\x04\x03\x02\x03\x03\x12\x03\"\x18\x19\n\x0b\n\
    \x04\x04\x03\x02\x04\x12\x03#\x02\x20\n\x0c\n\x05\x04\x03\x02\x04\x04\
    \x12\x03#\x02\n\n\x0c\n\x05\x04\x03\x02\x04\x06\x12\x03#\x0b\x12\n\x0c\n\
    \x05\x04\x03\x02\x04\x01\x12\x03#\x13\x1b\n\x0c\n\x05\x04\x03\x02\x04\
    \x03\x12\x03#\x1e\x1f\n\x0b\n\x04\x04\x03\x02\x05\x12\x03$\x02&\n\x0c\n\
    \x05\x04\x03\x02\x05\x04\x12\x03$\x02\n\n\x0c\n\x05\x04\x03\x02\x05\x05\
    \x12\x03$\x0b\x11\n\x0c\n\x05\x04\x03\x02\x05\x01\x12\x03$\x12!\n\x0c\n\
    \x05\x04\x03\x02\x05\x03\x12\x03$$%b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(VerificationMethod::generated_message_descriptor_data());
            messages.push(Signature::generated_message_descriptor_data());
            messages.push(Service::generated_message_descriptor_data());
            messages.push(Document::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(VerificationType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
