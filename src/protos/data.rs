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
pub struct AbilityData {
    // message fields
    ability_id: ::std::option::Option<u32>,
    link_name: ::protobuf::SingularField<::std::string::String>,
    link_index: ::std::option::Option<u32>,
    button_name: ::protobuf::SingularField<::std::string::String>,
    friendly_name: ::protobuf::SingularField<::std::string::String>,
    hotkey: ::protobuf::SingularField<::std::string::String>,
    remaps_to_ability_id: ::std::option::Option<u32>,
    available: ::std::option::Option<bool>,
    target: ::std::option::Option<AbilityData_Target>,
    allow_minimap: ::std::option::Option<bool>,
    allow_autocast: ::std::option::Option<bool>,
    is_building: ::std::option::Option<bool>,
    footprint_radius: ::std::option::Option<f32>,
    is_instant_placement: ::std::option::Option<bool>,
    cast_range: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AbilityData {}

impl AbilityData {
    pub fn new() -> AbilityData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AbilityData {
        static mut instance: ::protobuf::lazy::Lazy<AbilityData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AbilityData,
        };
        unsafe {
            instance.get(AbilityData::new)
        }
    }

    // optional uint32 ability_id = 1;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }

    // optional string link_name = 2;

    pub fn clear_link_name(&mut self) {
        self.link_name.clear();
    }

    pub fn has_link_name(&self) -> bool {
        self.link_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_link_name(&mut self, v: ::std::string::String) {
        self.link_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_link_name(&mut self) -> &mut ::std::string::String {
        if self.link_name.is_none() {
            self.link_name.set_default();
        }
        self.link_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_link_name(&mut self) -> ::std::string::String {
        self.link_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_link_name(&self) -> &str {
        match self.link_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_link_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.link_name
    }

    fn mut_link_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.link_name
    }

    // optional uint32 link_index = 3;

    pub fn clear_link_index(&mut self) {
        self.link_index = ::std::option::Option::None;
    }

    pub fn has_link_index(&self) -> bool {
        self.link_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_link_index(&mut self, v: u32) {
        self.link_index = ::std::option::Option::Some(v);
    }

    pub fn get_link_index(&self) -> u32 {
        self.link_index.unwrap_or(0)
    }

    fn get_link_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.link_index
    }

    fn mut_link_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.link_index
    }

    // optional string button_name = 4;

    pub fn clear_button_name(&mut self) {
        self.button_name.clear();
    }

    pub fn has_button_name(&self) -> bool {
        self.button_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_button_name(&mut self, v: ::std::string::String) {
        self.button_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_button_name(&mut self) -> &mut ::std::string::String {
        if self.button_name.is_none() {
            self.button_name.set_default();
        }
        self.button_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_button_name(&mut self) -> ::std::string::String {
        self.button_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_button_name(&self) -> &str {
        match self.button_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_button_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.button_name
    }

    fn mut_button_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.button_name
    }

    // optional string friendly_name = 5;

    pub fn clear_friendly_name(&mut self) {
        self.friendly_name.clear();
    }

    pub fn has_friendly_name(&self) -> bool {
        self.friendly_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friendly_name(&mut self, v: ::std::string::String) {
        self.friendly_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_friendly_name(&mut self) -> &mut ::std::string::String {
        if self.friendly_name.is_none() {
            self.friendly_name.set_default();
        }
        self.friendly_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_friendly_name(&mut self) -> ::std::string::String {
        self.friendly_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_friendly_name(&self) -> &str {
        match self.friendly_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_friendly_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.friendly_name
    }

    fn mut_friendly_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.friendly_name
    }

    // optional string hotkey = 6;

    pub fn clear_hotkey(&mut self) {
        self.hotkey.clear();
    }

    pub fn has_hotkey(&self) -> bool {
        self.hotkey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hotkey(&mut self, v: ::std::string::String) {
        self.hotkey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hotkey(&mut self) -> &mut ::std::string::String {
        if self.hotkey.is_none() {
            self.hotkey.set_default();
        }
        self.hotkey.as_mut().unwrap()
    }

    // Take field
    pub fn take_hotkey(&mut self) -> ::std::string::String {
        self.hotkey.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hotkey(&self) -> &str {
        match self.hotkey.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hotkey_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hotkey
    }

    fn mut_hotkey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hotkey
    }

    // optional uint32 remaps_to_ability_id = 7;

    pub fn clear_remaps_to_ability_id(&mut self) {
        self.remaps_to_ability_id = ::std::option::Option::None;
    }

    pub fn has_remaps_to_ability_id(&self) -> bool {
        self.remaps_to_ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaps_to_ability_id(&mut self, v: u32) {
        self.remaps_to_ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_remaps_to_ability_id(&self) -> u32 {
        self.remaps_to_ability_id.unwrap_or(0)
    }

    fn get_remaps_to_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.remaps_to_ability_id
    }

    fn mut_remaps_to_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.remaps_to_ability_id
    }

    // optional bool available = 8;

    pub fn clear_available(&mut self) {
        self.available = ::std::option::Option::None;
    }

    pub fn has_available(&self) -> bool {
        self.available.is_some()
    }

    // Param is passed by value, moved
    pub fn set_available(&mut self, v: bool) {
        self.available = ::std::option::Option::Some(v);
    }

    pub fn get_available(&self) -> bool {
        self.available.unwrap_or(false)
    }

    fn get_available_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.available
    }

    fn mut_available_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.available
    }

    // optional .SC2APIProtocol.AbilityData.Target target = 9;

    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: AbilityData_Target) {
        self.target = ::std::option::Option::Some(v);
    }

    pub fn get_target(&self) -> AbilityData_Target {
        self.target.unwrap_or(AbilityData_Target::None)
    }

    fn get_target_for_reflect(&self) -> &::std::option::Option<AbilityData_Target> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::std::option::Option<AbilityData_Target> {
        &mut self.target
    }

    // optional bool allow_minimap = 10;

    pub fn clear_allow_minimap(&mut self) {
        self.allow_minimap = ::std::option::Option::None;
    }

    pub fn has_allow_minimap(&self) -> bool {
        self.allow_minimap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_minimap(&mut self, v: bool) {
        self.allow_minimap = ::std::option::Option::Some(v);
    }

    pub fn get_allow_minimap(&self) -> bool {
        self.allow_minimap.unwrap_or(false)
    }

    fn get_allow_minimap_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_minimap
    }

    fn mut_allow_minimap_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_minimap
    }

    // optional bool allow_autocast = 11;

    pub fn clear_allow_autocast(&mut self) {
        self.allow_autocast = ::std::option::Option::None;
    }

    pub fn has_allow_autocast(&self) -> bool {
        self.allow_autocast.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_autocast(&mut self, v: bool) {
        self.allow_autocast = ::std::option::Option::Some(v);
    }

    pub fn get_allow_autocast(&self) -> bool {
        self.allow_autocast.unwrap_or(false)
    }

    fn get_allow_autocast_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_autocast
    }

    fn mut_allow_autocast_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_autocast
    }

    // optional bool is_building = 12;

    pub fn clear_is_building(&mut self) {
        self.is_building = ::std::option::Option::None;
    }

    pub fn has_is_building(&self) -> bool {
        self.is_building.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_building(&mut self, v: bool) {
        self.is_building = ::std::option::Option::Some(v);
    }

    pub fn get_is_building(&self) -> bool {
        self.is_building.unwrap_or(false)
    }

    fn get_is_building_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_building
    }

    fn mut_is_building_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_building
    }

    // optional float footprint_radius = 13;

    pub fn clear_footprint_radius(&mut self) {
        self.footprint_radius = ::std::option::Option::None;
    }

    pub fn has_footprint_radius(&self) -> bool {
        self.footprint_radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_footprint_radius(&mut self, v: f32) {
        self.footprint_radius = ::std::option::Option::Some(v);
    }

    pub fn get_footprint_radius(&self) -> f32 {
        self.footprint_radius.unwrap_or(0.)
    }

    fn get_footprint_radius_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.footprint_radius
    }

    fn mut_footprint_radius_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.footprint_radius
    }

    // optional bool is_instant_placement = 14;

    pub fn clear_is_instant_placement(&mut self) {
        self.is_instant_placement = ::std::option::Option::None;
    }

    pub fn has_is_instant_placement(&self) -> bool {
        self.is_instant_placement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_instant_placement(&mut self, v: bool) {
        self.is_instant_placement = ::std::option::Option::Some(v);
    }

    pub fn get_is_instant_placement(&self) -> bool {
        self.is_instant_placement.unwrap_or(false)
    }

    fn get_is_instant_placement_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_instant_placement
    }

    fn mut_is_instant_placement_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_instant_placement
    }

    // optional float cast_range = 15;

    pub fn clear_cast_range(&mut self) {
        self.cast_range = ::std::option::Option::None;
    }

    pub fn has_cast_range(&self) -> bool {
        self.cast_range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cast_range(&mut self, v: f32) {
        self.cast_range = ::std::option::Option::Some(v);
    }

    pub fn get_cast_range(&self) -> f32 {
        self.cast_range.unwrap_or(0.)
    }

    fn get_cast_range_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.cast_range
    }

    fn mut_cast_range_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.cast_range
    }
}

impl ::protobuf::Message for AbilityData {
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
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.link_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.link_index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.button_name)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.friendly_name)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hotkey)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.remaps_to_ability_id = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.available = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.target = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allow_minimap = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allow_autocast = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_building = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.footprint_radius = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_instant_placement = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.cast_range = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.link_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.link_index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.button_name.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.friendly_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.hotkey.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.remaps_to_ability_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.available {
            my_size += 2;
        }
        if let Some(v) = self.target {
            my_size += ::protobuf::rt::enum_size(9, v);
        }
        if let Some(v) = self.allow_minimap {
            my_size += 2;
        }
        if let Some(v) = self.allow_autocast {
            my_size += 2;
        }
        if let Some(v) = self.is_building {
            my_size += 2;
        }
        if let Some(v) = self.footprint_radius {
            my_size += 5;
        }
        if let Some(v) = self.is_instant_placement {
            my_size += 2;
        }
        if let Some(v) = self.cast_range {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.link_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.link_index {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.button_name.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.friendly_name.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.hotkey.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.remaps_to_ability_id {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.available {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.target {
            os.write_enum(9, v.value())?;
        }
        if let Some(v) = self.allow_minimap {
            os.write_bool(10, v)?;
        }
        if let Some(v) = self.allow_autocast {
            os.write_bool(11, v)?;
        }
        if let Some(v) = self.is_building {
            os.write_bool(12, v)?;
        }
        if let Some(v) = self.footprint_radius {
            os.write_float(13, v)?;
        }
        if let Some(v) = self.is_instant_placement {
            os.write_bool(14, v)?;
        }
        if let Some(v) = self.cast_range {
            os.write_float(15, v)?;
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

impl ::protobuf::MessageStatic for AbilityData {
    fn new() -> AbilityData {
        AbilityData::new()
    }

    fn descriptor_static(_: ::std::option::Option<AbilityData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    AbilityData::get_ability_id_for_reflect,
                    AbilityData::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "link_name",
                    AbilityData::get_link_name_for_reflect,
                    AbilityData::mut_link_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "link_index",
                    AbilityData::get_link_index_for_reflect,
                    AbilityData::mut_link_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "button_name",
                    AbilityData::get_button_name_for_reflect,
                    AbilityData::mut_button_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "friendly_name",
                    AbilityData::get_friendly_name_for_reflect,
                    AbilityData::mut_friendly_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hotkey",
                    AbilityData::get_hotkey_for_reflect,
                    AbilityData::mut_hotkey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "remaps_to_ability_id",
                    AbilityData::get_remaps_to_ability_id_for_reflect,
                    AbilityData::mut_remaps_to_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "available",
                    AbilityData::get_available_for_reflect,
                    AbilityData::mut_available_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AbilityData_Target>>(
                    "target",
                    AbilityData::get_target_for_reflect,
                    AbilityData::mut_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_minimap",
                    AbilityData::get_allow_minimap_for_reflect,
                    AbilityData::mut_allow_minimap_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_autocast",
                    AbilityData::get_allow_autocast_for_reflect,
                    AbilityData::mut_allow_autocast_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_building",
                    AbilityData::get_is_building_for_reflect,
                    AbilityData::mut_is_building_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "footprint_radius",
                    AbilityData::get_footprint_radius_for_reflect,
                    AbilityData::mut_footprint_radius_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_instant_placement",
                    AbilityData::get_is_instant_placement_for_reflect,
                    AbilityData::mut_is_instant_placement_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "cast_range",
                    AbilityData::get_cast_range_for_reflect,
                    AbilityData::mut_cast_range_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AbilityData>(
                    "AbilityData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AbilityData {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.clear_link_name();
        self.clear_link_index();
        self.clear_button_name();
        self.clear_friendly_name();
        self.clear_hotkey();
        self.clear_remaps_to_ability_id();
        self.clear_available();
        self.clear_target();
        self.clear_allow_minimap();
        self.clear_allow_autocast();
        self.clear_is_building();
        self.clear_footprint_radius();
        self.clear_is_instant_placement();
        self.clear_cast_range();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AbilityData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AbilityData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AbilityData_Target {
    None = 1,
    Point = 2,
    Unit = 3,
    PointOrUnit = 4,
    PointOrNone = 5,
}

impl ::protobuf::ProtobufEnum for AbilityData_Target {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AbilityData_Target> {
        match value {
            1 => ::std::option::Option::Some(AbilityData_Target::None),
            2 => ::std::option::Option::Some(AbilityData_Target::Point),
            3 => ::std::option::Option::Some(AbilityData_Target::Unit),
            4 => ::std::option::Option::Some(AbilityData_Target::PointOrUnit),
            5 => ::std::option::Option::Some(AbilityData_Target::PointOrNone),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AbilityData_Target] = &[
            AbilityData_Target::None,
            AbilityData_Target::Point,
            AbilityData_Target::Unit,
            AbilityData_Target::PointOrUnit,
            AbilityData_Target::PointOrNone,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<AbilityData_Target>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AbilityData_Target", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AbilityData_Target {
}

impl ::protobuf::reflect::ProtobufValue for AbilityData_Target {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DamageBonus {
    // message fields
    attribute: ::std::option::Option<Attribute>,
    bonus: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DamageBonus {}

impl DamageBonus {
    pub fn new() -> DamageBonus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DamageBonus {
        static mut instance: ::protobuf::lazy::Lazy<DamageBonus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DamageBonus,
        };
        unsafe {
            instance.get(DamageBonus::new)
        }
    }

    // optional .SC2APIProtocol.Attribute attribute = 1;

    pub fn clear_attribute(&mut self) {
        self.attribute = ::std::option::Option::None;
    }

    pub fn has_attribute(&self) -> bool {
        self.attribute.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attribute(&mut self, v: Attribute) {
        self.attribute = ::std::option::Option::Some(v);
    }

    pub fn get_attribute(&self) -> Attribute {
        self.attribute.unwrap_or(Attribute::Light)
    }

    fn get_attribute_for_reflect(&self) -> &::std::option::Option<Attribute> {
        &self.attribute
    }

    fn mut_attribute_for_reflect(&mut self) -> &mut ::std::option::Option<Attribute> {
        &mut self.attribute
    }

    // optional float bonus = 2;

    pub fn clear_bonus(&mut self) {
        self.bonus = ::std::option::Option::None;
    }

    pub fn has_bonus(&self) -> bool {
        self.bonus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bonus(&mut self, v: f32) {
        self.bonus = ::std::option::Option::Some(v);
    }

    pub fn get_bonus(&self) -> f32 {
        self.bonus.unwrap_or(0.)
    }

    fn get_bonus_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.bonus
    }

    fn mut_bonus_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.bonus
    }
}

impl ::protobuf::Message for DamageBonus {
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
                    self.attribute = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.bonus = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.attribute {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.bonus {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.attribute {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.bonus {
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

impl ::protobuf::MessageStatic for DamageBonus {
    fn new() -> DamageBonus {
        DamageBonus::new()
    }

    fn descriptor_static(_: ::std::option::Option<DamageBonus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Attribute>>(
                    "attribute",
                    DamageBonus::get_attribute_for_reflect,
                    DamageBonus::mut_attribute_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "bonus",
                    DamageBonus::get_bonus_for_reflect,
                    DamageBonus::mut_bonus_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DamageBonus>(
                    "DamageBonus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DamageBonus {
    fn clear(&mut self) {
        self.clear_attribute();
        self.clear_bonus();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DamageBonus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DamageBonus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Weapon {
    // message fields
    field_type: ::std::option::Option<Weapon_TargetType>,
    damage: ::std::option::Option<f32>,
    damage_bonus: ::protobuf::RepeatedField<DamageBonus>,
    attacks: ::std::option::Option<u32>,
    range: ::std::option::Option<f32>,
    speed: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Weapon {}

impl Weapon {
    pub fn new() -> Weapon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Weapon {
        static mut instance: ::protobuf::lazy::Lazy<Weapon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Weapon,
        };
        unsafe {
            instance.get(Weapon::new)
        }
    }

    // optional .SC2APIProtocol.Weapon.TargetType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Weapon_TargetType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> Weapon_TargetType {
        self.field_type.unwrap_or(Weapon_TargetType::Ground)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<Weapon_TargetType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<Weapon_TargetType> {
        &mut self.field_type
    }

    // optional float damage = 2;

    pub fn clear_damage(&mut self) {
        self.damage = ::std::option::Option::None;
    }

    pub fn has_damage(&self) -> bool {
        self.damage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage(&mut self, v: f32) {
        self.damage = ::std::option::Option::Some(v);
    }

    pub fn get_damage(&self) -> f32 {
        self.damage.unwrap_or(0.)
    }

    fn get_damage_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.damage
    }

    fn mut_damage_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.damage
    }

    // repeated .SC2APIProtocol.DamageBonus damage_bonus = 3;

    pub fn clear_damage_bonus(&mut self) {
        self.damage_bonus.clear();
    }

    // Param is passed by value, moved
    pub fn set_damage_bonus(&mut self, v: ::protobuf::RepeatedField<DamageBonus>) {
        self.damage_bonus = v;
    }

    // Mutable pointer to the field.
    pub fn mut_damage_bonus(&mut self) -> &mut ::protobuf::RepeatedField<DamageBonus> {
        &mut self.damage_bonus
    }

    // Take field
    pub fn take_damage_bonus(&mut self) -> ::protobuf::RepeatedField<DamageBonus> {
        ::std::mem::replace(&mut self.damage_bonus, ::protobuf::RepeatedField::new())
    }

    pub fn get_damage_bonus(&self) -> &[DamageBonus] {
        &self.damage_bonus
    }

    fn get_damage_bonus_for_reflect(&self) -> &::protobuf::RepeatedField<DamageBonus> {
        &self.damage_bonus
    }

    fn mut_damage_bonus_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DamageBonus> {
        &mut self.damage_bonus
    }

    // optional uint32 attacks = 4;

    pub fn clear_attacks(&mut self) {
        self.attacks = ::std::option::Option::None;
    }

    pub fn has_attacks(&self) -> bool {
        self.attacks.is_some()
    }

    // Param is passed by value, moved
    pub fn set_attacks(&mut self, v: u32) {
        self.attacks = ::std::option::Option::Some(v);
    }

    pub fn get_attacks(&self) -> u32 {
        self.attacks.unwrap_or(0)
    }

    fn get_attacks_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.attacks
    }

    fn mut_attacks_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.attacks
    }

    // optional float range = 5;

    pub fn clear_range(&mut self) {
        self.range = ::std::option::Option::None;
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: f32) {
        self.range = ::std::option::Option::Some(v);
    }

    pub fn get_range(&self) -> f32 {
        self.range.unwrap_or(0.)
    }

    fn get_range_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.range
    }

    fn mut_range_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.range
    }

    // optional float speed = 6;

    pub fn clear_speed(&mut self) {
        self.speed = ::std::option::Option::None;
    }

    pub fn has_speed(&self) -> bool {
        self.speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speed(&mut self, v: f32) {
        self.speed = ::std::option::Option::Some(v);
    }

    pub fn get_speed(&self) -> f32 {
        self.speed.unwrap_or(0.)
    }

    fn get_speed_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.speed
    }

    fn mut_speed_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.speed
    }
}

impl ::protobuf::Message for Weapon {
    fn is_initialized(&self) -> bool {
        for v in &self.damage_bonus {
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
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.damage = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.damage_bonus)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.attacks = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.range = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.speed = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.damage {
            my_size += 5;
        }
        for value in &self.damage_bonus {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.attacks {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.range {
            my_size += 5;
        }
        if let Some(v) = self.speed {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.damage {
            os.write_float(2, v)?;
        }
        for v in &self.damage_bonus {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.attacks {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.range {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.speed {
            os.write_float(6, v)?;
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

impl ::protobuf::MessageStatic for Weapon {
    fn new() -> Weapon {
        Weapon::new()
    }

    fn descriptor_static(_: ::std::option::Option<Weapon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Weapon_TargetType>>(
                    "type",
                    Weapon::get_field_type_for_reflect,
                    Weapon::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "damage",
                    Weapon::get_damage_for_reflect,
                    Weapon::mut_damage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DamageBonus>>(
                    "damage_bonus",
                    Weapon::get_damage_bonus_for_reflect,
                    Weapon::mut_damage_bonus_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "attacks",
                    Weapon::get_attacks_for_reflect,
                    Weapon::mut_attacks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "range",
                    Weapon::get_range_for_reflect,
                    Weapon::mut_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "speed",
                    Weapon::get_speed_for_reflect,
                    Weapon::mut_speed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Weapon>(
                    "Weapon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Weapon {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_damage();
        self.clear_damage_bonus();
        self.clear_attacks();
        self.clear_range();
        self.clear_speed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Weapon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Weapon {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Weapon_TargetType {
    Ground = 1,
    Air = 2,
    Any = 3,
}

impl ::protobuf::ProtobufEnum for Weapon_TargetType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Weapon_TargetType> {
        match value {
            1 => ::std::option::Option::Some(Weapon_TargetType::Ground),
            2 => ::std::option::Option::Some(Weapon_TargetType::Air),
            3 => ::std::option::Option::Some(Weapon_TargetType::Any),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Weapon_TargetType] = &[
            Weapon_TargetType::Ground,
            Weapon_TargetType::Air,
            Weapon_TargetType::Any,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Weapon_TargetType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Weapon_TargetType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Weapon_TargetType {
}

impl ::protobuf::reflect::ProtobufValue for Weapon_TargetType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnitTypeData {
    // message fields
    unit_id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    available: ::std::option::Option<bool>,
    cargo_size: ::std::option::Option<u32>,
    mineral_cost: ::std::option::Option<u32>,
    vespene_cost: ::std::option::Option<u32>,
    food_required: ::std::option::Option<f32>,
    food_provided: ::std::option::Option<f32>,
    ability_id: ::std::option::Option<u32>,
    race: ::std::option::Option<super::common::Race>,
    build_time: ::std::option::Option<f32>,
    has_vespene: ::std::option::Option<bool>,
    has_minerals: ::std::option::Option<bool>,
    tech_alias: ::std::vec::Vec<u32>,
    unit_alias: ::std::option::Option<u32>,
    tech_requirement: ::std::option::Option<u32>,
    require_attached: ::std::option::Option<bool>,
    attributes: ::std::vec::Vec<Attribute>,
    movement_speed: ::std::option::Option<f32>,
    armor: ::std::option::Option<f32>,
    weapons: ::protobuf::RepeatedField<Weapon>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnitTypeData {}

impl UnitTypeData {
    pub fn new() -> UnitTypeData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnitTypeData {
        static mut instance: ::protobuf::lazy::Lazy<UnitTypeData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnitTypeData,
        };
        unsafe {
            instance.get(UnitTypeData::new)
        }
    }

    // optional uint32 unit_id = 1;

    pub fn clear_unit_id(&mut self) {
        self.unit_id = ::std::option::Option::None;
    }

    pub fn has_unit_id(&self) -> bool {
        self.unit_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_id(&mut self, v: u32) {
        self.unit_id = ::std::option::Option::Some(v);
    }

    pub fn get_unit_id(&self) -> u32 {
        self.unit_id.unwrap_or(0)
    }

    fn get_unit_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unit_id
    }

    fn mut_unit_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unit_id
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional bool available = 3;

    pub fn clear_available(&mut self) {
        self.available = ::std::option::Option::None;
    }

    pub fn has_available(&self) -> bool {
        self.available.is_some()
    }

    // Param is passed by value, moved
    pub fn set_available(&mut self, v: bool) {
        self.available = ::std::option::Option::Some(v);
    }

    pub fn get_available(&self) -> bool {
        self.available.unwrap_or(false)
    }

    fn get_available_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.available
    }

    fn mut_available_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.available
    }

    // optional uint32 cargo_size = 4;

    pub fn clear_cargo_size(&mut self) {
        self.cargo_size = ::std::option::Option::None;
    }

    pub fn has_cargo_size(&self) -> bool {
        self.cargo_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cargo_size(&mut self, v: u32) {
        self.cargo_size = ::std::option::Option::Some(v);
    }

    pub fn get_cargo_size(&self) -> u32 {
        self.cargo_size.unwrap_or(0)
    }

    fn get_cargo_size_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.cargo_size
    }

    fn mut_cargo_size_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.cargo_size
    }

    // optional uint32 mineral_cost = 12;

    pub fn clear_mineral_cost(&mut self) {
        self.mineral_cost = ::std::option::Option::None;
    }

    pub fn has_mineral_cost(&self) -> bool {
        self.mineral_cost.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mineral_cost(&mut self, v: u32) {
        self.mineral_cost = ::std::option::Option::Some(v);
    }

    pub fn get_mineral_cost(&self) -> u32 {
        self.mineral_cost.unwrap_or(0)
    }

    fn get_mineral_cost_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.mineral_cost
    }

    fn mut_mineral_cost_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.mineral_cost
    }

    // optional uint32 vespene_cost = 13;

    pub fn clear_vespene_cost(&mut self) {
        self.vespene_cost = ::std::option::Option::None;
    }

    pub fn has_vespene_cost(&self) -> bool {
        self.vespene_cost.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vespene_cost(&mut self, v: u32) {
        self.vespene_cost = ::std::option::Option::Some(v);
    }

    pub fn get_vespene_cost(&self) -> u32 {
        self.vespene_cost.unwrap_or(0)
    }

    fn get_vespene_cost_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.vespene_cost
    }

    fn mut_vespene_cost_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.vespene_cost
    }

    // optional float food_required = 14;

    pub fn clear_food_required(&mut self) {
        self.food_required = ::std::option::Option::None;
    }

    pub fn has_food_required(&self) -> bool {
        self.food_required.is_some()
    }

    // Param is passed by value, moved
    pub fn set_food_required(&mut self, v: f32) {
        self.food_required = ::std::option::Option::Some(v);
    }

    pub fn get_food_required(&self) -> f32 {
        self.food_required.unwrap_or(0.)
    }

    fn get_food_required_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.food_required
    }

    fn mut_food_required_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.food_required
    }

    // optional float food_provided = 18;

    pub fn clear_food_provided(&mut self) {
        self.food_provided = ::std::option::Option::None;
    }

    pub fn has_food_provided(&self) -> bool {
        self.food_provided.is_some()
    }

    // Param is passed by value, moved
    pub fn set_food_provided(&mut self, v: f32) {
        self.food_provided = ::std::option::Option::Some(v);
    }

    pub fn get_food_provided(&self) -> f32 {
        self.food_provided.unwrap_or(0.)
    }

    fn get_food_provided_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.food_provided
    }

    fn mut_food_provided_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.food_provided
    }

    // optional uint32 ability_id = 15;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }

    // optional .SC2APIProtocol.Race race = 16;

    pub fn clear_race(&mut self) {
        self.race = ::std::option::Option::None;
    }

    pub fn has_race(&self) -> bool {
        self.race.is_some()
    }

    // Param is passed by value, moved
    pub fn set_race(&mut self, v: super::common::Race) {
        self.race = ::std::option::Option::Some(v);
    }

    pub fn get_race(&self) -> super::common::Race {
        self.race.unwrap_or(super::common::Race::NoRace)
    }

    fn get_race_for_reflect(&self) -> &::std::option::Option<super::common::Race> {
        &self.race
    }

    fn mut_race_for_reflect(&mut self) -> &mut ::std::option::Option<super::common::Race> {
        &mut self.race
    }

    // optional float build_time = 17;

    pub fn clear_build_time(&mut self) {
        self.build_time = ::std::option::Option::None;
    }

    pub fn has_build_time(&self) -> bool {
        self.build_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_build_time(&mut self, v: f32) {
        self.build_time = ::std::option::Option::Some(v);
    }

    pub fn get_build_time(&self) -> f32 {
        self.build_time.unwrap_or(0.)
    }

    fn get_build_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.build_time
    }

    fn mut_build_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.build_time
    }

    // optional bool has_vespene = 19;

    pub fn clear_has_vespene(&mut self) {
        self.has_vespene = ::std::option::Option::None;
    }

    pub fn has_has_vespene(&self) -> bool {
        self.has_vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_vespene(&mut self, v: bool) {
        self.has_vespene = ::std::option::Option::Some(v);
    }

    pub fn get_has_vespene(&self) -> bool {
        self.has_vespene.unwrap_or(false)
    }

    fn get_has_vespene_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_vespene
    }

    fn mut_has_vespene_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_vespene
    }

    // optional bool has_minerals = 20;

    pub fn clear_has_minerals(&mut self) {
        self.has_minerals = ::std::option::Option::None;
    }

    pub fn has_has_minerals(&self) -> bool {
        self.has_minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_minerals(&mut self, v: bool) {
        self.has_minerals = ::std::option::Option::Some(v);
    }

    pub fn get_has_minerals(&self) -> bool {
        self.has_minerals.unwrap_or(false)
    }

    fn get_has_minerals_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_minerals
    }

    fn mut_has_minerals_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_minerals
    }

    // repeated uint32 tech_alias = 21;

    pub fn clear_tech_alias(&mut self) {
        self.tech_alias.clear();
    }

    // Param is passed by value, moved
    pub fn set_tech_alias(&mut self, v: ::std::vec::Vec<u32>) {
        self.tech_alias = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tech_alias(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.tech_alias
    }

    // Take field
    pub fn take_tech_alias(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.tech_alias, ::std::vec::Vec::new())
    }

    pub fn get_tech_alias(&self) -> &[u32] {
        &self.tech_alias
    }

    fn get_tech_alias_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.tech_alias
    }

    fn mut_tech_alias_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.tech_alias
    }

    // optional uint32 unit_alias = 22;

    pub fn clear_unit_alias(&mut self) {
        self.unit_alias = ::std::option::Option::None;
    }

    pub fn has_unit_alias(&self) -> bool {
        self.unit_alias.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_alias(&mut self, v: u32) {
        self.unit_alias = ::std::option::Option::Some(v);
    }

    pub fn get_unit_alias(&self) -> u32 {
        self.unit_alias.unwrap_or(0)
    }

    fn get_unit_alias_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unit_alias
    }

    fn mut_unit_alias_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unit_alias
    }

    // optional uint32 tech_requirement = 23;

    pub fn clear_tech_requirement(&mut self) {
        self.tech_requirement = ::std::option::Option::None;
    }

    pub fn has_tech_requirement(&self) -> bool {
        self.tech_requirement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tech_requirement(&mut self, v: u32) {
        self.tech_requirement = ::std::option::Option::Some(v);
    }

    pub fn get_tech_requirement(&self) -> u32 {
        self.tech_requirement.unwrap_or(0)
    }

    fn get_tech_requirement_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tech_requirement
    }

    fn mut_tech_requirement_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tech_requirement
    }

    // optional bool require_attached = 24;

    pub fn clear_require_attached(&mut self) {
        self.require_attached = ::std::option::Option::None;
    }

    pub fn has_require_attached(&self) -> bool {
        self.require_attached.is_some()
    }

    // Param is passed by value, moved
    pub fn set_require_attached(&mut self, v: bool) {
        self.require_attached = ::std::option::Option::Some(v);
    }

    pub fn get_require_attached(&self) -> bool {
        self.require_attached.unwrap_or(false)
    }

    fn get_require_attached_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.require_attached
    }

    fn mut_require_attached_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.require_attached
    }

    // repeated .SC2APIProtocol.Attribute attributes = 8;

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: ::std::vec::Vec<Attribute>) {
        self.attributes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attributes(&mut self) -> &mut ::std::vec::Vec<Attribute> {
        &mut self.attributes
    }

    // Take field
    pub fn take_attributes(&mut self) -> ::std::vec::Vec<Attribute> {
        ::std::mem::replace(&mut self.attributes, ::std::vec::Vec::new())
    }

    pub fn get_attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    fn get_attributes_for_reflect(&self) -> &::std::vec::Vec<Attribute> {
        &self.attributes
    }

    fn mut_attributes_for_reflect(&mut self) -> &mut ::std::vec::Vec<Attribute> {
        &mut self.attributes
    }

    // optional float movement_speed = 9;

    pub fn clear_movement_speed(&mut self) {
        self.movement_speed = ::std::option::Option::None;
    }

    pub fn has_movement_speed(&self) -> bool {
        self.movement_speed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_movement_speed(&mut self, v: f32) {
        self.movement_speed = ::std::option::Option::Some(v);
    }

    pub fn get_movement_speed(&self) -> f32 {
        self.movement_speed.unwrap_or(0.)
    }

    fn get_movement_speed_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.movement_speed
    }

    fn mut_movement_speed_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.movement_speed
    }

    // optional float armor = 10;

    pub fn clear_armor(&mut self) {
        self.armor = ::std::option::Option::None;
    }

    pub fn has_armor(&self) -> bool {
        self.armor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_armor(&mut self, v: f32) {
        self.armor = ::std::option::Option::Some(v);
    }

    pub fn get_armor(&self) -> f32 {
        self.armor.unwrap_or(0.)
    }

    fn get_armor_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.armor
    }

    fn mut_armor_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.armor
    }

    // repeated .SC2APIProtocol.Weapon weapons = 11;

    pub fn clear_weapons(&mut self) {
        self.weapons.clear();
    }

    // Param is passed by value, moved
    pub fn set_weapons(&mut self, v: ::protobuf::RepeatedField<Weapon>) {
        self.weapons = v;
    }

    // Mutable pointer to the field.
    pub fn mut_weapons(&mut self) -> &mut ::protobuf::RepeatedField<Weapon> {
        &mut self.weapons
    }

    // Take field
    pub fn take_weapons(&mut self) -> ::protobuf::RepeatedField<Weapon> {
        ::std::mem::replace(&mut self.weapons, ::protobuf::RepeatedField::new())
    }

    pub fn get_weapons(&self) -> &[Weapon] {
        &self.weapons
    }

    fn get_weapons_for_reflect(&self) -> &::protobuf::RepeatedField<Weapon> {
        &self.weapons
    }

    fn mut_weapons_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Weapon> {
        &mut self.weapons
    }
}

impl ::protobuf::Message for UnitTypeData {
    fn is_initialized(&self) -> bool {
        for v in &self.weapons {
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
                    let tmp = is.read_uint32()?;
                    self.unit_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.available = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cargo_size = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.mineral_cost = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.vespene_cost = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.food_required = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.food_provided = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.race = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.build_time = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_vespene = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_minerals = ::std::option::Option::Some(tmp);
                },
                21 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.tech_alias)?;
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.unit_alias = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.tech_requirement = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.require_attached = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.attributes)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.movement_speed = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.armor = ::std::option::Option::Some(tmp);
                },
                11 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.weapons)?;
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
        if let Some(v) = self.unit_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.available {
            my_size += 2;
        }
        if let Some(v) = self.cargo_size {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.mineral_cost {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.vespene_cost {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.food_required {
            my_size += 5;
        }
        if let Some(v) = self.food_provided {
            my_size += 6;
        }
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.race {
            my_size += ::protobuf::rt::enum_size(16, v);
        }
        if let Some(v) = self.build_time {
            my_size += 6;
        }
        if let Some(v) = self.has_vespene {
            my_size += 3;
        }
        if let Some(v) = self.has_minerals {
            my_size += 3;
        }
        for value in &self.tech_alias {
            my_size += ::protobuf::rt::value_size(21, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.unit_alias {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.tech_requirement {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.require_attached {
            my_size += 3;
        }
        for value in &self.attributes {
            my_size += ::protobuf::rt::enum_size(8, *value);
        };
        if let Some(v) = self.movement_speed {
            my_size += 5;
        }
        if let Some(v) = self.armor {
            my_size += 5;
        }
        for value in &self.weapons {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.available {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.cargo_size {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.mineral_cost {
            os.write_uint32(12, v)?;
        }
        if let Some(v) = self.vespene_cost {
            os.write_uint32(13, v)?;
        }
        if let Some(v) = self.food_required {
            os.write_float(14, v)?;
        }
        if let Some(v) = self.food_provided {
            os.write_float(18, v)?;
        }
        if let Some(v) = self.ability_id {
            os.write_uint32(15, v)?;
        }
        if let Some(v) = self.race {
            os.write_enum(16, v.value())?;
        }
        if let Some(v) = self.build_time {
            os.write_float(17, v)?;
        }
        if let Some(v) = self.has_vespene {
            os.write_bool(19, v)?;
        }
        if let Some(v) = self.has_minerals {
            os.write_bool(20, v)?;
        }
        for v in &self.tech_alias {
            os.write_uint32(21, *v)?;
        };
        if let Some(v) = self.unit_alias {
            os.write_uint32(22, v)?;
        }
        if let Some(v) = self.tech_requirement {
            os.write_uint32(23, v)?;
        }
        if let Some(v) = self.require_attached {
            os.write_bool(24, v)?;
        }
        for v in &self.attributes {
            os.write_enum(8, v.value())?;
        };
        if let Some(v) = self.movement_speed {
            os.write_float(9, v)?;
        }
        if let Some(v) = self.armor {
            os.write_float(10, v)?;
        }
        for v in &self.weapons {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for UnitTypeData {
    fn new() -> UnitTypeData {
        UnitTypeData::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnitTypeData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unit_id",
                    UnitTypeData::get_unit_id_for_reflect,
                    UnitTypeData::mut_unit_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    UnitTypeData::get_name_for_reflect,
                    UnitTypeData::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "available",
                    UnitTypeData::get_available_for_reflect,
                    UnitTypeData::mut_available_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "cargo_size",
                    UnitTypeData::get_cargo_size_for_reflect,
                    UnitTypeData::mut_cargo_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mineral_cost",
                    UnitTypeData::get_mineral_cost_for_reflect,
                    UnitTypeData::mut_mineral_cost_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "vespene_cost",
                    UnitTypeData::get_vespene_cost_for_reflect,
                    UnitTypeData::mut_vespene_cost_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "food_required",
                    UnitTypeData::get_food_required_for_reflect,
                    UnitTypeData::mut_food_required_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "food_provided",
                    UnitTypeData::get_food_provided_for_reflect,
                    UnitTypeData::mut_food_provided_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    UnitTypeData::get_ability_id_for_reflect,
                    UnitTypeData::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::common::Race>>(
                    "race",
                    UnitTypeData::get_race_for_reflect,
                    UnitTypeData::mut_race_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "build_time",
                    UnitTypeData::get_build_time_for_reflect,
                    UnitTypeData::mut_build_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_vespene",
                    UnitTypeData::get_has_vespene_for_reflect,
                    UnitTypeData::mut_has_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_minerals",
                    UnitTypeData::get_has_minerals_for_reflect,
                    UnitTypeData::mut_has_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tech_alias",
                    UnitTypeData::get_tech_alias_for_reflect,
                    UnitTypeData::mut_tech_alias_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unit_alias",
                    UnitTypeData::get_unit_alias_for_reflect,
                    UnitTypeData::mut_unit_alias_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tech_requirement",
                    UnitTypeData::get_tech_requirement_for_reflect,
                    UnitTypeData::mut_tech_requirement_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "require_attached",
                    UnitTypeData::get_require_attached_for_reflect,
                    UnitTypeData::mut_require_attached_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Attribute>>(
                    "attributes",
                    UnitTypeData::get_attributes_for_reflect,
                    UnitTypeData::mut_attributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "movement_speed",
                    UnitTypeData::get_movement_speed_for_reflect,
                    UnitTypeData::mut_movement_speed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "armor",
                    UnitTypeData::get_armor_for_reflect,
                    UnitTypeData::mut_armor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Weapon>>(
                    "weapons",
                    UnitTypeData::get_weapons_for_reflect,
                    UnitTypeData::mut_weapons_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnitTypeData>(
                    "UnitTypeData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnitTypeData {
    fn clear(&mut self) {
        self.clear_unit_id();
        self.clear_name();
        self.clear_available();
        self.clear_cargo_size();
        self.clear_mineral_cost();
        self.clear_vespene_cost();
        self.clear_food_required();
        self.clear_food_provided();
        self.clear_ability_id();
        self.clear_race();
        self.clear_build_time();
        self.clear_has_vespene();
        self.clear_has_minerals();
        self.clear_tech_alias();
        self.clear_unit_alias();
        self.clear_tech_requirement();
        self.clear_require_attached();
        self.clear_attributes();
        self.clear_movement_speed();
        self.clear_armor();
        self.clear_weapons();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnitTypeData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnitTypeData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UpgradeData {
    // message fields
    upgrade_id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    mineral_cost: ::std::option::Option<u32>,
    vespene_cost: ::std::option::Option<u32>,
    research_time: ::std::option::Option<f32>,
    ability_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UpgradeData {}

impl UpgradeData {
    pub fn new() -> UpgradeData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpgradeData {
        static mut instance: ::protobuf::lazy::Lazy<UpgradeData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpgradeData,
        };
        unsafe {
            instance.get(UpgradeData::new)
        }
    }

    // optional uint32 upgrade_id = 1;

    pub fn clear_upgrade_id(&mut self) {
        self.upgrade_id = ::std::option::Option::None;
    }

    pub fn has_upgrade_id(&self) -> bool {
        self.upgrade_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade_id(&mut self, v: u32) {
        self.upgrade_id = ::std::option::Option::Some(v);
    }

    pub fn get_upgrade_id(&self) -> u32 {
        self.upgrade_id.unwrap_or(0)
    }

    fn get_upgrade_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.upgrade_id
    }

    fn mut_upgrade_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.upgrade_id
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional uint32 mineral_cost = 3;

    pub fn clear_mineral_cost(&mut self) {
        self.mineral_cost = ::std::option::Option::None;
    }

    pub fn has_mineral_cost(&self) -> bool {
        self.mineral_cost.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mineral_cost(&mut self, v: u32) {
        self.mineral_cost = ::std::option::Option::Some(v);
    }

    pub fn get_mineral_cost(&self) -> u32 {
        self.mineral_cost.unwrap_or(0)
    }

    fn get_mineral_cost_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.mineral_cost
    }

    fn mut_mineral_cost_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.mineral_cost
    }

    // optional uint32 vespene_cost = 4;

    pub fn clear_vespene_cost(&mut self) {
        self.vespene_cost = ::std::option::Option::None;
    }

    pub fn has_vespene_cost(&self) -> bool {
        self.vespene_cost.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vespene_cost(&mut self, v: u32) {
        self.vespene_cost = ::std::option::Option::Some(v);
    }

    pub fn get_vespene_cost(&self) -> u32 {
        self.vespene_cost.unwrap_or(0)
    }

    fn get_vespene_cost_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.vespene_cost
    }

    fn mut_vespene_cost_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.vespene_cost
    }

    // optional float research_time = 5;

    pub fn clear_research_time(&mut self) {
        self.research_time = ::std::option::Option::None;
    }

    pub fn has_research_time(&self) -> bool {
        self.research_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_research_time(&mut self, v: f32) {
        self.research_time = ::std::option::Option::Some(v);
    }

    pub fn get_research_time(&self) -> f32 {
        self.research_time.unwrap_or(0.)
    }

    fn get_research_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.research_time
    }

    fn mut_research_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.research_time
    }

    // optional uint32 ability_id = 6;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u32) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u32 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ability_id
    }
}

impl ::protobuf::Message for UpgradeData {
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
                    let tmp = is.read_uint32()?;
                    self.upgrade_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.mineral_cost = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.vespene_cost = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.research_time = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.upgrade_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.mineral_cost {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.vespene_cost {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.research_time {
            my_size += 5;
        }
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.upgrade_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.mineral_cost {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.vespene_cost {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.research_time {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.ability_id {
            os.write_uint32(6, v)?;
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

impl ::protobuf::MessageStatic for UpgradeData {
    fn new() -> UpgradeData {
        UpgradeData::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpgradeData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "upgrade_id",
                    UpgradeData::get_upgrade_id_for_reflect,
                    UpgradeData::mut_upgrade_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    UpgradeData::get_name_for_reflect,
                    UpgradeData::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "mineral_cost",
                    UpgradeData::get_mineral_cost_for_reflect,
                    UpgradeData::mut_mineral_cost_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "vespene_cost",
                    UpgradeData::get_vespene_cost_for_reflect,
                    UpgradeData::mut_vespene_cost_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "research_time",
                    UpgradeData::get_research_time_for_reflect,
                    UpgradeData::mut_research_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    UpgradeData::get_ability_id_for_reflect,
                    UpgradeData::mut_ability_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpgradeData>(
                    "UpgradeData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpgradeData {
    fn clear(&mut self) {
        self.clear_upgrade_id();
        self.clear_name();
        self.clear_mineral_cost();
        self.clear_vespene_cost();
        self.clear_research_time();
        self.clear_ability_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UpgradeData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpgradeData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BuffData {
    // message fields
    buff_id: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BuffData {}

impl BuffData {
    pub fn new() -> BuffData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BuffData {
        static mut instance: ::protobuf::lazy::Lazy<BuffData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BuffData,
        };
        unsafe {
            instance.get(BuffData::new)
        }
    }

    // optional uint32 buff_id = 1;

    pub fn clear_buff_id(&mut self) {
        self.buff_id = ::std::option::Option::None;
    }

    pub fn has_buff_id(&self) -> bool {
        self.buff_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buff_id(&mut self, v: u32) {
        self.buff_id = ::std::option::Option::Some(v);
    }

    pub fn get_buff_id(&self) -> u32 {
        self.buff_id.unwrap_or(0)
    }

    fn get_buff_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.buff_id
    }

    fn mut_buff_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.buff_id
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }
}

impl ::protobuf::Message for BuffData {
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
                    let tmp = is.read_uint32()?;
                    self.buff_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.buff_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.buff_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for BuffData {
    fn new() -> BuffData {
        BuffData::new()
    }

    fn descriptor_static(_: ::std::option::Option<BuffData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "buff_id",
                    BuffData::get_buff_id_for_reflect,
                    BuffData::mut_buff_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    BuffData::get_name_for_reflect,
                    BuffData::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BuffData>(
                    "BuffData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BuffData {
    fn clear(&mut self) {
        self.clear_buff_id();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BuffData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BuffData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Attribute {
    Light = 1,
    Armored = 2,
    Biological = 3,
    Mechanical = 4,
    Robotic = 5,
    Psionic = 6,
    Massive = 7,
    Structure = 8,
    Hover = 9,
    Heroic = 10,
    Summoned = 11,
}

impl ::protobuf::ProtobufEnum for Attribute {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Attribute> {
        match value {
            1 => ::std::option::Option::Some(Attribute::Light),
            2 => ::std::option::Option::Some(Attribute::Armored),
            3 => ::std::option::Option::Some(Attribute::Biological),
            4 => ::std::option::Option::Some(Attribute::Mechanical),
            5 => ::std::option::Option::Some(Attribute::Robotic),
            6 => ::std::option::Option::Some(Attribute::Psionic),
            7 => ::std::option::Option::Some(Attribute::Massive),
            8 => ::std::option::Option::Some(Attribute::Structure),
            9 => ::std::option::Option::Some(Attribute::Hover),
            10 => ::std::option::Option::Some(Attribute::Heroic),
            11 => ::std::option::Option::Some(Attribute::Summoned),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Attribute] = &[
            Attribute::Light,
            Attribute::Armored,
            Attribute::Biological,
            Attribute::Mechanical,
            Attribute::Robotic,
            Attribute::Psionic,
            Attribute::Massive,
            Attribute::Structure,
            Attribute::Hover,
            Attribute::Heroic,
            Attribute::Summoned,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Attribute>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Attribute", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Attribute {
}

impl ::protobuf::reflect::ProtobufValue for Attribute {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bs2clientprotocol/data.proto\x12\x0eSC2APIProtocol\x1a\x1ds2clientp\
    rotocol/common.proto\"\x85\x05\n\x0bAbilityData\x12\x1d\n\nability_id\
    \x18\x01\x20\x01(\rR\tabilityId\x12\x1b\n\tlink_name\x18\x02\x20\x01(\tR\
    \x08linkName\x12\x1d\n\nlink_index\x18\x03\x20\x01(\rR\tlinkIndex\x12\
    \x1f\n\x0bbutton_name\x18\x04\x20\x01(\tR\nbuttonName\x12#\n\rfriendly_n\
    ame\x18\x05\x20\x01(\tR\x0cfriendlyName\x12\x16\n\x06hotkey\x18\x06\x20\
    \x01(\tR\x06hotkey\x12/\n\x14remaps_to_ability_id\x18\x07\x20\x01(\rR\
    \x11remapsToAbilityId\x12\x1c\n\tavailable\x18\x08\x20\x01(\x08R\tavaila\
    ble\x12:\n\x06target\x18\t\x20\x01(\x0e2\".SC2APIProtocol.AbilityData.Ta\
    rgetR\x06target\x12#\n\rallow_minimap\x18\n\x20\x01(\x08R\x0callowMinima\
    p\x12%\n\x0eallow_autocast\x18\x0b\x20\x01(\x08R\rallowAutocast\x12\x1f\
    \n\x0bis_building\x18\x0c\x20\x01(\x08R\nisBuilding\x12)\n\x10footprint_\
    radius\x18\r\x20\x01(\x02R\x0ffootprintRadius\x120\n\x14is_instant_place\
    ment\x18\x0e\x20\x01(\x08R\x12isInstantPlacement\x12\x1d\n\ncast_range\
    \x18\x0f\x20\x01(\x02R\tcastRange\"I\n\x06Target\x12\x08\n\x04None\x10\
    \x01\x12\t\n\x05Point\x10\x02\x12\x08\n\x04Unit\x10\x03\x12\x0f\n\x0bPoi\
    ntOrUnit\x10\x04\x12\x0f\n\x0bPointOrNone\x10\x05\"\\\n\x0bDamageBonus\
    \x127\n\tattribute\x18\x01\x20\x01(\x0e2\x19.SC2APIProtocol.AttributeR\t\
    attribute\x12\x14\n\x05bonus\x18\x02\x20\x01(\x02R\x05bonus\"\x89\x02\n\
    \x06Weapon\x125\n\x04type\x18\x01\x20\x01(\x0e2!.SC2APIProtocol.Weapon.T\
    argetTypeR\x04type\x12\x16\n\x06damage\x18\x02\x20\x01(\x02R\x06damage\
    \x12>\n\x0cdamage_bonus\x18\x03\x20\x03(\x0b2\x1b.SC2APIProtocol.DamageB\
    onusR\x0bdamageBonus\x12\x18\n\x07attacks\x18\x04\x20\x01(\rR\x07attacks\
    \x12\x14\n\x05range\x18\x05\x20\x01(\x02R\x05range\x12\x14\n\x05speed\
    \x18\x06\x20\x01(\x02R\x05speed\"*\n\nTargetType\x12\n\n\x06Ground\x10\
    \x01\x12\x07\n\x03Air\x10\x02\x12\x07\n\x03Any\x10\x03\"\xf2\x05\n\x0cUn\
    itTypeData\x12\x17\n\x07unit_id\x18\x01\x20\x01(\rR\x06unitId\x12\x12\n\
    \x04name\x18\x02\x20\x01(\tR\x04name\x12\x1c\n\tavailable\x18\x03\x20\
    \x01(\x08R\tavailable\x12\x1d\n\ncargo_size\x18\x04\x20\x01(\rR\tcargoSi\
    ze\x12!\n\x0cmineral_cost\x18\x0c\x20\x01(\rR\x0bmineralCost\x12!\n\x0cv\
    espene_cost\x18\r\x20\x01(\rR\x0bvespeneCost\x12#\n\rfood_required\x18\
    \x0e\x20\x01(\x02R\x0cfoodRequired\x12#\n\rfood_provided\x18\x12\x20\x01\
    (\x02R\x0cfoodProvided\x12\x1d\n\nability_id\x18\x0f\x20\x01(\rR\tabilit\
    yId\x12(\n\x04race\x18\x10\x20\x01(\x0e2\x14.SC2APIProtocol.RaceR\x04rac\
    e\x12\x1d\n\nbuild_time\x18\x11\x20\x01(\x02R\tbuildTime\x12\x1f\n\x0bha\
    s_vespene\x18\x13\x20\x01(\x08R\nhasVespene\x12!\n\x0chas_minerals\x18\
    \x14\x20\x01(\x08R\x0bhasMinerals\x12\x1d\n\ntech_alias\x18\x15\x20\x03(\
    \rR\ttechAlias\x12\x1d\n\nunit_alias\x18\x16\x20\x01(\rR\tunitAlias\x12)\
    \n\x10tech_requirement\x18\x17\x20\x01(\rR\x0ftechRequirement\x12)\n\x10\
    require_attached\x18\x18\x20\x01(\x08R\x0frequireAttached\x129\n\nattrib\
    utes\x18\x08\x20\x03(\x0e2\x19.SC2APIProtocol.AttributeR\nattributes\x12\
    %\n\x0emovement_speed\x18\t\x20\x01(\x02R\rmovementSpeed\x12\x14\n\x05ar\
    mor\x18\n\x20\x01(\x02R\x05armor\x120\n\x07weapons\x18\x0b\x20\x03(\x0b2\
    \x16.SC2APIProtocol.WeaponR\x07weapons\"\xca\x01\n\x0bUpgradeData\x12\
    \x1d\n\nupgrade_id\x18\x01\x20\x01(\rR\tupgradeId\x12\x12\n\x04name\x18\
    \x02\x20\x01(\tR\x04name\x12!\n\x0cmineral_cost\x18\x03\x20\x01(\rR\x0bm\
    ineralCost\x12!\n\x0cvespene_cost\x18\x04\x20\x01(\rR\x0bvespeneCost\x12\
    #\n\rresearch_time\x18\x05\x20\x01(\x02R\x0cresearchTime\x12\x1d\n\nabil\
    ity_id\x18\x06\x20\x01(\rR\tabilityId\"7\n\x08BuffData\x12\x17\n\x07buff\
    _id\x18\x01\x20\x01(\rR\x06buffId\x12\x12\n\x04name\x18\x02\x20\x01(\tR\
    \x04name*\x9e\x01\n\tAttribute\x12\t\n\x05Light\x10\x01\x12\x0b\n\x07Arm\
    ored\x10\x02\x12\x0e\n\nBiological\x10\x03\x12\x0e\n\nMechanical\x10\x04\
    \x12\x0b\n\x07Robotic\x10\x05\x12\x0b\n\x07Psionic\x10\x06\x12\x0b\n\x07\
    Massive\x10\x07\x12\r\n\tStructure\x10\x08\x12\t\n\x05Hover\x10\t\x12\n\
    \n\x06Heroic\x10\n\x12\x0c\n\x08Summoned\x10\x0bJ\xcd2\n\x06\x12\x04\x01\
    \0o\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\x08\n\x01\x02\x12\x03\x03\
    \x08\x16\n\t\n\x02\x03\0\x12\x03\x05\x07&\n\xe4\x01\n\x02\x04\0\x12\x04\
    \x0b\0$\x01\x1a\xd7\x01\x20May\x20not\x20relevant:\x20queueable\x20(ever\
    ything\x20is\x20queueable).\n\x20May\x20not\x20be\x20important:\x20AbilS\
    etId\x20-\x20marine\x20stim,\x20marauder\x20stim.\n\x20Stuff\x20omitted:\
    \x20transient.\n\x20Stuff\x20that\x20may\x20be\x20important:\x20cost,\
    \x20range,\x20Alignment,\x20targetfilters.\n\n\n\n\x03\x04\0\x01\x12\x03\
    \x0b\x08\x13\n\x19\n\x04\x04\0\x02\0\x12\x03\x0c\x02!\"\x0c\x20Stable\
    \x20ID.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x0c\x02\n\n\x0c\n\x05\x04\
    \0\x02\0\x05\x12\x03\x0c\x0b\x11\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\
    \x12\x1c\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c\x1f\x20\n+\n\x04\x04\0\
    \x02\x01\x12\x03\r\x02\x20\"\x1e\x20Catalog\x20name\x20of\x20the\x20abil\
    ity.\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\r\x02\n\n\x0c\n\x05\x04\0\
    \x02\x01\x05\x12\x03\r\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\r\
    \x12\x1b\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\r\x1e\x1f\n,\n\x04\x04\0\
    \x02\x02\x12\x03\x0e\x02!\"\x1f\x20Catalog\x20index\x20of\x20the\x20abil\
    ity.\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x0e\x02\n\n\x0c\n\x05\x04\0\
    \x02\x02\x05\x12\x03\x0e\x0b\x11\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\
    \x0e\x12\x1c\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0e\x1f\x20\nE\n\x04\
    \x04\0\x02\x03\x12\x03\x0f\x02\"\"8\x20Name\x20used\x20for\x20the\x20com\
    mand\x20card.\x20May\x20not\x20always\x20be\x20set.\n\n\x0c\n\x05\x04\0\
    \x02\x03\x04\x12\x03\x0f\x02\n\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x0f\
    \x0b\x11\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0f\x12\x1d\n\x0c\n\x05\
    \x04\0\x02\x03\x03\x12\x03\x0f\x20!\nY\n\x04\x04\0\x02\x04\x12\x03\x10\
    \x02$\"L\x20A\x20human\x20friendly\x20name\x20when\x20the\x20button\x20n\
    ame\x20or\x20link\x20name\x20isn't\x20descriptive.\n\n\x0c\n\x05\x04\0\
    \x02\x04\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x10\
    \x0b\x11\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x10\x12\x1f\n\x0c\n\x05\
    \x04\0\x02\x04\x03\x12\x03\x10\"#\n-\n\x04\x04\0\x02\x05\x12\x03\x11\x02\
    \x1d\"\x20\x20Hotkey.\x20May\x20not\x20always\x20be\x20set.\n\n\x0c\n\
    \x05\x04\0\x02\x05\x04\x12\x03\x11\x02\n\n\x0c\n\x05\x04\0\x02\x05\x05\
    \x12\x03\x11\x0b\x11\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x11\x12\x18\n\
    \x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x11\x1b\x1c\nO\n\x04\x04\0\x02\x06\
    \x12\x03\x12\x02+\"B\x20This\x20ability\x20id\x20may\x20be\x20represente\
    d\x20by\x20the\x20given\x20more\x20generic\x20id.\n\n\x0c\n\x05\x04\0\
    \x02\x06\x04\x12\x03\x12\x02\n\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\x12\
    \x0b\x11\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x12\x12&\n\x0c\n\x05\x04\
    \0\x02\x06\x03\x12\x03\x12)*\n\x0c\n\x04\x04\0\x04\0\x12\x04\x14\x02\x1a\
    \x03\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\x14\x07\r\n+\n\x06\x04\0\x04\0\
    \x02\0\x12\x03\x15\x04\r\"\x1c\x20Does\x20not\x20require\x20a\x20target.\
    \n\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03\x15\x04\x08\n\x0e\n\x07\x04\
    \0\x04\0\x02\0\x02\x12\x03\x15\x0b\x0c\n,\n\x06\x04\0\x04\0\x02\x01\x12\
    \x03\x16\x04\x0e\"\x1d\x20Requires\x20a\x20target\x20position.\n\n\x0e\n\
    \x07\x04\0\x04\0\x02\x01\x01\x12\x03\x16\x04\t\n\x0e\n\x07\x04\0\x04\0\
    \x02\x01\x02\x12\x03\x16\x0c\r\nS\n\x06\x04\0\x04\0\x02\x02\x12\x03\x17\
    \x04\r\"D\x20Requires\x20a\x20unit\x20to\x20target.\x20Given\x20by\x20po\
    sition\x20using\x20feature\x20layers.\n\n\x0e\n\x07\x04\0\x04\0\x02\x02\
    \x01\x12\x03\x17\x04\x08\n\x0e\n\x07\x04\0\x04\0\x02\x02\x02\x12\x03\x17\
    \x0b\x0c\n?\n\x06\x04\0\x04\0\x02\x03\x12\x03\x18\x04\x14\"0\x20Requires\
    \x20either\x20a\x20target\x20point\x20or\x20target\x20unit.\n\n\x0e\n\
    \x07\x04\0\x04\0\x02\x03\x01\x12\x03\x18\x04\x0f\n\x0e\n\x07\x04\0\x04\0\
    \x02\x03\x02\x12\x03\x18\x12\x13\nT\n\x06\x04\0\x04\0\x02\x04\x12\x03\
    \x19\x04\x14\"E\x20Requires\x20either\x20a\x20target\x20point\x20or\x20n\
    o\x20target.\x20(eg.\x20building\x20add-ons)\n\n\x0e\n\x07\x04\0\x04\0\
    \x02\x04\x01\x12\x03\x19\x04\x0f\n\x0e\n\x07\x04\0\x04\0\x02\x04\x02\x12\
    \x03\x19\x12\x13\nH\n\x04\x04\0\x02\x07\x12\x03\x1c\x02\x1e\";\x20If\x20\
    true,\x20the\x20ability\x20may\x20be\x20used\x20by\x20this\x20set\x20of\
    \x20mods/map.\n\n\x0c\n\x05\x04\0\x02\x07\x04\x12\x03\x1c\x02\n\n\x0c\n\
    \x05\x04\0\x02\x07\x05\x12\x03\x1c\x0b\x0f\n\x0c\n\x05\x04\0\x02\x07\x01\
    \x12\x03\x1c\x10\x19\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x1c\x1c\x1d\n\
    =\n\x04\x04\0\x02\x08\x12\x03\x1d\x02\x1d\"0\x20Determines\x20if\x20a\
    \x20point\x20is\x20optional\x20or\x20required.\n\n\x0c\n\x05\x04\0\x02\
    \x08\x04\x12\x03\x1d\x02\n\n\x0c\n\x05\x04\0\x02\x08\x06\x12\x03\x1d\x0b\
    \x11\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x1d\x12\x18\n\x0c\n\x05\x04\0\
    \x02\x08\x03\x12\x03\x1d\x1b\x1c\n*\n\x04\x04\0\x02\t\x12\x03\x1e\x02#\"\
    \x1d\x20Can\x20be\x20cast\x20in\x20the\x20minimap.\n\n\x0c\n\x05\x04\0\
    \x02\t\x04\x12\x03\x1e\x02\n\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03\x1e\x0b\
    \x0f\n\x0c\n\x05\x04\0\x02\t\x01\x12\x03\x1e\x10\x1d\n\x0c\n\x05\x04\0\
    \x02\t\x03\x12\x03\x1e\x20\"\n#\n\x04\x04\0\x02\n\x12\x03\x1f\x02$\"\x16\
    \x20Autocast\x20can\x20be\x20set.\n\n\x0c\n\x05\x04\0\x02\n\x04\x12\x03\
    \x1f\x02\n\n\x0c\n\x05\x04\0\x02\n\x05\x12\x03\x1f\x0b\x0f\n\x0c\n\x05\
    \x04\0\x02\n\x01\x12\x03\x1f\x10\x1e\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03\
    \x1f!#\n:\n\x04\x04\0\x02\x0b\x12\x03\x20\x02!\"-\x20Requires\x20placeme\
    nt\x20to\x20construct\x20a\x20building.\n\n\x0c\n\x05\x04\0\x02\x0b\x04\
    \x12\x03\x20\x02\n\n\x0c\n\x05\x04\0\x02\x0b\x05\x12\x03\x20\x0b\x0f\n\
    \x0c\n\x05\x04\0\x02\x0b\x01\x12\x03\x20\x10\x1b\n\x0c\n\x05\x04\0\x02\
    \x0b\x03\x12\x03\x20\x1e\x20\nI\n\x04\x04\0\x02\x0c\x12\x03!\x02'\"<\x20\
    Estimation\x20of\x20the\x20footprint\x20size.\x20Need\x20a\x20better\x20\
    footprint.\n\n\x0c\n\x05\x04\0\x02\x0c\x04\x12\x03!\x02\n\n\x0c\n\x05\
    \x04\0\x02\x0c\x05\x12\x03!\x0b\x10\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\
    \x03!\x11!\n\x0c\n\x05\x04\0\x02\x0c\x03\x12\x03!$&\nX\n\x04\x04\0\x02\r\
    \x12\x03\"\x02*\"K\x20Placement\x20next\x20to\x20an\x20existing\x20struc\
    ture,\x20e.g.,\x20an\x20add-on\x20like\x20a\x20Tech\x20Lab.\n\n\x0c\n\
    \x05\x04\0\x02\r\x04\x12\x03\"\x02\n\n\x0c\n\x05\x04\0\x02\r\x05\x12\x03\
    \"\x0b\x0f\n\x0c\n\x05\x04\0\x02\r\x01\x12\x03\"\x10$\n\x0c\n\x05\x04\0\
    \x02\r\x03\x12\x03\"')\nN\n\x04\x04\0\x02\x0e\x12\x03#\x02!\"A\x20Range\
    \x20unit\x20can\x20cast\x20ability\x20without\x20needing\x20to\x20approa\
    ch\x20target.\n\n\x0c\n\x05\x04\0\x02\x0e\x04\x12\x03#\x02\n\n\x0c\n\x05\
    \x04\0\x02\x0e\x05\x12\x03#\x0b\x10\n\x0c\n\x05\x04\0\x02\x0e\x01\x12\
    \x03#\x11\x1b\n\x0c\n\x05\x04\0\x02\x0e\x03\x12\x03#\x1e\x20\n\n\n\x02\
    \x05\0\x12\x04&\02\x01\n\n\n\x03\x05\0\x01\x12\x03&\x05\x0e\n\x0b\n\x04\
    \x05\0\x02\0\x12\x03'\x02\x0c\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03'\x02\
    \x07\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03'\n\x0b\n\x0b\n\x04\x05\0\x02\
    \x01\x12\x03(\x02\x0e\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03(\x02\t\n\x0c\
    \n\x05\x05\0\x02\x01\x02\x12\x03(\x0c\r\n\x0b\n\x04\x05\0\x02\x02\x12\
    \x03)\x02\x11\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03)\x02\x0c\n\x0c\n\x05\
    \x05\0\x02\x02\x02\x12\x03)\x0f\x10\n\x0b\n\x04\x05\0\x02\x03\x12\x03*\
    \x02\x11\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03*\x02\x0c\n\x0c\n\x05\x05\
    \0\x02\x03\x02\x12\x03*\x0f\x10\n\x0b\n\x04\x05\0\x02\x04\x12\x03+\x02\
    \x0e\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03+\x02\t\n\x0c\n\x05\x05\0\x02\
    \x04\x02\x12\x03+\x0c\r\n\x0b\n\x04\x05\0\x02\x05\x12\x03,\x02\x0e\n\x0c\
    \n\x05\x05\0\x02\x05\x01\x12\x03,\x02\t\n\x0c\n\x05\x05\0\x02\x05\x02\
    \x12\x03,\x0c\r\n\x0b\n\x04\x05\0\x02\x06\x12\x03-\x02\x0e\n\x0c\n\x05\
    \x05\0\x02\x06\x01\x12\x03-\x02\t\n\x0c\n\x05\x05\0\x02\x06\x02\x12\x03-\
    \x0c\r\n\x0b\n\x04\x05\0\x02\x07\x12\x03.\x02\x10\n\x0c\n\x05\x05\0\x02\
    \x07\x01\x12\x03.\x02\x0b\n\x0c\n\x05\x05\0\x02\x07\x02\x12\x03.\x0e\x0f\
    \n\x0b\n\x04\x05\0\x02\x08\x12\x03/\x02\x0c\n\x0c\n\x05\x05\0\x02\x08\
    \x01\x12\x03/\x02\x07\n\x0c\n\x05\x05\0\x02\x08\x02\x12\x03/\n\x0b\n\x0b\
    \n\x04\x05\0\x02\t\x12\x030\x02\x0e\n\x0c\n\x05\x05\0\x02\t\x01\x12\x030\
    \x02\x08\n\x0c\n\x05\x05\0\x02\t\x02\x12\x030\x0b\r\n\x0b\n\x04\x05\0\
    \x02\n\x12\x031\x02\x10\n\x0c\n\x05\x05\0\x02\n\x01\x12\x031\x02\n\n\x0c\
    \n\x05\x05\0\x02\n\x02\x12\x031\r\x0f\n\n\n\x02\x04\x01\x12\x044\07\x01\
    \n\n\n\x03\x04\x01\x01\x12\x034\x08\x13\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x035\x02#\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x035\x02\n\n\x0c\n\x05\x04\
    \x01\x02\0\x06\x12\x035\x0b\x14\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x035\
    \x15\x1e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x035!\"\n\x0b\n\x04\x04\x01\
    \x02\x01\x12\x036\x02\x1b\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x036\x02\n\
    \n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x036\x0b\x10\n\x0c\n\x05\x04\x01\
    \x02\x01\x01\x12\x036\x11\x16\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x036\
    \x19\x1a\n\n\n\x02\x04\x02\x12\x049\0E\x01\n\n\n\x03\x04\x02\x01\x12\x03\
    9\x08\x0e\n\x0c\n\x04\x04\x02\x04\0\x12\x04:\x02>\x03\n\x0c\n\x05\x04\
    \x02\x04\0\x01\x12\x03:\x07\x11\n\r\n\x06\x04\x02\x04\0\x02\0\x12\x03;\
    \x04\x0f\n\x0e\n\x07\x04\x02\x04\0\x02\0\x01\x12\x03;\x04\n\n\x0e\n\x07\
    \x04\x02\x04\0\x02\0\x02\x12\x03;\r\x0e\n\r\n\x06\x04\x02\x04\0\x02\x01\
    \x12\x03<\x04\x0c\n\x0e\n\x07\x04\x02\x04\0\x02\x01\x01\x12\x03<\x04\x07\
    \n\x0e\n\x07\x04\x02\x04\0\x02\x01\x02\x12\x03<\n\x0b\n\r\n\x06\x04\x02\
    \x04\0\x02\x02\x12\x03=\x04\x0c\n\x0e\n\x07\x04\x02\x04\0\x02\x02\x01\
    \x12\x03=\x04\x07\n\x0e\n\x07\x04\x02\x04\0\x02\x02\x02\x12\x03=\n\x0b\n\
    \x0b\n\x04\x04\x02\x02\0\x12\x03?\x02\x1f\n\x0c\n\x05\x04\x02\x02\0\x04\
    \x12\x03?\x02\n\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03?\x0b\x15\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03?\x16\x1a\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03?\x1d\x1e\n\x0b\n\x04\x04\x02\x02\x01\x12\x03@\x02\x1c\n\x0c\n\
    \x05\x04\x02\x02\x01\x04\x12\x03@\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\
    \x12\x03@\x0b\x10\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03@\x11\x17\n\x0c\
    \n\x05\x04\x02\x02\x01\x03\x12\x03@\x1a\x1b\n\x0b\n\x04\x04\x02\x02\x02\
    \x12\x03A\x02(\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03A\x02\n\n\x0c\n\
    \x05\x04\x02\x02\x02\x06\x12\x03A\x0b\x16\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03A\x17#\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03A&'\nD\n\x04\
    \x04\x02\x02\x03\x12\x03B\x02\x1e\"7\x20Number\x20of\x20hits\x20per\x20a\
    ttack.\x20(eg.\x20Colossus\x20has\x202\x20beams)\n\n\x0c\n\x05\x04\x02\
    \x02\x03\x04\x12\x03B\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03B\x0b\
    \x11\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03B\x12\x19\n\x0c\n\x05\x04\
    \x02\x02\x03\x03\x12\x03B\x1c\x1d\n\x0b\n\x04\x04\x02\x02\x04\x12\x03C\
    \x02\x1b\n\x0c\n\x05\x04\x02\x02\x04\x04\x12\x03C\x02\n\n\x0c\n\x05\x04\
    \x02\x02\x04\x05\x12\x03C\x0b\x10\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\
    \x03C\x11\x16\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03C\x19\x1a\n$\n\x04\
    \x04\x02\x02\x05\x12\x03D\x02\x1b\"\x17\x20Time\x20between\x20attacks.\n\
    \n\x0c\n\x05\x04\x02\x02\x05\x04\x12\x03D\x02\n\n\x0c\n\x05\x04\x02\x02\
    \x05\x05\x12\x03D\x0b\x10\n\x0c\n\x05\x04\x02\x02\x05\x01\x12\x03D\x11\
    \x16\n\x0c\n\x05\x04\x02\x02\x05\x03\x12\x03D\x19\x1a\n\n\n\x02\x04\x03\
    \x12\x04G\0a\x01\n\n\n\x03\x04\x03\x01\x12\x03G\x08\x14\n\x19\n\x04\x04\
    \x03\x02\0\x12\x03H\x02\x1e\"\x0c\x20Stable\x20ID.\n\n\x0c\n\x05\x04\x03\
    \x02\0\x04\x12\x03H\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03H\x0b\x11\
    \n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03H\x12\x19\n\x0c\n\x05\x04\x03\x02\
    \0\x03\x12\x03H\x1c\x1d\n(\n\x04\x04\x03\x02\x01\x12\x03I\x02\x1b\"\x1b\
    \x20Catalog\x20name\x20of\x20the\x20unit.\n\n\x0c\n\x05\x04\x03\x02\x01\
    \x04\x12\x03I\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03I\x0b\x11\n\
    \x0c\n\x05\x04\x03\x02\x01\x01\x12\x03I\x12\x16\n\x0c\n\x05\x04\x03\x02\
    \x01\x03\x12\x03I\x19\x1a\nH\n\x04\x04\x03\x02\x02\x12\x03J\x02\x1e\";\
    \x20If\x20true,\x20the\x20ability\x20may\x20be\x20used\x20by\x20this\x20\
    set\x20of\x20mods/map.\n\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03J\x02\n\
    \n\x0c\n\x05\x04\x03\x02\x02\x05\x12\x03J\x0b\x0f\n\x0c\n\x05\x04\x03\
    \x02\x02\x01\x12\x03J\x10\x19\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03J\
    \x1c\x1d\n?\n\x04\x04\x03\x02\x03\x12\x03K\x02!\"2\x20Number\x20of\x20ca\
    rgo\x20slots\x20it\x20occupies\x20in\x20transports.\n\n\x0c\n\x05\x04\
    \x03\x02\x03\x04\x12\x03K\x02\n\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03K\
    \x0b\x11\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03K\x12\x1c\n\x0c\n\x05\
    \x04\x03\x02\x03\x03\x12\x03K\x1f\x20\n\x0b\n\x04\x04\x03\x02\x04\x12\
    \x03L\x02$\n\x0c\n\x05\x04\x03\x02\x04\x04\x12\x03L\x02\n\n\x0c\n\x05\
    \x04\x03\x02\x04\x05\x12\x03L\x0b\x11\n\x0c\n\x05\x04\x03\x02\x04\x01\
    \x12\x03L\x12\x1e\n\x0c\n\x05\x04\x03\x02\x04\x03\x12\x03L!#\n\x0b\n\x04\
    \x04\x03\x02\x05\x12\x03M\x02$\n\x0c\n\x05\x04\x03\x02\x05\x04\x12\x03M\
    \x02\n\n\x0c\n\x05\x04\x03\x02\x05\x05\x12\x03M\x0b\x11\n\x0c\n\x05\x04\
    \x03\x02\x05\x01\x12\x03M\x12\x1e\n\x0c\n\x05\x04\x03\x02\x05\x03\x12\
    \x03M!#\n\x0b\n\x04\x04\x03\x02\x06\x12\x03N\x02$\n\x0c\n\x05\x04\x03\
    \x02\x06\x04\x12\x03N\x02\n\n\x0c\n\x05\x04\x03\x02\x06\x05\x12\x03N\x0b\
    \x10\n\x0c\n\x05\x04\x03\x02\x06\x01\x12\x03N\x11\x1e\n\x0c\n\x05\x04\
    \x03\x02\x06\x03\x12\x03N!#\n\x0b\n\x04\x04\x03\x02\x07\x12\x03O\x02$\n\
    \x0c\n\x05\x04\x03\x02\x07\x04\x12\x03O\x02\n\n\x0c\n\x05\x04\x03\x02\
    \x07\x05\x12\x03O\x0b\x10\n\x0c\n\x05\x04\x03\x02\x07\x01\x12\x03O\x11\
    \x1e\n\x0c\n\x05\x04\x03\x02\x07\x03\x12\x03O!#\n6\n\x04\x04\x03\x02\x08\
    \x12\x03P\x02\"\")\x20This\x20is\x20the\x20ability\x20the\x20builds\x20t\
    he\x20unit\n\n\x0c\n\x05\x04\x03\x02\x08\x04\x12\x03P\x02\n\n\x0c\n\x05\
    \x04\x03\x02\x08\x05\x12\x03P\x0b\x11\n\x0c\n\x05\x04\x03\x02\x08\x01\
    \x12\x03P\x12\x1c\n\x0c\n\x05\x04\x03\x02\x08\x03\x12\x03P\x1f!\n\x0b\n\
    \x04\x04\x03\x02\t\x12\x03Q\x02\x1a\n\x0c\n\x05\x04\x03\x02\t\x04\x12\
    \x03Q\x02\n\n\x0c\n\x05\x04\x03\x02\t\x06\x12\x03Q\x0b\x0f\n\x0c\n\x05\
    \x04\x03\x02\t\x01\x12\x03Q\x10\x14\n\x0c\n\x05\x04\x03\x02\t\x03\x12\
    \x03Q\x17\x19\n\x0b\n\x04\x04\x03\x02\n\x12\x03R\x02!\n\x0c\n\x05\x04\
    \x03\x02\n\x04\x12\x03R\x02\n\n\x0c\n\x05\x04\x03\x02\n\x05\x12\x03R\x0b\
    \x10\n\x0c\n\x05\x04\x03\x02\n\x01\x12\x03R\x11\x1b\n\x0c\n\x05\x04\x03\
    \x02\n\x03\x12\x03R\x1e\x20\n\x0b\n\x04\x04\x03\x02\x0b\x12\x03S\x02!\n\
    \x0c\n\x05\x04\x03\x02\x0b\x04\x12\x03S\x02\n\n\x0c\n\x05\x04\x03\x02\
    \x0b\x05\x12\x03S\x0b\x0f\n\x0c\n\x05\x04\x03\x02\x0b\x01\x12\x03S\x10\
    \x1b\n\x0c\n\x05\x04\x03\x02\x0b\x03\x12\x03S\x1e\x20\n\x0b\n\x04\x04\
    \x03\x02\x0c\x12\x03T\x02\"\n\x0c\n\x05\x04\x03\x02\x0c\x04\x12\x03T\x02\
    \n\n\x0c\n\x05\x04\x03\x02\x0c\x05\x12\x03T\x0b\x0f\n\x0c\n\x05\x04\x03\
    \x02\x0c\x01\x12\x03T\x10\x1c\n\x0c\n\x05\x04\x03\x02\x0c\x03\x12\x03T\
    \x1f!\nS\n\x04\x04\x03\x02\r\x12\x03V\x02\"\"F\x20Units\x20this\x20is\
    \x20equivalent\x20to\x20in\x20terms\x20of\x20satisfying\x20tech\x20requi\
    rement.\n\n\x0c\n\x05\x04\x03\x02\r\x04\x12\x03V\x02\n\n\x0c\n\x05\x04\
    \x03\x02\r\x05\x12\x03V\x0b\x11\n\x0c\n\x05\x04\x03\x02\r\x01\x12\x03V\
    \x12\x1c\n\x0c\n\x05\x04\x03\x02\r\x03\x12\x03V\x1f!\n@\n\x04\x04\x03\
    \x02\x0e\x12\x03W\x02\"\"3\x20Units\x20that\x20are\x20morphed\x20variant\
    s\x20of\x20the\x20same\x20unit.\n\n\x0c\n\x05\x04\x03\x02\x0e\x04\x12\
    \x03W\x02\n\n\x0c\n\x05\x04\x03\x02\x0e\x05\x12\x03W\x0b\x11\n\x0c\n\x05\
    \x04\x03\x02\x0e\x01\x12\x03W\x12\x1c\n\x0c\n\x05\x04\x03\x02\x0e\x03\
    \x12\x03W\x1f!\nW\n\x04\x04\x03\x02\x0f\x12\x03Y\x02(\"J\x20Structure\
    \x20required\x20to\x20build\x20this\x20unit.\x20(Or\x20any\x20with\x20th\
    e\x20same\x20tech_alias)\n\n\x0c\n\x05\x04\x03\x02\x0f\x04\x12\x03Y\x02\
    \n\n\x0c\n\x05\x04\x03\x02\x0f\x05\x12\x03Y\x0b\x11\n\x0c\n\x05\x04\x03\
    \x02\x0f\x01\x12\x03Y\x12\"\n\x0c\n\x05\x04\x03\x02\x0f\x03\x12\x03Y%'\n\
    5\n\x04\x04\x03\x02\x10\x12\x03Z\x02&\"(\x20Whether\x20tech_requirement\
    \x20is\x20an\x20add-on.\n\n\x0c\n\x05\x04\x03\x02\x10\x04\x12\x03Z\x02\n\
    \n\x0c\n\x05\x04\x03\x02\x10\x05\x12\x03Z\x0b\x0f\n\x0c\n\x05\x04\x03\
    \x02\x10\x01\x12\x03Z\x10\x20\n\x0c\n\x05\x04\x03\x02\x10\x03\x12\x03Z#%\
    \n3\n\x04\x04\x03\x02\x11\x12\x03]\x02$\x1a&\x20Values\x20include\x20cha\
    nges\x20from\x20upgrades\n\n\x0c\n\x05\x04\x03\x02\x11\x04\x12\x03]\x02\
    \n\n\x0c\n\x05\x04\x03\x02\x11\x06\x12\x03]\x0b\x14\n\x0c\n\x05\x04\x03\
    \x02\x11\x01\x12\x03]\x15\x1f\n\x0c\n\x05\x04\x03\x02\x11\x03\x12\x03]\"\
    #\n\x0b\n\x04\x04\x03\x02\x12\x12\x03^\x02$\n\x0c\n\x05\x04\x03\x02\x12\
    \x04\x12\x03^\x02\n\n\x0c\n\x05\x04\x03\x02\x12\x05\x12\x03^\x0b\x10\n\
    \x0c\n\x05\x04\x03\x02\x12\x01\x12\x03^\x11\x1f\n\x0c\n\x05\x04\x03\x02\
    \x12\x03\x12\x03^\"#\n\x0b\n\x04\x04\x03\x02\x13\x12\x03_\x02\x1c\n\x0c\
    \n\x05\x04\x03\x02\x13\x04\x12\x03_\x02\n\n\x0c\n\x05\x04\x03\x02\x13\
    \x05\x12\x03_\x0b\x10\n\x0c\n\x05\x04\x03\x02\x13\x01\x12\x03_\x11\x16\n\
    \x0c\n\x05\x04\x03\x02\x13\x03\x12\x03_\x19\x1b\n\x0b\n\x04\x04\x03\x02\
    \x14\x12\x03`\x02\x1f\n\x0c\n\x05\x04\x03\x02\x14\x04\x12\x03`\x02\n\n\
    \x0c\n\x05\x04\x03\x02\x14\x06\x12\x03`\x0b\x11\n\x0c\n\x05\x04\x03\x02\
    \x14\x01\x12\x03`\x12\x19\n\x0c\n\x05\x04\x03\x02\x14\x03\x12\x03`\x1c\
    \x1e\n\n\n\x02\x04\x04\x12\x04c\0j\x01\n\n\n\x03\x04\x04\x01\x12\x03c\
    \x08\x13\n\x19\n\x04\x04\x04\x02\0\x12\x03d\x02!\"\x0c\x20Stable\x20ID.\
    \n\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03d\x02\n\n\x0c\n\x05\x04\x04\x02\
    \0\x05\x12\x03d\x0b\x11\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03d\x12\x1c\n\
    \x0c\n\x05\x04\x04\x02\0\x03\x12\x03d\x1f\x20\n\x0b\n\x04\x04\x04\x02\
    \x01\x12\x03e\x02\x1b\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03e\x02\n\n\
    \x0c\n\x05\x04\x04\x02\x01\x05\x12\x03e\x0b\x11\n\x0c\n\x05\x04\x04\x02\
    \x01\x01\x12\x03e\x12\x16\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03e\x19\
    \x1a\n\x0b\n\x04\x04\x04\x02\x02\x12\x03f\x02#\n\x0c\n\x05\x04\x04\x02\
    \x02\x04\x12\x03f\x02\n\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03f\x0b\x11\
    \n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03f\x12\x1e\n\x0c\n\x05\x04\x04\
    \x02\x02\x03\x12\x03f!\"\n\x0b\n\x04\x04\x04\x02\x03\x12\x03g\x02#\n\x0c\
    \n\x05\x04\x04\x02\x03\x04\x12\x03g\x02\n\n\x0c\n\x05\x04\x04\x02\x03\
    \x05\x12\x03g\x0b\x11\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03g\x12\x1e\n\
    \x0c\n\x05\x04\x04\x02\x03\x03\x12\x03g!\"\n\x0b\n\x04\x04\x04\x02\x04\
    \x12\x03h\x02#\n\x0c\n\x05\x04\x04\x02\x04\x04\x12\x03h\x02\n\n\x0c\n\
    \x05\x04\x04\x02\x04\x05\x12\x03h\x0b\x10\n\x0c\n\x05\x04\x04\x02\x04\
    \x01\x12\x03h\x11\x1e\n\x0c\n\x05\x04\x04\x02\x04\x03\x12\x03h!\"\n\x0b\
    \n\x04\x04\x04\x02\x05\x12\x03i\x02!\n\x0c\n\x05\x04\x04\x02\x05\x04\x12\
    \x03i\x02\n\n\x0c\n\x05\x04\x04\x02\x05\x05\x12\x03i\x0b\x11\n\x0c\n\x05\
    \x04\x04\x02\x05\x01\x12\x03i\x12\x1c\n\x0c\n\x05\x04\x04\x02\x05\x03\
    \x12\x03i\x1f\x20\n\n\n\x02\x04\x05\x12\x04l\0o\x01\n\n\n\x03\x04\x05\
    \x01\x12\x03l\x08\x10\n\x19\n\x04\x04\x05\x02\0\x12\x03m\x02\x1e\"\x0c\
    \x20Stable\x20ID.\n\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03m\x02\n\n\x0c\n\
    \x05\x04\x05\x02\0\x05\x12\x03m\x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\
    \x12\x03m\x12\x19\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03m\x1c\x1d\n\x0b\n\
    \x04\x04\x05\x02\x01\x12\x03n\x02\x1b\n\x0c\n\x05\x04\x05\x02\x01\x04\
    \x12\x03n\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03n\x0b\x11\n\x0c\n\
    \x05\x04\x05\x02\x01\x01\x12\x03n\x12\x16\n\x0c\n\x05\x04\x05\x02\x01\
    \x03\x12\x03n\x19\x1a\
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
