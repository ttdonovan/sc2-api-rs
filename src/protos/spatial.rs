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
pub struct ObservationFeatureLayer {
    // message fields
    renders: ::protobuf::SingularPtrField<FeatureLayers>,
    minimap_renders: ::protobuf::SingularPtrField<FeatureLayersMinimap>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObservationFeatureLayer {}

impl ObservationFeatureLayer {
    pub fn new() -> ObservationFeatureLayer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObservationFeatureLayer {
        static mut instance: ::protobuf::lazy::Lazy<ObservationFeatureLayer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObservationFeatureLayer,
        };
        unsafe {
            instance.get(ObservationFeatureLayer::new)
        }
    }

    // optional .SC2APIProtocol.FeatureLayers renders = 1;

    pub fn clear_renders(&mut self) {
        self.renders.clear();
    }

    pub fn has_renders(&self) -> bool {
        self.renders.is_some()
    }

    // Param is passed by value, moved
    pub fn set_renders(&mut self, v: FeatureLayers) {
        self.renders = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_renders(&mut self) -> &mut FeatureLayers {
        if self.renders.is_none() {
            self.renders.set_default();
        }
        self.renders.as_mut().unwrap()
    }

    // Take field
    pub fn take_renders(&mut self) -> FeatureLayers {
        self.renders.take().unwrap_or_else(|| FeatureLayers::new())
    }

    pub fn get_renders(&self) -> &FeatureLayers {
        self.renders.as_ref().unwrap_or_else(|| FeatureLayers::default_instance())
    }

    fn get_renders_for_reflect(&self) -> &::protobuf::SingularPtrField<FeatureLayers> {
        &self.renders
    }

    fn mut_renders_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FeatureLayers> {
        &mut self.renders
    }

    // optional .SC2APIProtocol.FeatureLayersMinimap minimap_renders = 2;

    pub fn clear_minimap_renders(&mut self) {
        self.minimap_renders.clear();
    }

    pub fn has_minimap_renders(&self) -> bool {
        self.minimap_renders.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimap_renders(&mut self, v: FeatureLayersMinimap) {
        self.minimap_renders = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_minimap_renders(&mut self) -> &mut FeatureLayersMinimap {
        if self.minimap_renders.is_none() {
            self.minimap_renders.set_default();
        }
        self.minimap_renders.as_mut().unwrap()
    }

    // Take field
    pub fn take_minimap_renders(&mut self) -> FeatureLayersMinimap {
        self.minimap_renders.take().unwrap_or_else(|| FeatureLayersMinimap::new())
    }

    pub fn get_minimap_renders(&self) -> &FeatureLayersMinimap {
        self.minimap_renders.as_ref().unwrap_or_else(|| FeatureLayersMinimap::default_instance())
    }

    fn get_minimap_renders_for_reflect(&self) -> &::protobuf::SingularPtrField<FeatureLayersMinimap> {
        &self.minimap_renders
    }

    fn mut_minimap_renders_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FeatureLayersMinimap> {
        &mut self.minimap_renders
    }
}

impl ::protobuf::Message for ObservationFeatureLayer {
    fn is_initialized(&self) -> bool {
        for v in &self.renders {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.minimap_renders {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.renders)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.minimap_renders)?;
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
        if let Some(ref v) = self.renders.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.minimap_renders.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.renders.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.minimap_renders.as_ref() {
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

impl ::protobuf::MessageStatic for ObservationFeatureLayer {
    fn new() -> ObservationFeatureLayer {
        ObservationFeatureLayer::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObservationFeatureLayer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FeatureLayers>>(
                    "renders",
                    ObservationFeatureLayer::get_renders_for_reflect,
                    ObservationFeatureLayer::mut_renders_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FeatureLayersMinimap>>(
                    "minimap_renders",
                    ObservationFeatureLayer::get_minimap_renders_for_reflect,
                    ObservationFeatureLayer::mut_minimap_renders_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObservationFeatureLayer>(
                    "ObservationFeatureLayer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObservationFeatureLayer {
    fn clear(&mut self) {
        self.clear_renders();
        self.clear_minimap_renders();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ObservationFeatureLayer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ObservationFeatureLayer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FeatureLayers {
    // message fields
    height_map: ::protobuf::SingularPtrField<super::common::ImageData>,
    visibility_map: ::protobuf::SingularPtrField<super::common::ImageData>,
    creep: ::protobuf::SingularPtrField<super::common::ImageData>,
    power: ::protobuf::SingularPtrField<super::common::ImageData>,
    player_id: ::protobuf::SingularPtrField<super::common::ImageData>,
    unit_type: ::protobuf::SingularPtrField<super::common::ImageData>,
    selected: ::protobuf::SingularPtrField<super::common::ImageData>,
    unit_hit_points: ::protobuf::SingularPtrField<super::common::ImageData>,
    unit_hit_points_ratio: ::protobuf::SingularPtrField<super::common::ImageData>,
    unit_energy: ::protobuf::SingularPtrField<super::common::ImageData>,
    unit_shields: ::protobuf::SingularPtrField<super::common::ImageData>,
    player_relative: ::protobuf::SingularPtrField<super::common::ImageData>,
    unit_density_aa: ::protobuf::SingularPtrField<super::common::ImageData>,
    unit_density: ::protobuf::SingularPtrField<super::common::ImageData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeatureLayers {}

impl FeatureLayers {
    pub fn new() -> FeatureLayers {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeatureLayers {
        static mut instance: ::protobuf::lazy::Lazy<FeatureLayers> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeatureLayers,
        };
        unsafe {
            instance.get(FeatureLayers::new)
        }
    }

    // optional .SC2APIProtocol.ImageData height_map = 1;

    pub fn clear_height_map(&mut self) {
        self.height_map.clear();
    }

    pub fn has_height_map(&self) -> bool {
        self.height_map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height_map(&mut self, v: super::common::ImageData) {
        self.height_map = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_height_map(&mut self) -> &mut super::common::ImageData {
        if self.height_map.is_none() {
            self.height_map.set_default();
        }
        self.height_map.as_mut().unwrap()
    }

    // Take field
    pub fn take_height_map(&mut self) -> super::common::ImageData {
        self.height_map.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_height_map(&self) -> &super::common::ImageData {
        self.height_map.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_height_map_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.height_map
    }

    fn mut_height_map_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.height_map
    }

    // optional .SC2APIProtocol.ImageData visibility_map = 2;

    pub fn clear_visibility_map(&mut self) {
        self.visibility_map.clear();
    }

    pub fn has_visibility_map(&self) -> bool {
        self.visibility_map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visibility_map(&mut self, v: super::common::ImageData) {
        self.visibility_map = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_visibility_map(&mut self) -> &mut super::common::ImageData {
        if self.visibility_map.is_none() {
            self.visibility_map.set_default();
        }
        self.visibility_map.as_mut().unwrap()
    }

    // Take field
    pub fn take_visibility_map(&mut self) -> super::common::ImageData {
        self.visibility_map.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_visibility_map(&self) -> &super::common::ImageData {
        self.visibility_map.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_visibility_map_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.visibility_map
    }

    fn mut_visibility_map_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.visibility_map
    }

    // optional .SC2APIProtocol.ImageData creep = 3;

    pub fn clear_creep(&mut self) {
        self.creep.clear();
    }

    pub fn has_creep(&self) -> bool {
        self.creep.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creep(&mut self, v: super::common::ImageData) {
        self.creep = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creep(&mut self) -> &mut super::common::ImageData {
        if self.creep.is_none() {
            self.creep.set_default();
        }
        self.creep.as_mut().unwrap()
    }

    // Take field
    pub fn take_creep(&mut self) -> super::common::ImageData {
        self.creep.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_creep(&self) -> &super::common::ImageData {
        self.creep.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_creep_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.creep
    }

    fn mut_creep_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.creep
    }

    // optional .SC2APIProtocol.ImageData power = 4;

    pub fn clear_power(&mut self) {
        self.power.clear();
    }

    pub fn has_power(&self) -> bool {
        self.power.is_some()
    }

    // Param is passed by value, moved
    pub fn set_power(&mut self, v: super::common::ImageData) {
        self.power = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_power(&mut self) -> &mut super::common::ImageData {
        if self.power.is_none() {
            self.power.set_default();
        }
        self.power.as_mut().unwrap()
    }

    // Take field
    pub fn take_power(&mut self) -> super::common::ImageData {
        self.power.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_power(&self) -> &super::common::ImageData {
        self.power.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_power_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.power
    }

    fn mut_power_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.power
    }

    // optional .SC2APIProtocol.ImageData player_id = 5;

    pub fn clear_player_id(&mut self) {
        self.player_id.clear();
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: super::common::ImageData) {
        self.player_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_id(&mut self) -> &mut super::common::ImageData {
        if self.player_id.is_none() {
            self.player_id.set_default();
        }
        self.player_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_id(&mut self) -> super::common::ImageData {
        self.player_id.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_player_id(&self) -> &super::common::ImageData {
        self.player_id.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_player_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.player_id
    }

    // optional .SC2APIProtocol.ImageData unit_type = 6;

    pub fn clear_unit_type(&mut self) {
        self.unit_type.clear();
    }

    pub fn has_unit_type(&self) -> bool {
        self.unit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_type(&mut self, v: super::common::ImageData) {
        self.unit_type = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit_type(&mut self) -> &mut super::common::ImageData {
        if self.unit_type.is_none() {
            self.unit_type.set_default();
        }
        self.unit_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit_type(&mut self) -> super::common::ImageData {
        self.unit_type.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_unit_type(&self) -> &super::common::ImageData {
        self.unit_type.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_unit_type_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.unit_type
    }

    fn mut_unit_type_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.unit_type
    }

    // optional .SC2APIProtocol.ImageData selected = 7;

    pub fn clear_selected(&mut self) {
        self.selected.clear();
    }

    pub fn has_selected(&self) -> bool {
        self.selected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selected(&mut self, v: super::common::ImageData) {
        self.selected = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selected(&mut self) -> &mut super::common::ImageData {
        if self.selected.is_none() {
            self.selected.set_default();
        }
        self.selected.as_mut().unwrap()
    }

    // Take field
    pub fn take_selected(&mut self) -> super::common::ImageData {
        self.selected.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_selected(&self) -> &super::common::ImageData {
        self.selected.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_selected_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.selected
    }

    fn mut_selected_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.selected
    }

    // optional .SC2APIProtocol.ImageData unit_hit_points = 8;

    pub fn clear_unit_hit_points(&mut self) {
        self.unit_hit_points.clear();
    }

    pub fn has_unit_hit_points(&self) -> bool {
        self.unit_hit_points.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_hit_points(&mut self, v: super::common::ImageData) {
        self.unit_hit_points = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit_hit_points(&mut self) -> &mut super::common::ImageData {
        if self.unit_hit_points.is_none() {
            self.unit_hit_points.set_default();
        }
        self.unit_hit_points.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit_hit_points(&mut self) -> super::common::ImageData {
        self.unit_hit_points.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_unit_hit_points(&self) -> &super::common::ImageData {
        self.unit_hit_points.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_unit_hit_points_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.unit_hit_points
    }

    fn mut_unit_hit_points_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.unit_hit_points
    }

    // optional .SC2APIProtocol.ImageData unit_hit_points_ratio = 17;

    pub fn clear_unit_hit_points_ratio(&mut self) {
        self.unit_hit_points_ratio.clear();
    }

    pub fn has_unit_hit_points_ratio(&self) -> bool {
        self.unit_hit_points_ratio.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_hit_points_ratio(&mut self, v: super::common::ImageData) {
        self.unit_hit_points_ratio = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit_hit_points_ratio(&mut self) -> &mut super::common::ImageData {
        if self.unit_hit_points_ratio.is_none() {
            self.unit_hit_points_ratio.set_default();
        }
        self.unit_hit_points_ratio.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit_hit_points_ratio(&mut self) -> super::common::ImageData {
        self.unit_hit_points_ratio.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_unit_hit_points_ratio(&self) -> &super::common::ImageData {
        self.unit_hit_points_ratio.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_unit_hit_points_ratio_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.unit_hit_points_ratio
    }

    fn mut_unit_hit_points_ratio_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.unit_hit_points_ratio
    }

    // optional .SC2APIProtocol.ImageData unit_energy = 9;

    pub fn clear_unit_energy(&mut self) {
        self.unit_energy.clear();
    }

    pub fn has_unit_energy(&self) -> bool {
        self.unit_energy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_energy(&mut self, v: super::common::ImageData) {
        self.unit_energy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit_energy(&mut self) -> &mut super::common::ImageData {
        if self.unit_energy.is_none() {
            self.unit_energy.set_default();
        }
        self.unit_energy.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit_energy(&mut self) -> super::common::ImageData {
        self.unit_energy.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_unit_energy(&self) -> &super::common::ImageData {
        self.unit_energy.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_unit_energy_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.unit_energy
    }

    fn mut_unit_energy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.unit_energy
    }

    // optional .SC2APIProtocol.ImageData unit_shields = 10;

    pub fn clear_unit_shields(&mut self) {
        self.unit_shields.clear();
    }

    pub fn has_unit_shields(&self) -> bool {
        self.unit_shields.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_shields(&mut self, v: super::common::ImageData) {
        self.unit_shields = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit_shields(&mut self) -> &mut super::common::ImageData {
        if self.unit_shields.is_none() {
            self.unit_shields.set_default();
        }
        self.unit_shields.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit_shields(&mut self) -> super::common::ImageData {
        self.unit_shields.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_unit_shields(&self) -> &super::common::ImageData {
        self.unit_shields.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_unit_shields_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.unit_shields
    }

    fn mut_unit_shields_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.unit_shields
    }

    // optional .SC2APIProtocol.ImageData player_relative = 11;

    pub fn clear_player_relative(&mut self) {
        self.player_relative.clear();
    }

    pub fn has_player_relative(&self) -> bool {
        self.player_relative.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_relative(&mut self, v: super::common::ImageData) {
        self.player_relative = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_relative(&mut self) -> &mut super::common::ImageData {
        if self.player_relative.is_none() {
            self.player_relative.set_default();
        }
        self.player_relative.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_relative(&mut self) -> super::common::ImageData {
        self.player_relative.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_player_relative(&self) -> &super::common::ImageData {
        self.player_relative.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_player_relative_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.player_relative
    }

    fn mut_player_relative_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.player_relative
    }

    // optional .SC2APIProtocol.ImageData unit_density_aa = 14;

    pub fn clear_unit_density_aa(&mut self) {
        self.unit_density_aa.clear();
    }

    pub fn has_unit_density_aa(&self) -> bool {
        self.unit_density_aa.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_density_aa(&mut self, v: super::common::ImageData) {
        self.unit_density_aa = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit_density_aa(&mut self) -> &mut super::common::ImageData {
        if self.unit_density_aa.is_none() {
            self.unit_density_aa.set_default();
        }
        self.unit_density_aa.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit_density_aa(&mut self) -> super::common::ImageData {
        self.unit_density_aa.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_unit_density_aa(&self) -> &super::common::ImageData {
        self.unit_density_aa.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_unit_density_aa_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.unit_density_aa
    }

    fn mut_unit_density_aa_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.unit_density_aa
    }

    // optional .SC2APIProtocol.ImageData unit_density = 15;

    pub fn clear_unit_density(&mut self) {
        self.unit_density.clear();
    }

    pub fn has_unit_density(&self) -> bool {
        self.unit_density.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_density(&mut self, v: super::common::ImageData) {
        self.unit_density = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit_density(&mut self) -> &mut super::common::ImageData {
        if self.unit_density.is_none() {
            self.unit_density.set_default();
        }
        self.unit_density.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit_density(&mut self) -> super::common::ImageData {
        self.unit_density.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_unit_density(&self) -> &super::common::ImageData {
        self.unit_density.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_unit_density_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.unit_density
    }

    fn mut_unit_density_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.unit_density
    }
}

impl ::protobuf::Message for FeatureLayers {
    fn is_initialized(&self) -> bool {
        for v in &self.height_map {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.visibility_map {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.creep {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.power {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.player_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unit_type {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.selected {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unit_hit_points {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unit_hit_points_ratio {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unit_energy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unit_shields {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.player_relative {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unit_density_aa {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unit_density {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.height_map)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.visibility_map)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.creep)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.power)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_id)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit_type)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selected)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit_hit_points)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit_hit_points_ratio)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit_energy)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit_shields)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_relative)?;
                },
                14 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit_density_aa)?;
                },
                15 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit_density)?;
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
        if let Some(ref v) = self.height_map.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.visibility_map.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.creep.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.power.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.player_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unit_type.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.selected.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unit_hit_points.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unit_hit_points_ratio.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unit_energy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unit_shields.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.player_relative.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unit_density_aa.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unit_density.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.height_map.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.visibility_map.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.creep.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.power.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.player_id.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unit_type.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.selected.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unit_hit_points.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unit_hit_points_ratio.as_ref() {
            os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unit_energy.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unit_shields.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.player_relative.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unit_density_aa.as_ref() {
            os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unit_density.as_ref() {
            os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for FeatureLayers {
    fn new() -> FeatureLayers {
        FeatureLayers::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeatureLayers>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "height_map",
                    FeatureLayers::get_height_map_for_reflect,
                    FeatureLayers::mut_height_map_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "visibility_map",
                    FeatureLayers::get_visibility_map_for_reflect,
                    FeatureLayers::mut_visibility_map_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "creep",
                    FeatureLayers::get_creep_for_reflect,
                    FeatureLayers::mut_creep_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "power",
                    FeatureLayers::get_power_for_reflect,
                    FeatureLayers::mut_power_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "player_id",
                    FeatureLayers::get_player_id_for_reflect,
                    FeatureLayers::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "unit_type",
                    FeatureLayers::get_unit_type_for_reflect,
                    FeatureLayers::mut_unit_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "selected",
                    FeatureLayers::get_selected_for_reflect,
                    FeatureLayers::mut_selected_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "unit_hit_points",
                    FeatureLayers::get_unit_hit_points_for_reflect,
                    FeatureLayers::mut_unit_hit_points_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "unit_hit_points_ratio",
                    FeatureLayers::get_unit_hit_points_ratio_for_reflect,
                    FeatureLayers::mut_unit_hit_points_ratio_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "unit_energy",
                    FeatureLayers::get_unit_energy_for_reflect,
                    FeatureLayers::mut_unit_energy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "unit_shields",
                    FeatureLayers::get_unit_shields_for_reflect,
                    FeatureLayers::mut_unit_shields_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "player_relative",
                    FeatureLayers::get_player_relative_for_reflect,
                    FeatureLayers::mut_player_relative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "unit_density_aa",
                    FeatureLayers::get_unit_density_aa_for_reflect,
                    FeatureLayers::mut_unit_density_aa_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "unit_density",
                    FeatureLayers::get_unit_density_for_reflect,
                    FeatureLayers::mut_unit_density_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FeatureLayers>(
                    "FeatureLayers",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeatureLayers {
    fn clear(&mut self) {
        self.clear_height_map();
        self.clear_visibility_map();
        self.clear_creep();
        self.clear_power();
        self.clear_player_id();
        self.clear_unit_type();
        self.clear_selected();
        self.clear_unit_hit_points();
        self.clear_unit_hit_points_ratio();
        self.clear_unit_energy();
        self.clear_unit_shields();
        self.clear_player_relative();
        self.clear_unit_density_aa();
        self.clear_unit_density();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeatureLayers {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeatureLayers {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FeatureLayersMinimap {
    // message fields
    height_map: ::protobuf::SingularPtrField<super::common::ImageData>,
    visibility_map: ::protobuf::SingularPtrField<super::common::ImageData>,
    creep: ::protobuf::SingularPtrField<super::common::ImageData>,
    camera: ::protobuf::SingularPtrField<super::common::ImageData>,
    player_id: ::protobuf::SingularPtrField<super::common::ImageData>,
    player_relative: ::protobuf::SingularPtrField<super::common::ImageData>,
    selected: ::protobuf::SingularPtrField<super::common::ImageData>,
    unit_type: ::protobuf::SingularPtrField<super::common::ImageData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FeatureLayersMinimap {}

impl FeatureLayersMinimap {
    pub fn new() -> FeatureLayersMinimap {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FeatureLayersMinimap {
        static mut instance: ::protobuf::lazy::Lazy<FeatureLayersMinimap> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FeatureLayersMinimap,
        };
        unsafe {
            instance.get(FeatureLayersMinimap::new)
        }
    }

    // optional .SC2APIProtocol.ImageData height_map = 1;

    pub fn clear_height_map(&mut self) {
        self.height_map.clear();
    }

    pub fn has_height_map(&self) -> bool {
        self.height_map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_height_map(&mut self, v: super::common::ImageData) {
        self.height_map = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_height_map(&mut self) -> &mut super::common::ImageData {
        if self.height_map.is_none() {
            self.height_map.set_default();
        }
        self.height_map.as_mut().unwrap()
    }

    // Take field
    pub fn take_height_map(&mut self) -> super::common::ImageData {
        self.height_map.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_height_map(&self) -> &super::common::ImageData {
        self.height_map.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_height_map_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.height_map
    }

    fn mut_height_map_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.height_map
    }

    // optional .SC2APIProtocol.ImageData visibility_map = 2;

    pub fn clear_visibility_map(&mut self) {
        self.visibility_map.clear();
    }

    pub fn has_visibility_map(&self) -> bool {
        self.visibility_map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visibility_map(&mut self, v: super::common::ImageData) {
        self.visibility_map = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_visibility_map(&mut self) -> &mut super::common::ImageData {
        if self.visibility_map.is_none() {
            self.visibility_map.set_default();
        }
        self.visibility_map.as_mut().unwrap()
    }

    // Take field
    pub fn take_visibility_map(&mut self) -> super::common::ImageData {
        self.visibility_map.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_visibility_map(&self) -> &super::common::ImageData {
        self.visibility_map.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_visibility_map_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.visibility_map
    }

    fn mut_visibility_map_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.visibility_map
    }

    // optional .SC2APIProtocol.ImageData creep = 3;

    pub fn clear_creep(&mut self) {
        self.creep.clear();
    }

    pub fn has_creep(&self) -> bool {
        self.creep.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creep(&mut self, v: super::common::ImageData) {
        self.creep = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creep(&mut self) -> &mut super::common::ImageData {
        if self.creep.is_none() {
            self.creep.set_default();
        }
        self.creep.as_mut().unwrap()
    }

    // Take field
    pub fn take_creep(&mut self) -> super::common::ImageData {
        self.creep.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_creep(&self) -> &super::common::ImageData {
        self.creep.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_creep_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.creep
    }

    fn mut_creep_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.creep
    }

    // optional .SC2APIProtocol.ImageData camera = 4;

    pub fn clear_camera(&mut self) {
        self.camera.clear();
    }

    pub fn has_camera(&self) -> bool {
        self.camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera(&mut self, v: super::common::ImageData) {
        self.camera = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_camera(&mut self) -> &mut super::common::ImageData {
        if self.camera.is_none() {
            self.camera.set_default();
        }
        self.camera.as_mut().unwrap()
    }

    // Take field
    pub fn take_camera(&mut self) -> super::common::ImageData {
        self.camera.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_camera(&self) -> &super::common::ImageData {
        self.camera.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_camera_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.camera
    }

    fn mut_camera_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.camera
    }

    // optional .SC2APIProtocol.ImageData player_id = 5;

    pub fn clear_player_id(&mut self) {
        self.player_id.clear();
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: super::common::ImageData) {
        self.player_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_id(&mut self) -> &mut super::common::ImageData {
        if self.player_id.is_none() {
            self.player_id.set_default();
        }
        self.player_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_id(&mut self) -> super::common::ImageData {
        self.player_id.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_player_id(&self) -> &super::common::ImageData {
        self.player_id.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_player_id_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.player_id
    }

    // optional .SC2APIProtocol.ImageData player_relative = 6;

    pub fn clear_player_relative(&mut self) {
        self.player_relative.clear();
    }

    pub fn has_player_relative(&self) -> bool {
        self.player_relative.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_relative(&mut self, v: super::common::ImageData) {
        self.player_relative = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_relative(&mut self) -> &mut super::common::ImageData {
        if self.player_relative.is_none() {
            self.player_relative.set_default();
        }
        self.player_relative.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_relative(&mut self) -> super::common::ImageData {
        self.player_relative.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_player_relative(&self) -> &super::common::ImageData {
        self.player_relative.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_player_relative_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.player_relative
    }

    fn mut_player_relative_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.player_relative
    }

    // optional .SC2APIProtocol.ImageData selected = 7;

    pub fn clear_selected(&mut self) {
        self.selected.clear();
    }

    pub fn has_selected(&self) -> bool {
        self.selected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selected(&mut self, v: super::common::ImageData) {
        self.selected = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selected(&mut self) -> &mut super::common::ImageData {
        if self.selected.is_none() {
            self.selected.set_default();
        }
        self.selected.as_mut().unwrap()
    }

    // Take field
    pub fn take_selected(&mut self) -> super::common::ImageData {
        self.selected.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_selected(&self) -> &super::common::ImageData {
        self.selected.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_selected_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.selected
    }

    fn mut_selected_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.selected
    }

    // optional .SC2APIProtocol.ImageData unit_type = 8;

    pub fn clear_unit_type(&mut self) {
        self.unit_type.clear();
    }

    pub fn has_unit_type(&self) -> bool {
        self.unit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_type(&mut self, v: super::common::ImageData) {
        self.unit_type = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit_type(&mut self) -> &mut super::common::ImageData {
        if self.unit_type.is_none() {
            self.unit_type.set_default();
        }
        self.unit_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit_type(&mut self) -> super::common::ImageData {
        self.unit_type.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_unit_type(&self) -> &super::common::ImageData {
        self.unit_type.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_unit_type_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.unit_type
    }

    fn mut_unit_type_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.unit_type
    }
}

impl ::protobuf::Message for FeatureLayersMinimap {
    fn is_initialized(&self) -> bool {
        for v in &self.height_map {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.visibility_map {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.creep {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.camera {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.player_id {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.player_relative {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.selected {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.unit_type {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.height_map)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.visibility_map)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.creep)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.camera)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_id)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_relative)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selected)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit_type)?;
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
        if let Some(ref v) = self.height_map.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.visibility_map.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.creep.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.camera.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.player_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.player_relative.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.selected.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.unit_type.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.height_map.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.visibility_map.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.creep.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.camera.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.player_id.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.player_relative.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.selected.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.unit_type.as_ref() {
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

impl ::protobuf::MessageStatic for FeatureLayersMinimap {
    fn new() -> FeatureLayersMinimap {
        FeatureLayersMinimap::new()
    }

    fn descriptor_static(_: ::std::option::Option<FeatureLayersMinimap>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "height_map",
                    FeatureLayersMinimap::get_height_map_for_reflect,
                    FeatureLayersMinimap::mut_height_map_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "visibility_map",
                    FeatureLayersMinimap::get_visibility_map_for_reflect,
                    FeatureLayersMinimap::mut_visibility_map_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "creep",
                    FeatureLayersMinimap::get_creep_for_reflect,
                    FeatureLayersMinimap::mut_creep_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "camera",
                    FeatureLayersMinimap::get_camera_for_reflect,
                    FeatureLayersMinimap::mut_camera_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "player_id",
                    FeatureLayersMinimap::get_player_id_for_reflect,
                    FeatureLayersMinimap::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "player_relative",
                    FeatureLayersMinimap::get_player_relative_for_reflect,
                    FeatureLayersMinimap::mut_player_relative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "selected",
                    FeatureLayersMinimap::get_selected_for_reflect,
                    FeatureLayersMinimap::mut_selected_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "unit_type",
                    FeatureLayersMinimap::get_unit_type_for_reflect,
                    FeatureLayersMinimap::mut_unit_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FeatureLayersMinimap>(
                    "FeatureLayersMinimap",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FeatureLayersMinimap {
    fn clear(&mut self) {
        self.clear_height_map();
        self.clear_visibility_map();
        self.clear_creep();
        self.clear_camera();
        self.clear_player_id();
        self.clear_player_relative();
        self.clear_selected();
        self.clear_unit_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FeatureLayersMinimap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FeatureLayersMinimap {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ObservationRender {
    // message fields
    map: ::protobuf::SingularPtrField<super::common::ImageData>,
    minimap: ::protobuf::SingularPtrField<super::common::ImageData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObservationRender {}

impl ObservationRender {
    pub fn new() -> ObservationRender {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObservationRender {
        static mut instance: ::protobuf::lazy::Lazy<ObservationRender> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObservationRender,
        };
        unsafe {
            instance.get(ObservationRender::new)
        }
    }

    // optional .SC2APIProtocol.ImageData map = 1;

    pub fn clear_map(&mut self) {
        self.map.clear();
    }

    pub fn has_map(&self) -> bool {
        self.map.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map(&mut self, v: super::common::ImageData) {
        self.map = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map(&mut self) -> &mut super::common::ImageData {
        if self.map.is_none() {
            self.map.set_default();
        }
        self.map.as_mut().unwrap()
    }

    // Take field
    pub fn take_map(&mut self) -> super::common::ImageData {
        self.map.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_map(&self) -> &super::common::ImageData {
        self.map.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_map_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.map
    }

    fn mut_map_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.map
    }

    // optional .SC2APIProtocol.ImageData minimap = 2;

    pub fn clear_minimap(&mut self) {
        self.minimap.clear();
    }

    pub fn has_minimap(&self) -> bool {
        self.minimap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimap(&mut self, v: super::common::ImageData) {
        self.minimap = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_minimap(&mut self) -> &mut super::common::ImageData {
        if self.minimap.is_none() {
            self.minimap.set_default();
        }
        self.minimap.as_mut().unwrap()
    }

    // Take field
    pub fn take_minimap(&mut self) -> super::common::ImageData {
        self.minimap.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_minimap(&self) -> &super::common::ImageData {
        self.minimap.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_minimap_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.minimap
    }

    fn mut_minimap_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.minimap
    }
}

impl ::protobuf::Message for ObservationRender {
    fn is_initialized(&self) -> bool {
        for v in &self.map {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.minimap {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.map)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.minimap)?;
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
        if let Some(ref v) = self.map.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.minimap.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.map.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.minimap.as_ref() {
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

impl ::protobuf::MessageStatic for ObservationRender {
    fn new() -> ObservationRender {
        ObservationRender::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObservationRender>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "map",
                    ObservationRender::get_map_for_reflect,
                    ObservationRender::mut_map_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "minimap",
                    ObservationRender::get_minimap_for_reflect,
                    ObservationRender::mut_minimap_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObservationRender>(
                    "ObservationRender",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObservationRender {
    fn clear(&mut self) {
        self.clear_map();
        self.clear_minimap();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ObservationRender {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ObservationRender {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionSpatial {
    // message oneof groups
    action: ::std::option::Option<ActionSpatial_oneof_action>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionSpatial {}

#[derive(Clone,PartialEq)]
pub enum ActionSpatial_oneof_action {
    unit_command(ActionSpatialUnitCommand),
    camera_move(ActionSpatialCameraMove),
    unit_selection_point(ActionSpatialUnitSelectionPoint),
    unit_selection_rect(ActionSpatialUnitSelectionRect),
}

impl ActionSpatial {
    pub fn new() -> ActionSpatial {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionSpatial {
        static mut instance: ::protobuf::lazy::Lazy<ActionSpatial> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionSpatial,
        };
        unsafe {
            instance.get(ActionSpatial::new)
        }
    }

    // optional .SC2APIProtocol.ActionSpatialUnitCommand unit_command = 1;

    pub fn clear_unit_command(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_unit_command(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::unit_command(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unit_command(&mut self, v: ActionSpatialUnitCommand) {
        self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::unit_command(v))
    }

    // Mutable pointer to the field.
    pub fn mut_unit_command(&mut self) -> &mut ActionSpatialUnitCommand {
        if let ::std::option::Option::Some(ActionSpatial_oneof_action::unit_command(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::unit_command(ActionSpatialUnitCommand::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::unit_command(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_unit_command(&mut self) -> ActionSpatialUnitCommand {
        if self.has_unit_command() {
            match self.action.take() {
                ::std::option::Option::Some(ActionSpatial_oneof_action::unit_command(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionSpatialUnitCommand::new()
        }
    }

    pub fn get_unit_command(&self) -> &ActionSpatialUnitCommand {
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::unit_command(ref v)) => v,
            _ => ActionSpatialUnitCommand::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionSpatialCameraMove camera_move = 2;

    pub fn clear_camera_move(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_camera_move(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::camera_move(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_camera_move(&mut self, v: ActionSpatialCameraMove) {
        self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::camera_move(v))
    }

    // Mutable pointer to the field.
    pub fn mut_camera_move(&mut self) -> &mut ActionSpatialCameraMove {
        if let ::std::option::Option::Some(ActionSpatial_oneof_action::camera_move(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::camera_move(ActionSpatialCameraMove::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::camera_move(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_camera_move(&mut self) -> ActionSpatialCameraMove {
        if self.has_camera_move() {
            match self.action.take() {
                ::std::option::Option::Some(ActionSpatial_oneof_action::camera_move(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionSpatialCameraMove::new()
        }
    }

    pub fn get_camera_move(&self) -> &ActionSpatialCameraMove {
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::camera_move(ref v)) => v,
            _ => ActionSpatialCameraMove::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionSpatialUnitSelectionPoint unit_selection_point = 3;

    pub fn clear_unit_selection_point(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_unit_selection_point(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_point(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unit_selection_point(&mut self, v: ActionSpatialUnitSelectionPoint) {
        self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_point(v))
    }

    // Mutable pointer to the field.
    pub fn mut_unit_selection_point(&mut self) -> &mut ActionSpatialUnitSelectionPoint {
        if let ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_point(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_point(ActionSpatialUnitSelectionPoint::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_point(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_unit_selection_point(&mut self) -> ActionSpatialUnitSelectionPoint {
        if self.has_unit_selection_point() {
            match self.action.take() {
                ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_point(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionSpatialUnitSelectionPoint::new()
        }
    }

    pub fn get_unit_selection_point(&self) -> &ActionSpatialUnitSelectionPoint {
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_point(ref v)) => v,
            _ => ActionSpatialUnitSelectionPoint::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionSpatialUnitSelectionRect unit_selection_rect = 4;

    pub fn clear_unit_selection_rect(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_unit_selection_rect(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_rect(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unit_selection_rect(&mut self, v: ActionSpatialUnitSelectionRect) {
        self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_rect(v))
    }

    // Mutable pointer to the field.
    pub fn mut_unit_selection_rect(&mut self) -> &mut ActionSpatialUnitSelectionRect {
        if let ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_rect(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_rect(ActionSpatialUnitSelectionRect::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_rect(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_unit_selection_rect(&mut self) -> ActionSpatialUnitSelectionRect {
        if self.has_unit_selection_rect() {
            match self.action.take() {
                ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_rect(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionSpatialUnitSelectionRect::new()
        }
    }

    pub fn get_unit_selection_rect(&self) -> &ActionSpatialUnitSelectionRect {
        match self.action {
            ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_rect(ref v)) => v,
            _ => ActionSpatialUnitSelectionRect::default_instance(),
        }
    }
}

impl ::protobuf::Message for ActionSpatial {
    fn is_initialized(&self) -> bool {
        if let Some(ActionSpatial_oneof_action::unit_command(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionSpatial_oneof_action::camera_move(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionSpatial_oneof_action::unit_selection_point(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionSpatial_oneof_action::unit_selection_rect(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
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
                    self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::unit_command(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::camera_move(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_point(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionSpatial_oneof_action::unit_selection_rect(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.action {
            match v {
                &ActionSpatial_oneof_action::unit_command(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionSpatial_oneof_action::camera_move(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionSpatial_oneof_action::unit_selection_point(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionSpatial_oneof_action::unit_selection_rect(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.action {
            match v {
                &ActionSpatial_oneof_action::unit_command(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionSpatial_oneof_action::camera_move(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionSpatial_oneof_action::unit_selection_point(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionSpatial_oneof_action::unit_selection_rect(ref v) => {
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

impl ::protobuf::MessageStatic for ActionSpatial {
    fn new() -> ActionSpatial {
        ActionSpatial::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionSpatial>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionSpatialUnitCommand>(
                    "unit_command",
                    ActionSpatial::has_unit_command,
                    ActionSpatial::get_unit_command,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionSpatialCameraMove>(
                    "camera_move",
                    ActionSpatial::has_camera_move,
                    ActionSpatial::get_camera_move,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionSpatialUnitSelectionPoint>(
                    "unit_selection_point",
                    ActionSpatial::has_unit_selection_point,
                    ActionSpatial::get_unit_selection_point,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionSpatialUnitSelectionRect>(
                    "unit_selection_rect",
                    ActionSpatial::has_unit_selection_rect,
                    ActionSpatial::get_unit_selection_rect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionSpatial>(
                    "ActionSpatial",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionSpatial {
    fn clear(&mut self) {
        self.clear_unit_command();
        self.clear_camera_move();
        self.clear_unit_selection_point();
        self.clear_unit_selection_rect();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionSpatial {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionSpatial {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionSpatialUnitCommand {
    // message fields
    ability_id: ::std::option::Option<i32>,
    queue_command: ::std::option::Option<bool>,
    // message oneof groups
    target: ::std::option::Option<ActionSpatialUnitCommand_oneof_target>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionSpatialUnitCommand {}

#[derive(Clone,PartialEq)]
pub enum ActionSpatialUnitCommand_oneof_target {
    target_screen_coord(super::common::PointI),
    target_minimap_coord(super::common::PointI),
}

impl ActionSpatialUnitCommand {
    pub fn new() -> ActionSpatialUnitCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionSpatialUnitCommand {
        static mut instance: ::protobuf::lazy::Lazy<ActionSpatialUnitCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionSpatialUnitCommand,
        };
        unsafe {
            instance.get(ActionSpatialUnitCommand::new)
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

    // optional .SC2APIProtocol.PointI target_screen_coord = 2;

    pub fn clear_target_screen_coord(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target_screen_coord(&self) -> bool {
        match self.target {
            ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_screen_coord(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_target_screen_coord(&mut self, v: super::common::PointI) {
        self.target = ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_screen_coord(v))
    }

    // Mutable pointer to the field.
    pub fn mut_target_screen_coord(&mut self) -> &mut super::common::PointI {
        if let ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_screen_coord(_)) = self.target {
        } else {
            self.target = ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_screen_coord(super::common::PointI::new()));
        }
        match self.target {
            ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_screen_coord(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_target_screen_coord(&mut self) -> super::common::PointI {
        if self.has_target_screen_coord() {
            match self.target.take() {
                ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_screen_coord(v)) => v,
                _ => panic!(),
            }
        } else {
            super::common::PointI::new()
        }
    }

    pub fn get_target_screen_coord(&self) -> &super::common::PointI {
        match self.target {
            ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_screen_coord(ref v)) => v,
            _ => super::common::PointI::default_instance(),
        }
    }

    // optional .SC2APIProtocol.PointI target_minimap_coord = 3;

    pub fn clear_target_minimap_coord(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target_minimap_coord(&self) -> bool {
        match self.target {
            ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_minimap_coord(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_target_minimap_coord(&mut self, v: super::common::PointI) {
        self.target = ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_minimap_coord(v))
    }

    // Mutable pointer to the field.
    pub fn mut_target_minimap_coord(&mut self) -> &mut super::common::PointI {
        if let ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_minimap_coord(_)) = self.target {
        } else {
            self.target = ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_minimap_coord(super::common::PointI::new()));
        }
        match self.target {
            ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_minimap_coord(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_target_minimap_coord(&mut self) -> super::common::PointI {
        if self.has_target_minimap_coord() {
            match self.target.take() {
                ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_minimap_coord(v)) => v,
                _ => panic!(),
            }
        } else {
            super::common::PointI::new()
        }
    }

    pub fn get_target_minimap_coord(&self) -> &super::common::PointI {
        match self.target {
            ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_minimap_coord(ref v)) => v,
            _ => super::common::PointI::default_instance(),
        }
    }

    // optional bool queue_command = 4;

    pub fn clear_queue_command(&mut self) {
        self.queue_command = ::std::option::Option::None;
    }

    pub fn has_queue_command(&self) -> bool {
        self.queue_command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_queue_command(&mut self, v: bool) {
        self.queue_command = ::std::option::Option::Some(v);
    }

    pub fn get_queue_command(&self) -> bool {
        self.queue_command.unwrap_or(false)
    }

    fn get_queue_command_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.queue_command
    }

    fn mut_queue_command_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.queue_command
    }
}

impl ::protobuf::Message for ActionSpatialUnitCommand {
    fn is_initialized(&self) -> bool {
        if let Some(ActionSpatialUnitCommand_oneof_target::target_screen_coord(ref v)) = self.target {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionSpatialUnitCommand_oneof_target::target_minimap_coord(ref v)) = self.target {
            if !v.is_initialized() {
                return false;
            }
        }
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target = ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_screen_coord(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target = ::std::option::Option::Some(ActionSpatialUnitCommand_oneof_target::target_minimap_coord(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.queue_command = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.queue_command {
            my_size += 2;
        }
        if let ::std::option::Option::Some(ref v) = self.target {
            match v {
                &ActionSpatialUnitCommand_oneof_target::target_screen_coord(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionSpatialUnitCommand_oneof_target::target_minimap_coord(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_id {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.queue_command {
            os.write_bool(4, v)?;
        }
        if let ::std::option::Option::Some(ref v) = self.target {
            match v {
                &ActionSpatialUnitCommand_oneof_target::target_screen_coord(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionSpatialUnitCommand_oneof_target::target_minimap_coord(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ActionSpatialUnitCommand {
    fn new() -> ActionSpatialUnitCommand {
        ActionSpatialUnitCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionSpatialUnitCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ability_id",
                    ActionSpatialUnitCommand::get_ability_id_for_reflect,
                    ActionSpatialUnitCommand::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::common::PointI>(
                    "target_screen_coord",
                    ActionSpatialUnitCommand::has_target_screen_coord,
                    ActionSpatialUnitCommand::get_target_screen_coord,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::common::PointI>(
                    "target_minimap_coord",
                    ActionSpatialUnitCommand::has_target_minimap_coord,
                    ActionSpatialUnitCommand::get_target_minimap_coord,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "queue_command",
                    ActionSpatialUnitCommand::get_queue_command_for_reflect,
                    ActionSpatialUnitCommand::mut_queue_command_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionSpatialUnitCommand>(
                    "ActionSpatialUnitCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionSpatialUnitCommand {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.clear_target_screen_coord();
        self.clear_target_minimap_coord();
        self.clear_queue_command();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionSpatialUnitCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionSpatialUnitCommand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionSpatialCameraMove {
    // message fields
    center_minimap: ::protobuf::SingularPtrField<super::common::PointI>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionSpatialCameraMove {}

impl ActionSpatialCameraMove {
    pub fn new() -> ActionSpatialCameraMove {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionSpatialCameraMove {
        static mut instance: ::protobuf::lazy::Lazy<ActionSpatialCameraMove> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionSpatialCameraMove,
        };
        unsafe {
            instance.get(ActionSpatialCameraMove::new)
        }
    }

    // optional .SC2APIProtocol.PointI center_minimap = 1;

    pub fn clear_center_minimap(&mut self) {
        self.center_minimap.clear();
    }

    pub fn has_center_minimap(&self) -> bool {
        self.center_minimap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_center_minimap(&mut self, v: super::common::PointI) {
        self.center_minimap = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_center_minimap(&mut self) -> &mut super::common::PointI {
        if self.center_minimap.is_none() {
            self.center_minimap.set_default();
        }
        self.center_minimap.as_mut().unwrap()
    }

    // Take field
    pub fn take_center_minimap(&mut self) -> super::common::PointI {
        self.center_minimap.take().unwrap_or_else(|| super::common::PointI::new())
    }

    pub fn get_center_minimap(&self) -> &super::common::PointI {
        self.center_minimap.as_ref().unwrap_or_else(|| super::common::PointI::default_instance())
    }

    fn get_center_minimap_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::PointI> {
        &self.center_minimap
    }

    fn mut_center_minimap_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::PointI> {
        &mut self.center_minimap
    }
}

impl ::protobuf::Message for ActionSpatialCameraMove {
    fn is_initialized(&self) -> bool {
        for v in &self.center_minimap {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.center_minimap)?;
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
        if let Some(ref v) = self.center_minimap.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.center_minimap.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ActionSpatialCameraMove {
    fn new() -> ActionSpatialCameraMove {
        ActionSpatialCameraMove::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionSpatialCameraMove>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::PointI>>(
                    "center_minimap",
                    ActionSpatialCameraMove::get_center_minimap_for_reflect,
                    ActionSpatialCameraMove::mut_center_minimap_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionSpatialCameraMove>(
                    "ActionSpatialCameraMove",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionSpatialCameraMove {
    fn clear(&mut self) {
        self.clear_center_minimap();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionSpatialCameraMove {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionSpatialCameraMove {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionSpatialUnitSelectionPoint {
    // message fields
    selection_screen_coord: ::protobuf::SingularPtrField<super::common::PointI>,
    field_type: ::std::option::Option<ActionSpatialUnitSelectionPoint_Type>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionSpatialUnitSelectionPoint {}

impl ActionSpatialUnitSelectionPoint {
    pub fn new() -> ActionSpatialUnitSelectionPoint {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionSpatialUnitSelectionPoint {
        static mut instance: ::protobuf::lazy::Lazy<ActionSpatialUnitSelectionPoint> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionSpatialUnitSelectionPoint,
        };
        unsafe {
            instance.get(ActionSpatialUnitSelectionPoint::new)
        }
    }

    // optional .SC2APIProtocol.PointI selection_screen_coord = 1;

    pub fn clear_selection_screen_coord(&mut self) {
        self.selection_screen_coord.clear();
    }

    pub fn has_selection_screen_coord(&self) -> bool {
        self.selection_screen_coord.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selection_screen_coord(&mut self, v: super::common::PointI) {
        self.selection_screen_coord = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selection_screen_coord(&mut self) -> &mut super::common::PointI {
        if self.selection_screen_coord.is_none() {
            self.selection_screen_coord.set_default();
        }
        self.selection_screen_coord.as_mut().unwrap()
    }

    // Take field
    pub fn take_selection_screen_coord(&mut self) -> super::common::PointI {
        self.selection_screen_coord.take().unwrap_or_else(|| super::common::PointI::new())
    }

    pub fn get_selection_screen_coord(&self) -> &super::common::PointI {
        self.selection_screen_coord.as_ref().unwrap_or_else(|| super::common::PointI::default_instance())
    }

    fn get_selection_screen_coord_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::PointI> {
        &self.selection_screen_coord
    }

    fn mut_selection_screen_coord_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::PointI> {
        &mut self.selection_screen_coord
    }

    // optional .SC2APIProtocol.ActionSpatialUnitSelectionPoint.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ActionSpatialUnitSelectionPoint_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ActionSpatialUnitSelectionPoint_Type {
        self.field_type.unwrap_or(ActionSpatialUnitSelectionPoint_Type::Select)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<ActionSpatialUnitSelectionPoint_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<ActionSpatialUnitSelectionPoint_Type> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for ActionSpatialUnitSelectionPoint {
    fn is_initialized(&self) -> bool {
        for v in &self.selection_screen_coord {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selection_screen_coord)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.selection_screen_coord.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.selection_screen_coord.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.field_type {
            os.write_enum(2, v.value())?;
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

impl ::protobuf::MessageStatic for ActionSpatialUnitSelectionPoint {
    fn new() -> ActionSpatialUnitSelectionPoint {
        ActionSpatialUnitSelectionPoint::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionSpatialUnitSelectionPoint>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::PointI>>(
                    "selection_screen_coord",
                    ActionSpatialUnitSelectionPoint::get_selection_screen_coord_for_reflect,
                    ActionSpatialUnitSelectionPoint::mut_selection_screen_coord_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ActionSpatialUnitSelectionPoint_Type>>(
                    "type",
                    ActionSpatialUnitSelectionPoint::get_field_type_for_reflect,
                    ActionSpatialUnitSelectionPoint::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionSpatialUnitSelectionPoint>(
                    "ActionSpatialUnitSelectionPoint",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionSpatialUnitSelectionPoint {
    fn clear(&mut self) {
        self.clear_selection_screen_coord();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionSpatialUnitSelectionPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionSpatialUnitSelectionPoint {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ActionSpatialUnitSelectionPoint_Type {
    Select = 1,
    Toggle = 2,
    AllType = 3,
    AddAllType = 4,
}

impl ::protobuf::ProtobufEnum for ActionSpatialUnitSelectionPoint_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ActionSpatialUnitSelectionPoint_Type> {
        match value {
            1 => ::std::option::Option::Some(ActionSpatialUnitSelectionPoint_Type::Select),
            2 => ::std::option::Option::Some(ActionSpatialUnitSelectionPoint_Type::Toggle),
            3 => ::std::option::Option::Some(ActionSpatialUnitSelectionPoint_Type::AllType),
            4 => ::std::option::Option::Some(ActionSpatialUnitSelectionPoint_Type::AddAllType),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ActionSpatialUnitSelectionPoint_Type] = &[
            ActionSpatialUnitSelectionPoint_Type::Select,
            ActionSpatialUnitSelectionPoint_Type::Toggle,
            ActionSpatialUnitSelectionPoint_Type::AllType,
            ActionSpatialUnitSelectionPoint_Type::AddAllType,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ActionSpatialUnitSelectionPoint_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ActionSpatialUnitSelectionPoint_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ActionSpatialUnitSelectionPoint_Type {
}

impl ::protobuf::reflect::ProtobufValue for ActionSpatialUnitSelectionPoint_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionSpatialUnitSelectionRect {
    // message fields
    selection_screen_coord: ::protobuf::RepeatedField<super::common::RectangleI>,
    selection_add: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionSpatialUnitSelectionRect {}

impl ActionSpatialUnitSelectionRect {
    pub fn new() -> ActionSpatialUnitSelectionRect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionSpatialUnitSelectionRect {
        static mut instance: ::protobuf::lazy::Lazy<ActionSpatialUnitSelectionRect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionSpatialUnitSelectionRect,
        };
        unsafe {
            instance.get(ActionSpatialUnitSelectionRect::new)
        }
    }

    // repeated .SC2APIProtocol.RectangleI selection_screen_coord = 1;

    pub fn clear_selection_screen_coord(&mut self) {
        self.selection_screen_coord.clear();
    }

    // Param is passed by value, moved
    pub fn set_selection_screen_coord(&mut self, v: ::protobuf::RepeatedField<super::common::RectangleI>) {
        self.selection_screen_coord = v;
    }

    // Mutable pointer to the field.
    pub fn mut_selection_screen_coord(&mut self) -> &mut ::protobuf::RepeatedField<super::common::RectangleI> {
        &mut self.selection_screen_coord
    }

    // Take field
    pub fn take_selection_screen_coord(&mut self) -> ::protobuf::RepeatedField<super::common::RectangleI> {
        ::std::mem::replace(&mut self.selection_screen_coord, ::protobuf::RepeatedField::new())
    }

    pub fn get_selection_screen_coord(&self) -> &[super::common::RectangleI] {
        &self.selection_screen_coord
    }

    fn get_selection_screen_coord_for_reflect(&self) -> &::protobuf::RepeatedField<super::common::RectangleI> {
        &self.selection_screen_coord
    }

    fn mut_selection_screen_coord_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::common::RectangleI> {
        &mut self.selection_screen_coord
    }

    // optional bool selection_add = 2;

    pub fn clear_selection_add(&mut self) {
        self.selection_add = ::std::option::Option::None;
    }

    pub fn has_selection_add(&self) -> bool {
        self.selection_add.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selection_add(&mut self, v: bool) {
        self.selection_add = ::std::option::Option::Some(v);
    }

    pub fn get_selection_add(&self) -> bool {
        self.selection_add.unwrap_or(false)
    }

    fn get_selection_add_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.selection_add
    }

    fn mut_selection_add_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.selection_add
    }
}

impl ::protobuf::Message for ActionSpatialUnitSelectionRect {
    fn is_initialized(&self) -> bool {
        for v in &self.selection_screen_coord {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.selection_screen_coord)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.selection_add = ::std::option::Option::Some(tmp);
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
        for value in &self.selection_screen_coord {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.selection_add {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.selection_screen_coord {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.selection_add {
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

impl ::protobuf::MessageStatic for ActionSpatialUnitSelectionRect {
    fn new() -> ActionSpatialUnitSelectionRect {
        ActionSpatialUnitSelectionRect::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionSpatialUnitSelectionRect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::RectangleI>>(
                    "selection_screen_coord",
                    ActionSpatialUnitSelectionRect::get_selection_screen_coord_for_reflect,
                    ActionSpatialUnitSelectionRect::mut_selection_screen_coord_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "selection_add",
                    ActionSpatialUnitSelectionRect::get_selection_add_for_reflect,
                    ActionSpatialUnitSelectionRect::mut_selection_add_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionSpatialUnitSelectionRect>(
                    "ActionSpatialUnitSelectionRect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionSpatialUnitSelectionRect {
    fn clear(&mut self) {
        self.clear_selection_screen_coord();
        self.clear_selection_add();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionSpatialUnitSelectionRect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionSpatialUnitSelectionRect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1es2clientprotocol/spatial.proto\x12\x0eSC2APIProtocol\x1a\x1ds2clie\
    ntprotocol/common.proto\"\xa1\x01\n\x17ObservationFeatureLayer\x127\n\
    \x07renders\x18\x01\x20\x01(\x0b2\x1d.SC2APIProtocol.FeatureLayersR\x07r\
    enders\x12M\n\x0fminimap_renders\x18\x02\x20\x01(\x0b2$.SC2APIProtocol.F\
    eatureLayersMinimapR\x0eminimapRenders\"\xe4\x06\n\rFeatureLayers\x128\n\
    \nheight_map\x18\x01\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\theight\
    Map\x12@\n\x0evisibility_map\x18\x02\x20\x01(\x0b2\x19.SC2APIProtocol.Im\
    ageDataR\rvisibilityMap\x12/\n\x05creep\x18\x03\x20\x01(\x0b2\x19.SC2API\
    Protocol.ImageDataR\x05creep\x12/\n\x05power\x18\x04\x20\x01(\x0b2\x19.S\
    C2APIProtocol.ImageDataR\x05power\x126\n\tplayer_id\x18\x05\x20\x01(\x0b\
    2\x19.SC2APIProtocol.ImageDataR\x08playerId\x126\n\tunit_type\x18\x06\
    \x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\x08unitType\x125\n\x08selec\
    ted\x18\x07\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\x08selected\x12A\
    \n\x0funit_hit_points\x18\x08\x20\x01(\x0b2\x19.SC2APIProtocol.ImageData\
    R\runitHitPoints\x12L\n\x15unit_hit_points_ratio\x18\x11\x20\x01(\x0b2\
    \x19.SC2APIProtocol.ImageDataR\x12unitHitPointsRatio\x12:\n\x0bunit_ener\
    gy\x18\t\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\nunitEnergy\x12<\n\
    \x0cunit_shields\x18\n\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\x0bun\
    itShields\x12B\n\x0fplayer_relative\x18\x0b\x20\x01(\x0b2\x19.SC2APIProt\
    ocol.ImageDataR\x0eplayerRelative\x12A\n\x0funit_density_aa\x18\x0e\x20\
    \x01(\x0b2\x19.SC2APIProtocol.ImageDataR\runitDensityAa\x12<\n\x0cunit_d\
    ensity\x18\x0f\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\x0bunitDensit\
    y\"\xe1\x03\n\x14FeatureLayersMinimap\x128\n\nheight_map\x18\x01\x20\x01\
    (\x0b2\x19.SC2APIProtocol.ImageDataR\theightMap\x12@\n\x0evisibility_map\
    \x18\x02\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\rvisibilityMap\x12/\
    \n\x05creep\x18\x03\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\x05creep\
    \x121\n\x06camera\x18\x04\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\
    \x06camera\x126\n\tplayer_id\x18\x05\x20\x01(\x0b2\x19.SC2APIProtocol.Im\
    ageDataR\x08playerId\x12B\n\x0fplayer_relative\x18\x06\x20\x01(\x0b2\x19\
    .SC2APIProtocol.ImageDataR\x0eplayerRelative\x125\n\x08selected\x18\x07\
    \x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\x08selected\x126\n\tunit_ty\
    pe\x18\x08\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\x08unitType\"u\n\
    \x11ObservationRender\x12+\n\x03map\x18\x01\x20\x01(\x0b2\x19.SC2APIProt\
    ocol.ImageDataR\x03map\x123\n\x07minimap\x18\x02\x20\x01(\x0b2\x19.SC2AP\
    IProtocol.ImageDataR\x07minimap\"\xfb\x02\n\rActionSpatial\x12M\n\x0cuni\
    t_command\x18\x01\x20\x01(\x0b2(.SC2APIProtocol.ActionSpatialUnitCommand\
    H\0R\x0bunitCommand\x12J\n\x0bcamera_move\x18\x02\x20\x01(\x0b2'.SC2APIP\
    rotocol.ActionSpatialCameraMoveH\0R\ncameraMove\x12c\n\x14unit_selection\
    _point\x18\x03\x20\x01(\x0b2/.SC2APIProtocol.ActionSpatialUnitSelectionP\
    ointH\0R\x12unitSelectionPoint\x12`\n\x13unit_selection_rect\x18\x04\x20\
    \x01(\x0b2..SC2APIProtocol.ActionSpatialUnitSelectionRectH\0R\x11unitSel\
    ectionRectB\x08\n\x06action\"\xfe\x01\n\x18ActionSpatialUnitCommand\x12\
    \x1d\n\nability_id\x18\x01\x20\x01(\x05R\tabilityId\x12H\n\x13target_scr\
    een_coord\x18\x02\x20\x01(\x0b2\x16.SC2APIProtocol.PointIH\0R\x11targetS\
    creenCoord\x12J\n\x14target_minimap_coord\x18\x03\x20\x01(\x0b2\x16.SC2A\
    PIProtocol.PointIH\0R\x12targetMinimapCoord\x12#\n\rqueue_command\x18\
    \x04\x20\x01(\x08R\x0cqueueCommandB\x08\n\x06target\"X\n\x17ActionSpatia\
    lCameraMove\x12=\n\x0ecenter_minimap\x18\x01\x20\x01(\x0b2\x16.SC2APIPro\
    tocol.PointIR\rcenterMinimap\"\xf6\x01\n\x1fActionSpatialUnitSelectionPo\
    int\x12L\n\x16selection_screen_coord\x18\x01\x20\x01(\x0b2\x16.SC2APIPro\
    tocol.PointIR\x14selectionScreenCoord\x12H\n\x04type\x18\x02\x20\x01(\
    \x0e24.SC2APIProtocol.ActionSpatialUnitSelectionPoint.TypeR\x04type\";\n\
    \x04Type\x12\n\n\x06Select\x10\x01\x12\n\n\x06Toggle\x10\x02\x12\x0b\n\
    \x07AllType\x10\x03\x12\x0e\n\nAddAllType\x10\x04\"\x97\x01\n\x1eActionS\
    patialUnitSelectionRect\x12P\n\x16selection_screen_coord\x18\x01\x20\x03\
    (\x0b2\x1a.SC2APIProtocol.RectangleIR\x14selectionScreenCoord\x12#\n\rse\
    lection_add\x18\x02\x20\x01(\x08R\x0cselectionAddJ\xf5$\n\x06\x12\x04\
    \x01\0b\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\x08\n\x01\x02\x12\x03\
    \x03\x08\x16\n\t\n\x02\x03\0\x12\x03\x05\x07&\n+\n\x02\x04\0\x12\x04\x0b\
    \0\x0e\x012\x1f\n\x20Observation\x20-\x20Feature\x20Layer\n\n\n\n\n\x03\
    \x04\0\x01\x12\x03\x0b\x08\x1f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x0c\x02%\
    \n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x0c\x02\n\n\x0c\n\x05\x04\0\x02\0\
    \x06\x12\x03\x0c\x0b\x18\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x0c\x19\x20\
    \n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c#$\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\r\x024\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\r\x02\n\n\x0c\n\x05\
    \x04\0\x02\x01\x06\x12\x03\r\x0b\x1f\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\r\x20/\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\r23\n\n\n\x02\x04\x01\
    \x12\x04\x10\0\x1f\x01\n\n\n\x03\x04\x01\x01\x12\x03\x10\x08\x15\n]\n\
    \x04\x04\x01\x02\0\x12\x03\x11\x02$\"P\x20uint8.\x20Terrain\x20height.\
    \x20World\x20space\x20units\x20of\x20[-200,\x20200]\x20encoded\x20into\
    \x20[0,\x20255].\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x11\x02\n\n\x0c\
    \n\x05\x04\x01\x02\0\x06\x12\x03\x11\x0b\x14\n\x0c\n\x05\x04\x01\x02\0\
    \x01\x12\x03\x11\x15\x1f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x11\"#\nA\
    \n\x04\x04\x01\x02\x01\x12\x03\x12\x02(\"4\x20uint8.\x200=Hidden,\x201=F\
    ogged,\x202=Visible,\x203=FullHidden\n\n\x0c\n\x05\x04\x01\x02\x01\x04\
    \x12\x03\x12\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x12\x0b\x14\n\
    \x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x12\x15#\n\x0c\n\x05\x04\x01\x02\
    \x01\x03\x12\x03\x12&'\n!\n\x04\x04\x01\x02\x02\x12\x03\x13\x02\x1f\"\
    \x14\x201-bit.\x20Zerg\x20creep.\n\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\
    \x03\x13\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03\x13\x0b\x14\n\x0c\
    \n\x05\x04\x01\x02\x02\x01\x12\x03\x13\x15\x1a\n\x0c\n\x05\x04\x01\x02\
    \x02\x03\x12\x03\x13\x1d\x1e\n$\n\x04\x04\x01\x02\x03\x12\x03\x14\x02\
    \x1f\"\x17\x201-bit.\x20Protoss\x20power.\n\n\x0c\n\x05\x04\x01\x02\x03\
    \x04\x12\x03\x14\x02\n\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03\x14\x0b\
    \x14\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x14\x15\x1a\n\x0c\n\x05\x04\
    \x01\x02\x03\x03\x12\x03\x14\x1d\x1e\n7\n\x04\x04\x01\x02\x04\x12\x03\
    \x15\x02#\"*\x20uint8.\x20Participants:\x20[1,\x2015]\x20Neutral:\x2016\
    \n\n\x0c\n\x05\x04\x01\x02\x04\x04\x12\x03\x15\x02\n\n\x0c\n\x05\x04\x01\
    \x02\x04\x06\x12\x03\x15\x0b\x14\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03\
    \x15\x15\x1e\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03\x15!\"\n9\n\x04\x04\
    \x01\x02\x05\x12\x03\x16\x02#\",\x20int32.\x20Unique\x20identifier\x20fo\
    r\x20type\x20of\x20unit.\n\n\x0c\n\x05\x04\x01\x02\x05\x04\x12\x03\x16\
    \x02\n\n\x0c\n\x05\x04\x01\x02\x05\x06\x12\x03\x16\x0b\x14\n\x0c\n\x05\
    \x04\x01\x02\x05\x01\x12\x03\x16\x15\x1e\n\x0c\n\x05\x04\x01\x02\x05\x03\
    \x12\x03\x16!\"\n%\n\x04\x04\x01\x02\x06\x12\x03\x17\x02\"\"\x18\x201-bi\
    t.\x20Selected\x20units.\n\n\x0c\n\x05\x04\x01\x02\x06\x04\x12\x03\x17\
    \x02\n\n\x0c\n\x05\x04\x01\x02\x06\x06\x12\x03\x17\x0b\x14\n\x0c\n\x05\
    \x04\x01\x02\x06\x01\x12\x03\x17\x15\x1d\n\x0c\n\x05\x04\x01\x02\x06\x03\
    \x12\x03\x17\x20!\n\x15\n\x04\x04\x01\x02\x07\x12\x03\x18\x02)\"\x08\x20\
    int32.\n\n\x0c\n\x05\x04\x01\x02\x07\x04\x12\x03\x18\x02\n\n\x0c\n\x05\
    \x04\x01\x02\x07\x06\x12\x03\x18\x0b\x14\n\x0c\n\x05\x04\x01\x02\x07\x01\
    \x12\x03\x18\x15$\n\x0c\n\x05\x04\x01\x02\x07\x03\x12\x03\x18'(\n^\n\x04\
    \x04\x01\x02\x08\x12\x03\x19\x020\"Q\x20uint8.\x20Ratio\x20of\x20current\
    \x20health\x20to\x20max\x20health.\x20[0%,\x20100%]\x20encoded\x20into\
    \x20[0,\x20255].\n\n\x0c\n\x05\x04\x01\x02\x08\x04\x12\x03\x19\x02\n\n\
    \x0c\n\x05\x04\x01\x02\x08\x06\x12\x03\x19\x0b\x14\n\x0c\n\x05\x04\x01\
    \x02\x08\x01\x12\x03\x19\x15*\n\x0c\n\x05\x04\x01\x02\x08\x03\x12\x03\
    \x19-/\n\x15\n\x04\x04\x01\x02\t\x12\x03\x1a\x02%\"\x08\x20int32.\n\n\
    \x0c\n\x05\x04\x01\x02\t\x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\x01\x02\t\
    \x06\x12\x03\x1a\x0b\x14\n\x0c\n\x05\x04\x01\x02\t\x01\x12\x03\x1a\x15\
    \x20\n\x0c\n\x05\x04\x01\x02\t\x03\x12\x03\x1a#$\n\x15\n\x04\x04\x01\x02\
    \n\x12\x03\x1b\x02'\"\x08\x20int32.\n\n\x0c\n\x05\x04\x01\x02\n\x04\x12\
    \x03\x1b\x02\n\n\x0c\n\x05\x04\x01\x02\n\x06\x12\x03\x1b\x0b\x14\n\x0c\n\
    \x05\x04\x01\x02\n\x01\x12\x03\x1b\x15!\n\x0c\n\x05\x04\x01\x02\n\x03\
    \x12\x03\x1b$&\nF\n\x04\x04\x01\x02\x0b\x12\x03\x1c\x02*\"9\x20uint8.\
    \x20See\x20\"Alliance\"\x20enum\x20in\x20raw.proto.\x20Range:\x20[1,\x20\
    4]\x20\n\n\x0c\n\x05\x04\x01\x02\x0b\x04\x12\x03\x1c\x02\n\n\x0c\n\x05\
    \x04\x01\x02\x0b\x06\x12\x03\x1c\x0b\x14\n\x0c\n\x05\x04\x01\x02\x0b\x01\
    \x12\x03\x1c\x15$\n\x0c\n\x05\x04\x01\x02\x0b\x03\x12\x03\x1c')\nm\n\x04\
    \x04\x01\x02\x0c\x12\x03\x1d\x02*\"`\x20uint8.\x20Density\x20of\x20units\
    \x20overlapping\x20a\x20pixel,\x20anti-aliased.\x20[0.0,\x2016.0f]\x20en\
    coded\x20into\x20[0,\x20255].\n\n\x0c\n\x05\x04\x01\x02\x0c\x04\x12\x03\
    \x1d\x02\n\n\x0c\n\x05\x04\x01\x02\x0c\x06\x12\x03\x1d\x0b\x14\n\x0c\n\
    \x05\x04\x01\x02\x0c\x01\x12\x03\x1d\x15$\n\x0c\n\x05\x04\x01\x02\x0c\
    \x03\x12\x03\x1d')\n9\n\x04\x04\x01\x02\r\x12\x03\x1e\x02'\",\x20uint8.\
    \x20Count\x20of\x20units\x20overlapping\x20a\x20pixel.\n\n\x0c\n\x05\x04\
    \x01\x02\r\x04\x12\x03\x1e\x02\n\n\x0c\n\x05\x04\x01\x02\r\x06\x12\x03\
    \x1e\x0b\x14\n\x0c\n\x05\x04\x01\x02\r\x01\x12\x03\x1e\x15!\n\x0c\n\x05\
    \x04\x01\x02\r\x03\x12\x03\x1e$&\n\n\n\x02\x04\x02\x12\x04!\0,\x01\n\n\n\
    \x03\x04\x02\x01\x12\x03!\x08\x1c\n]\n\x04\x04\x02\x02\0\x12\x03\"\x02$\
    \"P\x20uint8.\x20Terrain\x20height.\x20World\x20space\x20units\x20of\x20\
    [-200,\x20200]\x20encoded\x20into\x20[0,\x20255].\n\n\x0c\n\x05\x04\x02\
    \x02\0\x04\x12\x03\"\x02\n\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\"\x0b\
    \x14\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\"\x15\x1f\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x03\"\"#\nA\n\x04\x04\x02\x02\x01\x12\x03#\x02(\"4\x20uin\
    t8.\x200=Hidden,\x201=Fogged,\x202=Visible,\x203=FullHidden\n\n\x0c\n\
    \x05\x04\x02\x02\x01\x04\x12\x03#\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x06\
    \x12\x03#\x0b\x14\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03#\x15#\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03#&'\n!\n\x04\x04\x02\x02\x02\x12\x03$\
    \x02\x1f\"\x14\x201-bit.\x20Zerg\x20creep.\n\n\x0c\n\x05\x04\x02\x02\x02\
    \x04\x12\x03$\x02\n\n\x0c\n\x05\x04\x02\x02\x02\x06\x12\x03$\x0b\x14\n\
    \x0c\n\x05\x04\x02\x02\x02\x01\x12\x03$\x15\x1a\n\x0c\n\x05\x04\x02\x02\
    \x02\x03\x12\x03$\x1d\x1e\n1\n\x04\x04\x02\x02\x03\x12\x03%\x02\x20\"$\
    \x201-bit.\x20Area\x20covered\x20by\x20the\x20camera.\n\n\x0c\n\x05\x04\
    \x02\x02\x03\x04\x12\x03%\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03%\
    \x0b\x14\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03%\x15\x1b\n\x0c\n\x05\
    \x04\x02\x02\x03\x03\x12\x03%\x1e\x1f\n7\n\x04\x04\x02\x02\x04\x12\x03&\
    \x02#\"*\x20uint8.\x20Participants:\x20[1,\x2015]\x20Neutral:\x2016\n\n\
    \x0c\n\x05\x04\x02\x02\x04\x04\x12\x03&\x02\n\n\x0c\n\x05\x04\x02\x02\
    \x04\x06\x12\x03&\x0b\x14\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03&\x15\
    \x1e\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03&!\"\nF\n\x04\x04\x02\x02\
    \x05\x12\x03'\x02)\"9\x20uint8.\x20See\x20\"Alliance\"\x20enum\x20in\x20\
    raw.proto.\x20Range:\x20[1,\x204]\x20\n\n\x0c\n\x05\x04\x02\x02\x05\x04\
    \x12\x03'\x02\n\n\x0c\n\x05\x04\x02\x02\x05\x06\x12\x03'\x0b\x14\n\x0c\n\
    \x05\x04\x02\x02\x05\x01\x12\x03'\x15$\n\x0c\n\x05\x04\x02\x02\x05\x03\
    \x12\x03''(\n%\n\x04\x04\x02\x02\x06\x12\x03(\x02\"\"\x18\x201-bit.\x20S\
    elected\x20units.\n\n\x0c\n\x05\x04\x02\x02\x06\x04\x12\x03(\x02\n\n\x0c\
    \n\x05\x04\x02\x02\x06\x06\x12\x03(\x0b\x14\n\x0c\n\x05\x04\x02\x02\x06\
    \x01\x12\x03(\x15\x1d\n\x0c\n\x05\x04\x02\x02\x06\x03\x12\x03(\x20!\ne\n\
    \x04\x04\x02\x02\x07\x12\x03+\x02#\x1a*\x20Cheat\x20layers.\x20Only\x20p\
    opulated\x20in\x20replays.\n\",\x20int32.\x20Unique\x20identifier\x20for\
    \x20type\x20of\x20unit.\n\n\x0c\n\x05\x04\x02\x02\x07\x04\x12\x03+\x02\n\
    \n\x0c\n\x05\x04\x02\x02\x07\x06\x12\x03+\x0b\x14\n\x0c\n\x05\x04\x02\
    \x02\x07\x01\x12\x03+\x15\x1e\n\x0c\n\x05\x04\x02\x02\x07\x03\x12\x03+!\
    \"\n&\n\x02\x04\x03\x12\x043\06\x012\x1a\n\x20Observation\x20-\x20Render\
    ed\n\n\n\n\n\x03\x04\x03\x01\x12\x033\x08\x19\n\x0b\n\x04\x04\x03\x02\0\
    \x12\x034\x02\x1d\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x034\x02\n\n\x0c\n\
    \x05\x04\x03\x02\0\x06\x12\x034\x0b\x14\n\x0c\n\x05\x04\x03\x02\0\x01\
    \x12\x034\x15\x18\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x034\x1b\x1c\n\x0b\n\
    \x04\x04\x03\x02\x01\x12\x035\x02!\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\
    \x035\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\x035\x0b\x14\n\x0c\n\x05\
    \x04\x03\x02\x01\x01\x12\x035\x15\x1c\n\x0c\n\x05\x04\x03\x02\x01\x03\
    \x12\x035\x1f\x20\n\x16\n\x02\x04\x04\x12\x04=\0D\x012\n\n\x20Action\n\n\
    \n\n\n\x03\x04\x04\x01\x12\x03=\x08\x15\n\x0c\n\x04\x04\x04\x08\0\x12\
    \x04>\x02C\x03\n\x0c\n\x05\x04\x04\x08\0\x01\x12\x03>\x08\x0e\n\x0b\n\
    \x04\x04\x04\x02\0\x12\x03?\x04.\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03?\
    \x04\x1c\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03?\x1d)\n\x0c\n\x05\x04\x04\
    \x02\0\x03\x12\x03?,-\n\x0b\n\x04\x04\x04\x02\x01\x12\x03@\x04,\n\x0c\n\
    \x05\x04\x04\x02\x01\x06\x12\x03@\x04\x1b\n\x0c\n\x05\x04\x04\x02\x01\
    \x01\x12\x03@\x1c'\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03@*+\n\x0b\n\
    \x04\x04\x04\x02\x02\x12\x03A\x04=\n\x0c\n\x05\x04\x04\x02\x02\x06\x12\
    \x03A\x04#\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03A$8\n\x0c\n\x05\x04\
    \x04\x02\x02\x03\x12\x03A;<\n\x0b\n\x04\x04\x04\x02\x03\x12\x03B\x04;\n\
    \x0c\n\x05\x04\x04\x02\x03\x06\x12\x03B\x04\"\n\x0c\n\x05\x04\x04\x02\
    \x03\x01\x12\x03B#6\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03B9:\n\n\n\x02\
    \x04\x05\x12\x04F\0N\x01\n\n\n\x03\x04\x05\x01\x12\x03F\x08\x20\n\x0b\n\
    \x04\x04\x05\x02\0\x12\x03G\x02\x20\n\x0c\n\x05\x04\x05\x02\0\x04\x12\
    \x03G\x02\n\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03G\x0b\x10\n\x0c\n\x05\
    \x04\x05\x02\0\x01\x12\x03G\x11\x1b\n\x0c\n\x05\x04\x05\x02\0\x03\x12\
    \x03G\x1e\x1f\n\x0c\n\x04\x04\x05\x08\0\x12\x04H\x02K\x03\n\x0c\n\x05\
    \x04\x05\x08\0\x01\x12\x03H\x08\x0e\n\x0b\n\x04\x04\x05\x02\x01\x12\x03I\
    \x04#\n\x0c\n\x05\x04\x05\x02\x01\x06\x12\x03I\x04\n\n\x0c\n\x05\x04\x05\
    \x02\x01\x01\x12\x03I\x0b\x1e\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03I!\
    \"\n\x0b\n\x04\x04\x05\x02\x02\x12\x03J\x04$\n\x0c\n\x05\x04\x05\x02\x02\
    \x06\x12\x03J\x04\n\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03J\x0b\x1f\n\
    \x0c\n\x05\x04\x05\x02\x02\x03\x12\x03J\"#\n+\n\x04\x04\x05\x02\x03\x12\
    \x03M\x02\"\"\x1e\x20Equivalent\x20to\x20shift+command.\n\n\x0c\n\x05\
    \x04\x05\x02\x03\x04\x12\x03M\x02\n\n\x0c\n\x05\x04\x05\x02\x03\x05\x12\
    \x03M\x0b\x0f\n\x0c\n\x05\x04\x05\x02\x03\x01\x12\x03M\x10\x1d\n\x0c\n\
    \x05\x04\x05\x02\x03\x03\x12\x03M\x20!\n\n\n\x02\x04\x06\x12\x04P\0R\x01\
    \n\n\n\x03\x04\x06\x01\x12\x03P\x08\x1f\nC\n\x04\x04\x06\x02\0\x12\x03Q\
    \x02%\"6\x20Simulates\x20a\x20click\x20on\x20the\x20minimap\x20to\x20mov\
    e\x20the\x20camera.\n\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03Q\x02\n\n\x0c\
    \n\x05\x04\x06\x02\0\x06\x12\x03Q\x0b\x11\n\x0c\n\x05\x04\x06\x02\0\x01\
    \x12\x03Q\x12\x20\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03Q#$\n\n\n\x02\x04\
    \x07\x12\x04T\0]\x01\n\n\n\x03\x04\x07\x01\x12\x03T\x08'\n\x0b\n\x04\x04\
    \x07\x02\0\x12\x03U\x02-\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03U\x02\n\n\
    \x0c\n\x05\x04\x07\x02\0\x06\x12\x03U\x0b\x11\n\x0c\n\x05\x04\x07\x02\0\
    \x01\x12\x03U\x12(\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03U+,\n\x0c\n\x04\
    \x04\x07\x04\0\x12\x04V\x02[\x03\n\x0c\n\x05\x04\x07\x04\0\x01\x12\x03V\
    \x07\x0b\nG\n\x06\x04\x07\x04\0\x02\0\x12\x03W\x04\x0f\"8\x20Equivalent\
    \x20to\x20normal\x20click.\x20Changes\x20selection\x20to\x20unit.\n\n\
    \x0e\n\x07\x04\x07\x04\0\x02\0\x01\x12\x03W\x04\n\n\x0e\n\x07\x04\x07\
    \x04\0\x02\0\x02\x12\x03W\r\x0e\nE\n\x06\x04\x07\x04\0\x02\x01\x12\x03X\
    \x04\x0f\"6\x20Equivalent\x20to\x20shift+click.\x20Toggle\x20selection\
    \x20of\x20unit.\n\n\x0e\n\x07\x04\x07\x04\0\x02\x01\x01\x12\x03X\x04\n\n\
    \x0e\n\x07\x04\x07\x04\0\x02\x01\x02\x12\x03X\r\x0e\nP\n\x06\x04\x07\x04\
    \0\x02\x02\x12\x03Y\x04\x10\"A\x20Equivalent\x20to\x20control+click.\x20\
    Selects\x20all\x20units\x20of\x20a\x20given\x20type.\n\n\x0e\n\x07\x04\
    \x07\x04\0\x02\x02\x01\x12\x03Y\x04\x0b\n\x0e\n\x07\x04\x07\x04\0\x02\
    \x02\x02\x12\x03Y\x0e\x0f\nV\n\x06\x04\x07\x04\0\x02\x03\x12\x03Z\x04\
    \x13\"G\x20Equivalent\x20to\x20shift+control+click.\x20Selects\x20all\
    \x20units\x20of\x20a\x20given\x20type.\n\n\x0e\n\x07\x04\x07\x04\0\x02\
    \x03\x01\x12\x03Z\x04\x0e\n\x0e\n\x07\x04\x07\x04\0\x02\x03\x02\x12\x03Z\
    \x11\x12\n\x0b\n\x04\x04\x07\x02\x01\x12\x03\\\x02\x19\n\x0c\n\x05\x04\
    \x07\x02\x01\x04\x12\x03\\\x02\n\n\x0c\n\x05\x04\x07\x02\x01\x06\x12\x03\
    \\\x0b\x0f\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03\\\x10\x14\n\x0c\n\x05\
    \x04\x07\x02\x01\x03\x12\x03\\\x17\x18\n\n\n\x02\x04\x08\x12\x04_\0b\x01\
    \n\n\n\x03\x04\x08\x01\x12\x03_\x08&\ng\n\x04\x04\x08\x02\0\x12\x03`\x02\
    1\"Z\x20Eventually\x20this\x20should\x20not\x20be\x20an\x20array,\x20but\
    \x20a\x20single\x20field\x20(multiple\x20would\x20be\x20cheating).\n\n\
    \x0c\n\x05\x04\x08\x02\0\x04\x12\x03`\x02\n\n\x0c\n\x05\x04\x08\x02\0\
    \x06\x12\x03`\x0b\x15\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03`\x16,\n\x0c\
    \n\x05\x04\x08\x02\0\x03\x12\x03`/0\nA\n\x04\x04\x08\x02\x01\x12\x03a\
    \x02\"\"4\x20Equivalent\x20to\x20shift+drag.\x20Adds\x20units\x20to\x20s\
    election.\n\n\x0c\n\x05\x04\x08\x02\x01\x04\x12\x03a\x02\n\n\x0c\n\x05\
    \x04\x08\x02\x01\x05\x12\x03a\x0b\x0f\n\x0c\n\x05\x04\x08\x02\x01\x01\
    \x12\x03a\x10\x1d\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x03a\x20!\
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
