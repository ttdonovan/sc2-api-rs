// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct RequestQuery {
    // message fields
    pathing: ::protobuf::RepeatedField<RequestQueryPathing>,
    abilities: ::protobuf::RepeatedField<RequestQueryAvailableAbilities>,
    placements: ::protobuf::RepeatedField<RequestQueryBuildingPlacement>,
    ignore_resource_requirements: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestQuery {}

impl RequestQuery {
    pub fn new() -> RequestQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestQuery {
        static mut instance: ::protobuf::lazy::Lazy<RequestQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestQuery,
        };
        unsafe {
            instance.get(RequestQuery::new)
        }
    }

    // repeated .SC2APIProtocol.RequestQueryPathing pathing = 1;

    pub fn clear_pathing(&mut self) {
        self.pathing.clear();
    }

    // Param is passed by value, moved
    pub fn set_pathing(&mut self, v: ::protobuf::RepeatedField<RequestQueryPathing>) {
        self.pathing = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pathing(&mut self) -> &mut ::protobuf::RepeatedField<RequestQueryPathing> {
        &mut self.pathing
    }

    // Take field
    pub fn take_pathing(&mut self) -> ::protobuf::RepeatedField<RequestQueryPathing> {
        ::std::mem::replace(&mut self.pathing, ::protobuf::RepeatedField::new())
    }

    pub fn get_pathing(&self) -> &[RequestQueryPathing] {
        &self.pathing
    }

    fn get_pathing_for_reflect(&self) -> &::protobuf::RepeatedField<RequestQueryPathing> {
        &self.pathing
    }

    fn mut_pathing_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RequestQueryPathing> {
        &mut self.pathing
    }

    // repeated .SC2APIProtocol.RequestQueryAvailableAbilities abilities = 2;

    pub fn clear_abilities(&mut self) {
        self.abilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_abilities(&mut self, v: ::protobuf::RepeatedField<RequestQueryAvailableAbilities>) {
        self.abilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_abilities(&mut self) -> &mut ::protobuf::RepeatedField<RequestQueryAvailableAbilities> {
        &mut self.abilities
    }

    // Take field
    pub fn take_abilities(&mut self) -> ::protobuf::RepeatedField<RequestQueryAvailableAbilities> {
        ::std::mem::replace(&mut self.abilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_abilities(&self) -> &[RequestQueryAvailableAbilities] {
        &self.abilities
    }

    fn get_abilities_for_reflect(&self) -> &::protobuf::RepeatedField<RequestQueryAvailableAbilities> {
        &self.abilities
    }

    fn mut_abilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RequestQueryAvailableAbilities> {
        &mut self.abilities
    }

    // repeated .SC2APIProtocol.RequestQueryBuildingPlacement placements = 3;

    pub fn clear_placements(&mut self) {
        self.placements.clear();
    }

    // Param is passed by value, moved
    pub fn set_placements(&mut self, v: ::protobuf::RepeatedField<RequestQueryBuildingPlacement>) {
        self.placements = v;
    }

    // Mutable pointer to the field.
    pub fn mut_placements(&mut self) -> &mut ::protobuf::RepeatedField<RequestQueryBuildingPlacement> {
        &mut self.placements
    }

    // Take field
    pub fn take_placements(&mut self) -> ::protobuf::RepeatedField<RequestQueryBuildingPlacement> {
        ::std::mem::replace(&mut self.placements, ::protobuf::RepeatedField::new())
    }

    pub fn get_placements(&self) -> &[RequestQueryBuildingPlacement] {
        &self.placements
    }

    fn get_placements_for_reflect(&self) -> &::protobuf::RepeatedField<RequestQueryBuildingPlacement> {
        &self.placements
    }

    fn mut_placements_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RequestQueryBuildingPlacement> {
        &mut self.placements
    }

    // optional bool ignore_resource_requirements = 4;

    pub fn clear_ignore_resource_requirements(&mut self) {
        self.ignore_resource_requirements = ::std::option::Option::None;
    }

    pub fn has_ignore_resource_requirements(&self) -> bool {
        self.ignore_resource_requirements.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ignore_resource_requirements(&mut self, v: bool) {
        self.ignore_resource_requirements = ::std::option::Option::Some(v);
    }

    pub fn get_ignore_resource_requirements(&self) -> bool {
        self.ignore_resource_requirements.unwrap_or(false)
    }

    fn get_ignore_resource_requirements_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ignore_resource_requirements
    }

    fn mut_ignore_resource_requirements_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ignore_resource_requirements
    }
}

impl ::protobuf::Message for RequestQuery {
    fn is_initialized(&self) -> bool {
        for v in &self.pathing {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.abilities {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.placements {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pathing)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.abilities)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.placements)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.ignore_resource_requirements = ::std::option::Option::Some(tmp);
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
        for value in &self.pathing {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.abilities {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.placements {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.ignore_resource_requirements {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.pathing {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.abilities {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.placements {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.ignore_resource_requirements {
            os.write_bool(4, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestQuery {
    fn new() -> RequestQuery {
        RequestQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestQueryPathing>>(
                    "pathing",
                    RequestQuery::get_pathing_for_reflect,
                    RequestQuery::mut_pathing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestQueryAvailableAbilities>>(
                    "abilities",
                    RequestQuery::get_abilities_for_reflect,
                    RequestQuery::mut_abilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RequestQueryBuildingPlacement>>(
                    "placements",
                    RequestQuery::get_placements_for_reflect,
                    RequestQuery::mut_placements_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ignore_resource_requirements",
                    RequestQuery::get_ignore_resource_requirements_for_reflect,
                    RequestQuery::mut_ignore_resource_requirements_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestQuery>(
                    "RequestQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestQuery {
    fn clear(&mut self) {
        self.clear_pathing();
        self.clear_abilities();
        self.clear_placements();
        self.clear_ignore_resource_requirements();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseQuery {
    // message fields
    pathing: ::protobuf::RepeatedField<ResponseQueryPathing>,
    abilities: ::protobuf::RepeatedField<ResponseQueryAvailableAbilities>,
    placements: ::protobuf::RepeatedField<ResponseQueryBuildingPlacement>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseQuery {}

impl ResponseQuery {
    pub fn new() -> ResponseQuery {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseQuery {
        static mut instance: ::protobuf::lazy::Lazy<ResponseQuery> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseQuery,
        };
        unsafe {
            instance.get(ResponseQuery::new)
        }
    }

    // repeated .SC2APIProtocol.ResponseQueryPathing pathing = 1;

    pub fn clear_pathing(&mut self) {
        self.pathing.clear();
    }

    // Param is passed by value, moved
    pub fn set_pathing(&mut self, v: ::protobuf::RepeatedField<ResponseQueryPathing>) {
        self.pathing = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pathing(&mut self) -> &mut ::protobuf::RepeatedField<ResponseQueryPathing> {
        &mut self.pathing
    }

    // Take field
    pub fn take_pathing(&mut self) -> ::protobuf::RepeatedField<ResponseQueryPathing> {
        ::std::mem::replace(&mut self.pathing, ::protobuf::RepeatedField::new())
    }

    pub fn get_pathing(&self) -> &[ResponseQueryPathing] {
        &self.pathing
    }

    fn get_pathing_for_reflect(&self) -> &::protobuf::RepeatedField<ResponseQueryPathing> {
        &self.pathing
    }

    fn mut_pathing_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ResponseQueryPathing> {
        &mut self.pathing
    }

    // repeated .SC2APIProtocol.ResponseQueryAvailableAbilities abilities = 2;

    pub fn clear_abilities(&mut self) {
        self.abilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_abilities(&mut self, v: ::protobuf::RepeatedField<ResponseQueryAvailableAbilities>) {
        self.abilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_abilities(&mut self) -> &mut ::protobuf::RepeatedField<ResponseQueryAvailableAbilities> {
        &mut self.abilities
    }

    // Take field
    pub fn take_abilities(&mut self) -> ::protobuf::RepeatedField<ResponseQueryAvailableAbilities> {
        ::std::mem::replace(&mut self.abilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_abilities(&self) -> &[ResponseQueryAvailableAbilities] {
        &self.abilities
    }

    fn get_abilities_for_reflect(&self) -> &::protobuf::RepeatedField<ResponseQueryAvailableAbilities> {
        &self.abilities
    }

    fn mut_abilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ResponseQueryAvailableAbilities> {
        &mut self.abilities
    }

    // repeated .SC2APIProtocol.ResponseQueryBuildingPlacement placements = 3;

    pub fn clear_placements(&mut self) {
        self.placements.clear();
    }

    // Param is passed by value, moved
    pub fn set_placements(&mut self, v: ::protobuf::RepeatedField<ResponseQueryBuildingPlacement>) {
        self.placements = v;
    }

    // Mutable pointer to the field.
    pub fn mut_placements(&mut self) -> &mut ::protobuf::RepeatedField<ResponseQueryBuildingPlacement> {
        &mut self.placements
    }

    // Take field
    pub fn take_placements(&mut self) -> ::protobuf::RepeatedField<ResponseQueryBuildingPlacement> {
        ::std::mem::replace(&mut self.placements, ::protobuf::RepeatedField::new())
    }

    pub fn get_placements(&self) -> &[ResponseQueryBuildingPlacement] {
        &self.placements
    }

    fn get_placements_for_reflect(&self) -> &::protobuf::RepeatedField<ResponseQueryBuildingPlacement> {
        &self.placements
    }

    fn mut_placements_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ResponseQueryBuildingPlacement> {
        &mut self.placements
    }
}

impl ::protobuf::Message for ResponseQuery {
    fn is_initialized(&self) -> bool {
        for v in &self.pathing {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.abilities {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.placements {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pathing)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.abilities)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.placements)?;
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
        for value in &self.pathing {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.abilities {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.placements {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.pathing {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.abilities {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.placements {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseQuery {
    fn new() -> ResponseQuery {
        ResponseQuery::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseQuery>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseQueryPathing>>(
                    "pathing",
                    ResponseQuery::get_pathing_for_reflect,
                    ResponseQuery::mut_pathing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseQueryAvailableAbilities>>(
                    "abilities",
                    ResponseQuery::get_abilities_for_reflect,
                    ResponseQuery::mut_abilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResponseQueryBuildingPlacement>>(
                    "placements",
                    ResponseQuery::get_placements_for_reflect,
                    ResponseQuery::mut_placements_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseQuery>(
                    "ResponseQuery",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseQuery {
    fn clear(&mut self) {
        self.clear_pathing();
        self.clear_abilities();
        self.clear_placements();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseQuery {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseQuery {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestQueryPathing {
    // message fields
    end_pos: ::protobuf::SingularPtrField<super::common::Point2D>,
    // message oneof groups
    start: ::std::option::Option<RequestQueryPathing_oneof_start>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestQueryPathing {}

#[derive(Clone,PartialEq)]
pub enum RequestQueryPathing_oneof_start {
    start_pos(super::common::Point2D),
    unit_tag(u64),
}

impl RequestQueryPathing {
    pub fn new() -> RequestQueryPathing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestQueryPathing {
        static mut instance: ::protobuf::lazy::Lazy<RequestQueryPathing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestQueryPathing,
        };
        unsafe {
            instance.get(RequestQueryPathing::new)
        }
    }

    // optional .SC2APIProtocol.Point2D start_pos = 1;

    pub fn clear_start_pos(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start_pos(&self) -> bool {
        match self.start {
            ::std::option::Option::Some(RequestQueryPathing_oneof_start::start_pos(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_start_pos(&mut self, v: super::common::Point2D) {
        self.start = ::std::option::Option::Some(RequestQueryPathing_oneof_start::start_pos(v))
    }

    // Mutable pointer to the field.
    pub fn mut_start_pos(&mut self) -> &mut super::common::Point2D {
        if let ::std::option::Option::Some(RequestQueryPathing_oneof_start::start_pos(_)) = self.start {
        } else {
            self.start = ::std::option::Option::Some(RequestQueryPathing_oneof_start::start_pos(super::common::Point2D::new()));
        }
        match self.start {
            ::std::option::Option::Some(RequestQueryPathing_oneof_start::start_pos(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_start_pos(&mut self) -> super::common::Point2D {
        if self.has_start_pos() {
            match self.start.take() {
                ::std::option::Option::Some(RequestQueryPathing_oneof_start::start_pos(v)) => v,
                _ => panic!(),
            }
        } else {
            super::common::Point2D::new()
        }
    }

    pub fn get_start_pos(&self) -> &super::common::Point2D {
        match self.start {
            ::std::option::Option::Some(RequestQueryPathing_oneof_start::start_pos(ref v)) => v,
            _ => super::common::Point2D::default_instance(),
        }
    }

    // optional uint64 unit_tag = 2;

    pub fn clear_unit_tag(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_unit_tag(&self) -> bool {
        match self.start {
            ::std::option::Option::Some(RequestQueryPathing_oneof_start::unit_tag(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unit_tag(&mut self, v: u64) {
        self.start = ::std::option::Option::Some(RequestQueryPathing_oneof_start::unit_tag(v))
    }

    pub fn get_unit_tag(&self) -> u64 {
        match self.start {
            ::std::option::Option::Some(RequestQueryPathing_oneof_start::unit_tag(v)) => v,
            _ => 0,
        }
    }

    // optional .SC2APIProtocol.Point2D end_pos = 3;

    pub fn clear_end_pos(&mut self) {
        self.end_pos.clear();
    }

    pub fn has_end_pos(&self) -> bool {
        self.end_pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_pos(&mut self, v: super::common::Point2D) {
        self.end_pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_pos(&mut self) -> &mut super::common::Point2D {
        if self.end_pos.is_none() {
            self.end_pos.set_default();
        }
        self.end_pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_pos(&mut self) -> super::common::Point2D {
        self.end_pos.take().unwrap_or_else(|| super::common::Point2D::new())
    }

    pub fn get_end_pos(&self) -> &super::common::Point2D {
        self.end_pos.as_ref().unwrap_or_else(|| super::common::Point2D::default_instance())
    }

    fn get_end_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point2D> {
        &self.end_pos
    }

    fn mut_end_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point2D> {
        &mut self.end_pos
    }
}

impl ::protobuf::Message for RequestQueryPathing {
    fn is_initialized(&self) -> bool {
        if let Some(RequestQueryPathing_oneof_start::start_pos(ref v)) = self.start {
            if !v.is_initialized() {
                return false;
            }
        }
        for v in &self.end_pos {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.start = ::std::option::Option::Some(RequestQueryPathing_oneof_start::start_pos(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.start = ::std::option::Option::Some(RequestQueryPathing_oneof_start::unit_tag(is.read_uint64()?));
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end_pos)?;
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
        if let Some(ref v) = self.end_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.start {
            match v {
                &RequestQueryPathing_oneof_start::start_pos(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestQueryPathing_oneof_start::unit_tag(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.end_pos.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.start {
            match v {
                &RequestQueryPathing_oneof_start::start_pos(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RequestQueryPathing_oneof_start::unit_tag(v) => {
                    os.write_uint64(2, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestQueryPathing {
    fn new() -> RequestQueryPathing {
        RequestQueryPathing::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestQueryPathing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::common::Point2D>(
                    "start_pos",
                    RequestQueryPathing::has_start_pos,
                    RequestQueryPathing::get_start_pos,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "unit_tag",
                    RequestQueryPathing::has_unit_tag,
                    RequestQueryPathing::get_unit_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point2D>>(
                    "end_pos",
                    RequestQueryPathing::get_end_pos_for_reflect,
                    RequestQueryPathing::mut_end_pos_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestQueryPathing>(
                    "RequestQueryPathing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestQueryPathing {
    fn clear(&mut self) {
        self.clear_start_pos();
        self.clear_unit_tag();
        self.clear_end_pos();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestQueryPathing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestQueryPathing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseQueryPathing {
    // message fields
    distance: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseQueryPathing {}

impl ResponseQueryPathing {
    pub fn new() -> ResponseQueryPathing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseQueryPathing {
        static mut instance: ::protobuf::lazy::Lazy<ResponseQueryPathing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseQueryPathing,
        };
        unsafe {
            instance.get(ResponseQueryPathing::new)
        }
    }

    // optional float distance = 1;

    pub fn clear_distance(&mut self) {
        self.distance = ::std::option::Option::None;
    }

    pub fn has_distance(&self) -> bool {
        self.distance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distance(&mut self, v: f32) {
        self.distance = ::std::option::Option::Some(v);
    }

    pub fn get_distance(&self) -> f32 {
        self.distance.unwrap_or(0.)
    }

    fn get_distance_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.distance
    }

    fn mut_distance_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.distance
    }
}

impl ::protobuf::Message for ResponseQueryPathing {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.distance = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.distance {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.distance {
            os.write_float(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseQueryPathing {
    fn new() -> ResponseQueryPathing {
        ResponseQueryPathing::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseQueryPathing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "distance",
                    ResponseQueryPathing::get_distance_for_reflect,
                    ResponseQueryPathing::mut_distance_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseQueryPathing>(
                    "ResponseQueryPathing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseQueryPathing {
    fn clear(&mut self) {
        self.clear_distance();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseQueryPathing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseQueryPathing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestQueryAvailableAbilities {
    // message fields
    unit_tag: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestQueryAvailableAbilities {}

impl RequestQueryAvailableAbilities {
    pub fn new() -> RequestQueryAvailableAbilities {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestQueryAvailableAbilities {
        static mut instance: ::protobuf::lazy::Lazy<RequestQueryAvailableAbilities> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestQueryAvailableAbilities,
        };
        unsafe {
            instance.get(RequestQueryAvailableAbilities::new)
        }
    }

    // optional uint64 unit_tag = 1;

    pub fn clear_unit_tag(&mut self) {
        self.unit_tag = ::std::option::Option::None;
    }

    pub fn has_unit_tag(&self) -> bool {
        self.unit_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_tag(&mut self, v: u64) {
        self.unit_tag = ::std::option::Option::Some(v);
    }

    pub fn get_unit_tag(&self) -> u64 {
        self.unit_tag.unwrap_or(0)
    }

    fn get_unit_tag_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.unit_tag
    }

    fn mut_unit_tag_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.unit_tag
    }
}

impl ::protobuf::Message for RequestQueryAvailableAbilities {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.unit_tag = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.unit_tag {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_tag {
            os.write_uint64(1, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestQueryAvailableAbilities {
    fn new() -> RequestQueryAvailableAbilities {
        RequestQueryAvailableAbilities::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestQueryAvailableAbilities>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "unit_tag",
                    RequestQueryAvailableAbilities::get_unit_tag_for_reflect,
                    RequestQueryAvailableAbilities::mut_unit_tag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestQueryAvailableAbilities>(
                    "RequestQueryAvailableAbilities",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestQueryAvailableAbilities {
    fn clear(&mut self) {
        self.clear_unit_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestQueryAvailableAbilities {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestQueryAvailableAbilities {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseQueryAvailableAbilities {
    // message fields
    abilities: ::protobuf::RepeatedField<super::common::AvailableAbility>,
    unit_tag: ::std::option::Option<u64>,
    unit_type_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseQueryAvailableAbilities {}

impl ResponseQueryAvailableAbilities {
    pub fn new() -> ResponseQueryAvailableAbilities {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseQueryAvailableAbilities {
        static mut instance: ::protobuf::lazy::Lazy<ResponseQueryAvailableAbilities> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseQueryAvailableAbilities,
        };
        unsafe {
            instance.get(ResponseQueryAvailableAbilities::new)
        }
    }

    // repeated .SC2APIProtocol.AvailableAbility abilities = 1;

    pub fn clear_abilities(&mut self) {
        self.abilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_abilities(&mut self, v: ::protobuf::RepeatedField<super::common::AvailableAbility>) {
        self.abilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_abilities(&mut self) -> &mut ::protobuf::RepeatedField<super::common::AvailableAbility> {
        &mut self.abilities
    }

    // Take field
    pub fn take_abilities(&mut self) -> ::protobuf::RepeatedField<super::common::AvailableAbility> {
        ::std::mem::replace(&mut self.abilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_abilities(&self) -> &[super::common::AvailableAbility] {
        &self.abilities
    }

    fn get_abilities_for_reflect(&self) -> &::protobuf::RepeatedField<super::common::AvailableAbility> {
        &self.abilities
    }

    fn mut_abilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::common::AvailableAbility> {
        &mut self.abilities
    }

    // optional uint64 unit_tag = 2;

    pub fn clear_unit_tag(&mut self) {
        self.unit_tag = ::std::option::Option::None;
    }

    pub fn has_unit_tag(&self) -> bool {
        self.unit_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_tag(&mut self, v: u64) {
        self.unit_tag = ::std::option::Option::Some(v);
    }

    pub fn get_unit_tag(&self) -> u64 {
        self.unit_tag.unwrap_or(0)
    }

    fn get_unit_tag_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.unit_tag
    }

    fn mut_unit_tag_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.unit_tag
    }

    // optional uint32 unit_type_id = 3;

    pub fn clear_unit_type_id(&mut self) {
        self.unit_type_id = ::std::option::Option::None;
    }

    pub fn has_unit_type_id(&self) -> bool {
        self.unit_type_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_type_id(&mut self, v: u32) {
        self.unit_type_id = ::std::option::Option::Some(v);
    }

    pub fn get_unit_type_id(&self) -> u32 {
        self.unit_type_id.unwrap_or(0)
    }

    fn get_unit_type_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unit_type_id
    }

    fn mut_unit_type_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unit_type_id
    }
}

impl ::protobuf::Message for ResponseQueryAvailableAbilities {
    fn is_initialized(&self) -> bool {
        for v in &self.abilities {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.abilities)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.unit_tag = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.unit_type_id = ::std::option::Option::Some(tmp);
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
        for value in &self.abilities {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.unit_tag {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.unit_type_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.abilities {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.unit_tag {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.unit_type_id {
            os.write_uint32(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseQueryAvailableAbilities {
    fn new() -> ResponseQueryAvailableAbilities {
        ResponseQueryAvailableAbilities::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseQueryAvailableAbilities>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::AvailableAbility>>(
                    "abilities",
                    ResponseQueryAvailableAbilities::get_abilities_for_reflect,
                    ResponseQueryAvailableAbilities::mut_abilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "unit_tag",
                    ResponseQueryAvailableAbilities::get_unit_tag_for_reflect,
                    ResponseQueryAvailableAbilities::mut_unit_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unit_type_id",
                    ResponseQueryAvailableAbilities::get_unit_type_id_for_reflect,
                    ResponseQueryAvailableAbilities::mut_unit_type_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseQueryAvailableAbilities>(
                    "ResponseQueryAvailableAbilities",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseQueryAvailableAbilities {
    fn clear(&mut self) {
        self.clear_abilities();
        self.clear_unit_tag();
        self.clear_unit_type_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseQueryAvailableAbilities {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseQueryAvailableAbilities {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestQueryBuildingPlacement {
    // message fields
    ability_id: ::std::option::Option<i32>,
    target_pos: ::protobuf::SingularPtrField<super::common::Point2D>,
    placing_unit_tag: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestQueryBuildingPlacement {}

impl RequestQueryBuildingPlacement {
    pub fn new() -> RequestQueryBuildingPlacement {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestQueryBuildingPlacement {
        static mut instance: ::protobuf::lazy::Lazy<RequestQueryBuildingPlacement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestQueryBuildingPlacement,
        };
        unsafe {
            instance.get(RequestQueryBuildingPlacement::new)
        }
    }

    // optional int32 ability_id = 1;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: i32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> i32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ability_id
    }

    // optional .SC2APIProtocol.Point2D target_pos = 2;

    pub fn clear_target_pos(&mut self) {
        self.target_pos.clear();
    }

    pub fn has_target_pos(&self) -> bool {
        self.target_pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_pos(&mut self, v: super::common::Point2D) {
        self.target_pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target_pos(&mut self) -> &mut super::common::Point2D {
        if self.target_pos.is_none() {
            self.target_pos.set_default();
        }
        self.target_pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_target_pos(&mut self) -> super::common::Point2D {
        self.target_pos.take().unwrap_or_else(|| super::common::Point2D::new())
    }

    pub fn get_target_pos(&self) -> &super::common::Point2D {
        self.target_pos.as_ref().unwrap_or_else(|| super::common::Point2D::default_instance())
    }

    fn get_target_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point2D> {
        &self.target_pos
    }

    fn mut_target_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point2D> {
        &mut self.target_pos
    }

    // optional uint64 placing_unit_tag = 3;

    pub fn clear_placing_unit_tag(&mut self) {
        self.placing_unit_tag = ::std::option::Option::None;
    }

    pub fn has_placing_unit_tag(&self) -> bool {
        self.placing_unit_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_placing_unit_tag(&mut self, v: u64) {
        self.placing_unit_tag = ::std::option::Option::Some(v);
    }

    pub fn get_placing_unit_tag(&self) -> u64 {
        self.placing_unit_tag.unwrap_or(0)
    }

    fn get_placing_unit_tag_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.placing_unit_tag
    }

    fn mut_placing_unit_tag_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.placing_unit_tag
    }
}

impl ::protobuf::Message for RequestQueryBuildingPlacement {
    fn is_initialized(&self) -> bool {
        for v in &self.target_pos {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.target_pos)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.placing_unit_tag = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.target_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.placing_unit_tag {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_id {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.target_pos.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.placing_unit_tag {
            os.write_uint64(3, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestQueryBuildingPlacement {
    fn new() -> RequestQueryBuildingPlacement {
        RequestQueryBuildingPlacement::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestQueryBuildingPlacement>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ability_id",
                    RequestQueryBuildingPlacement::get_ability_id_for_reflect,
                    RequestQueryBuildingPlacement::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point2D>>(
                    "target_pos",
                    RequestQueryBuildingPlacement::get_target_pos_for_reflect,
                    RequestQueryBuildingPlacement::mut_target_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "placing_unit_tag",
                    RequestQueryBuildingPlacement::get_placing_unit_tag_for_reflect,
                    RequestQueryBuildingPlacement::mut_placing_unit_tag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestQueryBuildingPlacement>(
                    "RequestQueryBuildingPlacement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestQueryBuildingPlacement {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.clear_target_pos();
        self.clear_placing_unit_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestQueryBuildingPlacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestQueryBuildingPlacement {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseQueryBuildingPlacement {
    // message fields
    result: ::std::option::Option<super::error::ActionResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseQueryBuildingPlacement {}

impl ResponseQueryBuildingPlacement {
    pub fn new() -> ResponseQueryBuildingPlacement {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseQueryBuildingPlacement {
        static mut instance: ::protobuf::lazy::Lazy<ResponseQueryBuildingPlacement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseQueryBuildingPlacement,
        };
        unsafe {
            instance.get(ResponseQueryBuildingPlacement::new)
        }
    }

    // optional .SC2APIProtocol.ActionResult result = 1;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: super::error::ActionResult) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> super::error::ActionResult {
        self.result.unwrap_or(super::error::ActionResult::Success)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<super::error::ActionResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<super::error::ActionResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for ResponseQueryBuildingPlacement {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.result = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result {
            os.write_enum(1, v.value())?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseQueryBuildingPlacement {
    fn new() -> ResponseQueryBuildingPlacement {
        ResponseQueryBuildingPlacement::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseQueryBuildingPlacement>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::error::ActionResult>>(
                    "result",
                    ResponseQueryBuildingPlacement::get_result_for_reflect,
                    ResponseQueryBuildingPlacement::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseQueryBuildingPlacement>(
                    "ResponseQueryBuildingPlacement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseQueryBuildingPlacement {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseQueryBuildingPlacement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseQueryBuildingPlacement {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cs2clientprotocol/query.proto\x12\x0eSC2APIProtocol\x1a\x1ds2client\
    protocol/common.proto\x1a\x1cs2clientprotocol/error.proto\"\xac\x02\n\
    \x0cRequestQuery\x12=\n\x07pathing\x18\x01\x20\x03(\x0b2#.SC2APIProtocol\
    .RequestQueryPathingR\x07pathing\x12L\n\tabilities\x18\x02\x20\x03(\x0b2\
    ..SC2APIProtocol.RequestQueryAvailableAbilitiesR\tabilities\x12M\n\nplac\
    ements\x18\x03\x20\x03(\x0b2-.SC2APIProtocol.RequestQueryBuildingPlaceme\
    ntR\nplacements\x12@\n\x1cignore_resource_requirements\x18\x04\x20\x01(\
    \x08R\x1aignoreResourceRequirements\"\xee\x01\n\rResponseQuery\x12>\n\
    \x07pathing\x18\x01\x20\x03(\x0b2$.SC2APIProtocol.ResponseQueryPathingR\
    \x07pathing\x12M\n\tabilities\x18\x02\x20\x03(\x0b2/.SC2APIProtocol.Resp\
    onseQueryAvailableAbilitiesR\tabilities\x12N\n\nplacements\x18\x03\x20\
    \x03(\x0b2..SC2APIProtocol.ResponseQueryBuildingPlacementR\nplacements\"\
    \xa5\x01\n\x13RequestQueryPathing\x126\n\tstart_pos\x18\x01\x20\x01(\x0b\
    2\x17.SC2APIProtocol.Point2DH\0R\x08startPos\x12\x1b\n\x08unit_tag\x18\
    \x02\x20\x01(\x04H\0R\x07unitTag\x120\n\x07end_pos\x18\x03\x20\x01(\x0b2\
    \x17.SC2APIProtocol.Point2DR\x06endPosB\x07\n\x05start\"2\n\x14ResponseQ\
    ueryPathing\x12\x1a\n\x08distance\x18\x01\x20\x01(\x02R\x08distance\";\n\
    \x1eRequestQueryAvailableAbilities\x12\x19\n\x08unit_tag\x18\x01\x20\x01\
    (\x04R\x07unitTag\"\x9e\x01\n\x1fResponseQueryAvailableAbilities\x12>\n\
    \tabilities\x18\x01\x20\x03(\x0b2\x20.SC2APIProtocol.AvailableAbilityR\t\
    abilities\x12\x19\n\x08unit_tag\x18\x02\x20\x01(\x04R\x07unitTag\x12\x20\
    \n\x0cunit_type_id\x18\x03\x20\x01(\rR\nunitTypeId\"\xa0\x01\n\x1dReques\
    tQueryBuildingPlacement\x12\x1d\n\nability_id\x18\x01\x20\x01(\x05R\tabi\
    lityId\x126\n\ntarget_pos\x18\x02\x20\x01(\x0b2\x17.SC2APIProtocol.Point\
    2DR\ttargetPos\x12(\n\x10placing_unit_tag\x18\x03\x20\x01(\x04R\x0eplaci\
    ngUnitTag\"V\n\x1eResponseQueryBuildingPlacement\x124\n\x06result\x18\
    \x01\x20\x01(\x0e2\x1c.SC2APIProtocol.ActionResultR\x06resultJ\x9e\x0f\n\
    \x06\x12\x04\x01\06\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\x08\n\x01\
    \x02\x12\x03\x03\x08\x16\n\t\n\x02\x03\0\x12\x03\x05\x07&\n\t\n\x02\x03\
    \x01\x12\x03\x06\x07%\n\n\n\x02\x04\0\x12\x04\x08\0\r\x01\n\n\n\x03\x04\
    \0\x01\x12\x03\x08\x08\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\t\x02+\n\x0c\
    \n\x05\x04\0\x02\0\x04\x12\x03\t\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\
    \x03\t\x0b\x1e\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\t\x1f&\n\x0c\n\x05\
    \x04\0\x02\0\x03\x12\x03\t)*\n\x0b\n\x04\x04\0\x02\x01\x12\x03\n\x028\n\
    \x0c\n\x05\x04\0\x02\x01\x04\x12\x03\n\x02\n\n\x0c\n\x05\x04\0\x02\x01\
    \x06\x12\x03\n\x0b)\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\n*3\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\n67\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x0b\
    \x028\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x0b\x02\n\n\x0c\n\x05\x04\0\
    \x02\x02\x06\x12\x03\x0b\x0b(\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0b)\
    3\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0b67\nB\n\x04\x04\0\x02\x03\x12\
    \x03\x0c\x021\"5\x20Ignores\x20requirements\x20like\x20food,\x20minerals\
    \x20and\x20so\x20on.\n\n\x0c\n\x05\x04\0\x02\x03\x04\x12\x03\x0c\x02\n\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x0c\x0b\x0f\n\x0c\n\x05\x04\0\x02\
    \x03\x01\x12\x03\x0c\x10,\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x0c/0\n\
    \n\n\x02\x04\x01\x12\x04\x0f\0\x13\x01\n\n\n\x03\x04\x01\x01\x12\x03\x0f\
    \x08\x15\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x10\x02,\n\x0c\n\x05\x04\x01\
    \x02\0\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x10\
    \x0b\x1f\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x10\x20'\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x10*+\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x11\x02\
    9\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x11\x02\n\n\x0c\n\x05\x04\x01\
    \x02\x01\x06\x12\x03\x11\x0b*\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\
    \x11+4\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1178\n\x0b\n\x04\x04\x01\
    \x02\x02\x12\x03\x12\x029\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\x12\
    \x02\n\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03\x12\x0b)\n\x0c\n\x05\x04\
    \x01\x02\x02\x01\x12\x03\x12*4\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\
    \x1278\no\n\x02\x04\x02\x12\x04\x16\0\x1c\x01\x1ac----------------------\
    ------------------------------------------------------------------------\
    ----\n\n\n\n\x03\x04\x02\x01\x12\x03\x16\x08\x1b\n\x0c\n\x04\x04\x02\x08\
    \0\x12\x04\x17\x02\x1a\x03\n\x0c\n\x05\x04\x02\x08\0\x01\x12\x03\x17\x08\
    \r\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x18\x04\x1a\n\x0c\n\x05\x04\x02\x02\
    \0\x06\x12\x03\x18\x04\x0b\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x18\x0c\
    \x15\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x18\x18\x19\n\x0b\n\x04\x04\
    \x02\x02\x01\x12\x03\x19\x04\x18\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\
    \x19\x04\n\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x19\x0b\x13\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03\x19\x16\x17\n\x0b\n\x04\x04\x02\x02\x02\
    \x12\x03\x1b\x02\x1f\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03\x1b\x02\n\n\
    \x0c\n\x05\x04\x02\x02\x02\x06\x12\x03\x1b\x0b\x12\n\x0c\n\x05\x04\x02\
    \x02\x02\x01\x12\x03\x1b\x13\x1a\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\
    \x1b\x1d\x1e\n\n\n\x02\x04\x03\x12\x04\x1e\0\x20\x01\n\n\n\x03\x04\x03\
    \x01\x12\x03\x1e\x08\x1c\n\"\n\x04\x04\x03\x02\0\x12\x03\x1f\x02\x1e\"\
    \x15\x200\x20if\x20no\x20path\x20exists\n\n\x0c\n\x05\x04\x03\x02\0\x04\
    \x12\x03\x1f\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x1f\x0b\x10\n\
    \x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1f\x11\x19\n\x0c\n\x05\x04\x03\x02\
    \0\x03\x12\x03\x1f\x1c\x1d\no\n\x02\x04\x04\x12\x04#\0%\x01\x1ac--------\
    ------------------------------------------------------------------------\
    ------------------\n\n\n\n\x03\x04\x04\x01\x12\x03#\x08&\n\x0b\n\x04\x04\
    \x04\x02\0\x12\x03$\x02\x1f\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03$\x02\n\
    \n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03$\x0b\x11\n\x0c\n\x05\x04\x04\x02\
    \0\x01\x12\x03$\x12\x1a\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03$\x1d\x1e\n\
    \n\n\x02\x04\x05\x12\x04'\0+\x01\n\n\n\x03\x04\x05\x01\x12\x03'\x08'\n\
    \x0b\n\x04\x04\x05\x02\0\x12\x03(\x02*\n\x0c\n\x05\x04\x05\x02\0\x04\x12\
    \x03(\x02\n\n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03(\x0b\x1b\n\x0c\n\x05\
    \x04\x05\x02\0\x01\x12\x03(\x1c%\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03((\
    )\n\x0b\n\x04\x04\x05\x02\x01\x12\x03)\x02\x1f\n\x0c\n\x05\x04\x05\x02\
    \x01\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03)\x0b\x11\
    \n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03)\x12\x1a\n\x0c\n\x05\x04\x05\
    \x02\x01\x03\x12\x03)\x1d\x1e\n\x0b\n\x04\x04\x05\x02\x02\x12\x03*\x02#\
    \n\x0c\n\x05\x04\x05\x02\x02\x04\x12\x03*\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x02\x05\x12\x03*\x0b\x11\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03*\x12\
    \x1e\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03*!\"\no\n\x02\x04\x06\x12\
    \x04.\02\x01\x1ac-------------------------------------------------------\
    -------------------------------------------\n\n\n\n\x03\x04\x06\x01\x12\
    \x03.\x08%\n\x0b\n\x04\x04\x06\x02\0\x12\x03/\x02\x20\n\x0c\n\x05\x04\
    \x06\x02\0\x04\x12\x03/\x02\n\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x03/\x0b\
    \x10\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03/\x11\x1b\n\x0c\n\x05\x04\x06\
    \x02\0\x03\x12\x03/\x1e\x1f\n\x0b\n\x04\x04\x06\x02\x01\x12\x030\x02\"\n\
    \x0c\n\x05\x04\x06\x02\x01\x04\x12\x030\x02\n\n\x0c\n\x05\x04\x06\x02\
    \x01\x06\x12\x030\x0b\x12\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x030\x13\
    \x1d\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x030\x20!\n\x1b\n\x04\x04\x06\
    \x02\x02\x12\x031\x02'\"\x0e\x20Not\x20required\n\n\x0c\n\x05\x04\x06\
    \x02\x02\x04\x12\x031\x02\n\n\x0c\n\x05\x04\x06\x02\x02\x05\x12\x031\x0b\
    \x11\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x031\x12\"\n\x0c\n\x05\x04\x06\
    \x02\x02\x03\x12\x031%&\n\n\n\x02\x04\x07\x12\x044\06\x01\n\n\n\x03\x04\
    \x07\x01\x12\x034\x08&\n\x0b\n\x04\x04\x07\x02\0\x12\x035\x02#\n\x0c\n\
    \x05\x04\x07\x02\0\x04\x12\x035\x02\n\n\x0c\n\x05\x04\x07\x02\0\x06\x12\
    \x035\x0b\x17\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x035\x18\x1e\n\x0c\n\x05\
    \x04\x07\x02\0\x03\x12\x035!\"\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
