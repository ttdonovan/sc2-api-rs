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
pub struct Score {
    // message fields
    score_type: ::std::option::Option<Score_ScoreType>,
    score: ::std::option::Option<i32>,
    score_details: ::protobuf::SingularPtrField<ScoreDetails>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Score {}

impl Score {
    pub fn new() -> Score {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Score {
        static mut instance: ::protobuf::lazy::Lazy<Score> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Score,
        };
        unsafe {
            instance.get(Score::new)
        }
    }

    // optional .SC2APIProtocol.Score.ScoreType score_type = 6;

    pub fn clear_score_type(&mut self) {
        self.score_type = ::std::option::Option::None;
    }

    pub fn has_score_type(&self) -> bool {
        self.score_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_score_type(&mut self, v: Score_ScoreType) {
        self.score_type = ::std::option::Option::Some(v);
    }

    pub fn get_score_type(&self) -> Score_ScoreType {
        self.score_type.unwrap_or(Score_ScoreType::Curriculum)
    }

    fn get_score_type_for_reflect(&self) -> &::std::option::Option<Score_ScoreType> {
        &self.score_type
    }

    fn mut_score_type_for_reflect(&mut self) -> &mut ::std::option::Option<Score_ScoreType> {
        &mut self.score_type
    }

    // optional int32 score = 7;

    pub fn clear_score(&mut self) {
        self.score = ::std::option::Option::None;
    }

    pub fn has_score(&self) -> bool {
        self.score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: i32) {
        self.score = ::std::option::Option::Some(v);
    }

    pub fn get_score(&self) -> i32 {
        self.score.unwrap_or(0)
    }

    fn get_score_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.score
    }

    fn mut_score_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.score
    }

    // optional .SC2APIProtocol.ScoreDetails score_details = 8;

    pub fn clear_score_details(&mut self) {
        self.score_details.clear();
    }

    pub fn has_score_details(&self) -> bool {
        self.score_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_score_details(&mut self, v: ScoreDetails) {
        self.score_details = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_score_details(&mut self) -> &mut ScoreDetails {
        if self.score_details.is_none() {
            self.score_details.set_default();
        }
        self.score_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_score_details(&mut self) -> ScoreDetails {
        self.score_details.take().unwrap_or_else(|| ScoreDetails::new())
    }

    pub fn get_score_details(&self) -> &ScoreDetails {
        self.score_details.as_ref().unwrap_or_else(|| ScoreDetails::default_instance())
    }

    fn get_score_details_for_reflect(&self) -> &::protobuf::SingularPtrField<ScoreDetails> {
        &self.score_details
    }

    fn mut_score_details_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ScoreDetails> {
        &mut self.score_details
    }
}

impl ::protobuf::Message for Score {
    fn is_initialized(&self) -> bool {
        for v in &self.score_details {
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
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.score_type = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.score = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.score_details)?;
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
        if let Some(v) = self.score_type {
            my_size += ::protobuf::rt::enum_size(6, v);
        }
        if let Some(v) = self.score {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.score_details.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.score_type {
            os.write_enum(6, v.value())?;
        }
        if let Some(v) = self.score {
            os.write_int32(7, v)?;
        }
        if let Some(ref v) = self.score_details.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Score {
    fn new() -> Score {
        Score::new()
    }

    fn descriptor_static(_: ::std::option::Option<Score>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Score_ScoreType>>(
                    "score_type",
                    Score::get_score_type_for_reflect,
                    Score::mut_score_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "score",
                    Score::get_score_for_reflect,
                    Score::mut_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ScoreDetails>>(
                    "score_details",
                    Score::get_score_details_for_reflect,
                    Score::mut_score_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Score>(
                    "Score",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Score {
    fn clear(&mut self) {
        self.clear_score_type();
        self.clear_score();
        self.clear_score_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Score {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Score {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Score_ScoreType {
    Curriculum = 1,
    Melee = 2,
}

impl ::protobuf::ProtobufEnum for Score_ScoreType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Score_ScoreType> {
        match value {
            1 => ::std::option::Option::Some(Score_ScoreType::Curriculum),
            2 => ::std::option::Option::Some(Score_ScoreType::Melee),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Score_ScoreType] = &[
            Score_ScoreType::Curriculum,
            Score_ScoreType::Melee,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Score_ScoreType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Score_ScoreType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Score_ScoreType {
}

impl ::protobuf::reflect::ProtobufValue for Score_ScoreType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CategoryScoreDetails {
    // message fields
    none: ::std::option::Option<f32>,
    army: ::std::option::Option<f32>,
    economy: ::std::option::Option<f32>,
    technology: ::std::option::Option<f32>,
    upgrade: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CategoryScoreDetails {}

impl CategoryScoreDetails {
    pub fn new() -> CategoryScoreDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CategoryScoreDetails {
        static mut instance: ::protobuf::lazy::Lazy<CategoryScoreDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CategoryScoreDetails,
        };
        unsafe {
            instance.get(CategoryScoreDetails::new)
        }
    }

    // optional float none = 1;

    pub fn clear_none(&mut self) {
        self.none = ::std::option::Option::None;
    }

    pub fn has_none(&self) -> bool {
        self.none.is_some()
    }

    // Param is passed by value, moved
    pub fn set_none(&mut self, v: f32) {
        self.none = ::std::option::Option::Some(v);
    }

    pub fn get_none(&self) -> f32 {
        self.none.unwrap_or(0.)
    }

    fn get_none_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.none
    }

    fn mut_none_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.none
    }

    // optional float army = 2;

    pub fn clear_army(&mut self) {
        self.army = ::std::option::Option::None;
    }

    pub fn has_army(&self) -> bool {
        self.army.is_some()
    }

    // Param is passed by value, moved
    pub fn set_army(&mut self, v: f32) {
        self.army = ::std::option::Option::Some(v);
    }

    pub fn get_army(&self) -> f32 {
        self.army.unwrap_or(0.)
    }

    fn get_army_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.army
    }

    fn mut_army_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.army
    }

    // optional float economy = 3;

    pub fn clear_economy(&mut self) {
        self.economy = ::std::option::Option::None;
    }

    pub fn has_economy(&self) -> bool {
        self.economy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_economy(&mut self, v: f32) {
        self.economy = ::std::option::Option::Some(v);
    }

    pub fn get_economy(&self) -> f32 {
        self.economy.unwrap_or(0.)
    }

    fn get_economy_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.economy
    }

    fn mut_economy_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.economy
    }

    // optional float technology = 4;

    pub fn clear_technology(&mut self) {
        self.technology = ::std::option::Option::None;
    }

    pub fn has_technology(&self) -> bool {
        self.technology.is_some()
    }

    // Param is passed by value, moved
    pub fn set_technology(&mut self, v: f32) {
        self.technology = ::std::option::Option::Some(v);
    }

    pub fn get_technology(&self) -> f32 {
        self.technology.unwrap_or(0.)
    }

    fn get_technology_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.technology
    }

    fn mut_technology_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.technology
    }

    // optional float upgrade = 5;

    pub fn clear_upgrade(&mut self) {
        self.upgrade = ::std::option::Option::None;
    }

    pub fn has_upgrade(&self) -> bool {
        self.upgrade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade(&mut self, v: f32) {
        self.upgrade = ::std::option::Option::Some(v);
    }

    pub fn get_upgrade(&self) -> f32 {
        self.upgrade.unwrap_or(0.)
    }

    fn get_upgrade_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.upgrade
    }

    fn mut_upgrade_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.upgrade
    }
}

impl ::protobuf::Message for CategoryScoreDetails {
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
                    self.none = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.army = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.economy = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.technology = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.upgrade = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.none {
            my_size += 5;
        }
        if let Some(v) = self.army {
            my_size += 5;
        }
        if let Some(v) = self.economy {
            my_size += 5;
        }
        if let Some(v) = self.technology {
            my_size += 5;
        }
        if let Some(v) = self.upgrade {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.none {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.army {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.economy {
            os.write_float(3, v)?;
        }
        if let Some(v) = self.technology {
            os.write_float(4, v)?;
        }
        if let Some(v) = self.upgrade {
            os.write_float(5, v)?;
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

impl ::protobuf::MessageStatic for CategoryScoreDetails {
    fn new() -> CategoryScoreDetails {
        CategoryScoreDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CategoryScoreDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "none",
                    CategoryScoreDetails::get_none_for_reflect,
                    CategoryScoreDetails::mut_none_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "army",
                    CategoryScoreDetails::get_army_for_reflect,
                    CategoryScoreDetails::mut_army_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "economy",
                    CategoryScoreDetails::get_economy_for_reflect,
                    CategoryScoreDetails::mut_economy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "technology",
                    CategoryScoreDetails::get_technology_for_reflect,
                    CategoryScoreDetails::mut_technology_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "upgrade",
                    CategoryScoreDetails::get_upgrade_for_reflect,
                    CategoryScoreDetails::mut_upgrade_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CategoryScoreDetails>(
                    "CategoryScoreDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CategoryScoreDetails {
    fn clear(&mut self) {
        self.clear_none();
        self.clear_army();
        self.clear_economy();
        self.clear_technology();
        self.clear_upgrade();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CategoryScoreDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CategoryScoreDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VitalScoreDetails {
    // message fields
    life: ::std::option::Option<f32>,
    shields: ::std::option::Option<f32>,
    energy: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VitalScoreDetails {}

impl VitalScoreDetails {
    pub fn new() -> VitalScoreDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VitalScoreDetails {
        static mut instance: ::protobuf::lazy::Lazy<VitalScoreDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VitalScoreDetails,
        };
        unsafe {
            instance.get(VitalScoreDetails::new)
        }
    }

    // optional float life = 1;

    pub fn clear_life(&mut self) {
        self.life = ::std::option::Option::None;
    }

    pub fn has_life(&self) -> bool {
        self.life.is_some()
    }

    // Param is passed by value, moved
    pub fn set_life(&mut self, v: f32) {
        self.life = ::std::option::Option::Some(v);
    }

    pub fn get_life(&self) -> f32 {
        self.life.unwrap_or(0.)
    }

    fn get_life_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.life
    }

    fn mut_life_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.life
    }

    // optional float shields = 2;

    pub fn clear_shields(&mut self) {
        self.shields = ::std::option::Option::None;
    }

    pub fn has_shields(&self) -> bool {
        self.shields.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shields(&mut self, v: f32) {
        self.shields = ::std::option::Option::Some(v);
    }

    pub fn get_shields(&self) -> f32 {
        self.shields.unwrap_or(0.)
    }

    fn get_shields_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.shields
    }

    fn mut_shields_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.shields
    }

    // optional float energy = 3;

    pub fn clear_energy(&mut self) {
        self.energy = ::std::option::Option::None;
    }

    pub fn has_energy(&self) -> bool {
        self.energy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_energy(&mut self, v: f32) {
        self.energy = ::std::option::Option::Some(v);
    }

    pub fn get_energy(&self) -> f32 {
        self.energy.unwrap_or(0.)
    }

    fn get_energy_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.energy
    }

    fn mut_energy_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.energy
    }
}

impl ::protobuf::Message for VitalScoreDetails {
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
                    self.life = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.shields = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.energy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.life {
            my_size += 5;
        }
        if let Some(v) = self.shields {
            my_size += 5;
        }
        if let Some(v) = self.energy {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.life {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.shields {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.energy {
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

impl ::protobuf::MessageStatic for VitalScoreDetails {
    fn new() -> VitalScoreDetails {
        VitalScoreDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<VitalScoreDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "life",
                    VitalScoreDetails::get_life_for_reflect,
                    VitalScoreDetails::mut_life_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "shields",
                    VitalScoreDetails::get_shields_for_reflect,
                    VitalScoreDetails::mut_shields_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "energy",
                    VitalScoreDetails::get_energy_for_reflect,
                    VitalScoreDetails::mut_energy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VitalScoreDetails>(
                    "VitalScoreDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VitalScoreDetails {
    fn clear(&mut self) {
        self.clear_life();
        self.clear_shields();
        self.clear_energy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VitalScoreDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VitalScoreDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScoreDetails {
    // message fields
    idle_production_time: ::std::option::Option<f32>,
    idle_worker_time: ::std::option::Option<f32>,
    total_value_units: ::std::option::Option<f32>,
    total_value_structures: ::std::option::Option<f32>,
    killed_value_units: ::std::option::Option<f32>,
    killed_value_structures: ::std::option::Option<f32>,
    collected_minerals: ::std::option::Option<f32>,
    collected_vespene: ::std::option::Option<f32>,
    collection_rate_minerals: ::std::option::Option<f32>,
    collection_rate_vespene: ::std::option::Option<f32>,
    spent_minerals: ::std::option::Option<f32>,
    spent_vespene: ::std::option::Option<f32>,
    food_used: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    killed_minerals: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    killed_vespene: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    lost_minerals: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    lost_vespene: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    friendly_fire_minerals: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    friendly_fire_vespene: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    used_minerals: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    used_vespene: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    total_used_minerals: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    total_used_vespene: ::protobuf::SingularPtrField<CategoryScoreDetails>,
    total_damage_dealt: ::protobuf::SingularPtrField<VitalScoreDetails>,
    total_damage_taken: ::protobuf::SingularPtrField<VitalScoreDetails>,
    total_healed: ::protobuf::SingularPtrField<VitalScoreDetails>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScoreDetails {}

impl ScoreDetails {
    pub fn new() -> ScoreDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScoreDetails {
        static mut instance: ::protobuf::lazy::Lazy<ScoreDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScoreDetails,
        };
        unsafe {
            instance.get(ScoreDetails::new)
        }
    }

    // optional float idle_production_time = 1;

    pub fn clear_idle_production_time(&mut self) {
        self.idle_production_time = ::std::option::Option::None;
    }

    pub fn has_idle_production_time(&self) -> bool {
        self.idle_production_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_idle_production_time(&mut self, v: f32) {
        self.idle_production_time = ::std::option::Option::Some(v);
    }

    pub fn get_idle_production_time(&self) -> f32 {
        self.idle_production_time.unwrap_or(0.)
    }

    fn get_idle_production_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.idle_production_time
    }

    fn mut_idle_production_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.idle_production_time
    }

    // optional float idle_worker_time = 2;

    pub fn clear_idle_worker_time(&mut self) {
        self.idle_worker_time = ::std::option::Option::None;
    }

    pub fn has_idle_worker_time(&self) -> bool {
        self.idle_worker_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_idle_worker_time(&mut self, v: f32) {
        self.idle_worker_time = ::std::option::Option::Some(v);
    }

    pub fn get_idle_worker_time(&self) -> f32 {
        self.idle_worker_time.unwrap_or(0.)
    }

    fn get_idle_worker_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.idle_worker_time
    }

    fn mut_idle_worker_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.idle_worker_time
    }

    // optional float total_value_units = 3;

    pub fn clear_total_value_units(&mut self) {
        self.total_value_units = ::std::option::Option::None;
    }

    pub fn has_total_value_units(&self) -> bool {
        self.total_value_units.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_value_units(&mut self, v: f32) {
        self.total_value_units = ::std::option::Option::Some(v);
    }

    pub fn get_total_value_units(&self) -> f32 {
        self.total_value_units.unwrap_or(0.)
    }

    fn get_total_value_units_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.total_value_units
    }

    fn mut_total_value_units_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.total_value_units
    }

    // optional float total_value_structures = 4;

    pub fn clear_total_value_structures(&mut self) {
        self.total_value_structures = ::std::option::Option::None;
    }

    pub fn has_total_value_structures(&self) -> bool {
        self.total_value_structures.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_value_structures(&mut self, v: f32) {
        self.total_value_structures = ::std::option::Option::Some(v);
    }

    pub fn get_total_value_structures(&self) -> f32 {
        self.total_value_structures.unwrap_or(0.)
    }

    fn get_total_value_structures_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.total_value_structures
    }

    fn mut_total_value_structures_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.total_value_structures
    }

    // optional float killed_value_units = 5;

    pub fn clear_killed_value_units(&mut self) {
        self.killed_value_units = ::std::option::Option::None;
    }

    pub fn has_killed_value_units(&self) -> bool {
        self.killed_value_units.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killed_value_units(&mut self, v: f32) {
        self.killed_value_units = ::std::option::Option::Some(v);
    }

    pub fn get_killed_value_units(&self) -> f32 {
        self.killed_value_units.unwrap_or(0.)
    }

    fn get_killed_value_units_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.killed_value_units
    }

    fn mut_killed_value_units_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.killed_value_units
    }

    // optional float killed_value_structures = 6;

    pub fn clear_killed_value_structures(&mut self) {
        self.killed_value_structures = ::std::option::Option::None;
    }

    pub fn has_killed_value_structures(&self) -> bool {
        self.killed_value_structures.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killed_value_structures(&mut self, v: f32) {
        self.killed_value_structures = ::std::option::Option::Some(v);
    }

    pub fn get_killed_value_structures(&self) -> f32 {
        self.killed_value_structures.unwrap_or(0.)
    }

    fn get_killed_value_structures_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.killed_value_structures
    }

    fn mut_killed_value_structures_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.killed_value_structures
    }

    // optional float collected_minerals = 7;

    pub fn clear_collected_minerals(&mut self) {
        self.collected_minerals = ::std::option::Option::None;
    }

    pub fn has_collected_minerals(&self) -> bool {
        self.collected_minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collected_minerals(&mut self, v: f32) {
        self.collected_minerals = ::std::option::Option::Some(v);
    }

    pub fn get_collected_minerals(&self) -> f32 {
        self.collected_minerals.unwrap_or(0.)
    }

    fn get_collected_minerals_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.collected_minerals
    }

    fn mut_collected_minerals_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.collected_minerals
    }

    // optional float collected_vespene = 8;

    pub fn clear_collected_vespene(&mut self) {
        self.collected_vespene = ::std::option::Option::None;
    }

    pub fn has_collected_vespene(&self) -> bool {
        self.collected_vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collected_vespene(&mut self, v: f32) {
        self.collected_vespene = ::std::option::Option::Some(v);
    }

    pub fn get_collected_vespene(&self) -> f32 {
        self.collected_vespene.unwrap_or(0.)
    }

    fn get_collected_vespene_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.collected_vespene
    }

    fn mut_collected_vespene_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.collected_vespene
    }

    // optional float collection_rate_minerals = 9;

    pub fn clear_collection_rate_minerals(&mut self) {
        self.collection_rate_minerals = ::std::option::Option::None;
    }

    pub fn has_collection_rate_minerals(&self) -> bool {
        self.collection_rate_minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collection_rate_minerals(&mut self, v: f32) {
        self.collection_rate_minerals = ::std::option::Option::Some(v);
    }

    pub fn get_collection_rate_minerals(&self) -> f32 {
        self.collection_rate_minerals.unwrap_or(0.)
    }

    fn get_collection_rate_minerals_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.collection_rate_minerals
    }

    fn mut_collection_rate_minerals_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.collection_rate_minerals
    }

    // optional float collection_rate_vespene = 10;

    pub fn clear_collection_rate_vespene(&mut self) {
        self.collection_rate_vespene = ::std::option::Option::None;
    }

    pub fn has_collection_rate_vespene(&self) -> bool {
        self.collection_rate_vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collection_rate_vespene(&mut self, v: f32) {
        self.collection_rate_vespene = ::std::option::Option::Some(v);
    }

    pub fn get_collection_rate_vespene(&self) -> f32 {
        self.collection_rate_vespene.unwrap_or(0.)
    }

    fn get_collection_rate_vespene_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.collection_rate_vespene
    }

    fn mut_collection_rate_vespene_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.collection_rate_vespene
    }

    // optional float spent_minerals = 11;

    pub fn clear_spent_minerals(&mut self) {
        self.spent_minerals = ::std::option::Option::None;
    }

    pub fn has_spent_minerals(&self) -> bool {
        self.spent_minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spent_minerals(&mut self, v: f32) {
        self.spent_minerals = ::std::option::Option::Some(v);
    }

    pub fn get_spent_minerals(&self) -> f32 {
        self.spent_minerals.unwrap_or(0.)
    }

    fn get_spent_minerals_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.spent_minerals
    }

    fn mut_spent_minerals_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.spent_minerals
    }

    // optional float spent_vespene = 12;

    pub fn clear_spent_vespene(&mut self) {
        self.spent_vespene = ::std::option::Option::None;
    }

    pub fn has_spent_vespene(&self) -> bool {
        self.spent_vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spent_vespene(&mut self, v: f32) {
        self.spent_vespene = ::std::option::Option::Some(v);
    }

    pub fn get_spent_vespene(&self) -> f32 {
        self.spent_vespene.unwrap_or(0.)
    }

    fn get_spent_vespene_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.spent_vespene
    }

    fn mut_spent_vespene_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.spent_vespene
    }

    // optional .SC2APIProtocol.CategoryScoreDetails food_used = 13;

    pub fn clear_food_used(&mut self) {
        self.food_used.clear();
    }

    pub fn has_food_used(&self) -> bool {
        self.food_used.is_some()
    }

    // Param is passed by value, moved
    pub fn set_food_used(&mut self, v: CategoryScoreDetails) {
        self.food_used = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_food_used(&mut self) -> &mut CategoryScoreDetails {
        if self.food_used.is_none() {
            self.food_used.set_default();
        }
        self.food_used.as_mut().unwrap()
    }

    // Take field
    pub fn take_food_used(&mut self) -> CategoryScoreDetails {
        self.food_used.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_food_used(&self) -> &CategoryScoreDetails {
        self.food_used.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_food_used_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.food_used
    }

    fn mut_food_used_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.food_used
    }

    // optional .SC2APIProtocol.CategoryScoreDetails killed_minerals = 14;

    pub fn clear_killed_minerals(&mut self) {
        self.killed_minerals.clear();
    }

    pub fn has_killed_minerals(&self) -> bool {
        self.killed_minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killed_minerals(&mut self, v: CategoryScoreDetails) {
        self.killed_minerals = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_killed_minerals(&mut self) -> &mut CategoryScoreDetails {
        if self.killed_minerals.is_none() {
            self.killed_minerals.set_default();
        }
        self.killed_minerals.as_mut().unwrap()
    }

    // Take field
    pub fn take_killed_minerals(&mut self) -> CategoryScoreDetails {
        self.killed_minerals.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_killed_minerals(&self) -> &CategoryScoreDetails {
        self.killed_minerals.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_killed_minerals_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.killed_minerals
    }

    fn mut_killed_minerals_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.killed_minerals
    }

    // optional .SC2APIProtocol.CategoryScoreDetails killed_vespene = 15;

    pub fn clear_killed_vespene(&mut self) {
        self.killed_vespene.clear();
    }

    pub fn has_killed_vespene(&self) -> bool {
        self.killed_vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_killed_vespene(&mut self, v: CategoryScoreDetails) {
        self.killed_vespene = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_killed_vespene(&mut self) -> &mut CategoryScoreDetails {
        if self.killed_vespene.is_none() {
            self.killed_vespene.set_default();
        }
        self.killed_vespene.as_mut().unwrap()
    }

    // Take field
    pub fn take_killed_vespene(&mut self) -> CategoryScoreDetails {
        self.killed_vespene.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_killed_vespene(&self) -> &CategoryScoreDetails {
        self.killed_vespene.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_killed_vespene_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.killed_vespene
    }

    fn mut_killed_vespene_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.killed_vespene
    }

    // optional .SC2APIProtocol.CategoryScoreDetails lost_minerals = 16;

    pub fn clear_lost_minerals(&mut self) {
        self.lost_minerals.clear();
    }

    pub fn has_lost_minerals(&self) -> bool {
        self.lost_minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lost_minerals(&mut self, v: CategoryScoreDetails) {
        self.lost_minerals = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lost_minerals(&mut self) -> &mut CategoryScoreDetails {
        if self.lost_minerals.is_none() {
            self.lost_minerals.set_default();
        }
        self.lost_minerals.as_mut().unwrap()
    }

    // Take field
    pub fn take_lost_minerals(&mut self) -> CategoryScoreDetails {
        self.lost_minerals.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_lost_minerals(&self) -> &CategoryScoreDetails {
        self.lost_minerals.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_lost_minerals_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.lost_minerals
    }

    fn mut_lost_minerals_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.lost_minerals
    }

    // optional .SC2APIProtocol.CategoryScoreDetails lost_vespene = 17;

    pub fn clear_lost_vespene(&mut self) {
        self.lost_vespene.clear();
    }

    pub fn has_lost_vespene(&self) -> bool {
        self.lost_vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lost_vespene(&mut self, v: CategoryScoreDetails) {
        self.lost_vespene = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lost_vespene(&mut self) -> &mut CategoryScoreDetails {
        if self.lost_vespene.is_none() {
            self.lost_vespene.set_default();
        }
        self.lost_vespene.as_mut().unwrap()
    }

    // Take field
    pub fn take_lost_vespene(&mut self) -> CategoryScoreDetails {
        self.lost_vespene.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_lost_vespene(&self) -> &CategoryScoreDetails {
        self.lost_vespene.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_lost_vespene_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.lost_vespene
    }

    fn mut_lost_vespene_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.lost_vespene
    }

    // optional .SC2APIProtocol.CategoryScoreDetails friendly_fire_minerals = 18;

    pub fn clear_friendly_fire_minerals(&mut self) {
        self.friendly_fire_minerals.clear();
    }

    pub fn has_friendly_fire_minerals(&self) -> bool {
        self.friendly_fire_minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friendly_fire_minerals(&mut self, v: CategoryScoreDetails) {
        self.friendly_fire_minerals = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_friendly_fire_minerals(&mut self) -> &mut CategoryScoreDetails {
        if self.friendly_fire_minerals.is_none() {
            self.friendly_fire_minerals.set_default();
        }
        self.friendly_fire_minerals.as_mut().unwrap()
    }

    // Take field
    pub fn take_friendly_fire_minerals(&mut self) -> CategoryScoreDetails {
        self.friendly_fire_minerals.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_friendly_fire_minerals(&self) -> &CategoryScoreDetails {
        self.friendly_fire_minerals.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_friendly_fire_minerals_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.friendly_fire_minerals
    }

    fn mut_friendly_fire_minerals_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.friendly_fire_minerals
    }

    // optional .SC2APIProtocol.CategoryScoreDetails friendly_fire_vespene = 19;

    pub fn clear_friendly_fire_vespene(&mut self) {
        self.friendly_fire_vespene.clear();
    }

    pub fn has_friendly_fire_vespene(&self) -> bool {
        self.friendly_fire_vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friendly_fire_vespene(&mut self, v: CategoryScoreDetails) {
        self.friendly_fire_vespene = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_friendly_fire_vespene(&mut self) -> &mut CategoryScoreDetails {
        if self.friendly_fire_vespene.is_none() {
            self.friendly_fire_vespene.set_default();
        }
        self.friendly_fire_vespene.as_mut().unwrap()
    }

    // Take field
    pub fn take_friendly_fire_vespene(&mut self) -> CategoryScoreDetails {
        self.friendly_fire_vespene.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_friendly_fire_vespene(&self) -> &CategoryScoreDetails {
        self.friendly_fire_vespene.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_friendly_fire_vespene_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.friendly_fire_vespene
    }

    fn mut_friendly_fire_vespene_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.friendly_fire_vespene
    }

    // optional .SC2APIProtocol.CategoryScoreDetails used_minerals = 20;

    pub fn clear_used_minerals(&mut self) {
        self.used_minerals.clear();
    }

    pub fn has_used_minerals(&self) -> bool {
        self.used_minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_used_minerals(&mut self, v: CategoryScoreDetails) {
        self.used_minerals = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_used_minerals(&mut self) -> &mut CategoryScoreDetails {
        if self.used_minerals.is_none() {
            self.used_minerals.set_default();
        }
        self.used_minerals.as_mut().unwrap()
    }

    // Take field
    pub fn take_used_minerals(&mut self) -> CategoryScoreDetails {
        self.used_minerals.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_used_minerals(&self) -> &CategoryScoreDetails {
        self.used_minerals.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_used_minerals_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.used_minerals
    }

    fn mut_used_minerals_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.used_minerals
    }

    // optional .SC2APIProtocol.CategoryScoreDetails used_vespene = 21;

    pub fn clear_used_vespene(&mut self) {
        self.used_vespene.clear();
    }

    pub fn has_used_vespene(&self) -> bool {
        self.used_vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_used_vespene(&mut self, v: CategoryScoreDetails) {
        self.used_vespene = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_used_vespene(&mut self) -> &mut CategoryScoreDetails {
        if self.used_vespene.is_none() {
            self.used_vespene.set_default();
        }
        self.used_vespene.as_mut().unwrap()
    }

    // Take field
    pub fn take_used_vespene(&mut self) -> CategoryScoreDetails {
        self.used_vespene.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_used_vespene(&self) -> &CategoryScoreDetails {
        self.used_vespene.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_used_vespene_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.used_vespene
    }

    fn mut_used_vespene_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.used_vespene
    }

    // optional .SC2APIProtocol.CategoryScoreDetails total_used_minerals = 22;

    pub fn clear_total_used_minerals(&mut self) {
        self.total_used_minerals.clear();
    }

    pub fn has_total_used_minerals(&self) -> bool {
        self.total_used_minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_used_minerals(&mut self, v: CategoryScoreDetails) {
        self.total_used_minerals = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_total_used_minerals(&mut self) -> &mut CategoryScoreDetails {
        if self.total_used_minerals.is_none() {
            self.total_used_minerals.set_default();
        }
        self.total_used_minerals.as_mut().unwrap()
    }

    // Take field
    pub fn take_total_used_minerals(&mut self) -> CategoryScoreDetails {
        self.total_used_minerals.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_total_used_minerals(&self) -> &CategoryScoreDetails {
        self.total_used_minerals.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_total_used_minerals_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.total_used_minerals
    }

    fn mut_total_used_minerals_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.total_used_minerals
    }

    // optional .SC2APIProtocol.CategoryScoreDetails total_used_vespene = 23;

    pub fn clear_total_used_vespene(&mut self) {
        self.total_used_vespene.clear();
    }

    pub fn has_total_used_vespene(&self) -> bool {
        self.total_used_vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_used_vespene(&mut self, v: CategoryScoreDetails) {
        self.total_used_vespene = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_total_used_vespene(&mut self) -> &mut CategoryScoreDetails {
        if self.total_used_vespene.is_none() {
            self.total_used_vespene.set_default();
        }
        self.total_used_vespene.as_mut().unwrap()
    }

    // Take field
    pub fn take_total_used_vespene(&mut self) -> CategoryScoreDetails {
        self.total_used_vespene.take().unwrap_or_else(|| CategoryScoreDetails::new())
    }

    pub fn get_total_used_vespene(&self) -> &CategoryScoreDetails {
        self.total_used_vespene.as_ref().unwrap_or_else(|| CategoryScoreDetails::default_instance())
    }

    fn get_total_used_vespene_for_reflect(&self) -> &::protobuf::SingularPtrField<CategoryScoreDetails> {
        &self.total_used_vespene
    }

    fn mut_total_used_vespene_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CategoryScoreDetails> {
        &mut self.total_used_vespene
    }

    // optional .SC2APIProtocol.VitalScoreDetails total_damage_dealt = 24;

    pub fn clear_total_damage_dealt(&mut self) {
        self.total_damage_dealt.clear();
    }

    pub fn has_total_damage_dealt(&self) -> bool {
        self.total_damage_dealt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_damage_dealt(&mut self, v: VitalScoreDetails) {
        self.total_damage_dealt = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_total_damage_dealt(&mut self) -> &mut VitalScoreDetails {
        if self.total_damage_dealt.is_none() {
            self.total_damage_dealt.set_default();
        }
        self.total_damage_dealt.as_mut().unwrap()
    }

    // Take field
    pub fn take_total_damage_dealt(&mut self) -> VitalScoreDetails {
        self.total_damage_dealt.take().unwrap_or_else(|| VitalScoreDetails::new())
    }

    pub fn get_total_damage_dealt(&self) -> &VitalScoreDetails {
        self.total_damage_dealt.as_ref().unwrap_or_else(|| VitalScoreDetails::default_instance())
    }

    fn get_total_damage_dealt_for_reflect(&self) -> &::protobuf::SingularPtrField<VitalScoreDetails> {
        &self.total_damage_dealt
    }

    fn mut_total_damage_dealt_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<VitalScoreDetails> {
        &mut self.total_damage_dealt
    }

    // optional .SC2APIProtocol.VitalScoreDetails total_damage_taken = 25;

    pub fn clear_total_damage_taken(&mut self) {
        self.total_damage_taken.clear();
    }

    pub fn has_total_damage_taken(&self) -> bool {
        self.total_damage_taken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_damage_taken(&mut self, v: VitalScoreDetails) {
        self.total_damage_taken = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_total_damage_taken(&mut self) -> &mut VitalScoreDetails {
        if self.total_damage_taken.is_none() {
            self.total_damage_taken.set_default();
        }
        self.total_damage_taken.as_mut().unwrap()
    }

    // Take field
    pub fn take_total_damage_taken(&mut self) -> VitalScoreDetails {
        self.total_damage_taken.take().unwrap_or_else(|| VitalScoreDetails::new())
    }

    pub fn get_total_damage_taken(&self) -> &VitalScoreDetails {
        self.total_damage_taken.as_ref().unwrap_or_else(|| VitalScoreDetails::default_instance())
    }

    fn get_total_damage_taken_for_reflect(&self) -> &::protobuf::SingularPtrField<VitalScoreDetails> {
        &self.total_damage_taken
    }

    fn mut_total_damage_taken_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<VitalScoreDetails> {
        &mut self.total_damage_taken
    }

    // optional .SC2APIProtocol.VitalScoreDetails total_healed = 26;

    pub fn clear_total_healed(&mut self) {
        self.total_healed.clear();
    }

    pub fn has_total_healed(&self) -> bool {
        self.total_healed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_healed(&mut self, v: VitalScoreDetails) {
        self.total_healed = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_total_healed(&mut self) -> &mut VitalScoreDetails {
        if self.total_healed.is_none() {
            self.total_healed.set_default();
        }
        self.total_healed.as_mut().unwrap()
    }

    // Take field
    pub fn take_total_healed(&mut self) -> VitalScoreDetails {
        self.total_healed.take().unwrap_or_else(|| VitalScoreDetails::new())
    }

    pub fn get_total_healed(&self) -> &VitalScoreDetails {
        self.total_healed.as_ref().unwrap_or_else(|| VitalScoreDetails::default_instance())
    }

    fn get_total_healed_for_reflect(&self) -> &::protobuf::SingularPtrField<VitalScoreDetails> {
        &self.total_healed
    }

    fn mut_total_healed_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<VitalScoreDetails> {
        &mut self.total_healed
    }
}

impl ::protobuf::Message for ScoreDetails {
    fn is_initialized(&self) -> bool {
        for v in &self.food_used {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.killed_minerals {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.killed_vespene {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lost_minerals {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lost_vespene {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.friendly_fire_minerals {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.friendly_fire_vespene {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.used_minerals {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.used_vespene {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.total_used_minerals {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.total_used_vespene {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.total_damage_dealt {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.total_damage_taken {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.total_healed {
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
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.idle_production_time = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.idle_worker_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.total_value_units = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.total_value_structures = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.killed_value_units = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.killed_value_structures = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.collected_minerals = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.collected_vespene = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.collection_rate_minerals = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.collection_rate_vespene = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.spent_minerals = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.spent_vespene = ::std::option::Option::Some(tmp);
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.food_used)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.killed_minerals)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.killed_vespene)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lost_minerals)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lost_vespene)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.friendly_fire_minerals)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.friendly_fire_vespene)?;
                },
                20 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.used_minerals)?;
                },
                21 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.used_vespene)?;
                },
                22 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.total_used_minerals)?;
                },
                23 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.total_used_vespene)?;
                },
                24 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.total_damage_dealt)?;
                },
                25 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.total_damage_taken)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.total_healed)?;
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
        if let Some(v) = self.idle_production_time {
            my_size += 5;
        }
        if let Some(v) = self.idle_worker_time {
            my_size += 5;
        }
        if let Some(v) = self.total_value_units {
            my_size += 5;
        }
        if let Some(v) = self.total_value_structures {
            my_size += 5;
        }
        if let Some(v) = self.killed_value_units {
            my_size += 5;
        }
        if let Some(v) = self.killed_value_structures {
            my_size += 5;
        }
        if let Some(v) = self.collected_minerals {
            my_size += 5;
        }
        if let Some(v) = self.collected_vespene {
            my_size += 5;
        }
        if let Some(v) = self.collection_rate_minerals {
            my_size += 5;
        }
        if let Some(v) = self.collection_rate_vespene {
            my_size += 5;
        }
        if let Some(v) = self.spent_minerals {
            my_size += 5;
        }
        if let Some(v) = self.spent_vespene {
            my_size += 5;
        }
        if let Some(ref v) = self.food_used.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.killed_minerals.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.killed_vespene.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.lost_minerals.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.lost_vespene.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.friendly_fire_minerals.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.friendly_fire_vespene.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.used_minerals.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.used_vespene.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.total_used_minerals.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.total_used_vespene.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.total_damage_dealt.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.total_damage_taken.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.total_healed.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.idle_production_time {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.idle_worker_time {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.total_value_units {
            os.write_float(3, v)?;
        }
        if let Some(v) = self.total_value_structures {
            os.write_float(4, v)?;
        }
        if let Some(v) = self.killed_value_units {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.killed_value_structures {
            os.write_float(6, v)?;
        }
        if let Some(v) = self.collected_minerals {
            os.write_float(7, v)?;
        }
        if let Some(v) = self.collected_vespene {
            os.write_float(8, v)?;
        }
        if let Some(v) = self.collection_rate_minerals {
            os.write_float(9, v)?;
        }
        if let Some(v) = self.collection_rate_vespene {
            os.write_float(10, v)?;
        }
        if let Some(v) = self.spent_minerals {
            os.write_float(11, v)?;
        }
        if let Some(v) = self.spent_vespene {
            os.write_float(12, v)?;
        }
        if let Some(ref v) = self.food_used.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.killed_minerals.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.killed_vespene.as_ref() {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.lost_minerals.as_ref() {
            os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.lost_vespene.as_ref() {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.friendly_fire_minerals.as_ref() {
            os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.friendly_fire_vespene.as_ref() {
            os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.used_minerals.as_ref() {
            os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.used_vespene.as_ref() {
            os.write_tag(21, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.total_used_minerals.as_ref() {
            os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.total_used_vespene.as_ref() {
            os.write_tag(23, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.total_damage_dealt.as_ref() {
            os.write_tag(24, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.total_damage_taken.as_ref() {
            os.write_tag(25, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.total_healed.as_ref() {
            os.write_tag(26, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ScoreDetails {
    fn new() -> ScoreDetails {
        ScoreDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScoreDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "idle_production_time",
                    ScoreDetails::get_idle_production_time_for_reflect,
                    ScoreDetails::mut_idle_production_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "idle_worker_time",
                    ScoreDetails::get_idle_worker_time_for_reflect,
                    ScoreDetails::mut_idle_worker_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "total_value_units",
                    ScoreDetails::get_total_value_units_for_reflect,
                    ScoreDetails::mut_total_value_units_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "total_value_structures",
                    ScoreDetails::get_total_value_structures_for_reflect,
                    ScoreDetails::mut_total_value_structures_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "killed_value_units",
                    ScoreDetails::get_killed_value_units_for_reflect,
                    ScoreDetails::mut_killed_value_units_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "killed_value_structures",
                    ScoreDetails::get_killed_value_structures_for_reflect,
                    ScoreDetails::mut_killed_value_structures_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "collected_minerals",
                    ScoreDetails::get_collected_minerals_for_reflect,
                    ScoreDetails::mut_collected_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "collected_vespene",
                    ScoreDetails::get_collected_vespene_for_reflect,
                    ScoreDetails::mut_collected_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "collection_rate_minerals",
                    ScoreDetails::get_collection_rate_minerals_for_reflect,
                    ScoreDetails::mut_collection_rate_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "collection_rate_vespene",
                    ScoreDetails::get_collection_rate_vespene_for_reflect,
                    ScoreDetails::mut_collection_rate_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "spent_minerals",
                    ScoreDetails::get_spent_minerals_for_reflect,
                    ScoreDetails::mut_spent_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "spent_vespene",
                    ScoreDetails::get_spent_vespene_for_reflect,
                    ScoreDetails::mut_spent_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "food_used",
                    ScoreDetails::get_food_used_for_reflect,
                    ScoreDetails::mut_food_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "killed_minerals",
                    ScoreDetails::get_killed_minerals_for_reflect,
                    ScoreDetails::mut_killed_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "killed_vespene",
                    ScoreDetails::get_killed_vespene_for_reflect,
                    ScoreDetails::mut_killed_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "lost_minerals",
                    ScoreDetails::get_lost_minerals_for_reflect,
                    ScoreDetails::mut_lost_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "lost_vespene",
                    ScoreDetails::get_lost_vespene_for_reflect,
                    ScoreDetails::mut_lost_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "friendly_fire_minerals",
                    ScoreDetails::get_friendly_fire_minerals_for_reflect,
                    ScoreDetails::mut_friendly_fire_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "friendly_fire_vespene",
                    ScoreDetails::get_friendly_fire_vespene_for_reflect,
                    ScoreDetails::mut_friendly_fire_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "used_minerals",
                    ScoreDetails::get_used_minerals_for_reflect,
                    ScoreDetails::mut_used_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "used_vespene",
                    ScoreDetails::get_used_vespene_for_reflect,
                    ScoreDetails::mut_used_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "total_used_minerals",
                    ScoreDetails::get_total_used_minerals_for_reflect,
                    ScoreDetails::mut_total_used_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CategoryScoreDetails>>(
                    "total_used_vespene",
                    ScoreDetails::get_total_used_vespene_for_reflect,
                    ScoreDetails::mut_total_used_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VitalScoreDetails>>(
                    "total_damage_dealt",
                    ScoreDetails::get_total_damage_dealt_for_reflect,
                    ScoreDetails::mut_total_damage_dealt_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VitalScoreDetails>>(
                    "total_damage_taken",
                    ScoreDetails::get_total_damage_taken_for_reflect,
                    ScoreDetails::mut_total_damage_taken_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VitalScoreDetails>>(
                    "total_healed",
                    ScoreDetails::get_total_healed_for_reflect,
                    ScoreDetails::mut_total_healed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScoreDetails>(
                    "ScoreDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScoreDetails {
    fn clear(&mut self) {
        self.clear_idle_production_time();
        self.clear_idle_worker_time();
        self.clear_total_value_units();
        self.clear_total_value_structures();
        self.clear_killed_value_units();
        self.clear_killed_value_structures();
        self.clear_collected_minerals();
        self.clear_collected_vespene();
        self.clear_collection_rate_minerals();
        self.clear_collection_rate_vespene();
        self.clear_spent_minerals();
        self.clear_spent_vespene();
        self.clear_food_used();
        self.clear_killed_minerals();
        self.clear_killed_vespene();
        self.clear_lost_minerals();
        self.clear_lost_vespene();
        self.clear_friendly_fire_minerals();
        self.clear_friendly_fire_vespene();
        self.clear_used_minerals();
        self.clear_used_vespene();
        self.clear_total_used_minerals();
        self.clear_total_used_vespene();
        self.clear_total_damage_dealt();
        self.clear_total_damage_taken();
        self.clear_total_healed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScoreDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScoreDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cs2clientprotocol/score.proto\x12\x0eSC2APIProtocol\"\xc8\x01\n\x05\
    Score\x12>\n\nscore_type\x18\x06\x20\x01(\x0e2\x1f.SC2APIProtocol.Score.\
    ScoreTypeR\tscoreType\x12\x14\n\x05score\x18\x07\x20\x01(\x05R\x05score\
    \x12A\n\rscore_details\x18\x08\x20\x01(\x0b2\x1c.SC2APIProtocol.ScoreDet\
    ailsR\x0cscoreDetails\"&\n\tScoreType\x12\x0e\n\nCurriculum\x10\x01\x12\
    \t\n\x05Melee\x10\x02\"\x92\x01\n\x14CategoryScoreDetails\x12\x12\n\x04n\
    one\x18\x01\x20\x01(\x02R\x04none\x12\x12\n\x04army\x18\x02\x20\x01(\x02\
    R\x04army\x12\x18\n\x07economy\x18\x03\x20\x01(\x02R\x07economy\x12\x1e\
    \n\ntechnology\x18\x04\x20\x01(\x02R\ntechnology\x12\x18\n\x07upgrade\
    \x18\x05\x20\x01(\x02R\x07upgrade\"Y\n\x11VitalScoreDetails\x12\x12\n\
    \x04life\x18\x01\x20\x01(\x02R\x04life\x12\x18\n\x07shields\x18\x02\x20\
    \x01(\x02R\x07shields\x12\x16\n\x06energy\x18\x03\x20\x01(\x02R\x06energ\
    y\"\x9b\r\n\x0cScoreDetails\x120\n\x14idle_production_time\x18\x01\x20\
    \x01(\x02R\x12idleProductionTime\x12(\n\x10idle_worker_time\x18\x02\x20\
    \x01(\x02R\x0eidleWorkerTime\x12*\n\x11total_value_units\x18\x03\x20\x01\
    (\x02R\x0ftotalValueUnits\x124\n\x16total_value_structures\x18\x04\x20\
    \x01(\x02R\x14totalValueStructures\x12,\n\x12killed_value_units\x18\x05\
    \x20\x01(\x02R\x10killedValueUnits\x126\n\x17killed_value_structures\x18\
    \x06\x20\x01(\x02R\x15killedValueStructures\x12-\n\x12collected_minerals\
    \x18\x07\x20\x01(\x02R\x11collectedMinerals\x12+\n\x11collected_vespene\
    \x18\x08\x20\x01(\x02R\x10collectedVespene\x128\n\x18collection_rate_min\
    erals\x18\t\x20\x01(\x02R\x16collectionRateMinerals\x126\n\x17collection\
    _rate_vespene\x18\n\x20\x01(\x02R\x15collectionRateVespene\x12%\n\x0espe\
    nt_minerals\x18\x0b\x20\x01(\x02R\rspentMinerals\x12#\n\rspent_vespene\
    \x18\x0c\x20\x01(\x02R\x0cspentVespene\x12A\n\tfood_used\x18\r\x20\x01(\
    \x0b2$.SC2APIProtocol.CategoryScoreDetailsR\x08foodUsed\x12M\n\x0fkilled\
    _minerals\x18\x0e\x20\x01(\x0b2$.SC2APIProtocol.CategoryScoreDetailsR\
    \x0ekilledMinerals\x12K\n\x0ekilled_vespene\x18\x0f\x20\x01(\x0b2$.SC2AP\
    IProtocol.CategoryScoreDetailsR\rkilledVespene\x12I\n\rlost_minerals\x18\
    \x10\x20\x01(\x0b2$.SC2APIProtocol.CategoryScoreDetailsR\x0clostMinerals\
    \x12G\n\x0clost_vespene\x18\x11\x20\x01(\x0b2$.SC2APIProtocol.CategorySc\
    oreDetailsR\x0blostVespene\x12Z\n\x16friendly_fire_minerals\x18\x12\x20\
    \x01(\x0b2$.SC2APIProtocol.CategoryScoreDetailsR\x14friendlyFireMinerals\
    \x12X\n\x15friendly_fire_vespene\x18\x13\x20\x01(\x0b2$.SC2APIProtocol.C\
    ategoryScoreDetailsR\x13friendlyFireVespene\x12I\n\rused_minerals\x18\
    \x14\x20\x01(\x0b2$.SC2APIProtocol.CategoryScoreDetailsR\x0cusedMinerals\
    \x12G\n\x0cused_vespene\x18\x15\x20\x01(\x0b2$.SC2APIProtocol.CategorySc\
    oreDetailsR\x0busedVespene\x12T\n\x13total_used_minerals\x18\x16\x20\x01\
    (\x0b2$.SC2APIProtocol.CategoryScoreDetailsR\x11totalUsedMinerals\x12R\n\
    \x12total_used_vespene\x18\x17\x20\x01(\x0b2$.SC2APIProtocol.CategorySco\
    reDetailsR\x10totalUsedVespene\x12O\n\x12total_damage_dealt\x18\x18\x20\
    \x01(\x0b2!.SC2APIProtocol.VitalScoreDetailsR\x10totalDamageDealt\x12O\n\
    \x12total_damage_taken\x18\x19\x20\x01(\x0b2!.SC2APIProtocol.VitalScoreD\
    etailsR\x10totalDamageTaken\x12D\n\x0ctotal_healed\x18\x1a\x20\x01(\x0b2\
    !.SC2APIProtocol.VitalScoreDetailsR\x0btotalHealedJ\xc2\x1c\n\x06\x12\
    \x04\x01\0H\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\x08\n\x01\x02\x12\
    \x03\x03\x08\x16\n\n\n\x02\x04\0\x12\x04\x05\0\x0e\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x05\x08\r\n\x0c\n\x04\x04\0\x04\0\x12\x04\x06\x02\t\x03\n\
    \x0c\n\x05\x04\0\x04\0\x01\x12\x03\x06\x07\x10\nP\n\x06\x04\0\x04\0\x02\
    \0\x12\x03\x07\x04\x13\"A\x20map\x20generated\x20score\x20(from\x20curri\
    culum\x20maps\x20with\x20special\x20scoring)\n\n\x0e\n\x07\x04\0\x04\0\
    \x02\0\x01\x12\x03\x07\x04\x0e\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\x12\x03\
    \x07\x11\x12\n`\n\x06\x04\0\x04\0\x02\x01\x12\x03\x08\x04\x0e\"Q\x20summ\
    ation\x20of\x20in-progress\x20and\x20current\x20units/buildings\x20value\
    \x20+\x20minerals\x20+\x20vespene\n\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\
    \x12\x03\x08\x04\t\n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03\x08\x0c\r\
    \n\x0b\n\x04\x04\0\x02\0\x12\x03\x0b\x02$\n\x0c\n\x05\x04\0\x02\0\x04\
    \x12\x03\x0b\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x0b\x0b\x14\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\x0b\x15\x1f\n\x0c\n\x05\x04\0\x02\0\x03\
    \x12\x03\x0b\"#\n_\n\x04\x04\0\x02\x01\x12\x03\x0c\x02\x1b\"R\x20Note:\
    \x20check\x20score_type\x20to\x20know\x20whether\x20this\x20is\x20a\x20m\
    elee\x20score\x20or\x20curriculum\x20score\n\n\x0c\n\x05\x04\0\x02\x01\
    \x04\x12\x03\x0c\x02\n\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x0c\x0b\x10\
    \n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\x11\x16\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x0c\x19\x1a\n\x0b\n\x04\x04\0\x02\x02\x12\x03\r\x02*\n\
    \x0c\n\x05\x04\0\x02\x02\x04\x12\x03\r\x02\n\n\x0c\n\x05\x04\0\x02\x02\
    \x06\x12\x03\r\x0b\x17\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\r\x18%\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03\r()\n\n\n\x02\x04\x01\x12\x04\x10\0\
    \x16\x01\n\n\n\x03\x04\x01\x01\x12\x03\x10\x08\x1c\nE\n\x04\x04\x01\x02\
    \0\x12\x03\x11\x02\x1a\"8\x20Used\x20when\x20no\x20other\x20category\x20\
    is\x20configured\x20in\x20game\x20data\n\n\x0c\n\x05\x04\x01\x02\0\x04\
    \x12\x03\x11\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x11\x0b\x10\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x11\x11\x15\n\x0c\n\x05\x04\x01\x02\
    \0\x03\x12\x03\x11\x18\x19\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x12\x02\
    \x1a\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03\x12\x02\n\n\x0c\n\x05\x04\
    \x01\x02\x01\x05\x12\x03\x12\x0b\x10\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\
    \x03\x12\x11\x15\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x12\x18\x19\n\
    \x0b\n\x04\x04\x01\x02\x02\x12\x03\x13\x02\x1d\n\x0c\n\x05\x04\x01\x02\
    \x02\x04\x12\x03\x13\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x13\
    \x0b\x10\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x13\x11\x18\n\x0c\n\x05\
    \x04\x01\x02\x02\x03\x12\x03\x13\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x03\x12\
    \x03\x14\x02\x20\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03\x14\x02\n\n\x0c\
    \n\x05\x04\x01\x02\x03\x05\x12\x03\x14\x0b\x10\n\x0c\n\x05\x04\x01\x02\
    \x03\x01\x12\x03\x14\x11\x1b\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x14\
    \x1e\x1f\n\x0b\n\x04\x04\x01\x02\x04\x12\x03\x15\x02\x1d\n\x0c\n\x05\x04\
    \x01\x02\x04\x04\x12\x03\x15\x02\n\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\
    \x03\x15\x0b\x10\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\x15\x11\x18\n\
    \x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x15\x1b\x1c\n\n\n\x02\x04\x02\x12\
    \x04\x18\0\x1c\x01\n\n\n\x03\x04\x02\x01\x12\x03\x18\x08\x19\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03\x19\x02\x1a\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\
    \x19\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x19\x0b\x10\n\x0c\n\x05\
    \x04\x02\x02\0\x01\x12\x03\x19\x11\x15\n\x0c\n\x05\x04\x02\x02\0\x03\x12\
    \x03\x19\x18\x19\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x1a\x02\x1d\n\x0c\n\
    \x05\x04\x02\x02\x01\x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\x02\x02\x01\
    \x05\x12\x03\x1a\x0b\x10\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1a\x11\
    \x18\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x1a\x1b\x1c\n\x0b\n\x04\x04\
    \x02\x02\x02\x12\x03\x1b\x02\x1c\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03\
    \x1b\x02\n\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03\x1b\x0b\x10\n\x0c\n\
    \x05\x04\x02\x02\x02\x01\x12\x03\x1b\x11\x17\n\x0c\n\x05\x04\x02\x02\x02\
    \x03\x12\x03\x1b\x1a\x1b\n\n\n\x02\x04\x03\x12\x04\x1e\0H\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03\x1e\x08\x14\n%\n\x04\x04\x03\x02\0\x12\x03\x1f\x02*\
    \"\x18\x20Interesting\x20as\x20a\x20delta\n\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03\x1f\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x1f\x0b\x10\
    \n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x1f\x11%\n\x0c\n\x05\x04\x03\x02\
    \0\x03\x12\x03\x1f()\n%\n\x04\x04\x03\x02\x01\x12\x03\x20\x02&\"\x18\x20\
    Interesting\x20as\x20a\x20delta\n\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\
    \x03\x20\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x20\x0b\x10\n\x0c\
    \n\x05\x04\x03\x02\x01\x01\x12\x03\x20\x11!\n\x0c\n\x05\x04\x03\x02\x01\
    \x03\x12\x03\x20$%\n\x88\x01\n\x04\x04\x03\x02\x02\x12\x03#\x02'\x1a{\
    \x20Note\x20the\x20\"total_value\"\x20fields\x20are\x20a\x20combination\
    \x20of\x20minerals,\x20vespene\x20and\x20a\x20human\x20designer\x20guess\
    .\x20Maybe\x20useful\x20as\x20a\x20delta.\n\n\x0c\n\x05\x04\x03\x02\x02\
    \x04\x12\x03#\x02\n\n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x03#\x0b\x10\n\
    \x0c\n\x05\x04\x03\x02\x02\x01\x12\x03#\x11\"\n\x0c\n\x05\x04\x03\x02\
    \x02\x03\x12\x03#%&\n\x0b\n\x04\x04\x03\x02\x03\x12\x03$\x02,\n\x0c\n\
    \x05\x04\x03\x02\x03\x04\x12\x03$\x02\n\n\x0c\n\x05\x04\x03\x02\x03\x05\
    \x12\x03$\x0b\x10\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03$\x11'\n\x0c\n\
    \x05\x04\x03\x02\x03\x03\x12\x03$*+\n\x85\x02\n\x04\x04\x03\x02\x04\x12\
    \x03(\x02(\x1a\xf7\x01\x20Note\x20the\x20\"killed_value\"\x20fields\x20a\
    re\x20a\x20combination\x20of\x20minerals,\x20vespene\x20and\x20a\x20huma\
    n\x20designer\x20guess.\x20Maybe\x20useful\x20as\x20a\x20delta.\n\x20The\
    \x20weighting\x20of\x20the\x20combination\x20and\x20the\x20human\x20desi\
    gner\x20guess\x20is\x20different\x20(not\x20symmetric)\x20with\x20the\
    \x20\"total_value\"\x20fields!\n\n\x0c\n\x05\x04\x03\x02\x04\x04\x12\x03\
    (\x02\n\n\x0c\n\x05\x04\x03\x02\x04\x05\x12\x03(\x0b\x10\n\x0c\n\x05\x04\
    \x03\x02\x04\x01\x12\x03(\x11#\n\x0c\n\x05\x04\x03\x02\x04\x03\x12\x03(&\
    '\n\x0b\n\x04\x04\x03\x02\x05\x12\x03)\x02-\n\x0c\n\x05\x04\x03\x02\x05\
    \x04\x12\x03)\x02\n\n\x0c\n\x05\x04\x03\x02\x05\x05\x12\x03)\x0b\x10\n\
    \x0c\n\x05\x04\x03\x02\x05\x01\x12\x03)\x11(\n\x0c\n\x05\x04\x03\x02\x05\
    \x03\x12\x03)+,\n\x0b\n\x04\x04\x03\x02\x06\x12\x03+\x02(\n\x0c\n\x05\
    \x04\x03\x02\x06\x04\x12\x03+\x02\n\n\x0c\n\x05\x04\x03\x02\x06\x05\x12\
    \x03+\x0b\x10\n\x0c\n\x05\x04\x03\x02\x06\x01\x12\x03+\x11#\n\x0c\n\x05\
    \x04\x03\x02\x06\x03\x12\x03+&'\n\x0b\n\x04\x04\x03\x02\x07\x12\x03,\x02\
    '\n\x0c\n\x05\x04\x03\x02\x07\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\x03\x02\
    \x07\x05\x12\x03,\x0b\x10\n\x0c\n\x05\x04\x03\x02\x07\x01\x12\x03,\x11\"\
    \n\x0c\n\x05\x04\x03\x02\x07\x03\x12\x03,%&\n\x0b\n\x04\x04\x03\x02\x08\
    \x12\x03.\x02.\n\x0c\n\x05\x04\x03\x02\x08\x04\x12\x03.\x02\n\n\x0c\n\
    \x05\x04\x03\x02\x08\x05\x12\x03.\x0b\x10\n\x0c\n\x05\x04\x03\x02\x08\
    \x01\x12\x03.\x11)\n\x0c\n\x05\x04\x03\x02\x08\x03\x12\x03.,-\n\x0b\n\
    \x04\x04\x03\x02\t\x12\x03/\x02.\n\x0c\n\x05\x04\x03\x02\t\x04\x12\x03/\
    \x02\n\n\x0c\n\x05\x04\x03\x02\t\x05\x12\x03/\x0b\x10\n\x0c\n\x05\x04\
    \x03\x02\t\x01\x12\x03/\x11(\n\x0c\n\x05\x04\x03\x02\t\x03\x12\x03/+-\n\
    \x0b\n\x04\x04\x03\x02\n\x12\x031\x02%\n\x0c\n\x05\x04\x03\x02\n\x04\x12\
    \x031\x02\n\n\x0c\n\x05\x04\x03\x02\n\x05\x12\x031\x0b\x10\n\x0c\n\x05\
    \x04\x03\x02\n\x01\x12\x031\x11\x1f\n\x0c\n\x05\x04\x03\x02\n\x03\x12\
    \x031\"$\n\x0b\n\x04\x04\x03\x02\x0b\x12\x032\x02$\n\x0c\n\x05\x04\x03\
    \x02\x0b\x04\x12\x032\x02\n\n\x0c\n\x05\x04\x03\x02\x0b\x05\x12\x032\x0b\
    \x10\n\x0c\n\x05\x04\x03\x02\x0b\x01\x12\x032\x11\x1e\n\x0c\n\x05\x04\
    \x03\x02\x0b\x03\x12\x032!#\n\x0b\n\x04\x04\x03\x02\x0c\x12\x034\x02/\n\
    \x0c\n\x05\x04\x03\x02\x0c\x04\x12\x034\x02\n\n\x0c\n\x05\x04\x03\x02\
    \x0c\x06\x12\x034\x0b\x1f\n\x0c\n\x05\x04\x03\x02\x0c\x01\x12\x034\x20)\
    \n\x0c\n\x05\x04\x03\x02\x0c\x03\x12\x034,.\n\x0b\n\x04\x04\x03\x02\r\
    \x12\x036\x025\n\x0c\n\x05\x04\x03\x02\r\x04\x12\x036\x02\n\n\x0c\n\x05\
    \x04\x03\x02\r\x06\x12\x036\x0b\x1f\n\x0c\n\x05\x04\x03\x02\r\x01\x12\
    \x036\x20/\n\x0c\n\x05\x04\x03\x02\r\x03\x12\x03624\n\x0b\n\x04\x04\x03\
    \x02\x0e\x12\x037\x024\n\x0c\n\x05\x04\x03\x02\x0e\x04\x12\x037\x02\n\n\
    \x0c\n\x05\x04\x03\x02\x0e\x06\x12\x037\x0b\x1f\n\x0c\n\x05\x04\x03\x02\
    \x0e\x01\x12\x037\x20.\n\x0c\n\x05\x04\x03\x02\x0e\x03\x12\x03713\n\x0b\
    \n\x04\x04\x03\x02\x0f\x12\x039\x023\n\x0c\n\x05\x04\x03\x02\x0f\x04\x12\
    \x039\x02\n\n\x0c\n\x05\x04\x03\x02\x0f\x06\x12\x039\x0b\x1f\n\x0c\n\x05\
    \x04\x03\x02\x0f\x01\x12\x039\x20-\n\x0c\n\x05\x04\x03\x02\x0f\x03\x12\
    \x03902\n\x0b\n\x04\x04\x03\x02\x10\x12\x03:\x022\n\x0c\n\x05\x04\x03\
    \x02\x10\x04\x12\x03:\x02\n\n\x0c\n\x05\x04\x03\x02\x10\x06\x12\x03:\x0b\
    \x1f\n\x0c\n\x05\x04\x03\x02\x10\x01\x12\x03:\x20,\n\x0c\n\x05\x04\x03\
    \x02\x10\x03\x12\x03:/1\n\x0b\n\x04\x04\x03\x02\x11\x12\x03<\x02<\n\x0c\
    \n\x05\x04\x03\x02\x11\x04\x12\x03<\x02\n\n\x0c\n\x05\x04\x03\x02\x11\
    \x06\x12\x03<\x0b\x1f\n\x0c\n\x05\x04\x03\x02\x11\x01\x12\x03<\x206\n\
    \x0c\n\x05\x04\x03\x02\x11\x03\x12\x03<9;\n\x0b\n\x04\x04\x03\x02\x12\
    \x12\x03=\x02;\n\x0c\n\x05\x04\x03\x02\x12\x04\x12\x03=\x02\n\n\x0c\n\
    \x05\x04\x03\x02\x12\x06\x12\x03=\x0b\x1f\n\x0c\n\x05\x04\x03\x02\x12\
    \x01\x12\x03=\x205\n\x0c\n\x05\x04\x03\x02\x12\x03\x12\x03=8:\n\x0b\n\
    \x04\x04\x03\x02\x13\x12\x03?\x023\n\x0c\n\x05\x04\x03\x02\x13\x04\x12\
    \x03?\x02\n\n\x0c\n\x05\x04\x03\x02\x13\x06\x12\x03?\x0b\x1f\n\x0c\n\x05\
    \x04\x03\x02\x13\x01\x12\x03?\x20-\n\x0c\n\x05\x04\x03\x02\x13\x03\x12\
    \x03?02\n\x0b\n\x04\x04\x03\x02\x14\x12\x03@\x022\n\x0c\n\x05\x04\x03\
    \x02\x14\x04\x12\x03@\x02\n\n\x0c\n\x05\x04\x03\x02\x14\x06\x12\x03@\x0b\
    \x1f\n\x0c\n\x05\x04\x03\x02\x14\x01\x12\x03@\x20,\n\x0c\n\x05\x04\x03\
    \x02\x14\x03\x12\x03@/1\n%\n\x04\x04\x03\x02\x15\x12\x03B\x029\"\x18\x20\
    Interesting\x20as\x20a\x20delta\n\n\x0c\n\x05\x04\x03\x02\x15\x04\x12\
    \x03B\x02\n\n\x0c\n\x05\x04\x03\x02\x15\x06\x12\x03B\x0b\x1f\n\x0c\n\x05\
    \x04\x03\x02\x15\x01\x12\x03B\x203\n\x0c\n\x05\x04\x03\x02\x15\x03\x12\
    \x03B68\n%\n\x04\x04\x03\x02\x16\x12\x03C\x028\"\x18\x20Interesting\x20a\
    s\x20a\x20delta\n\n\x0c\n\x05\x04\x03\x02\x16\x04\x12\x03C\x02\n\n\x0c\n\
    \x05\x04\x03\x02\x16\x06\x12\x03C\x0b\x1f\n\x0c\n\x05\x04\x03\x02\x16\
    \x01\x12\x03C\x202\n\x0c\n\x05\x04\x03\x02\x16\x03\x12\x03C57\n%\n\x04\
    \x04\x03\x02\x17\x12\x03E\x025\"\x18\x20Interesting\x20as\x20a\x20delta\
    \n\n\x0c\n\x05\x04\x03\x02\x17\x04\x12\x03E\x02\n\n\x0c\n\x05\x04\x03\
    \x02\x17\x06\x12\x03E\x0b\x1c\n\x0c\n\x05\x04\x03\x02\x17\x01\x12\x03E\
    \x1d/\n\x0c\n\x05\x04\x03\x02\x17\x03\x12\x03E24\n%\n\x04\x04\x03\x02\
    \x18\x12\x03F\x025\"\x18\x20Interesting\x20as\x20a\x20delta\n\n\x0c\n\
    \x05\x04\x03\x02\x18\x04\x12\x03F\x02\n\n\x0c\n\x05\x04\x03\x02\x18\x06\
    \x12\x03F\x0b\x1c\n\x0c\n\x05\x04\x03\x02\x18\x01\x12\x03F\x1d/\n\x0c\n\
    \x05\x04\x03\x02\x18\x03\x12\x03F24\n%\n\x04\x04\x03\x02\x19\x12\x03G\
    \x02/\"\x18\x20Interesting\x20as\x20a\x20delta\n\n\x0c\n\x05\x04\x03\x02\
    \x19\x04\x12\x03G\x02\n\n\x0c\n\x05\x04\x03\x02\x19\x06\x12\x03G\x0b\x1c\
    \n\x0c\n\x05\x04\x03\x02\x19\x01\x12\x03G\x1d)\n\x0c\n\x05\x04\x03\x02\
    \x19\x03\x12\x03G,.\
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
