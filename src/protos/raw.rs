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
pub struct StartRaw {
    // message fields
    map_size: ::protobuf::SingularPtrField<super::common::Size2DI>,
    pathing_grid: ::protobuf::SingularPtrField<super::common::ImageData>,
    terrain_height: ::protobuf::SingularPtrField<super::common::ImageData>,
    placement_grid: ::protobuf::SingularPtrField<super::common::ImageData>,
    playable_area: ::protobuf::SingularPtrField<super::common::RectangleI>,
    start_locations: ::protobuf::RepeatedField<super::common::Point2D>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StartRaw {}

impl StartRaw {
    pub fn new() -> StartRaw {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartRaw {
        static mut instance: ::protobuf::lazy::Lazy<StartRaw> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartRaw,
        };
        unsafe {
            instance.get(StartRaw::new)
        }
    }

    // optional .SC2APIProtocol.Size2DI map_size = 1;

    pub fn clear_map_size(&mut self) {
        self.map_size.clear();
    }

    pub fn has_map_size(&self) -> bool {
        self.map_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_size(&mut self, v: super::common::Size2DI) {
        self.map_size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_size(&mut self) -> &mut super::common::Size2DI {
        if self.map_size.is_none() {
            self.map_size.set_default();
        }
        self.map_size.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_size(&mut self) -> super::common::Size2DI {
        self.map_size.take().unwrap_or_else(|| super::common::Size2DI::new())
    }

    pub fn get_map_size(&self) -> &super::common::Size2DI {
        self.map_size.as_ref().unwrap_or_else(|| super::common::Size2DI::default_instance())
    }

    fn get_map_size_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Size2DI> {
        &self.map_size
    }

    fn mut_map_size_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Size2DI> {
        &mut self.map_size
    }

    // optional .SC2APIProtocol.ImageData pathing_grid = 2;

    pub fn clear_pathing_grid(&mut self) {
        self.pathing_grid.clear();
    }

    pub fn has_pathing_grid(&self) -> bool {
        self.pathing_grid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pathing_grid(&mut self, v: super::common::ImageData) {
        self.pathing_grid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pathing_grid(&mut self) -> &mut super::common::ImageData {
        if self.pathing_grid.is_none() {
            self.pathing_grid.set_default();
        }
        self.pathing_grid.as_mut().unwrap()
    }

    // Take field
    pub fn take_pathing_grid(&mut self) -> super::common::ImageData {
        self.pathing_grid.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_pathing_grid(&self) -> &super::common::ImageData {
        self.pathing_grid.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_pathing_grid_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.pathing_grid
    }

    fn mut_pathing_grid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.pathing_grid
    }

    // optional .SC2APIProtocol.ImageData terrain_height = 3;

    pub fn clear_terrain_height(&mut self) {
        self.terrain_height.clear();
    }

    pub fn has_terrain_height(&self) -> bool {
        self.terrain_height.is_some()
    }

    // Param is passed by value, moved
    pub fn set_terrain_height(&mut self, v: super::common::ImageData) {
        self.terrain_height = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_terrain_height(&mut self) -> &mut super::common::ImageData {
        if self.terrain_height.is_none() {
            self.terrain_height.set_default();
        }
        self.terrain_height.as_mut().unwrap()
    }

    // Take field
    pub fn take_terrain_height(&mut self) -> super::common::ImageData {
        self.terrain_height.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_terrain_height(&self) -> &super::common::ImageData {
        self.terrain_height.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_terrain_height_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.terrain_height
    }

    fn mut_terrain_height_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.terrain_height
    }

    // optional .SC2APIProtocol.ImageData placement_grid = 4;

    pub fn clear_placement_grid(&mut self) {
        self.placement_grid.clear();
    }

    pub fn has_placement_grid(&self) -> bool {
        self.placement_grid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_placement_grid(&mut self, v: super::common::ImageData) {
        self.placement_grid = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_placement_grid(&mut self) -> &mut super::common::ImageData {
        if self.placement_grid.is_none() {
            self.placement_grid.set_default();
        }
        self.placement_grid.as_mut().unwrap()
    }

    // Take field
    pub fn take_placement_grid(&mut self) -> super::common::ImageData {
        self.placement_grid.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_placement_grid(&self) -> &super::common::ImageData {
        self.placement_grid.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_placement_grid_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.placement_grid
    }

    fn mut_placement_grid_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.placement_grid
    }

    // optional .SC2APIProtocol.RectangleI playable_area = 5;

    pub fn clear_playable_area(&mut self) {
        self.playable_area.clear();
    }

    pub fn has_playable_area(&self) -> bool {
        self.playable_area.is_some()
    }

    // Param is passed by value, moved
    pub fn set_playable_area(&mut self, v: super::common::RectangleI) {
        self.playable_area = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_playable_area(&mut self) -> &mut super::common::RectangleI {
        if self.playable_area.is_none() {
            self.playable_area.set_default();
        }
        self.playable_area.as_mut().unwrap()
    }

    // Take field
    pub fn take_playable_area(&mut self) -> super::common::RectangleI {
        self.playable_area.take().unwrap_or_else(|| super::common::RectangleI::new())
    }

    pub fn get_playable_area(&self) -> &super::common::RectangleI {
        self.playable_area.as_ref().unwrap_or_else(|| super::common::RectangleI::default_instance())
    }

    fn get_playable_area_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::RectangleI> {
        &self.playable_area
    }

    fn mut_playable_area_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::RectangleI> {
        &mut self.playable_area
    }

    // repeated .SC2APIProtocol.Point2D start_locations = 6;

    pub fn clear_start_locations(&mut self) {
        self.start_locations.clear();
    }

    // Param is passed by value, moved
    pub fn set_start_locations(&mut self, v: ::protobuf::RepeatedField<super::common::Point2D>) {
        self.start_locations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_start_locations(&mut self) -> &mut ::protobuf::RepeatedField<super::common::Point2D> {
        &mut self.start_locations
    }

    // Take field
    pub fn take_start_locations(&mut self) -> ::protobuf::RepeatedField<super::common::Point2D> {
        ::std::mem::replace(&mut self.start_locations, ::protobuf::RepeatedField::new())
    }

    pub fn get_start_locations(&self) -> &[super::common::Point2D] {
        &self.start_locations
    }

    fn get_start_locations_for_reflect(&self) -> &::protobuf::RepeatedField<super::common::Point2D> {
        &self.start_locations
    }

    fn mut_start_locations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::common::Point2D> {
        &mut self.start_locations
    }
}

impl ::protobuf::Message for StartRaw {
    fn is_initialized(&self) -> bool {
        for v in &self.map_size {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pathing_grid {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.terrain_height {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.placement_grid {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.playable_area {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.start_locations {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.map_size)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pathing_grid)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.terrain_height)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.placement_grid)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.playable_area)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.start_locations)?;
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
        if let Some(ref v) = self.map_size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.pathing_grid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.terrain_height.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.placement_grid.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.playable_area.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.start_locations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.map_size.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.pathing_grid.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.terrain_height.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.placement_grid.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.playable_area.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.start_locations {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for StartRaw {
    fn new() -> StartRaw {
        StartRaw::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartRaw>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Size2DI>>(
                    "map_size",
                    StartRaw::get_map_size_for_reflect,
                    StartRaw::mut_map_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "pathing_grid",
                    StartRaw::get_pathing_grid_for_reflect,
                    StartRaw::mut_pathing_grid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "terrain_height",
                    StartRaw::get_terrain_height_for_reflect,
                    StartRaw::mut_terrain_height_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "placement_grid",
                    StartRaw::get_placement_grid_for_reflect,
                    StartRaw::mut_placement_grid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::RectangleI>>(
                    "playable_area",
                    StartRaw::get_playable_area_for_reflect,
                    StartRaw::mut_playable_area_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point2D>>(
                    "start_locations",
                    StartRaw::get_start_locations_for_reflect,
                    StartRaw::mut_start_locations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StartRaw>(
                    "StartRaw",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartRaw {
    fn clear(&mut self) {
        self.clear_map_size();
        self.clear_pathing_grid();
        self.clear_terrain_height();
        self.clear_placement_grid();
        self.clear_playable_area();
        self.clear_start_locations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StartRaw {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StartRaw {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ObservationRaw {
    // message fields
    player: ::protobuf::SingularPtrField<PlayerRaw>,
    units: ::protobuf::RepeatedField<Unit>,
    map_state: ::protobuf::SingularPtrField<MapState>,
    event: ::protobuf::SingularPtrField<Event>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObservationRaw {}

impl ObservationRaw {
    pub fn new() -> ObservationRaw {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObservationRaw {
        static mut instance: ::protobuf::lazy::Lazy<ObservationRaw> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObservationRaw,
        };
        unsafe {
            instance.get(ObservationRaw::new)
        }
    }

    // optional .SC2APIProtocol.PlayerRaw player = 1;

    pub fn clear_player(&mut self) {
        self.player.clear();
    }

    pub fn has_player(&self) -> bool {
        self.player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player(&mut self, v: PlayerRaw) {
        self.player = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player(&mut self) -> &mut PlayerRaw {
        if self.player.is_none() {
            self.player.set_default();
        }
        self.player.as_mut().unwrap()
    }

    // Take field
    pub fn take_player(&mut self) -> PlayerRaw {
        self.player.take().unwrap_or_else(|| PlayerRaw::new())
    }

    pub fn get_player(&self) -> &PlayerRaw {
        self.player.as_ref().unwrap_or_else(|| PlayerRaw::default_instance())
    }

    fn get_player_for_reflect(&self) -> &::protobuf::SingularPtrField<PlayerRaw> {
        &self.player
    }

    fn mut_player_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PlayerRaw> {
        &mut self.player
    }

    // repeated .SC2APIProtocol.Unit units = 2;

    pub fn clear_units(&mut self) {
        self.units.clear();
    }

    // Param is passed by value, moved
    pub fn set_units(&mut self, v: ::protobuf::RepeatedField<Unit>) {
        self.units = v;
    }

    // Mutable pointer to the field.
    pub fn mut_units(&mut self) -> &mut ::protobuf::RepeatedField<Unit> {
        &mut self.units
    }

    // Take field
    pub fn take_units(&mut self) -> ::protobuf::RepeatedField<Unit> {
        ::std::mem::replace(&mut self.units, ::protobuf::RepeatedField::new())
    }

    pub fn get_units(&self) -> &[Unit] {
        &self.units
    }

    fn get_units_for_reflect(&self) -> &::protobuf::RepeatedField<Unit> {
        &self.units
    }

    fn mut_units_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Unit> {
        &mut self.units
    }

    // optional .SC2APIProtocol.MapState map_state = 3;

    pub fn clear_map_state(&mut self) {
        self.map_state.clear();
    }

    pub fn has_map_state(&self) -> bool {
        self.map_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_state(&mut self, v: MapState) {
        self.map_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_state(&mut self) -> &mut MapState {
        if self.map_state.is_none() {
            self.map_state.set_default();
        }
        self.map_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_state(&mut self) -> MapState {
        self.map_state.take().unwrap_or_else(|| MapState::new())
    }

    pub fn get_map_state(&self) -> &MapState {
        self.map_state.as_ref().unwrap_or_else(|| MapState::default_instance())
    }

    fn get_map_state_for_reflect(&self) -> &::protobuf::SingularPtrField<MapState> {
        &self.map_state
    }

    fn mut_map_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MapState> {
        &mut self.map_state
    }

    // optional .SC2APIProtocol.Event event = 4;

    pub fn clear_event(&mut self) {
        self.event.clear();
    }

    pub fn has_event(&self) -> bool {
        self.event.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event(&mut self, v: Event) {
        self.event = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event(&mut self) -> &mut Event {
        if self.event.is_none() {
            self.event.set_default();
        }
        self.event.as_mut().unwrap()
    }

    // Take field
    pub fn take_event(&mut self) -> Event {
        self.event.take().unwrap_or_else(|| Event::new())
    }

    pub fn get_event(&self) -> &Event {
        self.event.as_ref().unwrap_or_else(|| Event::default_instance())
    }

    fn get_event_for_reflect(&self) -> &::protobuf::SingularPtrField<Event> {
        &self.event
    }

    fn mut_event_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Event> {
        &mut self.event
    }
}

impl ::protobuf::Message for ObservationRaw {
    fn is_initialized(&self) -> bool {
        for v in &self.player {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.units {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.map_state {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.event {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.units)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.map_state)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.event)?;
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
        if let Some(ref v) = self.player.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.units {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.map_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.event.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.player.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.units {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.map_state.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.event.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ObservationRaw {
    fn new() -> ObservationRaw {
        ObservationRaw::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObservationRaw>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlayerRaw>>(
                    "player",
                    ObservationRaw::get_player_for_reflect,
                    ObservationRaw::mut_player_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Unit>>(
                    "units",
                    ObservationRaw::get_units_for_reflect,
                    ObservationRaw::mut_units_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MapState>>(
                    "map_state",
                    ObservationRaw::get_map_state_for_reflect,
                    ObservationRaw::mut_map_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Event>>(
                    "event",
                    ObservationRaw::get_event_for_reflect,
                    ObservationRaw::mut_event_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObservationRaw>(
                    "ObservationRaw",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObservationRaw {
    fn clear(&mut self) {
        self.clear_player();
        self.clear_units();
        self.clear_map_state();
        self.clear_event();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ObservationRaw {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ObservationRaw {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PowerSource {
    // message fields
    pos: ::protobuf::SingularPtrField<super::common::Point>,
    radius: ::std::option::Option<f32>,
    tag: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PowerSource {}

impl PowerSource {
    pub fn new() -> PowerSource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PowerSource {
        static mut instance: ::protobuf::lazy::Lazy<PowerSource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PowerSource,
        };
        unsafe {
            instance.get(PowerSource::new)
        }
    }

    // optional .SC2APIProtocol.Point pos = 1;

    pub fn clear_pos(&mut self) {
        self.pos.clear();
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: super::common::Point) {
        self.pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pos(&mut self) -> &mut super::common::Point {
        if self.pos.is_none() {
            self.pos.set_default();
        }
        self.pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_pos(&mut self) -> super::common::Point {
        self.pos.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_pos(&self) -> &super::common::Point {
        self.pos.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.pos
    }

    fn mut_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.pos
    }

    // optional float radius = 2;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: f32) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius(&self) -> f32 {
        self.radius.unwrap_or(0.)
    }

    fn get_radius_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.radius
    }

    fn mut_radius_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.radius
    }

    // optional uint64 tag = 3;

    pub fn clear_tag(&mut self) {
        self.tag = ::std::option::Option::None;
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: u64) {
        self.tag = ::std::option::Option::Some(v);
    }

    pub fn get_tag(&self) -> u64 {
        self.tag.unwrap_or(0)
    }

    fn get_tag_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.tag
    }
}

impl ::protobuf::Message for PowerSource {
    fn is_initialized(&self) -> bool {
        for v in &self.pos {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pos)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.radius = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.tag = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.radius {
            my_size += 5;
        }
        if let Some(v) = self.tag {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.pos.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.radius {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.tag {
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

impl ::protobuf::MessageStatic for PowerSource {
    fn new() -> PowerSource {
        PowerSource::new()
    }

    fn descriptor_static(_: ::std::option::Option<PowerSource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "pos",
                    PowerSource::get_pos_for_reflect,
                    PowerSource::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "radius",
                    PowerSource::get_radius_for_reflect,
                    PowerSource::mut_radius_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "tag",
                    PowerSource::get_tag_for_reflect,
                    PowerSource::mut_tag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PowerSource>(
                    "PowerSource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PowerSource {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_radius();
        self.clear_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PowerSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PowerSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlayerRaw {
    // message fields
    power_sources: ::protobuf::RepeatedField<PowerSource>,
    camera: ::protobuf::SingularPtrField<super::common::Point>,
    upgrade_ids: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerRaw {}

impl PlayerRaw {
    pub fn new() -> PlayerRaw {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerRaw {
        static mut instance: ::protobuf::lazy::Lazy<PlayerRaw> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerRaw,
        };
        unsafe {
            instance.get(PlayerRaw::new)
        }
    }

    // repeated .SC2APIProtocol.PowerSource power_sources = 1;

    pub fn clear_power_sources(&mut self) {
        self.power_sources.clear();
    }

    // Param is passed by value, moved
    pub fn set_power_sources(&mut self, v: ::protobuf::RepeatedField<PowerSource>) {
        self.power_sources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_power_sources(&mut self) -> &mut ::protobuf::RepeatedField<PowerSource> {
        &mut self.power_sources
    }

    // Take field
    pub fn take_power_sources(&mut self) -> ::protobuf::RepeatedField<PowerSource> {
        ::std::mem::replace(&mut self.power_sources, ::protobuf::RepeatedField::new())
    }

    pub fn get_power_sources(&self) -> &[PowerSource] {
        &self.power_sources
    }

    fn get_power_sources_for_reflect(&self) -> &::protobuf::RepeatedField<PowerSource> {
        &self.power_sources
    }

    fn mut_power_sources_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PowerSource> {
        &mut self.power_sources
    }

    // optional .SC2APIProtocol.Point camera = 2;

    pub fn clear_camera(&mut self) {
        self.camera.clear();
    }

    pub fn has_camera(&self) -> bool {
        self.camera.is_some()
    }

    // Param is passed by value, moved
    pub fn set_camera(&mut self, v: super::common::Point) {
        self.camera = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_camera(&mut self) -> &mut super::common::Point {
        if self.camera.is_none() {
            self.camera.set_default();
        }
        self.camera.as_mut().unwrap()
    }

    // Take field
    pub fn take_camera(&mut self) -> super::common::Point {
        self.camera.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_camera(&self) -> &super::common::Point {
        self.camera.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_camera_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.camera
    }

    fn mut_camera_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.camera
    }

    // repeated uint32 upgrade_ids = 3;

    pub fn clear_upgrade_ids(&mut self) {
        self.upgrade_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_upgrade_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.upgrade_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_upgrade_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.upgrade_ids
    }

    // Take field
    pub fn take_upgrade_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.upgrade_ids, ::std::vec::Vec::new())
    }

    pub fn get_upgrade_ids(&self) -> &[u32] {
        &self.upgrade_ids
    }

    fn get_upgrade_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.upgrade_ids
    }

    fn mut_upgrade_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.upgrade_ids
    }
}

impl ::protobuf::Message for PlayerRaw {
    fn is_initialized(&self) -> bool {
        for v in &self.power_sources {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.camera {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.power_sources)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.camera)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.upgrade_ids)?;
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
        for value in &self.power_sources {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.camera.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.upgrade_ids {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.power_sources {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.camera.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.upgrade_ids {
            os.write_uint32(3, *v)?;
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

impl ::protobuf::MessageStatic for PlayerRaw {
    fn new() -> PlayerRaw {
        PlayerRaw::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerRaw>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PowerSource>>(
                    "power_sources",
                    PlayerRaw::get_power_sources_for_reflect,
                    PlayerRaw::mut_power_sources_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "camera",
                    PlayerRaw::get_camera_for_reflect,
                    PlayerRaw::mut_camera_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "upgrade_ids",
                    PlayerRaw::get_upgrade_ids_for_reflect,
                    PlayerRaw::mut_upgrade_ids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerRaw>(
                    "PlayerRaw",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerRaw {
    fn clear(&mut self) {
        self.clear_power_sources();
        self.clear_camera();
        self.clear_upgrade_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlayerRaw {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerRaw {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnitOrder {
    // message fields
    ability_id: ::std::option::Option<u32>,
    progress: ::std::option::Option<f32>,
    // message oneof groups
    target: ::std::option::Option<UnitOrder_oneof_target>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnitOrder {}

#[derive(Clone,PartialEq)]
pub enum UnitOrder_oneof_target {
    target_world_space_pos(super::common::Point),
    target_unit_tag(u64),
}

impl UnitOrder {
    pub fn new() -> UnitOrder {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnitOrder {
        static mut instance: ::protobuf::lazy::Lazy<UnitOrder> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnitOrder,
        };
        unsafe {
            instance.get(UnitOrder::new)
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

    // optional .SC2APIProtocol.Point target_world_space_pos = 2;

    pub fn clear_target_world_space_pos(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target_world_space_pos(&self) -> bool {
        match self.target {
            ::std::option::Option::Some(UnitOrder_oneof_target::target_world_space_pos(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_target_world_space_pos(&mut self, v: super::common::Point) {
        self.target = ::std::option::Option::Some(UnitOrder_oneof_target::target_world_space_pos(v))
    }

    // Mutable pointer to the field.
    pub fn mut_target_world_space_pos(&mut self) -> &mut super::common::Point {
        if let ::std::option::Option::Some(UnitOrder_oneof_target::target_world_space_pos(_)) = self.target {
        } else {
            self.target = ::std::option::Option::Some(UnitOrder_oneof_target::target_world_space_pos(super::common::Point::new()));
        }
        match self.target {
            ::std::option::Option::Some(UnitOrder_oneof_target::target_world_space_pos(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_target_world_space_pos(&mut self) -> super::common::Point {
        if self.has_target_world_space_pos() {
            match self.target.take() {
                ::std::option::Option::Some(UnitOrder_oneof_target::target_world_space_pos(v)) => v,
                _ => panic!(),
            }
        } else {
            super::common::Point::new()
        }
    }

    pub fn get_target_world_space_pos(&self) -> &super::common::Point {
        match self.target {
            ::std::option::Option::Some(UnitOrder_oneof_target::target_world_space_pos(ref v)) => v,
            _ => super::common::Point::default_instance(),
        }
    }

    // optional uint64 target_unit_tag = 3;

    pub fn clear_target_unit_tag(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target_unit_tag(&self) -> bool {
        match self.target {
            ::std::option::Option::Some(UnitOrder_oneof_target::target_unit_tag(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_target_unit_tag(&mut self, v: u64) {
        self.target = ::std::option::Option::Some(UnitOrder_oneof_target::target_unit_tag(v))
    }

    pub fn get_target_unit_tag(&self) -> u64 {
        match self.target {
            ::std::option::Option::Some(UnitOrder_oneof_target::target_unit_tag(v)) => v,
            _ => 0,
        }
    }

    // optional float progress = 4;

    pub fn clear_progress(&mut self) {
        self.progress = ::std::option::Option::None;
    }

    pub fn has_progress(&self) -> bool {
        self.progress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_progress(&mut self, v: f32) {
        self.progress = ::std::option::Option::Some(v);
    }

    pub fn get_progress(&self) -> f32 {
        self.progress.unwrap_or(0.)
    }

    fn get_progress_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.progress
    }

    fn mut_progress_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.progress
    }
}

impl ::protobuf::Message for UnitOrder {
    fn is_initialized(&self) -> bool {
        if let Some(UnitOrder_oneof_target::target_world_space_pos(ref v)) = self.target {
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
                    let tmp = is.read_uint32()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target = ::std::option::Option::Some(UnitOrder_oneof_target::target_world_space_pos(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target = ::std::option::Option::Some(UnitOrder_oneof_target::target_unit_tag(is.read_uint64()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.progress = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.progress {
            my_size += 5;
        }
        if let ::std::option::Option::Some(ref v) = self.target {
            match v {
                &UnitOrder_oneof_target::target_world_space_pos(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &UnitOrder_oneof_target::target_unit_tag(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.progress {
            os.write_float(4, v)?;
        }
        if let ::std::option::Option::Some(ref v) = self.target {
            match v {
                &UnitOrder_oneof_target::target_world_space_pos(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &UnitOrder_oneof_target::target_unit_tag(v) => {
                    os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for UnitOrder {
    fn new() -> UnitOrder {
        UnitOrder::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnitOrder>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ability_id",
                    UnitOrder::get_ability_id_for_reflect,
                    UnitOrder::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::common::Point>(
                    "target_world_space_pos",
                    UnitOrder::has_target_world_space_pos,
                    UnitOrder::get_target_world_space_pos,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "target_unit_tag",
                    UnitOrder::has_target_unit_tag,
                    UnitOrder::get_target_unit_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "progress",
                    UnitOrder::get_progress_for_reflect,
                    UnitOrder::mut_progress_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnitOrder>(
                    "UnitOrder",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnitOrder {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.clear_target_world_space_pos();
        self.clear_target_unit_tag();
        self.clear_progress();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnitOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnitOrder {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PassengerUnit {
    // message fields
    tag: ::std::option::Option<u64>,
    health: ::std::option::Option<f32>,
    health_max: ::std::option::Option<f32>,
    shield: ::std::option::Option<f32>,
    energy: ::std::option::Option<f32>,
    unit_type: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PassengerUnit {}

impl PassengerUnit {
    pub fn new() -> PassengerUnit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PassengerUnit {
        static mut instance: ::protobuf::lazy::Lazy<PassengerUnit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PassengerUnit,
        };
        unsafe {
            instance.get(PassengerUnit::new)
        }
    }

    // optional uint64 tag = 1;

    pub fn clear_tag(&mut self) {
        self.tag = ::std::option::Option::None;
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: u64) {
        self.tag = ::std::option::Option::Some(v);
    }

    pub fn get_tag(&self) -> u64 {
        self.tag.unwrap_or(0)
    }

    fn get_tag_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.tag
    }

    // optional float health = 2;

    pub fn clear_health(&mut self) {
        self.health = ::std::option::Option::None;
    }

    pub fn has_health(&self) -> bool {
        self.health.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health(&mut self, v: f32) {
        self.health = ::std::option::Option::Some(v);
    }

    pub fn get_health(&self) -> f32 {
        self.health.unwrap_or(0.)
    }

    fn get_health_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.health
    }

    fn mut_health_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.health
    }

    // optional float health_max = 3;

    pub fn clear_health_max(&mut self) {
        self.health_max = ::std::option::Option::None;
    }

    pub fn has_health_max(&self) -> bool {
        self.health_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health_max(&mut self, v: f32) {
        self.health_max = ::std::option::Option::Some(v);
    }

    pub fn get_health_max(&self) -> f32 {
        self.health_max.unwrap_or(0.)
    }

    fn get_health_max_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.health_max
    }

    fn mut_health_max_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.health_max
    }

    // optional float shield = 4;

    pub fn clear_shield(&mut self) {
        self.shield = ::std::option::Option::None;
    }

    pub fn has_shield(&self) -> bool {
        self.shield.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shield(&mut self, v: f32) {
        self.shield = ::std::option::Option::Some(v);
    }

    pub fn get_shield(&self) -> f32 {
        self.shield.unwrap_or(0.)
    }

    fn get_shield_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.shield
    }

    fn mut_shield_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.shield
    }

    // optional float energy = 5;

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

    // optional uint32 unit_type = 6;

    pub fn clear_unit_type(&mut self) {
        self.unit_type = ::std::option::Option::None;
    }

    pub fn has_unit_type(&self) -> bool {
        self.unit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_type(&mut self, v: u32) {
        self.unit_type = ::std::option::Option::Some(v);
    }

    pub fn get_unit_type(&self) -> u32 {
        self.unit_type.unwrap_or(0)
    }

    fn get_unit_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unit_type
    }

    fn mut_unit_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unit_type
    }
}

impl ::protobuf::Message for PassengerUnit {
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
                    self.tag = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.health = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.health_max = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.shield = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.energy = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.unit_type = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tag {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.health {
            my_size += 5;
        }
        if let Some(v) = self.health_max {
            my_size += 5;
        }
        if let Some(v) = self.shield {
            my_size += 5;
        }
        if let Some(v) = self.energy {
            my_size += 5;
        }
        if let Some(v) = self.unit_type {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tag {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.health {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.health_max {
            os.write_float(3, v)?;
        }
        if let Some(v) = self.shield {
            os.write_float(4, v)?;
        }
        if let Some(v) = self.energy {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.unit_type {
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

impl ::protobuf::MessageStatic for PassengerUnit {
    fn new() -> PassengerUnit {
        PassengerUnit::new()
    }

    fn descriptor_static(_: ::std::option::Option<PassengerUnit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "tag",
                    PassengerUnit::get_tag_for_reflect,
                    PassengerUnit::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "health",
                    PassengerUnit::get_health_for_reflect,
                    PassengerUnit::mut_health_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "health_max",
                    PassengerUnit::get_health_max_for_reflect,
                    PassengerUnit::mut_health_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "shield",
                    PassengerUnit::get_shield_for_reflect,
                    PassengerUnit::mut_shield_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "energy",
                    PassengerUnit::get_energy_for_reflect,
                    PassengerUnit::mut_energy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unit_type",
                    PassengerUnit::get_unit_type_for_reflect,
                    PassengerUnit::mut_unit_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PassengerUnit>(
                    "PassengerUnit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PassengerUnit {
    fn clear(&mut self) {
        self.clear_tag();
        self.clear_health();
        self.clear_health_max();
        self.clear_shield();
        self.clear_energy();
        self.clear_unit_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PassengerUnit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PassengerUnit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Unit {
    // message fields
    display_type: ::std::option::Option<DisplayType>,
    alliance: ::std::option::Option<Alliance>,
    tag: ::std::option::Option<u64>,
    unit_type: ::std::option::Option<u32>,
    owner: ::std::option::Option<i32>,
    pos: ::protobuf::SingularPtrField<super::common::Point>,
    facing: ::std::option::Option<f32>,
    radius: ::std::option::Option<f32>,
    build_progress: ::std::option::Option<f32>,
    cloak: ::std::option::Option<CloakState>,
    detect_range: ::std::option::Option<f32>,
    radar_range: ::std::option::Option<f32>,
    is_selected: ::std::option::Option<bool>,
    is_on_screen: ::std::option::Option<bool>,
    is_blip: ::std::option::Option<bool>,
    is_powered: ::std::option::Option<bool>,
    health: ::std::option::Option<f32>,
    health_max: ::std::option::Option<f32>,
    shield: ::std::option::Option<f32>,
    energy: ::std::option::Option<f32>,
    mineral_contents: ::std::option::Option<i32>,
    vespene_contents: ::std::option::Option<i32>,
    is_flying: ::std::option::Option<bool>,
    is_burrowed: ::std::option::Option<bool>,
    orders: ::protobuf::RepeatedField<UnitOrder>,
    add_on_tag: ::std::option::Option<u64>,
    passengers: ::protobuf::RepeatedField<PassengerUnit>,
    cargo_space_taken: ::std::option::Option<i32>,
    cargo_space_max: ::std::option::Option<i32>,
    buff_ids: ::std::vec::Vec<u32>,
    assigned_harvesters: ::std::option::Option<i32>,
    ideal_harvesters: ::std::option::Option<i32>,
    weapon_cooldown: ::std::option::Option<f32>,
    engaged_target_tag: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Unit {}

impl Unit {
    pub fn new() -> Unit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Unit {
        static mut instance: ::protobuf::lazy::Lazy<Unit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Unit,
        };
        unsafe {
            instance.get(Unit::new)
        }
    }

    // optional .SC2APIProtocol.DisplayType display_type = 1;

    pub fn clear_display_type(&mut self) {
        self.display_type = ::std::option::Option::None;
    }

    pub fn has_display_type(&self) -> bool {
        self.display_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_display_type(&mut self, v: DisplayType) {
        self.display_type = ::std::option::Option::Some(v);
    }

    pub fn get_display_type(&self) -> DisplayType {
        self.display_type.unwrap_or(DisplayType::Visible)
    }

    fn get_display_type_for_reflect(&self) -> &::std::option::Option<DisplayType> {
        &self.display_type
    }

    fn mut_display_type_for_reflect(&mut self) -> &mut ::std::option::Option<DisplayType> {
        &mut self.display_type
    }

    // optional .SC2APIProtocol.Alliance alliance = 2;

    pub fn clear_alliance(&mut self) {
        self.alliance = ::std::option::Option::None;
    }

    pub fn has_alliance(&self) -> bool {
        self.alliance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alliance(&mut self, v: Alliance) {
        self.alliance = ::std::option::Option::Some(v);
    }

    pub fn get_alliance(&self) -> Alliance {
        self.alliance.unwrap_or(Alliance::Self)
    }

    fn get_alliance_for_reflect(&self) -> &::std::option::Option<Alliance> {
        &self.alliance
    }

    fn mut_alliance_for_reflect(&mut self) -> &mut ::std::option::Option<Alliance> {
        &mut self.alliance
    }

    // optional uint64 tag = 3;

    pub fn clear_tag(&mut self) {
        self.tag = ::std::option::Option::None;
    }

    pub fn has_tag(&self) -> bool {
        self.tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: u64) {
        self.tag = ::std::option::Option::Some(v);
    }

    pub fn get_tag(&self) -> u64 {
        self.tag.unwrap_or(0)
    }

    fn get_tag_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.tag
    }

    // optional uint32 unit_type = 4;

    pub fn clear_unit_type(&mut self) {
        self.unit_type = ::std::option::Option::None;
    }

    pub fn has_unit_type(&self) -> bool {
        self.unit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_type(&mut self, v: u32) {
        self.unit_type = ::std::option::Option::Some(v);
    }

    pub fn get_unit_type(&self) -> u32 {
        self.unit_type.unwrap_or(0)
    }

    fn get_unit_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unit_type
    }

    fn mut_unit_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unit_type
    }

    // optional int32 owner = 5;

    pub fn clear_owner(&mut self) {
        self.owner = ::std::option::Option::None;
    }

    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner(&mut self, v: i32) {
        self.owner = ::std::option::Option::Some(v);
    }

    pub fn get_owner(&self) -> i32 {
        self.owner.unwrap_or(0)
    }

    fn get_owner_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.owner
    }

    fn mut_owner_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.owner
    }

    // optional .SC2APIProtocol.Point pos = 6;

    pub fn clear_pos(&mut self) {
        self.pos.clear();
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: super::common::Point) {
        self.pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pos(&mut self) -> &mut super::common::Point {
        if self.pos.is_none() {
            self.pos.set_default();
        }
        self.pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_pos(&mut self) -> super::common::Point {
        self.pos.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_pos(&self) -> &super::common::Point {
        self.pos.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.pos
    }

    fn mut_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.pos
    }

    // optional float facing = 7;

    pub fn clear_facing(&mut self) {
        self.facing = ::std::option::Option::None;
    }

    pub fn has_facing(&self) -> bool {
        self.facing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_facing(&mut self, v: f32) {
        self.facing = ::std::option::Option::Some(v);
    }

    pub fn get_facing(&self) -> f32 {
        self.facing.unwrap_or(0.)
    }

    fn get_facing_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.facing
    }

    fn mut_facing_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.facing
    }

    // optional float radius = 8;

    pub fn clear_radius(&mut self) {
        self.radius = ::std::option::Option::None;
    }

    pub fn has_radius(&self) -> bool {
        self.radius.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radius(&mut self, v: f32) {
        self.radius = ::std::option::Option::Some(v);
    }

    pub fn get_radius(&self) -> f32 {
        self.radius.unwrap_or(0.)
    }

    fn get_radius_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.radius
    }

    fn mut_radius_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.radius
    }

    // optional float build_progress = 9;

    pub fn clear_build_progress(&mut self) {
        self.build_progress = ::std::option::Option::None;
    }

    pub fn has_build_progress(&self) -> bool {
        self.build_progress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_build_progress(&mut self, v: f32) {
        self.build_progress = ::std::option::Option::Some(v);
    }

    pub fn get_build_progress(&self) -> f32 {
        self.build_progress.unwrap_or(0.)
    }

    fn get_build_progress_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.build_progress
    }

    fn mut_build_progress_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.build_progress
    }

    // optional .SC2APIProtocol.CloakState cloak = 10;

    pub fn clear_cloak(&mut self) {
        self.cloak = ::std::option::Option::None;
    }

    pub fn has_cloak(&self) -> bool {
        self.cloak.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cloak(&mut self, v: CloakState) {
        self.cloak = ::std::option::Option::Some(v);
    }

    pub fn get_cloak(&self) -> CloakState {
        self.cloak.unwrap_or(CloakState::Cloaked)
    }

    fn get_cloak_for_reflect(&self) -> &::std::option::Option<CloakState> {
        &self.cloak
    }

    fn mut_cloak_for_reflect(&mut self) -> &mut ::std::option::Option<CloakState> {
        &mut self.cloak
    }

    // optional float detect_range = 31;

    pub fn clear_detect_range(&mut self) {
        self.detect_range = ::std::option::Option::None;
    }

    pub fn has_detect_range(&self) -> bool {
        self.detect_range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_detect_range(&mut self, v: f32) {
        self.detect_range = ::std::option::Option::Some(v);
    }

    pub fn get_detect_range(&self) -> f32 {
        self.detect_range.unwrap_or(0.)
    }

    fn get_detect_range_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.detect_range
    }

    fn mut_detect_range_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.detect_range
    }

    // optional float radar_range = 32;

    pub fn clear_radar_range(&mut self) {
        self.radar_range = ::std::option::Option::None;
    }

    pub fn has_radar_range(&self) -> bool {
        self.radar_range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radar_range(&mut self, v: f32) {
        self.radar_range = ::std::option::Option::Some(v);
    }

    pub fn get_radar_range(&self) -> f32 {
        self.radar_range.unwrap_or(0.)
    }

    fn get_radar_range_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.radar_range
    }

    fn mut_radar_range_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.radar_range
    }

    // optional bool is_selected = 11;

    pub fn clear_is_selected(&mut self) {
        self.is_selected = ::std::option::Option::None;
    }

    pub fn has_is_selected(&self) -> bool {
        self.is_selected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_selected(&mut self, v: bool) {
        self.is_selected = ::std::option::Option::Some(v);
    }

    pub fn get_is_selected(&self) -> bool {
        self.is_selected.unwrap_or(false)
    }

    fn get_is_selected_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_selected
    }

    fn mut_is_selected_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_selected
    }

    // optional bool is_on_screen = 12;

    pub fn clear_is_on_screen(&mut self) {
        self.is_on_screen = ::std::option::Option::None;
    }

    pub fn has_is_on_screen(&self) -> bool {
        self.is_on_screen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_on_screen(&mut self, v: bool) {
        self.is_on_screen = ::std::option::Option::Some(v);
    }

    pub fn get_is_on_screen(&self) -> bool {
        self.is_on_screen.unwrap_or(false)
    }

    fn get_is_on_screen_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_on_screen
    }

    fn mut_is_on_screen_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_on_screen
    }

    // optional bool is_blip = 13;

    pub fn clear_is_blip(&mut self) {
        self.is_blip = ::std::option::Option::None;
    }

    pub fn has_is_blip(&self) -> bool {
        self.is_blip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_blip(&mut self, v: bool) {
        self.is_blip = ::std::option::Option::Some(v);
    }

    pub fn get_is_blip(&self) -> bool {
        self.is_blip.unwrap_or(false)
    }

    fn get_is_blip_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_blip
    }

    fn mut_is_blip_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_blip
    }

    // optional bool is_powered = 35;

    pub fn clear_is_powered(&mut self) {
        self.is_powered = ::std::option::Option::None;
    }

    pub fn has_is_powered(&self) -> bool {
        self.is_powered.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_powered(&mut self, v: bool) {
        self.is_powered = ::std::option::Option::Some(v);
    }

    pub fn get_is_powered(&self) -> bool {
        self.is_powered.unwrap_or(false)
    }

    fn get_is_powered_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_powered
    }

    fn mut_is_powered_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_powered
    }

    // optional float health = 14;

    pub fn clear_health(&mut self) {
        self.health = ::std::option::Option::None;
    }

    pub fn has_health(&self) -> bool {
        self.health.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health(&mut self, v: f32) {
        self.health = ::std::option::Option::Some(v);
    }

    pub fn get_health(&self) -> f32 {
        self.health.unwrap_or(0.)
    }

    fn get_health_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.health
    }

    fn mut_health_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.health
    }

    // optional float health_max = 15;

    pub fn clear_health_max(&mut self) {
        self.health_max = ::std::option::Option::None;
    }

    pub fn has_health_max(&self) -> bool {
        self.health_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health_max(&mut self, v: f32) {
        self.health_max = ::std::option::Option::Some(v);
    }

    pub fn get_health_max(&self) -> f32 {
        self.health_max.unwrap_or(0.)
    }

    fn get_health_max_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.health_max
    }

    fn mut_health_max_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.health_max
    }

    // optional float shield = 16;

    pub fn clear_shield(&mut self) {
        self.shield = ::std::option::Option::None;
    }

    pub fn has_shield(&self) -> bool {
        self.shield.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shield(&mut self, v: f32) {
        self.shield = ::std::option::Option::Some(v);
    }

    pub fn get_shield(&self) -> f32 {
        self.shield.unwrap_or(0.)
    }

    fn get_shield_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.shield
    }

    fn mut_shield_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.shield
    }

    // optional float energy = 17;

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

    // optional int32 mineral_contents = 18;

    pub fn clear_mineral_contents(&mut self) {
        self.mineral_contents = ::std::option::Option::None;
    }

    pub fn has_mineral_contents(&self) -> bool {
        self.mineral_contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mineral_contents(&mut self, v: i32) {
        self.mineral_contents = ::std::option::Option::Some(v);
    }

    pub fn get_mineral_contents(&self) -> i32 {
        self.mineral_contents.unwrap_or(0)
    }

    fn get_mineral_contents_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.mineral_contents
    }

    fn mut_mineral_contents_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.mineral_contents
    }

    // optional int32 vespene_contents = 19;

    pub fn clear_vespene_contents(&mut self) {
        self.vespene_contents = ::std::option::Option::None;
    }

    pub fn has_vespene_contents(&self) -> bool {
        self.vespene_contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vespene_contents(&mut self, v: i32) {
        self.vespene_contents = ::std::option::Option::Some(v);
    }

    pub fn get_vespene_contents(&self) -> i32 {
        self.vespene_contents.unwrap_or(0)
    }

    fn get_vespene_contents_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.vespene_contents
    }

    fn mut_vespene_contents_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.vespene_contents
    }

    // optional bool is_flying = 20;

    pub fn clear_is_flying(&mut self) {
        self.is_flying = ::std::option::Option::None;
    }

    pub fn has_is_flying(&self) -> bool {
        self.is_flying.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_flying(&mut self, v: bool) {
        self.is_flying = ::std::option::Option::Some(v);
    }

    pub fn get_is_flying(&self) -> bool {
        self.is_flying.unwrap_or(false)
    }

    fn get_is_flying_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_flying
    }

    fn mut_is_flying_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_flying
    }

    // optional bool is_burrowed = 21;

    pub fn clear_is_burrowed(&mut self) {
        self.is_burrowed = ::std::option::Option::None;
    }

    pub fn has_is_burrowed(&self) -> bool {
        self.is_burrowed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_burrowed(&mut self, v: bool) {
        self.is_burrowed = ::std::option::Option::Some(v);
    }

    pub fn get_is_burrowed(&self) -> bool {
        self.is_burrowed.unwrap_or(false)
    }

    fn get_is_burrowed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_burrowed
    }

    fn mut_is_burrowed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_burrowed
    }

    // repeated .SC2APIProtocol.UnitOrder orders = 22;

    pub fn clear_orders(&mut self) {
        self.orders.clear();
    }

    // Param is passed by value, moved
    pub fn set_orders(&mut self, v: ::protobuf::RepeatedField<UnitOrder>) {
        self.orders = v;
    }

    // Mutable pointer to the field.
    pub fn mut_orders(&mut self) -> &mut ::protobuf::RepeatedField<UnitOrder> {
        &mut self.orders
    }

    // Take field
    pub fn take_orders(&mut self) -> ::protobuf::RepeatedField<UnitOrder> {
        ::std::mem::replace(&mut self.orders, ::protobuf::RepeatedField::new())
    }

    pub fn get_orders(&self) -> &[UnitOrder] {
        &self.orders
    }

    fn get_orders_for_reflect(&self) -> &::protobuf::RepeatedField<UnitOrder> {
        &self.orders
    }

    fn mut_orders_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UnitOrder> {
        &mut self.orders
    }

    // optional uint64 add_on_tag = 23;

    pub fn clear_add_on_tag(&mut self) {
        self.add_on_tag = ::std::option::Option::None;
    }

    pub fn has_add_on_tag(&self) -> bool {
        self.add_on_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_add_on_tag(&mut self, v: u64) {
        self.add_on_tag = ::std::option::Option::Some(v);
    }

    pub fn get_add_on_tag(&self) -> u64 {
        self.add_on_tag.unwrap_or(0)
    }

    fn get_add_on_tag_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.add_on_tag
    }

    fn mut_add_on_tag_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.add_on_tag
    }

    // repeated .SC2APIProtocol.PassengerUnit passengers = 24;

    pub fn clear_passengers(&mut self) {
        self.passengers.clear();
    }

    // Param is passed by value, moved
    pub fn set_passengers(&mut self, v: ::protobuf::RepeatedField<PassengerUnit>) {
        self.passengers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_passengers(&mut self) -> &mut ::protobuf::RepeatedField<PassengerUnit> {
        &mut self.passengers
    }

    // Take field
    pub fn take_passengers(&mut self) -> ::protobuf::RepeatedField<PassengerUnit> {
        ::std::mem::replace(&mut self.passengers, ::protobuf::RepeatedField::new())
    }

    pub fn get_passengers(&self) -> &[PassengerUnit] {
        &self.passengers
    }

    fn get_passengers_for_reflect(&self) -> &::protobuf::RepeatedField<PassengerUnit> {
        &self.passengers
    }

    fn mut_passengers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PassengerUnit> {
        &mut self.passengers
    }

    // optional int32 cargo_space_taken = 25;

    pub fn clear_cargo_space_taken(&mut self) {
        self.cargo_space_taken = ::std::option::Option::None;
    }

    pub fn has_cargo_space_taken(&self) -> bool {
        self.cargo_space_taken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cargo_space_taken(&mut self, v: i32) {
        self.cargo_space_taken = ::std::option::Option::Some(v);
    }

    pub fn get_cargo_space_taken(&self) -> i32 {
        self.cargo_space_taken.unwrap_or(0)
    }

    fn get_cargo_space_taken_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.cargo_space_taken
    }

    fn mut_cargo_space_taken_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.cargo_space_taken
    }

    // optional int32 cargo_space_max = 26;

    pub fn clear_cargo_space_max(&mut self) {
        self.cargo_space_max = ::std::option::Option::None;
    }

    pub fn has_cargo_space_max(&self) -> bool {
        self.cargo_space_max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cargo_space_max(&mut self, v: i32) {
        self.cargo_space_max = ::std::option::Option::Some(v);
    }

    pub fn get_cargo_space_max(&self) -> i32 {
        self.cargo_space_max.unwrap_or(0)
    }

    fn get_cargo_space_max_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.cargo_space_max
    }

    fn mut_cargo_space_max_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.cargo_space_max
    }

    // repeated uint32 buff_ids = 27;

    pub fn clear_buff_ids(&mut self) {
        self.buff_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_buff_ids(&mut self, v: ::std::vec::Vec<u32>) {
        self.buff_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_buff_ids(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.buff_ids
    }

    // Take field
    pub fn take_buff_ids(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.buff_ids, ::std::vec::Vec::new())
    }

    pub fn get_buff_ids(&self) -> &[u32] {
        &self.buff_ids
    }

    fn get_buff_ids_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.buff_ids
    }

    fn mut_buff_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.buff_ids
    }

    // optional int32 assigned_harvesters = 28;

    pub fn clear_assigned_harvesters(&mut self) {
        self.assigned_harvesters = ::std::option::Option::None;
    }

    pub fn has_assigned_harvesters(&self) -> bool {
        self.assigned_harvesters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_assigned_harvesters(&mut self, v: i32) {
        self.assigned_harvesters = ::std::option::Option::Some(v);
    }

    pub fn get_assigned_harvesters(&self) -> i32 {
        self.assigned_harvesters.unwrap_or(0)
    }

    fn get_assigned_harvesters_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.assigned_harvesters
    }

    fn mut_assigned_harvesters_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.assigned_harvesters
    }

    // optional int32 ideal_harvesters = 29;

    pub fn clear_ideal_harvesters(&mut self) {
        self.ideal_harvesters = ::std::option::Option::None;
    }

    pub fn has_ideal_harvesters(&self) -> bool {
        self.ideal_harvesters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ideal_harvesters(&mut self, v: i32) {
        self.ideal_harvesters = ::std::option::Option::Some(v);
    }

    pub fn get_ideal_harvesters(&self) -> i32 {
        self.ideal_harvesters.unwrap_or(0)
    }

    fn get_ideal_harvesters_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ideal_harvesters
    }

    fn mut_ideal_harvesters_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ideal_harvesters
    }

    // optional float weapon_cooldown = 30;

    pub fn clear_weapon_cooldown(&mut self) {
        self.weapon_cooldown = ::std::option::Option::None;
    }

    pub fn has_weapon_cooldown(&self) -> bool {
        self.weapon_cooldown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weapon_cooldown(&mut self, v: f32) {
        self.weapon_cooldown = ::std::option::Option::Some(v);
    }

    pub fn get_weapon_cooldown(&self) -> f32 {
        self.weapon_cooldown.unwrap_or(0.)
    }

    fn get_weapon_cooldown_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.weapon_cooldown
    }

    fn mut_weapon_cooldown_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.weapon_cooldown
    }

    // optional uint64 engaged_target_tag = 34;

    pub fn clear_engaged_target_tag(&mut self) {
        self.engaged_target_tag = ::std::option::Option::None;
    }

    pub fn has_engaged_target_tag(&self) -> bool {
        self.engaged_target_tag.is_some()
    }

    // Param is passed by value, moved
    pub fn set_engaged_target_tag(&mut self, v: u64) {
        self.engaged_target_tag = ::std::option::Option::Some(v);
    }

    pub fn get_engaged_target_tag(&self) -> u64 {
        self.engaged_target_tag.unwrap_or(0)
    }

    fn get_engaged_target_tag_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.engaged_target_tag
    }

    fn mut_engaged_target_tag_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.engaged_target_tag
    }
}

impl ::protobuf::Message for Unit {
    fn is_initialized(&self) -> bool {
        for v in &self.pos {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.orders {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.passengers {
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
                    self.display_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.alliance = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.tag = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.unit_type = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.owner = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pos)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.facing = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.radius = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.build_progress = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.cloak = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.detect_range = ::std::option::Option::Some(tmp);
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.radar_range = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_selected = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_on_screen = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_blip = ::std::option::Option::Some(tmp);
                },
                35 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_powered = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.health = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.health_max = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.shield = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.energy = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.mineral_contents = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.vespene_contents = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_flying = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_burrowed = ::std::option::Option::Some(tmp);
                },
                22 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.orders)?;
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.add_on_tag = ::std::option::Option::Some(tmp);
                },
                24 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.passengers)?;
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.cargo_space_taken = ::std::option::Option::Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.cargo_space_max = ::std::option::Option::Some(tmp);
                },
                27 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.buff_ids)?;
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.assigned_harvesters = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.ideal_harvesters = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.weapon_cooldown = ::std::option::Option::Some(tmp);
                },
                34 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.engaged_target_tag = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.display_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.alliance {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.tag {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.unit_type {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.owner {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.facing {
            my_size += 5;
        }
        if let Some(v) = self.radius {
            my_size += 5;
        }
        if let Some(v) = self.build_progress {
            my_size += 5;
        }
        if let Some(v) = self.cloak {
            my_size += ::protobuf::rt::enum_size(10, v);
        }
        if let Some(v) = self.detect_range {
            my_size += 6;
        }
        if let Some(v) = self.radar_range {
            my_size += 6;
        }
        if let Some(v) = self.is_selected {
            my_size += 2;
        }
        if let Some(v) = self.is_on_screen {
            my_size += 2;
        }
        if let Some(v) = self.is_blip {
            my_size += 2;
        }
        if let Some(v) = self.is_powered {
            my_size += 3;
        }
        if let Some(v) = self.health {
            my_size += 5;
        }
        if let Some(v) = self.health_max {
            my_size += 5;
        }
        if let Some(v) = self.shield {
            my_size += 6;
        }
        if let Some(v) = self.energy {
            my_size += 6;
        }
        if let Some(v) = self.mineral_contents {
            my_size += ::protobuf::rt::value_size(18, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.vespene_contents {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_flying {
            my_size += 3;
        }
        if let Some(v) = self.is_burrowed {
            my_size += 3;
        }
        for value in &self.orders {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.add_on_tag {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.passengers {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.cargo_space_taken {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cargo_space_max {
            my_size += ::protobuf::rt::value_size(26, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.buff_ids {
            my_size += ::protobuf::rt::value_size(27, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.assigned_harvesters {
            my_size += ::protobuf::rt::value_size(28, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ideal_harvesters {
            my_size += ::protobuf::rt::value_size(29, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.weapon_cooldown {
            my_size += 6;
        }
        if let Some(v) = self.engaged_target_tag {
            my_size += ::protobuf::rt::value_size(34, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.display_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.alliance {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.tag {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.unit_type {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.owner {
            os.write_int32(5, v)?;
        }
        if let Some(ref v) = self.pos.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.facing {
            os.write_float(7, v)?;
        }
        if let Some(v) = self.radius {
            os.write_float(8, v)?;
        }
        if let Some(v) = self.build_progress {
            os.write_float(9, v)?;
        }
        if let Some(v) = self.cloak {
            os.write_enum(10, v.value())?;
        }
        if let Some(v) = self.detect_range {
            os.write_float(31, v)?;
        }
        if let Some(v) = self.radar_range {
            os.write_float(32, v)?;
        }
        if let Some(v) = self.is_selected {
            os.write_bool(11, v)?;
        }
        if let Some(v) = self.is_on_screen {
            os.write_bool(12, v)?;
        }
        if let Some(v) = self.is_blip {
            os.write_bool(13, v)?;
        }
        if let Some(v) = self.is_powered {
            os.write_bool(35, v)?;
        }
        if let Some(v) = self.health {
            os.write_float(14, v)?;
        }
        if let Some(v) = self.health_max {
            os.write_float(15, v)?;
        }
        if let Some(v) = self.shield {
            os.write_float(16, v)?;
        }
        if let Some(v) = self.energy {
            os.write_float(17, v)?;
        }
        if let Some(v) = self.mineral_contents {
            os.write_int32(18, v)?;
        }
        if let Some(v) = self.vespene_contents {
            os.write_int32(19, v)?;
        }
        if let Some(v) = self.is_flying {
            os.write_bool(20, v)?;
        }
        if let Some(v) = self.is_burrowed {
            os.write_bool(21, v)?;
        }
        for v in &self.orders {
            os.write_tag(22, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.add_on_tag {
            os.write_uint64(23, v)?;
        }
        for v in &self.passengers {
            os.write_tag(24, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.cargo_space_taken {
            os.write_int32(25, v)?;
        }
        if let Some(v) = self.cargo_space_max {
            os.write_int32(26, v)?;
        }
        for v in &self.buff_ids {
            os.write_uint32(27, *v)?;
        };
        if let Some(v) = self.assigned_harvesters {
            os.write_int32(28, v)?;
        }
        if let Some(v) = self.ideal_harvesters {
            os.write_int32(29, v)?;
        }
        if let Some(v) = self.weapon_cooldown {
            os.write_float(30, v)?;
        }
        if let Some(v) = self.engaged_target_tag {
            os.write_uint64(34, v)?;
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

impl ::protobuf::MessageStatic for Unit {
    fn new() -> Unit {
        Unit::new()
    }

    fn descriptor_static(_: ::std::option::Option<Unit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DisplayType>>(
                    "display_type",
                    Unit::get_display_type_for_reflect,
                    Unit::mut_display_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Alliance>>(
                    "alliance",
                    Unit::get_alliance_for_reflect,
                    Unit::mut_alliance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "tag",
                    Unit::get_tag_for_reflect,
                    Unit::mut_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unit_type",
                    Unit::get_unit_type_for_reflect,
                    Unit::mut_unit_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "owner",
                    Unit::get_owner_for_reflect,
                    Unit::mut_owner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "pos",
                    Unit::get_pos_for_reflect,
                    Unit::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "facing",
                    Unit::get_facing_for_reflect,
                    Unit::mut_facing_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "radius",
                    Unit::get_radius_for_reflect,
                    Unit::mut_radius_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "build_progress",
                    Unit::get_build_progress_for_reflect,
                    Unit::mut_build_progress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CloakState>>(
                    "cloak",
                    Unit::get_cloak_for_reflect,
                    Unit::mut_cloak_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "detect_range",
                    Unit::get_detect_range_for_reflect,
                    Unit::mut_detect_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "radar_range",
                    Unit::get_radar_range_for_reflect,
                    Unit::mut_radar_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_selected",
                    Unit::get_is_selected_for_reflect,
                    Unit::mut_is_selected_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_on_screen",
                    Unit::get_is_on_screen_for_reflect,
                    Unit::mut_is_on_screen_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_blip",
                    Unit::get_is_blip_for_reflect,
                    Unit::mut_is_blip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_powered",
                    Unit::get_is_powered_for_reflect,
                    Unit::mut_is_powered_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "health",
                    Unit::get_health_for_reflect,
                    Unit::mut_health_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "health_max",
                    Unit::get_health_max_for_reflect,
                    Unit::mut_health_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "shield",
                    Unit::get_shield_for_reflect,
                    Unit::mut_shield_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "energy",
                    Unit::get_energy_for_reflect,
                    Unit::mut_energy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "mineral_contents",
                    Unit::get_mineral_contents_for_reflect,
                    Unit::mut_mineral_contents_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "vespene_contents",
                    Unit::get_vespene_contents_for_reflect,
                    Unit::mut_vespene_contents_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_flying",
                    Unit::get_is_flying_for_reflect,
                    Unit::mut_is_flying_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_burrowed",
                    Unit::get_is_burrowed_for_reflect,
                    Unit::mut_is_burrowed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnitOrder>>(
                    "orders",
                    Unit::get_orders_for_reflect,
                    Unit::mut_orders_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "add_on_tag",
                    Unit::get_add_on_tag_for_reflect,
                    Unit::mut_add_on_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PassengerUnit>>(
                    "passengers",
                    Unit::get_passengers_for_reflect,
                    Unit::mut_passengers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cargo_space_taken",
                    Unit::get_cargo_space_taken_for_reflect,
                    Unit::mut_cargo_space_taken_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cargo_space_max",
                    Unit::get_cargo_space_max_for_reflect,
                    Unit::mut_cargo_space_max_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "buff_ids",
                    Unit::get_buff_ids_for_reflect,
                    Unit::mut_buff_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "assigned_harvesters",
                    Unit::get_assigned_harvesters_for_reflect,
                    Unit::mut_assigned_harvesters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ideal_harvesters",
                    Unit::get_ideal_harvesters_for_reflect,
                    Unit::mut_ideal_harvesters_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "weapon_cooldown",
                    Unit::get_weapon_cooldown_for_reflect,
                    Unit::mut_weapon_cooldown_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "engaged_target_tag",
                    Unit::get_engaged_target_tag_for_reflect,
                    Unit::mut_engaged_target_tag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Unit>(
                    "Unit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Unit {
    fn clear(&mut self) {
        self.clear_display_type();
        self.clear_alliance();
        self.clear_tag();
        self.clear_unit_type();
        self.clear_owner();
        self.clear_pos();
        self.clear_facing();
        self.clear_radius();
        self.clear_build_progress();
        self.clear_cloak();
        self.clear_detect_range();
        self.clear_radar_range();
        self.clear_is_selected();
        self.clear_is_on_screen();
        self.clear_is_blip();
        self.clear_is_powered();
        self.clear_health();
        self.clear_health_max();
        self.clear_shield();
        self.clear_energy();
        self.clear_mineral_contents();
        self.clear_vespene_contents();
        self.clear_is_flying();
        self.clear_is_burrowed();
        self.clear_orders();
        self.clear_add_on_tag();
        self.clear_passengers();
        self.clear_cargo_space_taken();
        self.clear_cargo_space_max();
        self.clear_buff_ids();
        self.clear_assigned_harvesters();
        self.clear_ideal_harvesters();
        self.clear_weapon_cooldown();
        self.clear_engaged_target_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Unit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MapState {
    // message fields
    visibility: ::protobuf::SingularPtrField<super::common::ImageData>,
    creep: ::protobuf::SingularPtrField<super::common::ImageData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MapState {}

impl MapState {
    pub fn new() -> MapState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MapState {
        static mut instance: ::protobuf::lazy::Lazy<MapState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MapState,
        };
        unsafe {
            instance.get(MapState::new)
        }
    }

    // optional .SC2APIProtocol.ImageData visibility = 1;

    pub fn clear_visibility(&mut self) {
        self.visibility.clear();
    }

    pub fn has_visibility(&self) -> bool {
        self.visibility.is_some()
    }

    // Param is passed by value, moved
    pub fn set_visibility(&mut self, v: super::common::ImageData) {
        self.visibility = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_visibility(&mut self) -> &mut super::common::ImageData {
        if self.visibility.is_none() {
            self.visibility.set_default();
        }
        self.visibility.as_mut().unwrap()
    }

    // Take field
    pub fn take_visibility(&mut self) -> super::common::ImageData {
        self.visibility.take().unwrap_or_else(|| super::common::ImageData::new())
    }

    pub fn get_visibility(&self) -> &super::common::ImageData {
        self.visibility.as_ref().unwrap_or_else(|| super::common::ImageData::default_instance())
    }

    fn get_visibility_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::ImageData> {
        &self.visibility
    }

    fn mut_visibility_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::ImageData> {
        &mut self.visibility
    }

    // optional .SC2APIProtocol.ImageData creep = 2;

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
}

impl ::protobuf::Message for MapState {
    fn is_initialized(&self) -> bool {
        for v in &self.visibility {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.creep {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.visibility)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.creep)?;
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
        if let Some(ref v) = self.visibility.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.creep.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.visibility.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.creep.as_ref() {
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

impl ::protobuf::MessageStatic for MapState {
    fn new() -> MapState {
        MapState::new()
    }

    fn descriptor_static(_: ::std::option::Option<MapState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "visibility",
                    MapState::get_visibility_for_reflect,
                    MapState::mut_visibility_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::ImageData>>(
                    "creep",
                    MapState::get_creep_for_reflect,
                    MapState::mut_creep_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MapState>(
                    "MapState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MapState {
    fn clear(&mut self) {
        self.clear_visibility();
        self.clear_creep();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MapState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MapState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionRaw {
    // message oneof groups
    action: ::std::option::Option<ActionRaw_oneof_action>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionRaw {}

#[derive(Clone,PartialEq)]
pub enum ActionRaw_oneof_action {
    unit_command(ActionRawUnitCommand),
    camera_move(ActionRawCameraMove),
    toggle_autocast(ActionRawToggleAutocast),
}

impl ActionRaw {
    pub fn new() -> ActionRaw {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionRaw {
        static mut instance: ::protobuf::lazy::Lazy<ActionRaw> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionRaw,
        };
        unsafe {
            instance.get(ActionRaw::new)
        }
    }

    // optional .SC2APIProtocol.ActionRawUnitCommand unit_command = 1;

    pub fn clear_unit_command(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_unit_command(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionRaw_oneof_action::unit_command(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unit_command(&mut self, v: ActionRawUnitCommand) {
        self.action = ::std::option::Option::Some(ActionRaw_oneof_action::unit_command(v))
    }

    // Mutable pointer to the field.
    pub fn mut_unit_command(&mut self) -> &mut ActionRawUnitCommand {
        if let ::std::option::Option::Some(ActionRaw_oneof_action::unit_command(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionRaw_oneof_action::unit_command(ActionRawUnitCommand::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionRaw_oneof_action::unit_command(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_unit_command(&mut self) -> ActionRawUnitCommand {
        if self.has_unit_command() {
            match self.action.take() {
                ::std::option::Option::Some(ActionRaw_oneof_action::unit_command(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionRawUnitCommand::new()
        }
    }

    pub fn get_unit_command(&self) -> &ActionRawUnitCommand {
        match self.action {
            ::std::option::Option::Some(ActionRaw_oneof_action::unit_command(ref v)) => v,
            _ => ActionRawUnitCommand::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionRawCameraMove camera_move = 2;

    pub fn clear_camera_move(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_camera_move(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionRaw_oneof_action::camera_move(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_camera_move(&mut self, v: ActionRawCameraMove) {
        self.action = ::std::option::Option::Some(ActionRaw_oneof_action::camera_move(v))
    }

    // Mutable pointer to the field.
    pub fn mut_camera_move(&mut self) -> &mut ActionRawCameraMove {
        if let ::std::option::Option::Some(ActionRaw_oneof_action::camera_move(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionRaw_oneof_action::camera_move(ActionRawCameraMove::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionRaw_oneof_action::camera_move(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_camera_move(&mut self) -> ActionRawCameraMove {
        if self.has_camera_move() {
            match self.action.take() {
                ::std::option::Option::Some(ActionRaw_oneof_action::camera_move(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionRawCameraMove::new()
        }
    }

    pub fn get_camera_move(&self) -> &ActionRawCameraMove {
        match self.action {
            ::std::option::Option::Some(ActionRaw_oneof_action::camera_move(ref v)) => v,
            _ => ActionRawCameraMove::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionRawToggleAutocast toggle_autocast = 3;

    pub fn clear_toggle_autocast(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_toggle_autocast(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionRaw_oneof_action::toggle_autocast(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_toggle_autocast(&mut self, v: ActionRawToggleAutocast) {
        self.action = ::std::option::Option::Some(ActionRaw_oneof_action::toggle_autocast(v))
    }

    // Mutable pointer to the field.
    pub fn mut_toggle_autocast(&mut self) -> &mut ActionRawToggleAutocast {
        if let ::std::option::Option::Some(ActionRaw_oneof_action::toggle_autocast(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionRaw_oneof_action::toggle_autocast(ActionRawToggleAutocast::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionRaw_oneof_action::toggle_autocast(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_toggle_autocast(&mut self) -> ActionRawToggleAutocast {
        if self.has_toggle_autocast() {
            match self.action.take() {
                ::std::option::Option::Some(ActionRaw_oneof_action::toggle_autocast(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionRawToggleAutocast::new()
        }
    }

    pub fn get_toggle_autocast(&self) -> &ActionRawToggleAutocast {
        match self.action {
            ::std::option::Option::Some(ActionRaw_oneof_action::toggle_autocast(ref v)) => v,
            _ => ActionRawToggleAutocast::default_instance(),
        }
    }
}

impl ::protobuf::Message for ActionRaw {
    fn is_initialized(&self) -> bool {
        if let Some(ActionRaw_oneof_action::unit_command(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionRaw_oneof_action::camera_move(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionRaw_oneof_action::toggle_autocast(ref v)) = self.action {
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
                    self.action = ::std::option::Option::Some(ActionRaw_oneof_action::unit_command(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionRaw_oneof_action::camera_move(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionRaw_oneof_action::toggle_autocast(is.read_message()?));
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
                &ActionRaw_oneof_action::unit_command(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionRaw_oneof_action::camera_move(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionRaw_oneof_action::toggle_autocast(ref v) => {
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
                &ActionRaw_oneof_action::unit_command(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionRaw_oneof_action::camera_move(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionRaw_oneof_action::toggle_autocast(ref v) => {
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

impl ::protobuf::MessageStatic for ActionRaw {
    fn new() -> ActionRaw {
        ActionRaw::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionRaw>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionRawUnitCommand>(
                    "unit_command",
                    ActionRaw::has_unit_command,
                    ActionRaw::get_unit_command,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionRawCameraMove>(
                    "camera_move",
                    ActionRaw::has_camera_move,
                    ActionRaw::get_camera_move,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionRawToggleAutocast>(
                    "toggle_autocast",
                    ActionRaw::has_toggle_autocast,
                    ActionRaw::get_toggle_autocast,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionRaw>(
                    "ActionRaw",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionRaw {
    fn clear(&mut self) {
        self.clear_unit_command();
        self.clear_camera_move();
        self.clear_toggle_autocast();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionRaw {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionRaw {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionRawUnitCommand {
    // message fields
    ability_id: ::std::option::Option<i32>,
    unit_tags: ::std::vec::Vec<u64>,
    queue_command: ::std::option::Option<bool>,
    // message oneof groups
    target: ::std::option::Option<ActionRawUnitCommand_oneof_target>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionRawUnitCommand {}

#[derive(Clone,PartialEq)]
pub enum ActionRawUnitCommand_oneof_target {
    target_world_space_pos(super::common::Point2D),
    target_unit_tag(u64),
}

impl ActionRawUnitCommand {
    pub fn new() -> ActionRawUnitCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionRawUnitCommand {
        static mut instance: ::protobuf::lazy::Lazy<ActionRawUnitCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionRawUnitCommand,
        };
        unsafe {
            instance.get(ActionRawUnitCommand::new)
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

    // optional .SC2APIProtocol.Point2D target_world_space_pos = 2;

    pub fn clear_target_world_space_pos(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target_world_space_pos(&self) -> bool {
        match self.target {
            ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_world_space_pos(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_target_world_space_pos(&mut self, v: super::common::Point2D) {
        self.target = ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_world_space_pos(v))
    }

    // Mutable pointer to the field.
    pub fn mut_target_world_space_pos(&mut self) -> &mut super::common::Point2D {
        if let ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_world_space_pos(_)) = self.target {
        } else {
            self.target = ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_world_space_pos(super::common::Point2D::new()));
        }
        match self.target {
            ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_world_space_pos(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_target_world_space_pos(&mut self) -> super::common::Point2D {
        if self.has_target_world_space_pos() {
            match self.target.take() {
                ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_world_space_pos(v)) => v,
                _ => panic!(),
            }
        } else {
            super::common::Point2D::new()
        }
    }

    pub fn get_target_world_space_pos(&self) -> &super::common::Point2D {
        match self.target {
            ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_world_space_pos(ref v)) => v,
            _ => super::common::Point2D::default_instance(),
        }
    }

    // optional uint64 target_unit_tag = 3;

    pub fn clear_target_unit_tag(&mut self) {
        self.target = ::std::option::Option::None;
    }

    pub fn has_target_unit_tag(&self) -> bool {
        match self.target {
            ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_unit_tag(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_target_unit_tag(&mut self, v: u64) {
        self.target = ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_unit_tag(v))
    }

    pub fn get_target_unit_tag(&self) -> u64 {
        match self.target {
            ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_unit_tag(v)) => v,
            _ => 0,
        }
    }

    // repeated uint64 unit_tags = 4;

    pub fn clear_unit_tags(&mut self) {
        self.unit_tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_unit_tags(&mut self, v: ::std::vec::Vec<u64>) {
        self.unit_tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unit_tags(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.unit_tags
    }

    // Take field
    pub fn take_unit_tags(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.unit_tags, ::std::vec::Vec::new())
    }

    pub fn get_unit_tags(&self) -> &[u64] {
        &self.unit_tags
    }

    fn get_unit_tags_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.unit_tags
    }

    fn mut_unit_tags_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.unit_tags
    }

    // optional bool queue_command = 5;

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

impl ::protobuf::Message for ActionRawUnitCommand {
    fn is_initialized(&self) -> bool {
        if let Some(ActionRawUnitCommand_oneof_target::target_world_space_pos(ref v)) = self.target {
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
                    self.target = ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_world_space_pos(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.target = ::std::option::Option::Some(ActionRawUnitCommand_oneof_target::target_unit_tag(is.read_uint64()?));
                },
                4 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.unit_tags)?;
                },
                5 => {
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
        for value in &self.unit_tags {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.queue_command {
            my_size += 2;
        }
        if let ::std::option::Option::Some(ref v) = self.target {
            match v {
                &ActionRawUnitCommand_oneof_target::target_world_space_pos(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionRawUnitCommand_oneof_target::target_unit_tag(v) => {
                    my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        for v in &self.unit_tags {
            os.write_uint64(4, *v)?;
        };
        if let Some(v) = self.queue_command {
            os.write_bool(5, v)?;
        }
        if let ::std::option::Option::Some(ref v) = self.target {
            match v {
                &ActionRawUnitCommand_oneof_target::target_world_space_pos(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionRawUnitCommand_oneof_target::target_unit_tag(v) => {
                    os.write_uint64(3, v)?;
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

impl ::protobuf::MessageStatic for ActionRawUnitCommand {
    fn new() -> ActionRawUnitCommand {
        ActionRawUnitCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionRawUnitCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ability_id",
                    ActionRawUnitCommand::get_ability_id_for_reflect,
                    ActionRawUnitCommand::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::common::Point2D>(
                    "target_world_space_pos",
                    ActionRawUnitCommand::has_target_world_space_pos,
                    ActionRawUnitCommand::get_target_world_space_pos,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor::<_>(
                    "target_unit_tag",
                    ActionRawUnitCommand::has_target_unit_tag,
                    ActionRawUnitCommand::get_target_unit_tag,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "unit_tags",
                    ActionRawUnitCommand::get_unit_tags_for_reflect,
                    ActionRawUnitCommand::mut_unit_tags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "queue_command",
                    ActionRawUnitCommand::get_queue_command_for_reflect,
                    ActionRawUnitCommand::mut_queue_command_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionRawUnitCommand>(
                    "ActionRawUnitCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionRawUnitCommand {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.clear_target_world_space_pos();
        self.clear_target_unit_tag();
        self.clear_unit_tags();
        self.clear_queue_command();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionRawUnitCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionRawUnitCommand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionRawCameraMove {
    // message fields
    center_world_space: ::protobuf::SingularPtrField<super::common::Point>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionRawCameraMove {}

impl ActionRawCameraMove {
    pub fn new() -> ActionRawCameraMove {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionRawCameraMove {
        static mut instance: ::protobuf::lazy::Lazy<ActionRawCameraMove> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionRawCameraMove,
        };
        unsafe {
            instance.get(ActionRawCameraMove::new)
        }
    }

    // optional .SC2APIProtocol.Point center_world_space = 1;

    pub fn clear_center_world_space(&mut self) {
        self.center_world_space.clear();
    }

    pub fn has_center_world_space(&self) -> bool {
        self.center_world_space.is_some()
    }

    // Param is passed by value, moved
    pub fn set_center_world_space(&mut self, v: super::common::Point) {
        self.center_world_space = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_center_world_space(&mut self) -> &mut super::common::Point {
        if self.center_world_space.is_none() {
            self.center_world_space.set_default();
        }
        self.center_world_space.as_mut().unwrap()
    }

    // Take field
    pub fn take_center_world_space(&mut self) -> super::common::Point {
        self.center_world_space.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_center_world_space(&self) -> &super::common::Point {
        self.center_world_space.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_center_world_space_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.center_world_space
    }

    fn mut_center_world_space_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.center_world_space
    }
}

impl ::protobuf::Message for ActionRawCameraMove {
    fn is_initialized(&self) -> bool {
        for v in &self.center_world_space {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.center_world_space)?;
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
        if let Some(ref v) = self.center_world_space.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.center_world_space.as_ref() {
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

impl ::protobuf::MessageStatic for ActionRawCameraMove {
    fn new() -> ActionRawCameraMove {
        ActionRawCameraMove::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionRawCameraMove>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "center_world_space",
                    ActionRawCameraMove::get_center_world_space_for_reflect,
                    ActionRawCameraMove::mut_center_world_space_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionRawCameraMove>(
                    "ActionRawCameraMove",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionRawCameraMove {
    fn clear(&mut self) {
        self.clear_center_world_space();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionRawCameraMove {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionRawCameraMove {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionRawToggleAutocast {
    // message fields
    ability_id: ::std::option::Option<i32>,
    unit_tags: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionRawToggleAutocast {}

impl ActionRawToggleAutocast {
    pub fn new() -> ActionRawToggleAutocast {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionRawToggleAutocast {
        static mut instance: ::protobuf::lazy::Lazy<ActionRawToggleAutocast> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionRawToggleAutocast,
        };
        unsafe {
            instance.get(ActionRawToggleAutocast::new)
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

    // repeated uint64 unit_tags = 2;

    pub fn clear_unit_tags(&mut self) {
        self.unit_tags.clear();
    }

    // Param is passed by value, moved
    pub fn set_unit_tags(&mut self, v: ::std::vec::Vec<u64>) {
        self.unit_tags = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unit_tags(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.unit_tags
    }

    // Take field
    pub fn take_unit_tags(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.unit_tags, ::std::vec::Vec::new())
    }

    pub fn get_unit_tags(&self) -> &[u64] {
        &self.unit_tags
    }

    fn get_unit_tags_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.unit_tags
    }

    fn mut_unit_tags_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.unit_tags
    }
}

impl ::protobuf::Message for ActionRawToggleAutocast {
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
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.unit_tags)?;
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
        for value in &self.unit_tags {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_id {
            os.write_int32(1, v)?;
        }
        for v in &self.unit_tags {
            os.write_uint64(2, *v)?;
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

impl ::protobuf::MessageStatic for ActionRawToggleAutocast {
    fn new() -> ActionRawToggleAutocast {
        ActionRawToggleAutocast::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionRawToggleAutocast>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ability_id",
                    ActionRawToggleAutocast::get_ability_id_for_reflect,
                    ActionRawToggleAutocast::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "unit_tags",
                    ActionRawToggleAutocast::get_unit_tags_for_reflect,
                    ActionRawToggleAutocast::mut_unit_tags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionRawToggleAutocast>(
                    "ActionRawToggleAutocast",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionRawToggleAutocast {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.clear_unit_tags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionRawToggleAutocast {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionRawToggleAutocast {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Event {
    // message fields
    dead_units: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Event {}

impl Event {
    pub fn new() -> Event {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Event {
        static mut instance: ::protobuf::lazy::Lazy<Event> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Event,
        };
        unsafe {
            instance.get(Event::new)
        }
    }

    // repeated uint64 dead_units = 1;

    pub fn clear_dead_units(&mut self) {
        self.dead_units.clear();
    }

    // Param is passed by value, moved
    pub fn set_dead_units(&mut self, v: ::std::vec::Vec<u64>) {
        self.dead_units = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dead_units(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.dead_units
    }

    // Take field
    pub fn take_dead_units(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.dead_units, ::std::vec::Vec::new())
    }

    pub fn get_dead_units(&self) -> &[u64] {
        &self.dead_units
    }

    fn get_dead_units_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.dead_units
    }

    fn mut_dead_units_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.dead_units
    }
}

impl ::protobuf::Message for Event {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.dead_units)?;
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
        for value in &self.dead_units {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.dead_units {
            os.write_uint64(1, *v)?;
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

impl ::protobuf::MessageStatic for Event {
    fn new() -> Event {
        Event::new()
    }

    fn descriptor_static(_: ::std::option::Option<Event>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "dead_units",
                    Event::get_dead_units_for_reflect,
                    Event::mut_dead_units_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Event>(
                    "Event",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Event {
    fn clear(&mut self) {
        self.clear_dead_units();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Event {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DisplayType {
    Visible = 1,
    Snapshot = 2,
    Hidden = 3,
}

impl ::protobuf::ProtobufEnum for DisplayType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DisplayType> {
        match value {
            1 => ::std::option::Option::Some(DisplayType::Visible),
            2 => ::std::option::Option::Some(DisplayType::Snapshot),
            3 => ::std::option::Option::Some(DisplayType::Hidden),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DisplayType] = &[
            DisplayType::Visible,
            DisplayType::Snapshot,
            DisplayType::Hidden,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DisplayType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DisplayType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DisplayType {
}

impl ::protobuf::reflect::ProtobufValue for DisplayType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Alliance {
    Self = 1,
    Ally = 2,
    Neutral = 3,
    Enemy = 4,
}

impl ::protobuf::ProtobufEnum for Alliance {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Alliance> {
        match value {
            1 => ::std::option::Option::Some(Alliance::Self),
            2 => ::std::option::Option::Some(Alliance::Ally),
            3 => ::std::option::Option::Some(Alliance::Neutral),
            4 => ::std::option::Option::Some(Alliance::Enemy),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Alliance] = &[
            Alliance::Self,
            Alliance::Ally,
            Alliance::Neutral,
            Alliance::Enemy,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Alliance>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Alliance", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Alliance {
}

impl ::protobuf::reflect::ProtobufValue for Alliance {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CloakState {
    Cloaked = 1,
    CloakedDetected = 2,
    NotCloaked = 3,
}

impl ::protobuf::ProtobufEnum for CloakState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CloakState> {
        match value {
            1 => ::std::option::Option::Some(CloakState::Cloaked),
            2 => ::std::option::Option::Some(CloakState::CloakedDetected),
            3 => ::std::option::Option::Some(CloakState::NotCloaked),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CloakState] = &[
            CloakState::Cloaked,
            CloakState::CloakedDetected,
            CloakState::NotCloaked,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CloakState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CloakState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CloakState {
}

impl ::protobuf::reflect::ProtobufValue for CloakState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1as2clientprotocol/raw.proto\x12\x0eSC2APIProtocol\x1a\x1ds2clientpr\
    otocol/common.proto\"\x83\x03\n\x08StartRaw\x122\n\x08map_size\x18\x01\
    \x20\x01(\x0b2\x17.SC2APIProtocol.Size2DIR\x07mapSize\x12<\n\x0cpathing_\
    grid\x18\x02\x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\x0bpathingGrid\
    \x12@\n\x0eterrain_height\x18\x03\x20\x01(\x0b2\x19.SC2APIProtocol.Image\
    DataR\rterrainHeight\x12@\n\x0eplacement_grid\x18\x04\x20\x01(\x0b2\x19.\
    SC2APIProtocol.ImageDataR\rplacementGrid\x12?\n\rplayable_area\x18\x05\
    \x20\x01(\x0b2\x1a.SC2APIProtocol.RectangleIR\x0cplayableArea\x12@\n\x0f\
    start_locations\x18\x06\x20\x03(\x0b2\x17.SC2APIProtocol.Point2DR\x0esta\
    rtLocations\"\xd3\x01\n\x0eObservationRaw\x121\n\x06player\x18\x01\x20\
    \x01(\x0b2\x19.SC2APIProtocol.PlayerRawR\x06player\x12*\n\x05units\x18\
    \x02\x20\x03(\x0b2\x14.SC2APIProtocol.UnitR\x05units\x125\n\tmap_state\
    \x18\x03\x20\x01(\x0b2\x18.SC2APIProtocol.MapStateR\x08mapState\x12+\n\
    \x05event\x18\x04\x20\x01(\x0b2\x15.SC2APIProtocol.EventR\x05event\"`\n\
    \x0bPowerSource\x12'\n\x03pos\x18\x01\x20\x01(\x0b2\x15.SC2APIProtocol.P\
    ointR\x03pos\x12\x16\n\x06radius\x18\x02\x20\x01(\x02R\x06radius\x12\x10\
    \n\x03tag\x18\x03\x20\x01(\x04R\x03tag\"\x9d\x01\n\tPlayerRaw\x12@\n\rpo\
    wer_sources\x18\x01\x20\x03(\x0b2\x1b.SC2APIProtocol.PowerSourceR\x0cpow\
    erSources\x12-\n\x06camera\x18\x02\x20\x01(\x0b2\x15.SC2APIProtocol.Poin\
    tR\x06camera\x12\x1f\n\x0bupgrade_ids\x18\x03\x20\x03(\rR\nupgradeIds\"\
    \xc8\x01\n\tUnitOrder\x12\x1d\n\nability_id\x18\x01\x20\x01(\rR\tability\
    Id\x12L\n\x16target_world_space_pos\x18\x02\x20\x01(\x0b2\x15.SC2APIProt\
    ocol.PointH\0R\x13targetWorldSpacePos\x12(\n\x0ftarget_unit_tag\x18\x03\
    \x20\x01(\x04H\0R\rtargetUnitTag\x12\x1a\n\x08progress\x18\x04\x20\x01(\
    \x02R\x08progressB\x08\n\x06target\"\xa5\x01\n\rPassengerUnit\x12\x10\n\
    \x03tag\x18\x01\x20\x01(\x04R\x03tag\x12\x16\n\x06health\x18\x02\x20\x01\
    (\x02R\x06health\x12\x1d\n\nhealth_max\x18\x03\x20\x01(\x02R\thealthMax\
    \x12\x16\n\x06shield\x18\x04\x20\x01(\x02R\x06shield\x12\x16\n\x06energy\
    \x18\x05\x20\x01(\x02R\x06energy\x12\x1b\n\tunit_type\x18\x06\x20\x01(\r\
    R\x08unitType\"\xdf\t\n\x04Unit\x12>\n\x0cdisplay_type\x18\x01\x20\x01(\
    \x0e2\x1b.SC2APIProtocol.DisplayTypeR\x0bdisplayType\x124\n\x08alliance\
    \x18\x02\x20\x01(\x0e2\x18.SC2APIProtocol.AllianceR\x08alliance\x12\x10\
    \n\x03tag\x18\x03\x20\x01(\x04R\x03tag\x12\x1b\n\tunit_type\x18\x04\x20\
    \x01(\rR\x08unitType\x12\x14\n\x05owner\x18\x05\x20\x01(\x05R\x05owner\
    \x12'\n\x03pos\x18\x06\x20\x01(\x0b2\x15.SC2APIProtocol.PointR\x03pos\
    \x12\x16\n\x06facing\x18\x07\x20\x01(\x02R\x06facing\x12\x16\n\x06radius\
    \x18\x08\x20\x01(\x02R\x06radius\x12%\n\x0ebuild_progress\x18\t\x20\x01(\
    \x02R\rbuildProgress\x120\n\x05cloak\x18\n\x20\x01(\x0e2\x1a.SC2APIProto\
    col.CloakStateR\x05cloak\x12!\n\x0cdetect_range\x18\x1f\x20\x01(\x02R\
    \x0bdetectRange\x12\x1f\n\x0bradar_range\x18\x20\x20\x01(\x02R\nradarRan\
    ge\x12\x1f\n\x0bis_selected\x18\x0b\x20\x01(\x08R\nisSelected\x12\x20\n\
    \x0cis_on_screen\x18\x0c\x20\x01(\x08R\nisOnScreen\x12\x17\n\x07is_blip\
    \x18\r\x20\x01(\x08R\x06isBlip\x12\x1d\n\nis_powered\x18#\x20\x01(\x08R\
    \tisPowered\x12\x16\n\x06health\x18\x0e\x20\x01(\x02R\x06health\x12\x1d\
    \n\nhealth_max\x18\x0f\x20\x01(\x02R\thealthMax\x12\x16\n\x06shield\x18\
    \x10\x20\x01(\x02R\x06shield\x12\x16\n\x06energy\x18\x11\x20\x01(\x02R\
    \x06energy\x12)\n\x10mineral_contents\x18\x12\x20\x01(\x05R\x0fmineralCo\
    ntents\x12)\n\x10vespene_contents\x18\x13\x20\x01(\x05R\x0fvespeneConten\
    ts\x12\x1b\n\tis_flying\x18\x14\x20\x01(\x08R\x08isFlying\x12\x1f\n\x0bi\
    s_burrowed\x18\x15\x20\x01(\x08R\nisBurrowed\x121\n\x06orders\x18\x16\
    \x20\x03(\x0b2\x19.SC2APIProtocol.UnitOrderR\x06orders\x12\x1c\n\nadd_on\
    _tag\x18\x17\x20\x01(\x04R\x08addOnTag\x12=\n\npassengers\x18\x18\x20\
    \x03(\x0b2\x1d.SC2APIProtocol.PassengerUnitR\npassengers\x12*\n\x11cargo\
    _space_taken\x18\x19\x20\x01(\x05R\x0fcargoSpaceTaken\x12&\n\x0fcargo_sp\
    ace_max\x18\x1a\x20\x01(\x05R\rcargoSpaceMax\x12\x19\n\x08buff_ids\x18\
    \x1b\x20\x03(\rR\x07buffIds\x12/\n\x13assigned_harvesters\x18\x1c\x20\
    \x01(\x05R\x12assignedHarvesters\x12)\n\x10ideal_harvesters\x18\x1d\x20\
    \x01(\x05R\x0fidealHarvesters\x12'\n\x0fweapon_cooldown\x18\x1e\x20\x01(\
    \x02R\x0eweaponCooldown\x12,\n\x12engaged_target_tag\x18\"\x20\x01(\x04R\
    \x10engagedTargetTag\"v\n\x08MapState\x129\n\nvisibility\x18\x01\x20\x01\
    (\x0b2\x19.SC2APIProtocol.ImageDataR\nvisibility\x12/\n\x05creep\x18\x02\
    \x20\x01(\x0b2\x19.SC2APIProtocol.ImageDataR\x05creep\"\xfc\x01\n\tActio\
    nRaw\x12I\n\x0cunit_command\x18\x01\x20\x01(\x0b2$.SC2APIProtocol.Action\
    RawUnitCommandH\0R\x0bunitCommand\x12F\n\x0bcamera_move\x18\x02\x20\x01(\
    \x0b2#.SC2APIProtocol.ActionRawCameraMoveH\0R\ncameraMove\x12R\n\x0ftogg\
    le_autocast\x18\x03\x20\x01(\x0b2'.SC2APIProtocol.ActionRawToggleAutocas\
    tH\0R\x0etoggleAutocastB\x08\n\x06action\"\xfb\x01\n\x14ActionRawUnitCom\
    mand\x12\x1d\n\nability_id\x18\x01\x20\x01(\x05R\tabilityId\x12N\n\x16ta\
    rget_world_space_pos\x18\x02\x20\x01(\x0b2\x17.SC2APIProtocol.Point2DH\0\
    R\x13targetWorldSpacePos\x12(\n\x0ftarget_unit_tag\x18\x03\x20\x01(\x04H\
    \0R\rtargetUnitTag\x12\x1b\n\tunit_tags\x18\x04\x20\x03(\x04R\x08unitTag\
    s\x12#\n\rqueue_command\x18\x05\x20\x01(\x08R\x0cqueueCommandB\x08\n\x06\
    target\"Z\n\x13ActionRawCameraMove\x12C\n\x12center_world_space\x18\x01\
    \x20\x01(\x0b2\x15.SC2APIProtocol.PointR\x10centerWorldSpace\"U\n\x17Act\
    ionRawToggleAutocast\x12\x1d\n\nability_id\x18\x01\x20\x01(\x05R\tabilit\
    yId\x12\x1b\n\tunit_tags\x18\x02\x20\x03(\x04R\x08unitTags\"&\n\x05Event\
    \x12\x1d\n\ndead_units\x18\x01\x20\x03(\x04R\tdeadUnits*4\n\x0bDisplayTy\
    pe\x12\x0b\n\x07Visible\x10\x01\x12\x0c\n\x08Snapshot\x10\x02\x12\n\n\
    \x06Hidden\x10\x03*6\n\x08Alliance\x12\x08\n\x04Self\x10\x01\x12\x08\n\
    \x04Ally\x10\x02\x12\x0b\n\x07Neutral\x10\x03\x12\t\n\x05Enemy\x10\x04*>\
    \n\nCloakState\x12\x0b\n\x07Cloaked\x10\x01\x12\x13\n\x0fCloakedDetected\
    \x10\x02\x12\x0e\n\nNotCloaked\x10\x03J\xc45\n\x07\x12\x05\x01\0\xa6\x01\
    \x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\x08\n\x01\x02\x12\x03\x03\x08\
    \x16\n\t\n\x02\x03\0\x12\x03\x05\x07&\n\x15\n\x02\x04\0\x12\x04\x0b\0\
    \x12\x012\t\n\x20Start\n\n\n\n\n\x03\x04\0\x01\x12\x03\x0b\x08\x10\n+\n\
    \x04\x04\0\x02\0\x12\x03\x0c\x02\x20\"\x1e\x20Width\x20and\x20height\x20\
    of\x20the\x20map.\n\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x0c\x02\n\n\x0c\
    \n\x05\x04\0\x02\0\x06\x12\x03\x0c\x0b\x12\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\x0c\x13\x1b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x0c\x1e\x1f\n1\
    \n\x04\x04\0\x02\x01\x12\x03\r\x02&\"$\x201\x20byte\x20bitmap\x20of\x20t\
    he\x20pathing\x20grid.\n\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\r\x02\n\n\
    \x0c\n\x05\x04\0\x02\x01\x06\x12\x03\r\x0b\x14\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\r\x15!\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\r$%\n3\n\x04\
    \x04\0\x02\x02\x12\x03\x0e\x02(\"&\x201\x20byte\x20bitmap\x20of\x20the\
    \x20terrain\x20height.\n\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\x0e\x02\n\
    \n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x0e\x0b\x14\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x0e\x15#\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0e&'\n<\
    \n\x04\x04\0\x02\x03\x12\x03\x0f\x02(\"/\x201\x20byte\x20bitmap\x20of\
    \x20the\x20building\x20placement\x20grid.\n\n\x0c\n\x05\x04\0\x02\x03\
    \x04\x12\x03\x0f\x02\n\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x0f\x0b\x14\
    \n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x0f\x15#\n\x0c\n\x05\x04\0\x02\
    \x03\x03\x12\x03\x0f&'\n\"\n\x04\x04\0\x02\x04\x12\x03\x10\x02(\"\x15\
    \x20The\x20playable\x20cells.\n\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\
    \x10\x02\n\n\x0c\n\x05\x04\0\x02\x04\x06\x12\x03\x10\x0b\x15\n\x0c\n\x05\
    \x04\0\x02\x04\x01\x12\x03\x10\x16#\n\x0c\n\x05\x04\0\x02\x04\x03\x12\
    \x03\x10&'\n4\n\x04\x04\0\x02\x05\x12\x03\x11\x02'\"'\x20Possible\x20sta\
    rt\x20locations\x20for\x20players.\n\n\x0c\n\x05\x04\0\x02\x05\x04\x12\
    \x03\x11\x02\n\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x03\x11\x0b\x12\n\x0c\n\
    \x05\x04\0\x02\x05\x01\x12\x03\x11\x13\"\n\x0c\n\x05\x04\0\x02\x05\x03\
    \x12\x03\x11%&\n\x1b\n\x02\x04\x01\x12\x04\x19\0\x1e\x012\x0f\n\x20Obser\
    vation\n\n\n\n\n\x03\x04\x01\x01\x12\x03\x19\x08\x16\n\x0b\n\x04\x04\x01\
    \x02\0\x12\x03\x1a\x02\x20\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x1a\x02\
    \n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x1a\x0b\x14\n\x0c\n\x05\x04\x01\
    \x02\0\x01\x12\x03\x1a\x15\x1b\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x1a\
    \x1e\x1f\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x1b\x02\x1a\n\x0c\n\x05\x04\
    \x01\x02\x01\x04\x12\x03\x1b\x02\n\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\
    \x03\x1b\x0b\x0f\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x1b\x10\x15\n\
    \x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1b\x18\x19\nO\n\x04\x04\x01\x02\
    \x02\x12\x03\x1c\x02\"\"B\x20Fog\x20of\x20war,\x20creep\x20and\x20so\x20\
    on.\x20Board\x20stuff\x20that\x20changes\x20per\x20frame.\n\n\x0c\n\x05\
    \x04\x01\x02\x02\x04\x12\x03\x1c\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x06\
    \x12\x03\x1c\x0b\x13\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x1c\x14\x1d\
    \n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\x1c\x20!\n\x0b\n\x04\x04\x01\
    \x02\x03\x12\x03\x1d\x02\x1b\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03\x1d\
    \x02\n\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03\x1d\x0b\x10\n\x0c\n\x05\
    \x04\x01\x02\x03\x01\x12\x03\x1d\x11\x16\n\x0c\n\x05\x04\x01\x02\x03\x03\
    \x12\x03\x1d\x19\x1a\n\n\n\x02\x04\x02\x12\x04\x20\0$\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03\x20\x08\x13\n\x0b\n\x04\x04\x02\x02\0\x12\x03!\x02\x19\
    \n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03!\x02\n\n\x0c\n\x05\x04\x02\x02\0\
    \x06\x12\x03!\x0b\x10\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03!\x11\x14\n\
    \x0c\n\x05\x04\x02\x02\0\x03\x12\x03!\x17\x18\n\x0b\n\x04\x04\x02\x02\
    \x01\x12\x03\"\x02\x1c\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\"\x02\n\n\
    \x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\"\x0b\x10\n\x0c\n\x05\x04\x02\x02\
    \x01\x01\x12\x03\"\x11\x17\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\"\x1a\
    \x1b\n\x0b\n\x04\x04\x02\x02\x02\x12\x03#\x02\x1a\n\x0c\n\x05\x04\x02\
    \x02\x02\x04\x12\x03#\x02\n\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03#\x0b\
    \x11\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03#\x12\x15\n\x0c\n\x05\x04\
    \x02\x02\x02\x03\x12\x03#\x18\x19\n\n\n\x02\x04\x03\x12\x04&\0*\x01\n\n\
    \n\x03\x04\x03\x01\x12\x03&\x08\x11\n\x0b\n\x04\x04\x03\x02\0\x12\x03'\
    \x02)\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03'\x02\n\n\x0c\n\x05\x04\x03\
    \x02\0\x06\x12\x03'\x0b\x16\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03'\x17$\
    \n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03''(\n\x0b\n\x04\x04\x03\x02\x01\
    \x12\x03(\x02\x1c\n\x0c\n\x05\x04\x03\x02\x01\x04\x12\x03(\x02\n\n\x0c\n\
    \x05\x04\x03\x02\x01\x06\x12\x03(\x0b\x10\n\x0c\n\x05\x04\x03\x02\x01\
    \x01\x12\x03(\x11\x17\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03(\x1a\x1b\n\
    +\n\x04\x04\x03\x02\x02\x12\x03)\x02\"\"\x1e\x20TODO:\x20Add\x20to\x20UI\
    \x20observation?\n\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03)\x02\n\n\x0c\
    \n\x05\x04\x03\x02\x02\x05\x12\x03)\x0b\x11\n\x0c\n\x05\x04\x03\x02\x02\
    \x01\x12\x03)\x12\x1d\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03)\x20!\n\n\
    \n\x02\x04\x04\x12\x04,\03\x01\n\n\n\x03\x04\x04\x01\x12\x03,\x08\x11\n\
    \x0b\n\x04\x04\x04\x02\0\x12\x03-\x02!\n\x0c\n\x05\x04\x04\x02\0\x04\x12\
    \x03-\x02\n\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03-\x0b\x11\n\x0c\n\x05\
    \x04\x04\x02\0\x01\x12\x03-\x12\x1c\n\x0c\n\x05\x04\x04\x02\0\x03\x12\
    \x03-\x1f\x20\n\x0c\n\x04\x04\x04\x08\0\x12\x04.\x021\x03\n\x0c\n\x05\
    \x04\x04\x08\0\x01\x12\x03.\x08\x0e\n\x0b\n\x04\x04\x04\x02\x01\x12\x03/\
    \x04%\n\x0c\n\x05\x04\x04\x02\x01\x06\x12\x03/\x04\t\n\x0c\n\x05\x04\x04\
    \x02\x01\x01\x12\x03/\n\x20\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03/#$\n\
    \x0b\n\x04\x04\x04\x02\x02\x12\x030\x04\x1f\n\x0c\n\x05\x04\x04\x02\x02\
    \x05\x12\x030\x04\n\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x030\x0b\x1a\n\
    \x0c\n\x05\x04\x04\x02\x02\x03\x12\x030\x1d\x1e\n=\n\x04\x04\x04\x02\x03\
    \x12\x032\x02\x1e\"0\x20Progress\x20of\x20train\x20abilities.\x20Range:\
    \x20[0.0,\x201.0]\n\n\x0c\n\x05\x04\x04\x02\x03\x04\x12\x032\x02\n\n\x0c\
    \n\x05\x04\x04\x02\x03\x05\x12\x032\x0b\x10\n\x0c\n\x05\x04\x04\x02\x03\
    \x01\x12\x032\x11\x19\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x032\x1c\x1d\n\
    \n\n\x02\x05\0\x12\x045\09\x01\n\n\n\x03\x05\0\x01\x12\x035\x05\x10\n\
    \x1c\n\x04\x05\0\x02\0\x12\x036\x02\x0e\"\x0f\x20Fully\x20visible\n\n\
    \x0c\n\x05\x05\0\x02\0\x01\x12\x036\x02\t\n\x0c\n\x05\x05\0\x02\0\x02\
    \x12\x036\x0c\r\nK\n\x04\x05\0\x02\x01\x12\x037\x02\x0f\">\x20Dimmed\x20\
    version\x20of\x20unit\x20left\x20behind\x20after\x20entering\x20fog\x20o\
    f\x20war\n\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x037\x02\n\n\x0c\n\x05\x05\
    \0\x02\x01\x02\x12\x037\r\x0e\n\x1b\n\x04\x05\0\x02\x02\x12\x038\x02\r\"\
    \x0e\x20Fully\x20hidden\n\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x038\x02\x08\
    \n\x0c\n\x05\x05\0\x02\x02\x02\x12\x038\x0b\x0c\n\n\n\x02\x05\x01\x12\
    \x04;\0@\x01\n\n\n\x03\x05\x01\x01\x12\x03;\x05\r\n\x0b\n\x04\x05\x01\
    \x02\0\x12\x03<\x02\x0b\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03<\x02\x06\n\
    \x0c\n\x05\x05\x01\x02\0\x02\x12\x03<\t\n\n\x0b\n\x04\x05\x01\x02\x01\
    \x12\x03=\x02\x0b\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03=\x02\x06\n\x0c\
    \n\x05\x05\x01\x02\x01\x02\x12\x03=\t\n\n\x0b\n\x04\x05\x01\x02\x02\x12\
    \x03>\x02\x0e\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x03>\x02\t\n\x0c\n\x05\
    \x05\x01\x02\x02\x02\x12\x03>\x0c\r\n\x0b\n\x04\x05\x01\x02\x03\x12\x03?\
    \x02\x0c\n\x0c\n\x05\x05\x01\x02\x03\x01\x12\x03?\x02\x07\n\x0c\n\x05\
    \x05\x01\x02\x03\x02\x12\x03?\n\x0b\n\n\n\x02\x05\x02\x12\x04B\0F\x01\n\
    \n\n\x03\x05\x02\x01\x12\x03B\x05\x0f\n\x0b\n\x04\x05\x02\x02\0\x12\x03C\
    \x02\x0e\n\x0c\n\x05\x05\x02\x02\0\x01\x12\x03C\x02\t\n\x0c\n\x05\x05\
    \x02\x02\0\x02\x12\x03C\x0c\r\n\x0b\n\x04\x05\x02\x02\x01\x12\x03D\x02\
    \x16\n\x0c\n\x05\x05\x02\x02\x01\x01\x12\x03D\x02\x11\n\x0c\n\x05\x05\
    \x02\x02\x01\x02\x12\x03D\x14\x15\n\x0b\n\x04\x05\x02\x02\x02\x12\x03E\
    \x02\x11\n\x0c\n\x05\x05\x02\x02\x02\x01\x12\x03E\x02\x0c\n\x0c\n\x05\
    \x05\x02\x02\x02\x02\x12\x03E\x0f\x10\n\n\n\x02\x04\x05\x12\x04H\0O\x01\
    \n\n\n\x03\x04\x05\x01\x12\x03H\x08\x15\n\x0b\n\x04\x04\x05\x02\0\x12\
    \x03I\x02\x1a\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03I\x02\n\n\x0c\n\x05\
    \x04\x05\x02\0\x05\x12\x03I\x0b\x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\
    \x03I\x12\x15\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03I\x18\x19\n\x0b\n\x04\
    \x04\x05\x02\x01\x12\x03J\x02\x1c\n\x0c\n\x05\x04\x05\x02\x01\x04\x12\
    \x03J\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03J\x0b\x10\n\x0c\n\x05\
    \x04\x05\x02\x01\x01\x12\x03J\x11\x17\n\x0c\n\x05\x04\x05\x02\x01\x03\
    \x12\x03J\x1a\x1b\n\x0b\n\x04\x04\x05\x02\x02\x12\x03K\x02\x20\n\x0c\n\
    \x05\x04\x05\x02\x02\x04\x12\x03K\x02\n\n\x0c\n\x05\x04\x05\x02\x02\x05\
    \x12\x03K\x0b\x10\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03K\x11\x1b\n\x0c\
    \n\x05\x04\x05\x02\x02\x03\x12\x03K\x1e\x1f\n\x0b\n\x04\x04\x05\x02\x03\
    \x12\x03L\x02\x1c\n\x0c\n\x05\x04\x05\x02\x03\x04\x12\x03L\x02\n\n\x0c\n\
    \x05\x04\x05\x02\x03\x05\x12\x03L\x0b\x10\n\x0c\n\x05\x04\x05\x02\x03\
    \x01\x12\x03L\x11\x17\n\x0c\n\x05\x04\x05\x02\x03\x03\x12\x03L\x1a\x1b\n\
    \x0b\n\x04\x04\x05\x02\x04\x12\x03M\x02\x1c\n\x0c\n\x05\x04\x05\x02\x04\
    \x04\x12\x03M\x02\n\n\x0c\n\x05\x04\x05\x02\x04\x05\x12\x03M\x0b\x10\n\
    \x0c\n\x05\x04\x05\x02\x04\x01\x12\x03M\x11\x17\n\x0c\n\x05\x04\x05\x02\
    \x04\x03\x12\x03M\x1a\x1b\n\x0b\n\x04\x04\x05\x02\x05\x12\x03N\x02\x20\n\
    \x0c\n\x05\x04\x05\x02\x05\x04\x12\x03N\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x05\x05\x12\x03N\x0b\x11\n\x0c\n\x05\x04\x05\x02\x05\x01\x12\x03N\x12\
    \x1b\n\x0c\n\x05\x04\x05\x02\x05\x03\x12\x03N\x1e\x1f\n\n\n\x02\x04\x06\
    \x12\x04Q\0}\x01\n\n\n\x03\x04\x06\x01\x12\x03Q\x08\x0c\n:\n\x04\x04\x06\
    \x02\0\x12\x03S\x02(\x1a-\x20Fields\x20are\x20populated\x20based\x20on\
    \x20type/alliance\n\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03S\x02\n\n\x0c\n\
    \x05\x04\x06\x02\0\x06\x12\x03S\x0b\x16\n\x0c\n\x05\x04\x06\x02\0\x01\
    \x12\x03S\x17#\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03S&'\n\x0b\n\x04\x04\
    \x06\x02\x01\x12\x03T\x02!\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03T\x02\
    \n\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x03T\x0b\x13\n\x0c\n\x05\x04\x06\
    \x02\x01\x01\x12\x03T\x14\x1c\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03T\
    \x1f\x20\n+\n\x04\x04\x06\x02\x02\x12\x03V\x02\x1a\"\x1e\x20Unique\x20id\
    entifier\x20for\x20a\x20unit\n\n\x0c\n\x05\x04\x06\x02\x02\x04\x12\x03V\
    \x02\n\n\x0c\n\x05\x04\x06\x02\x02\x05\x12\x03V\x0b\x11\n\x0c\n\x05\x04\
    \x06\x02\x02\x01\x12\x03V\x12\x15\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\
    \x03V\x18\x19\n\x0b\n\x04\x04\x06\x02\x03\x12\x03W\x02\x20\n\x0c\n\x05\
    \x04\x06\x02\x03\x04\x12\x03W\x02\n\n\x0c\n\x05\x04\x06\x02\x03\x05\x12\
    \x03W\x0b\x11\n\x0c\n\x05\x04\x06\x02\x03\x01\x12\x03W\x12\x1b\n\x0c\n\
    \x05\x04\x06\x02\x03\x03\x12\x03W\x1e\x1f\n\x0b\n\x04\x04\x06\x02\x04\
    \x12\x03X\x02\x1b\n\x0c\n\x05\x04\x06\x02\x04\x04\x12\x03X\x02\n\n\x0c\n\
    \x05\x04\x06\x02\x04\x05\x12\x03X\x0b\x10\n\x0c\n\x05\x04\x06\x02\x04\
    \x01\x12\x03X\x11\x16\n\x0c\n\x05\x04\x06\x02\x04\x03\x12\x03X\x19\x1a\n\
    \x0b\n\x04\x04\x06\x02\x05\x12\x03Z\x02\x19\n\x0c\n\x05\x04\x06\x02\x05\
    \x04\x12\x03Z\x02\n\n\x0c\n\x05\x04\x06\x02\x05\x06\x12\x03Z\x0b\x10\n\
    \x0c\n\x05\x04\x06\x02\x05\x01\x12\x03Z\x11\x14\n\x0c\n\x05\x04\x06\x02\
    \x05\x03\x12\x03Z\x17\x18\n\x0b\n\x04\x04\x06\x02\x06\x12\x03[\x02\x1c\n\
    \x0c\n\x05\x04\x06\x02\x06\x04\x12\x03[\x02\n\n\x0c\n\x05\x04\x06\x02\
    \x06\x05\x12\x03[\x0b\x10\n\x0c\n\x05\x04\x06\x02\x06\x01\x12\x03[\x11\
    \x17\n\x0c\n\x05\x04\x06\x02\x06\x03\x12\x03[\x1a\x1b\n\x0b\n\x04\x04\
    \x06\x02\x07\x12\x03\\\x02\x1c\n\x0c\n\x05\x04\x06\x02\x07\x04\x12\x03\\\
    \x02\n\n\x0c\n\x05\x04\x06\x02\x07\x05\x12\x03\\\x0b\x10\n\x0c\n\x05\x04\
    \x06\x02\x07\x01\x12\x03\\\x11\x17\n\x0c\n\x05\x04\x06\x02\x07\x03\x12\
    \x03\\\x1a\x1b\n\x20\n\x04\x04\x06\x02\x08\x12\x03]\x02$\"\x13\x20Range:\
    \x20[0.0,\x201.0]\n\n\x0c\n\x05\x04\x06\x02\x08\x04\x12\x03]\x02\n\n\x0c\
    \n\x05\x04\x06\x02\x08\x05\x12\x03]\x0b\x10\n\x0c\n\x05\x04\x06\x02\x08\
    \x01\x12\x03]\x11\x1f\n\x0c\n\x05\x04\x06\x02\x08\x03\x12\x03]\"#\n\x0b\
    \n\x04\x04\x06\x02\t\x12\x03^\x02!\n\x0c\n\x05\x04\x06\x02\t\x04\x12\x03\
    ^\x02\n\n\x0c\n\x05\x04\x06\x02\t\x06\x12\x03^\x0b\x15\n\x0c\n\x05\x04\
    \x06\x02\t\x01\x12\x03^\x16\x1b\n\x0c\n\x05\x04\x06\x02\t\x03\x12\x03^\
    \x1e\x20\n\x0b\n\x04\x04\x06\x02\n\x12\x03`\x02#\n\x0c\n\x05\x04\x06\x02\
    \n\x04\x12\x03`\x02\n\n\x0c\n\x05\x04\x06\x02\n\x05\x12\x03`\x0b\x10\n\
    \x0c\n\x05\x04\x06\x02\n\x01\x12\x03`\x11\x1d\n\x0c\n\x05\x04\x06\x02\n\
    \x03\x12\x03`\x20\"\n\x0b\n\x04\x04\x06\x02\x0b\x12\x03a\x02\"\n\x0c\n\
    \x05\x04\x06\x02\x0b\x04\x12\x03a\x02\n\n\x0c\n\x05\x04\x06\x02\x0b\x05\
    \x12\x03a\x0b\x10\n\x0c\n\x05\x04\x06\x02\x0b\x01\x12\x03a\x11\x1c\n\x0c\
    \n\x05\x04\x06\x02\x0b\x03\x12\x03a\x1f!\n\x0b\n\x04\x04\x06\x02\x0c\x12\
    \x03c\x02!\n\x0c\n\x05\x04\x06\x02\x0c\x04\x12\x03c\x02\n\n\x0c\n\x05\
    \x04\x06\x02\x0c\x05\x12\x03c\x0b\x0f\n\x0c\n\x05\x04\x06\x02\x0c\x01\
    \x12\x03c\x10\x1b\n\x0c\n\x05\x04\x06\x02\x0c\x03\x12\x03c\x1e\x20\n6\n\
    \x04\x04\x06\x02\r\x12\x03d\x02\"\")\x20Visible\x20and\x20within\x20the\
    \x20camera\x20frustrum.\n\n\x0c\n\x05\x04\x06\x02\r\x04\x12\x03d\x02\n\n\
    \x0c\n\x05\x04\x06\x02\r\x05\x12\x03d\x0b\x0f\n\x0c\n\x05\x04\x06\x02\r\
    \x01\x12\x03d\x10\x1c\n\x0c\n\x05\x04\x06\x02\r\x03\x12\x03d\x1f!\n'\n\
    \x04\x04\x06\x02\x0e\x12\x03e\x02\x1d\"\x1a\x20Detected\x20by\x20sensor\
    \x20tower\n\n\x0c\n\x05\x04\x06\x02\x0e\x04\x12\x03e\x02\n\n\x0c\n\x05\
    \x04\x06\x02\x0e\x05\x12\x03e\x0b\x0f\n\x0c\n\x05\x04\x06\x02\x0e\x01\
    \x12\x03e\x10\x17\n\x0c\n\x05\x04\x06\x02\x0e\x03\x12\x03e\x1a\x1c\n\x0b\
    \n\x04\x04\x06\x02\x0f\x12\x03f\x02\x20\n\x0c\n\x05\x04\x06\x02\x0f\x04\
    \x12\x03f\x02\n\n\x0c\n\x05\x04\x06\x02\x0f\x05\x12\x03f\x0b\x0f\n\x0c\n\
    \x05\x04\x06\x02\x0f\x01\x12\x03f\x10\x1a\n\x0c\n\x05\x04\x06\x02\x0f\
    \x03\x12\x03f\x1d\x1f\n*\n\x04\x04\x06\x02\x10\x12\x03i\x02\x1d\x1a\x1d\
    \x20Not\x20populated\x20for\x20snapshots\n\n\x0c\n\x05\x04\x06\x02\x10\
    \x04\x12\x03i\x02\n\n\x0c\n\x05\x04\x06\x02\x10\x05\x12\x03i\x0b\x10\n\
    \x0c\n\x05\x04\x06\x02\x10\x01\x12\x03i\x11\x17\n\x0c\n\x05\x04\x06\x02\
    \x10\x03\x12\x03i\x1a\x1c\n\x0b\n\x04\x04\x06\x02\x11\x12\x03j\x02!\n\
    \x0c\n\x05\x04\x06\x02\x11\x04\x12\x03j\x02\n\n\x0c\n\x05\x04\x06\x02\
    \x11\x05\x12\x03j\x0b\x10\n\x0c\n\x05\x04\x06\x02\x11\x01\x12\x03j\x11\
    \x1b\n\x0c\n\x05\x04\x06\x02\x11\x03\x12\x03j\x1e\x20\n\x0b\n\x04\x04\
    \x06\x02\x12\x12\x03k\x02\x1d\n\x0c\n\x05\x04\x06\x02\x12\x04\x12\x03k\
    \x02\n\n\x0c\n\x05\x04\x06\x02\x12\x05\x12\x03k\x0b\x10\n\x0c\n\x05\x04\
    \x06\x02\x12\x01\x12\x03k\x11\x17\n\x0c\n\x05\x04\x06\x02\x12\x03\x12\
    \x03k\x1a\x1c\n\x0b\n\x04\x04\x06\x02\x13\x12\x03l\x02\x1d\n\x0c\n\x05\
    \x04\x06\x02\x13\x04\x12\x03l\x02\n\n\x0c\n\x05\x04\x06\x02\x13\x05\x12\
    \x03l\x0b\x10\n\x0c\n\x05\x04\x06\x02\x13\x01\x12\x03l\x11\x17\n\x0c\n\
    \x05\x04\x06\x02\x13\x03\x12\x03l\x1a\x1c\n\x0b\n\x04\x04\x06\x02\x14\
    \x12\x03m\x02'\n\x0c\n\x05\x04\x06\x02\x14\x04\x12\x03m\x02\n\n\x0c\n\
    \x05\x04\x06\x02\x14\x05\x12\x03m\x0b\x10\n\x0c\n\x05\x04\x06\x02\x14\
    \x01\x12\x03m\x11!\n\x0c\n\x05\x04\x06\x02\x14\x03\x12\x03m$&\n\x0b\n\
    \x04\x04\x06\x02\x15\x12\x03n\x02'\n\x0c\n\x05\x04\x06\x02\x15\x04\x12\
    \x03n\x02\n\n\x0c\n\x05\x04\x06\x02\x15\x05\x12\x03n\x0b\x10\n\x0c\n\x05\
    \x04\x06\x02\x15\x01\x12\x03n\x11!\n\x0c\n\x05\x04\x06\x02\x15\x03\x12\
    \x03n$&\n\x0b\n\x04\x04\x06\x02\x16\x12\x03o\x02\x1f\n\x0c\n\x05\x04\x06\
    \x02\x16\x04\x12\x03o\x02\n\n\x0c\n\x05\x04\x06\x02\x16\x05\x12\x03o\x0b\
    \x0f\n\x0c\n\x05\x04\x06\x02\x16\x01\x12\x03o\x10\x19\n\x0c\n\x05\x04\
    \x06\x02\x16\x03\x12\x03o\x1c\x1e\n\x0b\n\x04\x04\x06\x02\x17\x12\x03p\
    \x02!\n\x0c\n\x05\x04\x06\x02\x17\x04\x12\x03p\x02\n\n\x0c\n\x05\x04\x06\
    \x02\x17\x05\x12\x03p\x0b\x0f\n\x0c\n\x05\x04\x06\x02\x17\x01\x12\x03p\
    \x10\x1b\n\x0c\n\x05\x04\x06\x02\x17\x03\x12\x03p\x1e\x20\n(\n\x04\x04\
    \x06\x02\x18\x12\x03s\x02!\x1a\x1b\x20Not\x20populated\x20for\x20enemies\
    \n\n\x0c\n\x05\x04\x06\x02\x18\x04\x12\x03s\x02\n\n\x0c\n\x05\x04\x06\
    \x02\x18\x06\x12\x03s\x0b\x14\n\x0c\n\x05\x04\x06\x02\x18\x01\x12\x03s\
    \x15\x1b\n\x0c\n\x05\x04\x06\x02\x18\x03\x12\x03s\x1e\x20\n\x0b\n\x04\
    \x04\x06\x02\x19\x12\x03t\x02\"\n\x0c\n\x05\x04\x06\x02\x19\x04\x12\x03t\
    \x02\n\n\x0c\n\x05\x04\x06\x02\x19\x05\x12\x03t\x0b\x11\n\x0c\n\x05\x04\
    \x06\x02\x19\x01\x12\x03t\x12\x1c\n\x0c\n\x05\x04\x06\x02\x19\x03\x12\
    \x03t\x1f!\n\x0b\n\x04\x04\x06\x02\x1a\x12\x03u\x02)\n\x0c\n\x05\x04\x06\
    \x02\x1a\x04\x12\x03u\x02\n\n\x0c\n\x05\x04\x06\x02\x1a\x06\x12\x03u\x0b\
    \x18\n\x0c\n\x05\x04\x06\x02\x1a\x01\x12\x03u\x19#\n\x0c\n\x05\x04\x06\
    \x02\x1a\x03\x12\x03u&(\n\x0b\n\x04\x04\x06\x02\x1b\x12\x03v\x02(\n\x0c\
    \n\x05\x04\x06\x02\x1b\x04\x12\x03v\x02\n\n\x0c\n\x05\x04\x06\x02\x1b\
    \x05\x12\x03v\x0b\x10\n\x0c\n\x05\x04\x06\x02\x1b\x01\x12\x03v\x11\"\n\
    \x0c\n\x05\x04\x06\x02\x1b\x03\x12\x03v%'\n\x0b\n\x04\x04\x06\x02\x1c\
    \x12\x03w\x02&\n\x0c\n\x05\x04\x06\x02\x1c\x04\x12\x03w\x02\n\n\x0c\n\
    \x05\x04\x06\x02\x1c\x05\x12\x03w\x0b\x10\n\x0c\n\x05\x04\x06\x02\x1c\
    \x01\x12\x03w\x11\x20\n\x0c\n\x05\x04\x06\x02\x1c\x03\x12\x03w#%\n:\n\
    \x04\x04\x06\x02\x1d\x12\x03x\x02\x20\"-\x20TODO:\x20Should\x20this\x20b\
    e\x20populated\x20for\x20enemies?\n\n\x0c\n\x05\x04\x06\x02\x1d\x04\x12\
    \x03x\x02\n\n\x0c\n\x05\x04\x06\x02\x1d\x05\x12\x03x\x0b\x11\n\x0c\n\x05\
    \x04\x06\x02\x1d\x01\x12\x03x\x12\x1a\n\x0c\n\x05\x04\x06\x02\x1d\x03\
    \x12\x03x\x1d\x1f\n\x0b\n\x04\x04\x06\x02\x1e\x12\x03y\x02*\n\x0c\n\x05\
    \x04\x06\x02\x1e\x04\x12\x03y\x02\n\n\x0c\n\x05\x04\x06\x02\x1e\x05\x12\
    \x03y\x0b\x10\n\x0c\n\x05\x04\x06\x02\x1e\x01\x12\x03y\x11$\n\x0c\n\x05\
    \x04\x06\x02\x1e\x03\x12\x03y')\n\x0b\n\x04\x04\x06\x02\x1f\x12\x03z\x02\
    '\n\x0c\n\x05\x04\x06\x02\x1f\x04\x12\x03z\x02\n\n\x0c\n\x05\x04\x06\x02\
    \x1f\x05\x12\x03z\x0b\x10\n\x0c\n\x05\x04\x06\x02\x1f\x01\x12\x03z\x11!\
    \n\x0c\n\x05\x04\x06\x02\x1f\x03\x12\x03z$&\n\x0b\n\x04\x04\x06\x02\x20\
    \x12\x03{\x02&\n\x0c\n\x05\x04\x06\x02\x20\x04\x12\x03{\x02\n\n\x0c\n\
    \x05\x04\x06\x02\x20\x05\x12\x03{\x0b\x10\n\x0c\n\x05\x04\x06\x02\x20\
    \x01\x12\x03{\x11\x20\n\x0c\n\x05\x04\x06\x02\x20\x03\x12\x03{#%\n\x0b\n\
    \x04\x04\x06\x02!\x12\x03|\x02*\n\x0c\n\x05\x04\x06\x02!\x04\x12\x03|\
    \x02\n\n\x0c\n\x05\x04\x06\x02!\x05\x12\x03|\x0b\x11\n\x0c\n\x05\x04\x06\
    \x02!\x01\x12\x03|\x12$\n\x0c\n\x05\x04\x06\x02!\x03\x12\x03|')\n\x0b\n\
    \x02\x04\x07\x12\x05\x7f\0\x82\x01\x01\n\n\n\x03\x04\x07\x01\x12\x03\x7f\
    \x08\x10\n(\n\x04\x04\x07\x02\0\x12\x04\x80\x01\x02$\"\x1a\x201\x20byte\
    \x20visibility\x20layer.\n\n\r\n\x05\x04\x07\x02\0\x04\x12\x04\x80\x01\
    \x02\n\n\r\n\x05\x04\x07\x02\0\x06\x12\x04\x80\x01\x0b\x14\n\r\n\x05\x04\
    \x07\x02\0\x01\x12\x04\x80\x01\x15\x1f\n\r\n\x05\x04\x07\x02\0\x03\x12\
    \x04\x80\x01\"#\n#\n\x04\x04\x07\x02\x01\x12\x04\x81\x01\x02\x1f\"\x15\
    \x201\x20byte\x20creep\x20layer.\n\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04\
    \x81\x01\x02\n\n\r\n\x05\x04\x07\x02\x01\x06\x12\x04\x81\x01\x0b\x14\n\r\
    \n\x05\x04\x07\x02\x01\x01\x12\x04\x81\x01\x15\x1a\n\r\n\x05\x04\x07\x02\
    \x01\x03\x12\x04\x81\x01\x1d\x1e\n\x18\n\x02\x04\x08\x12\x06\x89\x01\0\
    \x8f\x01\x012\n\n\x20Action\n\n\n\x0b\n\x03\x04\x08\x01\x12\x04\x89\x01\
    \x08\x11\n\x0e\n\x04\x04\x08\x08\0\x12\x06\x8a\x01\x02\x8e\x01\x03\n\r\n\
    \x05\x04\x08\x08\0\x01\x12\x04\x8a\x01\x08\x0e\n\x0c\n\x04\x04\x08\x02\0\
    \x12\x04\x8b\x01\x04*\n\r\n\x05\x04\x08\x02\0\x06\x12\x04\x8b\x01\x04\
    \x18\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\x8b\x01\x19%\n\r\n\x05\x04\x08\
    \x02\0\x03\x12\x04\x8b\x01()\n\x0c\n\x04\x04\x08\x02\x01\x12\x04\x8c\x01\
    \x04(\n\r\n\x05\x04\x08\x02\x01\x06\x12\x04\x8c\x01\x04\x17\n\r\n\x05\
    \x04\x08\x02\x01\x01\x12\x04\x8c\x01\x18#\n\r\n\x05\x04\x08\x02\x01\x03\
    \x12\x04\x8c\x01&'\n\x0c\n\x04\x04\x08\x02\x02\x12\x04\x8d\x01\x040\n\r\
    \n\x05\x04\x08\x02\x02\x06\x12\x04\x8d\x01\x04\x1b\n\r\n\x05\x04\x08\x02\
    \x02\x01\x12\x04\x8d\x01\x1c+\n\r\n\x05\x04\x08\x02\x02\x03\x12\x04\x8d\
    \x01./\n\x0c\n\x02\x04\t\x12\x06\x91\x01\0\x99\x01\x01\n\x0b\n\x03\x04\t\
    \x01\x12\x04\x91\x01\x08\x1c\n\x0c\n\x04\x04\t\x02\0\x12\x04\x92\x01\x02\
    \x20\n\r\n\x05\x04\t\x02\0\x04\x12\x04\x92\x01\x02\n\n\r\n\x05\x04\t\x02\
    \0\x05\x12\x04\x92\x01\x0b\x10\n\r\n\x05\x04\t\x02\0\x01\x12\x04\x92\x01\
    \x11\x1b\n\r\n\x05\x04\t\x02\0\x03\x12\x04\x92\x01\x1e\x1f\n\x0e\n\x04\
    \x04\t\x08\0\x12\x06\x93\x01\x02\x96\x01\x03\n\r\n\x05\x04\t\x08\0\x01\
    \x12\x04\x93\x01\x08\x0e\n\x0c\n\x04\x04\t\x02\x01\x12\x04\x94\x01\x04'\
    \n\r\n\x05\x04\t\x02\x01\x06\x12\x04\x94\x01\x04\x0b\n\r\n\x05\x04\t\x02\
    \x01\x01\x12\x04\x94\x01\x0c\"\n\r\n\x05\x04\t\x02\x01\x03\x12\x04\x94\
    \x01%&\n\x0c\n\x04\x04\t\x02\x02\x12\x04\x95\x01\x04\x1f\n\r\n\x05\x04\t\
    \x02\x02\x05\x12\x04\x95\x01\x04\n\n\r\n\x05\x04\t\x02\x02\x01\x12\x04\
    \x95\x01\x0b\x1a\n\r\n\x05\x04\t\x02\x02\x03\x12\x04\x95\x01\x1d\x1e\n\
    \x0c\n\x04\x04\t\x02\x03\x12\x04\x97\x01\x02\x20\n\r\n\x05\x04\t\x02\x03\
    \x04\x12\x04\x97\x01\x02\n\n\r\n\x05\x04\t\x02\x03\x05\x12\x04\x97\x01\
    \x0b\x11\n\r\n\x05\x04\t\x02\x03\x01\x12\x04\x97\x01\x12\x1b\n\r\n\x05\
    \x04\t\x02\x03\x03\x12\x04\x97\x01\x1e\x1f\n\x0c\n\x04\x04\t\x02\x04\x12\
    \x04\x98\x01\x02\"\n\r\n\x05\x04\t\x02\x04\x04\x12\x04\x98\x01\x02\n\n\r\
    \n\x05\x04\t\x02\x04\x05\x12\x04\x98\x01\x0b\x0f\n\r\n\x05\x04\t\x02\x04\
    \x01\x12\x04\x98\x01\x10\x1d\n\r\n\x05\x04\t\x02\x04\x03\x12\x04\x98\x01\
    \x20!\n\x0c\n\x02\x04\n\x12\x06\x9b\x01\0\x9d\x01\x01\n\x0b\n\x03\x04\n\
    \x01\x12\x04\x9b\x01\x08\x1b\n\x0c\n\x04\x04\n\x02\0\x12\x04\x9c\x01\x02\
    (\n\r\n\x05\x04\n\x02\0\x04\x12\x04\x9c\x01\x02\n\n\r\n\x05\x04\n\x02\0\
    \x06\x12\x04\x9c\x01\x0b\x10\n\r\n\x05\x04\n\x02\0\x01\x12\x04\x9c\x01\
    \x11#\n\r\n\x05\x04\n\x02\0\x03\x12\x04\x9c\x01&'\n\x0c\n\x02\x04\x0b\
    \x12\x06\x9f\x01\0\xa2\x01\x01\n\x0b\n\x03\x04\x0b\x01\x12\x04\x9f\x01\
    \x08\x1f\n\x0c\n\x04\x04\x0b\x02\0\x12\x04\xa0\x01\x02\x20\n\r\n\x05\x04\
    \x0b\x02\0\x04\x12\x04\xa0\x01\x02\n\n\r\n\x05\x04\x0b\x02\0\x05\x12\x04\
    \xa0\x01\x0b\x10\n\r\n\x05\x04\x0b\x02\0\x01\x12\x04\xa0\x01\x11\x1b\n\r\
    \n\x05\x04\x0b\x02\0\x03\x12\x04\xa0\x01\x1e\x1f\n\x0c\n\x04\x04\x0b\x02\
    \x01\x12\x04\xa1\x01\x02\x20\n\r\n\x05\x04\x0b\x02\x01\x04\x12\x04\xa1\
    \x01\x02\n\n\r\n\x05\x04\x0b\x02\x01\x05\x12\x04\xa1\x01\x0b\x11\n\r\n\
    \x05\x04\x0b\x02\x01\x01\x12\x04\xa1\x01\x12\x1b\n\r\n\x05\x04\x0b\x02\
    \x01\x03\x12\x04\xa1\x01\x1e\x1f\n\x0c\n\x02\x04\x0c\x12\x06\xa4\x01\0\
    \xa6\x01\x01\n\x0b\n\x03\x04\x0c\x01\x12\x04\xa4\x01\x08\r\n\x0c\n\x04\
    \x04\x0c\x02\0\x12\x04\xa5\x01\x02!\n\r\n\x05\x04\x0c\x02\0\x04\x12\x04\
    \xa5\x01\x02\n\n\r\n\x05\x04\x0c\x02\0\x05\x12\x04\xa5\x01\x0b\x11\n\r\n\
    \x05\x04\x0c\x02\0\x01\x12\x04\xa5\x01\x12\x1c\n\r\n\x05\x04\x0c\x02\0\
    \x03\x12\x04\xa5\x01\x1f\x20\
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
