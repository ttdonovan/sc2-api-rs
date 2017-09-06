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
pub struct ObservationUI {
    // message fields
    groups: ::protobuf::RepeatedField<ControlGroup>,
    // message oneof groups
    panel: ::std::option::Option<ObservationUI_oneof_panel>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObservationUI {}

#[derive(Clone,PartialEq)]
pub enum ObservationUI_oneof_panel {
    single(SinglePanel),
    multi(MultiPanel),
    cargo(CargoPanel),
    production(ProductionPanel),
}

impl ObservationUI {
    pub fn new() -> ObservationUI {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObservationUI {
        static mut instance: ::protobuf::lazy::Lazy<ObservationUI> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObservationUI,
        };
        unsafe {
            instance.get(ObservationUI::new)
        }
    }

    // repeated .SC2APIProtocol.ControlGroup groups = 1;

    pub fn clear_groups(&mut self) {
        self.groups.clear();
    }

    // Param is passed by value, moved
    pub fn set_groups(&mut self, v: ::protobuf::RepeatedField<ControlGroup>) {
        self.groups = v;
    }

    // Mutable pointer to the field.
    pub fn mut_groups(&mut self) -> &mut ::protobuf::RepeatedField<ControlGroup> {
        &mut self.groups
    }

    // Take field
    pub fn take_groups(&mut self) -> ::protobuf::RepeatedField<ControlGroup> {
        ::std::mem::replace(&mut self.groups, ::protobuf::RepeatedField::new())
    }

    pub fn get_groups(&self) -> &[ControlGroup] {
        &self.groups
    }

    fn get_groups_for_reflect(&self) -> &::protobuf::RepeatedField<ControlGroup> {
        &self.groups
    }

    fn mut_groups_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ControlGroup> {
        &mut self.groups
    }

    // optional .SC2APIProtocol.SinglePanel single = 2;

    pub fn clear_single(&mut self) {
        self.panel = ::std::option::Option::None;
    }

    pub fn has_single(&self) -> bool {
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::single(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_single(&mut self, v: SinglePanel) {
        self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::single(v))
    }

    // Mutable pointer to the field.
    pub fn mut_single(&mut self) -> &mut SinglePanel {
        if let ::std::option::Option::Some(ObservationUI_oneof_panel::single(_)) = self.panel {
        } else {
            self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::single(SinglePanel::new()));
        }
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::single(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_single(&mut self) -> SinglePanel {
        if self.has_single() {
            match self.panel.take() {
                ::std::option::Option::Some(ObservationUI_oneof_panel::single(v)) => v,
                _ => panic!(),
            }
        } else {
            SinglePanel::new()
        }
    }

    pub fn get_single(&self) -> &SinglePanel {
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::single(ref v)) => v,
            _ => SinglePanel::default_instance(),
        }
    }

    // optional .SC2APIProtocol.MultiPanel multi = 3;

    pub fn clear_multi(&mut self) {
        self.panel = ::std::option::Option::None;
    }

    pub fn has_multi(&self) -> bool {
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::multi(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_multi(&mut self, v: MultiPanel) {
        self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::multi(v))
    }

    // Mutable pointer to the field.
    pub fn mut_multi(&mut self) -> &mut MultiPanel {
        if let ::std::option::Option::Some(ObservationUI_oneof_panel::multi(_)) = self.panel {
        } else {
            self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::multi(MultiPanel::new()));
        }
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::multi(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_multi(&mut self) -> MultiPanel {
        if self.has_multi() {
            match self.panel.take() {
                ::std::option::Option::Some(ObservationUI_oneof_panel::multi(v)) => v,
                _ => panic!(),
            }
        } else {
            MultiPanel::new()
        }
    }

    pub fn get_multi(&self) -> &MultiPanel {
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::multi(ref v)) => v,
            _ => MultiPanel::default_instance(),
        }
    }

    // optional .SC2APIProtocol.CargoPanel cargo = 4;

    pub fn clear_cargo(&mut self) {
        self.panel = ::std::option::Option::None;
    }

    pub fn has_cargo(&self) -> bool {
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::cargo(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cargo(&mut self, v: CargoPanel) {
        self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::cargo(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cargo(&mut self) -> &mut CargoPanel {
        if let ::std::option::Option::Some(ObservationUI_oneof_panel::cargo(_)) = self.panel {
        } else {
            self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::cargo(CargoPanel::new()));
        }
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::cargo(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cargo(&mut self) -> CargoPanel {
        if self.has_cargo() {
            match self.panel.take() {
                ::std::option::Option::Some(ObservationUI_oneof_panel::cargo(v)) => v,
                _ => panic!(),
            }
        } else {
            CargoPanel::new()
        }
    }

    pub fn get_cargo(&self) -> &CargoPanel {
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::cargo(ref v)) => v,
            _ => CargoPanel::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ProductionPanel production = 5;

    pub fn clear_production(&mut self) {
        self.panel = ::std::option::Option::None;
    }

    pub fn has_production(&self) -> bool {
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::production(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_production(&mut self, v: ProductionPanel) {
        self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::production(v))
    }

    // Mutable pointer to the field.
    pub fn mut_production(&mut self) -> &mut ProductionPanel {
        if let ::std::option::Option::Some(ObservationUI_oneof_panel::production(_)) = self.panel {
        } else {
            self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::production(ProductionPanel::new()));
        }
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::production(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_production(&mut self) -> ProductionPanel {
        if self.has_production() {
            match self.panel.take() {
                ::std::option::Option::Some(ObservationUI_oneof_panel::production(v)) => v,
                _ => panic!(),
            }
        } else {
            ProductionPanel::new()
        }
    }

    pub fn get_production(&self) -> &ProductionPanel {
        match self.panel {
            ::std::option::Option::Some(ObservationUI_oneof_panel::production(ref v)) => v,
            _ => ProductionPanel::default_instance(),
        }
    }
}

impl ::protobuf::Message for ObservationUI {
    fn is_initialized(&self) -> bool {
        for v in &self.groups {
            if !v.is_initialized() {
                return false;
            }
        };
        if let Some(ObservationUI_oneof_panel::single(ref v)) = self.panel {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ObservationUI_oneof_panel::multi(ref v)) = self.panel {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ObservationUI_oneof_panel::cargo(ref v)) = self.panel {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ObservationUI_oneof_panel::production(ref v)) = self.panel {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.groups)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::single(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::multi(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::cargo(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.panel = ::std::option::Option::Some(ObservationUI_oneof_panel::production(is.read_message()?));
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
        for value in &self.groups {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let ::std::option::Option::Some(ref v) = self.panel {
            match v {
                &ObservationUI_oneof_panel::single(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ObservationUI_oneof_panel::multi(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ObservationUI_oneof_panel::cargo(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ObservationUI_oneof_panel::production(ref v) => {
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
        for v in &self.groups {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let ::std::option::Option::Some(ref v) = self.panel {
            match v {
                &ObservationUI_oneof_panel::single(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ObservationUI_oneof_panel::multi(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ObservationUI_oneof_panel::cargo(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ObservationUI_oneof_panel::production(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ObservationUI {
    fn new() -> ObservationUI {
        ObservationUI::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObservationUI>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ControlGroup>>(
                    "groups",
                    ObservationUI::get_groups_for_reflect,
                    ObservationUI::mut_groups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, SinglePanel>(
                    "single",
                    ObservationUI::has_single,
                    ObservationUI::get_single,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, MultiPanel>(
                    "multi",
                    ObservationUI::has_multi,
                    ObservationUI::get_multi,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, CargoPanel>(
                    "cargo",
                    ObservationUI::has_cargo,
                    ObservationUI::get_cargo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ProductionPanel>(
                    "production",
                    ObservationUI::has_production,
                    ObservationUI::get_production,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObservationUI>(
                    "ObservationUI",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObservationUI {
    fn clear(&mut self) {
        self.clear_groups();
        self.clear_single();
        self.clear_multi();
        self.clear_cargo();
        self.clear_production();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ObservationUI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ObservationUI {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ControlGroup {
    // message fields
    control_group_index: ::std::option::Option<u32>,
    leader_unit_type: ::std::option::Option<u32>,
    count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ControlGroup {}

impl ControlGroup {
    pub fn new() -> ControlGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ControlGroup {
        static mut instance: ::protobuf::lazy::Lazy<ControlGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ControlGroup,
        };
        unsafe {
            instance.get(ControlGroup::new)
        }
    }

    // optional uint32 control_group_index = 1;

    pub fn clear_control_group_index(&mut self) {
        self.control_group_index = ::std::option::Option::None;
    }

    pub fn has_control_group_index(&self) -> bool {
        self.control_group_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_group_index(&mut self, v: u32) {
        self.control_group_index = ::std::option::Option::Some(v);
    }

    pub fn get_control_group_index(&self) -> u32 {
        self.control_group_index.unwrap_or(0)
    }

    fn get_control_group_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.control_group_index
    }

    fn mut_control_group_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.control_group_index
    }

    // optional uint32 leader_unit_type = 2;

    pub fn clear_leader_unit_type(&mut self) {
        self.leader_unit_type = ::std::option::Option::None;
    }

    pub fn has_leader_unit_type(&self) -> bool {
        self.leader_unit_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_unit_type(&mut self, v: u32) {
        self.leader_unit_type = ::std::option::Option::Some(v);
    }

    pub fn get_leader_unit_type(&self) -> u32 {
        self.leader_unit_type.unwrap_or(0)
    }

    fn get_leader_unit_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.leader_unit_type
    }

    fn mut_leader_unit_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.leader_unit_type
    }

    // optional uint32 count = 3;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    fn get_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.count
    }
}

impl ::protobuf::Message for ControlGroup {
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
                    self.control_group_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.leader_unit_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.count = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.control_group_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.leader_unit_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.control_group_index {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.leader_unit_type {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.count {
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

impl ::protobuf::MessageStatic for ControlGroup {
    fn new() -> ControlGroup {
        ControlGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<ControlGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "control_group_index",
                    ControlGroup::get_control_group_index_for_reflect,
                    ControlGroup::mut_control_group_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "leader_unit_type",
                    ControlGroup::get_leader_unit_type_for_reflect,
                    ControlGroup::mut_leader_unit_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    ControlGroup::get_count_for_reflect,
                    ControlGroup::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ControlGroup>(
                    "ControlGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ControlGroup {
    fn clear(&mut self) {
        self.clear_control_group_index();
        self.clear_leader_unit_type();
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ControlGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ControlGroup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnitInfo {
    // message fields
    unit_type: ::std::option::Option<u32>,
    player_relative: ::std::option::Option<u32>,
    health: ::std::option::Option<i32>,
    shields: ::std::option::Option<i32>,
    energy: ::std::option::Option<i32>,
    transport_slots_taken: ::std::option::Option<i32>,
    build_progress: ::std::option::Option<f32>,
    add_on: ::protobuf::SingularPtrField<UnitInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnitInfo {}

impl UnitInfo {
    pub fn new() -> UnitInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnitInfo {
        static mut instance: ::protobuf::lazy::Lazy<UnitInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnitInfo,
        };
        unsafe {
            instance.get(UnitInfo::new)
        }
    }

    // optional uint32 unit_type = 1;

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

    // optional uint32 player_relative = 2;

    pub fn clear_player_relative(&mut self) {
        self.player_relative = ::std::option::Option::None;
    }

    pub fn has_player_relative(&self) -> bool {
        self.player_relative.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_relative(&mut self, v: u32) {
        self.player_relative = ::std::option::Option::Some(v);
    }

    pub fn get_player_relative(&self) -> u32 {
        self.player_relative.unwrap_or(0)
    }

    fn get_player_relative_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_relative
    }

    fn mut_player_relative_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_relative
    }

    // optional int32 health = 3;

    pub fn clear_health(&mut self) {
        self.health = ::std::option::Option::None;
    }

    pub fn has_health(&self) -> bool {
        self.health.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health(&mut self, v: i32) {
        self.health = ::std::option::Option::Some(v);
    }

    pub fn get_health(&self) -> i32 {
        self.health.unwrap_or(0)
    }

    fn get_health_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.health
    }

    fn mut_health_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.health
    }

    // optional int32 shields = 4;

    pub fn clear_shields(&mut self) {
        self.shields = ::std::option::Option::None;
    }

    pub fn has_shields(&self) -> bool {
        self.shields.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shields(&mut self, v: i32) {
        self.shields = ::std::option::Option::Some(v);
    }

    pub fn get_shields(&self) -> i32 {
        self.shields.unwrap_or(0)
    }

    fn get_shields_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.shields
    }

    fn mut_shields_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.shields
    }

    // optional int32 energy = 5;

    pub fn clear_energy(&mut self) {
        self.energy = ::std::option::Option::None;
    }

    pub fn has_energy(&self) -> bool {
        self.energy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_energy(&mut self, v: i32) {
        self.energy = ::std::option::Option::Some(v);
    }

    pub fn get_energy(&self) -> i32 {
        self.energy.unwrap_or(0)
    }

    fn get_energy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.energy
    }

    fn mut_energy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.energy
    }

    // optional int32 transport_slots_taken = 6;

    pub fn clear_transport_slots_taken(&mut self) {
        self.transport_slots_taken = ::std::option::Option::None;
    }

    pub fn has_transport_slots_taken(&self) -> bool {
        self.transport_slots_taken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transport_slots_taken(&mut self, v: i32) {
        self.transport_slots_taken = ::std::option::Option::Some(v);
    }

    pub fn get_transport_slots_taken(&self) -> i32 {
        self.transport_slots_taken.unwrap_or(0)
    }

    fn get_transport_slots_taken_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.transport_slots_taken
    }

    fn mut_transport_slots_taken_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.transport_slots_taken
    }

    // optional float build_progress = 7;

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

    // optional .SC2APIProtocol.UnitInfo add_on = 8;

    pub fn clear_add_on(&mut self) {
        self.add_on.clear();
    }

    pub fn has_add_on(&self) -> bool {
        self.add_on.is_some()
    }

    // Param is passed by value, moved
    pub fn set_add_on(&mut self, v: UnitInfo) {
        self.add_on = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_add_on(&mut self) -> &mut UnitInfo {
        if self.add_on.is_none() {
            self.add_on.set_default();
        }
        self.add_on.as_mut().unwrap()
    }

    // Take field
    pub fn take_add_on(&mut self) -> UnitInfo {
        self.add_on.take().unwrap_or_else(|| UnitInfo::new())
    }

    pub fn get_add_on(&self) -> &UnitInfo {
        self.add_on.as_ref().unwrap_or_else(|| UnitInfo::default_instance())
    }

    fn get_add_on_for_reflect(&self) -> &::protobuf::SingularPtrField<UnitInfo> {
        &self.add_on
    }

    fn mut_add_on_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UnitInfo> {
        &mut self.add_on
    }
}

impl ::protobuf::Message for UnitInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.add_on {
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
                    self.unit_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.player_relative = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.health = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.shields = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.energy = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.transport_slots_taken = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.build_progress = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.add_on)?;
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
        if let Some(v) = self.unit_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.player_relative {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.health {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.shields {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.energy {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.transport_slots_taken {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.build_progress {
            my_size += 5;
        }
        if let Some(ref v) = self.add_on.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_type {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.player_relative {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.health {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.shields {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.energy {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.transport_slots_taken {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.build_progress {
            os.write_float(7, v)?;
        }
        if let Some(ref v) = self.add_on.as_ref() {
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

impl ::protobuf::MessageStatic for UnitInfo {
    fn new() -> UnitInfo {
        UnitInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnitInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unit_type",
                    UnitInfo::get_unit_type_for_reflect,
                    UnitInfo::mut_unit_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_relative",
                    UnitInfo::get_player_relative_for_reflect,
                    UnitInfo::mut_player_relative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "health",
                    UnitInfo::get_health_for_reflect,
                    UnitInfo::mut_health_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "shields",
                    UnitInfo::get_shields_for_reflect,
                    UnitInfo::mut_shields_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "energy",
                    UnitInfo::get_energy_for_reflect,
                    UnitInfo::mut_energy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "transport_slots_taken",
                    UnitInfo::get_transport_slots_taken_for_reflect,
                    UnitInfo::mut_transport_slots_taken_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "build_progress",
                    UnitInfo::get_build_progress_for_reflect,
                    UnitInfo::mut_build_progress_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnitInfo>>(
                    "add_on",
                    UnitInfo::get_add_on_for_reflect,
                    UnitInfo::mut_add_on_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnitInfo>(
                    "UnitInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnitInfo {
    fn clear(&mut self) {
        self.clear_unit_type();
        self.clear_player_relative();
        self.clear_health();
        self.clear_shields();
        self.clear_energy();
        self.clear_transport_slots_taken();
        self.clear_build_progress();
        self.clear_add_on();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnitInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnitInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SinglePanel {
    // message fields
    unit: ::protobuf::SingularPtrField<UnitInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SinglePanel {}

impl SinglePanel {
    pub fn new() -> SinglePanel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SinglePanel {
        static mut instance: ::protobuf::lazy::Lazy<SinglePanel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SinglePanel,
        };
        unsafe {
            instance.get(SinglePanel::new)
        }
    }

    // optional .SC2APIProtocol.UnitInfo unit = 1;

    pub fn clear_unit(&mut self) {
        self.unit.clear();
    }

    pub fn has_unit(&self) -> bool {
        self.unit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit(&mut self, v: UnitInfo) {
        self.unit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit(&mut self) -> &mut UnitInfo {
        if self.unit.is_none() {
            self.unit.set_default();
        }
        self.unit.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit(&mut self) -> UnitInfo {
        self.unit.take().unwrap_or_else(|| UnitInfo::new())
    }

    pub fn get_unit(&self) -> &UnitInfo {
        self.unit.as_ref().unwrap_or_else(|| UnitInfo::default_instance())
    }

    fn get_unit_for_reflect(&self) -> &::protobuf::SingularPtrField<UnitInfo> {
        &self.unit
    }

    fn mut_unit_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UnitInfo> {
        &mut self.unit
    }
}

impl ::protobuf::Message for SinglePanel {
    fn is_initialized(&self) -> bool {
        for v in &self.unit {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit)?;
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
        if let Some(ref v) = self.unit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.unit.as_ref() {
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

impl ::protobuf::MessageStatic for SinglePanel {
    fn new() -> SinglePanel {
        SinglePanel::new()
    }

    fn descriptor_static(_: ::std::option::Option<SinglePanel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnitInfo>>(
                    "unit",
                    SinglePanel::get_unit_for_reflect,
                    SinglePanel::mut_unit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SinglePanel>(
                    "SinglePanel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SinglePanel {
    fn clear(&mut self) {
        self.clear_unit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SinglePanel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SinglePanel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MultiPanel {
    // message fields
    units: ::protobuf::RepeatedField<UnitInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MultiPanel {}

impl MultiPanel {
    pub fn new() -> MultiPanel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MultiPanel {
        static mut instance: ::protobuf::lazy::Lazy<MultiPanel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MultiPanel,
        };
        unsafe {
            instance.get(MultiPanel::new)
        }
    }

    // repeated .SC2APIProtocol.UnitInfo units = 1;

    pub fn clear_units(&mut self) {
        self.units.clear();
    }

    // Param is passed by value, moved
    pub fn set_units(&mut self, v: ::protobuf::RepeatedField<UnitInfo>) {
        self.units = v;
    }

    // Mutable pointer to the field.
    pub fn mut_units(&mut self) -> &mut ::protobuf::RepeatedField<UnitInfo> {
        &mut self.units
    }

    // Take field
    pub fn take_units(&mut self) -> ::protobuf::RepeatedField<UnitInfo> {
        ::std::mem::replace(&mut self.units, ::protobuf::RepeatedField::new())
    }

    pub fn get_units(&self) -> &[UnitInfo] {
        &self.units
    }

    fn get_units_for_reflect(&self) -> &::protobuf::RepeatedField<UnitInfo> {
        &self.units
    }

    fn mut_units_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UnitInfo> {
        &mut self.units
    }
}

impl ::protobuf::Message for MultiPanel {
    fn is_initialized(&self) -> bool {
        for v in &self.units {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.units)?;
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
        for value in &self.units {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.units {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for MultiPanel {
    fn new() -> MultiPanel {
        MultiPanel::new()
    }

    fn descriptor_static(_: ::std::option::Option<MultiPanel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnitInfo>>(
                    "units",
                    MultiPanel::get_units_for_reflect,
                    MultiPanel::mut_units_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MultiPanel>(
                    "MultiPanel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MultiPanel {
    fn clear(&mut self) {
        self.clear_units();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MultiPanel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MultiPanel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CargoPanel {
    // message fields
    unit: ::protobuf::SingularPtrField<UnitInfo>,
    passengers: ::protobuf::RepeatedField<UnitInfo>,
    slots_available: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CargoPanel {}

impl CargoPanel {
    pub fn new() -> CargoPanel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CargoPanel {
        static mut instance: ::protobuf::lazy::Lazy<CargoPanel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CargoPanel,
        };
        unsafe {
            instance.get(CargoPanel::new)
        }
    }

    // optional .SC2APIProtocol.UnitInfo unit = 1;

    pub fn clear_unit(&mut self) {
        self.unit.clear();
    }

    pub fn has_unit(&self) -> bool {
        self.unit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit(&mut self, v: UnitInfo) {
        self.unit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit(&mut self) -> &mut UnitInfo {
        if self.unit.is_none() {
            self.unit.set_default();
        }
        self.unit.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit(&mut self) -> UnitInfo {
        self.unit.take().unwrap_or_else(|| UnitInfo::new())
    }

    pub fn get_unit(&self) -> &UnitInfo {
        self.unit.as_ref().unwrap_or_else(|| UnitInfo::default_instance())
    }

    fn get_unit_for_reflect(&self) -> &::protobuf::SingularPtrField<UnitInfo> {
        &self.unit
    }

    fn mut_unit_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UnitInfo> {
        &mut self.unit
    }

    // repeated .SC2APIProtocol.UnitInfo passengers = 2;

    pub fn clear_passengers(&mut self) {
        self.passengers.clear();
    }

    // Param is passed by value, moved
    pub fn set_passengers(&mut self, v: ::protobuf::RepeatedField<UnitInfo>) {
        self.passengers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_passengers(&mut self) -> &mut ::protobuf::RepeatedField<UnitInfo> {
        &mut self.passengers
    }

    // Take field
    pub fn take_passengers(&mut self) -> ::protobuf::RepeatedField<UnitInfo> {
        ::std::mem::replace(&mut self.passengers, ::protobuf::RepeatedField::new())
    }

    pub fn get_passengers(&self) -> &[UnitInfo] {
        &self.passengers
    }

    fn get_passengers_for_reflect(&self) -> &::protobuf::RepeatedField<UnitInfo> {
        &self.passengers
    }

    fn mut_passengers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UnitInfo> {
        &mut self.passengers
    }

    // optional int32 slots_available = 3;

    pub fn clear_slots_available(&mut self) {
        self.slots_available = ::std::option::Option::None;
    }

    pub fn has_slots_available(&self) -> bool {
        self.slots_available.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slots_available(&mut self, v: i32) {
        self.slots_available = ::std::option::Option::Some(v);
    }

    pub fn get_slots_available(&self) -> i32 {
        self.slots_available.unwrap_or(0)
    }

    fn get_slots_available_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slots_available
    }

    fn mut_slots_available_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slots_available
    }
}

impl ::protobuf::Message for CargoPanel {
    fn is_initialized(&self) -> bool {
        for v in &self.unit {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.passengers)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.slots_available = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.unit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.passengers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.slots_available {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.unit.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.passengers {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.slots_available {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CargoPanel {
    fn new() -> CargoPanel {
        CargoPanel::new()
    }

    fn descriptor_static(_: ::std::option::Option<CargoPanel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnitInfo>>(
                    "unit",
                    CargoPanel::get_unit_for_reflect,
                    CargoPanel::mut_unit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnitInfo>>(
                    "passengers",
                    CargoPanel::get_passengers_for_reflect,
                    CargoPanel::mut_passengers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slots_available",
                    CargoPanel::get_slots_available_for_reflect,
                    CargoPanel::mut_slots_available_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CargoPanel>(
                    "CargoPanel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CargoPanel {
    fn clear(&mut self) {
        self.clear_unit();
        self.clear_passengers();
        self.clear_slots_available();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CargoPanel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CargoPanel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProductionPanel {
    // message fields
    unit: ::protobuf::SingularPtrField<UnitInfo>,
    build_queue: ::protobuf::RepeatedField<UnitInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProductionPanel {}

impl ProductionPanel {
    pub fn new() -> ProductionPanel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProductionPanel {
        static mut instance: ::protobuf::lazy::Lazy<ProductionPanel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProductionPanel,
        };
        unsafe {
            instance.get(ProductionPanel::new)
        }
    }

    // optional .SC2APIProtocol.UnitInfo unit = 1;

    pub fn clear_unit(&mut self) {
        self.unit.clear();
    }

    pub fn has_unit(&self) -> bool {
        self.unit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit(&mut self, v: UnitInfo) {
        self.unit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_unit(&mut self) -> &mut UnitInfo {
        if self.unit.is_none() {
            self.unit.set_default();
        }
        self.unit.as_mut().unwrap()
    }

    // Take field
    pub fn take_unit(&mut self) -> UnitInfo {
        self.unit.take().unwrap_or_else(|| UnitInfo::new())
    }

    pub fn get_unit(&self) -> &UnitInfo {
        self.unit.as_ref().unwrap_or_else(|| UnitInfo::default_instance())
    }

    fn get_unit_for_reflect(&self) -> &::protobuf::SingularPtrField<UnitInfo> {
        &self.unit
    }

    fn mut_unit_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UnitInfo> {
        &mut self.unit
    }

    // repeated .SC2APIProtocol.UnitInfo build_queue = 2;

    pub fn clear_build_queue(&mut self) {
        self.build_queue.clear();
    }

    // Param is passed by value, moved
    pub fn set_build_queue(&mut self, v: ::protobuf::RepeatedField<UnitInfo>) {
        self.build_queue = v;
    }

    // Mutable pointer to the field.
    pub fn mut_build_queue(&mut self) -> &mut ::protobuf::RepeatedField<UnitInfo> {
        &mut self.build_queue
    }

    // Take field
    pub fn take_build_queue(&mut self) -> ::protobuf::RepeatedField<UnitInfo> {
        ::std::mem::replace(&mut self.build_queue, ::protobuf::RepeatedField::new())
    }

    pub fn get_build_queue(&self) -> &[UnitInfo] {
        &self.build_queue
    }

    fn get_build_queue_for_reflect(&self) -> &::protobuf::RepeatedField<UnitInfo> {
        &self.build_queue
    }

    fn mut_build_queue_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UnitInfo> {
        &mut self.build_queue
    }
}

impl ::protobuf::Message for ProductionPanel {
    fn is_initialized(&self) -> bool {
        for v in &self.unit {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.build_queue {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.unit)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.build_queue)?;
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
        if let Some(ref v) = self.unit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.build_queue {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.unit.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.build_queue {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ProductionPanel {
    fn new() -> ProductionPanel {
        ProductionPanel::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProductionPanel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnitInfo>>(
                    "unit",
                    ProductionPanel::get_unit_for_reflect,
                    ProductionPanel::mut_unit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UnitInfo>>(
                    "build_queue",
                    ProductionPanel::get_build_queue_for_reflect,
                    ProductionPanel::mut_build_queue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProductionPanel>(
                    "ProductionPanel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProductionPanel {
    fn clear(&mut self) {
        self.clear_unit();
        self.clear_build_queue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProductionPanel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProductionPanel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionUI {
    // message oneof groups
    action: ::std::option::Option<ActionUI_oneof_action>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionUI {}

#[derive(Clone,PartialEq)]
pub enum ActionUI_oneof_action {
    control_group(ActionControlGroup),
    select_army(ActionSelectArmy),
    select_warp_gates(ActionSelectWarpGates),
    select_larva(ActionSelectLarva),
    select_idle_worker(ActionSelectIdleWorker),
    multi_panel(ActionMultiPanel),
    cargo_panel(ActionCargoPanelUnload),
    production_panel(ActionProductionPanelRemoveFromQueue),
    toggle_autocast(ActionToggleAutocast),
}

impl ActionUI {
    pub fn new() -> ActionUI {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionUI {
        static mut instance: ::protobuf::lazy::Lazy<ActionUI> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionUI,
        };
        unsafe {
            instance.get(ActionUI::new)
        }
    }

    // optional .SC2APIProtocol.ActionControlGroup control_group = 1;

    pub fn clear_control_group(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_control_group(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::control_group(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_control_group(&mut self, v: ActionControlGroup) {
        self.action = ::std::option::Option::Some(ActionUI_oneof_action::control_group(v))
    }

    // Mutable pointer to the field.
    pub fn mut_control_group(&mut self) -> &mut ActionControlGroup {
        if let ::std::option::Option::Some(ActionUI_oneof_action::control_group(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionUI_oneof_action::control_group(ActionControlGroup::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::control_group(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_control_group(&mut self) -> ActionControlGroup {
        if self.has_control_group() {
            match self.action.take() {
                ::std::option::Option::Some(ActionUI_oneof_action::control_group(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionControlGroup::new()
        }
    }

    pub fn get_control_group(&self) -> &ActionControlGroup {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::control_group(ref v)) => v,
            _ => ActionControlGroup::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionSelectArmy select_army = 2;

    pub fn clear_select_army(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_select_army(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_army(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_select_army(&mut self, v: ActionSelectArmy) {
        self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_army(v))
    }

    // Mutable pointer to the field.
    pub fn mut_select_army(&mut self) -> &mut ActionSelectArmy {
        if let ::std::option::Option::Some(ActionUI_oneof_action::select_army(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_army(ActionSelectArmy::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_army(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_select_army(&mut self) -> ActionSelectArmy {
        if self.has_select_army() {
            match self.action.take() {
                ::std::option::Option::Some(ActionUI_oneof_action::select_army(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionSelectArmy::new()
        }
    }

    pub fn get_select_army(&self) -> &ActionSelectArmy {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_army(ref v)) => v,
            _ => ActionSelectArmy::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionSelectWarpGates select_warp_gates = 3;

    pub fn clear_select_warp_gates(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_select_warp_gates(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_warp_gates(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_select_warp_gates(&mut self, v: ActionSelectWarpGates) {
        self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_warp_gates(v))
    }

    // Mutable pointer to the field.
    pub fn mut_select_warp_gates(&mut self) -> &mut ActionSelectWarpGates {
        if let ::std::option::Option::Some(ActionUI_oneof_action::select_warp_gates(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_warp_gates(ActionSelectWarpGates::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_warp_gates(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_select_warp_gates(&mut self) -> ActionSelectWarpGates {
        if self.has_select_warp_gates() {
            match self.action.take() {
                ::std::option::Option::Some(ActionUI_oneof_action::select_warp_gates(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionSelectWarpGates::new()
        }
    }

    pub fn get_select_warp_gates(&self) -> &ActionSelectWarpGates {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_warp_gates(ref v)) => v,
            _ => ActionSelectWarpGates::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionSelectLarva select_larva = 4;

    pub fn clear_select_larva(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_select_larva(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_larva(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_select_larva(&mut self, v: ActionSelectLarva) {
        self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_larva(v))
    }

    // Mutable pointer to the field.
    pub fn mut_select_larva(&mut self) -> &mut ActionSelectLarva {
        if let ::std::option::Option::Some(ActionUI_oneof_action::select_larva(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_larva(ActionSelectLarva::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_larva(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_select_larva(&mut self) -> ActionSelectLarva {
        if self.has_select_larva() {
            match self.action.take() {
                ::std::option::Option::Some(ActionUI_oneof_action::select_larva(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionSelectLarva::new()
        }
    }

    pub fn get_select_larva(&self) -> &ActionSelectLarva {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_larva(ref v)) => v,
            _ => ActionSelectLarva::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionSelectIdleWorker select_idle_worker = 5;

    pub fn clear_select_idle_worker(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_select_idle_worker(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_idle_worker(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_select_idle_worker(&mut self, v: ActionSelectIdleWorker) {
        self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_idle_worker(v))
    }

    // Mutable pointer to the field.
    pub fn mut_select_idle_worker(&mut self) -> &mut ActionSelectIdleWorker {
        if let ::std::option::Option::Some(ActionUI_oneof_action::select_idle_worker(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_idle_worker(ActionSelectIdleWorker::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_idle_worker(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_select_idle_worker(&mut self) -> ActionSelectIdleWorker {
        if self.has_select_idle_worker() {
            match self.action.take() {
                ::std::option::Option::Some(ActionUI_oneof_action::select_idle_worker(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionSelectIdleWorker::new()
        }
    }

    pub fn get_select_idle_worker(&self) -> &ActionSelectIdleWorker {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::select_idle_worker(ref v)) => v,
            _ => ActionSelectIdleWorker::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionMultiPanel multi_panel = 6;

    pub fn clear_multi_panel(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_multi_panel(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::multi_panel(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_multi_panel(&mut self, v: ActionMultiPanel) {
        self.action = ::std::option::Option::Some(ActionUI_oneof_action::multi_panel(v))
    }

    // Mutable pointer to the field.
    pub fn mut_multi_panel(&mut self) -> &mut ActionMultiPanel {
        if let ::std::option::Option::Some(ActionUI_oneof_action::multi_panel(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionUI_oneof_action::multi_panel(ActionMultiPanel::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::multi_panel(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_multi_panel(&mut self) -> ActionMultiPanel {
        if self.has_multi_panel() {
            match self.action.take() {
                ::std::option::Option::Some(ActionUI_oneof_action::multi_panel(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionMultiPanel::new()
        }
    }

    pub fn get_multi_panel(&self) -> &ActionMultiPanel {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::multi_panel(ref v)) => v,
            _ => ActionMultiPanel::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionCargoPanelUnload cargo_panel = 7;

    pub fn clear_cargo_panel(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_cargo_panel(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::cargo_panel(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cargo_panel(&mut self, v: ActionCargoPanelUnload) {
        self.action = ::std::option::Option::Some(ActionUI_oneof_action::cargo_panel(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cargo_panel(&mut self) -> &mut ActionCargoPanelUnload {
        if let ::std::option::Option::Some(ActionUI_oneof_action::cargo_panel(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionUI_oneof_action::cargo_panel(ActionCargoPanelUnload::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::cargo_panel(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cargo_panel(&mut self) -> ActionCargoPanelUnload {
        if self.has_cargo_panel() {
            match self.action.take() {
                ::std::option::Option::Some(ActionUI_oneof_action::cargo_panel(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionCargoPanelUnload::new()
        }
    }

    pub fn get_cargo_panel(&self) -> &ActionCargoPanelUnload {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::cargo_panel(ref v)) => v,
            _ => ActionCargoPanelUnload::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionProductionPanelRemoveFromQueue production_panel = 8;

    pub fn clear_production_panel(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_production_panel(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::production_panel(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_production_panel(&mut self, v: ActionProductionPanelRemoveFromQueue) {
        self.action = ::std::option::Option::Some(ActionUI_oneof_action::production_panel(v))
    }

    // Mutable pointer to the field.
    pub fn mut_production_panel(&mut self) -> &mut ActionProductionPanelRemoveFromQueue {
        if let ::std::option::Option::Some(ActionUI_oneof_action::production_panel(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionUI_oneof_action::production_panel(ActionProductionPanelRemoveFromQueue::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::production_panel(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_production_panel(&mut self) -> ActionProductionPanelRemoveFromQueue {
        if self.has_production_panel() {
            match self.action.take() {
                ::std::option::Option::Some(ActionUI_oneof_action::production_panel(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionProductionPanelRemoveFromQueue::new()
        }
    }

    pub fn get_production_panel(&self) -> &ActionProductionPanelRemoveFromQueue {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::production_panel(ref v)) => v,
            _ => ActionProductionPanelRemoveFromQueue::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ActionToggleAutocast toggle_autocast = 9;

    pub fn clear_toggle_autocast(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_toggle_autocast(&self) -> bool {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::toggle_autocast(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_toggle_autocast(&mut self, v: ActionToggleAutocast) {
        self.action = ::std::option::Option::Some(ActionUI_oneof_action::toggle_autocast(v))
    }

    // Mutable pointer to the field.
    pub fn mut_toggle_autocast(&mut self) -> &mut ActionToggleAutocast {
        if let ::std::option::Option::Some(ActionUI_oneof_action::toggle_autocast(_)) = self.action {
        } else {
            self.action = ::std::option::Option::Some(ActionUI_oneof_action::toggle_autocast(ActionToggleAutocast::new()));
        }
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::toggle_autocast(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_toggle_autocast(&mut self) -> ActionToggleAutocast {
        if self.has_toggle_autocast() {
            match self.action.take() {
                ::std::option::Option::Some(ActionUI_oneof_action::toggle_autocast(v)) => v,
                _ => panic!(),
            }
        } else {
            ActionToggleAutocast::new()
        }
    }

    pub fn get_toggle_autocast(&self) -> &ActionToggleAutocast {
        match self.action {
            ::std::option::Option::Some(ActionUI_oneof_action::toggle_autocast(ref v)) => v,
            _ => ActionToggleAutocast::default_instance(),
        }
    }
}

impl ::protobuf::Message for ActionUI {
    fn is_initialized(&self) -> bool {
        if let Some(ActionUI_oneof_action::control_group(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionUI_oneof_action::select_army(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionUI_oneof_action::select_warp_gates(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionUI_oneof_action::select_larva(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionUI_oneof_action::select_idle_worker(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionUI_oneof_action::multi_panel(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionUI_oneof_action::cargo_panel(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionUI_oneof_action::production_panel(ref v)) = self.action {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(ActionUI_oneof_action::toggle_autocast(ref v)) = self.action {
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
                    self.action = ::std::option::Option::Some(ActionUI_oneof_action::control_group(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_army(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_warp_gates(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_larva(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionUI_oneof_action::select_idle_worker(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionUI_oneof_action::multi_panel(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionUI_oneof_action::cargo_panel(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionUI_oneof_action::production_panel(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.action = ::std::option::Option::Some(ActionUI_oneof_action::toggle_autocast(is.read_message()?));
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
                &ActionUI_oneof_action::control_group(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionUI_oneof_action::select_army(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionUI_oneof_action::select_warp_gates(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionUI_oneof_action::select_larva(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionUI_oneof_action::select_idle_worker(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionUI_oneof_action::multi_panel(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionUI_oneof_action::cargo_panel(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionUI_oneof_action::production_panel(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &ActionUI_oneof_action::toggle_autocast(ref v) => {
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
                &ActionUI_oneof_action::control_group(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionUI_oneof_action::select_army(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionUI_oneof_action::select_warp_gates(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionUI_oneof_action::select_larva(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionUI_oneof_action::select_idle_worker(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionUI_oneof_action::multi_panel(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionUI_oneof_action::cargo_panel(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionUI_oneof_action::production_panel(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &ActionUI_oneof_action::toggle_autocast(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ActionUI {
    fn new() -> ActionUI {
        ActionUI::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionUI>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionControlGroup>(
                    "control_group",
                    ActionUI::has_control_group,
                    ActionUI::get_control_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionSelectArmy>(
                    "select_army",
                    ActionUI::has_select_army,
                    ActionUI::get_select_army,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionSelectWarpGates>(
                    "select_warp_gates",
                    ActionUI::has_select_warp_gates,
                    ActionUI::get_select_warp_gates,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionSelectLarva>(
                    "select_larva",
                    ActionUI::has_select_larva,
                    ActionUI::get_select_larva,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionSelectIdleWorker>(
                    "select_idle_worker",
                    ActionUI::has_select_idle_worker,
                    ActionUI::get_select_idle_worker,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionMultiPanel>(
                    "multi_panel",
                    ActionUI::has_multi_panel,
                    ActionUI::get_multi_panel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionCargoPanelUnload>(
                    "cargo_panel",
                    ActionUI::has_cargo_panel,
                    ActionUI::get_cargo_panel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionProductionPanelRemoveFromQueue>(
                    "production_panel",
                    ActionUI::has_production_panel,
                    ActionUI::get_production_panel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ActionToggleAutocast>(
                    "toggle_autocast",
                    ActionUI::has_toggle_autocast,
                    ActionUI::get_toggle_autocast,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionUI>(
                    "ActionUI",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionUI {
    fn clear(&mut self) {
        self.clear_control_group();
        self.clear_select_army();
        self.clear_select_warp_gates();
        self.clear_select_larva();
        self.clear_select_idle_worker();
        self.clear_multi_panel();
        self.clear_cargo_panel();
        self.clear_production_panel();
        self.clear_toggle_autocast();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionUI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionUI {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionControlGroup {
    // message fields
    action: ::std::option::Option<ActionControlGroup_ControlGroupAction>,
    control_group_index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionControlGroup {}

impl ActionControlGroup {
    pub fn new() -> ActionControlGroup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionControlGroup {
        static mut instance: ::protobuf::lazy::Lazy<ActionControlGroup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionControlGroup,
        };
        unsafe {
            instance.get(ActionControlGroup::new)
        }
    }

    // optional .SC2APIProtocol.ActionControlGroup.ControlGroupAction action = 1;

    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: ActionControlGroup_ControlGroupAction) {
        self.action = ::std::option::Option::Some(v);
    }

    pub fn get_action(&self) -> ActionControlGroup_ControlGroupAction {
        self.action.unwrap_or(ActionControlGroup_ControlGroupAction::Recall)
    }

    fn get_action_for_reflect(&self) -> &::std::option::Option<ActionControlGroup_ControlGroupAction> {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut ::std::option::Option<ActionControlGroup_ControlGroupAction> {
        &mut self.action
    }

    // optional uint32 control_group_index = 2;

    pub fn clear_control_group_index(&mut self) {
        self.control_group_index = ::std::option::Option::None;
    }

    pub fn has_control_group_index(&self) -> bool {
        self.control_group_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_control_group_index(&mut self, v: u32) {
        self.control_group_index = ::std::option::Option::Some(v);
    }

    pub fn get_control_group_index(&self) -> u32 {
        self.control_group_index.unwrap_or(0)
    }

    fn get_control_group_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.control_group_index
    }

    fn mut_control_group_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.control_group_index
    }
}

impl ::protobuf::Message for ActionControlGroup {
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
                    self.action = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.control_group_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.control_group_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.action {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.control_group_index {
            os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for ActionControlGroup {
    fn new() -> ActionControlGroup {
        ActionControlGroup::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionControlGroup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ActionControlGroup_ControlGroupAction>>(
                    "action",
                    ActionControlGroup::get_action_for_reflect,
                    ActionControlGroup::mut_action_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "control_group_index",
                    ActionControlGroup::get_control_group_index_for_reflect,
                    ActionControlGroup::mut_control_group_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionControlGroup>(
                    "ActionControlGroup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionControlGroup {
    fn clear(&mut self) {
        self.clear_action();
        self.clear_control_group_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionControlGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionControlGroup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ActionControlGroup_ControlGroupAction {
    Recall = 1,
    Set = 2,
    Append = 3,
    SetAndSteal = 4,
    AppendAndSteal = 5,
}

impl ::protobuf::ProtobufEnum for ActionControlGroup_ControlGroupAction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ActionControlGroup_ControlGroupAction> {
        match value {
            1 => ::std::option::Option::Some(ActionControlGroup_ControlGroupAction::Recall),
            2 => ::std::option::Option::Some(ActionControlGroup_ControlGroupAction::Set),
            3 => ::std::option::Option::Some(ActionControlGroup_ControlGroupAction::Append),
            4 => ::std::option::Option::Some(ActionControlGroup_ControlGroupAction::SetAndSteal),
            5 => ::std::option::Option::Some(ActionControlGroup_ControlGroupAction::AppendAndSteal),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ActionControlGroup_ControlGroupAction] = &[
            ActionControlGroup_ControlGroupAction::Recall,
            ActionControlGroup_ControlGroupAction::Set,
            ActionControlGroup_ControlGroupAction::Append,
            ActionControlGroup_ControlGroupAction::SetAndSteal,
            ActionControlGroup_ControlGroupAction::AppendAndSteal,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ActionControlGroup_ControlGroupAction>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ActionControlGroup_ControlGroupAction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ActionControlGroup_ControlGroupAction {
}

impl ::protobuf::reflect::ProtobufValue for ActionControlGroup_ControlGroupAction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionSelectArmy {
    // message fields
    selection_add: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionSelectArmy {}

impl ActionSelectArmy {
    pub fn new() -> ActionSelectArmy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionSelectArmy {
        static mut instance: ::protobuf::lazy::Lazy<ActionSelectArmy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionSelectArmy,
        };
        unsafe {
            instance.get(ActionSelectArmy::new)
        }
    }

    // optional bool selection_add = 1;

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

impl ::protobuf::Message for ActionSelectArmy {
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
        if let Some(v) = self.selection_add {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.selection_add {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for ActionSelectArmy {
    fn new() -> ActionSelectArmy {
        ActionSelectArmy::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionSelectArmy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "selection_add",
                    ActionSelectArmy::get_selection_add_for_reflect,
                    ActionSelectArmy::mut_selection_add_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionSelectArmy>(
                    "ActionSelectArmy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionSelectArmy {
    fn clear(&mut self) {
        self.clear_selection_add();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionSelectArmy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionSelectArmy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionSelectWarpGates {
    // message fields
    selection_add: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionSelectWarpGates {}

impl ActionSelectWarpGates {
    pub fn new() -> ActionSelectWarpGates {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionSelectWarpGates {
        static mut instance: ::protobuf::lazy::Lazy<ActionSelectWarpGates> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionSelectWarpGates,
        };
        unsafe {
            instance.get(ActionSelectWarpGates::new)
        }
    }

    // optional bool selection_add = 1;

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

impl ::protobuf::Message for ActionSelectWarpGates {
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
        if let Some(v) = self.selection_add {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.selection_add {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for ActionSelectWarpGates {
    fn new() -> ActionSelectWarpGates {
        ActionSelectWarpGates::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionSelectWarpGates>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "selection_add",
                    ActionSelectWarpGates::get_selection_add_for_reflect,
                    ActionSelectWarpGates::mut_selection_add_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionSelectWarpGates>(
                    "ActionSelectWarpGates",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionSelectWarpGates {
    fn clear(&mut self) {
        self.clear_selection_add();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionSelectWarpGates {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionSelectWarpGates {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionSelectLarva {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionSelectLarva {}

impl ActionSelectLarva {
    pub fn new() -> ActionSelectLarva {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionSelectLarva {
        static mut instance: ::protobuf::lazy::Lazy<ActionSelectLarva> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionSelectLarva,
        };
        unsafe {
            instance.get(ActionSelectLarva::new)
        }
    }
}

impl ::protobuf::Message for ActionSelectLarva {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ActionSelectLarva {
    fn new() -> ActionSelectLarva {
        ActionSelectLarva::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionSelectLarva>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ActionSelectLarva>(
                    "ActionSelectLarva",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionSelectLarva {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionSelectLarva {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionSelectLarva {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionSelectIdleWorker {
    // message fields
    field_type: ::std::option::Option<ActionSelectIdleWorker_Type>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionSelectIdleWorker {}

impl ActionSelectIdleWorker {
    pub fn new() -> ActionSelectIdleWorker {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionSelectIdleWorker {
        static mut instance: ::protobuf::lazy::Lazy<ActionSelectIdleWorker> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionSelectIdleWorker,
        };
        unsafe {
            instance.get(ActionSelectIdleWorker::new)
        }
    }

    // optional .SC2APIProtocol.ActionSelectIdleWorker.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ActionSelectIdleWorker_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ActionSelectIdleWorker_Type {
        self.field_type.unwrap_or(ActionSelectIdleWorker_Type::Set)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<ActionSelectIdleWorker_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<ActionSelectIdleWorker_Type> {
        &mut self.field_type
    }
}

impl ::protobuf::Message for ActionSelectIdleWorker {
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
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

impl ::protobuf::MessageStatic for ActionSelectIdleWorker {
    fn new() -> ActionSelectIdleWorker {
        ActionSelectIdleWorker::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionSelectIdleWorker>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ActionSelectIdleWorker_Type>>(
                    "type",
                    ActionSelectIdleWorker::get_field_type_for_reflect,
                    ActionSelectIdleWorker::mut_field_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionSelectIdleWorker>(
                    "ActionSelectIdleWorker",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionSelectIdleWorker {
    fn clear(&mut self) {
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionSelectIdleWorker {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionSelectIdleWorker {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ActionSelectIdleWorker_Type {
    Set = 1,
    Add = 2,
    All = 3,
    AddAll = 4,
}

impl ::protobuf::ProtobufEnum for ActionSelectIdleWorker_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ActionSelectIdleWorker_Type> {
        match value {
            1 => ::std::option::Option::Some(ActionSelectIdleWorker_Type::Set),
            2 => ::std::option::Option::Some(ActionSelectIdleWorker_Type::Add),
            3 => ::std::option::Option::Some(ActionSelectIdleWorker_Type::All),
            4 => ::std::option::Option::Some(ActionSelectIdleWorker_Type::AddAll),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ActionSelectIdleWorker_Type] = &[
            ActionSelectIdleWorker_Type::Set,
            ActionSelectIdleWorker_Type::Add,
            ActionSelectIdleWorker_Type::All,
            ActionSelectIdleWorker_Type::AddAll,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ActionSelectIdleWorker_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ActionSelectIdleWorker_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ActionSelectIdleWorker_Type {
}

impl ::protobuf::reflect::ProtobufValue for ActionSelectIdleWorker_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionMultiPanel {
    // message fields
    field_type: ::std::option::Option<ActionMultiPanel_Type>,
    unit_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionMultiPanel {}

impl ActionMultiPanel {
    pub fn new() -> ActionMultiPanel {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionMultiPanel {
        static mut instance: ::protobuf::lazy::Lazy<ActionMultiPanel> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionMultiPanel,
        };
        unsafe {
            instance.get(ActionMultiPanel::new)
        }
    }

    // optional .SC2APIProtocol.ActionMultiPanel.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ActionMultiPanel_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ActionMultiPanel_Type {
        self.field_type.unwrap_or(ActionMultiPanel_Type::SingleSelect)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<ActionMultiPanel_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<ActionMultiPanel_Type> {
        &mut self.field_type
    }

    // optional int32 unit_index = 2;

    pub fn clear_unit_index(&mut self) {
        self.unit_index = ::std::option::Option::None;
    }

    pub fn has_unit_index(&self) -> bool {
        self.unit_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_index(&mut self, v: i32) {
        self.unit_index = ::std::option::Option::Some(v);
    }

    pub fn get_unit_index(&self) -> i32 {
        self.unit_index.unwrap_or(0)
    }

    fn get_unit_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.unit_index
    }

    fn mut_unit_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.unit_index
    }
}

impl ::protobuf::Message for ActionMultiPanel {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.unit_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.unit_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.unit_index {
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

impl ::protobuf::MessageStatic for ActionMultiPanel {
    fn new() -> ActionMultiPanel {
        ActionMultiPanel::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionMultiPanel>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ActionMultiPanel_Type>>(
                    "type",
                    ActionMultiPanel::get_field_type_for_reflect,
                    ActionMultiPanel::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "unit_index",
                    ActionMultiPanel::get_unit_index_for_reflect,
                    ActionMultiPanel::mut_unit_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionMultiPanel>(
                    "ActionMultiPanel",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionMultiPanel {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_unit_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionMultiPanel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionMultiPanel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ActionMultiPanel_Type {
    SingleSelect = 1,
    DeselectUnit = 2,
    SelectAllOfType = 3,
    DeselectAllOfType = 4,
}

impl ::protobuf::ProtobufEnum for ActionMultiPanel_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ActionMultiPanel_Type> {
        match value {
            1 => ::std::option::Option::Some(ActionMultiPanel_Type::SingleSelect),
            2 => ::std::option::Option::Some(ActionMultiPanel_Type::DeselectUnit),
            3 => ::std::option::Option::Some(ActionMultiPanel_Type::SelectAllOfType),
            4 => ::std::option::Option::Some(ActionMultiPanel_Type::DeselectAllOfType),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ActionMultiPanel_Type] = &[
            ActionMultiPanel_Type::SingleSelect,
            ActionMultiPanel_Type::DeselectUnit,
            ActionMultiPanel_Type::SelectAllOfType,
            ActionMultiPanel_Type::DeselectAllOfType,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ActionMultiPanel_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ActionMultiPanel_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ActionMultiPanel_Type {
}

impl ::protobuf::reflect::ProtobufValue for ActionMultiPanel_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionCargoPanelUnload {
    // message fields
    unit_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionCargoPanelUnload {}

impl ActionCargoPanelUnload {
    pub fn new() -> ActionCargoPanelUnload {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionCargoPanelUnload {
        static mut instance: ::protobuf::lazy::Lazy<ActionCargoPanelUnload> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionCargoPanelUnload,
        };
        unsafe {
            instance.get(ActionCargoPanelUnload::new)
        }
    }

    // optional int32 unit_index = 1;

    pub fn clear_unit_index(&mut self) {
        self.unit_index = ::std::option::Option::None;
    }

    pub fn has_unit_index(&self) -> bool {
        self.unit_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_index(&mut self, v: i32) {
        self.unit_index = ::std::option::Option::Some(v);
    }

    pub fn get_unit_index(&self) -> i32 {
        self.unit_index.unwrap_or(0)
    }

    fn get_unit_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.unit_index
    }

    fn mut_unit_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.unit_index
    }
}

impl ::protobuf::Message for ActionCargoPanelUnload {
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
                    self.unit_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.unit_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_index {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for ActionCargoPanelUnload {
    fn new() -> ActionCargoPanelUnload {
        ActionCargoPanelUnload::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionCargoPanelUnload>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "unit_index",
                    ActionCargoPanelUnload::get_unit_index_for_reflect,
                    ActionCargoPanelUnload::mut_unit_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionCargoPanelUnload>(
                    "ActionCargoPanelUnload",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionCargoPanelUnload {
    fn clear(&mut self) {
        self.clear_unit_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionCargoPanelUnload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionCargoPanelUnload {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionProductionPanelRemoveFromQueue {
    // message fields
    unit_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionProductionPanelRemoveFromQueue {}

impl ActionProductionPanelRemoveFromQueue {
    pub fn new() -> ActionProductionPanelRemoveFromQueue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionProductionPanelRemoveFromQueue {
        static mut instance: ::protobuf::lazy::Lazy<ActionProductionPanelRemoveFromQueue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionProductionPanelRemoveFromQueue,
        };
        unsafe {
            instance.get(ActionProductionPanelRemoveFromQueue::new)
        }
    }

    // optional int32 unit_index = 1;

    pub fn clear_unit_index(&mut self) {
        self.unit_index = ::std::option::Option::None;
    }

    pub fn has_unit_index(&self) -> bool {
        self.unit_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_index(&mut self, v: i32) {
        self.unit_index = ::std::option::Option::Some(v);
    }

    pub fn get_unit_index(&self) -> i32 {
        self.unit_index.unwrap_or(0)
    }

    fn get_unit_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.unit_index
    }

    fn mut_unit_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.unit_index
    }
}

impl ::protobuf::Message for ActionProductionPanelRemoveFromQueue {
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
                    self.unit_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.unit_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_index {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for ActionProductionPanelRemoveFromQueue {
    fn new() -> ActionProductionPanelRemoveFromQueue {
        ActionProductionPanelRemoveFromQueue::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionProductionPanelRemoveFromQueue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "unit_index",
                    ActionProductionPanelRemoveFromQueue::get_unit_index_for_reflect,
                    ActionProductionPanelRemoveFromQueue::mut_unit_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionProductionPanelRemoveFromQueue>(
                    "ActionProductionPanelRemoveFromQueue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionProductionPanelRemoveFromQueue {
    fn clear(&mut self) {
        self.clear_unit_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionProductionPanelRemoveFromQueue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionProductionPanelRemoveFromQueue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionToggleAutocast {
    // message fields
    ability_id: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionToggleAutocast {}

impl ActionToggleAutocast {
    pub fn new() -> ActionToggleAutocast {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionToggleAutocast {
        static mut instance: ::protobuf::lazy::Lazy<ActionToggleAutocast> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionToggleAutocast,
        };
        unsafe {
            instance.get(ActionToggleAutocast::new)
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
}

impl ::protobuf::Message for ActionToggleAutocast {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_id {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for ActionToggleAutocast {
    fn new() -> ActionToggleAutocast {
        ActionToggleAutocast::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionToggleAutocast>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ability_id",
                    ActionToggleAutocast::get_ability_id_for_reflect,
                    ActionToggleAutocast::mut_ability_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionToggleAutocast>(
                    "ActionToggleAutocast",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionToggleAutocast {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionToggleAutocast {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionToggleAutocast {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19s2clientprotocol/ui.proto\x12\x0eSC2APIProtocol\"\xb0\x02\n\rObser\
    vationUI\x124\n\x06groups\x18\x01\x20\x03(\x0b2\x1c.SC2APIProtocol.Contr\
    olGroupR\x06groups\x125\n\x06single\x18\x02\x20\x01(\x0b2\x1b.SC2APIProt\
    ocol.SinglePanelH\0R\x06single\x122\n\x05multi\x18\x03\x20\x01(\x0b2\x1a\
    .SC2APIProtocol.MultiPanelH\0R\x05multi\x122\n\x05cargo\x18\x04\x20\x01(\
    \x0b2\x1a.SC2APIProtocol.CargoPanelH\0R\x05cargo\x12A\n\nproduction\x18\
    \x05\x20\x01(\x0b2\x1f.SC2APIProtocol.ProductionPanelH\0R\nproductionB\
    \x07\n\x05panel\"~\n\x0cControlGroup\x12.\n\x13control_group_index\x18\
    \x01\x20\x01(\rR\x11controlGroupIndex\x12(\n\x10leader_unit_type\x18\x02\
    \x20\x01(\rR\x0eleaderUnitType\x12\x14\n\x05count\x18\x03\x20\x01(\rR\
    \x05count\"\xa6\x02\n\x08UnitInfo\x12\x1b\n\tunit_type\x18\x01\x20\x01(\
    \rR\x08unitType\x12'\n\x0fplayer_relative\x18\x02\x20\x01(\rR\x0eplayerR\
    elative\x12\x16\n\x06health\x18\x03\x20\x01(\x05R\x06health\x12\x18\n\
    \x07shields\x18\x04\x20\x01(\x05R\x07shields\x12\x16\n\x06energy\x18\x05\
    \x20\x01(\x05R\x06energy\x122\n\x15transport_slots_taken\x18\x06\x20\x01\
    (\x05R\x13transportSlotsTaken\x12%\n\x0ebuild_progress\x18\x07\x20\x01(\
    \x02R\rbuildProgress\x12/\n\x06add_on\x18\x08\x20\x01(\x0b2\x18.SC2APIPr\
    otocol.UnitInfoR\x05addOn\";\n\x0bSinglePanel\x12,\n\x04unit\x18\x01\x20\
    \x01(\x0b2\x18.SC2APIProtocol.UnitInfoR\x04unit\"<\n\nMultiPanel\x12.\n\
    \x05units\x18\x01\x20\x03(\x0b2\x18.SC2APIProtocol.UnitInfoR\x05units\"\
    \x9d\x01\n\nCargoPanel\x12,\n\x04unit\x18\x01\x20\x01(\x0b2\x18.SC2APIPr\
    otocol.UnitInfoR\x04unit\x128\n\npassengers\x18\x02\x20\x03(\x0b2\x18.SC\
    2APIProtocol.UnitInfoR\npassengers\x12'\n\x0fslots_available\x18\x03\x20\
    \x01(\x05R\x0eslotsAvailable\"z\n\x0fProductionPanel\x12,\n\x04unit\x18\
    \x01\x20\x01(\x0b2\x18.SC2APIProtocol.UnitInfoR\x04unit\x129\n\x0bbuild_\
    queue\x18\x02\x20\x03(\x0b2\x18.SC2APIProtocol.UnitInfoR\nbuildQueue\"\
    \xdd\x05\n\x08ActionUI\x12I\n\rcontrol_group\x18\x01\x20\x01(\x0b2\".SC2\
    APIProtocol.ActionControlGroupH\0R\x0ccontrolGroup\x12C\n\x0bselect_army\
    \x18\x02\x20\x01(\x0b2\x20.SC2APIProtocol.ActionSelectArmyH\0R\nselectAr\
    my\x12S\n\x11select_warp_gates\x18\x03\x20\x01(\x0b2%.SC2APIProtocol.Act\
    ionSelectWarpGatesH\0R\x0fselectWarpGates\x12F\n\x0cselect_larva\x18\x04\
    \x20\x01(\x0b2!.SC2APIProtocol.ActionSelectLarvaH\0R\x0bselectLarva\x12V\
    \n\x12select_idle_worker\x18\x05\x20\x01(\x0b2&.SC2APIProtocol.ActionSel\
    ectIdleWorkerH\0R\x10selectIdleWorker\x12C\n\x0bmulti_panel\x18\x06\x20\
    \x01(\x0b2\x20.SC2APIProtocol.ActionMultiPanelH\0R\nmultiPanel\x12I\n\
    \x0bcargo_panel\x18\x07\x20\x01(\x0b2&.SC2APIProtocol.ActionCargoPanelUn\
    loadH\0R\ncargoPanel\x12a\n\x10production_panel\x18\x08\x20\x01(\x0b24.S\
    C2APIProtocol.ActionProductionPanelRemoveFromQueueH\0R\x0fproductionPane\
    l\x12O\n\x0ftoggle_autocast\x18\t\x20\x01(\x0b2$.SC2APIProtocol.ActionTo\
    ggleAutocastH\0R\x0etoggleAutocastB\x08\n\x06action\"\xef\x01\n\x12Actio\
    nControlGroup\x12M\n\x06action\x18\x01\x20\x01(\x0e25.SC2APIProtocol.Act\
    ionControlGroup.ControlGroupActionR\x06action\x12.\n\x13control_group_in\
    dex\x18\x02\x20\x01(\rR\x11controlGroupIndex\"Z\n\x12ControlGroupAction\
    \x12\n\n\x06Recall\x10\x01\x12\x07\n\x03Set\x10\x02\x12\n\n\x06Append\
    \x10\x03\x12\x0f\n\x0bSetAndSteal\x10\x04\x12\x12\n\x0eAppendAndSteal\
    \x10\x05\"7\n\x10ActionSelectArmy\x12#\n\rselection_add\x18\x01\x20\x01(\
    \x08R\x0cselectionAdd\"<\n\x15ActionSelectWarpGates\x12#\n\rselection_ad\
    d\x18\x01\x20\x01(\x08R\x0cselectionAdd\"\x13\n\x11ActionSelectLarva\"\
    \x88\x01\n\x16ActionSelectIdleWorker\x12?\n\x04type\x18\x01\x20\x01(\x0e\
    2+.SC2APIProtocol.ActionSelectIdleWorker.TypeR\x04type\"-\n\x04Type\x12\
    \x07\n\x03Set\x10\x01\x12\x07\n\x03Add\x10\x02\x12\x07\n\x03All\x10\x03\
    \x12\n\n\x06AddAll\x10\x04\"\xc4\x01\n\x10ActionMultiPanel\x129\n\x04typ\
    e\x18\x01\x20\x01(\x0e2%.SC2APIProtocol.ActionMultiPanel.TypeR\x04type\
    \x12\x1d\n\nunit_index\x18\x02\x20\x01(\x05R\tunitIndex\"V\n\x04Type\x12\
    \x10\n\x0cSingleSelect\x10\x01\x12\x10\n\x0cDeselectUnit\x10\x02\x12\x13\
    \n\x0fSelectAllOfType\x10\x03\x12\x15\n\x11DeselectAllOfType\x10\x04\"7\
    \n\x16ActionCargoPanelUnload\x12\x1d\n\nunit_index\x18\x01\x20\x01(\x05R\
    \tunitIndex\"E\n$ActionProductionPanelRemoveFromQueue\x12\x1d\n\nunit_in\
    dex\x18\x01\x20\x01(\x05R\tunitIndex\"5\n\x14ActionToggleAutocast\x12\
    \x1d\n\nability_id\x18\x01\x20\x01(\x05R\tabilityIdJ\xc2&\n\x07\x12\x05\
    \x01\0\x82\x01\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\x08\n\x01\x02\x12\
    \x03\x03\x08\x16\n\x1b\n\x02\x04\0\x12\x04\t\0\x11\x012\x0f\n\x20Observa\
    tion\n\n\n\n\n\x03\x04\0\x01\x12\x03\t\x08\x15\n\x0b\n\x04\x04\0\x02\0\
    \x12\x03\n\x02#\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\n\x02\n\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x03\n\x0b\x17\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\n\
    \x18\x1e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\n!\"\n\x0c\n\x04\x04\0\x08\
    \0\x12\x04\x0b\x02\x10\x03\n\x0c\n\x05\x04\0\x08\0\x01\x12\x03\x0b\x08\r\
    \n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0c\x04\x1b\n\x0c\n\x05\x04\0\x02\x01\
    \x06\x12\x03\x0c\x04\x0f\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0c\x10\
    \x16\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0c\x19\x1a\n\x0b\n\x04\x04\0\
    \x02\x02\x12\x03\r\x04\x19\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\r\x04\
    \x0e\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\r\x0f\x14\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x03\r\x17\x18\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x0e\x04\
    \x19\n\x0c\n\x05\x04\0\x02\x03\x06\x12\x03\x0e\x04\x0e\n\x0c\n\x05\x04\0\
    \x02\x03\x01\x12\x03\x0e\x0f\x14\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\
    \x0e\x17\x18\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x0f\x04#\n\x0c\n\x05\x04\
    \0\x02\x04\x06\x12\x03\x0f\x04\x13\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\
    \x0f\x14\x1e\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x0f!\"\n\n\n\x02\x04\
    \x01\x12\x04\x13\0\x17\x01\n\n\n\x03\x04\x01\x01\x12\x03\x13\x08\x14\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x03\x14\x02*\n\x0c\n\x05\x04\x01\x02\0\x04\
    \x12\x03\x14\x02\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x14\x0b\x11\n\
    \x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x14\x12%\n\x0c\n\x05\x04\x01\x02\0\
    \x03\x12\x03\x14()\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x15\x02'\n\x0c\n\
    \x05\x04\x01\x02\x01\x04\x12\x03\x15\x02\n\n\x0c\n\x05\x04\x01\x02\x01\
    \x05\x12\x03\x15\x0b\x11\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x15\x12\
    \"\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x15%&\n\x0b\n\x04\x04\x01\x02\
    \x02\x12\x03\x16\x02\x1c\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\x16\x02\
    \n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x16\x0b\x11\n\x0c\n\x05\x04\
    \x01\x02\x02\x01\x12\x03\x16\x12\x17\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\
    \x03\x16\x1a\x1b\n\n\n\x02\x04\x02\x12\x04\x19\0\"\x01\n\n\n\x03\x04\x02\
    \x01\x12\x03\x19\x08\x10\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1a\x02\x20\n\
    \x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\x02\x02\0\
    \x05\x12\x03\x1a\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1a\x12\
    \x1b\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1a\x1e\x1f\n\x0b\n\x04\x04\
    \x02\x02\x01\x12\x03\x1b\x02&\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\
    \x1b\x02\n\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x1b\x0b\x11\n\x0c\n\
    \x05\x04\x02\x02\x01\x01\x12\x03\x1b\x12!\n\x0c\n\x05\x04\x02\x02\x01\
    \x03\x12\x03\x1b$%\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x1c\x02\x1c\n\x0c\
    \n\x05\x04\x02\x02\x02\x04\x12\x03\x1c\x02\n\n\x0c\n\x05\x04\x02\x02\x02\
    \x05\x12\x03\x1c\x0b\x10\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x1c\x11\
    \x17\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x1c\x1a\x1b\n\x0b\n\x04\x04\
    \x02\x02\x03\x12\x03\x1d\x02\x1d\n\x0c\n\x05\x04\x02\x02\x03\x04\x12\x03\
    \x1d\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\x1d\x0b\x10\n\x0c\n\
    \x05\x04\x02\x02\x03\x01\x12\x03\x1d\x11\x18\n\x0c\n\x05\x04\x02\x02\x03\
    \x03\x12\x03\x1d\x1b\x1c\n\x0b\n\x04\x04\x02\x02\x04\x12\x03\x1e\x02\x1c\
    \n\x0c\n\x05\x04\x02\x02\x04\x04\x12\x03\x1e\x02\n\n\x0c\n\x05\x04\x02\
    \x02\x04\x05\x12\x03\x1e\x0b\x10\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03\
    \x1e\x11\x17\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03\x1e\x1a\x1b\n\x0b\n\
    \x04\x04\x02\x02\x05\x12\x03\x1f\x02+\n\x0c\n\x05\x04\x02\x02\x05\x04\
    \x12\x03\x1f\x02\n\n\x0c\n\x05\x04\x02\x02\x05\x05\x12\x03\x1f\x0b\x10\n\
    \x0c\n\x05\x04\x02\x02\x05\x01\x12\x03\x1f\x11&\n\x0c\n\x05\x04\x02\x02\
    \x05\x03\x12\x03\x1f)*\n\x20\n\x04\x04\x02\x02\x06\x12\x03\x20\x02$\"\
    \x13\x20Range:\x20[0.0,\x201.0]\n\n\x0c\n\x05\x04\x02\x02\x06\x04\x12\
    \x03\x20\x02\n\n\x0c\n\x05\x04\x02\x02\x06\x05\x12\x03\x20\x0b\x10\n\x0c\
    \n\x05\x04\x02\x02\x06\x01\x12\x03\x20\x11\x1f\n\x0c\n\x05\x04\x02\x02\
    \x06\x03\x12\x03\x20\"#\n\x0b\n\x04\x04\x02\x02\x07\x12\x03!\x02\x1f\n\
    \x0c\n\x05\x04\x02\x02\x07\x04\x12\x03!\x02\n\n\x0c\n\x05\x04\x02\x02\
    \x07\x06\x12\x03!\x0b\x13\n\x0c\n\x05\x04\x02\x02\x07\x01\x12\x03!\x14\
    \x1a\n\x0c\n\x05\x04\x02\x02\x07\x03\x12\x03!\x1d\x1e\n\n\n\x02\x04\x03\
    \x12\x04$\0(\x01\n\n\n\x03\x04\x03\x01\x12\x03$\x08\x13\n\x20\n\x04\x04\
    \x03\x02\0\x12\x03%\x02\x1d\"\x13\x20Upgrades?\n\x20Buffs?\n\n\x0c\n\x05\
    \x04\x03\x02\0\x04\x12\x03%\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03%\
    \x0b\x13\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03%\x14\x18\n\x0c\n\x05\x04\
    \x03\x02\0\x03\x12\x03%\x1b\x1c\n\n\n\x02\x04\x04\x12\x04*\0,\x01\n\n\n\
    \x03\x04\x04\x01\x12\x03*\x08\x12\n\x0b\n\x04\x04\x04\x02\0\x12\x03+\x02\
    \x1e\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03+\x02\n\n\x0c\n\x05\x04\x04\
    \x02\0\x06\x12\x03+\x0b\x13\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03+\x14\
    \x19\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03+\x1c\x1d\n\n\n\x02\x04\x05\
    \x12\x04.\02\x01\n\n\n\x03\x04\x05\x01\x12\x03.\x08\x12\n\x0b\n\x04\x04\
    \x05\x02\0\x12\x03/\x02\x1d\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03/\x02\n\
    \n\x0c\n\x05\x04\x05\x02\0\x06\x12\x03/\x0b\x13\n\x0c\n\x05\x04\x05\x02\
    \0\x01\x12\x03/\x14\x18\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03/\x1b\x1c\n\
    \x0b\n\x04\x04\x05\x02\x01\x12\x030\x02#\n\x0c\n\x05\x04\x05\x02\x01\x04\
    \x12\x030\x02\n\n\x0c\n\x05\x04\x05\x02\x01\x06\x12\x030\x0b\x13\n\x0c\n\
    \x05\x04\x05\x02\x01\x01\x12\x030\x14\x1e\n\x0c\n\x05\x04\x05\x02\x01\
    \x03\x12\x030!\"\n)\n\x04\x04\x05\x02\x02\x12\x031\x02%\"\x1c\x20TODO:\
    \x20Change\x20to\x20cargo\x20size\n\n\x0c\n\x05\x04\x05\x02\x02\x04\x12\
    \x031\x02\n\n\x0c\n\x05\x04\x05\x02\x02\x05\x12\x031\x0b\x10\n\x0c\n\x05\
    \x04\x05\x02\x02\x01\x12\x031\x11\x20\n\x0c\n\x05\x04\x05\x02\x02\x03\
    \x12\x031#$\n\n\n\x02\x04\x06\x12\x044\07\x01\n\n\n\x03\x04\x06\x01\x12\
    \x034\x08\x17\n\x0b\n\x04\x04\x06\x02\0\x12\x035\x02\x1d\n\x0c\n\x05\x04\
    \x06\x02\0\x04\x12\x035\x02\n\n\x0c\n\x05\x04\x06\x02\0\x06\x12\x035\x0b\
    \x13\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x035\x14\x18\n\x0c\n\x05\x04\x06\
    \x02\0\x03\x12\x035\x1b\x1c\n\x0b\n\x04\x04\x06\x02\x01\x12\x036\x02$\n\
    \x0c\n\x05\x04\x06\x02\x01\x04\x12\x036\x02\n\n\x0c\n\x05\x04\x06\x02\
    \x01\x06\x12\x036\x0b\x13\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x036\x14\
    \x1f\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x036\"#\n\x16\n\x02\x04\x07\x12\
    \x04>\0J\x012\n\n\x20Action\n\n\n\n\n\x03\x04\x07\x01\x12\x03>\x08\x10\n\
    \x0c\n\x04\x04\x07\x08\0\x12\x04?\x02I\x03\n\x0c\n\x05\x04\x07\x08\0\x01\
    \x12\x03?\x08\x0e\n\x0b\n\x04\x04\x07\x02\0\x12\x03@\x04)\n\x0c\n\x05\
    \x04\x07\x02\0\x06\x12\x03@\x04\x16\n\x0c\n\x05\x04\x07\x02\0\x01\x12\
    \x03@\x17$\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03@'(\n\x0b\n\x04\x04\x07\
    \x02\x01\x12\x03A\x04%\n\x0c\n\x05\x04\x07\x02\x01\x06\x12\x03A\x04\x14\
    \n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03A\x15\x20\n\x0c\n\x05\x04\x07\
    \x02\x01\x03\x12\x03A#$\n\x0b\n\x04\x04\x07\x02\x02\x12\x03B\x040\n\x0c\
    \n\x05\x04\x07\x02\x02\x06\x12\x03B\x04\x19\n\x0c\n\x05\x04\x07\x02\x02\
    \x01\x12\x03B\x1a+\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\x03B./\n\x0b\n\
    \x04\x04\x07\x02\x03\x12\x03C\x04'\n\x0c\n\x05\x04\x07\x02\x03\x06\x12\
    \x03C\x04\x15\n\x0c\n\x05\x04\x07\x02\x03\x01\x12\x03C\x16\"\n\x0c\n\x05\
    \x04\x07\x02\x03\x03\x12\x03C%&\n\x0b\n\x04\x04\x07\x02\x04\x12\x03D\x04\
    2\n\x0c\n\x05\x04\x07\x02\x04\x06\x12\x03D\x04\x1a\n\x0c\n\x05\x04\x07\
    \x02\x04\x01\x12\x03D\x1b-\n\x0c\n\x05\x04\x07\x02\x04\x03\x12\x03D01\n\
    \x0b\n\x04\x04\x07\x02\x05\x12\x03E\x04%\n\x0c\n\x05\x04\x07\x02\x05\x06\
    \x12\x03E\x04\x14\n\x0c\n\x05\x04\x07\x02\x05\x01\x12\x03E\x15\x20\n\x0c\
    \n\x05\x04\x07\x02\x05\x03\x12\x03E#$\n\x0b\n\x04\x04\x07\x02\x06\x12\
    \x03F\x04+\n\x0c\n\x05\x04\x07\x02\x06\x06\x12\x03F\x04\x1a\n\x0c\n\x05\
    \x04\x07\x02\x06\x01\x12\x03F\x1b&\n\x0c\n\x05\x04\x07\x02\x06\x03\x12\
    \x03F)*\n\x0b\n\x04\x04\x07\x02\x07\x12\x03G\x04>\n\x0c\n\x05\x04\x07\
    \x02\x07\x06\x12\x03G\x04(\n\x0c\n\x05\x04\x07\x02\x07\x01\x12\x03G)9\n\
    \x0c\n\x05\x04\x07\x02\x07\x03\x12\x03G<=\n\x0b\n\x04\x04\x07\x02\x08\
    \x12\x03H\x04-\n\x0c\n\x05\x04\x07\x02\x08\x06\x12\x03H\x04\x18\n\x0c\n\
    \x05\x04\x07\x02\x08\x01\x12\x03H\x19(\n\x0c\n\x05\x04\x07\x02\x08\x03\
    \x12\x03H+,\n\n\n\x02\x04\x08\x12\x04L\0V\x01\n\n\n\x03\x04\x08\x01\x12\
    \x03L\x08\x1a\n\x0c\n\x04\x04\x08\x04\0\x12\x04M\x02S\x03\n\x0c\n\x05\
    \x04\x08\x04\0\x01\x12\x03M\x07\x19\n\\\n\x06\x04\x08\x04\0\x02\0\x12\
    \x03N\x04\x0f\"M\x20Equivalent\x20to\x20number\x20hotkey.\x20Replaces\
    \x20current\x20selection\x20with\x20control\x20group.\n\n\x0e\n\x07\x04\
    \x08\x04\0\x02\0\x01\x12\x03N\x04\n\n\x0e\n\x07\x04\x08\x04\0\x02\0\x02\
    \x12\x03N\r\x0e\n`\n\x06\x04\x08\x04\0\x02\x01\x12\x03O\x04\x0c\"Q\x20Eq\
    uivalent\x20to\x20Control\x20+\x20number\x20hotkey.\x20Sets\x20control\
    \x20group\x20to\x20current\x20selection.\n\n\x0e\n\x07\x04\x08\x04\0\x02\
    \x01\x01\x12\x03O\x04\x07\n\x0e\n\x07\x04\x08\x04\0\x02\x01\x02\x12\x03O\
    \n\x0b\n`\n\x06\x04\x08\x04\0\x02\x02\x12\x03P\x04\x0f\"Q\x20Equivalent\
    \x20to\x20Shift\x20+\x20number\x20hotkey.\x20Adds\x20current\x20selectio\
    n\x20into\x20control\x20group.\n\n\x0e\n\x07\x04\x08\x04\0\x02\x02\x01\
    \x12\x03P\x04\n\n\x0e\n\x07\x04\x08\x04\0\x02\x02\x02\x12\x03P\r\x0e\n\
    \x94\x01\n\x06\x04\x08\x04\0\x02\x03\x12\x03Q\x04\x14\"\x84\x01\x20Equiv\
    alent\x20to\x20Control\x20+\x20Alt\x20+\x20number\x20hotkey.\x20Sets\x20\
    control\x20group\x20to\x20current\x20selection.\x20Units\x20are\x20remov\
    ed\x20from\x20other\x20control\x20groups.\n\n\x0e\n\x07\x04\x08\x04\0\
    \x02\x03\x01\x12\x03Q\x04\x0f\n\x0e\n\x07\x04\x08\x04\0\x02\x03\x02\x12\
    \x03Q\x12\x13\n\x94\x01\n\x06\x04\x08\x04\0\x02\x04\x12\x03R\x04\x17\"\
    \x84\x01\x20Equivalent\x20to\x20Shift\x20+\x20Alt\x20+\x20number\x20hotk\
    ey.\x20Adds\x20current\x20selection\x20into\x20control\x20group.\x20Unit\
    s\x20are\x20removed\x20from\x20other\x20control\x20groups.\n\n\x0e\n\x07\
    \x04\x08\x04\0\x02\x04\x01\x12\x03R\x04\x12\n\x0e\n\x07\x04\x08\x04\0\
    \x02\x04\x02\x12\x03R\x15\x16\n\x0b\n\x04\x04\x08\x02\0\x12\x03T\x02)\n\
    \x0c\n\x05\x04\x08\x02\0\x04\x12\x03T\x02\n\n\x0c\n\x05\x04\x08\x02\0\
    \x06\x12\x03T\x0b\x1d\n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03T\x1e$\n\x0c\
    \n\x05\x04\x08\x02\0\x03\x12\x03T'(\n\x0b\n\x04\x04\x08\x02\x01\x12\x03U\
    \x02*\n\x0c\n\x05\x04\x08\x02\x01\x04\x12\x03U\x02\n\n\x0c\n\x05\x04\x08\
    \x02\x01\x05\x12\x03U\x0b\x11\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03U\
    \x12%\n\x0c\n\x05\x04\x08\x02\x01\x03\x12\x03U()\n\n\n\x02\x04\t\x12\x04\
    X\0Z\x01\n\n\n\x03\x04\t\x01\x12\x03X\x08\x18\n\x0b\n\x04\x04\t\x02\0\
    \x12\x03Y\x02\"\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03Y\x02\n\n\x0c\n\x05\
    \x04\t\x02\0\x05\x12\x03Y\x0b\x0f\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03Y\
    \x10\x1d\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03Y\x20!\n\n\n\x02\x04\n\x12\
    \x04\\\0^\x01\n\n\n\x03\x04\n\x01\x12\x03\\\x08\x1d\n\x0b\n\x04\x04\n\
    \x02\0\x12\x03]\x02\"\n\x0c\n\x05\x04\n\x02\0\x04\x12\x03]\x02\n\n\x0c\n\
    \x05\x04\n\x02\0\x05\x12\x03]\x0b\x0f\n\x0c\n\x05\x04\n\x02\0\x01\x12\
    \x03]\x10\x1d\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03]\x20!\n\n\n\x02\x04\
    \x0b\x12\x04`\0a\x01\n\n\n\x03\x04\x0b\x01\x12\x03`\x08\x19\n\n\n\x02\
    \x04\x0c\x12\x04c\0k\x01\n\n\n\x03\x04\x0c\x01\x12\x03c\x08\x1e\n\x0c\n\
    \x04\x04\x0c\x04\0\x12\x04d\x02i\x03\n\x0c\n\x05\x04\x0c\x04\0\x01\x12\
    \x03d\x07\x0b\nc\n\x06\x04\x0c\x04\0\x02\0\x12\x03e\x04\x0c\"T\x20Equiva\
    lent\x20to\x20click\x20with\x20no\x20modifiers.\x20Replaces\x20selection\
    \x20with\x20single\x20idle\x20worker.\n\n\x0e\n\x07\x04\x0c\x04\0\x02\0\
    \x01\x12\x03e\x04\x07\n\x0e\n\x07\x04\x0c\x04\0\x02\0\x02\x12\x03e\n\x0b\
    \nY\n\x06\x04\x0c\x04\0\x02\x01\x12\x03f\x04\x0c\"J\x20Equivalent\x20to\
    \x20shift+click.\x20Adds\x20single\x20idle\x20worker\x20to\x20current\
    \x20selection.\n\n\x0e\n\x07\x04\x0c\x04\0\x02\x01\x01\x12\x03f\x04\x07\
    \n\x0e\n\x07\x04\x0c\x04\0\x02\x01\x02\x12\x03f\n\x0b\nG\n\x06\x04\x0c\
    \x04\0\x02\x02\x12\x03g\x04\x0c\"8\x20Equivalent\x20to\x20control+click.\
    \x20Selects\x20all\x20idle\x20workers.\n\n\x0e\n\x07\x04\x0c\x04\0\x02\
    \x02\x01\x12\x03g\x04\x07\n\x0e\n\x07\x04\x0c\x04\0\x02\x02\x02\x12\x03g\
    \n\x0b\n_\n\x06\x04\x0c\x04\0\x02\x03\x12\x03h\x04\x0f\"P\x20Equivalent\
    \x20to\x20shift+control+click.\x20Adds\x20all\x20idle\x20workers\x20to\
    \x20current\x20selection.\n\n\x0e\n\x07\x04\x0c\x04\0\x02\x03\x01\x12\
    \x03h\x04\n\n\x0e\n\x07\x04\x0c\x04\0\x02\x03\x02\x12\x03h\r\x0e\n\x0b\n\
    \x04\x04\x0c\x02\0\x12\x03j\x02\x19\n\x0c\n\x05\x04\x0c\x02\0\x04\x12\
    \x03j\x02\n\n\x0c\n\x05\x04\x0c\x02\0\x06\x12\x03j\x0b\x0f\n\x0c\n\x05\
    \x04\x0c\x02\0\x01\x12\x03j\x10\x14\n\x0c\n\x05\x04\x0c\x02\0\x03\x12\
    \x03j\x17\x18\n\n\n\x02\x04\r\x12\x04m\0v\x01\n\n\n\x03\x04\r\x01\x12\
    \x03m\x08\x18\n\x0c\n\x04\x04\r\x04\0\x12\x04n\x02s\x03\n\x0c\n\x05\x04\
    \r\x04\0\x01\x12\x03n\x07\x0b\n\x1e\n\x06\x04\r\x04\0\x02\0\x12\x03o\x04\
    \x15\"\x0f\x20Click\x20on\x20icon\n\n\x0e\n\x07\x04\r\x04\0\x02\0\x01\
    \x12\x03o\x04\x10\n\x0e\n\x07\x04\r\x04\0\x02\0\x02\x12\x03o\x13\x14\n$\
    \n\x06\x04\r\x04\0\x02\x01\x12\x03p\x04\x15\"\x15\x20Shift\x20Click\x20o\
    n\x20icon\n\n\x0e\n\x07\x04\r\x04\0\x02\x01\x01\x12\x03p\x04\x10\n\x0e\n\
    \x07\x04\r\x04\0\x02\x01\x02\x12\x03p\x13\x14\n'\n\x06\x04\r\x04\0\x02\
    \x02\x12\x03q\x04\x18\"\x18\x20Control\x20Click\x20on\x20icon.\n\n\x0e\n\
    \x07\x04\r\x04\0\x02\x02\x01\x12\x03q\x04\x13\n\x0e\n\x07\x04\r\x04\0\
    \x02\x02\x02\x12\x03q\x16\x17\n-\n\x06\x04\r\x04\0\x02\x03\x12\x03r\x04\
    \x1a\"\x1e\x20Control+Shift\x20Click\x20on\x20icon.\n\n\x0e\n\x07\x04\r\
    \x04\0\x02\x03\x01\x12\x03r\x04\x15\n\x0e\n\x07\x04\r\x04\0\x02\x03\x02\
    \x12\x03r\x18\x19\n\x0b\n\x04\x04\r\x02\0\x12\x03t\x02\x19\n\x0c\n\x05\
    \x04\r\x02\0\x04\x12\x03t\x02\n\n\x0c\n\x05\x04\r\x02\0\x06\x12\x03t\x0b\
    \x0f\n\x0c\n\x05\x04\r\x02\0\x01\x12\x03t\x10\x14\n\x0c\n\x05\x04\r\x02\
    \0\x03\x12\x03t\x17\x18\n\x0b\n\x04\x04\r\x02\x01\x12\x03u\x02\x20\n\x0c\
    \n\x05\x04\r\x02\x01\x04\x12\x03u\x02\n\n\x0c\n\x05\x04\r\x02\x01\x05\
    \x12\x03u\x0b\x10\n\x0c\n\x05\x04\r\x02\x01\x01\x12\x03u\x11\x1b\n\x0c\n\
    \x05\x04\r\x02\x01\x03\x12\x03u\x1e\x1f\n\n\n\x02\x04\x0e\x12\x04x\0z\
    \x01\n\n\n\x03\x04\x0e\x01\x12\x03x\x08\x1e\n\x0b\n\x04\x04\x0e\x02\0\
    \x12\x03y\x02\x20\n\x0c\n\x05\x04\x0e\x02\0\x04\x12\x03y\x02\n\n\x0c\n\
    \x05\x04\x0e\x02\0\x05\x12\x03y\x0b\x10\n\x0c\n\x05\x04\x0e\x02\0\x01\
    \x12\x03y\x11\x1b\n\x0c\n\x05\x04\x0e\x02\0\x03\x12\x03y\x1e\x1f\n\n\n\
    \x02\x04\x0f\x12\x04|\0~\x01\n\n\n\x03\x04\x0f\x01\x12\x03|\x08,\n\x0b\n\
    \x04\x04\x0f\x02\0\x12\x03}\x02\x20\n\x0c\n\x05\x04\x0f\x02\0\x04\x12\
    \x03}\x02\n\n\x0c\n\x05\x04\x0f\x02\0\x05\x12\x03}\x0b\x10\n\x0c\n\x05\
    \x04\x0f\x02\0\x01\x12\x03}\x11\x1b\n\x0c\n\x05\x04\x0f\x02\0\x03\x12\
    \x03}\x1e\x1f\n\x0c\n\x02\x04\x10\x12\x06\x80\x01\0\x82\x01\x01\n\x0b\n\
    \x03\x04\x10\x01\x12\x04\x80\x01\x08\x1c\n\x0c\n\x04\x04\x10\x02\0\x12\
    \x04\x81\x01\x02\x20\n\r\n\x05\x04\x10\x02\0\x04\x12\x04\x81\x01\x02\n\n\
    \r\n\x05\x04\x10\x02\0\x05\x12\x04\x81\x01\x0b\x10\n\r\n\x05\x04\x10\x02\
    \0\x01\x12\x04\x81\x01\x11\x1b\n\r\n\x05\x04\x10\x02\0\x03\x12\x04\x81\
    \x01\x1e\x1f\
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
