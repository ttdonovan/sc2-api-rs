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
pub struct AvailableAbility {
    // message fields
    ability_id: ::std::option::Option<i32>,
    requires_point: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AvailableAbility {}

impl AvailableAbility {
    pub fn new() -> AvailableAbility {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AvailableAbility {
        static mut instance: ::protobuf::lazy::Lazy<AvailableAbility> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AvailableAbility,
        };
        unsafe {
            instance.get(AvailableAbility::new)
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

    // optional bool requires_point = 2;

    pub fn clear_requires_point(&mut self) {
        self.requires_point = ::std::option::Option::None;
    }

    pub fn has_requires_point(&self) -> bool {
        self.requires_point.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requires_point(&mut self, v: bool) {
        self.requires_point = ::std::option::Option::Some(v);
    }

    pub fn get_requires_point(&self) -> bool {
        self.requires_point.unwrap_or(false)
    }

    fn get_requires_point_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.requires_point
    }

    fn mut_requires_point_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.requires_point
    }
}

impl ::protobuf::Message for AvailableAbility {
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
                    let tmp = is.read_int32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.requires_point = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.requires_point {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_id {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.requires_point {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for AvailableAbility {
    fn new() -> AvailableAbility {
        AvailableAbility::new()
    }

    fn descriptor_static(_: ::std::option::Option<AvailableAbility>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ability_id",
                    AvailableAbility::get_ability_id_for_reflect,
                    AvailableAbility::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "requires_point",
                    AvailableAbility::get_requires_point_for_reflect,
                    AvailableAbility::mut_requires_point_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AvailableAbility>(
                    "AvailableAbility",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AvailableAbility {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.clear_requires_point();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AvailableAbility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AvailableAbility {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImageData {
    // message fields
    bits_per_pixel: ::std::option::Option<i32>,
    size: ::protobuf::SingularPtrField<Size2DI>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImageData {}

impl ImageData {
    pub fn new() -> ImageData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImageData {
        static mut instance: ::protobuf::lazy::Lazy<ImageData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImageData,
        };
        unsafe {
            instance.get(ImageData::new)
        }
    }

    // optional int32 bits_per_pixel = 1;

    pub fn clear_bits_per_pixel(&mut self) {
        self.bits_per_pixel = ::std::option::Option::None;
    }

    pub fn has_bits_per_pixel(&self) -> bool {
        self.bits_per_pixel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bits_per_pixel(&mut self, v: i32) {
        self.bits_per_pixel = ::std::option::Option::Some(v);
    }

    pub fn get_bits_per_pixel(&self) -> i32 {
        self.bits_per_pixel.unwrap_or(0)
    }

    fn get_bits_per_pixel_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.bits_per_pixel
    }

    fn mut_bits_per_pixel_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.bits_per_pixel
    }

    // optional .SC2APIProtocol.Size2DI size = 2;

    pub fn clear_size(&mut self) {
        self.size.clear();
    }

    pub fn has_size(&self) -> bool {
        self.size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: Size2DI) {
        self.size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_size(&mut self) -> &mut Size2DI {
        if self.size.is_none() {
            self.size.set_default();
        }
        self.size.as_mut().unwrap()
    }

    // Take field
    pub fn take_size(&mut self) -> Size2DI {
        self.size.take().unwrap_or_else(|| Size2DI::new())
    }

    pub fn get_size(&self) -> &Size2DI {
        self.size.as_ref().unwrap_or_else(|| Size2DI::default_instance())
    }

    fn get_size_for_reflect(&self) -> &::protobuf::SingularPtrField<Size2DI> {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Size2DI> {
        &mut self.size
    }

    // optional bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for ImageData {
    fn is_initialized(&self) -> bool {
        for v in &self.size {
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
                    self.bits_per_pixel = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.size)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
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
        if let Some(v) = self.bits_per_pixel {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bits_per_pixel {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.size.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for ImageData {
    fn new() -> ImageData {
        ImageData::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImageData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "bits_per_pixel",
                    ImageData::get_bits_per_pixel_for_reflect,
                    ImageData::mut_bits_per_pixel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Size2DI>>(
                    "size",
                    ImageData::get_size_for_reflect,
                    ImageData::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ImageData::get_data_for_reflect,
                    ImageData::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImageData>(
                    "ImageData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImageData {
    fn clear(&mut self) {
        self.clear_bits_per_pixel();
        self.clear_size();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImageData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImageData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PointI {
    // message fields
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PointI {}

impl PointI {
    pub fn new() -> PointI {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PointI {
        static mut instance: ::protobuf::lazy::Lazy<PointI> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PointI,
        };
        unsafe {
            instance.get(PointI::new)
        }
    }

    // optional int32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.x
    }

    // optional int32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.y
    }
}

impl ::protobuf::Message for PointI {
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
                    let tmp = is.read_int32()?;
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.y = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.y {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for PointI {
    fn new() -> PointI {
        PointI::new()
    }

    fn descriptor_static(_: ::std::option::Option<PointI>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "x",
                    PointI::get_x_for_reflect,
                    PointI::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "y",
                    PointI::get_y_for_reflect,
                    PointI::mut_y_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PointI>(
                    "PointI",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PointI {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PointI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PointI {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RectangleI {
    // message fields
    p0: ::protobuf::SingularPtrField<PointI>,
    p1: ::protobuf::SingularPtrField<PointI>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RectangleI {}

impl RectangleI {
    pub fn new() -> RectangleI {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RectangleI {
        static mut instance: ::protobuf::lazy::Lazy<RectangleI> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RectangleI,
        };
        unsafe {
            instance.get(RectangleI::new)
        }
    }

    // optional .SC2APIProtocol.PointI p0 = 1;

    pub fn clear_p0(&mut self) {
        self.p0.clear();
    }

    pub fn has_p0(&self) -> bool {
        self.p0.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p0(&mut self, v: PointI) {
        self.p0 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_p0(&mut self) -> &mut PointI {
        if self.p0.is_none() {
            self.p0.set_default();
        }
        self.p0.as_mut().unwrap()
    }

    // Take field
    pub fn take_p0(&mut self) -> PointI {
        self.p0.take().unwrap_or_else(|| PointI::new())
    }

    pub fn get_p0(&self) -> &PointI {
        self.p0.as_ref().unwrap_or_else(|| PointI::default_instance())
    }

    fn get_p0_for_reflect(&self) -> &::protobuf::SingularPtrField<PointI> {
        &self.p0
    }

    fn mut_p0_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PointI> {
        &mut self.p0
    }

    // optional .SC2APIProtocol.PointI p1 = 2;

    pub fn clear_p1(&mut self) {
        self.p1.clear();
    }

    pub fn has_p1(&self) -> bool {
        self.p1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p1(&mut self, v: PointI) {
        self.p1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_p1(&mut self) -> &mut PointI {
        if self.p1.is_none() {
            self.p1.set_default();
        }
        self.p1.as_mut().unwrap()
    }

    // Take field
    pub fn take_p1(&mut self) -> PointI {
        self.p1.take().unwrap_or_else(|| PointI::new())
    }

    pub fn get_p1(&self) -> &PointI {
        self.p1.as_ref().unwrap_or_else(|| PointI::default_instance())
    }

    fn get_p1_for_reflect(&self) -> &::protobuf::SingularPtrField<PointI> {
        &self.p1
    }

    fn mut_p1_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PointI> {
        &mut self.p1
    }
}

impl ::protobuf::Message for RectangleI {
    fn is_initialized(&self) -> bool {
        for v in &self.p0 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.p1 {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.p0)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.p1)?;
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
        if let Some(ref v) = self.p0.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.p1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.p0.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.p1.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for RectangleI {
    fn new() -> RectangleI {
        RectangleI::new()
    }

    fn descriptor_static(_: ::std::option::Option<RectangleI>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PointI>>(
                    "p0",
                    RectangleI::get_p0_for_reflect,
                    RectangleI::mut_p0_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PointI>>(
                    "p1",
                    RectangleI::get_p1_for_reflect,
                    RectangleI::mut_p1_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RectangleI>(
                    "RectangleI",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RectangleI {
    fn clear(&mut self) {
        self.clear_p0();
        self.clear_p1();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RectangleI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RectangleI {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Point2D {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Point2D {}

impl Point2D {
    pub fn new() -> Point2D {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Point2D {
        static mut instance: ::protobuf::lazy::Lazy<Point2D> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Point2D,
        };
        unsafe {
            instance.get(Point2D::new)
        }
    }

    // optional float x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.x
    }

    // optional float y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.y
    }
}

impl ::protobuf::Message for Point2D {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += 5;
        }
        if let Some(v) = self.y {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_float(2, v)?;
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

impl ::protobuf::MessageStatic for Point2D {
    fn new() -> Point2D {
        Point2D::new()
    }

    fn descriptor_static(_: ::std::option::Option<Point2D>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    Point2D::get_x_for_reflect,
                    Point2D::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    Point2D::get_y_for_reflect,
                    Point2D::mut_y_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Point2D>(
                    "Point2D",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Point2D {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Point2D {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Point2D {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Point {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    z: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Point {}

impl Point {
    pub fn new() -> Point {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Point {
        static mut instance: ::protobuf::lazy::Lazy<Point> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Point,
        };
        unsafe {
            instance.get(Point::new)
        }
    }

    // optional float x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.x
    }

    // optional float y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.y
    }

    // optional float z = 3;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> f32 {
        self.z.unwrap_or(0.)
    }

    fn get_z_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.z
    }

    fn mut_z_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.z
    }
}

impl ::protobuf::Message for Point {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.z = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += 5;
        }
        if let Some(v) = self.y {
            my_size += 5;
        }
        if let Some(v) = self.z {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.z {
            os.write_float(3, v)?;
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

impl ::protobuf::MessageStatic for Point {
    fn new() -> Point {
        Point::new()
    }

    fn descriptor_static(_: ::std::option::Option<Point>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    Point::get_x_for_reflect,
                    Point::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    Point::get_y_for_reflect,
                    Point::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "z",
                    Point::get_z_for_reflect,
                    Point::mut_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Point>(
                    "Point",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Point {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Point {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Size2DI {
    // message fields
    x: ::std::option::Option<i32>,
    y: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Size2DI {}

impl Size2DI {
    pub fn new() -> Size2DI {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Size2DI {
        static mut instance: ::protobuf::lazy::Lazy<Size2DI> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Size2DI,
        };
        unsafe {
            instance.get(Size2DI::new)
        }
    }

    // optional int32 x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: i32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> i32 {
        self.x.unwrap_or(0)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.x
    }

    // optional int32 y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: i32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> i32 {
        self.y.unwrap_or(0)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.y
    }
}

impl ::protobuf::Message for Size2DI {
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
                    let tmp = is.read_int32()?;
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.y = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.y {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for Size2DI {
    fn new() -> Size2DI {
        Size2DI::new()
    }

    fn descriptor_static(_: ::std::option::Option<Size2DI>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "x",
                    Size2DI::get_x_for_reflect,
                    Size2DI::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "y",
                    Size2DI::get_y_for_reflect,
                    Size2DI::mut_y_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Size2DI>(
                    "Size2DI",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Size2DI {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Size2DI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Size2DI {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Race {
    NoRace = 0,
    Terran = 1,
    Zerg = 2,
    Protoss = 3,
    Random = 4,
}

impl ::protobuf::ProtobufEnum for Race {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Race> {
        match value {
            0 => ::std::option::Option::Some(Race::NoRace),
            1 => ::std::option::Option::Some(Race::Terran),
            2 => ::std::option::Option::Some(Race::Zerg),
            3 => ::std::option::Option::Some(Race::Protoss),
            4 => ::std::option::Option::Some(Race::Random),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Race] = &[
            Race::NoRace,
            Race::Terran,
            Race::Zerg,
            Race::Protoss,
            Race::Random,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Race>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Race", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Race {
}

impl ::protobuf::reflect::ProtobufValue for Race {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1ds2clientprotocol/common.proto\x12\x0eSC2APIProtocol\"X\n\x10Availa\
    bleAbility\x12\x1d\n\nability_id\x18\x01\x20\x01(\x05R\tabilityId\x12%\n\
    \x0erequires_point\x18\x02\x20\x01(\x08R\rrequiresPoint\"r\n\tImageData\
    \x12$\n\x0ebits_per_pixel\x18\x01\x20\x01(\x05R\x0cbitsPerPixel\x12+\n\
    \x04size\x18\x02\x20\x01(\x0b2\x17.SC2APIProtocol.Size2DIR\x04size\x12\
    \x12\n\x04data\x18\x03\x20\x01(\x0cR\x04data\"$\n\x06PointI\x12\x0c\n\
    \x01x\x18\x01\x20\x01(\x05R\x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x05R\
    \x01y\"\\\n\nRectangleI\x12&\n\x02p0\x18\x01\x20\x01(\x0b2\x16.SC2APIPro\
    tocol.PointIR\x02p0\x12&\n\x02p1\x18\x02\x20\x01(\x0b2\x16.SC2APIProtoco\
    l.PointIR\x02p1\"%\n\x07Point2D\x12\x0c\n\x01x\x18\x01\x20\x01(\x02R\x01\
    x\x12\x0c\n\x01y\x18\x02\x20\x01(\x02R\x01y\"1\n\x05Point\x12\x0c\n\x01x\
    \x18\x01\x20\x01(\x02R\x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x02R\x01y\
    \x12\x0c\n\x01z\x18\x03\x20\x01(\x02R\x01z\"%\n\x07Size2DI\x12\x0c\n\x01\
    x\x18\x01\x20\x01(\x05R\x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x05R\x01y*A\
    \n\x04Race\x12\n\n\x06NoRace\x10\0\x12\n\n\x06Terran\x10\x01\x12\x08\n\
    \x04Zerg\x10\x02\x12\x0b\n\x07Protoss\x10\x03\x12\n\n\x06Random\x10\x04J\
    \xca\x0f\n\x06\x12\x04\x01\08\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\
    \x08\n\x01\x02\x12\x03\x03\x08\x16\n\n\n\x02\x04\0\x12\x04\x05\0\x08\x01\
    \n\n\n\x03\x04\0\x01\x12\x03\x05\x08\x18\n\x0b\n\x04\x04\0\x02\0\x12\x03\
    \x06\x02\x20\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x06\x02\n\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x06\x0b\x10\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x06\x11\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06\x1e\x1f\n\x0b\n\x04\
    \x04\0\x02\x01\x12\x03\x07\x02#\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\
    \x07\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x07\x0b\x0f\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x07\x10\x1e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x07!\"\n\n\n\x02\x04\x01\x12\x04\n\0\x0e\x01\n\n\n\x03\x04\x01\x01\
    \x12\x03\n\x08\x11\n?\n\x04\x04\x01\x02\0\x12\x03\x0b\x02$\"2\x20Number\
    \x20of\x20bits\x20per\x20pixel;\x208\x20bits\x20for\x20a\x20byte\x20etc.\
    \n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x0b\x02\n\n\x0c\n\x05\x04\x01\
    \x02\0\x05\x12\x03\x0b\x0b\x10\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0b\
    \x11\x1f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b\"#\n#\n\x04\x04\x01\
    \x02\x01\x12\x03\x0c\x02\x1c\"\x16\x20Dimension\x20in\x20pixels.\n\n\x0c\
    \n\x05\x04\x01\x02\x01\x04\x12\x03\x0c\x02\n\n\x0c\n\x05\x04\x01\x02\x01\
    \x06\x12\x03\x0c\x0b\x12\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0c\x13\
    \x17\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x0c\x1a\x1b\nd\n\x04\x04\
    \x01\x02\x02\x12\x03\r\x02\x1a\"W\x20Binary\x20data;\x20the\x20size\x20o\
    f\x20this\x20buffer\x20in\x20bytes\x20is\x20width\x20*\x20height\x20*\
    \x20bits_per_pixel\x20/\x208.\n\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\
    \r\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\r\x0b\x10\n\x0c\n\x05\
    \x04\x01\x02\x02\x01\x12\x03\r\x11\x15\n\x0c\n\x05\x04\x01\x02\x02\x03\
    \x12\x03\r\x18\x19\nb\n\x02\x04\x02\x12\x04\x12\0\x15\x01\x1aV\x20Point\
    \x20on\x20the\x20screen/minimap\x20(e.g.,\x200..64).\n\x20Note:\x20botto\
    m\x20left\x20of\x20the\x20screen\x20is\x200,\x200.\n\n\n\n\x03\x04\x02\
    \x01\x12\x03\x12\x08\x0e\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x13\x02\x17\n\
    \x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x13\x02\n\n\x0c\n\x05\x04\x02\x02\0\
    \x05\x12\x03\x13\x0b\x10\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x13\x11\
    \x12\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x13\x15\x16\n\x0b\n\x04\x04\
    \x02\x02\x01\x12\x03\x14\x02\x17\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\
    \x14\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x14\x0b\x10\n\x0c\n\
    \x05\x04\x02\x02\x01\x01\x12\x03\x14\x11\x12\n\x0c\n\x05\x04\x02\x02\x01\
    \x03\x12\x03\x14\x15\x16\n,\n\x02\x04\x03\x12\x04\x18\0\x1b\x01\x1a\x20\
    \x20Screen\x20space\x20rectangular\x20area.\n\n\n\n\x03\x04\x03\x01\x12\
    \x03\x18\x08\x12\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x19\x02\x19\n\x0c\n\
    \x05\x04\x03\x02\0\x04\x12\x03\x19\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\
    \x12\x03\x19\x0b\x11\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x19\x12\x14\n\
    \x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x19\x17\x18\n\x0b\n\x04\x04\x03\x02\
    \x01\x12\x03\x1a\x02\x19\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03\x1a\x02\
    \n\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x03\x1a\x0b\x11\n\x0c\n\x05\x04\
    \x03\x02\x01\x01\x12\x03\x1a\x12\x14\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\
    \x03\x1a\x17\x18\nX\n\x02\x04\x04\x12\x04\x1f\0\"\x01\x1aL\x20Point\x20o\
    n\x20the\x20game\x20board,\x200..255.\n\x20Note:\x20bottom\x20left\x20of\
    \x20the\x20screen\x20is\x200,\x200.\n\n\n\n\x03\x04\x04\x01\x12\x03\x1f\
    \x08\x0f\n\x0b\n\x04\x04\x04\x02\0\x12\x03\x20\x02\x17\n\x0c\n\x05\x04\
    \x04\x02\0\x04\x12\x03\x20\x02\n\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03\
    \x20\x0b\x10\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x20\x11\x12\n\x0c\n\
    \x05\x04\x04\x02\0\x03\x12\x03\x20\x15\x16\n\x0b\n\x04\x04\x04\x02\x01\
    \x12\x03!\x02\x17\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03!\x02\n\n\x0c\n\
    \x05\x04\x04\x02\x01\x05\x12\x03!\x0b\x10\n\x0c\n\x05\x04\x04\x02\x01\
    \x01\x12\x03!\x11\x12\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03!\x15\x16\n\
    X\n\x02\x04\x05\x12\x04&\0*\x01\x1aL\x20Point\x20on\x20the\x20game\x20bo\
    ard,\x200..255.\n\x20Note:\x20bottom\x20left\x20of\x20the\x20screen\x20i\
    s\x200,\x200.\n\n\n\n\x03\x04\x05\x01\x12\x03&\x08\r\n\x0b\n\x04\x04\x05\
    \x02\0\x12\x03'\x02\x17\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03'\x02\n\n\
    \x0c\n\x05\x04\x05\x02\0\x05\x12\x03'\x0b\x10\n\x0c\n\x05\x04\x05\x02\0\
    \x01\x12\x03'\x11\x12\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03'\x15\x16\n\
    \x0b\n\x04\x04\x05\x02\x01\x12\x03(\x02\x17\n\x0c\n\x05\x04\x05\x02\x01\
    \x04\x12\x03(\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03(\x0b\x10\n\
    \x0c\n\x05\x04\x05\x02\x01\x01\x12\x03(\x11\x12\n\x0c\n\x05\x04\x05\x02\
    \x01\x03\x12\x03(\x15\x16\n\x0b\n\x04\x04\x05\x02\x02\x12\x03)\x02\x17\n\
    \x0c\n\x05\x04\x05\x02\x02\x04\x12\x03)\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x02\x05\x12\x03)\x0b\x10\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03)\x11\
    \x12\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03)\x15\x16\n\x20\n\x02\x04\
    \x06\x12\x04-\00\x01\x1a\x14\x20Screen\x20dimensions.\n\n\n\n\x03\x04\
    \x06\x01\x12\x03-\x08\x0f\n\x0b\n\x04\x04\x06\x02\0\x12\x03.\x02\x17\n\
    \x0c\n\x05\x04\x06\x02\0\x04\x12\x03.\x02\n\n\x0c\n\x05\x04\x06\x02\0\
    \x05\x12\x03.\x0b\x10\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03.\x11\x12\n\
    \x0c\n\x05\x04\x06\x02\0\x03\x12\x03.\x15\x16\n\x0b\n\x04\x04\x06\x02\
    \x01\x12\x03/\x02\x17\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03/\x02\n\n\
    \x0c\n\x05\x04\x06\x02\x01\x05\x12\x03/\x0b\x10\n\x0c\n\x05\x04\x06\x02\
    \x01\x01\x12\x03/\x11\x12\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03/\x15\
    \x16\n\n\n\x02\x05\0\x12\x042\08\x01\n\n\n\x03\x05\0\x01\x12\x032\x05\t\
    \n\x0b\n\x04\x05\0\x02\0\x12\x033\x02\r\n\x0c\n\x05\x05\0\x02\0\x01\x12\
    \x033\x02\x08\n\x0c\n\x05\x05\0\x02\0\x02\x12\x033\x0b\x0c\n\x0b\n\x04\
    \x05\0\x02\x01\x12\x034\x02\r\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x034\x02\
    \x08\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x034\x0b\x0c\n\x0b\n\x04\x05\0\
    \x02\x02\x12\x035\x02\x0b\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x035\x02\x06\
    \n\x0c\n\x05\x05\0\x02\x02\x02\x12\x035\t\n\n\x0b\n\x04\x05\0\x02\x03\
    \x12\x036\x02\x0e\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x036\x02\t\n\x0c\n\
    \x05\x05\0\x02\x03\x02\x12\x036\x0c\r\n\x0b\n\x04\x05\0\x02\x04\x12\x037\
    \x02\r\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x037\x02\x08\n\x0c\n\x05\x05\0\
    \x02\x04\x02\x12\x037\x0b\x0c\
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
