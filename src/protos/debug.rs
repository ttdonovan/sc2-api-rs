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
pub struct DebugCommand {
    // message oneof groups
    command: ::std::option::Option<DebugCommand_oneof_command>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugCommand {}

#[derive(Clone,PartialEq)]
pub enum DebugCommand_oneof_command {
    draw(DebugDraw),
    game_state(DebugGameState),
    create_unit(DebugCreateUnit),
    kill_unit(DebugKillUnit),
    test_process(DebugTestProcess),
    score(DebugSetScore),
    end_game(DebugEndGame),
    unit_value(DebugSetUnitValue),
    chat(DebugChat),
}

impl DebugCommand {
    pub fn new() -> DebugCommand {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugCommand {
        static mut instance: ::protobuf::lazy::Lazy<DebugCommand> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugCommand,
        };
        unsafe {
            instance.get(DebugCommand::new)
        }
    }

    // optional .SC2APIProtocol.DebugDraw draw = 1;

    pub fn clear_draw(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_draw(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::draw(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_draw(&mut self, v: DebugDraw) {
        self.command = ::std::option::Option::Some(DebugCommand_oneof_command::draw(v))
    }

    // Mutable pointer to the field.
    pub fn mut_draw(&mut self) -> &mut DebugDraw {
        if let ::std::option::Option::Some(DebugCommand_oneof_command::draw(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(DebugCommand_oneof_command::draw(DebugDraw::new()));
        }
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::draw(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_draw(&mut self) -> DebugDraw {
        if self.has_draw() {
            match self.command.take() {
                ::std::option::Option::Some(DebugCommand_oneof_command::draw(v)) => v,
                _ => panic!(),
            }
        } else {
            DebugDraw::new()
        }
    }

    pub fn get_draw(&self) -> &DebugDraw {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::draw(ref v)) => v,
            _ => DebugDraw::default_instance(),
        }
    }

    // optional .SC2APIProtocol.DebugGameState game_state = 2;

    pub fn clear_game_state(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_game_state(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::game_state(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_game_state(&mut self, v: DebugGameState) {
        self.command = ::std::option::Option::Some(DebugCommand_oneof_command::game_state(v))
    }

    pub fn get_game_state(&self) -> DebugGameState {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::game_state(v)) => v,
            _ => DebugGameState::show_map,
        }
    }

    // optional .SC2APIProtocol.DebugCreateUnit create_unit = 3;

    pub fn clear_create_unit(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_create_unit(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::create_unit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_unit(&mut self, v: DebugCreateUnit) {
        self.command = ::std::option::Option::Some(DebugCommand_oneof_command::create_unit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_unit(&mut self) -> &mut DebugCreateUnit {
        if let ::std::option::Option::Some(DebugCommand_oneof_command::create_unit(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(DebugCommand_oneof_command::create_unit(DebugCreateUnit::new()));
        }
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::create_unit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_unit(&mut self) -> DebugCreateUnit {
        if self.has_create_unit() {
            match self.command.take() {
                ::std::option::Option::Some(DebugCommand_oneof_command::create_unit(v)) => v,
                _ => panic!(),
            }
        } else {
            DebugCreateUnit::new()
        }
    }

    pub fn get_create_unit(&self) -> &DebugCreateUnit {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::create_unit(ref v)) => v,
            _ => DebugCreateUnit::default_instance(),
        }
    }

    // optional .SC2APIProtocol.DebugKillUnit kill_unit = 4;

    pub fn clear_kill_unit(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_kill_unit(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::kill_unit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_kill_unit(&mut self, v: DebugKillUnit) {
        self.command = ::std::option::Option::Some(DebugCommand_oneof_command::kill_unit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_kill_unit(&mut self) -> &mut DebugKillUnit {
        if let ::std::option::Option::Some(DebugCommand_oneof_command::kill_unit(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(DebugCommand_oneof_command::kill_unit(DebugKillUnit::new()));
        }
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::kill_unit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_kill_unit(&mut self) -> DebugKillUnit {
        if self.has_kill_unit() {
            match self.command.take() {
                ::std::option::Option::Some(DebugCommand_oneof_command::kill_unit(v)) => v,
                _ => panic!(),
            }
        } else {
            DebugKillUnit::new()
        }
    }

    pub fn get_kill_unit(&self) -> &DebugKillUnit {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::kill_unit(ref v)) => v,
            _ => DebugKillUnit::default_instance(),
        }
    }

    // optional .SC2APIProtocol.DebugTestProcess test_process = 5;

    pub fn clear_test_process(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_test_process(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::test_process(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_test_process(&mut self, v: DebugTestProcess) {
        self.command = ::std::option::Option::Some(DebugCommand_oneof_command::test_process(v))
    }

    // Mutable pointer to the field.
    pub fn mut_test_process(&mut self) -> &mut DebugTestProcess {
        if let ::std::option::Option::Some(DebugCommand_oneof_command::test_process(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(DebugCommand_oneof_command::test_process(DebugTestProcess::new()));
        }
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::test_process(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_test_process(&mut self) -> DebugTestProcess {
        if self.has_test_process() {
            match self.command.take() {
                ::std::option::Option::Some(DebugCommand_oneof_command::test_process(v)) => v,
                _ => panic!(),
            }
        } else {
            DebugTestProcess::new()
        }
    }

    pub fn get_test_process(&self) -> &DebugTestProcess {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::test_process(ref v)) => v,
            _ => DebugTestProcess::default_instance(),
        }
    }

    // optional .SC2APIProtocol.DebugSetScore score = 6;

    pub fn clear_score(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_score(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::score(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: DebugSetScore) {
        self.command = ::std::option::Option::Some(DebugCommand_oneof_command::score(v))
    }

    // Mutable pointer to the field.
    pub fn mut_score(&mut self) -> &mut DebugSetScore {
        if let ::std::option::Option::Some(DebugCommand_oneof_command::score(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(DebugCommand_oneof_command::score(DebugSetScore::new()));
        }
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::score(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_score(&mut self) -> DebugSetScore {
        if self.has_score() {
            match self.command.take() {
                ::std::option::Option::Some(DebugCommand_oneof_command::score(v)) => v,
                _ => panic!(),
            }
        } else {
            DebugSetScore::new()
        }
    }

    pub fn get_score(&self) -> &DebugSetScore {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::score(ref v)) => v,
            _ => DebugSetScore::default_instance(),
        }
    }

    // optional .SC2APIProtocol.DebugEndGame end_game = 7;

    pub fn clear_end_game(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_end_game(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::end_game(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_end_game(&mut self, v: DebugEndGame) {
        self.command = ::std::option::Option::Some(DebugCommand_oneof_command::end_game(v))
    }

    // Mutable pointer to the field.
    pub fn mut_end_game(&mut self) -> &mut DebugEndGame {
        if let ::std::option::Option::Some(DebugCommand_oneof_command::end_game(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(DebugCommand_oneof_command::end_game(DebugEndGame::new()));
        }
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::end_game(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_end_game(&mut self) -> DebugEndGame {
        if self.has_end_game() {
            match self.command.take() {
                ::std::option::Option::Some(DebugCommand_oneof_command::end_game(v)) => v,
                _ => panic!(),
            }
        } else {
            DebugEndGame::new()
        }
    }

    pub fn get_end_game(&self) -> &DebugEndGame {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::end_game(ref v)) => v,
            _ => DebugEndGame::default_instance(),
        }
    }

    // optional .SC2APIProtocol.DebugSetUnitValue unit_value = 8;

    pub fn clear_unit_value(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_unit_value(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::unit_value(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_unit_value(&mut self, v: DebugSetUnitValue) {
        self.command = ::std::option::Option::Some(DebugCommand_oneof_command::unit_value(v))
    }

    // Mutable pointer to the field.
    pub fn mut_unit_value(&mut self) -> &mut DebugSetUnitValue {
        if let ::std::option::Option::Some(DebugCommand_oneof_command::unit_value(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(DebugCommand_oneof_command::unit_value(DebugSetUnitValue::new()));
        }
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::unit_value(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_unit_value(&mut self) -> DebugSetUnitValue {
        if self.has_unit_value() {
            match self.command.take() {
                ::std::option::Option::Some(DebugCommand_oneof_command::unit_value(v)) => v,
                _ => panic!(),
            }
        } else {
            DebugSetUnitValue::new()
        }
    }

    pub fn get_unit_value(&self) -> &DebugSetUnitValue {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::unit_value(ref v)) => v,
            _ => DebugSetUnitValue::default_instance(),
        }
    }

    // optional .SC2APIProtocol.DebugChat chat = 9;

    pub fn clear_chat(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_chat(&self) -> bool {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::chat(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_chat(&mut self, v: DebugChat) {
        self.command = ::std::option::Option::Some(DebugCommand_oneof_command::chat(v))
    }

    // Mutable pointer to the field.
    pub fn mut_chat(&mut self) -> &mut DebugChat {
        if let ::std::option::Option::Some(DebugCommand_oneof_command::chat(_)) = self.command {
        } else {
            self.command = ::std::option::Option::Some(DebugCommand_oneof_command::chat(DebugChat::new()));
        }
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::chat(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_chat(&mut self) -> DebugChat {
        if self.has_chat() {
            match self.command.take() {
                ::std::option::Option::Some(DebugCommand_oneof_command::chat(v)) => v,
                _ => panic!(),
            }
        } else {
            DebugChat::new()
        }
    }

    pub fn get_chat(&self) -> &DebugChat {
        match self.command {
            ::std::option::Option::Some(DebugCommand_oneof_command::chat(ref v)) => v,
            _ => DebugChat::default_instance(),
        }
    }
}

impl ::protobuf::Message for DebugCommand {
    fn is_initialized(&self) -> bool {
        if let Some(DebugCommand_oneof_command::draw(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DebugCommand_oneof_command::create_unit(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DebugCommand_oneof_command::kill_unit(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DebugCommand_oneof_command::test_process(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DebugCommand_oneof_command::score(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DebugCommand_oneof_command::end_game(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DebugCommand_oneof_command::unit_value(ref v)) = self.command {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(DebugCommand_oneof_command::chat(ref v)) = self.command {
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
                    self.command = ::std::option::Option::Some(DebugCommand_oneof_command::draw(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(DebugCommand_oneof_command::game_state(is.read_enum()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(DebugCommand_oneof_command::create_unit(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(DebugCommand_oneof_command::kill_unit(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(DebugCommand_oneof_command::test_process(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(DebugCommand_oneof_command::score(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(DebugCommand_oneof_command::end_game(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(DebugCommand_oneof_command::unit_value(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.command = ::std::option::Option::Some(DebugCommand_oneof_command::chat(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.command {
            match v {
                &DebugCommand_oneof_command::draw(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DebugCommand_oneof_command::game_state(v) => {
                    my_size += ::protobuf::rt::enum_size(2, v);
                },
                &DebugCommand_oneof_command::create_unit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DebugCommand_oneof_command::kill_unit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DebugCommand_oneof_command::test_process(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DebugCommand_oneof_command::score(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DebugCommand_oneof_command::end_game(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DebugCommand_oneof_command::unit_value(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &DebugCommand_oneof_command::chat(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.command {
            match v {
                &DebugCommand_oneof_command::draw(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DebugCommand_oneof_command::game_state(v) => {
                    os.write_enum(2, v.value())?;
                },
                &DebugCommand_oneof_command::create_unit(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DebugCommand_oneof_command::kill_unit(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DebugCommand_oneof_command::test_process(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DebugCommand_oneof_command::score(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DebugCommand_oneof_command::end_game(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DebugCommand_oneof_command::unit_value(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &DebugCommand_oneof_command::chat(ref v) => {
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

impl ::protobuf::MessageStatic for DebugCommand {
    fn new() -> DebugCommand {
        DebugCommand::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugCommand>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DebugDraw>(
                    "draw",
                    DebugCommand::has_draw,
                    DebugCommand::get_draw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, DebugGameState>(
                    "game_state",
                    DebugCommand::has_game_state,
                    DebugCommand::get_game_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DebugCreateUnit>(
                    "create_unit",
                    DebugCommand::has_create_unit,
                    DebugCommand::get_create_unit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DebugKillUnit>(
                    "kill_unit",
                    DebugCommand::has_kill_unit,
                    DebugCommand::get_kill_unit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DebugTestProcess>(
                    "test_process",
                    DebugCommand::has_test_process,
                    DebugCommand::get_test_process,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DebugSetScore>(
                    "score",
                    DebugCommand::has_score,
                    DebugCommand::get_score,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DebugEndGame>(
                    "end_game",
                    DebugCommand::has_end_game,
                    DebugCommand::get_end_game,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DebugSetUnitValue>(
                    "unit_value",
                    DebugCommand::has_unit_value,
                    DebugCommand::get_unit_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, DebugChat>(
                    "chat",
                    DebugCommand::has_chat,
                    DebugCommand::get_chat,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugCommand>(
                    "DebugCommand",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugCommand {
    fn clear(&mut self) {
        self.clear_draw();
        self.clear_game_state();
        self.clear_create_unit();
        self.clear_kill_unit();
        self.clear_test_process();
        self.clear_score();
        self.clear_end_game();
        self.clear_unit_value();
        self.clear_chat();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugCommand {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugCommand {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugDraw {
    // message fields
    text: ::protobuf::RepeatedField<DebugText>,
    lines: ::protobuf::RepeatedField<DebugLine>,
    boxes: ::protobuf::RepeatedField<DebugBox>,
    spheres: ::protobuf::RepeatedField<DebugSphere>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugDraw {}

impl DebugDraw {
    pub fn new() -> DebugDraw {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugDraw {
        static mut instance: ::protobuf::lazy::Lazy<DebugDraw> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugDraw,
        };
        unsafe {
            instance.get(DebugDraw::new)
        }
    }

    // repeated .SC2APIProtocol.DebugText text = 1;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::protobuf::RepeatedField<DebugText>) {
        self.text = v;
    }

    // Mutable pointer to the field.
    pub fn mut_text(&mut self) -> &mut ::protobuf::RepeatedField<DebugText> {
        &mut self.text
    }

    // Take field
    pub fn take_text(&mut self) -> ::protobuf::RepeatedField<DebugText> {
        ::std::mem::replace(&mut self.text, ::protobuf::RepeatedField::new())
    }

    pub fn get_text(&self) -> &[DebugText] {
        &self.text
    }

    fn get_text_for_reflect(&self) -> &::protobuf::RepeatedField<DebugText> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DebugText> {
        &mut self.text
    }

    // repeated .SC2APIProtocol.DebugLine lines = 2;

    pub fn clear_lines(&mut self) {
        self.lines.clear();
    }

    // Param is passed by value, moved
    pub fn set_lines(&mut self, v: ::protobuf::RepeatedField<DebugLine>) {
        self.lines = v;
    }

    // Mutable pointer to the field.
    pub fn mut_lines(&mut self) -> &mut ::protobuf::RepeatedField<DebugLine> {
        &mut self.lines
    }

    // Take field
    pub fn take_lines(&mut self) -> ::protobuf::RepeatedField<DebugLine> {
        ::std::mem::replace(&mut self.lines, ::protobuf::RepeatedField::new())
    }

    pub fn get_lines(&self) -> &[DebugLine] {
        &self.lines
    }

    fn get_lines_for_reflect(&self) -> &::protobuf::RepeatedField<DebugLine> {
        &self.lines
    }

    fn mut_lines_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DebugLine> {
        &mut self.lines
    }

    // repeated .SC2APIProtocol.DebugBox boxes = 3;

    pub fn clear_boxes(&mut self) {
        self.boxes.clear();
    }

    // Param is passed by value, moved
    pub fn set_boxes(&mut self, v: ::protobuf::RepeatedField<DebugBox>) {
        self.boxes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_boxes(&mut self) -> &mut ::protobuf::RepeatedField<DebugBox> {
        &mut self.boxes
    }

    // Take field
    pub fn take_boxes(&mut self) -> ::protobuf::RepeatedField<DebugBox> {
        ::std::mem::replace(&mut self.boxes, ::protobuf::RepeatedField::new())
    }

    pub fn get_boxes(&self) -> &[DebugBox] {
        &self.boxes
    }

    fn get_boxes_for_reflect(&self) -> &::protobuf::RepeatedField<DebugBox> {
        &self.boxes
    }

    fn mut_boxes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DebugBox> {
        &mut self.boxes
    }

    // repeated .SC2APIProtocol.DebugSphere spheres = 4;

    pub fn clear_spheres(&mut self) {
        self.spheres.clear();
    }

    // Param is passed by value, moved
    pub fn set_spheres(&mut self, v: ::protobuf::RepeatedField<DebugSphere>) {
        self.spheres = v;
    }

    // Mutable pointer to the field.
    pub fn mut_spheres(&mut self) -> &mut ::protobuf::RepeatedField<DebugSphere> {
        &mut self.spheres
    }

    // Take field
    pub fn take_spheres(&mut self) -> ::protobuf::RepeatedField<DebugSphere> {
        ::std::mem::replace(&mut self.spheres, ::protobuf::RepeatedField::new())
    }

    pub fn get_spheres(&self) -> &[DebugSphere] {
        &self.spheres
    }

    fn get_spheres_for_reflect(&self) -> &::protobuf::RepeatedField<DebugSphere> {
        &self.spheres
    }

    fn mut_spheres_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DebugSphere> {
        &mut self.spheres
    }
}

impl ::protobuf::Message for DebugDraw {
    fn is_initialized(&self) -> bool {
        for v in &self.text {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lines {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.boxes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.spheres {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.text)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.lines)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.boxes)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.spheres)?;
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
        for value in &self.text {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.lines {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.boxes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.spheres {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.text {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.lines {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.boxes {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.spheres {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DebugDraw {
    fn new() -> DebugDraw {
        DebugDraw::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugDraw>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugText>>(
                    "text",
                    DebugDraw::get_text_for_reflect,
                    DebugDraw::mut_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugLine>>(
                    "lines",
                    DebugDraw::get_lines_for_reflect,
                    DebugDraw::mut_lines_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugBox>>(
                    "boxes",
                    DebugDraw::get_boxes_for_reflect,
                    DebugDraw::mut_boxes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DebugSphere>>(
                    "spheres",
                    DebugDraw::get_spheres_for_reflect,
                    DebugDraw::mut_spheres_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugDraw>(
                    "DebugDraw",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugDraw {
    fn clear(&mut self) {
        self.clear_text();
        self.clear_lines();
        self.clear_boxes();
        self.clear_spheres();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugDraw {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugDraw {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Line {
    // message fields
    p0: ::protobuf::SingularPtrField<super::common::Point>,
    p1: ::protobuf::SingularPtrField<super::common::Point>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Line {}

impl Line {
    pub fn new() -> Line {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Line {
        static mut instance: ::protobuf::lazy::Lazy<Line> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Line,
        };
        unsafe {
            instance.get(Line::new)
        }
    }

    // optional .SC2APIProtocol.Point p0 = 1;

    pub fn clear_p0(&mut self) {
        self.p0.clear();
    }

    pub fn has_p0(&self) -> bool {
        self.p0.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p0(&mut self, v: super::common::Point) {
        self.p0 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_p0(&mut self) -> &mut super::common::Point {
        if self.p0.is_none() {
            self.p0.set_default();
        }
        self.p0.as_mut().unwrap()
    }

    // Take field
    pub fn take_p0(&mut self) -> super::common::Point {
        self.p0.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_p0(&self) -> &super::common::Point {
        self.p0.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_p0_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.p0
    }

    fn mut_p0_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.p0
    }

    // optional .SC2APIProtocol.Point p1 = 2;

    pub fn clear_p1(&mut self) {
        self.p1.clear();
    }

    pub fn has_p1(&self) -> bool {
        self.p1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p1(&mut self, v: super::common::Point) {
        self.p1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_p1(&mut self) -> &mut super::common::Point {
        if self.p1.is_none() {
            self.p1.set_default();
        }
        self.p1.as_mut().unwrap()
    }

    // Take field
    pub fn take_p1(&mut self) -> super::common::Point {
        self.p1.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_p1(&self) -> &super::common::Point {
        self.p1.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_p1_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.p1
    }

    fn mut_p1_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.p1
    }
}

impl ::protobuf::Message for Line {
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

impl ::protobuf::MessageStatic for Line {
    fn new() -> Line {
        Line::new()
    }

    fn descriptor_static(_: ::std::option::Option<Line>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "p0",
                    Line::get_p0_for_reflect,
                    Line::mut_p0_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "p1",
                    Line::get_p1_for_reflect,
                    Line::mut_p1_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Line>(
                    "Line",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Line {
    fn clear(&mut self) {
        self.clear_p0();
        self.clear_p1();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Line {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Line {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Color {
    // message fields
    r: ::std::option::Option<u32>,
    g: ::std::option::Option<u32>,
    b: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Color {}

impl Color {
    pub fn new() -> Color {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Color {
        static mut instance: ::protobuf::lazy::Lazy<Color> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Color,
        };
        unsafe {
            instance.get(Color::new)
        }
    }

    // optional uint32 r = 1;

    pub fn clear_r(&mut self) {
        self.r = ::std::option::Option::None;
    }

    pub fn has_r(&self) -> bool {
        self.r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r(&mut self, v: u32) {
        self.r = ::std::option::Option::Some(v);
    }

    pub fn get_r(&self) -> u32 {
        self.r.unwrap_or(0)
    }

    fn get_r_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.r
    }

    fn mut_r_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.r
    }

    // optional uint32 g = 2;

    pub fn clear_g(&mut self) {
        self.g = ::std::option::Option::None;
    }

    pub fn has_g(&self) -> bool {
        self.g.is_some()
    }

    // Param is passed by value, moved
    pub fn set_g(&mut self, v: u32) {
        self.g = ::std::option::Option::Some(v);
    }

    pub fn get_g(&self) -> u32 {
        self.g.unwrap_or(0)
    }

    fn get_g_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.g
    }

    fn mut_g_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.g
    }

    // optional uint32 b = 3;

    pub fn clear_b(&mut self) {
        self.b = ::std::option::Option::None;
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: u32) {
        self.b = ::std::option::Option::Some(v);
    }

    pub fn get_b(&self) -> u32 {
        self.b.unwrap_or(0)
    }

    fn get_b_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.b
    }

    fn mut_b_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.b
    }
}

impl ::protobuf::Message for Color {
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
                    self.r = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.g = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.b = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.r {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.g {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.b {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.r {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.g {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.b {
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

impl ::protobuf::MessageStatic for Color {
    fn new() -> Color {
        Color::new()
    }

    fn descriptor_static(_: ::std::option::Option<Color>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "r",
                    Color::get_r_for_reflect,
                    Color::mut_r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "g",
                    Color::get_g_for_reflect,
                    Color::mut_g_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "b",
                    Color::get_b_for_reflect,
                    Color::mut_b_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Color>(
                    "Color",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Color {
    fn clear(&mut self) {
        self.clear_r();
        self.clear_g();
        self.clear_b();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Color {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugText {
    // message fields
    color: ::protobuf::SingularPtrField<Color>,
    text: ::protobuf::SingularField<::std::string::String>,
    virtual_pos: ::protobuf::SingularPtrField<super::common::Point>,
    world_pos: ::protobuf::SingularPtrField<super::common::Point>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugText {}

impl DebugText {
    pub fn new() -> DebugText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugText {
        static mut instance: ::protobuf::lazy::Lazy<DebugText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugText,
        };
        unsafe {
            instance.get(DebugText::new)
        }
    }

    // optional .SC2APIProtocol.Color color = 1;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: Color) {
        self.color = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut Color {
        if self.color.is_none() {
            self.color.set_default();
        }
        self.color.as_mut().unwrap()
    }

    // Take field
    pub fn take_color(&mut self) -> Color {
        self.color.take().unwrap_or_else(|| Color::new())
    }

    pub fn get_color(&self) -> &Color {
        self.color.as_ref().unwrap_or_else(|| Color::default_instance())
    }

    fn get_color_for_reflect(&self) -> &::protobuf::SingularPtrField<Color> {
        &self.color
    }

    fn mut_color_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Color> {
        &mut self.color
    }

    // optional string text = 2;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        }
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_text_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.text
    }

    // optional .SC2APIProtocol.Point virtual_pos = 3;

    pub fn clear_virtual_pos(&mut self) {
        self.virtual_pos.clear();
    }

    pub fn has_virtual_pos(&self) -> bool {
        self.virtual_pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_virtual_pos(&mut self, v: super::common::Point) {
        self.virtual_pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_virtual_pos(&mut self) -> &mut super::common::Point {
        if self.virtual_pos.is_none() {
            self.virtual_pos.set_default();
        }
        self.virtual_pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_virtual_pos(&mut self) -> super::common::Point {
        self.virtual_pos.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_virtual_pos(&self) -> &super::common::Point {
        self.virtual_pos.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_virtual_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.virtual_pos
    }

    fn mut_virtual_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.virtual_pos
    }

    // optional .SC2APIProtocol.Point world_pos = 4;

    pub fn clear_world_pos(&mut self) {
        self.world_pos.clear();
    }

    pub fn has_world_pos(&self) -> bool {
        self.world_pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_world_pos(&mut self, v: super::common::Point) {
        self.world_pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_world_pos(&mut self) -> &mut super::common::Point {
        if self.world_pos.is_none() {
            self.world_pos.set_default();
        }
        self.world_pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_world_pos(&mut self) -> super::common::Point {
        self.world_pos.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_world_pos(&self) -> &super::common::Point {
        self.world_pos.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_world_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.world_pos
    }

    fn mut_world_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.world_pos
    }
}

impl ::protobuf::Message for DebugText {
    fn is_initialized(&self) -> bool {
        for v in &self.color {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.virtual_pos {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.world_pos {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.color)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.virtual_pos)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.world_pos)?;
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
        if let Some(ref v) = self.color.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.virtual_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.world_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.color.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.text.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.virtual_pos.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.world_pos.as_ref() {
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

impl ::protobuf::MessageStatic for DebugText {
    fn new() -> DebugText {
        DebugText::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Color>>(
                    "color",
                    DebugText::get_color_for_reflect,
                    DebugText::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    DebugText::get_text_for_reflect,
                    DebugText::mut_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "virtual_pos",
                    DebugText::get_virtual_pos_for_reflect,
                    DebugText::mut_virtual_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "world_pos",
                    DebugText::get_world_pos_for_reflect,
                    DebugText::mut_world_pos_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugText>(
                    "DebugText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugText {
    fn clear(&mut self) {
        self.clear_color();
        self.clear_text();
        self.clear_virtual_pos();
        self.clear_world_pos();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugLine {
    // message fields
    color: ::protobuf::SingularPtrField<Color>,
    line: ::protobuf::SingularPtrField<Line>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugLine {}

impl DebugLine {
    pub fn new() -> DebugLine {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugLine {
        static mut instance: ::protobuf::lazy::Lazy<DebugLine> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugLine,
        };
        unsafe {
            instance.get(DebugLine::new)
        }
    }

    // optional .SC2APIProtocol.Color color = 1;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: Color) {
        self.color = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut Color {
        if self.color.is_none() {
            self.color.set_default();
        }
        self.color.as_mut().unwrap()
    }

    // Take field
    pub fn take_color(&mut self) -> Color {
        self.color.take().unwrap_or_else(|| Color::new())
    }

    pub fn get_color(&self) -> &Color {
        self.color.as_ref().unwrap_or_else(|| Color::default_instance())
    }

    fn get_color_for_reflect(&self) -> &::protobuf::SingularPtrField<Color> {
        &self.color
    }

    fn mut_color_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Color> {
        &mut self.color
    }

    // optional .SC2APIProtocol.Line line = 2;

    pub fn clear_line(&mut self) {
        self.line.clear();
    }

    pub fn has_line(&self) -> bool {
        self.line.is_some()
    }

    // Param is passed by value, moved
    pub fn set_line(&mut self, v: Line) {
        self.line = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_line(&mut self) -> &mut Line {
        if self.line.is_none() {
            self.line.set_default();
        }
        self.line.as_mut().unwrap()
    }

    // Take field
    pub fn take_line(&mut self) -> Line {
        self.line.take().unwrap_or_else(|| Line::new())
    }

    pub fn get_line(&self) -> &Line {
        self.line.as_ref().unwrap_or_else(|| Line::default_instance())
    }

    fn get_line_for_reflect(&self) -> &::protobuf::SingularPtrField<Line> {
        &self.line
    }

    fn mut_line_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Line> {
        &mut self.line
    }
}

impl ::protobuf::Message for DebugLine {
    fn is_initialized(&self) -> bool {
        for v in &self.color {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.line {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.color)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.line)?;
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
        if let Some(ref v) = self.color.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.line.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.color.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.line.as_ref() {
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

impl ::protobuf::MessageStatic for DebugLine {
    fn new() -> DebugLine {
        DebugLine::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugLine>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Color>>(
                    "color",
                    DebugLine::get_color_for_reflect,
                    DebugLine::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Line>>(
                    "line",
                    DebugLine::get_line_for_reflect,
                    DebugLine::mut_line_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugLine>(
                    "DebugLine",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugLine {
    fn clear(&mut self) {
        self.clear_color();
        self.clear_line();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugLine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugLine {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugBox {
    // message fields
    color: ::protobuf::SingularPtrField<Color>,
    min: ::protobuf::SingularPtrField<super::common::Point>,
    max: ::protobuf::SingularPtrField<super::common::Point>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugBox {}

impl DebugBox {
    pub fn new() -> DebugBox {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugBox {
        static mut instance: ::protobuf::lazy::Lazy<DebugBox> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugBox,
        };
        unsafe {
            instance.get(DebugBox::new)
        }
    }

    // optional .SC2APIProtocol.Color color = 1;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: Color) {
        self.color = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut Color {
        if self.color.is_none() {
            self.color.set_default();
        }
        self.color.as_mut().unwrap()
    }

    // Take field
    pub fn take_color(&mut self) -> Color {
        self.color.take().unwrap_or_else(|| Color::new())
    }

    pub fn get_color(&self) -> &Color {
        self.color.as_ref().unwrap_or_else(|| Color::default_instance())
    }

    fn get_color_for_reflect(&self) -> &::protobuf::SingularPtrField<Color> {
        &self.color
    }

    fn mut_color_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Color> {
        &mut self.color
    }

    // optional .SC2APIProtocol.Point min = 2;

    pub fn clear_min(&mut self) {
        self.min.clear();
    }

    pub fn has_min(&self) -> bool {
        self.min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min(&mut self, v: super::common::Point) {
        self.min = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_min(&mut self) -> &mut super::common::Point {
        if self.min.is_none() {
            self.min.set_default();
        }
        self.min.as_mut().unwrap()
    }

    // Take field
    pub fn take_min(&mut self) -> super::common::Point {
        self.min.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_min(&self) -> &super::common::Point {
        self.min.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_min_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.min
    }

    fn mut_min_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.min
    }

    // optional .SC2APIProtocol.Point max = 3;

    pub fn clear_max(&mut self) {
        self.max.clear();
    }

    pub fn has_max(&self) -> bool {
        self.max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max(&mut self, v: super::common::Point) {
        self.max = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max(&mut self) -> &mut super::common::Point {
        if self.max.is_none() {
            self.max.set_default();
        }
        self.max.as_mut().unwrap()
    }

    // Take field
    pub fn take_max(&mut self) -> super::common::Point {
        self.max.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_max(&self) -> &super::common::Point {
        self.max.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_max_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.max
    }

    fn mut_max_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.max
    }
}

impl ::protobuf::Message for DebugBox {
    fn is_initialized(&self) -> bool {
        for v in &self.color {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.min {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.max {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.color)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.min)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max)?;
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
        if let Some(ref v) = self.color.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.min.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.max.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.color.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.min.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.max.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DebugBox {
    fn new() -> DebugBox {
        DebugBox::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugBox>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Color>>(
                    "color",
                    DebugBox::get_color_for_reflect,
                    DebugBox::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "min",
                    DebugBox::get_min_for_reflect,
                    DebugBox::mut_min_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "max",
                    DebugBox::get_max_for_reflect,
                    DebugBox::mut_max_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugBox>(
                    "DebugBox",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugBox {
    fn clear(&mut self) {
        self.clear_color();
        self.clear_min();
        self.clear_max();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugBox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugBox {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugSphere {
    // message fields
    color: ::protobuf::SingularPtrField<Color>,
    p: ::protobuf::SingularPtrField<super::common::Point>,
    r: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugSphere {}

impl DebugSphere {
    pub fn new() -> DebugSphere {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugSphere {
        static mut instance: ::protobuf::lazy::Lazy<DebugSphere> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugSphere,
        };
        unsafe {
            instance.get(DebugSphere::new)
        }
    }

    // optional .SC2APIProtocol.Color color = 1;

    pub fn clear_color(&mut self) {
        self.color.clear();
    }

    pub fn has_color(&self) -> bool {
        self.color.is_some()
    }

    // Param is passed by value, moved
    pub fn set_color(&mut self, v: Color) {
        self.color = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_color(&mut self) -> &mut Color {
        if self.color.is_none() {
            self.color.set_default();
        }
        self.color.as_mut().unwrap()
    }

    // Take field
    pub fn take_color(&mut self) -> Color {
        self.color.take().unwrap_or_else(|| Color::new())
    }

    pub fn get_color(&self) -> &Color {
        self.color.as_ref().unwrap_or_else(|| Color::default_instance())
    }

    fn get_color_for_reflect(&self) -> &::protobuf::SingularPtrField<Color> {
        &self.color
    }

    fn mut_color_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Color> {
        &mut self.color
    }

    // optional .SC2APIProtocol.Point p = 2;

    pub fn clear_p(&mut self) {
        self.p.clear();
    }

    pub fn has_p(&self) -> bool {
        self.p.is_some()
    }

    // Param is passed by value, moved
    pub fn set_p(&mut self, v: super::common::Point) {
        self.p = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_p(&mut self) -> &mut super::common::Point {
        if self.p.is_none() {
            self.p.set_default();
        }
        self.p.as_mut().unwrap()
    }

    // Take field
    pub fn take_p(&mut self) -> super::common::Point {
        self.p.take().unwrap_or_else(|| super::common::Point::new())
    }

    pub fn get_p(&self) -> &super::common::Point {
        self.p.as_ref().unwrap_or_else(|| super::common::Point::default_instance())
    }

    fn get_p_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point> {
        &self.p
    }

    fn mut_p_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point> {
        &mut self.p
    }

    // optional float r = 3;

    pub fn clear_r(&mut self) {
        self.r = ::std::option::Option::None;
    }

    pub fn has_r(&self) -> bool {
        self.r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r(&mut self, v: f32) {
        self.r = ::std::option::Option::Some(v);
    }

    pub fn get_r(&self) -> f32 {
        self.r.unwrap_or(0.)
    }

    fn get_r_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.r
    }

    fn mut_r_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.r
    }
}

impl ::protobuf::Message for DebugSphere {
    fn is_initialized(&self) -> bool {
        for v in &self.color {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.p {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.color)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.p)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.r = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.color.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.p.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.r {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.color.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.p.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.r {
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

impl ::protobuf::MessageStatic for DebugSphere {
    fn new() -> DebugSphere {
        DebugSphere::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugSphere>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Color>>(
                    "color",
                    DebugSphere::get_color_for_reflect,
                    DebugSphere::mut_color_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point>>(
                    "p",
                    DebugSphere::get_p_for_reflect,
                    DebugSphere::mut_p_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "r",
                    DebugSphere::get_r_for_reflect,
                    DebugSphere::mut_r_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugSphere>(
                    "DebugSphere",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugSphere {
    fn clear(&mut self) {
        self.clear_color();
        self.clear_p();
        self.clear_r();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugSphere {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugSphere {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugCreateUnit {
    // message fields
    unit_type: ::std::option::Option<u32>,
    owner: ::std::option::Option<i32>,
    pos: ::protobuf::SingularPtrField<super::common::Point2D>,
    quantity: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugCreateUnit {}

impl DebugCreateUnit {
    pub fn new() -> DebugCreateUnit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugCreateUnit {
        static mut instance: ::protobuf::lazy::Lazy<DebugCreateUnit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugCreateUnit,
        };
        unsafe {
            instance.get(DebugCreateUnit::new)
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

    // optional int32 owner = 2;

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

    // optional .SC2APIProtocol.Point2D pos = 3;

    pub fn clear_pos(&mut self) {
        self.pos.clear();
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: super::common::Point2D) {
        self.pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pos(&mut self) -> &mut super::common::Point2D {
        if self.pos.is_none() {
            self.pos.set_default();
        }
        self.pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_pos(&mut self) -> super::common::Point2D {
        self.pos.take().unwrap_or_else(|| super::common::Point2D::new())
    }

    pub fn get_pos(&self) -> &super::common::Point2D {
        self.pos.as_ref().unwrap_or_else(|| super::common::Point2D::default_instance())
    }

    fn get_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Point2D> {
        &self.pos
    }

    fn mut_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Point2D> {
        &mut self.pos
    }

    // optional uint32 quantity = 4;

    pub fn clear_quantity(&mut self) {
        self.quantity = ::std::option::Option::None;
    }

    pub fn has_quantity(&self) -> bool {
        self.quantity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quantity(&mut self, v: u32) {
        self.quantity = ::std::option::Option::Some(v);
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity.unwrap_or(0)
    }

    fn get_quantity_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quantity
    }

    fn mut_quantity_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quantity
    }
}

impl ::protobuf::Message for DebugCreateUnit {
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
                    let tmp = is.read_int32()?;
                    self.owner = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pos)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.quantity = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.owner {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.quantity {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_type {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.owner {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.pos.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.quantity {
            os.write_uint32(4, v)?;
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

impl ::protobuf::MessageStatic for DebugCreateUnit {
    fn new() -> DebugCreateUnit {
        DebugCreateUnit::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugCreateUnit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "unit_type",
                    DebugCreateUnit::get_unit_type_for_reflect,
                    DebugCreateUnit::mut_unit_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "owner",
                    DebugCreateUnit::get_owner_for_reflect,
                    DebugCreateUnit::mut_owner_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Point2D>>(
                    "pos",
                    DebugCreateUnit::get_pos_for_reflect,
                    DebugCreateUnit::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quantity",
                    DebugCreateUnit::get_quantity_for_reflect,
                    DebugCreateUnit::mut_quantity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugCreateUnit>(
                    "DebugCreateUnit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugCreateUnit {
    fn clear(&mut self) {
        self.clear_unit_type();
        self.clear_owner();
        self.clear_pos();
        self.clear_quantity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugCreateUnit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugCreateUnit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugKillUnit {
    // message fields
    tag: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugKillUnit {}

impl DebugKillUnit {
    pub fn new() -> DebugKillUnit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugKillUnit {
        static mut instance: ::protobuf::lazy::Lazy<DebugKillUnit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugKillUnit,
        };
        unsafe {
            instance.get(DebugKillUnit::new)
        }
    }

    // repeated uint64 tag = 1;

    pub fn clear_tag(&mut self) {
        self.tag.clear();
    }

    // Param is passed by value, moved
    pub fn set_tag(&mut self, v: ::std::vec::Vec<u64>) {
        self.tag = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tag(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.tag
    }

    // Take field
    pub fn take_tag(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.tag, ::std::vec::Vec::new())
    }

    pub fn get_tag(&self) -> &[u64] {
        &self.tag
    }

    fn get_tag_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.tag
    }

    fn mut_tag_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.tag
    }
}

impl ::protobuf::Message for DebugKillUnit {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.tag)?;
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
        for value in &self.tag {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tag {
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

impl ::protobuf::MessageStatic for DebugKillUnit {
    fn new() -> DebugKillUnit {
        DebugKillUnit::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugKillUnit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "tag",
                    DebugKillUnit::get_tag_for_reflect,
                    DebugKillUnit::mut_tag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugKillUnit>(
                    "DebugKillUnit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugKillUnit {
    fn clear(&mut self) {
        self.clear_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugKillUnit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugKillUnit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugTestProcess {
    // message fields
    test: ::std::option::Option<DebugTestProcess_Test>,
    delay_ms: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugTestProcess {}

impl DebugTestProcess {
    pub fn new() -> DebugTestProcess {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugTestProcess {
        static mut instance: ::protobuf::lazy::Lazy<DebugTestProcess> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugTestProcess,
        };
        unsafe {
            instance.get(DebugTestProcess::new)
        }
    }

    // optional .SC2APIProtocol.DebugTestProcess.Test test = 1;

    pub fn clear_test(&mut self) {
        self.test = ::std::option::Option::None;
    }

    pub fn has_test(&self) -> bool {
        self.test.is_some()
    }

    // Param is passed by value, moved
    pub fn set_test(&mut self, v: DebugTestProcess_Test) {
        self.test = ::std::option::Option::Some(v);
    }

    pub fn get_test(&self) -> DebugTestProcess_Test {
        self.test.unwrap_or(DebugTestProcess_Test::hang)
    }

    fn get_test_for_reflect(&self) -> &::std::option::Option<DebugTestProcess_Test> {
        &self.test
    }

    fn mut_test_for_reflect(&mut self) -> &mut ::std::option::Option<DebugTestProcess_Test> {
        &mut self.test
    }

    // optional int32 delay_ms = 2;

    pub fn clear_delay_ms(&mut self) {
        self.delay_ms = ::std::option::Option::None;
    }

    pub fn has_delay_ms(&self) -> bool {
        self.delay_ms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay_ms(&mut self, v: i32) {
        self.delay_ms = ::std::option::Option::Some(v);
    }

    pub fn get_delay_ms(&self) -> i32 {
        self.delay_ms.unwrap_or(0)
    }

    fn get_delay_ms_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.delay_ms
    }

    fn mut_delay_ms_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.delay_ms
    }
}

impl ::protobuf::Message for DebugTestProcess {
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
                    self.test = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.delay_ms = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.test {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.delay_ms {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.test {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.delay_ms {
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

impl ::protobuf::MessageStatic for DebugTestProcess {
    fn new() -> DebugTestProcess {
        DebugTestProcess::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugTestProcess>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DebugTestProcess_Test>>(
                    "test",
                    DebugTestProcess::get_test_for_reflect,
                    DebugTestProcess::mut_test_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "delay_ms",
                    DebugTestProcess::get_delay_ms_for_reflect,
                    DebugTestProcess::mut_delay_ms_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugTestProcess>(
                    "DebugTestProcess",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugTestProcess {
    fn clear(&mut self) {
        self.clear_test();
        self.clear_delay_ms();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugTestProcess {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugTestProcess {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DebugTestProcess_Test {
    hang = 1,
    crash = 2,
    exit = 3,
}

impl ::protobuf::ProtobufEnum for DebugTestProcess_Test {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DebugTestProcess_Test> {
        match value {
            1 => ::std::option::Option::Some(DebugTestProcess_Test::hang),
            2 => ::std::option::Option::Some(DebugTestProcess_Test::crash),
            3 => ::std::option::Option::Some(DebugTestProcess_Test::exit),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DebugTestProcess_Test] = &[
            DebugTestProcess_Test::hang,
            DebugTestProcess_Test::crash,
            DebugTestProcess_Test::exit,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DebugTestProcess_Test>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DebugTestProcess_Test", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DebugTestProcess_Test {
}

impl ::protobuf::reflect::ProtobufValue for DebugTestProcess_Test {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugSetScore {
    // message fields
    score: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugSetScore {}

impl DebugSetScore {
    pub fn new() -> DebugSetScore {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugSetScore {
        static mut instance: ::protobuf::lazy::Lazy<DebugSetScore> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugSetScore,
        };
        unsafe {
            instance.get(DebugSetScore::new)
        }
    }

    // optional float score = 1;

    pub fn clear_score(&mut self) {
        self.score = ::std::option::Option::None;
    }

    pub fn has_score(&self) -> bool {
        self.score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: f32) {
        self.score = ::std::option::Option::Some(v);
    }

    pub fn get_score(&self) -> f32 {
        self.score.unwrap_or(0.)
    }

    fn get_score_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.score
    }

    fn mut_score_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.score
    }
}

impl ::protobuf::Message for DebugSetScore {
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
                    self.score = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.score {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.score {
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

impl ::protobuf::MessageStatic for DebugSetScore {
    fn new() -> DebugSetScore {
        DebugSetScore::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugSetScore>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "score",
                    DebugSetScore::get_score_for_reflect,
                    DebugSetScore::mut_score_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugSetScore>(
                    "DebugSetScore",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugSetScore {
    fn clear(&mut self) {
        self.clear_score();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugSetScore {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugSetScore {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugEndGame {
    // message fields
    end_result: ::std::option::Option<DebugEndGame_EndResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugEndGame {}

impl DebugEndGame {
    pub fn new() -> DebugEndGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugEndGame {
        static mut instance: ::protobuf::lazy::Lazy<DebugEndGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugEndGame,
        };
        unsafe {
            instance.get(DebugEndGame::new)
        }
    }

    // optional .SC2APIProtocol.DebugEndGame.EndResult end_result = 1;

    pub fn clear_end_result(&mut self) {
        self.end_result = ::std::option::Option::None;
    }

    pub fn has_end_result(&self) -> bool {
        self.end_result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_result(&mut self, v: DebugEndGame_EndResult) {
        self.end_result = ::std::option::Option::Some(v);
    }

    pub fn get_end_result(&self) -> DebugEndGame_EndResult {
        self.end_result.unwrap_or(DebugEndGame_EndResult::Surrender)
    }

    fn get_end_result_for_reflect(&self) -> &::std::option::Option<DebugEndGame_EndResult> {
        &self.end_result
    }

    fn mut_end_result_for_reflect(&mut self) -> &mut ::std::option::Option<DebugEndGame_EndResult> {
        &mut self.end_result
    }
}

impl ::protobuf::Message for DebugEndGame {
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
                    self.end_result = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.end_result {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.end_result {
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

impl ::protobuf::MessageStatic for DebugEndGame {
    fn new() -> DebugEndGame {
        DebugEndGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugEndGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DebugEndGame_EndResult>>(
                    "end_result",
                    DebugEndGame::get_end_result_for_reflect,
                    DebugEndGame::mut_end_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugEndGame>(
                    "DebugEndGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugEndGame {
    fn clear(&mut self) {
        self.clear_end_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugEndGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugEndGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DebugEndGame_EndResult {
    Surrender = 1,
    DeclareVictory = 2,
}

impl ::protobuf::ProtobufEnum for DebugEndGame_EndResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DebugEndGame_EndResult> {
        match value {
            1 => ::std::option::Option::Some(DebugEndGame_EndResult::Surrender),
            2 => ::std::option::Option::Some(DebugEndGame_EndResult::DeclareVictory),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DebugEndGame_EndResult] = &[
            DebugEndGame_EndResult::Surrender,
            DebugEndGame_EndResult::DeclareVictory,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DebugEndGame_EndResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DebugEndGame_EndResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DebugEndGame_EndResult {
}

impl ::protobuf::reflect::ProtobufValue for DebugEndGame_EndResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugSetUnitValue {
    // message fields
    unit_value: ::std::option::Option<DebugSetUnitValue_UnitValue>,
    value: ::std::option::Option<f32>,
    unit_tag: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugSetUnitValue {}

impl DebugSetUnitValue {
    pub fn new() -> DebugSetUnitValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugSetUnitValue {
        static mut instance: ::protobuf::lazy::Lazy<DebugSetUnitValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugSetUnitValue,
        };
        unsafe {
            instance.get(DebugSetUnitValue::new)
        }
    }

    // optional .SC2APIProtocol.DebugSetUnitValue.UnitValue unit_value = 1;

    pub fn clear_unit_value(&mut self) {
        self.unit_value = ::std::option::Option::None;
    }

    pub fn has_unit_value(&self) -> bool {
        self.unit_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_value(&mut self, v: DebugSetUnitValue_UnitValue) {
        self.unit_value = ::std::option::Option::Some(v);
    }

    pub fn get_unit_value(&self) -> DebugSetUnitValue_UnitValue {
        self.unit_value.unwrap_or(DebugSetUnitValue_UnitValue::Energy)
    }

    fn get_unit_value_for_reflect(&self) -> &::std::option::Option<DebugSetUnitValue_UnitValue> {
        &self.unit_value
    }

    fn mut_unit_value_for_reflect(&mut self) -> &mut ::std::option::Option<DebugSetUnitValue_UnitValue> {
        &mut self.unit_value
    }

    // optional float value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f32) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> f32 {
        self.value.unwrap_or(0.)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.value
    }

    // optional uint64 unit_tag = 3;

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

impl ::protobuf::Message for DebugSetUnitValue {
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
                    self.unit_value = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if let Some(v) = self.unit_value {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.value {
            my_size += 5;
        }
        if let Some(v) = self.unit_tag {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_value {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.value {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.unit_tag {
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

impl ::protobuf::MessageStatic for DebugSetUnitValue {
    fn new() -> DebugSetUnitValue {
        DebugSetUnitValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugSetUnitValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DebugSetUnitValue_UnitValue>>(
                    "unit_value",
                    DebugSetUnitValue::get_unit_value_for_reflect,
                    DebugSetUnitValue::mut_unit_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "value",
                    DebugSetUnitValue::get_value_for_reflect,
                    DebugSetUnitValue::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "unit_tag",
                    DebugSetUnitValue::get_unit_tag_for_reflect,
                    DebugSetUnitValue::mut_unit_tag_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugSetUnitValue>(
                    "DebugSetUnitValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugSetUnitValue {
    fn clear(&mut self) {
        self.clear_unit_value();
        self.clear_value();
        self.clear_unit_tag();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugSetUnitValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugSetUnitValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DebugSetUnitValue_UnitValue {
    Energy = 1,
    Life = 2,
    Shields = 3,
}

impl ::protobuf::ProtobufEnum for DebugSetUnitValue_UnitValue {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DebugSetUnitValue_UnitValue> {
        match value {
            1 => ::std::option::Option::Some(DebugSetUnitValue_UnitValue::Energy),
            2 => ::std::option::Option::Some(DebugSetUnitValue_UnitValue::Life),
            3 => ::std::option::Option::Some(DebugSetUnitValue_UnitValue::Shields),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DebugSetUnitValue_UnitValue] = &[
            DebugSetUnitValue_UnitValue::Energy,
            DebugSetUnitValue_UnitValue::Life,
            DebugSetUnitValue_UnitValue::Shields,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DebugSetUnitValue_UnitValue>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DebugSetUnitValue_UnitValue", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DebugSetUnitValue_UnitValue {
}

impl ::protobuf::reflect::ProtobufValue for DebugSetUnitValue_UnitValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DebugChat {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DebugChat {}

impl DebugChat {
    pub fn new() -> DebugChat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DebugChat {
        static mut instance: ::protobuf::lazy::Lazy<DebugChat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DebugChat,
        };
        unsafe {
            instance.get(DebugChat::new)
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }
}

impl ::protobuf::Message for DebugChat {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
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
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for DebugChat {
    fn new() -> DebugChat {
        DebugChat::new()
    }

    fn descriptor_static(_: ::std::option::Option<DebugChat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    DebugChat::get_message_for_reflect,
                    DebugChat::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DebugChat>(
                    "DebugChat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DebugChat {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DebugChat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DebugChat {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DebugGameState {
    show_map = 1,
    control_enemy = 2,
    food = 3,
    free = 4,
    all_resources = 5,
    god = 6,
    minerals = 7,
    gas = 8,
    cooldown = 9,
    tech_tree = 10,
    upgrade = 11,
    fast_build = 12,
}

impl ::protobuf::ProtobufEnum for DebugGameState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DebugGameState> {
        match value {
            1 => ::std::option::Option::Some(DebugGameState::show_map),
            2 => ::std::option::Option::Some(DebugGameState::control_enemy),
            3 => ::std::option::Option::Some(DebugGameState::food),
            4 => ::std::option::Option::Some(DebugGameState::free),
            5 => ::std::option::Option::Some(DebugGameState::all_resources),
            6 => ::std::option::Option::Some(DebugGameState::god),
            7 => ::std::option::Option::Some(DebugGameState::minerals),
            8 => ::std::option::Option::Some(DebugGameState::gas),
            9 => ::std::option::Option::Some(DebugGameState::cooldown),
            10 => ::std::option::Option::Some(DebugGameState::tech_tree),
            11 => ::std::option::Option::Some(DebugGameState::upgrade),
            12 => ::std::option::Option::Some(DebugGameState::fast_build),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DebugGameState] = &[
            DebugGameState::show_map,
            DebugGameState::control_enemy,
            DebugGameState::food,
            DebugGameState::free,
            DebugGameState::all_resources,
            DebugGameState::god,
            DebugGameState::minerals,
            DebugGameState::gas,
            DebugGameState::cooldown,
            DebugGameState::tech_tree,
            DebugGameState::upgrade,
            DebugGameState::fast_build,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DebugGameState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DebugGameState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DebugGameState {
}

impl ::protobuf::reflect::ProtobufValue for DebugGameState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cs2clientprotocol/debug.proto\x12\x0eSC2APIProtocol\x1a\x1ds2client\
    protocol/common.proto\"\xbb\x04\n\x0cDebugCommand\x12/\n\x04draw\x18\x01\
    \x20\x01(\x0b2\x19.SC2APIProtocol.DebugDrawH\0R\x04draw\x12?\n\ngame_sta\
    te\x18\x02\x20\x01(\x0e2\x1e.SC2APIProtocol.DebugGameStateH\0R\tgameStat\
    e\x12B\n\x0bcreate_unit\x18\x03\x20\x01(\x0b2\x1f.SC2APIProtocol.DebugCr\
    eateUnitH\0R\ncreateUnit\x12<\n\tkill_unit\x18\x04\x20\x01(\x0b2\x1d.SC2\
    APIProtocol.DebugKillUnitH\0R\x08killUnit\x12E\n\x0ctest_process\x18\x05\
    \x20\x01(\x0b2\x20.SC2APIProtocol.DebugTestProcessH\0R\x0btestProcess\
    \x125\n\x05score\x18\x06\x20\x01(\x0b2\x1d.SC2APIProtocol.DebugSetScoreH\
    \0R\x05score\x129\n\x08end_game\x18\x07\x20\x01(\x0b2\x1c.SC2APIProtocol\
    .DebugEndGameH\0R\x07endGame\x12B\n\nunit_value\x18\x08\x20\x01(\x0b2!.S\
    C2APIProtocol.DebugSetUnitValueH\0R\tunitValue\x12/\n\x04chat\x18\t\x20\
    \x01(\x0b2\x19.SC2APIProtocol.DebugChatH\0R\x04chatB\t\n\x07command\"\
    \xd2\x01\n\tDebugDraw\x12-\n\x04text\x18\x01\x20\x03(\x0b2\x19.SC2APIPro\
    tocol.DebugTextR\x04text\x12/\n\x05lines\x18\x02\x20\x03(\x0b2\x19.SC2AP\
    IProtocol.DebugLineR\x05lines\x12.\n\x05boxes\x18\x03\x20\x03(\x0b2\x18.\
    SC2APIProtocol.DebugBoxR\x05boxes\x125\n\x07spheres\x18\x04\x20\x03(\x0b\
    2\x1b.SC2APIProtocol.DebugSphereR\x07spheres\"T\n\x04Line\x12%\n\x02p0\
    \x18\x01\x20\x01(\x0b2\x15.SC2APIProtocol.PointR\x02p0\x12%\n\x02p1\x18\
    \x02\x20\x01(\x0b2\x15.SC2APIProtocol.PointR\x02p1\"1\n\x05Color\x12\x0c\
    \n\x01r\x18\x01\x20\x01(\rR\x01r\x12\x0c\n\x01g\x18\x02\x20\x01(\rR\x01g\
    \x12\x0c\n\x01b\x18\x03\x20\x01(\rR\x01b\"\xb8\x01\n\tDebugText\x12+\n\
    \x05color\x18\x01\x20\x01(\x0b2\x15.SC2APIProtocol.ColorR\x05color\x12\
    \x12\n\x04text\x18\x02\x20\x01(\tR\x04text\x126\n\x0bvirtual_pos\x18\x03\
    \x20\x01(\x0b2\x15.SC2APIProtocol.PointR\nvirtualPos\x122\n\tworld_pos\
    \x18\x04\x20\x01(\x0b2\x15.SC2APIProtocol.PointR\x08worldPos\"b\n\tDebug\
    Line\x12+\n\x05color\x18\x01\x20\x01(\x0b2\x15.SC2APIProtocol.ColorR\x05\
    color\x12(\n\x04line\x18\x02\x20\x01(\x0b2\x14.SC2APIProtocol.LineR\x04l\
    ine\"\x89\x01\n\x08DebugBox\x12+\n\x05color\x18\x01\x20\x01(\x0b2\x15.SC\
    2APIProtocol.ColorR\x05color\x12'\n\x03min\x18\x02\x20\x01(\x0b2\x15.SC2\
    APIProtocol.PointR\x03min\x12'\n\x03max\x18\x03\x20\x01(\x0b2\x15.SC2API\
    Protocol.PointR\x03max\"m\n\x0bDebugSphere\x12+\n\x05color\x18\x01\x20\
    \x01(\x0b2\x15.SC2APIProtocol.ColorR\x05color\x12#\n\x01p\x18\x02\x20\
    \x01(\x0b2\x15.SC2APIProtocol.PointR\x01p\x12\x0c\n\x01r\x18\x03\x20\x01\
    (\x02R\x01r\"\x8b\x01\n\x0fDebugCreateUnit\x12\x1b\n\tunit_type\x18\x01\
    \x20\x01(\rR\x08unitType\x12\x14\n\x05owner\x18\x02\x20\x01(\x05R\x05own\
    er\x12)\n\x03pos\x18\x03\x20\x01(\x0b2\x17.SC2APIProtocol.Point2DR\x03po\
    s\x12\x1a\n\x08quantity\x18\x04\x20\x01(\rR\x08quantity\"!\n\rDebugKillU\
    nit\x12\x10\n\x03tag\x18\x01\x20\x03(\x04R\x03tag\"\x8f\x01\n\x10DebugTe\
    stProcess\x129\n\x04test\x18\x01\x20\x01(\x0e2%.SC2APIProtocol.DebugTest\
    Process.TestR\x04test\x12\x19\n\x08delay_ms\x18\x02\x20\x01(\x05R\x07del\
    ayMs\"%\n\x04Test\x12\x08\n\x04hang\x10\x01\x12\t\n\x05crash\x10\x02\x12\
    \x08\n\x04exit\x10\x03\"%\n\rDebugSetScore\x12\x14\n\x05score\x18\x01\
    \x20\x01(\x02R\x05score\"\x85\x01\n\x0cDebugEndGame\x12E\n\nend_result\
    \x18\x01\x20\x01(\x0e2&.SC2APIProtocol.DebugEndGame.EndResultR\tendResul\
    t\".\n\tEndResult\x12\r\n\tSurrender\x10\x01\x12\x12\n\x0eDeclareVictory\
    \x10\x02\"\xc0\x01\n\x11DebugSetUnitValue\x12J\n\nunit_value\x18\x01\x20\
    \x01(\x0e2+.SC2APIProtocol.DebugSetUnitValue.UnitValueR\tunitValue\x12\
    \x14\n\x05value\x18\x02\x20\x01(\x02R\x05value\x12\x19\n\x08unit_tag\x18\
    \x03\x20\x01(\x04R\x07unitTag\".\n\tUnitValue\x12\n\n\x06Energy\x10\x01\
    \x12\x08\n\x04Life\x10\x02\x12\x0b\n\x07Shields\x10\x03\"%\n\tDebugChat\
    \x12\x18\n\x07message\x18\x01\x20\x01(\tR\x07message*\xb2\x01\n\x0eDebug\
    GameState\x12\x0c\n\x08show_map\x10\x01\x12\x11\n\rcontrol_enemy\x10\x02\
    \x12\x08\n\x04food\x10\x03\x12\x08\n\x04free\x10\x04\x12\x11\n\rall_reso\
    urces\x10\x05\x12\x07\n\x03god\x10\x06\x12\x0c\n\x08minerals\x10\x07\x12\
    \x07\n\x03gas\x10\x08\x12\x0c\n\x08cooldown\x10\t\x12\r\n\ttech_tree\x10\
    \n\x12\x0b\n\x07upgrade\x10\x0b\x12\x0e\n\nfast_build\x10\x0cJ\xe6$\n\
    \x07\x12\x05\x01\0\x81\x01\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\x08\n\
    \x01\x02\x12\x03\x03\x08\x16\n\t\n\x02\x03\0\x12\x03\x05\x07&\n?\n\x02\
    \x04\0\x12\x04\x08\0\x14\x01\x1a3\x20Issue\x20various\x20useful\x20comma\
    nds\x20to\x20the\x20game\x20engine.\n\n\n\n\x03\x04\0\x01\x12\x03\x08\
    \x08\x14\n\x0c\n\x04\x04\0\x08\0\x12\x04\t\x02\x13\x03\n\x0c\n\x05\x04\0\
    \x08\0\x01\x12\x03\t\x08\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\n\x04\x17\n\
    \x0c\n\x05\x04\0\x02\0\x06\x12\x03\n\x04\r\n\x0c\n\x05\x04\0\x02\0\x01\
    \x12\x03\n\x0e\x12\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\n\x15\x16\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x0b\x04\"\n\x0c\n\x05\x04\0\x02\x01\x06\x12\
    \x03\x0b\x04\x12\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0b\x13\x1d\n\x0c\
    \n\x05\x04\0\x02\x01\x03\x12\x03\x0b\x20!\n\x0b\n\x04\x04\0\x02\x02\x12\
    \x03\x0c\x04$\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x0c\x04\x13\n\x0c\n\
    \x05\x04\0\x02\x02\x01\x12\x03\x0c\x14\x1f\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03\x0c\"#\n\x0b\n\x04\x04\0\x02\x03\x12\x03\r\x04\x20\n\x0c\n\x05\
    \x04\0\x02\x03\x06\x12\x03\r\x04\x11\n\x0c\n\x05\x04\0\x02\x03\x01\x12\
    \x03\r\x12\x1b\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\r\x1e\x1f\n\x0b\n\
    \x04\x04\0\x02\x04\x12\x03\x0e\x04&\n\x0c\n\x05\x04\0\x02\x04\x06\x12\
    \x03\x0e\x04\x14\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x0e\x15!\n\x0c\n\
    \x05\x04\0\x02\x04\x03\x12\x03\x0e$%\n?\n\x04\x04\0\x02\x05\x12\x03\x0f\
    \x04\x1c\"2\x20Useful\x20only\x20for\x20single-player\x20\"curriculum\"\
    \x20maps.\n\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x03\x0f\x04\x11\n\x0c\n\
    \x05\x04\0\x02\x05\x01\x12\x03\x0f\x12\x17\n\x0c\n\x05\x04\0\x02\x05\x03\
    \x12\x03\x0f\x1a\x1b\n\x0b\n\x04\x04\0\x02\x06\x12\x03\x10\x04\x1e\n\x0c\
    \n\x05\x04\0\x02\x06\x06\x12\x03\x10\x04\x10\n\x0c\n\x05\x04\0\x02\x06\
    \x01\x12\x03\x10\x11\x19\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x10\x1c\
    \x1d\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x11\x04%\n\x0c\n\x05\x04\0\x02\
    \x07\x06\x12\x03\x11\x04\x15\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x11\
    \x16\x20\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x11#$\n\x14\n\x04\x04\0\
    \x02\x08\x12\x03\x12\x04\x17\"\x07\x20TODO.\n\n\x0c\n\x05\x04\0\x02\x08\
    \x06\x12\x03\x12\x04\r\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x12\x0e\x12\
    \n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x12\x15\x16\n\n\n\x02\x04\x01\x12\
    \x04\x16\0\x1b\x01\n\n\n\x03\x04\x01\x01\x12\x03\x16\x08\x11\n\x0b\n\x04\
    \x04\x01\x02\0\x12\x03\x17\x02\x1e\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\
    \x17\x02\n\n\x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x17\x0b\x14\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\x17\x15\x19\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\x17\x1c\x1d\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x18\x02\x1f\n\x0c\n\
    \x05\x04\x01\x02\x01\x04\x12\x03\x18\x02\n\n\x0c\n\x05\x04\x01\x02\x01\
    \x06\x12\x03\x18\x0b\x14\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x18\x15\
    \x1a\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x18\x1d\x1e\n\x0b\n\x04\x04\
    \x01\x02\x02\x12\x03\x19\x02\x1e\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03\
    \x19\x02\n\n\x0c\n\x05\x04\x01\x02\x02\x06\x12\x03\x19\x0b\x13\n\x0c\n\
    \x05\x04\x01\x02\x02\x01\x12\x03\x19\x14\x19\n\x0c\n\x05\x04\x01\x02\x02\
    \x03\x12\x03\x19\x1c\x1d\n\x0b\n\x04\x04\x01\x02\x03\x12\x03\x1a\x02#\n\
    \x0c\n\x05\x04\x01\x02\x03\x04\x12\x03\x1a\x02\n\n\x0c\n\x05\x04\x01\x02\
    \x03\x06\x12\x03\x1a\x0b\x16\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x1a\
    \x17\x1e\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03\x1a!\"\n\n\n\x02\x04\
    \x02\x12\x04\x1d\0\x20\x01\n\n\n\x03\x04\x02\x01\x12\x03\x1d\x08\x0c\n\
    \x0b\n\x04\x04\x02\x02\0\x12\x03\x1e\x02\x18\n\x0c\n\x05\x04\x02\x02\0\
    \x04\x12\x03\x1e\x02\n\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x1e\x0b\x10\
    \n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1e\x11\x13\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x03\x1e\x16\x17\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x1f\
    \x02\x18\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03\x1f\x02\n\n\x0c\n\x05\
    \x04\x02\x02\x01\x06\x12\x03\x1f\x0b\x10\n\x0c\n\x05\x04\x02\x02\x01\x01\
    \x12\x03\x1f\x11\x13\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x1f\x16\x17\
    \n\n\n\x02\x04\x03\x12\x04\"\0&\x01\n\n\n\x03\x04\x03\x01\x12\x03\"\x08\
    \r\n\x0b\n\x04\x04\x03\x02\0\x12\x03#\x02\x18\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03#\x02\n\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03#\x0b\x11\n\x0c\
    \n\x05\x04\x03\x02\0\x01\x12\x03#\x12\x13\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03#\x16\x17\n\x0b\n\x04\x04\x03\x02\x01\x12\x03$\x02\x18\n\x0c\n\
    \x05\x04\x03\x02\x01\x04\x12\x03$\x02\n\n\x0c\n\x05\x04\x03\x02\x01\x05\
    \x12\x03$\x0b\x11\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03$\x12\x13\n\x0c\
    \n\x05\x04\x03\x02\x01\x03\x12\x03$\x16\x17\n\x0b\n\x04\x04\x03\x02\x02\
    \x12\x03%\x02\x18\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03%\x02\n\n\x0c\n\
    \x05\x04\x03\x02\x02\x05\x12\x03%\x0b\x11\n\x0c\n\x05\x04\x03\x02\x02\
    \x01\x12\x03%\x12\x13\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03%\x16\x17\n\
    +\n\x02\x04\x04\x12\x04)\0.\x01\x1a\x1f\x20Display\x20debug\x20text\x20o\
    n\x20screen.\n\n\n\n\x03\x04\x04\x01\x12\x03)\x08\x11\n\x0b\n\x04\x04\
    \x04\x02\0\x12\x03*\x02\x1b\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03*\x02\n\
    \n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03*\x0b\x10\n\x0c\n\x05\x04\x04\x02\
    \0\x01\x12\x03*\x11\x16\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03*\x19\x1a\n\
    \x1f\n\x04\x04\x04\x02\x01\x12\x03+\x02\x1b\"\x12\x20Text\x20to\x20displ\
    ay.\n\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03+\x02\n\n\x0c\n\x05\x04\x04\
    \x02\x01\x05\x12\x03+\x0b\x11\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03+\
    \x12\x16\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03+\x19\x1a\nX\n\x04\x04\
    \x04\x02\x02\x12\x03,\x02!\"K\x20Virtualized\x20position\x20in\x202D\x20\
    (the\x20screen\x20is\x200..1,\x200..1\x20for\x20any\x20resolution).\n\n\
    \x0c\n\x05\x04\x04\x02\x02\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\x04\x02\
    \x02\x06\x12\x03,\x0b\x10\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03,\x11\
    \x1c\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03,\x1f\x20\n%\n\x04\x04\x04\
    \x02\x03\x12\x03-\x02\x1f\"\x18\x20Position\x20in\x20the\x20world.\n\n\
    \x0c\n\x05\x04\x04\x02\x03\x04\x12\x03-\x02\n\n\x0c\n\x05\x04\x04\x02\
    \x03\x06\x12\x03-\x0b\x10\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03-\x11\
    \x1a\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03-\x1d\x1e\n,\n\x02\x04\x05\
    \x12\x041\04\x01\x1a\x20\x20Display\x20debug\x20lines\x20on\x20screen.\n\
    \n\n\n\x03\x04\x05\x01\x12\x031\x08\x11\n\x0b\n\x04\x04\x05\x02\0\x12\
    \x032\x02\x1b\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x032\x02\n\n\x0c\n\x05\
    \x04\x05\x02\0\x06\x12\x032\x0b\x10\n\x0c\n\x05\x04\x05\x02\0\x01\x12\
    \x032\x11\x16\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x032\x19\x1a\n\x20\n\x04\
    \x04\x05\x02\x01\x12\x033\x02\x19\"\x13\x20World\x20space\x20line.\n\n\
    \x0c\n\x05\x04\x05\x02\x01\x04\x12\x033\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x01\x06\x12\x033\x0b\x0f\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x033\x10\
    \x14\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x033\x17\x18\n,\n\x02\x04\x06\
    \x12\x047\0;\x01\x1a\x20\x20Display\x20debug\x20boxes\x20on\x20screen.\n\
    \n\n\n\x03\x04\x06\x01\x12\x037\x08\x10\n\x0b\n\x04\x04\x06\x02\0\x12\
    \x038\x02\x1b\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x038\x02\n\n\x0c\n\x05\
    \x04\x06\x02\0\x06\x12\x038\x0b\x10\n\x0c\n\x05\x04\x06\x02\0\x01\x12\
    \x038\x11\x16\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x038\x19\x1a\n\x0b\n\x04\
    \x04\x06\x02\x01\x12\x039\x02\x19\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\
    \x039\x02\n\n\x0c\n\x05\x04\x06\x02\x01\x06\x12\x039\x0b\x10\n\x0c\n\x05\
    \x04\x06\x02\x01\x01\x12\x039\x11\x14\n\x0c\n\x05\x04\x06\x02\x01\x03\
    \x12\x039\x17\x18\n\x0b\n\x04\x04\x06\x02\x02\x12\x03:\x02\x19\n\x0c\n\
    \x05\x04\x06\x02\x02\x04\x12\x03:\x02\n\n\x0c\n\x05\x04\x06\x02\x02\x06\
    \x12\x03:\x0b\x10\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x03:\x11\x14\n\x0c\
    \n\x05\x04\x06\x02\x02\x03\x12\x03:\x17\x18\n.\n\x02\x04\x07\x12\x04>\0B\
    \x01\x1a\"\x20Display\x20debug\x20spheres\x20on\x20screen.\n\n\n\n\x03\
    \x04\x07\x01\x12\x03>\x08\x13\n\x0b\n\x04\x04\x07\x02\0\x12\x03?\x02\x1b\
    \n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03?\x02\n\n\x0c\n\x05\x04\x07\x02\0\
    \x06\x12\x03?\x0b\x10\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03?\x11\x16\n\
    \x0c\n\x05\x04\x07\x02\0\x03\x12\x03?\x19\x1a\n\x0b\n\x04\x04\x07\x02\
    \x01\x12\x03@\x02\x17\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\x03@\x02\n\n\
    \x0c\n\x05\x04\x07\x02\x01\x06\x12\x03@\x0b\x10\n\x0c\n\x05\x04\x07\x02\
    \x01\x01\x12\x03@\x11\x12\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03@\x15\
    \x16\n\x0b\n\x04\x04\x07\x02\x02\x12\x03A\x02\x17\n\x0c\n\x05\x04\x07\
    \x02\x02\x04\x12\x03A\x02\n\n\x0c\n\x05\x04\x07\x02\x02\x05\x12\x03A\x0b\
    \x10\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\x03A\x11\x12\n\x0c\n\x05\x04\
    \x07\x02\x02\x03\x12\x03A\x15\x16\n\n\n\x02\x05\0\x12\x04D\0Q\x01\n\n\n\
    \x03\x05\0\x01\x12\x03D\x05\x13\n\x0b\n\x04\x05\0\x02\0\x12\x03E\x02\x0f\
    \n\x0c\n\x05\x05\0\x02\0\x01\x12\x03E\x02\n\n\x0c\n\x05\x05\0\x02\0\x02\
    \x12\x03E\r\x0e\n\x0b\n\x04\x05\0\x02\x01\x12\x03F\x02\x14\n\x0c\n\x05\
    \x05\0\x02\x01\x01\x12\x03F\x02\x0f\n\x0c\n\x05\x05\0\x02\x01\x02\x12\
    \x03F\x12\x13\n\x0b\n\x04\x05\0\x02\x02\x12\x03G\x02\x0b\n\x0c\n\x05\x05\
    \0\x02\x02\x01\x12\x03G\x02\x06\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03G\t\
    \n\n\x0b\n\x04\x05\0\x02\x03\x12\x03H\x02\x0b\n\x0c\n\x05\x05\0\x02\x03\
    \x01\x12\x03H\x02\x06\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03H\t\n\n\x0b\n\
    \x04\x05\0\x02\x04\x12\x03I\x02\x14\n\x0c\n\x05\x05\0\x02\x04\x01\x12\
    \x03I\x02\x0f\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03I\x12\x13\n\x0b\n\x04\
    \x05\0\x02\x05\x12\x03J\x02\n\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03J\x02\
    \x05\n\x0c\n\x05\x05\0\x02\x05\x02\x12\x03J\x08\t\n\x0b\n\x04\x05\0\x02\
    \x06\x12\x03K\x02\x0f\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03K\x02\n\n\x0c\
    \n\x05\x05\0\x02\x06\x02\x12\x03K\r\x0e\n\x0b\n\x04\x05\0\x02\x07\x12\
    \x03L\x02\n\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03L\x02\x05\n\x0c\n\x05\
    \x05\0\x02\x07\x02\x12\x03L\x08\t\n\x0b\n\x04\x05\0\x02\x08\x12\x03M\x02\
    \x0f\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03M\x02\n\n\x0c\n\x05\x05\0\x02\
    \x08\x02\x12\x03M\r\x0e\n\x0b\n\x04\x05\0\x02\t\x12\x03N\x02\x11\n\x0c\n\
    \x05\x05\0\x02\t\x01\x12\x03N\x02\x0b\n\x0c\n\x05\x05\0\x02\t\x02\x12\
    \x03N\x0e\x10\n\x0b\n\x04\x05\0\x02\n\x12\x03O\x02\x0f\n\x0c\n\x05\x05\0\
    \x02\n\x01\x12\x03O\x02\t\n\x0c\n\x05\x05\0\x02\n\x02\x12\x03O\x0c\x0e\n\
    \x0b\n\x04\x05\0\x02\x0b\x12\x03P\x02\x12\n\x0c\n\x05\x05\0\x02\x0b\x01\
    \x12\x03P\x02\x0c\n\x0c\n\x05\x05\0\x02\x0b\x02\x12\x03P\x0f\x11\n\n\n\
    \x02\x04\x08\x12\x04S\0X\x01\n\n\n\x03\x04\x08\x01\x12\x03S\x08\x17\n\
    \x0b\n\x04\x04\x08\x02\0\x12\x03T\x02\x20\n\x0c\n\x05\x04\x08\x02\0\x04\
    \x12\x03T\x02\n\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03T\x0b\x11\n\x0c\n\
    \x05\x04\x08\x02\0\x01\x12\x03T\x12\x1b\n\x0c\n\x05\x04\x08\x02\0\x03\
    \x12\x03T\x1e\x1f\n\x0b\n\x04\x04\x08\x02\x01\x12\x03U\x02\x1b\n\x0c\n\
    \x05\x04\x08\x02\x01\x04\x12\x03U\x02\n\n\x0c\n\x05\x04\x08\x02\x01\x05\
    \x12\x03U\x0b\x10\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03U\x11\x16\n\x0c\
    \n\x05\x04\x08\x02\x01\x03\x12\x03U\x19\x1a\n\x0b\n\x04\x04\x08\x02\x02\
    \x12\x03V\x02\x1b\n\x0c\n\x05\x04\x08\x02\x02\x04\x12\x03V\x02\n\n\x0c\n\
    \x05\x04\x08\x02\x02\x06\x12\x03V\x0b\x12\n\x0c\n\x05\x04\x08\x02\x02\
    \x01\x12\x03V\x13\x16\n\x0c\n\x05\x04\x08\x02\x02\x03\x12\x03V\x19\x1a\n\
    \x0b\n\x04\x04\x08\x02\x03\x12\x03W\x02\x1f\n\x0c\n\x05\x04\x08\x02\x03\
    \x04\x12\x03W\x02\n\n\x0c\n\x05\x04\x08\x02\x03\x05\x12\x03W\x0b\x11\n\
    \x0c\n\x05\x04\x08\x02\x03\x01\x12\x03W\x12\x1a\n\x0c\n\x05\x04\x08\x02\
    \x03\x03\x12\x03W\x1d\x1e\n\n\n\x02\x04\t\x12\x04Z\0\\\x01\n\n\n\x03\x04\
    \t\x01\x12\x03Z\x08\x15\n\x0b\n\x04\x04\t\x02\0\x12\x03[\x02\x1a\n\x0c\n\
    \x05\x04\t\x02\0\x04\x12\x03[\x02\n\n\x0c\n\x05\x04\t\x02\0\x05\x12\x03[\
    \x0b\x11\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03[\x12\x15\n\x0c\n\x05\x04\t\
    \x02\0\x03\x12\x03[\x18\x19\n\n\n\x02\x04\n\x12\x04^\0f\x01\n\n\n\x03\
    \x04\n\x01\x12\x03^\x08\x18\n\x0c\n\x04\x04\n\x04\0\x12\x04_\x02c\x03\n\
    \x0c\n\x05\x04\n\x04\0\x01\x12\x03_\x07\x0b\n\r\n\x06\x04\n\x04\0\x02\0\
    \x12\x03`\x04\r\n\x0e\n\x07\x04\n\x04\0\x02\0\x01\x12\x03`\x04\x08\n\x0e\
    \n\x07\x04\n\x04\0\x02\0\x02\x12\x03`\x0b\x0c\n\r\n\x06\x04\n\x04\0\x02\
    \x01\x12\x03a\x04\x0e\n\x0e\n\x07\x04\n\x04\0\x02\x01\x01\x12\x03a\x04\t\
    \n\x0e\n\x07\x04\n\x04\0\x02\x01\x02\x12\x03a\x0c\r\n\r\n\x06\x04\n\x04\
    \0\x02\x02\x12\x03b\x04\r\n\x0e\n\x07\x04\n\x04\0\x02\x02\x01\x12\x03b\
    \x04\x08\n\x0e\n\x07\x04\n\x04\0\x02\x02\x02\x12\x03b\x0b\x0c\n\x0b\n\
    \x04\x04\n\x02\0\x12\x03d\x02\x19\n\x0c\n\x05\x04\n\x02\0\x04\x12\x03d\
    \x02\n\n\x0c\n\x05\x04\n\x02\0\x06\x12\x03d\x0b\x0f\n\x0c\n\x05\x04\n\
    \x02\0\x01\x12\x03d\x10\x14\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03d\x17\x18\
    \n\x0b\n\x04\x04\n\x02\x01\x12\x03e\x02\x1e\n\x0c\n\x05\x04\n\x02\x01\
    \x04\x12\x03e\x02\n\n\x0c\n\x05\x04\n\x02\x01\x05\x12\x03e\x0b\x10\n\x0c\
    \n\x05\x04\n\x02\x01\x01\x12\x03e\x11\x19\n\x0c\n\x05\x04\n\x02\x01\x03\
    \x12\x03e\x1c\x1d\n\n\n\x02\x04\x0b\x12\x04h\0j\x01\n\n\n\x03\x04\x0b\
    \x01\x12\x03h\x08\x15\n\x0b\n\x04\x04\x0b\x02\0\x12\x03i\x02\x1b\n\x0c\n\
    \x05\x04\x0b\x02\0\x04\x12\x03i\x02\n\n\x0c\n\x05\x04\x0b\x02\0\x05\x12\
    \x03i\x0b\x10\n\x0c\n\x05\x04\x0b\x02\0\x01\x12\x03i\x11\x16\n\x0c\n\x05\
    \x04\x0b\x02\0\x03\x12\x03i\x19\x1a\n\n\n\x02\x04\x0c\x12\x04l\0r\x01\n\
    \n\n\x03\x04\x0c\x01\x12\x03l\x08\x14\n\x0c\n\x04\x04\x0c\x04\0\x12\x04m\
    \x02p\x03\n\x0c\n\x05\x04\x0c\x04\0\x01\x12\x03m\x07\x10\nM\n\x06\x04\
    \x0c\x04\0\x02\0\x12\x03n\x04\x12\">\x20Default\x20if\x20nothing\x20is\
    \x20set.\x20The\x20current\x20player\x20admits\x20defeat.\n\n\x0e\n\x07\
    \x04\x0c\x04\0\x02\0\x01\x12\x03n\x04\r\n\x0e\n\x07\x04\x0c\x04\0\x02\0\
    \x02\x12\x03n\x10\x11\n\r\n\x06\x04\x0c\x04\0\x02\x01\x12\x03o\x04\x17\n\
    \x0e\n\x07\x04\x0c\x04\0\x02\x01\x01\x12\x03o\x04\x12\n\x0e\n\x07\x04\
    \x0c\x04\0\x02\x01\x02\x12\x03o\x15\x16\n\x0b\n\x04\x04\x0c\x02\0\x12\
    \x03q\x02$\n\x0c\n\x05\x04\x0c\x02\0\x04\x12\x03q\x02\n\n\x0c\n\x05\x04\
    \x0c\x02\0\x06\x12\x03q\x0b\x14\n\x0c\n\x05\x04\x0c\x02\0\x01\x12\x03q\
    \x15\x1f\n\x0c\n\x05\x04\x0c\x02\0\x03\x12\x03q\"#\n\n\n\x02\x04\r\x12\
    \x04t\0}\x01\n\n\n\x03\x04\r\x01\x12\x03t\x08\x19\n\x0c\n\x04\x04\r\x04\
    \0\x12\x04u\x02y\x03\n\x0c\n\x05\x04\r\x04\0\x01\x12\x03u\x07\x10\n\r\n\
    \x06\x04\r\x04\0\x02\0\x12\x03v\x04\x0f\n\x0e\n\x07\x04\r\x04\0\x02\0\
    \x01\x12\x03v\x04\n\n\x0e\n\x07\x04\r\x04\0\x02\0\x02\x12\x03v\r\x0e\n\r\
    \n\x06\x04\r\x04\0\x02\x01\x12\x03w\x04\r\n\x0e\n\x07\x04\r\x04\0\x02\
    \x01\x01\x12\x03w\x04\x08\n\x0e\n\x07\x04\r\x04\0\x02\x01\x02\x12\x03w\
    \x0b\x0c\n\r\n\x06\x04\r\x04\0\x02\x02\x12\x03x\x04\x10\n\x0e\n\x07\x04\
    \r\x04\0\x02\x02\x01\x12\x03x\x04\x0b\n\x0e\n\x07\x04\r\x04\0\x02\x02\
    \x02\x12\x03x\x0e\x0f\n\x0b\n\x04\x04\r\x02\0\x12\x03z\x02$\n\x0c\n\x05\
    \x04\r\x02\0\x04\x12\x03z\x02\n\n\x0c\n\x05\x04\r\x02\0\x06\x12\x03z\x0b\
    \x14\n\x0c\n\x05\x04\r\x02\0\x01\x12\x03z\x15\x1f\n\x0c\n\x05\x04\r\x02\
    \0\x03\x12\x03z\"#\n\x0b\n\x04\x04\r\x02\x01\x12\x03{\x02\x1b\n\x0c\n\
    \x05\x04\r\x02\x01\x04\x12\x03{\x02\n\n\x0c\n\x05\x04\r\x02\x01\x05\x12\
    \x03{\x0b\x10\n\x0c\n\x05\x04\r\x02\x01\x01\x12\x03{\x11\x16\n\x0c\n\x05\
    \x04\r\x02\x01\x03\x12\x03{\x19\x1a\n\x0b\n\x04\x04\r\x02\x02\x12\x03|\
    \x02\x1f\n\x0c\n\x05\x04\r\x02\x02\x04\x12\x03|\x02\n\n\x0c\n\x05\x04\r\
    \x02\x02\x05\x12\x03|\x0b\x11\n\x0c\n\x05\x04\r\x02\x02\x01\x12\x03|\x12\
    \x1a\n\x0c\n\x05\x04\r\x02\x02\x03\x12\x03|\x1d\x1e\n\x0b\n\x02\x04\x0e\
    \x12\x05\x7f\0\x81\x01\x01\n\n\n\x03\x04\x0e\x01\x12\x03\x7f\x08\x11\n\
    \x0c\n\x04\x04\x0e\x02\0\x12\x04\x80\x01\x04\x20\n\r\n\x05\x04\x0e\x02\0\
    \x04\x12\x04\x80\x01\x04\x0c\n\r\n\x05\x04\x0e\x02\0\x05\x12\x04\x80\x01\
    \r\x13\n\r\n\x05\x04\x0e\x02\0\x01\x12\x04\x80\x01\x14\x1b\n\r\n\x05\x04\
    \x0e\x02\0\x03\x12\x04\x80\x01\x1e\x1f\
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
