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
pub struct Request {
    // message oneof groups
    request: ::std::option::Option<Request_oneof_request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

#[derive(Clone,PartialEq)]
pub enum Request_oneof_request {
    create_game(RequestCreateGame),
    join_game(RequestJoinGame),
    restart_game(RequestRestartGame),
    start_replay(RequestStartReplay),
    leave_game(RequestLeaveGame),
    quick_save(RequestQuickSave),
    quick_load(RequestQuickLoad),
    quit(RequestQuit),
    game_info(RequestGameInfo),
    observation(RequestObservation),
    action(RequestAction),
    step(RequestStep),
    data(RequestData),
    query(super::query::RequestQuery),
    save_replay(RequestSaveReplay),
    replay_info(RequestReplayInfo),
    available_maps(RequestAvailableMaps),
    save_map(RequestSaveMap),
    ping(RequestPing),
    debug(RequestDebug),
}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // optional .SC2APIProtocol.RequestCreateGame create_game = 1;

    pub fn clear_create_game(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_create_game(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::create_game(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_game(&mut self, v: RequestCreateGame) {
        self.request = ::std::option::Option::Some(Request_oneof_request::create_game(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_game(&mut self) -> &mut RequestCreateGame {
        if let ::std::option::Option::Some(Request_oneof_request::create_game(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::create_game(RequestCreateGame::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::create_game(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_game(&mut self) -> RequestCreateGame {
        if self.has_create_game() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::create_game(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestCreateGame::new()
        }
    }

    pub fn get_create_game(&self) -> &RequestCreateGame {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::create_game(ref v)) => v,
            _ => RequestCreateGame::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestJoinGame join_game = 2;

    pub fn clear_join_game(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_join_game(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::join_game(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_join_game(&mut self, v: RequestJoinGame) {
        self.request = ::std::option::Option::Some(Request_oneof_request::join_game(v))
    }

    // Mutable pointer to the field.
    pub fn mut_join_game(&mut self) -> &mut RequestJoinGame {
        if let ::std::option::Option::Some(Request_oneof_request::join_game(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::join_game(RequestJoinGame::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::join_game(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_join_game(&mut self) -> RequestJoinGame {
        if self.has_join_game() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::join_game(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestJoinGame::new()
        }
    }

    pub fn get_join_game(&self) -> &RequestJoinGame {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::join_game(ref v)) => v,
            _ => RequestJoinGame::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestRestartGame restart_game = 3;

    pub fn clear_restart_game(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_restart_game(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::restart_game(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_restart_game(&mut self, v: RequestRestartGame) {
        self.request = ::std::option::Option::Some(Request_oneof_request::restart_game(v))
    }

    // Mutable pointer to the field.
    pub fn mut_restart_game(&mut self) -> &mut RequestRestartGame {
        if let ::std::option::Option::Some(Request_oneof_request::restart_game(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::restart_game(RequestRestartGame::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::restart_game(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_restart_game(&mut self) -> RequestRestartGame {
        if self.has_restart_game() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::restart_game(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestRestartGame::new()
        }
    }

    pub fn get_restart_game(&self) -> &RequestRestartGame {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::restart_game(ref v)) => v,
            _ => RequestRestartGame::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestStartReplay start_replay = 4;

    pub fn clear_start_replay(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_start_replay(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::start_replay(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_start_replay(&mut self, v: RequestStartReplay) {
        self.request = ::std::option::Option::Some(Request_oneof_request::start_replay(v))
    }

    // Mutable pointer to the field.
    pub fn mut_start_replay(&mut self) -> &mut RequestStartReplay {
        if let ::std::option::Option::Some(Request_oneof_request::start_replay(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::start_replay(RequestStartReplay::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::start_replay(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_start_replay(&mut self) -> RequestStartReplay {
        if self.has_start_replay() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::start_replay(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestStartReplay::new()
        }
    }

    pub fn get_start_replay(&self) -> &RequestStartReplay {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::start_replay(ref v)) => v,
            _ => RequestStartReplay::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestLeaveGame leave_game = 5;

    pub fn clear_leave_game(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_leave_game(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::leave_game(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_leave_game(&mut self, v: RequestLeaveGame) {
        self.request = ::std::option::Option::Some(Request_oneof_request::leave_game(v))
    }

    // Mutable pointer to the field.
    pub fn mut_leave_game(&mut self) -> &mut RequestLeaveGame {
        if let ::std::option::Option::Some(Request_oneof_request::leave_game(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::leave_game(RequestLeaveGame::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::leave_game(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_leave_game(&mut self) -> RequestLeaveGame {
        if self.has_leave_game() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::leave_game(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestLeaveGame::new()
        }
    }

    pub fn get_leave_game(&self) -> &RequestLeaveGame {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::leave_game(ref v)) => v,
            _ => RequestLeaveGame::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestQuickSave quick_save = 6;

    pub fn clear_quick_save(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_quick_save(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::quick_save(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_quick_save(&mut self, v: RequestQuickSave) {
        self.request = ::std::option::Option::Some(Request_oneof_request::quick_save(v))
    }

    // Mutable pointer to the field.
    pub fn mut_quick_save(&mut self) -> &mut RequestQuickSave {
        if let ::std::option::Option::Some(Request_oneof_request::quick_save(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::quick_save(RequestQuickSave::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::quick_save(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_quick_save(&mut self) -> RequestQuickSave {
        if self.has_quick_save() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::quick_save(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestQuickSave::new()
        }
    }

    pub fn get_quick_save(&self) -> &RequestQuickSave {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::quick_save(ref v)) => v,
            _ => RequestQuickSave::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestQuickLoad quick_load = 7;

    pub fn clear_quick_load(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_quick_load(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::quick_load(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_quick_load(&mut self, v: RequestQuickLoad) {
        self.request = ::std::option::Option::Some(Request_oneof_request::quick_load(v))
    }

    // Mutable pointer to the field.
    pub fn mut_quick_load(&mut self) -> &mut RequestQuickLoad {
        if let ::std::option::Option::Some(Request_oneof_request::quick_load(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::quick_load(RequestQuickLoad::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::quick_load(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_quick_load(&mut self) -> RequestQuickLoad {
        if self.has_quick_load() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::quick_load(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestQuickLoad::new()
        }
    }

    pub fn get_quick_load(&self) -> &RequestQuickLoad {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::quick_load(ref v)) => v,
            _ => RequestQuickLoad::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestQuit quit = 8;

    pub fn clear_quit(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_quit(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::quit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_quit(&mut self, v: RequestQuit) {
        self.request = ::std::option::Option::Some(Request_oneof_request::quit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_quit(&mut self) -> &mut RequestQuit {
        if let ::std::option::Option::Some(Request_oneof_request::quit(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::quit(RequestQuit::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::quit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_quit(&mut self) -> RequestQuit {
        if self.has_quit() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::quit(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestQuit::new()
        }
    }

    pub fn get_quit(&self) -> &RequestQuit {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::quit(ref v)) => v,
            _ => RequestQuit::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestGameInfo game_info = 9;

    pub fn clear_game_info(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_game_info(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::game_info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_game_info(&mut self, v: RequestGameInfo) {
        self.request = ::std::option::Option::Some(Request_oneof_request::game_info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_game_info(&mut self) -> &mut RequestGameInfo {
        if let ::std::option::Option::Some(Request_oneof_request::game_info(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::game_info(RequestGameInfo::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::game_info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_game_info(&mut self) -> RequestGameInfo {
        if self.has_game_info() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::game_info(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestGameInfo::new()
        }
    }

    pub fn get_game_info(&self) -> &RequestGameInfo {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::game_info(ref v)) => v,
            _ => RequestGameInfo::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestObservation observation = 10;

    pub fn clear_observation(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_observation(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::observation(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_observation(&mut self, v: RequestObservation) {
        self.request = ::std::option::Option::Some(Request_oneof_request::observation(v))
    }

    // Mutable pointer to the field.
    pub fn mut_observation(&mut self) -> &mut RequestObservation {
        if let ::std::option::Option::Some(Request_oneof_request::observation(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::observation(RequestObservation::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::observation(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_observation(&mut self) -> RequestObservation {
        if self.has_observation() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::observation(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestObservation::new()
        }
    }

    pub fn get_observation(&self) -> &RequestObservation {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::observation(ref v)) => v,
            _ => RequestObservation::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestAction action = 11;

    pub fn clear_action(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::action(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: RequestAction) {
        self.request = ::std::option::Option::Some(Request_oneof_request::action(v))
    }

    // Mutable pointer to the field.
    pub fn mut_action(&mut self) -> &mut RequestAction {
        if let ::std::option::Option::Some(Request_oneof_request::action(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::action(RequestAction::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::action(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_action(&mut self) -> RequestAction {
        if self.has_action() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::action(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestAction::new()
        }
    }

    pub fn get_action(&self) -> &RequestAction {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::action(ref v)) => v,
            _ => RequestAction::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestStep step = 12;

    pub fn clear_step(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_step(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::step(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_step(&mut self, v: RequestStep) {
        self.request = ::std::option::Option::Some(Request_oneof_request::step(v))
    }

    // Mutable pointer to the field.
    pub fn mut_step(&mut self) -> &mut RequestStep {
        if let ::std::option::Option::Some(Request_oneof_request::step(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::step(RequestStep::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::step(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_step(&mut self) -> RequestStep {
        if self.has_step() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::step(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestStep::new()
        }
    }

    pub fn get_step(&self) -> &RequestStep {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::step(ref v)) => v,
            _ => RequestStep::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestData data = 13;

    pub fn clear_data(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_data(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::data(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: RequestData) {
        self.request = ::std::option::Option::Some(Request_oneof_request::data(v))
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut RequestData {
        if let ::std::option::Option::Some(Request_oneof_request::data(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::data(RequestData::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::data(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_data(&mut self) -> RequestData {
        if self.has_data() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::data(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestData::new()
        }
    }

    pub fn get_data(&self) -> &RequestData {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::data(ref v)) => v,
            _ => RequestData::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestQuery query = 14;

    pub fn clear_query(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_query(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::query(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: super::query::RequestQuery) {
        self.request = ::std::option::Option::Some(Request_oneof_request::query(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query(&mut self) -> &mut super::query::RequestQuery {
        if let ::std::option::Option::Some(Request_oneof_request::query(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::query(super::query::RequestQuery::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::query(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query(&mut self) -> super::query::RequestQuery {
        if self.has_query() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::query(v)) => v,
                _ => panic!(),
            }
        } else {
            super::query::RequestQuery::new()
        }
    }

    pub fn get_query(&self) -> &super::query::RequestQuery {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::query(ref v)) => v,
            _ => super::query::RequestQuery::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestSaveReplay save_replay = 15;

    pub fn clear_save_replay(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_save_replay(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::save_replay(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_save_replay(&mut self, v: RequestSaveReplay) {
        self.request = ::std::option::Option::Some(Request_oneof_request::save_replay(v))
    }

    // Mutable pointer to the field.
    pub fn mut_save_replay(&mut self) -> &mut RequestSaveReplay {
        if let ::std::option::Option::Some(Request_oneof_request::save_replay(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::save_replay(RequestSaveReplay::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::save_replay(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_save_replay(&mut self) -> RequestSaveReplay {
        if self.has_save_replay() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::save_replay(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestSaveReplay::new()
        }
    }

    pub fn get_save_replay(&self) -> &RequestSaveReplay {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::save_replay(ref v)) => v,
            _ => RequestSaveReplay::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestReplayInfo replay_info = 16;

    pub fn clear_replay_info(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_replay_info(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::replay_info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_replay_info(&mut self, v: RequestReplayInfo) {
        self.request = ::std::option::Option::Some(Request_oneof_request::replay_info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_replay_info(&mut self) -> &mut RequestReplayInfo {
        if let ::std::option::Option::Some(Request_oneof_request::replay_info(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::replay_info(RequestReplayInfo::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::replay_info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_replay_info(&mut self) -> RequestReplayInfo {
        if self.has_replay_info() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::replay_info(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestReplayInfo::new()
        }
    }

    pub fn get_replay_info(&self) -> &RequestReplayInfo {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::replay_info(ref v)) => v,
            _ => RequestReplayInfo::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestAvailableMaps available_maps = 17;

    pub fn clear_available_maps(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_available_maps(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::available_maps(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_available_maps(&mut self, v: RequestAvailableMaps) {
        self.request = ::std::option::Option::Some(Request_oneof_request::available_maps(v))
    }

    // Mutable pointer to the field.
    pub fn mut_available_maps(&mut self) -> &mut RequestAvailableMaps {
        if let ::std::option::Option::Some(Request_oneof_request::available_maps(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::available_maps(RequestAvailableMaps::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::available_maps(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_available_maps(&mut self) -> RequestAvailableMaps {
        if self.has_available_maps() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::available_maps(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestAvailableMaps::new()
        }
    }

    pub fn get_available_maps(&self) -> &RequestAvailableMaps {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::available_maps(ref v)) => v,
            _ => RequestAvailableMaps::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestSaveMap save_map = 18;

    pub fn clear_save_map(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_save_map(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::save_map(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_save_map(&mut self, v: RequestSaveMap) {
        self.request = ::std::option::Option::Some(Request_oneof_request::save_map(v))
    }

    // Mutable pointer to the field.
    pub fn mut_save_map(&mut self) -> &mut RequestSaveMap {
        if let ::std::option::Option::Some(Request_oneof_request::save_map(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::save_map(RequestSaveMap::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::save_map(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_save_map(&mut self) -> RequestSaveMap {
        if self.has_save_map() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::save_map(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestSaveMap::new()
        }
    }

    pub fn get_save_map(&self) -> &RequestSaveMap {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::save_map(ref v)) => v,
            _ => RequestSaveMap::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestPing ping = 19;

    pub fn clear_ping(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_ping(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::ping(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: RequestPing) {
        self.request = ::std::option::Option::Some(Request_oneof_request::ping(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ping(&mut self) -> &mut RequestPing {
        if let ::std::option::Option::Some(Request_oneof_request::ping(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::ping(RequestPing::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::ping(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ping(&mut self) -> RequestPing {
        if self.has_ping() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::ping(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestPing::new()
        }
    }

    pub fn get_ping(&self) -> &RequestPing {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::ping(ref v)) => v,
            _ => RequestPing::default_instance(),
        }
    }

    // optional .SC2APIProtocol.RequestDebug debug = 20;

    pub fn clear_debug(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_debug(&self) -> bool {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::debug(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_debug(&mut self, v: RequestDebug) {
        self.request = ::std::option::Option::Some(Request_oneof_request::debug(v))
    }

    // Mutable pointer to the field.
    pub fn mut_debug(&mut self) -> &mut RequestDebug {
        if let ::std::option::Option::Some(Request_oneof_request::debug(_)) = self.request {
        } else {
            self.request = ::std::option::Option::Some(Request_oneof_request::debug(RequestDebug::new()));
        }
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::debug(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_debug(&mut self) -> RequestDebug {
        if self.has_debug() {
            match self.request.take() {
                ::std::option::Option::Some(Request_oneof_request::debug(v)) => v,
                _ => panic!(),
            }
        } else {
            RequestDebug::new()
        }
    }

    pub fn get_debug(&self) -> &RequestDebug {
        match self.request {
            ::std::option::Option::Some(Request_oneof_request::debug(ref v)) => v,
            _ => RequestDebug::default_instance(),
        }
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        if let Some(Request_oneof_request::create_game(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::join_game(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::restart_game(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::start_replay(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::leave_game(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::quick_save(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::quick_load(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::quit(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::game_info(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::observation(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::action(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::step(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::data(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::query(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::save_replay(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::replay_info(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::available_maps(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::save_map(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::ping(ref v)) = self.request {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Request_oneof_request::debug(ref v)) = self.request {
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
                    self.request = ::std::option::Option::Some(Request_oneof_request::create_game(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::join_game(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::restart_game(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::start_replay(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::leave_game(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::quick_save(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::quick_load(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::quit(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::game_info(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::observation(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::action(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::step(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::data(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::query(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::save_replay(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::replay_info(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::available_maps(is.read_message()?));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::save_map(is.read_message()?));
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::ping(is.read_message()?));
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.request = ::std::option::Option::Some(Request_oneof_request::debug(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &Request_oneof_request::create_game(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::join_game(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::restart_game(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::start_replay(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::leave_game(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::quick_save(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::quick_load(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::quit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::game_info(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::observation(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::action(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::step(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::data(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::query(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::save_replay(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::replay_info(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::available_maps(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::save_map(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::ping(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Request_oneof_request::debug(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.request {
            match v {
                &Request_oneof_request::create_game(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::join_game(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::restart_game(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::start_replay(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::leave_game(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::quick_save(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::quick_load(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::quit(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::game_info(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::observation(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::action(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::step(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::data(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::query(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::save_replay(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::replay_info(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::available_maps(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::save_map(ref v) => {
                    os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::ping(ref v) => {
                    os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Request_oneof_request::debug(ref v) => {
                    os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestCreateGame>(
                    "create_game",
                    Request::has_create_game,
                    Request::get_create_game,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestJoinGame>(
                    "join_game",
                    Request::has_join_game,
                    Request::get_join_game,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestRestartGame>(
                    "restart_game",
                    Request::has_restart_game,
                    Request::get_restart_game,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestStartReplay>(
                    "start_replay",
                    Request::has_start_replay,
                    Request::get_start_replay,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestLeaveGame>(
                    "leave_game",
                    Request::has_leave_game,
                    Request::get_leave_game,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestQuickSave>(
                    "quick_save",
                    Request::has_quick_save,
                    Request::get_quick_save,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestQuickLoad>(
                    "quick_load",
                    Request::has_quick_load,
                    Request::get_quick_load,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestQuit>(
                    "quit",
                    Request::has_quit,
                    Request::get_quit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestGameInfo>(
                    "game_info",
                    Request::has_game_info,
                    Request::get_game_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestObservation>(
                    "observation",
                    Request::has_observation,
                    Request::get_observation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestAction>(
                    "action",
                    Request::has_action,
                    Request::get_action,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestStep>(
                    "step",
                    Request::has_step,
                    Request::get_step,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestData>(
                    "data",
                    Request::has_data,
                    Request::get_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::query::RequestQuery>(
                    "query",
                    Request::has_query,
                    Request::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestSaveReplay>(
                    "save_replay",
                    Request::has_save_replay,
                    Request::get_save_replay,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestReplayInfo>(
                    "replay_info",
                    Request::has_replay_info,
                    Request::get_replay_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestAvailableMaps>(
                    "available_maps",
                    Request::has_available_maps,
                    Request::get_available_maps,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestSaveMap>(
                    "save_map",
                    Request::has_save_map,
                    Request::get_save_map,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestPing>(
                    "ping",
                    Request::has_ping,
                    Request::get_ping,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RequestDebug>(
                    "debug",
                    Request::has_debug,
                    Request::get_debug,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_create_game();
        self.clear_join_game();
        self.clear_restart_game();
        self.clear_start_replay();
        self.clear_leave_game();
        self.clear_quick_save();
        self.clear_quick_load();
        self.clear_quit();
        self.clear_game_info();
        self.clear_observation();
        self.clear_action();
        self.clear_step();
        self.clear_data();
        self.clear_query();
        self.clear_save_replay();
        self.clear_replay_info();
        self.clear_available_maps();
        self.clear_save_map();
        self.clear_ping();
        self.clear_debug();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    error: ::protobuf::RepeatedField<::std::string::String>,
    status: ::std::option::Option<Status>,
    // message oneof groups
    response: ::std::option::Option<Response_oneof_response>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

#[derive(Clone,PartialEq)]
pub enum Response_oneof_response {
    create_game(ResponseCreateGame),
    join_game(ResponseJoinGame),
    restart_game(ResponseRestartGame),
    start_replay(ResponseStartReplay),
    leave_game(ResponseLeaveGame),
    quick_save(ResponseQuickSave),
    quick_load(ResponseQuickLoad),
    quit(ResponseQuit),
    game_info(ResponseGameInfo),
    observation(ResponseObservation),
    action(ResponseAction),
    step(ResponseStep),
    data(ResponseData),
    query(super::query::ResponseQuery),
    save_replay(ResponseSaveReplay),
    replay_info(ResponseReplayInfo),
    available_maps(ResponseAvailableMaps),
    save_map(ResponseSaveMap),
    ping(ResponsePing),
    debug(ResponseDebug),
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // optional .SC2APIProtocol.ResponseCreateGame create_game = 1;

    pub fn clear_create_game(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_create_game(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::create_game(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_create_game(&mut self, v: ResponseCreateGame) {
        self.response = ::std::option::Option::Some(Response_oneof_response::create_game(v))
    }

    // Mutable pointer to the field.
    pub fn mut_create_game(&mut self) -> &mut ResponseCreateGame {
        if let ::std::option::Option::Some(Response_oneof_response::create_game(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::create_game(ResponseCreateGame::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::create_game(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_create_game(&mut self) -> ResponseCreateGame {
        if self.has_create_game() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::create_game(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseCreateGame::new()
        }
    }

    pub fn get_create_game(&self) -> &ResponseCreateGame {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::create_game(ref v)) => v,
            _ => ResponseCreateGame::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseJoinGame join_game = 2;

    pub fn clear_join_game(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_join_game(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::join_game(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_join_game(&mut self, v: ResponseJoinGame) {
        self.response = ::std::option::Option::Some(Response_oneof_response::join_game(v))
    }

    // Mutable pointer to the field.
    pub fn mut_join_game(&mut self) -> &mut ResponseJoinGame {
        if let ::std::option::Option::Some(Response_oneof_response::join_game(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::join_game(ResponseJoinGame::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::join_game(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_join_game(&mut self) -> ResponseJoinGame {
        if self.has_join_game() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::join_game(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseJoinGame::new()
        }
    }

    pub fn get_join_game(&self) -> &ResponseJoinGame {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::join_game(ref v)) => v,
            _ => ResponseJoinGame::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseRestartGame restart_game = 3;

    pub fn clear_restart_game(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_restart_game(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::restart_game(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_restart_game(&mut self, v: ResponseRestartGame) {
        self.response = ::std::option::Option::Some(Response_oneof_response::restart_game(v))
    }

    // Mutable pointer to the field.
    pub fn mut_restart_game(&mut self) -> &mut ResponseRestartGame {
        if let ::std::option::Option::Some(Response_oneof_response::restart_game(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::restart_game(ResponseRestartGame::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::restart_game(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_restart_game(&mut self) -> ResponseRestartGame {
        if self.has_restart_game() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::restart_game(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseRestartGame::new()
        }
    }

    pub fn get_restart_game(&self) -> &ResponseRestartGame {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::restart_game(ref v)) => v,
            _ => ResponseRestartGame::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseStartReplay start_replay = 4;

    pub fn clear_start_replay(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_start_replay(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::start_replay(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_start_replay(&mut self, v: ResponseStartReplay) {
        self.response = ::std::option::Option::Some(Response_oneof_response::start_replay(v))
    }

    // Mutable pointer to the field.
    pub fn mut_start_replay(&mut self) -> &mut ResponseStartReplay {
        if let ::std::option::Option::Some(Response_oneof_response::start_replay(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::start_replay(ResponseStartReplay::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::start_replay(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_start_replay(&mut self) -> ResponseStartReplay {
        if self.has_start_replay() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::start_replay(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseStartReplay::new()
        }
    }

    pub fn get_start_replay(&self) -> &ResponseStartReplay {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::start_replay(ref v)) => v,
            _ => ResponseStartReplay::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseLeaveGame leave_game = 5;

    pub fn clear_leave_game(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_leave_game(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::leave_game(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_leave_game(&mut self, v: ResponseLeaveGame) {
        self.response = ::std::option::Option::Some(Response_oneof_response::leave_game(v))
    }

    // Mutable pointer to the field.
    pub fn mut_leave_game(&mut self) -> &mut ResponseLeaveGame {
        if let ::std::option::Option::Some(Response_oneof_response::leave_game(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::leave_game(ResponseLeaveGame::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::leave_game(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_leave_game(&mut self) -> ResponseLeaveGame {
        if self.has_leave_game() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::leave_game(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseLeaveGame::new()
        }
    }

    pub fn get_leave_game(&self) -> &ResponseLeaveGame {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::leave_game(ref v)) => v,
            _ => ResponseLeaveGame::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseQuickSave quick_save = 6;

    pub fn clear_quick_save(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_quick_save(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::quick_save(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_quick_save(&mut self, v: ResponseQuickSave) {
        self.response = ::std::option::Option::Some(Response_oneof_response::quick_save(v))
    }

    // Mutable pointer to the field.
    pub fn mut_quick_save(&mut self) -> &mut ResponseQuickSave {
        if let ::std::option::Option::Some(Response_oneof_response::quick_save(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::quick_save(ResponseQuickSave::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::quick_save(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_quick_save(&mut self) -> ResponseQuickSave {
        if self.has_quick_save() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::quick_save(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseQuickSave::new()
        }
    }

    pub fn get_quick_save(&self) -> &ResponseQuickSave {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::quick_save(ref v)) => v,
            _ => ResponseQuickSave::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseQuickLoad quick_load = 7;

    pub fn clear_quick_load(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_quick_load(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::quick_load(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_quick_load(&mut self, v: ResponseQuickLoad) {
        self.response = ::std::option::Option::Some(Response_oneof_response::quick_load(v))
    }

    // Mutable pointer to the field.
    pub fn mut_quick_load(&mut self) -> &mut ResponseQuickLoad {
        if let ::std::option::Option::Some(Response_oneof_response::quick_load(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::quick_load(ResponseQuickLoad::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::quick_load(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_quick_load(&mut self) -> ResponseQuickLoad {
        if self.has_quick_load() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::quick_load(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseQuickLoad::new()
        }
    }

    pub fn get_quick_load(&self) -> &ResponseQuickLoad {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::quick_load(ref v)) => v,
            _ => ResponseQuickLoad::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseQuit quit = 8;

    pub fn clear_quit(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_quit(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::quit(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_quit(&mut self, v: ResponseQuit) {
        self.response = ::std::option::Option::Some(Response_oneof_response::quit(v))
    }

    // Mutable pointer to the field.
    pub fn mut_quit(&mut self) -> &mut ResponseQuit {
        if let ::std::option::Option::Some(Response_oneof_response::quit(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::quit(ResponseQuit::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::quit(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_quit(&mut self) -> ResponseQuit {
        if self.has_quit() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::quit(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseQuit::new()
        }
    }

    pub fn get_quit(&self) -> &ResponseQuit {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::quit(ref v)) => v,
            _ => ResponseQuit::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseGameInfo game_info = 9;

    pub fn clear_game_info(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_game_info(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::game_info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_game_info(&mut self, v: ResponseGameInfo) {
        self.response = ::std::option::Option::Some(Response_oneof_response::game_info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_game_info(&mut self) -> &mut ResponseGameInfo {
        if let ::std::option::Option::Some(Response_oneof_response::game_info(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::game_info(ResponseGameInfo::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::game_info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_game_info(&mut self) -> ResponseGameInfo {
        if self.has_game_info() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::game_info(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseGameInfo::new()
        }
    }

    pub fn get_game_info(&self) -> &ResponseGameInfo {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::game_info(ref v)) => v,
            _ => ResponseGameInfo::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseObservation observation = 10;

    pub fn clear_observation(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_observation(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::observation(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_observation(&mut self, v: ResponseObservation) {
        self.response = ::std::option::Option::Some(Response_oneof_response::observation(v))
    }

    // Mutable pointer to the field.
    pub fn mut_observation(&mut self) -> &mut ResponseObservation {
        if let ::std::option::Option::Some(Response_oneof_response::observation(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::observation(ResponseObservation::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::observation(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_observation(&mut self) -> ResponseObservation {
        if self.has_observation() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::observation(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseObservation::new()
        }
    }

    pub fn get_observation(&self) -> &ResponseObservation {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::observation(ref v)) => v,
            _ => ResponseObservation::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseAction action = 11;

    pub fn clear_action(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::action(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: ResponseAction) {
        self.response = ::std::option::Option::Some(Response_oneof_response::action(v))
    }

    // Mutable pointer to the field.
    pub fn mut_action(&mut self) -> &mut ResponseAction {
        if let ::std::option::Option::Some(Response_oneof_response::action(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::action(ResponseAction::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::action(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_action(&mut self) -> ResponseAction {
        if self.has_action() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::action(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseAction::new()
        }
    }

    pub fn get_action(&self) -> &ResponseAction {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::action(ref v)) => v,
            _ => ResponseAction::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseStep step = 12;

    pub fn clear_step(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_step(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::step(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_step(&mut self, v: ResponseStep) {
        self.response = ::std::option::Option::Some(Response_oneof_response::step(v))
    }

    // Mutable pointer to the field.
    pub fn mut_step(&mut self) -> &mut ResponseStep {
        if let ::std::option::Option::Some(Response_oneof_response::step(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::step(ResponseStep::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::step(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_step(&mut self) -> ResponseStep {
        if self.has_step() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::step(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseStep::new()
        }
    }

    pub fn get_step(&self) -> &ResponseStep {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::step(ref v)) => v,
            _ => ResponseStep::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseData data = 13;

    pub fn clear_data(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_data(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::data(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ResponseData) {
        self.response = ::std::option::Option::Some(Response_oneof_response::data(v))
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut ResponseData {
        if let ::std::option::Option::Some(Response_oneof_response::data(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::data(ResponseData::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::data(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_data(&mut self) -> ResponseData {
        if self.has_data() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::data(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseData::new()
        }
    }

    pub fn get_data(&self) -> &ResponseData {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::data(ref v)) => v,
            _ => ResponseData::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseQuery query = 14;

    pub fn clear_query(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_query(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::query(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_query(&mut self, v: super::query::ResponseQuery) {
        self.response = ::std::option::Option::Some(Response_oneof_response::query(v))
    }

    // Mutable pointer to the field.
    pub fn mut_query(&mut self) -> &mut super::query::ResponseQuery {
        if let ::std::option::Option::Some(Response_oneof_response::query(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::query(super::query::ResponseQuery::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::query(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_query(&mut self) -> super::query::ResponseQuery {
        if self.has_query() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::query(v)) => v,
                _ => panic!(),
            }
        } else {
            super::query::ResponseQuery::new()
        }
    }

    pub fn get_query(&self) -> &super::query::ResponseQuery {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::query(ref v)) => v,
            _ => super::query::ResponseQuery::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseSaveReplay save_replay = 15;

    pub fn clear_save_replay(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_save_replay(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::save_replay(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_save_replay(&mut self, v: ResponseSaveReplay) {
        self.response = ::std::option::Option::Some(Response_oneof_response::save_replay(v))
    }

    // Mutable pointer to the field.
    pub fn mut_save_replay(&mut self) -> &mut ResponseSaveReplay {
        if let ::std::option::Option::Some(Response_oneof_response::save_replay(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::save_replay(ResponseSaveReplay::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::save_replay(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_save_replay(&mut self) -> ResponseSaveReplay {
        if self.has_save_replay() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::save_replay(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseSaveReplay::new()
        }
    }

    pub fn get_save_replay(&self) -> &ResponseSaveReplay {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::save_replay(ref v)) => v,
            _ => ResponseSaveReplay::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseReplayInfo replay_info = 16;

    pub fn clear_replay_info(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_replay_info(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::replay_info(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_replay_info(&mut self, v: ResponseReplayInfo) {
        self.response = ::std::option::Option::Some(Response_oneof_response::replay_info(v))
    }

    // Mutable pointer to the field.
    pub fn mut_replay_info(&mut self) -> &mut ResponseReplayInfo {
        if let ::std::option::Option::Some(Response_oneof_response::replay_info(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::replay_info(ResponseReplayInfo::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::replay_info(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_replay_info(&mut self) -> ResponseReplayInfo {
        if self.has_replay_info() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::replay_info(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseReplayInfo::new()
        }
    }

    pub fn get_replay_info(&self) -> &ResponseReplayInfo {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::replay_info(ref v)) => v,
            _ => ResponseReplayInfo::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseAvailableMaps available_maps = 17;

    pub fn clear_available_maps(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_available_maps(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::available_maps(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_available_maps(&mut self, v: ResponseAvailableMaps) {
        self.response = ::std::option::Option::Some(Response_oneof_response::available_maps(v))
    }

    // Mutable pointer to the field.
    pub fn mut_available_maps(&mut self) -> &mut ResponseAvailableMaps {
        if let ::std::option::Option::Some(Response_oneof_response::available_maps(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::available_maps(ResponseAvailableMaps::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::available_maps(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_available_maps(&mut self) -> ResponseAvailableMaps {
        if self.has_available_maps() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::available_maps(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseAvailableMaps::new()
        }
    }

    pub fn get_available_maps(&self) -> &ResponseAvailableMaps {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::available_maps(ref v)) => v,
            _ => ResponseAvailableMaps::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseSaveMap save_map = 18;

    pub fn clear_save_map(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_save_map(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::save_map(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_save_map(&mut self, v: ResponseSaveMap) {
        self.response = ::std::option::Option::Some(Response_oneof_response::save_map(v))
    }

    // Mutable pointer to the field.
    pub fn mut_save_map(&mut self) -> &mut ResponseSaveMap {
        if let ::std::option::Option::Some(Response_oneof_response::save_map(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::save_map(ResponseSaveMap::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::save_map(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_save_map(&mut self) -> ResponseSaveMap {
        if self.has_save_map() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::save_map(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseSaveMap::new()
        }
    }

    pub fn get_save_map(&self) -> &ResponseSaveMap {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::save_map(ref v)) => v,
            _ => ResponseSaveMap::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponsePing ping = 19;

    pub fn clear_ping(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_ping(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::ping(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ping(&mut self, v: ResponsePing) {
        self.response = ::std::option::Option::Some(Response_oneof_response::ping(v))
    }

    // Mutable pointer to the field.
    pub fn mut_ping(&mut self) -> &mut ResponsePing {
        if let ::std::option::Option::Some(Response_oneof_response::ping(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::ping(ResponsePing::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::ping(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_ping(&mut self) -> ResponsePing {
        if self.has_ping() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::ping(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponsePing::new()
        }
    }

    pub fn get_ping(&self) -> &ResponsePing {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::ping(ref v)) => v,
            _ => ResponsePing::default_instance(),
        }
    }

    // optional .SC2APIProtocol.ResponseDebug debug = 20;

    pub fn clear_debug(&mut self) {
        self.response = ::std::option::Option::None;
    }

    pub fn has_debug(&self) -> bool {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::debug(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_debug(&mut self, v: ResponseDebug) {
        self.response = ::std::option::Option::Some(Response_oneof_response::debug(v))
    }

    // Mutable pointer to the field.
    pub fn mut_debug(&mut self) -> &mut ResponseDebug {
        if let ::std::option::Option::Some(Response_oneof_response::debug(_)) = self.response {
        } else {
            self.response = ::std::option::Option::Some(Response_oneof_response::debug(ResponseDebug::new()));
        }
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::debug(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_debug(&mut self) -> ResponseDebug {
        if self.has_debug() {
            match self.response.take() {
                ::std::option::Option::Some(Response_oneof_response::debug(v)) => v,
                _ => panic!(),
            }
        } else {
            ResponseDebug::new()
        }
    }

    pub fn get_debug(&self) -> &ResponseDebug {
        match self.response {
            ::std::option::Option::Some(Response_oneof_response::debug(ref v)) => v,
            _ => ResponseDebug::default_instance(),
        }
    }

    // repeated string error = 98;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.error = v;
    }

    // Mutable pointer to the field.
    pub fn mut_error(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.error
    }

    // Take field
    pub fn take_error(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.error, ::protobuf::RepeatedField::new())
    }

    pub fn get_error(&self) -> &[::std::string::String] {
        &self.error
    }

    fn get_error_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.error
    }

    // optional .SC2APIProtocol.Status status = 99;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> Status {
        self.status.unwrap_or(Status::launched)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<Status> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<Status> {
        &mut self.status
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        if let Some(Response_oneof_response::create_game(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::join_game(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::restart_game(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::start_replay(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::leave_game(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::quick_save(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::quick_load(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::quit(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::game_info(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::observation(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::action(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::step(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::data(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::query(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::save_replay(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::replay_info(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::available_maps(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::save_map(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::ping(ref v)) = self.response {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Response_oneof_response::debug(ref v)) = self.response {
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
                    self.response = ::std::option::Option::Some(Response_oneof_response::create_game(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::join_game(is.read_message()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::restart_game(is.read_message()?));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::start_replay(is.read_message()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::leave_game(is.read_message()?));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::quick_save(is.read_message()?));
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::quick_load(is.read_message()?));
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::quit(is.read_message()?));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::game_info(is.read_message()?));
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::observation(is.read_message()?));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::action(is.read_message()?));
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::step(is.read_message()?));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::data(is.read_message()?));
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::query(is.read_message()?));
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::save_replay(is.read_message()?));
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::replay_info(is.read_message()?));
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::available_maps(is.read_message()?));
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::save_map(is.read_message()?));
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::ping(is.read_message()?));
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.response = ::std::option::Option::Some(Response_oneof_response::debug(is.read_message()?));
                },
                98 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.error)?;
                },
                99 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.status = ::std::option::Option::Some(tmp);
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
        for value in &self.error {
            my_size += ::protobuf::rt::string_size(98, &value);
        };
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::enum_size(99, v);
        }
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &Response_oneof_response::create_game(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::join_game(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::restart_game(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::start_replay(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::leave_game(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::quick_save(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::quick_load(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::quit(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::game_info(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::observation(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::action(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::step(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::data(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::query(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::save_replay(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::replay_info(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::available_maps(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::save_map(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::ping(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Response_oneof_response::debug(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.error {
            os.write_string(98, &v)?;
        };
        if let Some(v) = self.status {
            os.write_enum(99, v.value())?;
        }
        if let ::std::option::Option::Some(ref v) = self.response {
            match v {
                &Response_oneof_response::create_game(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::join_game(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::restart_game(ref v) => {
                    os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::start_replay(ref v) => {
                    os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::leave_game(ref v) => {
                    os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::quick_save(ref v) => {
                    os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::quick_load(ref v) => {
                    os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::quit(ref v) => {
                    os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::game_info(ref v) => {
                    os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::observation(ref v) => {
                    os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::action(ref v) => {
                    os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::step(ref v) => {
                    os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::data(ref v) => {
                    os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::query(ref v) => {
                    os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::save_replay(ref v) => {
                    os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::replay_info(ref v) => {
                    os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::available_maps(ref v) => {
                    os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::save_map(ref v) => {
                    os.write_tag(18, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::ping(ref v) => {
                    os.write_tag(19, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Response_oneof_response::debug(ref v) => {
                    os.write_tag(20, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseCreateGame>(
                    "create_game",
                    Response::has_create_game,
                    Response::get_create_game,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseJoinGame>(
                    "join_game",
                    Response::has_join_game,
                    Response::get_join_game,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseRestartGame>(
                    "restart_game",
                    Response::has_restart_game,
                    Response::get_restart_game,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseStartReplay>(
                    "start_replay",
                    Response::has_start_replay,
                    Response::get_start_replay,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseLeaveGame>(
                    "leave_game",
                    Response::has_leave_game,
                    Response::get_leave_game,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseQuickSave>(
                    "quick_save",
                    Response::has_quick_save,
                    Response::get_quick_save,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseQuickLoad>(
                    "quick_load",
                    Response::has_quick_load,
                    Response::get_quick_load,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseQuit>(
                    "quit",
                    Response::has_quit,
                    Response::get_quit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseGameInfo>(
                    "game_info",
                    Response::has_game_info,
                    Response::get_game_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseObservation>(
                    "observation",
                    Response::has_observation,
                    Response::get_observation,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseAction>(
                    "action",
                    Response::has_action,
                    Response::get_action,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseStep>(
                    "step",
                    Response::has_step,
                    Response::get_step,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseData>(
                    "data",
                    Response::has_data,
                    Response::get_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, super::query::ResponseQuery>(
                    "query",
                    Response::has_query,
                    Response::get_query,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseSaveReplay>(
                    "save_replay",
                    Response::has_save_replay,
                    Response::get_save_replay,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseReplayInfo>(
                    "replay_info",
                    Response::has_replay_info,
                    Response::get_replay_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseAvailableMaps>(
                    "available_maps",
                    Response::has_available_maps,
                    Response::get_available_maps,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseSaveMap>(
                    "save_map",
                    Response::has_save_map,
                    Response::get_save_map,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponsePing>(
                    "ping",
                    Response::has_ping,
                    Response::get_ping,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, ResponseDebug>(
                    "debug",
                    Response::has_debug,
                    Response::get_debug,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    Response::get_error_for_reflect,
                    Response::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Status>>(
                    "status",
                    Response::get_status_for_reflect,
                    Response::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_create_game();
        self.clear_join_game();
        self.clear_restart_game();
        self.clear_start_replay();
        self.clear_leave_game();
        self.clear_quick_save();
        self.clear_quick_load();
        self.clear_quit();
        self.clear_game_info();
        self.clear_observation();
        self.clear_action();
        self.clear_step();
        self.clear_data();
        self.clear_query();
        self.clear_save_replay();
        self.clear_replay_info();
        self.clear_available_maps();
        self.clear_save_map();
        self.clear_ping();
        self.clear_debug();
        self.clear_error();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestCreateGame {
    // message fields
    player_setup: ::protobuf::RepeatedField<PlayerSetup>,
    disable_fog: ::std::option::Option<bool>,
    random_seed: ::std::option::Option<u32>,
    realtime: ::std::option::Option<bool>,
    // message oneof groups
    Map: ::std::option::Option<RequestCreateGame_oneof_Map>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestCreateGame {}

#[derive(Clone,PartialEq)]
pub enum RequestCreateGame_oneof_Map {
    local_map(LocalMap),
    battlenet_map_name(::std::string::String),
}

impl RequestCreateGame {
    pub fn new() -> RequestCreateGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestCreateGame {
        static mut instance: ::protobuf::lazy::Lazy<RequestCreateGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestCreateGame,
        };
        unsafe {
            instance.get(RequestCreateGame::new)
        }
    }

    // optional .SC2APIProtocol.LocalMap local_map = 1;

    pub fn clear_local_map(&mut self) {
        self.Map = ::std::option::Option::None;
    }

    pub fn has_local_map(&self) -> bool {
        match self.Map {
            ::std::option::Option::Some(RequestCreateGame_oneof_Map::local_map(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_local_map(&mut self, v: LocalMap) {
        self.Map = ::std::option::Option::Some(RequestCreateGame_oneof_Map::local_map(v))
    }

    // Mutable pointer to the field.
    pub fn mut_local_map(&mut self) -> &mut LocalMap {
        if let ::std::option::Option::Some(RequestCreateGame_oneof_Map::local_map(_)) = self.Map {
        } else {
            self.Map = ::std::option::Option::Some(RequestCreateGame_oneof_Map::local_map(LocalMap::new()));
        }
        match self.Map {
            ::std::option::Option::Some(RequestCreateGame_oneof_Map::local_map(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_local_map(&mut self) -> LocalMap {
        if self.has_local_map() {
            match self.Map.take() {
                ::std::option::Option::Some(RequestCreateGame_oneof_Map::local_map(v)) => v,
                _ => panic!(),
            }
        } else {
            LocalMap::new()
        }
    }

    pub fn get_local_map(&self) -> &LocalMap {
        match self.Map {
            ::std::option::Option::Some(RequestCreateGame_oneof_Map::local_map(ref v)) => v,
            _ => LocalMap::default_instance(),
        }
    }

    // optional string battlenet_map_name = 2;

    pub fn clear_battlenet_map_name(&mut self) {
        self.Map = ::std::option::Option::None;
    }

    pub fn has_battlenet_map_name(&self) -> bool {
        match self.Map {
            ::std::option::Option::Some(RequestCreateGame_oneof_Map::battlenet_map_name(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_battlenet_map_name(&mut self, v: ::std::string::String) {
        self.Map = ::std::option::Option::Some(RequestCreateGame_oneof_Map::battlenet_map_name(v))
    }

    // Mutable pointer to the field.
    pub fn mut_battlenet_map_name(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(RequestCreateGame_oneof_Map::battlenet_map_name(_)) = self.Map {
        } else {
            self.Map = ::std::option::Option::Some(RequestCreateGame_oneof_Map::battlenet_map_name(::std::string::String::new()));
        }
        match self.Map {
            ::std::option::Option::Some(RequestCreateGame_oneof_Map::battlenet_map_name(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_battlenet_map_name(&mut self) -> ::std::string::String {
        if self.has_battlenet_map_name() {
            match self.Map.take() {
                ::std::option::Option::Some(RequestCreateGame_oneof_Map::battlenet_map_name(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_battlenet_map_name(&self) -> &str {
        match self.Map {
            ::std::option::Option::Some(RequestCreateGame_oneof_Map::battlenet_map_name(ref v)) => v,
            _ => "",
        }
    }

    // repeated .SC2APIProtocol.PlayerSetup player_setup = 3;

    pub fn clear_player_setup(&mut self) {
        self.player_setup.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_setup(&mut self, v: ::protobuf::RepeatedField<PlayerSetup>) {
        self.player_setup = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_setup(&mut self) -> &mut ::protobuf::RepeatedField<PlayerSetup> {
        &mut self.player_setup
    }

    // Take field
    pub fn take_player_setup(&mut self) -> ::protobuf::RepeatedField<PlayerSetup> {
        ::std::mem::replace(&mut self.player_setup, ::protobuf::RepeatedField::new())
    }

    pub fn get_player_setup(&self) -> &[PlayerSetup] {
        &self.player_setup
    }

    fn get_player_setup_for_reflect(&self) -> &::protobuf::RepeatedField<PlayerSetup> {
        &self.player_setup
    }

    fn mut_player_setup_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PlayerSetup> {
        &mut self.player_setup
    }

    // optional bool disable_fog = 4;

    pub fn clear_disable_fog(&mut self) {
        self.disable_fog = ::std::option::Option::None;
    }

    pub fn has_disable_fog(&self) -> bool {
        self.disable_fog.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disable_fog(&mut self, v: bool) {
        self.disable_fog = ::std::option::Option::Some(v);
    }

    pub fn get_disable_fog(&self) -> bool {
        self.disable_fog.unwrap_or(false)
    }

    fn get_disable_fog_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.disable_fog
    }

    fn mut_disable_fog_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.disable_fog
    }

    // optional uint32 random_seed = 5;

    pub fn clear_random_seed(&mut self) {
        self.random_seed = ::std::option::Option::None;
    }

    pub fn has_random_seed(&self) -> bool {
        self.random_seed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_random_seed(&mut self, v: u32) {
        self.random_seed = ::std::option::Option::Some(v);
    }

    pub fn get_random_seed(&self) -> u32 {
        self.random_seed.unwrap_or(0)
    }

    fn get_random_seed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.random_seed
    }

    fn mut_random_seed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.random_seed
    }

    // optional bool realtime = 6;

    pub fn clear_realtime(&mut self) {
        self.realtime = ::std::option::Option::None;
    }

    pub fn has_realtime(&self) -> bool {
        self.realtime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_realtime(&mut self, v: bool) {
        self.realtime = ::std::option::Option::Some(v);
    }

    pub fn get_realtime(&self) -> bool {
        self.realtime.unwrap_or(false)
    }

    fn get_realtime_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.realtime
    }

    fn mut_realtime_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.realtime
    }
}

impl ::protobuf::Message for RequestCreateGame {
    fn is_initialized(&self) -> bool {
        if let Some(RequestCreateGame_oneof_Map::local_map(ref v)) = self.Map {
            if !v.is_initialized() {
                return false;
            }
        }
        for v in &self.player_setup {
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
                    self.Map = ::std::option::Option::Some(RequestCreateGame_oneof_Map::local_map(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.Map = ::std::option::Option::Some(RequestCreateGame_oneof_Map::battlenet_map_name(is.read_string()?));
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.player_setup)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.disable_fog = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.random_seed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.realtime = ::std::option::Option::Some(tmp);
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
        for value in &self.player_setup {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.disable_fog {
            my_size += 2;
        }
        if let Some(v) = self.random_seed {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.realtime {
            my_size += 2;
        }
        if let ::std::option::Option::Some(ref v) = self.Map {
            match v {
                &RequestCreateGame_oneof_Map::local_map(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &RequestCreateGame_oneof_Map::battlenet_map_name(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.player_setup {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.disable_fog {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.random_seed {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.realtime {
            os.write_bool(6, v)?;
        }
        if let ::std::option::Option::Some(ref v) = self.Map {
            match v {
                &RequestCreateGame_oneof_Map::local_map(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &RequestCreateGame_oneof_Map::battlenet_map_name(ref v) => {
                    os.write_string(2, v)?;
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

impl ::protobuf::MessageStatic for RequestCreateGame {
    fn new() -> RequestCreateGame {
        RequestCreateGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestCreateGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, LocalMap>(
                    "local_map",
                    RequestCreateGame::has_local_map,
                    RequestCreateGame::get_local_map,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "battlenet_map_name",
                    RequestCreateGame::has_battlenet_map_name,
                    RequestCreateGame::get_battlenet_map_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlayerSetup>>(
                    "player_setup",
                    RequestCreateGame::get_player_setup_for_reflect,
                    RequestCreateGame::mut_player_setup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "disable_fog",
                    RequestCreateGame::get_disable_fog_for_reflect,
                    RequestCreateGame::mut_disable_fog_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "random_seed",
                    RequestCreateGame::get_random_seed_for_reflect,
                    RequestCreateGame::mut_random_seed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "realtime",
                    RequestCreateGame::get_realtime_for_reflect,
                    RequestCreateGame::mut_realtime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestCreateGame>(
                    "RequestCreateGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestCreateGame {
    fn clear(&mut self) {
        self.clear_local_map();
        self.clear_battlenet_map_name();
        self.clear_player_setup();
        self.clear_disable_fog();
        self.clear_random_seed();
        self.clear_realtime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestCreateGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestCreateGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct LocalMap {
    // message fields
    map_path: ::protobuf::SingularField<::std::string::String>,
    map_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LocalMap {}

impl LocalMap {
    pub fn new() -> LocalMap {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocalMap {
        static mut instance: ::protobuf::lazy::Lazy<LocalMap> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocalMap,
        };
        unsafe {
            instance.get(LocalMap::new)
        }
    }

    // optional string map_path = 1;

    pub fn clear_map_path(&mut self) {
        self.map_path.clear();
    }

    pub fn has_map_path(&self) -> bool {
        self.map_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_path(&mut self, v: ::std::string::String) {
        self.map_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_path(&mut self) -> &mut ::std::string::String {
        if self.map_path.is_none() {
            self.map_path.set_default();
        }
        self.map_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_path(&mut self) -> ::std::string::String {
        self.map_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map_path(&self) -> &str {
        match self.map_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map_path
    }

    fn mut_map_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map_path
    }

    // optional bytes map_data = 7;

    pub fn clear_map_data(&mut self) {
        self.map_data.clear();
    }

    pub fn has_map_data(&self) -> bool {
        self.map_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.map_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.map_data.is_none() {
            self.map_data.set_default();
        }
        self.map_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_data(&mut self) -> ::std::vec::Vec<u8> {
        self.map_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_map_data(&self) -> &[u8] {
        match self.map_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_map_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.map_data
    }

    fn mut_map_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.map_data
    }
}

impl ::protobuf::Message for LocalMap {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_path)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.map_data)?;
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
        if let Some(ref v) = self.map_path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.map_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.map_path.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.map_data.as_ref() {
            os.write_bytes(7, &v)?;
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

impl ::protobuf::MessageStatic for LocalMap {
    fn new() -> LocalMap {
        LocalMap::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocalMap>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_path",
                    LocalMap::get_map_path_for_reflect,
                    LocalMap::mut_map_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "map_data",
                    LocalMap::get_map_data_for_reflect,
                    LocalMap::mut_map_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocalMap>(
                    "LocalMap",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocalMap {
    fn clear(&mut self) {
        self.clear_map_path();
        self.clear_map_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocalMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocalMap {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseCreateGame {
    // message fields
    error: ::std::option::Option<ResponseCreateGame_Error>,
    error_details: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseCreateGame {}

impl ResponseCreateGame {
    pub fn new() -> ResponseCreateGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseCreateGame {
        static mut instance: ::protobuf::lazy::Lazy<ResponseCreateGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseCreateGame,
        };
        unsafe {
            instance.get(ResponseCreateGame::new)
        }
    }

    // optional .SC2APIProtocol.ResponseCreateGame.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ResponseCreateGame_Error) {
        self.error = ::std::option::Option::Some(v);
    }

    pub fn get_error(&self) -> ResponseCreateGame_Error {
        self.error.unwrap_or(ResponseCreateGame_Error::MissingMap)
    }

    fn get_error_for_reflect(&self) -> &::std::option::Option<ResponseCreateGame_Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::option::Option<ResponseCreateGame_Error> {
        &mut self.error
    }

    // optional string error_details = 2;

    pub fn clear_error_details(&mut self) {
        self.error_details.clear();
    }

    pub fn has_error_details(&self) -> bool {
        self.error_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_details(&mut self, v: ::std::string::String) {
        self.error_details = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_details(&mut self) -> &mut ::std::string::String {
        if self.error_details.is_none() {
            self.error_details.set_default();
        }
        self.error_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_details(&mut self) -> ::std::string::String {
        self.error_details.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_details(&self) -> &str {
        match self.error_details.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_details_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_details
    }

    fn mut_error_details_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_details
    }
}

impl ::protobuf::Message for ResponseCreateGame {
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
                    self.error = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_details)?;
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
        if let Some(v) = self.error {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.error_details.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.error_details.as_ref() {
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

impl ::protobuf::MessageStatic for ResponseCreateGame {
    fn new() -> ResponseCreateGame {
        ResponseCreateGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseCreateGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ResponseCreateGame_Error>>(
                    "error",
                    ResponseCreateGame::get_error_for_reflect,
                    ResponseCreateGame::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_details",
                    ResponseCreateGame::get_error_details_for_reflect,
                    ResponseCreateGame::mut_error_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseCreateGame>(
                    "ResponseCreateGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseCreateGame {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_error_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseCreateGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseCreateGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ResponseCreateGame_Error {
    MissingMap = 1,
    InvalidMapPath = 2,
    InvalidMapData = 3,
    InvalidMapName = 4,
    InvalidMapHandle = 5,
    MissingPlayerSetup = 6,
    InvalidPlayerSetup = 7,
    MultiplayerUnsupported = 8,
}

impl ::protobuf::ProtobufEnum for ResponseCreateGame_Error {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ResponseCreateGame_Error> {
        match value {
            1 => ::std::option::Option::Some(ResponseCreateGame_Error::MissingMap),
            2 => ::std::option::Option::Some(ResponseCreateGame_Error::InvalidMapPath),
            3 => ::std::option::Option::Some(ResponseCreateGame_Error::InvalidMapData),
            4 => ::std::option::Option::Some(ResponseCreateGame_Error::InvalidMapName),
            5 => ::std::option::Option::Some(ResponseCreateGame_Error::InvalidMapHandle),
            6 => ::std::option::Option::Some(ResponseCreateGame_Error::MissingPlayerSetup),
            7 => ::std::option::Option::Some(ResponseCreateGame_Error::InvalidPlayerSetup),
            8 => ::std::option::Option::Some(ResponseCreateGame_Error::MultiplayerUnsupported),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ResponseCreateGame_Error] = &[
            ResponseCreateGame_Error::MissingMap,
            ResponseCreateGame_Error::InvalidMapPath,
            ResponseCreateGame_Error::InvalidMapData,
            ResponseCreateGame_Error::InvalidMapName,
            ResponseCreateGame_Error::InvalidMapHandle,
            ResponseCreateGame_Error::MissingPlayerSetup,
            ResponseCreateGame_Error::InvalidPlayerSetup,
            ResponseCreateGame_Error::MultiplayerUnsupported,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ResponseCreateGame_Error>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ResponseCreateGame_Error", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ResponseCreateGame_Error {
}

impl ::protobuf::reflect::ProtobufValue for ResponseCreateGame_Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestJoinGame {
    // message fields
    options: ::protobuf::SingularPtrField<InterfaceOptions>,
    server_ports: ::protobuf::SingularPtrField<PortSet>,
    client_ports: ::protobuf::RepeatedField<PortSet>,
    shared_port: ::std::option::Option<i32>,
    // message oneof groups
    participation: ::std::option::Option<RequestJoinGame_oneof_participation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestJoinGame {}

#[derive(Clone,PartialEq)]
pub enum RequestJoinGame_oneof_participation {
    race(super::common::Race),
    observed_player_id(u32),
}

impl RequestJoinGame {
    pub fn new() -> RequestJoinGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestJoinGame {
        static mut instance: ::protobuf::lazy::Lazy<RequestJoinGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestJoinGame,
        };
        unsafe {
            instance.get(RequestJoinGame::new)
        }
    }

    // optional .SC2APIProtocol.Race race = 1;

    pub fn clear_race(&mut self) {
        self.participation = ::std::option::Option::None;
    }

    pub fn has_race(&self) -> bool {
        match self.participation {
            ::std::option::Option::Some(RequestJoinGame_oneof_participation::race(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_race(&mut self, v: super::common::Race) {
        self.participation = ::std::option::Option::Some(RequestJoinGame_oneof_participation::race(v))
    }

    pub fn get_race(&self) -> super::common::Race {
        match self.participation {
            ::std::option::Option::Some(RequestJoinGame_oneof_participation::race(v)) => v,
            _ => super::common::Race::NoRace,
        }
    }

    // optional uint32 observed_player_id = 2;

    pub fn clear_observed_player_id(&mut self) {
        self.participation = ::std::option::Option::None;
    }

    pub fn has_observed_player_id(&self) -> bool {
        match self.participation {
            ::std::option::Option::Some(RequestJoinGame_oneof_participation::observed_player_id(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_observed_player_id(&mut self, v: u32) {
        self.participation = ::std::option::Option::Some(RequestJoinGame_oneof_participation::observed_player_id(v))
    }

    pub fn get_observed_player_id(&self) -> u32 {
        match self.participation {
            ::std::option::Option::Some(RequestJoinGame_oneof_participation::observed_player_id(v)) => v,
            _ => 0,
        }
    }

    // optional .SC2APIProtocol.InterfaceOptions options = 3;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: InterfaceOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut InterfaceOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> InterfaceOptions {
        self.options.take().unwrap_or_else(|| InterfaceOptions::new())
    }

    pub fn get_options(&self) -> &InterfaceOptions {
        self.options.as_ref().unwrap_or_else(|| InterfaceOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<InterfaceOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<InterfaceOptions> {
        &mut self.options
    }

    // optional .SC2APIProtocol.PortSet server_ports = 4;

    pub fn clear_server_ports(&mut self) {
        self.server_ports.clear();
    }

    pub fn has_server_ports(&self) -> bool {
        self.server_ports.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_ports(&mut self, v: PortSet) {
        self.server_ports = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_ports(&mut self) -> &mut PortSet {
        if self.server_ports.is_none() {
            self.server_ports.set_default();
        }
        self.server_ports.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_ports(&mut self) -> PortSet {
        self.server_ports.take().unwrap_or_else(|| PortSet::new())
    }

    pub fn get_server_ports(&self) -> &PortSet {
        self.server_ports.as_ref().unwrap_or_else(|| PortSet::default_instance())
    }

    fn get_server_ports_for_reflect(&self) -> &::protobuf::SingularPtrField<PortSet> {
        &self.server_ports
    }

    fn mut_server_ports_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PortSet> {
        &mut self.server_ports
    }

    // repeated .SC2APIProtocol.PortSet client_ports = 5;

    pub fn clear_client_ports(&mut self) {
        self.client_ports.clear();
    }

    // Param is passed by value, moved
    pub fn set_client_ports(&mut self, v: ::protobuf::RepeatedField<PortSet>) {
        self.client_ports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_client_ports(&mut self) -> &mut ::protobuf::RepeatedField<PortSet> {
        &mut self.client_ports
    }

    // Take field
    pub fn take_client_ports(&mut self) -> ::protobuf::RepeatedField<PortSet> {
        ::std::mem::replace(&mut self.client_ports, ::protobuf::RepeatedField::new())
    }

    pub fn get_client_ports(&self) -> &[PortSet] {
        &self.client_ports
    }

    fn get_client_ports_for_reflect(&self) -> &::protobuf::RepeatedField<PortSet> {
        &self.client_ports
    }

    fn mut_client_ports_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PortSet> {
        &mut self.client_ports
    }

    // optional int32 shared_port = 6;

    pub fn clear_shared_port(&mut self) {
        self.shared_port = ::std::option::Option::None;
    }

    pub fn has_shared_port(&self) -> bool {
        self.shared_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shared_port(&mut self, v: i32) {
        self.shared_port = ::std::option::Option::Some(v);
    }

    pub fn get_shared_port(&self) -> i32 {
        self.shared_port.unwrap_or(0)
    }

    fn get_shared_port_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.shared_port
    }

    fn mut_shared_port_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.shared_port
    }
}

impl ::protobuf::Message for RequestJoinGame {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.server_ports {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.client_ports {
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
                    self.participation = ::std::option::Option::Some(RequestJoinGame_oneof_participation::race(is.read_enum()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.participation = ::std::option::Option::Some(RequestJoinGame_oneof_participation::observed_player_id(is.read_uint32()?));
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.server_ports)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.client_ports)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.shared_port = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.server_ports.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.client_ports {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.shared_port {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let ::std::option::Option::Some(ref v) = self.participation {
            match v {
                &RequestJoinGame_oneof_participation::race(v) => {
                    my_size += ::protobuf::rt::enum_size(1, v);
                },
                &RequestJoinGame_oneof_participation::observed_player_id(v) => {
                    my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.server_ports.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.client_ports {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.shared_port {
            os.write_int32(6, v)?;
        }
        if let ::std::option::Option::Some(ref v) = self.participation {
            match v {
                &RequestJoinGame_oneof_participation::race(v) => {
                    os.write_enum(1, v.value())?;
                },
                &RequestJoinGame_oneof_participation::observed_player_id(v) => {
                    os.write_uint32(2, v)?;
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

impl ::protobuf::MessageStatic for RequestJoinGame {
    fn new() -> RequestJoinGame {
        RequestJoinGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestJoinGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor::<_, super::common::Race>(
                    "race",
                    RequestJoinGame::has_race,
                    RequestJoinGame::get_race,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor::<_>(
                    "observed_player_id",
                    RequestJoinGame::has_observed_player_id,
                    RequestJoinGame::get_observed_player_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<InterfaceOptions>>(
                    "options",
                    RequestJoinGame::get_options_for_reflect,
                    RequestJoinGame::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PortSet>>(
                    "server_ports",
                    RequestJoinGame::get_server_ports_for_reflect,
                    RequestJoinGame::mut_server_ports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PortSet>>(
                    "client_ports",
                    RequestJoinGame::get_client_ports_for_reflect,
                    RequestJoinGame::mut_client_ports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "shared_port",
                    RequestJoinGame::get_shared_port_for_reflect,
                    RequestJoinGame::mut_shared_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestJoinGame>(
                    "RequestJoinGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestJoinGame {
    fn clear(&mut self) {
        self.clear_race();
        self.clear_observed_player_id();
        self.clear_options();
        self.clear_server_ports();
        self.clear_client_ports();
        self.clear_shared_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestJoinGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestJoinGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PortSet {
    // message fields
    game_port: ::std::option::Option<i32>,
    base_port: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PortSet {}

impl PortSet {
    pub fn new() -> PortSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PortSet {
        static mut instance: ::protobuf::lazy::Lazy<PortSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PortSet,
        };
        unsafe {
            instance.get(PortSet::new)
        }
    }

    // optional int32 game_port = 1;

    pub fn clear_game_port(&mut self) {
        self.game_port = ::std::option::Option::None;
    }

    pub fn has_game_port(&self) -> bool {
        self.game_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_port(&mut self, v: i32) {
        self.game_port = ::std::option::Option::Some(v);
    }

    pub fn get_game_port(&self) -> i32 {
        self.game_port.unwrap_or(0)
    }

    fn get_game_port_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.game_port
    }

    fn mut_game_port_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.game_port
    }

    // optional int32 base_port = 2;

    pub fn clear_base_port(&mut self) {
        self.base_port = ::std::option::Option::None;
    }

    pub fn has_base_port(&self) -> bool {
        self.base_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_port(&mut self, v: i32) {
        self.base_port = ::std::option::Option::Some(v);
    }

    pub fn get_base_port(&self) -> i32 {
        self.base_port.unwrap_or(0)
    }

    fn get_base_port_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.base_port
    }

    fn mut_base_port_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.base_port
    }
}

impl ::protobuf::Message for PortSet {
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
                    self.game_port = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.base_port = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.game_port {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.base_port {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.game_port {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.base_port {
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

impl ::protobuf::MessageStatic for PortSet {
    fn new() -> PortSet {
        PortSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<PortSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "game_port",
                    PortSet::get_game_port_for_reflect,
                    PortSet::mut_game_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "base_port",
                    PortSet::get_base_port_for_reflect,
                    PortSet::mut_base_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PortSet>(
                    "PortSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PortSet {
    fn clear(&mut self) {
        self.clear_game_port();
        self.clear_base_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PortSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PortSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseJoinGame {
    // message fields
    player_id: ::std::option::Option<u32>,
    error: ::std::option::Option<ResponseJoinGame_Error>,
    error_details: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseJoinGame {}

impl ResponseJoinGame {
    pub fn new() -> ResponseJoinGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseJoinGame {
        static mut instance: ::protobuf::lazy::Lazy<ResponseJoinGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseJoinGame,
        };
        unsafe {
            instance.get(ResponseJoinGame::new)
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_id
    }

    // optional .SC2APIProtocol.ResponseJoinGame.Error error = 2;

    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ResponseJoinGame_Error) {
        self.error = ::std::option::Option::Some(v);
    }

    pub fn get_error(&self) -> ResponseJoinGame_Error {
        self.error.unwrap_or(ResponseJoinGame_Error::MissingParticipation)
    }

    fn get_error_for_reflect(&self) -> &::std::option::Option<ResponseJoinGame_Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::option::Option<ResponseJoinGame_Error> {
        &mut self.error
    }

    // optional string error_details = 3;

    pub fn clear_error_details(&mut self) {
        self.error_details.clear();
    }

    pub fn has_error_details(&self) -> bool {
        self.error_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_details(&mut self, v: ::std::string::String) {
        self.error_details = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_details(&mut self) -> &mut ::std::string::String {
        if self.error_details.is_none() {
            self.error_details.set_default();
        }
        self.error_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_details(&mut self) -> ::std::string::String {
        self.error_details.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_details(&self) -> &str {
        match self.error_details.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_details_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_details
    }

    fn mut_error_details_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_details
    }
}

impl ::protobuf::Message for ResponseJoinGame {
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
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.error = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_details)?;
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
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.error {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(ref v) = self.error_details.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.error {
            os.write_enum(2, v.value())?;
        }
        if let Some(ref v) = self.error_details.as_ref() {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for ResponseJoinGame {
    fn new() -> ResponseJoinGame {
        ResponseJoinGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseJoinGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_id",
                    ResponseJoinGame::get_player_id_for_reflect,
                    ResponseJoinGame::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ResponseJoinGame_Error>>(
                    "error",
                    ResponseJoinGame::get_error_for_reflect,
                    ResponseJoinGame::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_details",
                    ResponseJoinGame::get_error_details_for_reflect,
                    ResponseJoinGame::mut_error_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseJoinGame>(
                    "ResponseJoinGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseJoinGame {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_error();
        self.clear_error_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseJoinGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseJoinGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ResponseJoinGame_Error {
    MissingParticipation = 1,
    InvalidObservedPlayerId = 2,
    MissingOptions = 3,
    MissingPorts = 4,
    GameFull = 5,
    LaunchError = 6,
    FeatureUnsupported = 7,
    NoSpaceForUser = 8,
    MapDoesNotExist = 9,
    CannotOpenMap = 10,
    ChecksumError = 11,
    NetworkError = 12,
    OtherError = 13,
}

impl ::protobuf::ProtobufEnum for ResponseJoinGame_Error {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ResponseJoinGame_Error> {
        match value {
            1 => ::std::option::Option::Some(ResponseJoinGame_Error::MissingParticipation),
            2 => ::std::option::Option::Some(ResponseJoinGame_Error::InvalidObservedPlayerId),
            3 => ::std::option::Option::Some(ResponseJoinGame_Error::MissingOptions),
            4 => ::std::option::Option::Some(ResponseJoinGame_Error::MissingPorts),
            5 => ::std::option::Option::Some(ResponseJoinGame_Error::GameFull),
            6 => ::std::option::Option::Some(ResponseJoinGame_Error::LaunchError),
            7 => ::std::option::Option::Some(ResponseJoinGame_Error::FeatureUnsupported),
            8 => ::std::option::Option::Some(ResponseJoinGame_Error::NoSpaceForUser),
            9 => ::std::option::Option::Some(ResponseJoinGame_Error::MapDoesNotExist),
            10 => ::std::option::Option::Some(ResponseJoinGame_Error::CannotOpenMap),
            11 => ::std::option::Option::Some(ResponseJoinGame_Error::ChecksumError),
            12 => ::std::option::Option::Some(ResponseJoinGame_Error::NetworkError),
            13 => ::std::option::Option::Some(ResponseJoinGame_Error::OtherError),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ResponseJoinGame_Error] = &[
            ResponseJoinGame_Error::MissingParticipation,
            ResponseJoinGame_Error::InvalidObservedPlayerId,
            ResponseJoinGame_Error::MissingOptions,
            ResponseJoinGame_Error::MissingPorts,
            ResponseJoinGame_Error::GameFull,
            ResponseJoinGame_Error::LaunchError,
            ResponseJoinGame_Error::FeatureUnsupported,
            ResponseJoinGame_Error::NoSpaceForUser,
            ResponseJoinGame_Error::MapDoesNotExist,
            ResponseJoinGame_Error::CannotOpenMap,
            ResponseJoinGame_Error::ChecksumError,
            ResponseJoinGame_Error::NetworkError,
            ResponseJoinGame_Error::OtherError,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ResponseJoinGame_Error>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ResponseJoinGame_Error", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ResponseJoinGame_Error {
}

impl ::protobuf::reflect::ProtobufValue for ResponseJoinGame_Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestRestartGame {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestRestartGame {}

impl RequestRestartGame {
    pub fn new() -> RequestRestartGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestRestartGame {
        static mut instance: ::protobuf::lazy::Lazy<RequestRestartGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestRestartGame,
        };
        unsafe {
            instance.get(RequestRestartGame::new)
        }
    }
}

impl ::protobuf::Message for RequestRestartGame {
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

impl ::protobuf::MessageStatic for RequestRestartGame {
    fn new() -> RequestRestartGame {
        RequestRestartGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestRestartGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestRestartGame>(
                    "RequestRestartGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestRestartGame {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestRestartGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestRestartGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseRestartGame {
    // message fields
    error: ::std::option::Option<ResponseRestartGame_Error>,
    error_details: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseRestartGame {}

impl ResponseRestartGame {
    pub fn new() -> ResponseRestartGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseRestartGame {
        static mut instance: ::protobuf::lazy::Lazy<ResponseRestartGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseRestartGame,
        };
        unsafe {
            instance.get(ResponseRestartGame::new)
        }
    }

    // optional .SC2APIProtocol.ResponseRestartGame.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ResponseRestartGame_Error) {
        self.error = ::std::option::Option::Some(v);
    }

    pub fn get_error(&self) -> ResponseRestartGame_Error {
        self.error.unwrap_or(ResponseRestartGame_Error::LaunchError)
    }

    fn get_error_for_reflect(&self) -> &::std::option::Option<ResponseRestartGame_Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::option::Option<ResponseRestartGame_Error> {
        &mut self.error
    }

    // optional string error_details = 2;

    pub fn clear_error_details(&mut self) {
        self.error_details.clear();
    }

    pub fn has_error_details(&self) -> bool {
        self.error_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_details(&mut self, v: ::std::string::String) {
        self.error_details = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_details(&mut self) -> &mut ::std::string::String {
        if self.error_details.is_none() {
            self.error_details.set_default();
        }
        self.error_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_details(&mut self) -> ::std::string::String {
        self.error_details.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_details(&self) -> &str {
        match self.error_details.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_details_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_details
    }

    fn mut_error_details_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_details
    }
}

impl ::protobuf::Message for ResponseRestartGame {
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
                    self.error = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_details)?;
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
        if let Some(v) = self.error {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.error_details.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.error_details.as_ref() {
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

impl ::protobuf::MessageStatic for ResponseRestartGame {
    fn new() -> ResponseRestartGame {
        ResponseRestartGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseRestartGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ResponseRestartGame_Error>>(
                    "error",
                    ResponseRestartGame::get_error_for_reflect,
                    ResponseRestartGame::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_details",
                    ResponseRestartGame::get_error_details_for_reflect,
                    ResponseRestartGame::mut_error_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseRestartGame>(
                    "ResponseRestartGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseRestartGame {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_error_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseRestartGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseRestartGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ResponseRestartGame_Error {
    LaunchError = 1,
}

impl ::protobuf::ProtobufEnum for ResponseRestartGame_Error {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ResponseRestartGame_Error> {
        match value {
            1 => ::std::option::Option::Some(ResponseRestartGame_Error::LaunchError),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ResponseRestartGame_Error] = &[
            ResponseRestartGame_Error::LaunchError,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ResponseRestartGame_Error>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ResponseRestartGame_Error", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ResponseRestartGame_Error {
}

impl ::protobuf::reflect::ProtobufValue for ResponseRestartGame_Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestStartReplay {
    // message fields
    map_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    observed_player_id: ::std::option::Option<i32>,
    options: ::protobuf::SingularPtrField<InterfaceOptions>,
    disable_fog: ::std::option::Option<bool>,
    // message oneof groups
    replay: ::std::option::Option<RequestStartReplay_oneof_replay>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestStartReplay {}

#[derive(Clone,PartialEq)]
pub enum RequestStartReplay_oneof_replay {
    replay_path(::std::string::String),
    replay_data(::std::vec::Vec<u8>),
}

impl RequestStartReplay {
    pub fn new() -> RequestStartReplay {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestStartReplay {
        static mut instance: ::protobuf::lazy::Lazy<RequestStartReplay> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestStartReplay,
        };
        unsafe {
            instance.get(RequestStartReplay::new)
        }
    }

    // optional string replay_path = 1;

    pub fn clear_replay_path(&mut self) {
        self.replay = ::std::option::Option::None;
    }

    pub fn has_replay_path(&self) -> bool {
        match self.replay {
            ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_path(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_replay_path(&mut self, v: ::std::string::String) {
        self.replay = ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_path(v))
    }

    // Mutable pointer to the field.
    pub fn mut_replay_path(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_path(_)) = self.replay {
        } else {
            self.replay = ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_path(::std::string::String::new()));
        }
        match self.replay {
            ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_path(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_replay_path(&mut self) -> ::std::string::String {
        if self.has_replay_path() {
            match self.replay.take() {
                ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_path(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_replay_path(&self) -> &str {
        match self.replay {
            ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_path(ref v)) => v,
            _ => "",
        }
    }

    // optional bytes replay_data = 5;

    pub fn clear_replay_data(&mut self) {
        self.replay = ::std::option::Option::None;
    }

    pub fn has_replay_data(&self) -> bool {
        match self.replay {
            ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_data(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_replay_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.replay = ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_data(v))
    }

    // Mutable pointer to the field.
    pub fn mut_replay_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_data(_)) = self.replay {
        } else {
            self.replay = ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_data(::std::vec::Vec::new()));
        }
        match self.replay {
            ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_data(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_replay_data(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_replay_data() {
            match self.replay.take() {
                ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_data(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_replay_data(&self) -> &[u8] {
        match self.replay {
            ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_data(ref v)) => v,
            _ => &[],
        }
    }

    // optional bytes map_data = 6;

    pub fn clear_map_data(&mut self) {
        self.map_data.clear();
    }

    pub fn has_map_data(&self) -> bool {
        self.map_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.map_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.map_data.is_none() {
            self.map_data.set_default();
        }
        self.map_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_data(&mut self) -> ::std::vec::Vec<u8> {
        self.map_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_map_data(&self) -> &[u8] {
        match self.map_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_map_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.map_data
    }

    fn mut_map_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.map_data
    }

    // optional int32 observed_player_id = 2;

    pub fn clear_observed_player_id(&mut self) {
        self.observed_player_id = ::std::option::Option::None;
    }

    pub fn has_observed_player_id(&self) -> bool {
        self.observed_player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_observed_player_id(&mut self, v: i32) {
        self.observed_player_id = ::std::option::Option::Some(v);
    }

    pub fn get_observed_player_id(&self) -> i32 {
        self.observed_player_id.unwrap_or(0)
    }

    fn get_observed_player_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.observed_player_id
    }

    fn mut_observed_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.observed_player_id
    }

    // optional .SC2APIProtocol.InterfaceOptions options = 3;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: InterfaceOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut InterfaceOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> InterfaceOptions {
        self.options.take().unwrap_or_else(|| InterfaceOptions::new())
    }

    pub fn get_options(&self) -> &InterfaceOptions {
        self.options.as_ref().unwrap_or_else(|| InterfaceOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<InterfaceOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<InterfaceOptions> {
        &mut self.options
    }

    // optional bool disable_fog = 4;

    pub fn clear_disable_fog(&mut self) {
        self.disable_fog = ::std::option::Option::None;
    }

    pub fn has_disable_fog(&self) -> bool {
        self.disable_fog.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disable_fog(&mut self, v: bool) {
        self.disable_fog = ::std::option::Option::Some(v);
    }

    pub fn get_disable_fog(&self) -> bool {
        self.disable_fog.unwrap_or(false)
    }

    fn get_disable_fog_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.disable_fog
    }

    fn mut_disable_fog_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.disable_fog
    }
}

impl ::protobuf::Message for RequestStartReplay {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
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
                    self.replay = ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_path(is.read_string()?));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.replay = ::std::option::Option::Some(RequestStartReplay_oneof_replay::replay_data(is.read_bytes()?));
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.map_data)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.observed_player_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.disable_fog = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.map_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        }
        if let Some(v) = self.observed_player_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.disable_fog {
            my_size += 2;
        }
        if let ::std::option::Option::Some(ref v) = self.replay {
            match v {
                &RequestStartReplay_oneof_replay::replay_path(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &RequestStartReplay_oneof_replay::replay_data(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(5, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.map_data.as_ref() {
            os.write_bytes(6, &v)?;
        }
        if let Some(v) = self.observed_player_id {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.disable_fog {
            os.write_bool(4, v)?;
        }
        if let ::std::option::Option::Some(ref v) = self.replay {
            match v {
                &RequestStartReplay_oneof_replay::replay_path(ref v) => {
                    os.write_string(1, v)?;
                },
                &RequestStartReplay_oneof_replay::replay_data(ref v) => {
                    os.write_bytes(5, v)?;
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

impl ::protobuf::MessageStatic for RequestStartReplay {
    fn new() -> RequestStartReplay {
        RequestStartReplay::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestStartReplay>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "replay_path",
                    RequestStartReplay::has_replay_path,
                    RequestStartReplay::get_replay_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "replay_data",
                    RequestStartReplay::has_replay_data,
                    RequestStartReplay::get_replay_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "map_data",
                    RequestStartReplay::get_map_data_for_reflect,
                    RequestStartReplay::mut_map_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "observed_player_id",
                    RequestStartReplay::get_observed_player_id_for_reflect,
                    RequestStartReplay::mut_observed_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<InterfaceOptions>>(
                    "options",
                    RequestStartReplay::get_options_for_reflect,
                    RequestStartReplay::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "disable_fog",
                    RequestStartReplay::get_disable_fog_for_reflect,
                    RequestStartReplay::mut_disable_fog_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestStartReplay>(
                    "RequestStartReplay",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestStartReplay {
    fn clear(&mut self) {
        self.clear_replay_path();
        self.clear_replay_data();
        self.clear_map_data();
        self.clear_observed_player_id();
        self.clear_options();
        self.clear_disable_fog();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestStartReplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestStartReplay {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseStartReplay {
    // message fields
    error: ::std::option::Option<ResponseStartReplay_Error>,
    error_details: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseStartReplay {}

impl ResponseStartReplay {
    pub fn new() -> ResponseStartReplay {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseStartReplay {
        static mut instance: ::protobuf::lazy::Lazy<ResponseStartReplay> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseStartReplay,
        };
        unsafe {
            instance.get(ResponseStartReplay::new)
        }
    }

    // optional .SC2APIProtocol.ResponseStartReplay.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ResponseStartReplay_Error) {
        self.error = ::std::option::Option::Some(v);
    }

    pub fn get_error(&self) -> ResponseStartReplay_Error {
        self.error.unwrap_or(ResponseStartReplay_Error::MissingReplay)
    }

    fn get_error_for_reflect(&self) -> &::std::option::Option<ResponseStartReplay_Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::option::Option<ResponseStartReplay_Error> {
        &mut self.error
    }

    // optional string error_details = 2;

    pub fn clear_error_details(&mut self) {
        self.error_details.clear();
    }

    pub fn has_error_details(&self) -> bool {
        self.error_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_details(&mut self, v: ::std::string::String) {
        self.error_details = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_details(&mut self) -> &mut ::std::string::String {
        if self.error_details.is_none() {
            self.error_details.set_default();
        }
        self.error_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_details(&mut self) -> ::std::string::String {
        self.error_details.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_details(&self) -> &str {
        match self.error_details.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_details_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_details
    }

    fn mut_error_details_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_details
    }
}

impl ::protobuf::Message for ResponseStartReplay {
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
                    self.error = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_details)?;
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
        if let Some(v) = self.error {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.error_details.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.error_details.as_ref() {
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

impl ::protobuf::MessageStatic for ResponseStartReplay {
    fn new() -> ResponseStartReplay {
        ResponseStartReplay::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseStartReplay>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ResponseStartReplay_Error>>(
                    "error",
                    ResponseStartReplay::get_error_for_reflect,
                    ResponseStartReplay::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_details",
                    ResponseStartReplay::get_error_details_for_reflect,
                    ResponseStartReplay::mut_error_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseStartReplay>(
                    "ResponseStartReplay",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseStartReplay {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_error_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseStartReplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseStartReplay {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ResponseStartReplay_Error {
    MissingReplay = 1,
    InvalidReplayPath = 2,
    InvalidReplayData = 3,
    InvalidMapData = 4,
    InvalidObservedPlayerId = 5,
    MissingOptions = 6,
    LaunchError = 7,
}

impl ::protobuf::ProtobufEnum for ResponseStartReplay_Error {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ResponseStartReplay_Error> {
        match value {
            1 => ::std::option::Option::Some(ResponseStartReplay_Error::MissingReplay),
            2 => ::std::option::Option::Some(ResponseStartReplay_Error::InvalidReplayPath),
            3 => ::std::option::Option::Some(ResponseStartReplay_Error::InvalidReplayData),
            4 => ::std::option::Option::Some(ResponseStartReplay_Error::InvalidMapData),
            5 => ::std::option::Option::Some(ResponseStartReplay_Error::InvalidObservedPlayerId),
            6 => ::std::option::Option::Some(ResponseStartReplay_Error::MissingOptions),
            7 => ::std::option::Option::Some(ResponseStartReplay_Error::LaunchError),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ResponseStartReplay_Error] = &[
            ResponseStartReplay_Error::MissingReplay,
            ResponseStartReplay_Error::InvalidReplayPath,
            ResponseStartReplay_Error::InvalidReplayData,
            ResponseStartReplay_Error::InvalidMapData,
            ResponseStartReplay_Error::InvalidObservedPlayerId,
            ResponseStartReplay_Error::MissingOptions,
            ResponseStartReplay_Error::LaunchError,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ResponseStartReplay_Error>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ResponseStartReplay_Error", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ResponseStartReplay_Error {
}

impl ::protobuf::reflect::ProtobufValue for ResponseStartReplay_Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestLeaveGame {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestLeaveGame {}

impl RequestLeaveGame {
    pub fn new() -> RequestLeaveGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestLeaveGame {
        static mut instance: ::protobuf::lazy::Lazy<RequestLeaveGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestLeaveGame,
        };
        unsafe {
            instance.get(RequestLeaveGame::new)
        }
    }
}

impl ::protobuf::Message for RequestLeaveGame {
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

impl ::protobuf::MessageStatic for RequestLeaveGame {
    fn new() -> RequestLeaveGame {
        RequestLeaveGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestLeaveGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestLeaveGame>(
                    "RequestLeaveGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestLeaveGame {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestLeaveGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestLeaveGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseLeaveGame {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseLeaveGame {}

impl ResponseLeaveGame {
    pub fn new() -> ResponseLeaveGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseLeaveGame {
        static mut instance: ::protobuf::lazy::Lazy<ResponseLeaveGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseLeaveGame,
        };
        unsafe {
            instance.get(ResponseLeaveGame::new)
        }
    }
}

impl ::protobuf::Message for ResponseLeaveGame {
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

impl ::protobuf::MessageStatic for ResponseLeaveGame {
    fn new() -> ResponseLeaveGame {
        ResponseLeaveGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseLeaveGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseLeaveGame>(
                    "ResponseLeaveGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseLeaveGame {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseLeaveGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseLeaveGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestQuickSave {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestQuickSave {}

impl RequestQuickSave {
    pub fn new() -> RequestQuickSave {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestQuickSave {
        static mut instance: ::protobuf::lazy::Lazy<RequestQuickSave> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestQuickSave,
        };
        unsafe {
            instance.get(RequestQuickSave::new)
        }
    }
}

impl ::protobuf::Message for RequestQuickSave {
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

impl ::protobuf::MessageStatic for RequestQuickSave {
    fn new() -> RequestQuickSave {
        RequestQuickSave::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestQuickSave>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestQuickSave>(
                    "RequestQuickSave",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestQuickSave {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestQuickSave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestQuickSave {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseQuickSave {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseQuickSave {}

impl ResponseQuickSave {
    pub fn new() -> ResponseQuickSave {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseQuickSave {
        static mut instance: ::protobuf::lazy::Lazy<ResponseQuickSave> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseQuickSave,
        };
        unsafe {
            instance.get(ResponseQuickSave::new)
        }
    }
}

impl ::protobuf::Message for ResponseQuickSave {
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

impl ::protobuf::MessageStatic for ResponseQuickSave {
    fn new() -> ResponseQuickSave {
        ResponseQuickSave::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseQuickSave>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseQuickSave>(
                    "ResponseQuickSave",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseQuickSave {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseQuickSave {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseQuickSave {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestQuickLoad {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestQuickLoad {}

impl RequestQuickLoad {
    pub fn new() -> RequestQuickLoad {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestQuickLoad {
        static mut instance: ::protobuf::lazy::Lazy<RequestQuickLoad> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestQuickLoad,
        };
        unsafe {
            instance.get(RequestQuickLoad::new)
        }
    }
}

impl ::protobuf::Message for RequestQuickLoad {
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

impl ::protobuf::MessageStatic for RequestQuickLoad {
    fn new() -> RequestQuickLoad {
        RequestQuickLoad::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestQuickLoad>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestQuickLoad>(
                    "RequestQuickLoad",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestQuickLoad {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestQuickLoad {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestQuickLoad {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseQuickLoad {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseQuickLoad {}

impl ResponseQuickLoad {
    pub fn new() -> ResponseQuickLoad {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseQuickLoad {
        static mut instance: ::protobuf::lazy::Lazy<ResponseQuickLoad> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseQuickLoad,
        };
        unsafe {
            instance.get(ResponseQuickLoad::new)
        }
    }
}

impl ::protobuf::Message for ResponseQuickLoad {
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

impl ::protobuf::MessageStatic for ResponseQuickLoad {
    fn new() -> ResponseQuickLoad {
        ResponseQuickLoad::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseQuickLoad>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseQuickLoad>(
                    "ResponseQuickLoad",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseQuickLoad {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseQuickLoad {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseQuickLoad {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestQuit {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestQuit {}

impl RequestQuit {
    pub fn new() -> RequestQuit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestQuit {
        static mut instance: ::protobuf::lazy::Lazy<RequestQuit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestQuit,
        };
        unsafe {
            instance.get(RequestQuit::new)
        }
    }
}

impl ::protobuf::Message for RequestQuit {
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

impl ::protobuf::MessageStatic for RequestQuit {
    fn new() -> RequestQuit {
        RequestQuit::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestQuit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestQuit>(
                    "RequestQuit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestQuit {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestQuit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestQuit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseQuit {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseQuit {}

impl ResponseQuit {
    pub fn new() -> ResponseQuit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseQuit {
        static mut instance: ::protobuf::lazy::Lazy<ResponseQuit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseQuit,
        };
        unsafe {
            instance.get(ResponseQuit::new)
        }
    }
}

impl ::protobuf::Message for ResponseQuit {
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

impl ::protobuf::MessageStatic for ResponseQuit {
    fn new() -> ResponseQuit {
        ResponseQuit::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseQuit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseQuit>(
                    "ResponseQuit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseQuit {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseQuit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseQuit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestGameInfo {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestGameInfo {}

impl RequestGameInfo {
    pub fn new() -> RequestGameInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestGameInfo {
        static mut instance: ::protobuf::lazy::Lazy<RequestGameInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestGameInfo,
        };
        unsafe {
            instance.get(RequestGameInfo::new)
        }
    }
}

impl ::protobuf::Message for RequestGameInfo {
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

impl ::protobuf::MessageStatic for RequestGameInfo {
    fn new() -> RequestGameInfo {
        RequestGameInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestGameInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestGameInfo>(
                    "RequestGameInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestGameInfo {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestGameInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestGameInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseGameInfo {
    // message fields
    map_name: ::protobuf::SingularField<::std::string::String>,
    mod_names: ::protobuf::RepeatedField<::std::string::String>,
    local_map_path: ::protobuf::SingularField<::std::string::String>,
    player_info: ::protobuf::RepeatedField<PlayerInfo>,
    start_raw: ::protobuf::SingularPtrField<super::raw::StartRaw>,
    options: ::protobuf::SingularPtrField<InterfaceOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseGameInfo {}

impl ResponseGameInfo {
    pub fn new() -> ResponseGameInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseGameInfo {
        static mut instance: ::protobuf::lazy::Lazy<ResponseGameInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseGameInfo,
        };
        unsafe {
            instance.get(ResponseGameInfo::new)
        }
    }

    // optional string map_name = 1;

    pub fn clear_map_name(&mut self) {
        self.map_name.clear();
    }

    pub fn has_map_name(&self) -> bool {
        self.map_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_name(&mut self, v: ::std::string::String) {
        self.map_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_name(&mut self) -> &mut ::std::string::String {
        if self.map_name.is_none() {
            self.map_name.set_default();
        }
        self.map_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_name(&mut self) -> ::std::string::String {
        self.map_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map_name(&self) -> &str {
        match self.map_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map_name
    }

    fn mut_map_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map_name
    }

    // repeated string mod_names = 6;

    pub fn clear_mod_names(&mut self) {
        self.mod_names.clear();
    }

    // Param is passed by value, moved
    pub fn set_mod_names(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.mod_names = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mod_names(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.mod_names
    }

    // Take field
    pub fn take_mod_names(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.mod_names, ::protobuf::RepeatedField::new())
    }

    pub fn get_mod_names(&self) -> &[::std::string::String] {
        &self.mod_names
    }

    fn get_mod_names_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.mod_names
    }

    fn mut_mod_names_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.mod_names
    }

    // optional string local_map_path = 2;

    pub fn clear_local_map_path(&mut self) {
        self.local_map_path.clear();
    }

    pub fn has_local_map_path(&self) -> bool {
        self.local_map_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_local_map_path(&mut self, v: ::std::string::String) {
        self.local_map_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_local_map_path(&mut self) -> &mut ::std::string::String {
        if self.local_map_path.is_none() {
            self.local_map_path.set_default();
        }
        self.local_map_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_local_map_path(&mut self) -> ::std::string::String {
        self.local_map_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_local_map_path(&self) -> &str {
        match self.local_map_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_local_map_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.local_map_path
    }

    fn mut_local_map_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.local_map_path
    }

    // repeated .SC2APIProtocol.PlayerInfo player_info = 3;

    pub fn clear_player_info(&mut self) {
        self.player_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_info(&mut self, v: ::protobuf::RepeatedField<PlayerInfo>) {
        self.player_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_info(&mut self) -> &mut ::protobuf::RepeatedField<PlayerInfo> {
        &mut self.player_info
    }

    // Take field
    pub fn take_player_info(&mut self) -> ::protobuf::RepeatedField<PlayerInfo> {
        ::std::mem::replace(&mut self.player_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_player_info(&self) -> &[PlayerInfo] {
        &self.player_info
    }

    fn get_player_info_for_reflect(&self) -> &::protobuf::RepeatedField<PlayerInfo> {
        &self.player_info
    }

    fn mut_player_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PlayerInfo> {
        &mut self.player_info
    }

    // optional .SC2APIProtocol.StartRaw start_raw = 4;

    pub fn clear_start_raw(&mut self) {
        self.start_raw.clear();
    }

    pub fn has_start_raw(&self) -> bool {
        self.start_raw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_raw(&mut self, v: super::raw::StartRaw) {
        self.start_raw = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_raw(&mut self) -> &mut super::raw::StartRaw {
        if self.start_raw.is_none() {
            self.start_raw.set_default();
        }
        self.start_raw.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_raw(&mut self) -> super::raw::StartRaw {
        self.start_raw.take().unwrap_or_else(|| super::raw::StartRaw::new())
    }

    pub fn get_start_raw(&self) -> &super::raw::StartRaw {
        self.start_raw.as_ref().unwrap_or_else(|| super::raw::StartRaw::default_instance())
    }

    fn get_start_raw_for_reflect(&self) -> &::protobuf::SingularPtrField<super::raw::StartRaw> {
        &self.start_raw
    }

    fn mut_start_raw_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::raw::StartRaw> {
        &mut self.start_raw
    }

    // optional .SC2APIProtocol.InterfaceOptions options = 5;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: InterfaceOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut InterfaceOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> InterfaceOptions {
        self.options.take().unwrap_or_else(|| InterfaceOptions::new())
    }

    pub fn get_options(&self) -> &InterfaceOptions {
        self.options.as_ref().unwrap_or_else(|| InterfaceOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<InterfaceOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<InterfaceOptions> {
        &mut self.options
    }
}

impl ::protobuf::Message for ResponseGameInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.player_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.start_raw {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.options {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.mod_names)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.local_map_path)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.player_info)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start_raw)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
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
        if let Some(ref v) = self.map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.mod_names {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        if let Some(ref v) = self.local_map_path.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.player_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.start_raw.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.map_name.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.mod_names {
            os.write_string(6, &v)?;
        };
        if let Some(ref v) = self.local_map_path.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.player_info {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.start_raw.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ResponseGameInfo {
    fn new() -> ResponseGameInfo {
        ResponseGameInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseGameInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_name",
                    ResponseGameInfo::get_map_name_for_reflect,
                    ResponseGameInfo::mut_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mod_names",
                    ResponseGameInfo::get_mod_names_for_reflect,
                    ResponseGameInfo::mut_mod_names_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "local_map_path",
                    ResponseGameInfo::get_local_map_path_for_reflect,
                    ResponseGameInfo::mut_local_map_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlayerInfo>>(
                    "player_info",
                    ResponseGameInfo::get_player_info_for_reflect,
                    ResponseGameInfo::mut_player_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::raw::StartRaw>>(
                    "start_raw",
                    ResponseGameInfo::get_start_raw_for_reflect,
                    ResponseGameInfo::mut_start_raw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<InterfaceOptions>>(
                    "options",
                    ResponseGameInfo::get_options_for_reflect,
                    ResponseGameInfo::mut_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseGameInfo>(
                    "ResponseGameInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseGameInfo {
    fn clear(&mut self) {
        self.clear_map_name();
        self.clear_mod_names();
        self.clear_local_map_path();
        self.clear_player_info();
        self.clear_start_raw();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseGameInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseGameInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestObservation {
    // message fields
    disable_fog: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestObservation {}

impl RequestObservation {
    pub fn new() -> RequestObservation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestObservation {
        static mut instance: ::protobuf::lazy::Lazy<RequestObservation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestObservation,
        };
        unsafe {
            instance.get(RequestObservation::new)
        }
    }

    // optional bool disable_fog = 1;

    pub fn clear_disable_fog(&mut self) {
        self.disable_fog = ::std::option::Option::None;
    }

    pub fn has_disable_fog(&self) -> bool {
        self.disable_fog.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disable_fog(&mut self, v: bool) {
        self.disable_fog = ::std::option::Option::Some(v);
    }

    pub fn get_disable_fog(&self) -> bool {
        self.disable_fog.unwrap_or(false)
    }

    fn get_disable_fog_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.disable_fog
    }

    fn mut_disable_fog_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.disable_fog
    }
}

impl ::protobuf::Message for RequestObservation {
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
                    self.disable_fog = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.disable_fog {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.disable_fog {
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

impl ::protobuf::MessageStatic for RequestObservation {
    fn new() -> RequestObservation {
        RequestObservation::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestObservation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "disable_fog",
                    RequestObservation::get_disable_fog_for_reflect,
                    RequestObservation::mut_disable_fog_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestObservation>(
                    "RequestObservation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestObservation {
    fn clear(&mut self) {
        self.clear_disable_fog();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestObservation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestObservation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseObservation {
    // message fields
    actions: ::protobuf::RepeatedField<Action>,
    action_errors: ::protobuf::RepeatedField<ActionError>,
    observation: ::protobuf::SingularPtrField<Observation>,
    player_result: ::protobuf::RepeatedField<PlayerResult>,
    chat: ::protobuf::RepeatedField<ChatReceived>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseObservation {}

impl ResponseObservation {
    pub fn new() -> ResponseObservation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseObservation {
        static mut instance: ::protobuf::lazy::Lazy<ResponseObservation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseObservation,
        };
        unsafe {
            instance.get(ResponseObservation::new)
        }
    }

    // repeated .SC2APIProtocol.Action actions = 1;

    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }

    // Param is passed by value, moved
    pub fn set_actions(&mut self, v: ::protobuf::RepeatedField<Action>) {
        self.actions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_actions(&mut self) -> &mut ::protobuf::RepeatedField<Action> {
        &mut self.actions
    }

    // Take field
    pub fn take_actions(&mut self) -> ::protobuf::RepeatedField<Action> {
        ::std::mem::replace(&mut self.actions, ::protobuf::RepeatedField::new())
    }

    pub fn get_actions(&self) -> &[Action] {
        &self.actions
    }

    fn get_actions_for_reflect(&self) -> &::protobuf::RepeatedField<Action> {
        &self.actions
    }

    fn mut_actions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Action> {
        &mut self.actions
    }

    // repeated .SC2APIProtocol.ActionError action_errors = 2;

    pub fn clear_action_errors(&mut self) {
        self.action_errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_action_errors(&mut self, v: ::protobuf::RepeatedField<ActionError>) {
        self.action_errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_action_errors(&mut self) -> &mut ::protobuf::RepeatedField<ActionError> {
        &mut self.action_errors
    }

    // Take field
    pub fn take_action_errors(&mut self) -> ::protobuf::RepeatedField<ActionError> {
        ::std::mem::replace(&mut self.action_errors, ::protobuf::RepeatedField::new())
    }

    pub fn get_action_errors(&self) -> &[ActionError] {
        &self.action_errors
    }

    fn get_action_errors_for_reflect(&self) -> &::protobuf::RepeatedField<ActionError> {
        &self.action_errors
    }

    fn mut_action_errors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ActionError> {
        &mut self.action_errors
    }

    // optional .SC2APIProtocol.Observation observation = 3;

    pub fn clear_observation(&mut self) {
        self.observation.clear();
    }

    pub fn has_observation(&self) -> bool {
        self.observation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_observation(&mut self, v: Observation) {
        self.observation = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_observation(&mut self) -> &mut Observation {
        if self.observation.is_none() {
            self.observation.set_default();
        }
        self.observation.as_mut().unwrap()
    }

    // Take field
    pub fn take_observation(&mut self) -> Observation {
        self.observation.take().unwrap_or_else(|| Observation::new())
    }

    pub fn get_observation(&self) -> &Observation {
        self.observation.as_ref().unwrap_or_else(|| Observation::default_instance())
    }

    fn get_observation_for_reflect(&self) -> &::protobuf::SingularPtrField<Observation> {
        &self.observation
    }

    fn mut_observation_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Observation> {
        &mut self.observation
    }

    // repeated .SC2APIProtocol.PlayerResult player_result = 4;

    pub fn clear_player_result(&mut self) {
        self.player_result.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_result(&mut self, v: ::protobuf::RepeatedField<PlayerResult>) {
        self.player_result = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_result(&mut self) -> &mut ::protobuf::RepeatedField<PlayerResult> {
        &mut self.player_result
    }

    // Take field
    pub fn take_player_result(&mut self) -> ::protobuf::RepeatedField<PlayerResult> {
        ::std::mem::replace(&mut self.player_result, ::protobuf::RepeatedField::new())
    }

    pub fn get_player_result(&self) -> &[PlayerResult] {
        &self.player_result
    }

    fn get_player_result_for_reflect(&self) -> &::protobuf::RepeatedField<PlayerResult> {
        &self.player_result
    }

    fn mut_player_result_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PlayerResult> {
        &mut self.player_result
    }

    // repeated .SC2APIProtocol.ChatReceived chat = 5;

    pub fn clear_chat(&mut self) {
        self.chat.clear();
    }

    // Param is passed by value, moved
    pub fn set_chat(&mut self, v: ::protobuf::RepeatedField<ChatReceived>) {
        self.chat = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chat(&mut self) -> &mut ::protobuf::RepeatedField<ChatReceived> {
        &mut self.chat
    }

    // Take field
    pub fn take_chat(&mut self) -> ::protobuf::RepeatedField<ChatReceived> {
        ::std::mem::replace(&mut self.chat, ::protobuf::RepeatedField::new())
    }

    pub fn get_chat(&self) -> &[ChatReceived] {
        &self.chat
    }

    fn get_chat_for_reflect(&self) -> &::protobuf::RepeatedField<ChatReceived> {
        &self.chat
    }

    fn mut_chat_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ChatReceived> {
        &mut self.chat
    }
}

impl ::protobuf::Message for ResponseObservation {
    fn is_initialized(&self) -> bool {
        for v in &self.actions {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.action_errors {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.observation {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.player_result {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.chat {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.actions)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.action_errors)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.observation)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.player_result)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chat)?;
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
        for value in &self.actions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.action_errors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.observation.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.player_result {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.chat {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.actions {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.action_errors {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.observation.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.player_result {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.chat {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ResponseObservation {
    fn new() -> ResponseObservation {
        ResponseObservation::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseObservation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Action>>(
                    "actions",
                    ResponseObservation::get_actions_for_reflect,
                    ResponseObservation::mut_actions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ActionError>>(
                    "action_errors",
                    ResponseObservation::get_action_errors_for_reflect,
                    ResponseObservation::mut_action_errors_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Observation>>(
                    "observation",
                    ResponseObservation::get_observation_for_reflect,
                    ResponseObservation::mut_observation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlayerResult>>(
                    "player_result",
                    ResponseObservation::get_player_result_for_reflect,
                    ResponseObservation::mut_player_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChatReceived>>(
                    "chat",
                    ResponseObservation::get_chat_for_reflect,
                    ResponseObservation::mut_chat_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseObservation>(
                    "ResponseObservation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseObservation {
    fn clear(&mut self) {
        self.clear_actions();
        self.clear_action_errors();
        self.clear_observation();
        self.clear_player_result();
        self.clear_chat();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseObservation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseObservation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChatReceived {
    // message fields
    player_id: ::std::option::Option<i32>,
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChatReceived {}

impl ChatReceived {
    pub fn new() -> ChatReceived {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChatReceived {
        static mut instance: ::protobuf::lazy::Lazy<ChatReceived> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChatReceived,
        };
        unsafe {
            instance.get(ChatReceived::new)
        }
    }

    // optional int32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: i32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> i32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_id
    }

    // optional string message = 2;

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

impl ::protobuf::Message for ChatReceived {
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
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.message.as_ref() {
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

impl ::protobuf::MessageStatic for ChatReceived {
    fn new() -> ChatReceived {
        ChatReceived::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChatReceived>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_id",
                    ChatReceived::get_player_id_for_reflect,
                    ChatReceived::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    ChatReceived::get_message_for_reflect,
                    ChatReceived::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChatReceived>(
                    "ChatReceived",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChatReceived {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChatReceived {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChatReceived {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestAction {
    // message fields
    actions: ::protobuf::RepeatedField<Action>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestAction {}

impl RequestAction {
    pub fn new() -> RequestAction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestAction {
        static mut instance: ::protobuf::lazy::Lazy<RequestAction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestAction,
        };
        unsafe {
            instance.get(RequestAction::new)
        }
    }

    // repeated .SC2APIProtocol.Action actions = 1;

    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }

    // Param is passed by value, moved
    pub fn set_actions(&mut self, v: ::protobuf::RepeatedField<Action>) {
        self.actions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_actions(&mut self) -> &mut ::protobuf::RepeatedField<Action> {
        &mut self.actions
    }

    // Take field
    pub fn take_actions(&mut self) -> ::protobuf::RepeatedField<Action> {
        ::std::mem::replace(&mut self.actions, ::protobuf::RepeatedField::new())
    }

    pub fn get_actions(&self) -> &[Action] {
        &self.actions
    }

    fn get_actions_for_reflect(&self) -> &::protobuf::RepeatedField<Action> {
        &self.actions
    }

    fn mut_actions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Action> {
        &mut self.actions
    }
}

impl ::protobuf::Message for RequestAction {
    fn is_initialized(&self) -> bool {
        for v in &self.actions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.actions)?;
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
        for value in &self.actions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.actions {
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

impl ::protobuf::MessageStatic for RequestAction {
    fn new() -> RequestAction {
        RequestAction::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestAction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Action>>(
                    "actions",
                    RequestAction::get_actions_for_reflect,
                    RequestAction::mut_actions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestAction>(
                    "RequestAction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestAction {
    fn clear(&mut self) {
        self.clear_actions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestAction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseAction {
    // message fields
    result: ::std::vec::Vec<super::error::ActionResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseAction {}

impl ResponseAction {
    pub fn new() -> ResponseAction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseAction {
        static mut instance: ::protobuf::lazy::Lazy<ResponseAction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseAction,
        };
        unsafe {
            instance.get(ResponseAction::new)
        }
    }

    // repeated .SC2APIProtocol.ActionResult result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: ::std::vec::Vec<super::error::ActionResult>) {
        self.result = v;
    }

    // Mutable pointer to the field.
    pub fn mut_result(&mut self) -> &mut ::std::vec::Vec<super::error::ActionResult> {
        &mut self.result
    }

    // Take field
    pub fn take_result(&mut self) -> ::std::vec::Vec<super::error::ActionResult> {
        ::std::mem::replace(&mut self.result, ::std::vec::Vec::new())
    }

    pub fn get_result(&self) -> &[super::error::ActionResult] {
        &self.result
    }

    fn get_result_for_reflect(&self) -> &::std::vec::Vec<super::error::ActionResult> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::vec::Vec<super::error::ActionResult> {
        &mut self.result
    }
}

impl ::protobuf::Message for ResponseAction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.result)?;
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
        for value in &self.result {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.result {
            os.write_enum(1, v.value())?;
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

impl ::protobuf::MessageStatic for ResponseAction {
    fn new() -> ResponseAction {
        ResponseAction::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseAction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::error::ActionResult>>(
                    "result",
                    ResponseAction::get_result_for_reflect,
                    ResponseAction::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseAction>(
                    "ResponseAction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseAction {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseAction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestStep {
    // message fields
    count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestStep {}

impl RequestStep {
    pub fn new() -> RequestStep {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestStep {
        static mut instance: ::protobuf::lazy::Lazy<RequestStep> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestStep,
        };
        unsafe {
            instance.get(RequestStep::new)
        }
    }

    // optional uint32 count = 1;

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

impl ::protobuf::Message for RequestStep {
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
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.count {
            os.write_uint32(1, v)?;
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

impl ::protobuf::MessageStatic for RequestStep {
    fn new() -> RequestStep {
        RequestStep::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestStep>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "count",
                    RequestStep::get_count_for_reflect,
                    RequestStep::mut_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestStep>(
                    "RequestStep",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestStep {
    fn clear(&mut self) {
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestStep {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestStep {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseStep {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseStep {}

impl ResponseStep {
    pub fn new() -> ResponseStep {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseStep {
        static mut instance: ::protobuf::lazy::Lazy<ResponseStep> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseStep,
        };
        unsafe {
            instance.get(ResponseStep::new)
        }
    }
}

impl ::protobuf::Message for ResponseStep {
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

impl ::protobuf::MessageStatic for ResponseStep {
    fn new() -> ResponseStep {
        ResponseStep::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseStep>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseStep>(
                    "ResponseStep",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseStep {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseStep {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseStep {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestData {
    // message fields
    ability_id: ::std::option::Option<bool>,
    unit_type_id: ::std::option::Option<bool>,
    upgrade_id: ::std::option::Option<bool>,
    buff_id: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestData {}

impl RequestData {
    pub fn new() -> RequestData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestData {
        static mut instance: ::protobuf::lazy::Lazy<RequestData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestData,
        };
        unsafe {
            instance.get(RequestData::new)
        }
    }

    // optional bool ability_id = 1;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: bool) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> bool {
        self.ability_id.unwrap_or(false)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.ability_id
    }

    // optional bool unit_type_id = 2;

    pub fn clear_unit_type_id(&mut self) {
        self.unit_type_id = ::std::option::Option::None;
    }

    pub fn has_unit_type_id(&self) -> bool {
        self.unit_type_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unit_type_id(&mut self, v: bool) {
        self.unit_type_id = ::std::option::Option::Some(v);
    }

    pub fn get_unit_type_id(&self) -> bool {
        self.unit_type_id.unwrap_or(false)
    }

    fn get_unit_type_id_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.unit_type_id
    }

    fn mut_unit_type_id_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.unit_type_id
    }

    // optional bool upgrade_id = 3;

    pub fn clear_upgrade_id(&mut self) {
        self.upgrade_id = ::std::option::Option::None;
    }

    pub fn has_upgrade_id(&self) -> bool {
        self.upgrade_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upgrade_id(&mut self, v: bool) {
        self.upgrade_id = ::std::option::Option::Some(v);
    }

    pub fn get_upgrade_id(&self) -> bool {
        self.upgrade_id.unwrap_or(false)
    }

    fn get_upgrade_id_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.upgrade_id
    }

    fn mut_upgrade_id_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.upgrade_id
    }

    // optional bool buff_id = 4;

    pub fn clear_buff_id(&mut self) {
        self.buff_id = ::std::option::Option::None;
    }

    pub fn has_buff_id(&self) -> bool {
        self.buff_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buff_id(&mut self, v: bool) {
        self.buff_id = ::std::option::Option::Some(v);
    }

    pub fn get_buff_id(&self) -> bool {
        self.buff_id.unwrap_or(false)
    }

    fn get_buff_id_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.buff_id
    }

    fn mut_buff_id_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.buff_id
    }
}

impl ::protobuf::Message for RequestData {
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
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.unit_type_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.upgrade_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.buff_id = ::std::option::Option::Some(tmp);
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
            my_size += 2;
        }
        if let Some(v) = self.unit_type_id {
            my_size += 2;
        }
        if let Some(v) = self.upgrade_id {
            my_size += 2;
        }
        if let Some(v) = self.buff_id {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ability_id {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.unit_type_id {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.upgrade_id {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.buff_id {
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

impl ::protobuf::MessageStatic for RequestData {
    fn new() -> RequestData {
        RequestData::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "ability_id",
                    RequestData::get_ability_id_for_reflect,
                    RequestData::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "unit_type_id",
                    RequestData::get_unit_type_id_for_reflect,
                    RequestData::mut_unit_type_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "upgrade_id",
                    RequestData::get_upgrade_id_for_reflect,
                    RequestData::mut_upgrade_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "buff_id",
                    RequestData::get_buff_id_for_reflect,
                    RequestData::mut_buff_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestData>(
                    "RequestData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestData {
    fn clear(&mut self) {
        self.clear_ability_id();
        self.clear_unit_type_id();
        self.clear_upgrade_id();
        self.clear_buff_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseData {
    // message fields
    abilities: ::protobuf::RepeatedField<super::data::AbilityData>,
    units: ::protobuf::RepeatedField<super::data::UnitTypeData>,
    upgrades: ::protobuf::RepeatedField<super::data::UpgradeData>,
    buffs: ::protobuf::RepeatedField<super::data::BuffData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseData {}

impl ResponseData {
    pub fn new() -> ResponseData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseData {
        static mut instance: ::protobuf::lazy::Lazy<ResponseData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseData,
        };
        unsafe {
            instance.get(ResponseData::new)
        }
    }

    // repeated .SC2APIProtocol.AbilityData abilities = 1;

    pub fn clear_abilities(&mut self) {
        self.abilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_abilities(&mut self, v: ::protobuf::RepeatedField<super::data::AbilityData>) {
        self.abilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_abilities(&mut self) -> &mut ::protobuf::RepeatedField<super::data::AbilityData> {
        &mut self.abilities
    }

    // Take field
    pub fn take_abilities(&mut self) -> ::protobuf::RepeatedField<super::data::AbilityData> {
        ::std::mem::replace(&mut self.abilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_abilities(&self) -> &[super::data::AbilityData] {
        &self.abilities
    }

    fn get_abilities_for_reflect(&self) -> &::protobuf::RepeatedField<super::data::AbilityData> {
        &self.abilities
    }

    fn mut_abilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::data::AbilityData> {
        &mut self.abilities
    }

    // repeated .SC2APIProtocol.UnitTypeData units = 2;

    pub fn clear_units(&mut self) {
        self.units.clear();
    }

    // Param is passed by value, moved
    pub fn set_units(&mut self, v: ::protobuf::RepeatedField<super::data::UnitTypeData>) {
        self.units = v;
    }

    // Mutable pointer to the field.
    pub fn mut_units(&mut self) -> &mut ::protobuf::RepeatedField<super::data::UnitTypeData> {
        &mut self.units
    }

    // Take field
    pub fn take_units(&mut self) -> ::protobuf::RepeatedField<super::data::UnitTypeData> {
        ::std::mem::replace(&mut self.units, ::protobuf::RepeatedField::new())
    }

    pub fn get_units(&self) -> &[super::data::UnitTypeData] {
        &self.units
    }

    fn get_units_for_reflect(&self) -> &::protobuf::RepeatedField<super::data::UnitTypeData> {
        &self.units
    }

    fn mut_units_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::data::UnitTypeData> {
        &mut self.units
    }

    // repeated .SC2APIProtocol.UpgradeData upgrades = 3;

    pub fn clear_upgrades(&mut self) {
        self.upgrades.clear();
    }

    // Param is passed by value, moved
    pub fn set_upgrades(&mut self, v: ::protobuf::RepeatedField<super::data::UpgradeData>) {
        self.upgrades = v;
    }

    // Mutable pointer to the field.
    pub fn mut_upgrades(&mut self) -> &mut ::protobuf::RepeatedField<super::data::UpgradeData> {
        &mut self.upgrades
    }

    // Take field
    pub fn take_upgrades(&mut self) -> ::protobuf::RepeatedField<super::data::UpgradeData> {
        ::std::mem::replace(&mut self.upgrades, ::protobuf::RepeatedField::new())
    }

    pub fn get_upgrades(&self) -> &[super::data::UpgradeData] {
        &self.upgrades
    }

    fn get_upgrades_for_reflect(&self) -> &::protobuf::RepeatedField<super::data::UpgradeData> {
        &self.upgrades
    }

    fn mut_upgrades_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::data::UpgradeData> {
        &mut self.upgrades
    }

    // repeated .SC2APIProtocol.BuffData buffs = 4;

    pub fn clear_buffs(&mut self) {
        self.buffs.clear();
    }

    // Param is passed by value, moved
    pub fn set_buffs(&mut self, v: ::protobuf::RepeatedField<super::data::BuffData>) {
        self.buffs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_buffs(&mut self) -> &mut ::protobuf::RepeatedField<super::data::BuffData> {
        &mut self.buffs
    }

    // Take field
    pub fn take_buffs(&mut self) -> ::protobuf::RepeatedField<super::data::BuffData> {
        ::std::mem::replace(&mut self.buffs, ::protobuf::RepeatedField::new())
    }

    pub fn get_buffs(&self) -> &[super::data::BuffData] {
        &self.buffs
    }

    fn get_buffs_for_reflect(&self) -> &::protobuf::RepeatedField<super::data::BuffData> {
        &self.buffs
    }

    fn mut_buffs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::data::BuffData> {
        &mut self.buffs
    }
}

impl ::protobuf::Message for ResponseData {
    fn is_initialized(&self) -> bool {
        for v in &self.abilities {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.units {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.upgrades {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.buffs {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.units)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.upgrades)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.buffs)?;
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
        for value in &self.units {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.upgrades {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.buffs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.units {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.upgrades {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.buffs {
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

impl ::protobuf::MessageStatic for ResponseData {
    fn new() -> ResponseData {
        ResponseData::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::data::AbilityData>>(
                    "abilities",
                    ResponseData::get_abilities_for_reflect,
                    ResponseData::mut_abilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::data::UnitTypeData>>(
                    "units",
                    ResponseData::get_units_for_reflect,
                    ResponseData::mut_units_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::data::UpgradeData>>(
                    "upgrades",
                    ResponseData::get_upgrades_for_reflect,
                    ResponseData::mut_upgrades_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::data::BuffData>>(
                    "buffs",
                    ResponseData::get_buffs_for_reflect,
                    ResponseData::mut_buffs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseData>(
                    "ResponseData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseData {
    fn clear(&mut self) {
        self.clear_abilities();
        self.clear_units();
        self.clear_upgrades();
        self.clear_buffs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestSaveReplay {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestSaveReplay {}

impl RequestSaveReplay {
    pub fn new() -> RequestSaveReplay {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestSaveReplay {
        static mut instance: ::protobuf::lazy::Lazy<RequestSaveReplay> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestSaveReplay,
        };
        unsafe {
            instance.get(RequestSaveReplay::new)
        }
    }
}

impl ::protobuf::Message for RequestSaveReplay {
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

impl ::protobuf::MessageStatic for RequestSaveReplay {
    fn new() -> RequestSaveReplay {
        RequestSaveReplay::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestSaveReplay>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestSaveReplay>(
                    "RequestSaveReplay",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestSaveReplay {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestSaveReplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestSaveReplay {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseSaveReplay {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseSaveReplay {}

impl ResponseSaveReplay {
    pub fn new() -> ResponseSaveReplay {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseSaveReplay {
        static mut instance: ::protobuf::lazy::Lazy<ResponseSaveReplay> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseSaveReplay,
        };
        unsafe {
            instance.get(ResponseSaveReplay::new)
        }
    }

    // optional bytes data = 1;

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

impl ::protobuf::Message for ResponseSaveReplay {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
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
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(1, &v)?;
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

impl ::protobuf::MessageStatic for ResponseSaveReplay {
    fn new() -> ResponseSaveReplay {
        ResponseSaveReplay::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseSaveReplay>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ResponseSaveReplay::get_data_for_reflect,
                    ResponseSaveReplay::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseSaveReplay>(
                    "ResponseSaveReplay",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseSaveReplay {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseSaveReplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseSaveReplay {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestReplayInfo {
    // message fields
    download_data: ::std::option::Option<bool>,
    // message oneof groups
    replay: ::std::option::Option<RequestReplayInfo_oneof_replay>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestReplayInfo {}

#[derive(Clone,PartialEq)]
pub enum RequestReplayInfo_oneof_replay {
    replay_path(::std::string::String),
    replay_data(::std::vec::Vec<u8>),
}

impl RequestReplayInfo {
    pub fn new() -> RequestReplayInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestReplayInfo {
        static mut instance: ::protobuf::lazy::Lazy<RequestReplayInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestReplayInfo,
        };
        unsafe {
            instance.get(RequestReplayInfo::new)
        }
    }

    // optional string replay_path = 1;

    pub fn clear_replay_path(&mut self) {
        self.replay = ::std::option::Option::None;
    }

    pub fn has_replay_path(&self) -> bool {
        match self.replay {
            ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_path(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_replay_path(&mut self, v: ::std::string::String) {
        self.replay = ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_path(v))
    }

    // Mutable pointer to the field.
    pub fn mut_replay_path(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_path(_)) = self.replay {
        } else {
            self.replay = ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_path(::std::string::String::new()));
        }
        match self.replay {
            ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_path(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_replay_path(&mut self) -> ::std::string::String {
        if self.has_replay_path() {
            match self.replay.take() {
                ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_path(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    pub fn get_replay_path(&self) -> &str {
        match self.replay {
            ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_path(ref v)) => v,
            _ => "",
        }
    }

    // optional bytes replay_data = 2;

    pub fn clear_replay_data(&mut self) {
        self.replay = ::std::option::Option::None;
    }

    pub fn has_replay_data(&self) -> bool {
        match self.replay {
            ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_data(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_replay_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.replay = ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_data(v))
    }

    // Mutable pointer to the field.
    pub fn mut_replay_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if let ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_data(_)) = self.replay {
        } else {
            self.replay = ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_data(::std::vec::Vec::new()));
        }
        match self.replay {
            ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_data(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_replay_data(&mut self) -> ::std::vec::Vec<u8> {
        if self.has_replay_data() {
            match self.replay.take() {
                ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_data(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::vec::Vec::new()
        }
    }

    pub fn get_replay_data(&self) -> &[u8] {
        match self.replay {
            ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_data(ref v)) => v,
            _ => &[],
        }
    }

    // optional bool download_data = 3;

    pub fn clear_download_data(&mut self) {
        self.download_data = ::std::option::Option::None;
    }

    pub fn has_download_data(&self) -> bool {
        self.download_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_download_data(&mut self, v: bool) {
        self.download_data = ::std::option::Option::Some(v);
    }

    pub fn get_download_data(&self) -> bool {
        self.download_data.unwrap_or(false)
    }

    fn get_download_data_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.download_data
    }

    fn mut_download_data_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.download_data
    }
}

impl ::protobuf::Message for RequestReplayInfo {
    fn is_initialized(&self) -> bool {
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
                    self.replay = ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_path(is.read_string()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.replay = ::std::option::Option::Some(RequestReplayInfo_oneof_replay::replay_data(is.read_bytes()?));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.download_data = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.download_data {
            my_size += 2;
        }
        if let ::std::option::Option::Some(ref v) = self.replay {
            match v {
                &RequestReplayInfo_oneof_replay::replay_path(ref v) => {
                    my_size += ::protobuf::rt::string_size(1, &v);
                },
                &RequestReplayInfo_oneof_replay::replay_data(ref v) => {
                    my_size += ::protobuf::rt::bytes_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.download_data {
            os.write_bool(3, v)?;
        }
        if let ::std::option::Option::Some(ref v) = self.replay {
            match v {
                &RequestReplayInfo_oneof_replay::replay_path(ref v) => {
                    os.write_string(1, v)?;
                },
                &RequestReplayInfo_oneof_replay::replay_data(ref v) => {
                    os.write_bytes(2, v)?;
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

impl ::protobuf::MessageStatic for RequestReplayInfo {
    fn new() -> RequestReplayInfo {
        RequestReplayInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestReplayInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                    "replay_path",
                    RequestReplayInfo::has_replay_path,
                    RequestReplayInfo::get_replay_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor::<_>(
                    "replay_data",
                    RequestReplayInfo::has_replay_data,
                    RequestReplayInfo::get_replay_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "download_data",
                    RequestReplayInfo::get_download_data_for_reflect,
                    RequestReplayInfo::mut_download_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestReplayInfo>(
                    "RequestReplayInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestReplayInfo {
    fn clear(&mut self) {
        self.clear_replay_path();
        self.clear_replay_data();
        self.clear_download_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestReplayInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestReplayInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlayerInfoExtra {
    // message fields
    player_info: ::protobuf::SingularPtrField<PlayerInfo>,
    player_result: ::protobuf::SingularPtrField<PlayerResult>,
    player_mmr: ::std::option::Option<i32>,
    player_apm: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerInfoExtra {}

impl PlayerInfoExtra {
    pub fn new() -> PlayerInfoExtra {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerInfoExtra {
        static mut instance: ::protobuf::lazy::Lazy<PlayerInfoExtra> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerInfoExtra,
        };
        unsafe {
            instance.get(PlayerInfoExtra::new)
        }
    }

    // optional .SC2APIProtocol.PlayerInfo player_info = 1;

    pub fn clear_player_info(&mut self) {
        self.player_info.clear();
    }

    pub fn has_player_info(&self) -> bool {
        self.player_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_info(&mut self, v: PlayerInfo) {
        self.player_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_info(&mut self) -> &mut PlayerInfo {
        if self.player_info.is_none() {
            self.player_info.set_default();
        }
        self.player_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_info(&mut self) -> PlayerInfo {
        self.player_info.take().unwrap_or_else(|| PlayerInfo::new())
    }

    pub fn get_player_info(&self) -> &PlayerInfo {
        self.player_info.as_ref().unwrap_or_else(|| PlayerInfo::default_instance())
    }

    fn get_player_info_for_reflect(&self) -> &::protobuf::SingularPtrField<PlayerInfo> {
        &self.player_info
    }

    fn mut_player_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PlayerInfo> {
        &mut self.player_info
    }

    // optional .SC2APIProtocol.PlayerResult player_result = 2;

    pub fn clear_player_result(&mut self) {
        self.player_result.clear();
    }

    pub fn has_player_result(&self) -> bool {
        self.player_result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_result(&mut self, v: PlayerResult) {
        self.player_result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_result(&mut self) -> &mut PlayerResult {
        if self.player_result.is_none() {
            self.player_result.set_default();
        }
        self.player_result.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_result(&mut self) -> PlayerResult {
        self.player_result.take().unwrap_or_else(|| PlayerResult::new())
    }

    pub fn get_player_result(&self) -> &PlayerResult {
        self.player_result.as_ref().unwrap_or_else(|| PlayerResult::default_instance())
    }

    fn get_player_result_for_reflect(&self) -> &::protobuf::SingularPtrField<PlayerResult> {
        &self.player_result
    }

    fn mut_player_result_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PlayerResult> {
        &mut self.player_result
    }

    // optional int32 player_mmr = 3;

    pub fn clear_player_mmr(&mut self) {
        self.player_mmr = ::std::option::Option::None;
    }

    pub fn has_player_mmr(&self) -> bool {
        self.player_mmr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_mmr(&mut self, v: i32) {
        self.player_mmr = ::std::option::Option::Some(v);
    }

    pub fn get_player_mmr(&self) -> i32 {
        self.player_mmr.unwrap_or(0)
    }

    fn get_player_mmr_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_mmr
    }

    fn mut_player_mmr_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_mmr
    }

    // optional int32 player_apm = 4;

    pub fn clear_player_apm(&mut self) {
        self.player_apm = ::std::option::Option::None;
    }

    pub fn has_player_apm(&self) -> bool {
        self.player_apm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_apm(&mut self, v: i32) {
        self.player_apm = ::std::option::Option::Some(v);
    }

    pub fn get_player_apm(&self) -> i32 {
        self.player_apm.unwrap_or(0)
    }

    fn get_player_apm_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_apm
    }

    fn mut_player_apm_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_apm
    }
}

impl ::protobuf::Message for PlayerInfoExtra {
    fn is_initialized(&self) -> bool {
        for v in &self.player_info {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.player_result {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_info)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_result)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.player_mmr = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.player_apm = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.player_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.player_result.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.player_mmr {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.player_apm {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.player_info.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.player_result.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.player_mmr {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.player_apm {
            os.write_int32(4, v)?;
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

impl ::protobuf::MessageStatic for PlayerInfoExtra {
    fn new() -> PlayerInfoExtra {
        PlayerInfoExtra::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerInfoExtra>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlayerInfo>>(
                    "player_info",
                    PlayerInfoExtra::get_player_info_for_reflect,
                    PlayerInfoExtra::mut_player_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlayerResult>>(
                    "player_result",
                    PlayerInfoExtra::get_player_result_for_reflect,
                    PlayerInfoExtra::mut_player_result_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_mmr",
                    PlayerInfoExtra::get_player_mmr_for_reflect,
                    PlayerInfoExtra::mut_player_mmr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_apm",
                    PlayerInfoExtra::get_player_apm_for_reflect,
                    PlayerInfoExtra::mut_player_apm_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerInfoExtra>(
                    "PlayerInfoExtra",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerInfoExtra {
    fn clear(&mut self) {
        self.clear_player_info();
        self.clear_player_result();
        self.clear_player_mmr();
        self.clear_player_apm();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlayerInfoExtra {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerInfoExtra {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseReplayInfo {
    // message fields
    map_name: ::protobuf::SingularField<::std::string::String>,
    local_map_path: ::protobuf::SingularField<::std::string::String>,
    player_info: ::protobuf::RepeatedField<PlayerInfoExtra>,
    game_duration_loops: ::std::option::Option<u32>,
    game_duration_seconds: ::std::option::Option<f32>,
    game_version: ::protobuf::SingularField<::std::string::String>,
    data_version: ::protobuf::SingularField<::std::string::String>,
    data_build: ::std::option::Option<u32>,
    base_build: ::std::option::Option<u32>,
    error: ::std::option::Option<ResponseReplayInfo_Error>,
    error_details: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseReplayInfo {}

impl ResponseReplayInfo {
    pub fn new() -> ResponseReplayInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseReplayInfo {
        static mut instance: ::protobuf::lazy::Lazy<ResponseReplayInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseReplayInfo,
        };
        unsafe {
            instance.get(ResponseReplayInfo::new)
        }
    }

    // optional string map_name = 1;

    pub fn clear_map_name(&mut self) {
        self.map_name.clear();
    }

    pub fn has_map_name(&self) -> bool {
        self.map_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_name(&mut self, v: ::std::string::String) {
        self.map_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_name(&mut self) -> &mut ::std::string::String {
        if self.map_name.is_none() {
            self.map_name.set_default();
        }
        self.map_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_name(&mut self) -> ::std::string::String {
        self.map_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map_name(&self) -> &str {
        match self.map_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map_name
    }

    fn mut_map_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map_name
    }

    // optional string local_map_path = 2;

    pub fn clear_local_map_path(&mut self) {
        self.local_map_path.clear();
    }

    pub fn has_local_map_path(&self) -> bool {
        self.local_map_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_local_map_path(&mut self, v: ::std::string::String) {
        self.local_map_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_local_map_path(&mut self) -> &mut ::std::string::String {
        if self.local_map_path.is_none() {
            self.local_map_path.set_default();
        }
        self.local_map_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_local_map_path(&mut self) -> ::std::string::String {
        self.local_map_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_local_map_path(&self) -> &str {
        match self.local_map_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_local_map_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.local_map_path
    }

    fn mut_local_map_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.local_map_path
    }

    // repeated .SC2APIProtocol.PlayerInfoExtra player_info = 3;

    pub fn clear_player_info(&mut self) {
        self.player_info.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_info(&mut self, v: ::protobuf::RepeatedField<PlayerInfoExtra>) {
        self.player_info = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_info(&mut self) -> &mut ::protobuf::RepeatedField<PlayerInfoExtra> {
        &mut self.player_info
    }

    // Take field
    pub fn take_player_info(&mut self) -> ::protobuf::RepeatedField<PlayerInfoExtra> {
        ::std::mem::replace(&mut self.player_info, ::protobuf::RepeatedField::new())
    }

    pub fn get_player_info(&self) -> &[PlayerInfoExtra] {
        &self.player_info
    }

    fn get_player_info_for_reflect(&self) -> &::protobuf::RepeatedField<PlayerInfoExtra> {
        &self.player_info
    }

    fn mut_player_info_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PlayerInfoExtra> {
        &mut self.player_info
    }

    // optional uint32 game_duration_loops = 4;

    pub fn clear_game_duration_loops(&mut self) {
        self.game_duration_loops = ::std::option::Option::None;
    }

    pub fn has_game_duration_loops(&self) -> bool {
        self.game_duration_loops.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_duration_loops(&mut self, v: u32) {
        self.game_duration_loops = ::std::option::Option::Some(v);
    }

    pub fn get_game_duration_loops(&self) -> u32 {
        self.game_duration_loops.unwrap_or(0)
    }

    fn get_game_duration_loops_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_duration_loops
    }

    fn mut_game_duration_loops_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_duration_loops
    }

    // optional float game_duration_seconds = 5;

    pub fn clear_game_duration_seconds(&mut self) {
        self.game_duration_seconds = ::std::option::Option::None;
    }

    pub fn has_game_duration_seconds(&self) -> bool {
        self.game_duration_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_duration_seconds(&mut self, v: f32) {
        self.game_duration_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_game_duration_seconds(&self) -> f32 {
        self.game_duration_seconds.unwrap_or(0.)
    }

    fn get_game_duration_seconds_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.game_duration_seconds
    }

    fn mut_game_duration_seconds_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.game_duration_seconds
    }

    // optional string game_version = 6;

    pub fn clear_game_version(&mut self) {
        self.game_version.clear();
    }

    pub fn has_game_version(&self) -> bool {
        self.game_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_version(&mut self, v: ::std::string::String) {
        self.game_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_version(&mut self) -> &mut ::std::string::String {
        if self.game_version.is_none() {
            self.game_version.set_default();
        }
        self.game_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_version(&mut self) -> ::std::string::String {
        self.game_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_version(&self) -> &str {
        match self.game_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_version
    }

    fn mut_game_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_version
    }

    // optional string data_version = 11;

    pub fn clear_data_version(&mut self) {
        self.data_version.clear();
    }

    pub fn has_data_version(&self) -> bool {
        self.data_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_version(&mut self, v: ::std::string::String) {
        self.data_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_version(&mut self) -> &mut ::std::string::String {
        if self.data_version.is_none() {
            self.data_version.set_default();
        }
        self.data_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_data_version(&mut self) -> ::std::string::String {
        self.data_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data_version(&self) -> &str {
        match self.data_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_data_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.data_version
    }

    fn mut_data_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.data_version
    }

    // optional uint32 data_build = 7;

    pub fn clear_data_build(&mut self) {
        self.data_build = ::std::option::Option::None;
    }

    pub fn has_data_build(&self) -> bool {
        self.data_build.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_build(&mut self, v: u32) {
        self.data_build = ::std::option::Option::Some(v);
    }

    pub fn get_data_build(&self) -> u32 {
        self.data_build.unwrap_or(0)
    }

    fn get_data_build_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.data_build
    }

    fn mut_data_build_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.data_build
    }

    // optional uint32 base_build = 8;

    pub fn clear_base_build(&mut self) {
        self.base_build = ::std::option::Option::None;
    }

    pub fn has_base_build(&self) -> bool {
        self.base_build.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_build(&mut self, v: u32) {
        self.base_build = ::std::option::Option::Some(v);
    }

    pub fn get_base_build(&self) -> u32 {
        self.base_build.unwrap_or(0)
    }

    fn get_base_build_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.base_build
    }

    fn mut_base_build_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.base_build
    }

    // optional .SC2APIProtocol.ResponseReplayInfo.Error error = 9;

    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ResponseReplayInfo_Error) {
        self.error = ::std::option::Option::Some(v);
    }

    pub fn get_error(&self) -> ResponseReplayInfo_Error {
        self.error.unwrap_or(ResponseReplayInfo_Error::MissingReplay)
    }

    fn get_error_for_reflect(&self) -> &::std::option::Option<ResponseReplayInfo_Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::option::Option<ResponseReplayInfo_Error> {
        &mut self.error
    }

    // optional string error_details = 10;

    pub fn clear_error_details(&mut self) {
        self.error_details.clear();
    }

    pub fn has_error_details(&self) -> bool {
        self.error_details.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_details(&mut self, v: ::std::string::String) {
        self.error_details = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_details(&mut self) -> &mut ::std::string::String {
        if self.error_details.is_none() {
            self.error_details.set_default();
        }
        self.error_details.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_details(&mut self) -> ::std::string::String {
        self.error_details.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_details(&self) -> &str {
        match self.error_details.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_details_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_details
    }

    fn mut_error_details_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_details
    }
}

impl ::protobuf::Message for ResponseReplayInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.player_info {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.local_map_path)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.player_info)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.game_duration_loops = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.game_duration_seconds = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_version)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data_version)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.data_build = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.base_build = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.error = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_details)?;
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
        if let Some(ref v) = self.map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.local_map_path.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.player_info {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.game_duration_loops {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.game_duration_seconds {
            my_size += 5;
        }
        if let Some(ref v) = self.game_version.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.data_version.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        if let Some(v) = self.data_build {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.base_build {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.error {
            my_size += ::protobuf::rt::enum_size(9, v);
        }
        if let Some(ref v) = self.error_details.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.map_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.local_map_path.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.player_info {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.game_duration_loops {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.game_duration_seconds {
            os.write_float(5, v)?;
        }
        if let Some(ref v) = self.game_version.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.data_version.as_ref() {
            os.write_string(11, &v)?;
        }
        if let Some(v) = self.data_build {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.base_build {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.error {
            os.write_enum(9, v.value())?;
        }
        if let Some(ref v) = self.error_details.as_ref() {
            os.write_string(10, &v)?;
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

impl ::protobuf::MessageStatic for ResponseReplayInfo {
    fn new() -> ResponseReplayInfo {
        ResponseReplayInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseReplayInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_name",
                    ResponseReplayInfo::get_map_name_for_reflect,
                    ResponseReplayInfo::mut_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "local_map_path",
                    ResponseReplayInfo::get_local_map_path_for_reflect,
                    ResponseReplayInfo::mut_local_map_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlayerInfoExtra>>(
                    "player_info",
                    ResponseReplayInfo::get_player_info_for_reflect,
                    ResponseReplayInfo::mut_player_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_duration_loops",
                    ResponseReplayInfo::get_game_duration_loops_for_reflect,
                    ResponseReplayInfo::mut_game_duration_loops_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "game_duration_seconds",
                    ResponseReplayInfo::get_game_duration_seconds_for_reflect,
                    ResponseReplayInfo::mut_game_duration_seconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_version",
                    ResponseReplayInfo::get_game_version_for_reflect,
                    ResponseReplayInfo::mut_game_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data_version",
                    ResponseReplayInfo::get_data_version_for_reflect,
                    ResponseReplayInfo::mut_data_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "data_build",
                    ResponseReplayInfo::get_data_build_for_reflect,
                    ResponseReplayInfo::mut_data_build_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "base_build",
                    ResponseReplayInfo::get_base_build_for_reflect,
                    ResponseReplayInfo::mut_base_build_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ResponseReplayInfo_Error>>(
                    "error",
                    ResponseReplayInfo::get_error_for_reflect,
                    ResponseReplayInfo::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_details",
                    ResponseReplayInfo::get_error_details_for_reflect,
                    ResponseReplayInfo::mut_error_details_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseReplayInfo>(
                    "ResponseReplayInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseReplayInfo {
    fn clear(&mut self) {
        self.clear_map_name();
        self.clear_local_map_path();
        self.clear_player_info();
        self.clear_game_duration_loops();
        self.clear_game_duration_seconds();
        self.clear_game_version();
        self.clear_data_version();
        self.clear_data_build();
        self.clear_base_build();
        self.clear_error();
        self.clear_error_details();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseReplayInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseReplayInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ResponseReplayInfo_Error {
    MissingReplay = 1,
    InvalidReplayPath = 2,
    InvalidReplayData = 3,
    ParsingError = 4,
    DownloadError = 5,
}

impl ::protobuf::ProtobufEnum for ResponseReplayInfo_Error {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ResponseReplayInfo_Error> {
        match value {
            1 => ::std::option::Option::Some(ResponseReplayInfo_Error::MissingReplay),
            2 => ::std::option::Option::Some(ResponseReplayInfo_Error::InvalidReplayPath),
            3 => ::std::option::Option::Some(ResponseReplayInfo_Error::InvalidReplayData),
            4 => ::std::option::Option::Some(ResponseReplayInfo_Error::ParsingError),
            5 => ::std::option::Option::Some(ResponseReplayInfo_Error::DownloadError),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ResponseReplayInfo_Error] = &[
            ResponseReplayInfo_Error::MissingReplay,
            ResponseReplayInfo_Error::InvalidReplayPath,
            ResponseReplayInfo_Error::InvalidReplayData,
            ResponseReplayInfo_Error::ParsingError,
            ResponseReplayInfo_Error::DownloadError,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ResponseReplayInfo_Error>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ResponseReplayInfo_Error", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ResponseReplayInfo_Error {
}

impl ::protobuf::reflect::ProtobufValue for ResponseReplayInfo_Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestAvailableMaps {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestAvailableMaps {}

impl RequestAvailableMaps {
    pub fn new() -> RequestAvailableMaps {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestAvailableMaps {
        static mut instance: ::protobuf::lazy::Lazy<RequestAvailableMaps> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestAvailableMaps,
        };
        unsafe {
            instance.get(RequestAvailableMaps::new)
        }
    }
}

impl ::protobuf::Message for RequestAvailableMaps {
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

impl ::protobuf::MessageStatic for RequestAvailableMaps {
    fn new() -> RequestAvailableMaps {
        RequestAvailableMaps::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestAvailableMaps>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestAvailableMaps>(
                    "RequestAvailableMaps",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestAvailableMaps {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestAvailableMaps {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestAvailableMaps {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseAvailableMaps {
    // message fields
    local_map_paths: ::protobuf::RepeatedField<::std::string::String>,
    battlenet_map_names: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseAvailableMaps {}

impl ResponseAvailableMaps {
    pub fn new() -> ResponseAvailableMaps {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseAvailableMaps {
        static mut instance: ::protobuf::lazy::Lazy<ResponseAvailableMaps> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseAvailableMaps,
        };
        unsafe {
            instance.get(ResponseAvailableMaps::new)
        }
    }

    // repeated string local_map_paths = 1;

    pub fn clear_local_map_paths(&mut self) {
        self.local_map_paths.clear();
    }

    // Param is passed by value, moved
    pub fn set_local_map_paths(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.local_map_paths = v;
    }

    // Mutable pointer to the field.
    pub fn mut_local_map_paths(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.local_map_paths
    }

    // Take field
    pub fn take_local_map_paths(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.local_map_paths, ::protobuf::RepeatedField::new())
    }

    pub fn get_local_map_paths(&self) -> &[::std::string::String] {
        &self.local_map_paths
    }

    fn get_local_map_paths_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.local_map_paths
    }

    fn mut_local_map_paths_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.local_map_paths
    }

    // repeated string battlenet_map_names = 2;

    pub fn clear_battlenet_map_names(&mut self) {
        self.battlenet_map_names.clear();
    }

    // Param is passed by value, moved
    pub fn set_battlenet_map_names(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.battlenet_map_names = v;
    }

    // Mutable pointer to the field.
    pub fn mut_battlenet_map_names(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.battlenet_map_names
    }

    // Take field
    pub fn take_battlenet_map_names(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.battlenet_map_names, ::protobuf::RepeatedField::new())
    }

    pub fn get_battlenet_map_names(&self) -> &[::std::string::String] {
        &self.battlenet_map_names
    }

    fn get_battlenet_map_names_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.battlenet_map_names
    }

    fn mut_battlenet_map_names_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.battlenet_map_names
    }
}

impl ::protobuf::Message for ResponseAvailableMaps {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.local_map_paths)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.battlenet_map_names)?;
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
        for value in &self.local_map_paths {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.battlenet_map_names {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.local_map_paths {
            os.write_string(1, &v)?;
        };
        for v in &self.battlenet_map_names {
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

impl ::protobuf::MessageStatic for ResponseAvailableMaps {
    fn new() -> ResponseAvailableMaps {
        ResponseAvailableMaps::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseAvailableMaps>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "local_map_paths",
                    ResponseAvailableMaps::get_local_map_paths_for_reflect,
                    ResponseAvailableMaps::mut_local_map_paths_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "battlenet_map_names",
                    ResponseAvailableMaps::get_battlenet_map_names_for_reflect,
                    ResponseAvailableMaps::mut_battlenet_map_names_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseAvailableMaps>(
                    "ResponseAvailableMaps",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseAvailableMaps {
    fn clear(&mut self) {
        self.clear_local_map_paths();
        self.clear_battlenet_map_names();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseAvailableMaps {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseAvailableMaps {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestSaveMap {
    // message fields
    map_path: ::protobuf::SingularField<::std::string::String>,
    map_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestSaveMap {}

impl RequestSaveMap {
    pub fn new() -> RequestSaveMap {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestSaveMap {
        static mut instance: ::protobuf::lazy::Lazy<RequestSaveMap> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestSaveMap,
        };
        unsafe {
            instance.get(RequestSaveMap::new)
        }
    }

    // optional string map_path = 1;

    pub fn clear_map_path(&mut self) {
        self.map_path.clear();
    }

    pub fn has_map_path(&self) -> bool {
        self.map_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_path(&mut self, v: ::std::string::String) {
        self.map_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_path(&mut self) -> &mut ::std::string::String {
        if self.map_path.is_none() {
            self.map_path.set_default();
        }
        self.map_path.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_path(&mut self) -> ::std::string::String {
        self.map_path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map_path(&self) -> &str {
        match self.map_path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map_path
    }

    fn mut_map_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map_path
    }

    // optional bytes map_data = 2;

    pub fn clear_map_data(&mut self) {
        self.map_data.clear();
    }

    pub fn has_map_data(&self) -> bool {
        self.map_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.map_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.map_data.is_none() {
            self.map_data.set_default();
        }
        self.map_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_data(&mut self) -> ::std::vec::Vec<u8> {
        self.map_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_map_data(&self) -> &[u8] {
        match self.map_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_map_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.map_data
    }

    fn mut_map_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.map_data
    }
}

impl ::protobuf::Message for RequestSaveMap {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.map_data)?;
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
        if let Some(ref v) = self.map_path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.map_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.map_path.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.map_data.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for RequestSaveMap {
    fn new() -> RequestSaveMap {
        RequestSaveMap::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestSaveMap>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_path",
                    RequestSaveMap::get_map_path_for_reflect,
                    RequestSaveMap::mut_map_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "map_data",
                    RequestSaveMap::get_map_data_for_reflect,
                    RequestSaveMap::mut_map_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestSaveMap>(
                    "RequestSaveMap",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestSaveMap {
    fn clear(&mut self) {
        self.clear_map_path();
        self.clear_map_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestSaveMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestSaveMap {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseSaveMap {
    // message fields
    error: ::std::option::Option<ResponseSaveMap_Error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseSaveMap {}

impl ResponseSaveMap {
    pub fn new() -> ResponseSaveMap {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseSaveMap {
        static mut instance: ::protobuf::lazy::Lazy<ResponseSaveMap> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseSaveMap,
        };
        unsafe {
            instance.get(ResponseSaveMap::new)
        }
    }

    // optional .SC2APIProtocol.ResponseSaveMap.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None;
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ResponseSaveMap_Error) {
        self.error = ::std::option::Option::Some(v);
    }

    pub fn get_error(&self) -> ResponseSaveMap_Error {
        self.error.unwrap_or(ResponseSaveMap_Error::InvalidMapData)
    }

    fn get_error_for_reflect(&self) -> &::std::option::Option<ResponseSaveMap_Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::std::option::Option<ResponseSaveMap_Error> {
        &mut self.error
    }
}

impl ::protobuf::Message for ResponseSaveMap {
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
                    self.error = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.error {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error {
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

impl ::protobuf::MessageStatic for ResponseSaveMap {
    fn new() -> ResponseSaveMap {
        ResponseSaveMap::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseSaveMap>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ResponseSaveMap_Error>>(
                    "error",
                    ResponseSaveMap::get_error_for_reflect,
                    ResponseSaveMap::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseSaveMap>(
                    "ResponseSaveMap",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseSaveMap {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseSaveMap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseSaveMap {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ResponseSaveMap_Error {
    InvalidMapData = 1,
}

impl ::protobuf::ProtobufEnum for ResponseSaveMap_Error {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ResponseSaveMap_Error> {
        match value {
            1 => ::std::option::Option::Some(ResponseSaveMap_Error::InvalidMapData),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ResponseSaveMap_Error] = &[
            ResponseSaveMap_Error::InvalidMapData,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ResponseSaveMap_Error>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ResponseSaveMap_Error", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ResponseSaveMap_Error {
}

impl ::protobuf::reflect::ProtobufValue for ResponseSaveMap_Error {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestPing {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestPing {}

impl RequestPing {
    pub fn new() -> RequestPing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestPing {
        static mut instance: ::protobuf::lazy::Lazy<RequestPing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestPing,
        };
        unsafe {
            instance.get(RequestPing::new)
        }
    }
}

impl ::protobuf::Message for RequestPing {
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

impl ::protobuf::MessageStatic for RequestPing {
    fn new() -> RequestPing {
        RequestPing::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestPing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RequestPing>(
                    "RequestPing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestPing {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestPing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestPing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponsePing {
    // message fields
    game_version: ::protobuf::SingularField<::std::string::String>,
    data_version: ::protobuf::SingularField<::std::string::String>,
    data_build: ::std::option::Option<u32>,
    base_build: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponsePing {}

impl ResponsePing {
    pub fn new() -> ResponsePing {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponsePing {
        static mut instance: ::protobuf::lazy::Lazy<ResponsePing> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponsePing,
        };
        unsafe {
            instance.get(ResponsePing::new)
        }
    }

    // optional string game_version = 1;

    pub fn clear_game_version(&mut self) {
        self.game_version.clear();
    }

    pub fn has_game_version(&self) -> bool {
        self.game_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_version(&mut self, v: ::std::string::String) {
        self.game_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_version(&mut self) -> &mut ::std::string::String {
        if self.game_version.is_none() {
            self.game_version.set_default();
        }
        self.game_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_version(&mut self) -> ::std::string::String {
        self.game_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_version(&self) -> &str {
        match self.game_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_version
    }

    fn mut_game_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_version
    }

    // optional string data_version = 2;

    pub fn clear_data_version(&mut self) {
        self.data_version.clear();
    }

    pub fn has_data_version(&self) -> bool {
        self.data_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_version(&mut self, v: ::std::string::String) {
        self.data_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_version(&mut self) -> &mut ::std::string::String {
        if self.data_version.is_none() {
            self.data_version.set_default();
        }
        self.data_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_data_version(&mut self) -> ::std::string::String {
        self.data_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data_version(&self) -> &str {
        match self.data_version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_data_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.data_version
    }

    fn mut_data_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.data_version
    }

    // optional uint32 data_build = 3;

    pub fn clear_data_build(&mut self) {
        self.data_build = ::std::option::Option::None;
    }

    pub fn has_data_build(&self) -> bool {
        self.data_build.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_build(&mut self, v: u32) {
        self.data_build = ::std::option::Option::Some(v);
    }

    pub fn get_data_build(&self) -> u32 {
        self.data_build.unwrap_or(0)
    }

    fn get_data_build_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.data_build
    }

    fn mut_data_build_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.data_build
    }

    // optional uint32 base_build = 4;

    pub fn clear_base_build(&mut self) {
        self.base_build = ::std::option::Option::None;
    }

    pub fn has_base_build(&self) -> bool {
        self.base_build.is_some()
    }

    // Param is passed by value, moved
    pub fn set_base_build(&mut self, v: u32) {
        self.base_build = ::std::option::Option::Some(v);
    }

    pub fn get_base_build(&self) -> u32 {
        self.base_build.unwrap_or(0)
    }

    fn get_base_build_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.base_build
    }

    fn mut_base_build_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.base_build
    }
}

impl ::protobuf::Message for ResponsePing {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_version)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data_version)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.data_build = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.base_build = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.game_version.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.data_version.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.data_build {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.base_build {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.game_version.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.data_version.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.data_build {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.base_build {
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

impl ::protobuf::MessageStatic for ResponsePing {
    fn new() -> ResponsePing {
        ResponsePing::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponsePing>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_version",
                    ResponsePing::get_game_version_for_reflect,
                    ResponsePing::mut_game_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data_version",
                    ResponsePing::get_data_version_for_reflect,
                    ResponsePing::mut_data_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "data_build",
                    ResponsePing::get_data_build_for_reflect,
                    ResponsePing::mut_data_build_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "base_build",
                    ResponsePing::get_base_build_for_reflect,
                    ResponsePing::mut_base_build_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponsePing>(
                    "ResponsePing",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponsePing {
    fn clear(&mut self) {
        self.clear_game_version();
        self.clear_data_version();
        self.clear_data_build();
        self.clear_base_build();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponsePing {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponsePing {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RequestDebug {
    // message fields
    debug: ::protobuf::RepeatedField<super::debug::DebugCommand>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestDebug {}

impl RequestDebug {
    pub fn new() -> RequestDebug {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestDebug {
        static mut instance: ::protobuf::lazy::Lazy<RequestDebug> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestDebug,
        };
        unsafe {
            instance.get(RequestDebug::new)
        }
    }

    // repeated .SC2APIProtocol.DebugCommand debug = 1;

    pub fn clear_debug(&mut self) {
        self.debug.clear();
    }

    // Param is passed by value, moved
    pub fn set_debug(&mut self, v: ::protobuf::RepeatedField<super::debug::DebugCommand>) {
        self.debug = v;
    }

    // Mutable pointer to the field.
    pub fn mut_debug(&mut self) -> &mut ::protobuf::RepeatedField<super::debug::DebugCommand> {
        &mut self.debug
    }

    // Take field
    pub fn take_debug(&mut self) -> ::protobuf::RepeatedField<super::debug::DebugCommand> {
        ::std::mem::replace(&mut self.debug, ::protobuf::RepeatedField::new())
    }

    pub fn get_debug(&self) -> &[super::debug::DebugCommand] {
        &self.debug
    }

    fn get_debug_for_reflect(&self) -> &::protobuf::RepeatedField<super::debug::DebugCommand> {
        &self.debug
    }

    fn mut_debug_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::debug::DebugCommand> {
        &mut self.debug
    }
}

impl ::protobuf::Message for RequestDebug {
    fn is_initialized(&self) -> bool {
        for v in &self.debug {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.debug)?;
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
        for value in &self.debug {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.debug {
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

impl ::protobuf::MessageStatic for RequestDebug {
    fn new() -> RequestDebug {
        RequestDebug::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestDebug>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::debug::DebugCommand>>(
                    "debug",
                    RequestDebug::get_debug_for_reflect,
                    RequestDebug::mut_debug_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestDebug>(
                    "RequestDebug",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestDebug {
    fn clear(&mut self) {
        self.clear_debug();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RequestDebug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RequestDebug {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResponseDebug {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseDebug {}

impl ResponseDebug {
    pub fn new() -> ResponseDebug {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseDebug {
        static mut instance: ::protobuf::lazy::Lazy<ResponseDebug> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseDebug,
        };
        unsafe {
            instance.get(ResponseDebug::new)
        }
    }
}

impl ::protobuf::Message for ResponseDebug {
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

impl ::protobuf::MessageStatic for ResponseDebug {
    fn new() -> ResponseDebug {
        ResponseDebug::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseDebug>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ResponseDebug>(
                    "ResponseDebug",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseDebug {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResponseDebug {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResponseDebug {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlayerSetup {
    // message fields
    field_type: ::std::option::Option<PlayerType>,
    race: ::std::option::Option<super::common::Race>,
    difficulty: ::std::option::Option<Difficulty>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerSetup {}

impl PlayerSetup {
    pub fn new() -> PlayerSetup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerSetup {
        static mut instance: ::protobuf::lazy::Lazy<PlayerSetup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerSetup,
        };
        unsafe {
            instance.get(PlayerSetup::new)
        }
    }

    // optional .SC2APIProtocol.PlayerType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: PlayerType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> PlayerType {
        self.field_type.unwrap_or(PlayerType::Participant)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<PlayerType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<PlayerType> {
        &mut self.field_type
    }

    // optional .SC2APIProtocol.Race race = 2;

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

    // optional .SC2APIProtocol.Difficulty difficulty = 3;

    pub fn clear_difficulty(&mut self) {
        self.difficulty = ::std::option::Option::None;
    }

    pub fn has_difficulty(&self) -> bool {
        self.difficulty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_difficulty(&mut self, v: Difficulty) {
        self.difficulty = ::std::option::Option::Some(v);
    }

    pub fn get_difficulty(&self) -> Difficulty {
        self.difficulty.unwrap_or(Difficulty::VeryEasy)
    }

    fn get_difficulty_for_reflect(&self) -> &::std::option::Option<Difficulty> {
        &self.difficulty
    }

    fn mut_difficulty_for_reflect(&mut self) -> &mut ::std::option::Option<Difficulty> {
        &mut self.difficulty
    }
}

impl ::protobuf::Message for PlayerSetup {
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
                    let tmp = is.read_enum()?;
                    self.race = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.difficulty = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.race {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.difficulty {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.race {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.difficulty {
            os.write_enum(3, v.value())?;
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

impl ::protobuf::MessageStatic for PlayerSetup {
    fn new() -> PlayerSetup {
        PlayerSetup::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerSetup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PlayerType>>(
                    "type",
                    PlayerSetup::get_field_type_for_reflect,
                    PlayerSetup::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::common::Race>>(
                    "race",
                    PlayerSetup::get_race_for_reflect,
                    PlayerSetup::mut_race_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Difficulty>>(
                    "difficulty",
                    PlayerSetup::get_difficulty_for_reflect,
                    PlayerSetup::mut_difficulty_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerSetup>(
                    "PlayerSetup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerSetup {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_race();
        self.clear_difficulty();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlayerSetup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerSetup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SpatialCameraSetup {
    // message fields
    width: ::std::option::Option<f32>,
    resolution: ::protobuf::SingularPtrField<super::common::Size2DI>,
    minimap_resolution: ::protobuf::SingularPtrField<super::common::Size2DI>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SpatialCameraSetup {}

impl SpatialCameraSetup {
    pub fn new() -> SpatialCameraSetup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpatialCameraSetup {
        static mut instance: ::protobuf::lazy::Lazy<SpatialCameraSetup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpatialCameraSetup,
        };
        unsafe {
            instance.get(SpatialCameraSetup::new)
        }
    }

    // optional float width = 1;

    pub fn clear_width(&mut self) {
        self.width = ::std::option::Option::None;
    }

    pub fn has_width(&self) -> bool {
        self.width.is_some()
    }

    // Param is passed by value, moved
    pub fn set_width(&mut self, v: f32) {
        self.width = ::std::option::Option::Some(v);
    }

    pub fn get_width(&self) -> f32 {
        self.width.unwrap_or(0.)
    }

    fn get_width_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.width
    }

    fn mut_width_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.width
    }

    // optional .SC2APIProtocol.Size2DI resolution = 2;

    pub fn clear_resolution(&mut self) {
        self.resolution.clear();
    }

    pub fn has_resolution(&self) -> bool {
        self.resolution.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resolution(&mut self, v: super::common::Size2DI) {
        self.resolution = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resolution(&mut self) -> &mut super::common::Size2DI {
        if self.resolution.is_none() {
            self.resolution.set_default();
        }
        self.resolution.as_mut().unwrap()
    }

    // Take field
    pub fn take_resolution(&mut self) -> super::common::Size2DI {
        self.resolution.take().unwrap_or_else(|| super::common::Size2DI::new())
    }

    pub fn get_resolution(&self) -> &super::common::Size2DI {
        self.resolution.as_ref().unwrap_or_else(|| super::common::Size2DI::default_instance())
    }

    fn get_resolution_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Size2DI> {
        &self.resolution
    }

    fn mut_resolution_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Size2DI> {
        &mut self.resolution
    }

    // optional .SC2APIProtocol.Size2DI minimap_resolution = 3;

    pub fn clear_minimap_resolution(&mut self) {
        self.minimap_resolution.clear();
    }

    pub fn has_minimap_resolution(&self) -> bool {
        self.minimap_resolution.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minimap_resolution(&mut self, v: super::common::Size2DI) {
        self.minimap_resolution = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_minimap_resolution(&mut self) -> &mut super::common::Size2DI {
        if self.minimap_resolution.is_none() {
            self.minimap_resolution.set_default();
        }
        self.minimap_resolution.as_mut().unwrap()
    }

    // Take field
    pub fn take_minimap_resolution(&mut self) -> super::common::Size2DI {
        self.minimap_resolution.take().unwrap_or_else(|| super::common::Size2DI::new())
    }

    pub fn get_minimap_resolution(&self) -> &super::common::Size2DI {
        self.minimap_resolution.as_ref().unwrap_or_else(|| super::common::Size2DI::default_instance())
    }

    fn get_minimap_resolution_for_reflect(&self) -> &::protobuf::SingularPtrField<super::common::Size2DI> {
        &self.minimap_resolution
    }

    fn mut_minimap_resolution_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::common::Size2DI> {
        &mut self.minimap_resolution
    }
}

impl ::protobuf::Message for SpatialCameraSetup {
    fn is_initialized(&self) -> bool {
        for v in &self.resolution {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.minimap_resolution {
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
                    self.width = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.resolution)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.minimap_resolution)?;
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
        if let Some(v) = self.width {
            my_size += 5;
        }
        if let Some(ref v) = self.resolution.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.minimap_resolution.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.width {
            os.write_float(1, v)?;
        }
        if let Some(ref v) = self.resolution.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.minimap_resolution.as_ref() {
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

impl ::protobuf::MessageStatic for SpatialCameraSetup {
    fn new() -> SpatialCameraSetup {
        SpatialCameraSetup::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpatialCameraSetup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "width",
                    SpatialCameraSetup::get_width_for_reflect,
                    SpatialCameraSetup::mut_width_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Size2DI>>(
                    "resolution",
                    SpatialCameraSetup::get_resolution_for_reflect,
                    SpatialCameraSetup::mut_resolution_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Size2DI>>(
                    "minimap_resolution",
                    SpatialCameraSetup::get_minimap_resolution_for_reflect,
                    SpatialCameraSetup::mut_minimap_resolution_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SpatialCameraSetup>(
                    "SpatialCameraSetup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpatialCameraSetup {
    fn clear(&mut self) {
        self.clear_width();
        self.clear_resolution();
        self.clear_minimap_resolution();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SpatialCameraSetup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SpatialCameraSetup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InterfaceOptions {
    // message fields
    raw: ::std::option::Option<bool>,
    score: ::std::option::Option<bool>,
    feature_layer: ::protobuf::SingularPtrField<SpatialCameraSetup>,
    render: ::protobuf::SingularPtrField<SpatialCameraSetup>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InterfaceOptions {}

impl InterfaceOptions {
    pub fn new() -> InterfaceOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InterfaceOptions {
        static mut instance: ::protobuf::lazy::Lazy<InterfaceOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InterfaceOptions,
        };
        unsafe {
            instance.get(InterfaceOptions::new)
        }
    }

    // optional bool raw = 1;

    pub fn clear_raw(&mut self) {
        self.raw = ::std::option::Option::None;
    }

    pub fn has_raw(&self) -> bool {
        self.raw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw(&mut self, v: bool) {
        self.raw = ::std::option::Option::Some(v);
    }

    pub fn get_raw(&self) -> bool {
        self.raw.unwrap_or(false)
    }

    fn get_raw_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.raw
    }

    fn mut_raw_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.raw
    }

    // optional bool score = 2;

    pub fn clear_score(&mut self) {
        self.score = ::std::option::Option::None;
    }

    pub fn has_score(&self) -> bool {
        self.score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: bool) {
        self.score = ::std::option::Option::Some(v);
    }

    pub fn get_score(&self) -> bool {
        self.score.unwrap_or(false)
    }

    fn get_score_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.score
    }

    fn mut_score_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.score
    }

    // optional .SC2APIProtocol.SpatialCameraSetup feature_layer = 3;

    pub fn clear_feature_layer(&mut self) {
        self.feature_layer.clear();
    }

    pub fn has_feature_layer(&self) -> bool {
        self.feature_layer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_feature_layer(&mut self, v: SpatialCameraSetup) {
        self.feature_layer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_feature_layer(&mut self) -> &mut SpatialCameraSetup {
        if self.feature_layer.is_none() {
            self.feature_layer.set_default();
        }
        self.feature_layer.as_mut().unwrap()
    }

    // Take field
    pub fn take_feature_layer(&mut self) -> SpatialCameraSetup {
        self.feature_layer.take().unwrap_or_else(|| SpatialCameraSetup::new())
    }

    pub fn get_feature_layer(&self) -> &SpatialCameraSetup {
        self.feature_layer.as_ref().unwrap_or_else(|| SpatialCameraSetup::default_instance())
    }

    fn get_feature_layer_for_reflect(&self) -> &::protobuf::SingularPtrField<SpatialCameraSetup> {
        &self.feature_layer
    }

    fn mut_feature_layer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SpatialCameraSetup> {
        &mut self.feature_layer
    }

    // optional .SC2APIProtocol.SpatialCameraSetup render = 4;

    pub fn clear_render(&mut self) {
        self.render.clear();
    }

    pub fn has_render(&self) -> bool {
        self.render.is_some()
    }

    // Param is passed by value, moved
    pub fn set_render(&mut self, v: SpatialCameraSetup) {
        self.render = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_render(&mut self) -> &mut SpatialCameraSetup {
        if self.render.is_none() {
            self.render.set_default();
        }
        self.render.as_mut().unwrap()
    }

    // Take field
    pub fn take_render(&mut self) -> SpatialCameraSetup {
        self.render.take().unwrap_or_else(|| SpatialCameraSetup::new())
    }

    pub fn get_render(&self) -> &SpatialCameraSetup {
        self.render.as_ref().unwrap_or_else(|| SpatialCameraSetup::default_instance())
    }

    fn get_render_for_reflect(&self) -> &::protobuf::SingularPtrField<SpatialCameraSetup> {
        &self.render
    }

    fn mut_render_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SpatialCameraSetup> {
        &mut self.render
    }
}

impl ::protobuf::Message for InterfaceOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.feature_layer {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.render {
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
                    let tmp = is.read_bool()?;
                    self.raw = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.score = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.feature_layer)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.render)?;
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
        if let Some(v) = self.raw {
            my_size += 2;
        }
        if let Some(v) = self.score {
            my_size += 2;
        }
        if let Some(ref v) = self.feature_layer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.render.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.raw {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.score {
            os.write_bool(2, v)?;
        }
        if let Some(ref v) = self.feature_layer.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.render.as_ref() {
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

impl ::protobuf::MessageStatic for InterfaceOptions {
    fn new() -> InterfaceOptions {
        InterfaceOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<InterfaceOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "raw",
                    InterfaceOptions::get_raw_for_reflect,
                    InterfaceOptions::mut_raw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "score",
                    InterfaceOptions::get_score_for_reflect,
                    InterfaceOptions::mut_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SpatialCameraSetup>>(
                    "feature_layer",
                    InterfaceOptions::get_feature_layer_for_reflect,
                    InterfaceOptions::mut_feature_layer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SpatialCameraSetup>>(
                    "render",
                    InterfaceOptions::get_render_for_reflect,
                    InterfaceOptions::mut_render_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InterfaceOptions>(
                    "InterfaceOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InterfaceOptions {
    fn clear(&mut self) {
        self.clear_raw();
        self.clear_score();
        self.clear_feature_layer();
        self.clear_render();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InterfaceOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InterfaceOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlayerInfo {
    // message fields
    player_id: ::std::option::Option<u32>,
    field_type: ::std::option::Option<PlayerType>,
    race_requested: ::std::option::Option<super::common::Race>,
    race_actual: ::std::option::Option<super::common::Race>,
    difficulty: ::std::option::Option<Difficulty>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerInfo {}

impl PlayerInfo {
    pub fn new() -> PlayerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerInfo {
        static mut instance: ::protobuf::lazy::Lazy<PlayerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerInfo,
        };
        unsafe {
            instance.get(PlayerInfo::new)
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_id
    }

    // optional .SC2APIProtocol.PlayerType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: PlayerType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> PlayerType {
        self.field_type.unwrap_or(PlayerType::Participant)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<PlayerType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<PlayerType> {
        &mut self.field_type
    }

    // optional .SC2APIProtocol.Race race_requested = 3;

    pub fn clear_race_requested(&mut self) {
        self.race_requested = ::std::option::Option::None;
    }

    pub fn has_race_requested(&self) -> bool {
        self.race_requested.is_some()
    }

    // Param is passed by value, moved
    pub fn set_race_requested(&mut self, v: super::common::Race) {
        self.race_requested = ::std::option::Option::Some(v);
    }

    pub fn get_race_requested(&self) -> super::common::Race {
        self.race_requested.unwrap_or(super::common::Race::NoRace)
    }

    fn get_race_requested_for_reflect(&self) -> &::std::option::Option<super::common::Race> {
        &self.race_requested
    }

    fn mut_race_requested_for_reflect(&mut self) -> &mut ::std::option::Option<super::common::Race> {
        &mut self.race_requested
    }

    // optional .SC2APIProtocol.Race race_actual = 4;

    pub fn clear_race_actual(&mut self) {
        self.race_actual = ::std::option::Option::None;
    }

    pub fn has_race_actual(&self) -> bool {
        self.race_actual.is_some()
    }

    // Param is passed by value, moved
    pub fn set_race_actual(&mut self, v: super::common::Race) {
        self.race_actual = ::std::option::Option::Some(v);
    }

    pub fn get_race_actual(&self) -> super::common::Race {
        self.race_actual.unwrap_or(super::common::Race::NoRace)
    }

    fn get_race_actual_for_reflect(&self) -> &::std::option::Option<super::common::Race> {
        &self.race_actual
    }

    fn mut_race_actual_for_reflect(&mut self) -> &mut ::std::option::Option<super::common::Race> {
        &mut self.race_actual
    }

    // optional .SC2APIProtocol.Difficulty difficulty = 5;

    pub fn clear_difficulty(&mut self) {
        self.difficulty = ::std::option::Option::None;
    }

    pub fn has_difficulty(&self) -> bool {
        self.difficulty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_difficulty(&mut self, v: Difficulty) {
        self.difficulty = ::std::option::Option::Some(v);
    }

    pub fn get_difficulty(&self) -> Difficulty {
        self.difficulty.unwrap_or(Difficulty::VeryEasy)
    }

    fn get_difficulty_for_reflect(&self) -> &::std::option::Option<Difficulty> {
        &self.difficulty
    }

    fn mut_difficulty_for_reflect(&mut self) -> &mut ::std::option::Option<Difficulty> {
        &mut self.difficulty
    }
}

impl ::protobuf::Message for PlayerInfo {
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
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.race_requested = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.race_actual = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.difficulty = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.race_requested {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.race_actual {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(v) = self.difficulty {
            my_size += ::protobuf::rt::enum_size(5, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.field_type {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.race_requested {
            os.write_enum(3, v.value())?;
        }
        if let Some(v) = self.race_actual {
            os.write_enum(4, v.value())?;
        }
        if let Some(v) = self.difficulty {
            os.write_enum(5, v.value())?;
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

impl ::protobuf::MessageStatic for PlayerInfo {
    fn new() -> PlayerInfo {
        PlayerInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_id",
                    PlayerInfo::get_player_id_for_reflect,
                    PlayerInfo::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PlayerType>>(
                    "type",
                    PlayerInfo::get_field_type_for_reflect,
                    PlayerInfo::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::common::Race>>(
                    "race_requested",
                    PlayerInfo::get_race_requested_for_reflect,
                    PlayerInfo::mut_race_requested_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::common::Race>>(
                    "race_actual",
                    PlayerInfo::get_race_actual_for_reflect,
                    PlayerInfo::mut_race_actual_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Difficulty>>(
                    "difficulty",
                    PlayerInfo::get_difficulty_for_reflect,
                    PlayerInfo::mut_difficulty_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerInfo>(
                    "PlayerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerInfo {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_field_type();
        self.clear_race_requested();
        self.clear_race_actual();
        self.clear_difficulty();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlayerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlayerCommon {
    // message fields
    player_id: ::std::option::Option<u32>,
    minerals: ::std::option::Option<u32>,
    vespene: ::std::option::Option<u32>,
    food_cap: ::std::option::Option<u32>,
    food_used: ::std::option::Option<u32>,
    food_army: ::std::option::Option<u32>,
    food_workers: ::std::option::Option<u32>,
    idle_worker_count: ::std::option::Option<u32>,
    army_count: ::std::option::Option<u32>,
    warp_gate_count: ::std::option::Option<u32>,
    larva_count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerCommon {}

impl PlayerCommon {
    pub fn new() -> PlayerCommon {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerCommon {
        static mut instance: ::protobuf::lazy::Lazy<PlayerCommon> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerCommon,
        };
        unsafe {
            instance.get(PlayerCommon::new)
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_id
    }

    // optional uint32 minerals = 2;

    pub fn clear_minerals(&mut self) {
        self.minerals = ::std::option::Option::None;
    }

    pub fn has_minerals(&self) -> bool {
        self.minerals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minerals(&mut self, v: u32) {
        self.minerals = ::std::option::Option::Some(v);
    }

    pub fn get_minerals(&self) -> u32 {
        self.minerals.unwrap_or(0)
    }

    fn get_minerals_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.minerals
    }

    fn mut_minerals_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.minerals
    }

    // optional uint32 vespene = 3;

    pub fn clear_vespene(&mut self) {
        self.vespene = ::std::option::Option::None;
    }

    pub fn has_vespene(&self) -> bool {
        self.vespene.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vespene(&mut self, v: u32) {
        self.vespene = ::std::option::Option::Some(v);
    }

    pub fn get_vespene(&self) -> u32 {
        self.vespene.unwrap_or(0)
    }

    fn get_vespene_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.vespene
    }

    fn mut_vespene_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.vespene
    }

    // optional uint32 food_cap = 4;

    pub fn clear_food_cap(&mut self) {
        self.food_cap = ::std::option::Option::None;
    }

    pub fn has_food_cap(&self) -> bool {
        self.food_cap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_food_cap(&mut self, v: u32) {
        self.food_cap = ::std::option::Option::Some(v);
    }

    pub fn get_food_cap(&self) -> u32 {
        self.food_cap.unwrap_or(0)
    }

    fn get_food_cap_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.food_cap
    }

    fn mut_food_cap_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.food_cap
    }

    // optional uint32 food_used = 5;

    pub fn clear_food_used(&mut self) {
        self.food_used = ::std::option::Option::None;
    }

    pub fn has_food_used(&self) -> bool {
        self.food_used.is_some()
    }

    // Param is passed by value, moved
    pub fn set_food_used(&mut self, v: u32) {
        self.food_used = ::std::option::Option::Some(v);
    }

    pub fn get_food_used(&self) -> u32 {
        self.food_used.unwrap_or(0)
    }

    fn get_food_used_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.food_used
    }

    fn mut_food_used_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.food_used
    }

    // optional uint32 food_army = 6;

    pub fn clear_food_army(&mut self) {
        self.food_army = ::std::option::Option::None;
    }

    pub fn has_food_army(&self) -> bool {
        self.food_army.is_some()
    }

    // Param is passed by value, moved
    pub fn set_food_army(&mut self, v: u32) {
        self.food_army = ::std::option::Option::Some(v);
    }

    pub fn get_food_army(&self) -> u32 {
        self.food_army.unwrap_or(0)
    }

    fn get_food_army_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.food_army
    }

    fn mut_food_army_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.food_army
    }

    // optional uint32 food_workers = 7;

    pub fn clear_food_workers(&mut self) {
        self.food_workers = ::std::option::Option::None;
    }

    pub fn has_food_workers(&self) -> bool {
        self.food_workers.is_some()
    }

    // Param is passed by value, moved
    pub fn set_food_workers(&mut self, v: u32) {
        self.food_workers = ::std::option::Option::Some(v);
    }

    pub fn get_food_workers(&self) -> u32 {
        self.food_workers.unwrap_or(0)
    }

    fn get_food_workers_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.food_workers
    }

    fn mut_food_workers_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.food_workers
    }

    // optional uint32 idle_worker_count = 8;

    pub fn clear_idle_worker_count(&mut self) {
        self.idle_worker_count = ::std::option::Option::None;
    }

    pub fn has_idle_worker_count(&self) -> bool {
        self.idle_worker_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_idle_worker_count(&mut self, v: u32) {
        self.idle_worker_count = ::std::option::Option::Some(v);
    }

    pub fn get_idle_worker_count(&self) -> u32 {
        self.idle_worker_count.unwrap_or(0)
    }

    fn get_idle_worker_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.idle_worker_count
    }

    fn mut_idle_worker_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.idle_worker_count
    }

    // optional uint32 army_count = 9;

    pub fn clear_army_count(&mut self) {
        self.army_count = ::std::option::Option::None;
    }

    pub fn has_army_count(&self) -> bool {
        self.army_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_army_count(&mut self, v: u32) {
        self.army_count = ::std::option::Option::Some(v);
    }

    pub fn get_army_count(&self) -> u32 {
        self.army_count.unwrap_or(0)
    }

    fn get_army_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.army_count
    }

    fn mut_army_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.army_count
    }

    // optional uint32 warp_gate_count = 10;

    pub fn clear_warp_gate_count(&mut self) {
        self.warp_gate_count = ::std::option::Option::None;
    }

    pub fn has_warp_gate_count(&self) -> bool {
        self.warp_gate_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_warp_gate_count(&mut self, v: u32) {
        self.warp_gate_count = ::std::option::Option::Some(v);
    }

    pub fn get_warp_gate_count(&self) -> u32 {
        self.warp_gate_count.unwrap_or(0)
    }

    fn get_warp_gate_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.warp_gate_count
    }

    fn mut_warp_gate_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.warp_gate_count
    }

    // optional uint32 larva_count = 11;

    pub fn clear_larva_count(&mut self) {
        self.larva_count = ::std::option::Option::None;
    }

    pub fn has_larva_count(&self) -> bool {
        self.larva_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_larva_count(&mut self, v: u32) {
        self.larva_count = ::std::option::Option::Some(v);
    }

    pub fn get_larva_count(&self) -> u32 {
        self.larva_count.unwrap_or(0)
    }

    fn get_larva_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.larva_count
    }

    fn mut_larva_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.larva_count
    }
}

impl ::protobuf::Message for PlayerCommon {
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
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.minerals = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.vespene = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.food_cap = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.food_used = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.food_army = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.food_workers = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.idle_worker_count = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.army_count = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.warp_gate_count = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.larva_count = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.minerals {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.vespene {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.food_cap {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.food_used {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.food_army {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.food_workers {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.idle_worker_count {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.army_count {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.warp_gate_count {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.larva_count {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.minerals {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.vespene {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.food_cap {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.food_used {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.food_army {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.food_workers {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.idle_worker_count {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.army_count {
            os.write_uint32(9, v)?;
        }
        if let Some(v) = self.warp_gate_count {
            os.write_uint32(10, v)?;
        }
        if let Some(v) = self.larva_count {
            os.write_uint32(11, v)?;
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

impl ::protobuf::MessageStatic for PlayerCommon {
    fn new() -> PlayerCommon {
        PlayerCommon::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerCommon>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_id",
                    PlayerCommon::get_player_id_for_reflect,
                    PlayerCommon::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "minerals",
                    PlayerCommon::get_minerals_for_reflect,
                    PlayerCommon::mut_minerals_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "vespene",
                    PlayerCommon::get_vespene_for_reflect,
                    PlayerCommon::mut_vespene_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "food_cap",
                    PlayerCommon::get_food_cap_for_reflect,
                    PlayerCommon::mut_food_cap_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "food_used",
                    PlayerCommon::get_food_used_for_reflect,
                    PlayerCommon::mut_food_used_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "food_army",
                    PlayerCommon::get_food_army_for_reflect,
                    PlayerCommon::mut_food_army_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "food_workers",
                    PlayerCommon::get_food_workers_for_reflect,
                    PlayerCommon::mut_food_workers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "idle_worker_count",
                    PlayerCommon::get_idle_worker_count_for_reflect,
                    PlayerCommon::mut_idle_worker_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "army_count",
                    PlayerCommon::get_army_count_for_reflect,
                    PlayerCommon::mut_army_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "warp_gate_count",
                    PlayerCommon::get_warp_gate_count_for_reflect,
                    PlayerCommon::mut_warp_gate_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "larva_count",
                    PlayerCommon::get_larva_count_for_reflect,
                    PlayerCommon::mut_larva_count_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerCommon>(
                    "PlayerCommon",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerCommon {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_minerals();
        self.clear_vespene();
        self.clear_food_cap();
        self.clear_food_used();
        self.clear_food_army();
        self.clear_food_workers();
        self.clear_idle_worker_count();
        self.clear_army_count();
        self.clear_warp_gate_count();
        self.clear_larva_count();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlayerCommon {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerCommon {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Observation {
    // message fields
    game_loop: ::std::option::Option<u32>,
    player_common: ::protobuf::SingularPtrField<PlayerCommon>,
    alerts: ::std::vec::Vec<Alert>,
    abilities: ::protobuf::RepeatedField<super::common::AvailableAbility>,
    score: ::protobuf::SingularPtrField<super::score::Score>,
    raw_data: ::protobuf::SingularPtrField<super::raw::ObservationRaw>,
    feature_layer_data: ::protobuf::SingularPtrField<super::spatial::ObservationFeatureLayer>,
    render_data: ::protobuf::SingularPtrField<super::spatial::ObservationRender>,
    ui_data: ::protobuf::SingularPtrField<super::ui::ObservationUI>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Observation {}

impl Observation {
    pub fn new() -> Observation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Observation {
        static mut instance: ::protobuf::lazy::Lazy<Observation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Observation,
        };
        unsafe {
            instance.get(Observation::new)
        }
    }

    // optional uint32 game_loop = 9;

    pub fn clear_game_loop(&mut self) {
        self.game_loop = ::std::option::Option::None;
    }

    pub fn has_game_loop(&self) -> bool {
        self.game_loop.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_loop(&mut self, v: u32) {
        self.game_loop = ::std::option::Option::Some(v);
    }

    pub fn get_game_loop(&self) -> u32 {
        self.game_loop.unwrap_or(0)
    }

    fn get_game_loop_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_loop
    }

    fn mut_game_loop_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_loop
    }

    // optional .SC2APIProtocol.PlayerCommon player_common = 1;

    pub fn clear_player_common(&mut self) {
        self.player_common.clear();
    }

    pub fn has_player_common(&self) -> bool {
        self.player_common.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_common(&mut self, v: PlayerCommon) {
        self.player_common = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_player_common(&mut self) -> &mut PlayerCommon {
        if self.player_common.is_none() {
            self.player_common.set_default();
        }
        self.player_common.as_mut().unwrap()
    }

    // Take field
    pub fn take_player_common(&mut self) -> PlayerCommon {
        self.player_common.take().unwrap_or_else(|| PlayerCommon::new())
    }

    pub fn get_player_common(&self) -> &PlayerCommon {
        self.player_common.as_ref().unwrap_or_else(|| PlayerCommon::default_instance())
    }

    fn get_player_common_for_reflect(&self) -> &::protobuf::SingularPtrField<PlayerCommon> {
        &self.player_common
    }

    fn mut_player_common_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PlayerCommon> {
        &mut self.player_common
    }

    // repeated .SC2APIProtocol.Alert alerts = 10;

    pub fn clear_alerts(&mut self) {
        self.alerts.clear();
    }

    // Param is passed by value, moved
    pub fn set_alerts(&mut self, v: ::std::vec::Vec<Alert>) {
        self.alerts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_alerts(&mut self) -> &mut ::std::vec::Vec<Alert> {
        &mut self.alerts
    }

    // Take field
    pub fn take_alerts(&mut self) -> ::std::vec::Vec<Alert> {
        ::std::mem::replace(&mut self.alerts, ::std::vec::Vec::new())
    }

    pub fn get_alerts(&self) -> &[Alert] {
        &self.alerts
    }

    fn get_alerts_for_reflect(&self) -> &::std::vec::Vec<Alert> {
        &self.alerts
    }

    fn mut_alerts_for_reflect(&mut self) -> &mut ::std::vec::Vec<Alert> {
        &mut self.alerts
    }

    // repeated .SC2APIProtocol.AvailableAbility abilities = 3;

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

    // optional .SC2APIProtocol.Score score = 4;

    pub fn clear_score(&mut self) {
        self.score.clear();
    }

    pub fn has_score(&self) -> bool {
        self.score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_score(&mut self, v: super::score::Score) {
        self.score = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_score(&mut self) -> &mut super::score::Score {
        if self.score.is_none() {
            self.score.set_default();
        }
        self.score.as_mut().unwrap()
    }

    // Take field
    pub fn take_score(&mut self) -> super::score::Score {
        self.score.take().unwrap_or_else(|| super::score::Score::new())
    }

    pub fn get_score(&self) -> &super::score::Score {
        self.score.as_ref().unwrap_or_else(|| super::score::Score::default_instance())
    }

    fn get_score_for_reflect(&self) -> &::protobuf::SingularPtrField<super::score::Score> {
        &self.score
    }

    fn mut_score_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::score::Score> {
        &mut self.score
    }

    // optional .SC2APIProtocol.ObservationRaw raw_data = 5;

    pub fn clear_raw_data(&mut self) {
        self.raw_data.clear();
    }

    pub fn has_raw_data(&self) -> bool {
        self.raw_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raw_data(&mut self, v: super::raw::ObservationRaw) {
        self.raw_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raw_data(&mut self) -> &mut super::raw::ObservationRaw {
        if self.raw_data.is_none() {
            self.raw_data.set_default();
        }
        self.raw_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_raw_data(&mut self) -> super::raw::ObservationRaw {
        self.raw_data.take().unwrap_or_else(|| super::raw::ObservationRaw::new())
    }

    pub fn get_raw_data(&self) -> &super::raw::ObservationRaw {
        self.raw_data.as_ref().unwrap_or_else(|| super::raw::ObservationRaw::default_instance())
    }

    fn get_raw_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::raw::ObservationRaw> {
        &self.raw_data
    }

    fn mut_raw_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::raw::ObservationRaw> {
        &mut self.raw_data
    }

    // optional .SC2APIProtocol.ObservationFeatureLayer feature_layer_data = 6;

    pub fn clear_feature_layer_data(&mut self) {
        self.feature_layer_data.clear();
    }

    pub fn has_feature_layer_data(&self) -> bool {
        self.feature_layer_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_feature_layer_data(&mut self, v: super::spatial::ObservationFeatureLayer) {
        self.feature_layer_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_feature_layer_data(&mut self) -> &mut super::spatial::ObservationFeatureLayer {
        if self.feature_layer_data.is_none() {
            self.feature_layer_data.set_default();
        }
        self.feature_layer_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_feature_layer_data(&mut self) -> super::spatial::ObservationFeatureLayer {
        self.feature_layer_data.take().unwrap_or_else(|| super::spatial::ObservationFeatureLayer::new())
    }

    pub fn get_feature_layer_data(&self) -> &super::spatial::ObservationFeatureLayer {
        self.feature_layer_data.as_ref().unwrap_or_else(|| super::spatial::ObservationFeatureLayer::default_instance())
    }

    fn get_feature_layer_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::spatial::ObservationFeatureLayer> {
        &self.feature_layer_data
    }

    fn mut_feature_layer_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::spatial::ObservationFeatureLayer> {
        &mut self.feature_layer_data
    }

    // optional .SC2APIProtocol.ObservationRender render_data = 7;

    pub fn clear_render_data(&mut self) {
        self.render_data.clear();
    }

    pub fn has_render_data(&self) -> bool {
        self.render_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_render_data(&mut self, v: super::spatial::ObservationRender) {
        self.render_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_render_data(&mut self) -> &mut super::spatial::ObservationRender {
        if self.render_data.is_none() {
            self.render_data.set_default();
        }
        self.render_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_render_data(&mut self) -> super::spatial::ObservationRender {
        self.render_data.take().unwrap_or_else(|| super::spatial::ObservationRender::new())
    }

    pub fn get_render_data(&self) -> &super::spatial::ObservationRender {
        self.render_data.as_ref().unwrap_or_else(|| super::spatial::ObservationRender::default_instance())
    }

    fn get_render_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::spatial::ObservationRender> {
        &self.render_data
    }

    fn mut_render_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::spatial::ObservationRender> {
        &mut self.render_data
    }

    // optional .SC2APIProtocol.ObservationUI ui_data = 8;

    pub fn clear_ui_data(&mut self) {
        self.ui_data.clear();
    }

    pub fn has_ui_data(&self) -> bool {
        self.ui_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ui_data(&mut self, v: super::ui::ObservationUI) {
        self.ui_data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ui_data(&mut self) -> &mut super::ui::ObservationUI {
        if self.ui_data.is_none() {
            self.ui_data.set_default();
        }
        self.ui_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_ui_data(&mut self) -> super::ui::ObservationUI {
        self.ui_data.take().unwrap_or_else(|| super::ui::ObservationUI::new())
    }

    pub fn get_ui_data(&self) -> &super::ui::ObservationUI {
        self.ui_data.as_ref().unwrap_or_else(|| super::ui::ObservationUI::default_instance())
    }

    fn get_ui_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::ui::ObservationUI> {
        &self.ui_data
    }

    fn mut_ui_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::ui::ObservationUI> {
        &mut self.ui_data
    }
}

impl ::protobuf::Message for Observation {
    fn is_initialized(&self) -> bool {
        for v in &self.player_common {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.abilities {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.score {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.raw_data {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.feature_layer_data {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.render_data {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ui_data {
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
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.game_loop = ::std::option::Option::Some(tmp);
                },
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.player_common)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.alerts)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.abilities)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.score)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raw_data)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.feature_layer_data)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.render_data)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ui_data)?;
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
        if let Some(v) = self.game_loop {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.player_common.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.alerts {
            my_size += ::protobuf::rt::enum_size(10, *value);
        };
        for value in &self.abilities {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.score.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.raw_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.feature_layer_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.render_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.ui_data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.game_loop {
            os.write_uint32(9, v)?;
        }
        if let Some(ref v) = self.player_common.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.alerts {
            os.write_enum(10, v.value())?;
        };
        for v in &self.abilities {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.score.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.raw_data.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.feature_layer_data.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.render_data.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.ui_data.as_ref() {
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

impl ::protobuf::MessageStatic for Observation {
    fn new() -> Observation {
        Observation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Observation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_loop",
                    Observation::get_game_loop_for_reflect,
                    Observation::mut_game_loop_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PlayerCommon>>(
                    "player_common",
                    Observation::get_player_common_for_reflect,
                    Observation::mut_player_common_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Alert>>(
                    "alerts",
                    Observation::get_alerts_for_reflect,
                    Observation::mut_alerts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::AvailableAbility>>(
                    "abilities",
                    Observation::get_abilities_for_reflect,
                    Observation::mut_abilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::score::Score>>(
                    "score",
                    Observation::get_score_for_reflect,
                    Observation::mut_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::raw::ObservationRaw>>(
                    "raw_data",
                    Observation::get_raw_data_for_reflect,
                    Observation::mut_raw_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::spatial::ObservationFeatureLayer>>(
                    "feature_layer_data",
                    Observation::get_feature_layer_data_for_reflect,
                    Observation::mut_feature_layer_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::spatial::ObservationRender>>(
                    "render_data",
                    Observation::get_render_data_for_reflect,
                    Observation::mut_render_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::ui::ObservationUI>>(
                    "ui_data",
                    Observation::get_ui_data_for_reflect,
                    Observation::mut_ui_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Observation>(
                    "Observation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Observation {
    fn clear(&mut self) {
        self.clear_game_loop();
        self.clear_player_common();
        self.clear_alerts();
        self.clear_abilities();
        self.clear_score();
        self.clear_raw_data();
        self.clear_feature_layer_data();
        self.clear_render_data();
        self.clear_ui_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Observation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Observation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Action {
    // message fields
    action_raw: ::protobuf::SingularPtrField<super::raw::ActionRaw>,
    action_feature_layer: ::protobuf::SingularPtrField<super::spatial::ActionSpatial>,
    action_render: ::protobuf::SingularPtrField<super::spatial::ActionSpatial>,
    action_ui: ::protobuf::SingularPtrField<super::ui::ActionUI>,
    chat: ::protobuf::RepeatedField<ActionChat>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Action {}

impl Action {
    pub fn new() -> Action {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Action {
        static mut instance: ::protobuf::lazy::Lazy<Action> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Action,
        };
        unsafe {
            instance.get(Action::new)
        }
    }

    // optional .SC2APIProtocol.ActionRaw action_raw = 1;

    pub fn clear_action_raw(&mut self) {
        self.action_raw.clear();
    }

    pub fn has_action_raw(&self) -> bool {
        self.action_raw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_raw(&mut self, v: super::raw::ActionRaw) {
        self.action_raw = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action_raw(&mut self) -> &mut super::raw::ActionRaw {
        if self.action_raw.is_none() {
            self.action_raw.set_default();
        }
        self.action_raw.as_mut().unwrap()
    }

    // Take field
    pub fn take_action_raw(&mut self) -> super::raw::ActionRaw {
        self.action_raw.take().unwrap_or_else(|| super::raw::ActionRaw::new())
    }

    pub fn get_action_raw(&self) -> &super::raw::ActionRaw {
        self.action_raw.as_ref().unwrap_or_else(|| super::raw::ActionRaw::default_instance())
    }

    fn get_action_raw_for_reflect(&self) -> &::protobuf::SingularPtrField<super::raw::ActionRaw> {
        &self.action_raw
    }

    fn mut_action_raw_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::raw::ActionRaw> {
        &mut self.action_raw
    }

    // optional .SC2APIProtocol.ActionSpatial action_feature_layer = 2;

    pub fn clear_action_feature_layer(&mut self) {
        self.action_feature_layer.clear();
    }

    pub fn has_action_feature_layer(&self) -> bool {
        self.action_feature_layer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_feature_layer(&mut self, v: super::spatial::ActionSpatial) {
        self.action_feature_layer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action_feature_layer(&mut self) -> &mut super::spatial::ActionSpatial {
        if self.action_feature_layer.is_none() {
            self.action_feature_layer.set_default();
        }
        self.action_feature_layer.as_mut().unwrap()
    }

    // Take field
    pub fn take_action_feature_layer(&mut self) -> super::spatial::ActionSpatial {
        self.action_feature_layer.take().unwrap_or_else(|| super::spatial::ActionSpatial::new())
    }

    pub fn get_action_feature_layer(&self) -> &super::spatial::ActionSpatial {
        self.action_feature_layer.as_ref().unwrap_or_else(|| super::spatial::ActionSpatial::default_instance())
    }

    fn get_action_feature_layer_for_reflect(&self) -> &::protobuf::SingularPtrField<super::spatial::ActionSpatial> {
        &self.action_feature_layer
    }

    fn mut_action_feature_layer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::spatial::ActionSpatial> {
        &mut self.action_feature_layer
    }

    // optional .SC2APIProtocol.ActionSpatial action_render = 3;

    pub fn clear_action_render(&mut self) {
        self.action_render.clear();
    }

    pub fn has_action_render(&self) -> bool {
        self.action_render.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_render(&mut self, v: super::spatial::ActionSpatial) {
        self.action_render = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action_render(&mut self) -> &mut super::spatial::ActionSpatial {
        if self.action_render.is_none() {
            self.action_render.set_default();
        }
        self.action_render.as_mut().unwrap()
    }

    // Take field
    pub fn take_action_render(&mut self) -> super::spatial::ActionSpatial {
        self.action_render.take().unwrap_or_else(|| super::spatial::ActionSpatial::new())
    }

    pub fn get_action_render(&self) -> &super::spatial::ActionSpatial {
        self.action_render.as_ref().unwrap_or_else(|| super::spatial::ActionSpatial::default_instance())
    }

    fn get_action_render_for_reflect(&self) -> &::protobuf::SingularPtrField<super::spatial::ActionSpatial> {
        &self.action_render
    }

    fn mut_action_render_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::spatial::ActionSpatial> {
        &mut self.action_render
    }

    // optional .SC2APIProtocol.ActionUI action_ui = 4;

    pub fn clear_action_ui(&mut self) {
        self.action_ui.clear();
    }

    pub fn has_action_ui(&self) -> bool {
        self.action_ui.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action_ui(&mut self, v: super::ui::ActionUI) {
        self.action_ui = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_action_ui(&mut self) -> &mut super::ui::ActionUI {
        if self.action_ui.is_none() {
            self.action_ui.set_default();
        }
        self.action_ui.as_mut().unwrap()
    }

    // Take field
    pub fn take_action_ui(&mut self) -> super::ui::ActionUI {
        self.action_ui.take().unwrap_or_else(|| super::ui::ActionUI::new())
    }

    pub fn get_action_ui(&self) -> &super::ui::ActionUI {
        self.action_ui.as_ref().unwrap_or_else(|| super::ui::ActionUI::default_instance())
    }

    fn get_action_ui_for_reflect(&self) -> &::protobuf::SingularPtrField<super::ui::ActionUI> {
        &self.action_ui
    }

    fn mut_action_ui_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::ui::ActionUI> {
        &mut self.action_ui
    }

    // repeated .SC2APIProtocol.ActionChat chat = 5;

    pub fn clear_chat(&mut self) {
        self.chat.clear();
    }

    // Param is passed by value, moved
    pub fn set_chat(&mut self, v: ::protobuf::RepeatedField<ActionChat>) {
        self.chat = v;
    }

    // Mutable pointer to the field.
    pub fn mut_chat(&mut self) -> &mut ::protobuf::RepeatedField<ActionChat> {
        &mut self.chat
    }

    // Take field
    pub fn take_chat(&mut self) -> ::protobuf::RepeatedField<ActionChat> {
        ::std::mem::replace(&mut self.chat, ::protobuf::RepeatedField::new())
    }

    pub fn get_chat(&self) -> &[ActionChat] {
        &self.chat
    }

    fn get_chat_for_reflect(&self) -> &::protobuf::RepeatedField<ActionChat> {
        &self.chat
    }

    fn mut_chat_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ActionChat> {
        &mut self.chat
    }
}

impl ::protobuf::Message for Action {
    fn is_initialized(&self) -> bool {
        for v in &self.action_raw {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.action_feature_layer {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.action_render {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.action_ui {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.chat {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.action_raw)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.action_feature_layer)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.action_render)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.action_ui)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.chat)?;
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
        if let Some(ref v) = self.action_raw.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.action_feature_layer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.action_render.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.action_ui.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.chat {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.action_raw.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.action_feature_layer.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.action_render.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.action_ui.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.chat {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Action {
    fn new() -> Action {
        Action::new()
    }

    fn descriptor_static(_: ::std::option::Option<Action>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::raw::ActionRaw>>(
                    "action_raw",
                    Action::get_action_raw_for_reflect,
                    Action::mut_action_raw_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::spatial::ActionSpatial>>(
                    "action_feature_layer",
                    Action::get_action_feature_layer_for_reflect,
                    Action::mut_action_feature_layer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::spatial::ActionSpatial>>(
                    "action_render",
                    Action::get_action_render_for_reflect,
                    Action::mut_action_render_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::ui::ActionUI>>(
                    "action_ui",
                    Action::get_action_ui_for_reflect,
                    Action::mut_action_ui_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ActionChat>>(
                    "chat",
                    Action::get_chat_for_reflect,
                    Action::mut_chat_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Action>(
                    "Action",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Action {
    fn clear(&mut self) {
        self.clear_action_raw();
        self.clear_action_feature_layer();
        self.clear_action_render();
        self.clear_action_ui();
        self.clear_chat();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Action {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Action {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionChat {
    // message fields
    channel: ::std::option::Option<ActionChat_Channel>,
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionChat {}

impl ActionChat {
    pub fn new() -> ActionChat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionChat {
        static mut instance: ::protobuf::lazy::Lazy<ActionChat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionChat,
        };
        unsafe {
            instance.get(ActionChat::new)
        }
    }

    // optional .SC2APIProtocol.ActionChat.Channel channel = 1;

    pub fn clear_channel(&mut self) {
        self.channel = ::std::option::Option::None;
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: ActionChat_Channel) {
        self.channel = ::std::option::Option::Some(v);
    }

    pub fn get_channel(&self) -> ActionChat_Channel {
        self.channel.unwrap_or(ActionChat_Channel::Broadcast)
    }

    fn get_channel_for_reflect(&self) -> &::std::option::Option<ActionChat_Channel> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::std::option::Option<ActionChat_Channel> {
        &mut self.channel
    }

    // optional string message = 2;

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

impl ::protobuf::Message for ActionChat {
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
                    self.channel = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        if let Some(v) = self.channel {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.message.as_ref() {
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

impl ::protobuf::MessageStatic for ActionChat {
    fn new() -> ActionChat {
        ActionChat::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionChat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ActionChat_Channel>>(
                    "channel",
                    ActionChat::get_channel_for_reflect,
                    ActionChat::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    ActionChat::get_message_for_reflect,
                    ActionChat::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionChat>(
                    "ActionChat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionChat {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionChat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionChat {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ActionChat_Channel {
    Broadcast = 1,
    Team = 2,
}

impl ::protobuf::ProtobufEnum for ActionChat_Channel {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ActionChat_Channel> {
        match value {
            1 => ::std::option::Option::Some(ActionChat_Channel::Broadcast),
            2 => ::std::option::Option::Some(ActionChat_Channel::Team),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ActionChat_Channel] = &[
            ActionChat_Channel::Broadcast,
            ActionChat_Channel::Team,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ActionChat_Channel>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ActionChat_Channel", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ActionChat_Channel {
}

impl ::protobuf::reflect::ProtobufValue for ActionChat_Channel {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ActionError {
    // message fields
    unit_tag: ::std::option::Option<u64>,
    ability_id: ::std::option::Option<u64>,
    result: ::std::option::Option<super::error::ActionResult>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionError {}

impl ActionError {
    pub fn new() -> ActionError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionError {
        static mut instance: ::protobuf::lazy::Lazy<ActionError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionError,
        };
        unsafe {
            instance.get(ActionError::new)
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

    // optional uint64 ability_id = 2;

    pub fn clear_ability_id(&mut self) {
        self.ability_id = ::std::option::Option::None;
    }

    pub fn has_ability_id(&self) -> bool {
        self.ability_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ability_id(&mut self, v: u64) {
        self.ability_id = ::std::option::Option::Some(v);
    }

    pub fn get_ability_id(&self) -> u64 {
        self.ability_id.unwrap_or(0)
    }

    fn get_ability_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ability_id
    }

    fn mut_ability_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ability_id
    }

    // optional .SC2APIProtocol.ActionResult result = 3;

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

impl ::protobuf::Message for ActionError {
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ability_id = ::std::option::Option::Some(tmp);
                },
                3 => {
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
        if let Some(v) = self.unit_tag {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ability_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.unit_tag {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.ability_id {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.result {
            os.write_enum(3, v.value())?;
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

impl ::protobuf::MessageStatic for ActionError {
    fn new() -> ActionError {
        ActionError::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "unit_tag",
                    ActionError::get_unit_tag_for_reflect,
                    ActionError::mut_unit_tag_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ability_id",
                    ActionError::get_ability_id_for_reflect,
                    ActionError::mut_ability_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::error::ActionResult>>(
                    "result",
                    ActionError::get_result_for_reflect,
                    ActionError::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionError>(
                    "ActionError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionError {
    fn clear(&mut self) {
        self.clear_unit_tag();
        self.clear_ability_id();
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionError {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PlayerResult {
    // message fields
    player_id: ::std::option::Option<u32>,
    result: ::std::option::Option<Result>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PlayerResult {}

impl PlayerResult {
    pub fn new() -> PlayerResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PlayerResult {
        static mut instance: ::protobuf::lazy::Lazy<PlayerResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PlayerResult,
        };
        unsafe {
            instance.get(PlayerResult::new)
        }
    }

    // optional uint32 player_id = 1;

    pub fn clear_player_id(&mut self) {
        self.player_id = ::std::option::Option::None;
    }

    pub fn has_player_id(&self) -> bool {
        self.player_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_id(&mut self, v: u32) {
        self.player_id = ::std::option::Option::Some(v);
    }

    pub fn get_player_id(&self) -> u32 {
        self.player_id.unwrap_or(0)
    }

    fn get_player_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.player_id
    }

    fn mut_player_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.player_id
    }

    // optional .SC2APIProtocol.Result result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: Result) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> Result {
        self.result.unwrap_or(Result::Victory)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<Result> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<Result> {
        &mut self.result
    }
}

impl ::protobuf::Message for PlayerResult {
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
                    self.player_id = ::std::option::Option::Some(tmp);
                },
                2 => {
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
        if let Some(v) = self.player_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.player_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.result {
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

impl ::protobuf::MessageStatic for PlayerResult {
    fn new() -> PlayerResult {
        PlayerResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<PlayerResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "player_id",
                    PlayerResult::get_player_id_for_reflect,
                    PlayerResult::mut_player_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Result>>(
                    "result",
                    PlayerResult::get_result_for_reflect,
                    PlayerResult::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PlayerResult>(
                    "PlayerResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PlayerResult {
    fn clear(&mut self) {
        self.clear_player_id();
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PlayerResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Status {
    launched = 1,
    init_game = 2,
    in_game = 3,
    in_replay = 4,
    ended = 5,
    quit = 6,
    unknown = 99,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            1 => ::std::option::Option::Some(Status::launched),
            2 => ::std::option::Option::Some(Status::init_game),
            3 => ::std::option::Option::Some(Status::in_game),
            4 => ::std::option::Option::Some(Status::in_replay),
            5 => ::std::option::Option::Some(Status::ended),
            6 => ::std::option::Option::Some(Status::quit),
            99 => ::std::option::Option::Some(Status::unknown),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Status] = &[
            Status::launched,
            Status::init_game,
            Status::in_game,
            Status::in_replay,
            Status::ended,
            Status::quit,
            Status::unknown,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Status {
}

impl ::protobuf::reflect::ProtobufValue for Status {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Difficulty {
    VeryEasy = 1,
    Easy = 2,
    Medium = 3,
    MediumHard = 4,
    Hard = 5,
    Harder = 6,
    VeryHard = 7,
    CheatVision = 8,
    CheatMoney = 9,
    CheatInsane = 10,
}

impl ::protobuf::ProtobufEnum for Difficulty {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Difficulty> {
        match value {
            1 => ::std::option::Option::Some(Difficulty::VeryEasy),
            2 => ::std::option::Option::Some(Difficulty::Easy),
            3 => ::std::option::Option::Some(Difficulty::Medium),
            4 => ::std::option::Option::Some(Difficulty::MediumHard),
            5 => ::std::option::Option::Some(Difficulty::Hard),
            6 => ::std::option::Option::Some(Difficulty::Harder),
            7 => ::std::option::Option::Some(Difficulty::VeryHard),
            8 => ::std::option::Option::Some(Difficulty::CheatVision),
            9 => ::std::option::Option::Some(Difficulty::CheatMoney),
            10 => ::std::option::Option::Some(Difficulty::CheatInsane),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Difficulty] = &[
            Difficulty::VeryEasy,
            Difficulty::Easy,
            Difficulty::Medium,
            Difficulty::MediumHard,
            Difficulty::Hard,
            Difficulty::Harder,
            Difficulty::VeryHard,
            Difficulty::CheatVision,
            Difficulty::CheatMoney,
            Difficulty::CheatInsane,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Difficulty>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Difficulty", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Difficulty {
}

impl ::protobuf::reflect::ProtobufValue for Difficulty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PlayerType {
    Participant = 1,
    Computer = 2,
    Observer = 3,
}

impl ::protobuf::ProtobufEnum for PlayerType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PlayerType> {
        match value {
            1 => ::std::option::Option::Some(PlayerType::Participant),
            2 => ::std::option::Option::Some(PlayerType::Computer),
            3 => ::std::option::Option::Some(PlayerType::Observer),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PlayerType] = &[
            PlayerType::Participant,
            PlayerType::Computer,
            PlayerType::Observer,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<PlayerType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PlayerType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PlayerType {
}

impl ::protobuf::reflect::ProtobufValue for PlayerType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Alert {
    NuclearLaunchDetected = 1,
    NydusWormDetected = 2,
}

impl ::protobuf::ProtobufEnum for Alert {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Alert> {
        match value {
            1 => ::std::option::Option::Some(Alert::NuclearLaunchDetected),
            2 => ::std::option::Option::Some(Alert::NydusWormDetected),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Alert] = &[
            Alert::NuclearLaunchDetected,
            Alert::NydusWormDetected,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Alert>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Alert", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Alert {
}

impl ::protobuf::reflect::ProtobufValue for Alert {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Result {
    Victory = 1,
    Defeat = 2,
    Tie = 3,
    Undecided = 4,
}

impl ::protobuf::ProtobufEnum for Result {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Result> {
        match value {
            1 => ::std::option::Option::Some(Result::Victory),
            2 => ::std::option::Option::Some(Result::Defeat),
            3 => ::std::option::Option::Some(Result::Tie),
            4 => ::std::option::Option::Some(Result::Undecided),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Result] = &[
            Result::Victory,
            Result::Defeat,
            Result::Tie,
            Result::Undecided,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Result>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Result", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Result {
}

impl ::protobuf::reflect::ProtobufValue for Result {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1ds2clientprotocol/sc2api.proto\x12\x0eSC2APIProtocol\x1a\x1ds2clien\
    tprotocol/common.proto\x1a\x1bs2clientprotocol/data.proto\x1a\x1cs2clien\
    tprotocol/debug.proto\x1a\x1cs2clientprotocol/error.proto\x1a\x1cs2clien\
    tprotocol/query.proto\x1a\x1as2clientprotocol/raw.proto\x1a\x1cs2clientp\
    rotocol/score.proto\x1a\x1es2clientprotocol/spatial.proto\x1a\x19s2clien\
    tprotocol/ui.proto\"\x86\n\n\x07Request\x12D\n\x0bcreate_game\x18\x01\
    \x20\x01(\x0b2!.SC2APIProtocol.RequestCreateGameH\0R\ncreateGame\x12>\n\
    \tjoin_game\x18\x02\x20\x01(\x0b2\x1f.SC2APIProtocol.RequestJoinGameH\0R\
    \x08joinGame\x12G\n\x0crestart_game\x18\x03\x20\x01(\x0b2\".SC2APIProtoc\
    ol.RequestRestartGameH\0R\x0brestartGame\x12G\n\x0cstart_replay\x18\x04\
    \x20\x01(\x0b2\".SC2APIProtocol.RequestStartReplayH\0R\x0bstartReplay\
    \x12A\n\nleave_game\x18\x05\x20\x01(\x0b2\x20.SC2APIProtocol.RequestLeav\
    eGameH\0R\tleaveGame\x12A\n\nquick_save\x18\x06\x20\x01(\x0b2\x20.SC2API\
    Protocol.RequestQuickSaveH\0R\tquickSave\x12A\n\nquick_load\x18\x07\x20\
    \x01(\x0b2\x20.SC2APIProtocol.RequestQuickLoadH\0R\tquickLoad\x121\n\x04\
    quit\x18\x08\x20\x01(\x0b2\x1b.SC2APIProtocol.RequestQuitH\0R\x04quit\
    \x12>\n\tgame_info\x18\t\x20\x01(\x0b2\x1f.SC2APIProtocol.RequestGameInf\
    oH\0R\x08gameInfo\x12F\n\x0bobservation\x18\n\x20\x01(\x0b2\".SC2APIProt\
    ocol.RequestObservationH\0R\x0bobservation\x127\n\x06action\x18\x0b\x20\
    \x01(\x0b2\x1d.SC2APIProtocol.RequestActionH\0R\x06action\x121\n\x04step\
    \x18\x0c\x20\x01(\x0b2\x1b.SC2APIProtocol.RequestStepH\0R\x04step\x121\n\
    \x04data\x18\r\x20\x01(\x0b2\x1b.SC2APIProtocol.RequestDataH\0R\x04data\
    \x124\n\x05query\x18\x0e\x20\x01(\x0b2\x1c.SC2APIProtocol.RequestQueryH\
    \0R\x05query\x12D\n\x0bsave_replay\x18\x0f\x20\x01(\x0b2!.SC2APIProtocol\
    .RequestSaveReplayH\0R\nsaveReplay\x12D\n\x0breplay_info\x18\x10\x20\x01\
    (\x0b2!.SC2APIProtocol.RequestReplayInfoH\0R\nreplayInfo\x12M\n\x0eavail\
    able_maps\x18\x11\x20\x01(\x0b2$.SC2APIProtocol.RequestAvailableMapsH\0R\
    \ravailableMaps\x12;\n\x08save_map\x18\x12\x20\x01(\x0b2\x1e.SC2APIProto\
    col.RequestSaveMapH\0R\x07saveMap\x121\n\x04ping\x18\x13\x20\x01(\x0b2\
    \x1b.SC2APIProtocol.RequestPingH\0R\x04ping\x124\n\x05debug\x18\x14\x20\
    \x01(\x0b2\x1c.SC2APIProtocol.RequestDebugH\0R\x05debugB\t\n\x07request\
    \"\xe2\n\n\x08Response\x12E\n\x0bcreate_game\x18\x01\x20\x01(\x0b2\".SC2\
    APIProtocol.ResponseCreateGameH\0R\ncreateGame\x12?\n\tjoin_game\x18\x02\
    \x20\x01(\x0b2\x20.SC2APIProtocol.ResponseJoinGameH\0R\x08joinGame\x12H\
    \n\x0crestart_game\x18\x03\x20\x01(\x0b2#.SC2APIProtocol.ResponseRestart\
    GameH\0R\x0brestartGame\x12H\n\x0cstart_replay\x18\x04\x20\x01(\x0b2#.SC\
    2APIProtocol.ResponseStartReplayH\0R\x0bstartReplay\x12B\n\nleave_game\
    \x18\x05\x20\x01(\x0b2!.SC2APIProtocol.ResponseLeaveGameH\0R\tleaveGame\
    \x12B\n\nquick_save\x18\x06\x20\x01(\x0b2!.SC2APIProtocol.ResponseQuickS\
    aveH\0R\tquickSave\x12B\n\nquick_load\x18\x07\x20\x01(\x0b2!.SC2APIProto\
    col.ResponseQuickLoadH\0R\tquickLoad\x122\n\x04quit\x18\x08\x20\x01(\x0b\
    2\x1c.SC2APIProtocol.ResponseQuitH\0R\x04quit\x12?\n\tgame_info\x18\t\
    \x20\x01(\x0b2\x20.SC2APIProtocol.ResponseGameInfoH\0R\x08gameInfo\x12G\
    \n\x0bobservation\x18\n\x20\x01(\x0b2#.SC2APIProtocol.ResponseObservatio\
    nH\0R\x0bobservation\x128\n\x06action\x18\x0b\x20\x01(\x0b2\x1e.SC2APIPr\
    otocol.ResponseActionH\0R\x06action\x122\n\x04step\x18\x0c\x20\x01(\x0b2\
    \x1c.SC2APIProtocol.ResponseStepH\0R\x04step\x122\n\x04data\x18\r\x20\
    \x01(\x0b2\x1c.SC2APIProtocol.ResponseDataH\0R\x04data\x125\n\x05query\
    \x18\x0e\x20\x01(\x0b2\x1d.SC2APIProtocol.ResponseQueryH\0R\x05query\x12\
    E\n\x0bsave_replay\x18\x0f\x20\x01(\x0b2\".SC2APIProtocol.ResponseSaveRe\
    playH\0R\nsaveReplay\x12E\n\x0breplay_info\x18\x10\x20\x01(\x0b2\".SC2AP\
    IProtocol.ResponseReplayInfoH\0R\nreplayInfo\x12N\n\x0eavailable_maps\
    \x18\x11\x20\x01(\x0b2%.SC2APIProtocol.ResponseAvailableMapsH\0R\ravaila\
    bleMaps\x12<\n\x08save_map\x18\x12\x20\x01(\x0b2\x1f.SC2APIProtocol.Resp\
    onseSaveMapH\0R\x07saveMap\x122\n\x04ping\x18\x13\x20\x01(\x0b2\x1c.SC2A\
    PIProtocol.ResponsePingH\0R\x04ping\x125\n\x05debug\x18\x14\x20\x01(\x0b\
    2\x1d.SC2APIProtocol.ResponseDebugH\0R\x05debug\x12\x14\n\x05error\x18b\
    \x20\x03(\tR\x05error\x12.\n\x06status\x18c\x20\x01(\x0e2\x16.SC2APIProt\
    ocol.StatusR\x06statusB\n\n\x08response\"\xa1\x02\n\x11RequestCreateGame\
    \x127\n\tlocal_map\x18\x01\x20\x01(\x0b2\x18.SC2APIProtocol.LocalMapH\0R\
    \x08localMap\x12.\n\x12battlenet_map_name\x18\x02\x20\x01(\tH\0R\x10batt\
    lenetMapName\x12>\n\x0cplayer_setup\x18\x03\x20\x03(\x0b2\x1b.SC2APIProt\
    ocol.PlayerSetupR\x0bplayerSetup\x12\x1f\n\x0bdisable_fog\x18\x04\x20\
    \x01(\x08R\ndisableFog\x12\x1f\n\x0brandom_seed\x18\x05\x20\x01(\rR\nran\
    domSeed\x12\x1a\n\x08realtime\x18\x06\x20\x01(\x08R\x08realtimeB\x05\n\
    \x03Map\"@\n\x08LocalMap\x12\x19\n\x08map_path\x18\x01\x20\x01(\tR\x07ma\
    pPath\x12\x19\n\x08map_data\x18\x07\x20\x01(\x0cR\x07mapData\"\xb1\x02\n\
    \x12ResponseCreateGame\x12>\n\x05error\x18\x01\x20\x01(\x0e2(.SC2APIProt\
    ocol.ResponseCreateGame.ErrorR\x05error\x12#\n\rerror_details\x18\x02\
    \x20\x01(\tR\x0cerrorDetails\"\xb5\x01\n\x05Error\x12\x0e\n\nMissingMap\
    \x10\x01\x12\x12\n\x0eInvalidMapPath\x10\x02\x12\x12\n\x0eInvalidMapData\
    \x10\x03\x12\x12\n\x0eInvalidMapName\x10\x04\x12\x14\n\x10InvalidMapHand\
    le\x10\x05\x12\x16\n\x12MissingPlayerSetup\x10\x06\x12\x16\n\x12InvalidP\
    layerSetup\x10\x07\x12\x1a\n\x16MultiplayerUnsupported\x10\x08\"\xd3\x02\
    \n\x0fRequestJoinGame\x12*\n\x04race\x18\x01\x20\x01(\x0e2\x14.SC2APIPro\
    tocol.RaceH\0R\x04race\x12.\n\x12observed_player_id\x18\x02\x20\x01(\rH\
    \0R\x10observedPlayerId\x12:\n\x07options\x18\x03\x20\x01(\x0b2\x20.SC2A\
    PIProtocol.InterfaceOptionsR\x07options\x12:\n\x0cserver_ports\x18\x04\
    \x20\x01(\x0b2\x17.SC2APIProtocol.PortSetR\x0bserverPorts\x12:\n\x0cclie\
    nt_ports\x18\x05\x20\x03(\x0b2\x17.SC2APIProtocol.PortSetR\x0bclientPort\
    s\x12\x1f\n\x0bshared_port\x18\x06\x20\x01(\x05R\nsharedPortB\x0f\n\rpar\
    ticipation\"C\n\x07PortSet\x12\x1b\n\tgame_port\x18\x01\x20\x01(\x05R\
    \x08gamePort\x12\x1b\n\tbase_port\x18\x02\x20\x01(\x05R\x08basePort\"\
    \xa1\x03\n\x10ResponseJoinGame\x12\x1b\n\tplayer_id\x18\x01\x20\x01(\rR\
    \x08playerId\x12<\n\x05error\x18\x02\x20\x01(\x0e2&.SC2APIProtocol.Respo\
    nseJoinGame.ErrorR\x05error\x12#\n\rerror_details\x18\x03\x20\x01(\tR\
    \x0cerrorDetails\"\x8c\x02\n\x05Error\x12\x18\n\x14MissingParticipation\
    \x10\x01\x12\x1b\n\x17InvalidObservedPlayerId\x10\x02\x12\x12\n\x0eMissi\
    ngOptions\x10\x03\x12\x10\n\x0cMissingPorts\x10\x04\x12\x0c\n\x08GameFul\
    l\x10\x05\x12\x0f\n\x0bLaunchError\x10\x06\x12\x16\n\x12FeatureUnsupport\
    ed\x10\x07\x12\x12\n\x0eNoSpaceForUser\x10\x08\x12\x13\n\x0fMapDoesNotEx\
    ist\x10\t\x12\x11\n\rCannotOpenMap\x10\n\x12\x11\n\rChecksumError\x10\
    \x0b\x12\x10\n\x0cNetworkError\x10\x0c\x12\x0e\n\nOtherError\x10\r\"\x14\
    \n\x12RequestRestartGame\"\x95\x01\n\x13ResponseRestartGame\x12?\n\x05er\
    ror\x18\x01\x20\x01(\x0e2).SC2APIProtocol.ResponseRestartGame.ErrorR\x05\
    error\x12#\n\rerror_details\x18\x02\x20\x01(\tR\x0cerrorDetails\"\x18\n\
    \x05Error\x12\x0f\n\x0bLaunchError\x10\x01\"\x8a\x02\n\x12RequestStartRe\
    play\x12!\n\x0breplay_path\x18\x01\x20\x01(\tH\0R\nreplayPath\x12!\n\x0b\
    replay_data\x18\x05\x20\x01(\x0cH\0R\nreplayData\x12\x19\n\x08map_data\
    \x18\x06\x20\x01(\x0cR\x07mapData\x12,\n\x12observed_player_id\x18\x02\
    \x20\x01(\x05R\x10observedPlayerId\x12:\n\x07options\x18\x03\x20\x01(\
    \x0b2\x20.SC2APIProtocol.InterfaceOptionsR\x07options\x12\x1f\n\x0bdisab\
    le_fog\x18\x04\x20\x01(\x08R\ndisableFogB\x08\n\x06replay\"\x9c\x02\n\
    \x13ResponseStartReplay\x12?\n\x05error\x18\x01\x20\x01(\x0e2).SC2APIPro\
    tocol.ResponseStartReplay.ErrorR\x05error\x12#\n\rerror_details\x18\x02\
    \x20\x01(\tR\x0cerrorDetails\"\x9e\x01\n\x05Error\x12\x11\n\rMissingRepl\
    ay\x10\x01\x12\x15\n\x11InvalidReplayPath\x10\x02\x12\x15\n\x11InvalidRe\
    playData\x10\x03\x12\x12\n\x0eInvalidMapData\x10\x04\x12\x1b\n\x17Invali\
    dObservedPlayerId\x10\x05\x12\x12\n\x0eMissingOptions\x10\x06\x12\x0f\n\
    \x0bLaunchError\x10\x07\"\x12\n\x10RequestLeaveGame\"\x13\n\x11ResponseL\
    eaveGame\"\x12\n\x10RequestQuickSave\"\x13\n\x11ResponseQuickSave\"\x12\
    \n\x10RequestQuickLoad\"\x13\n\x11ResponseQuickLoad\"\r\n\x0bRequestQuit\
    \"\x0e\n\x0cResponseQuit\"\x11\n\x0fRequestGameInfo\"\xa0\x02\n\x10Respo\
    nseGameInfo\x12\x19\n\x08map_name\x18\x01\x20\x01(\tR\x07mapName\x12\x1b\
    \n\tmod_names\x18\x06\x20\x03(\tR\x08modNames\x12$\n\x0elocal_map_path\
    \x18\x02\x20\x01(\tR\x0clocalMapPath\x12;\n\x0bplayer_info\x18\x03\x20\
    \x03(\x0b2\x1a.SC2APIProtocol.PlayerInfoR\nplayerInfo\x125\n\tstart_raw\
    \x18\x04\x20\x01(\x0b2\x18.SC2APIProtocol.StartRawR\x08startRaw\x12:\n\
    \x07options\x18\x05\x20\x01(\x0b2\x20.SC2APIProtocol.InterfaceOptionsR\
    \x07options\"5\n\x12RequestObservation\x12\x1f\n\x0bdisable_fog\x18\x01\
    \x20\x01(\x08R\ndisableFog\"\xbd\x02\n\x13ResponseObservation\x120\n\x07\
    actions\x18\x01\x20\x03(\x0b2\x16.SC2APIProtocol.ActionR\x07actions\x12@\
    \n\raction_errors\x18\x02\x20\x03(\x0b2\x1b.SC2APIProtocol.ActionErrorR\
    \x0cactionErrors\x12=\n\x0bobservation\x18\x03\x20\x01(\x0b2\x1b.SC2APIP\
    rotocol.ObservationR\x0bobservation\x12A\n\rplayer_result\x18\x04\x20\
    \x03(\x0b2\x1c.SC2APIProtocol.PlayerResultR\x0cplayerResult\x120\n\x04ch\
    at\x18\x05\x20\x03(\x0b2\x1c.SC2APIProtocol.ChatReceivedR\x04chat\"E\n\
    \x0cChatReceived\x12\x1b\n\tplayer_id\x18\x01\x20\x01(\x05R\x08playerId\
    \x12\x18\n\x07message\x18\x02\x20\x01(\tR\x07message\"A\n\rRequestAction\
    \x120\n\x07actions\x18\x01\x20\x03(\x0b2\x16.SC2APIProtocol.ActionR\x07a\
    ctions\"F\n\x0eResponseAction\x124\n\x06result\x18\x01\x20\x03(\x0e2\x1c\
    .SC2APIProtocol.ActionResultR\x06result\"#\n\x0bRequestStep\x12\x14\n\
    \x05count\x18\x01\x20\x01(\rR\x05count\"\x0e\n\x0cResponseStep\"\x86\x01\
    \n\x0bRequestData\x12\x1d\n\nability_id\x18\x01\x20\x01(\x08R\tabilityId\
    \x12\x20\n\x0cunit_type_id\x18\x02\x20\x01(\x08R\nunitTypeId\x12\x1d\n\n\
    upgrade_id\x18\x03\x20\x01(\x08R\tupgradeId\x12\x17\n\x07buff_id\x18\x04\
    \x20\x01(\x08R\x06buffId\"\xe6\x01\n\x0cResponseData\x129\n\tabilities\
    \x18\x01\x20\x03(\x0b2\x1b.SC2APIProtocol.AbilityDataR\tabilities\x122\n\
    \x05units\x18\x02\x20\x03(\x0b2\x1c.SC2APIProtocol.UnitTypeDataR\x05unit\
    s\x127\n\x08upgrades\x18\x03\x20\x03(\x0b2\x1b.SC2APIProtocol.UpgradeDat\
    aR\x08upgrades\x12.\n\x05buffs\x18\x04\x20\x03(\x0b2\x18.SC2APIProtocol.\
    BuffDataR\x05buffs\"\x13\n\x11RequestSaveReplay\"(\n\x12ResponseSaveRepl\
    ay\x12\x12\n\x04data\x18\x01\x20\x01(\x0cR\x04data\"\x88\x01\n\x11Reques\
    tReplayInfo\x12!\n\x0breplay_path\x18\x01\x20\x01(\tH\0R\nreplayPath\x12\
    !\n\x0breplay_data\x18\x02\x20\x01(\x0cH\0R\nreplayData\x12#\n\rdownload\
    _data\x18\x03\x20\x01(\x08R\x0cdownloadDataB\x08\n\x06replay\"\xcf\x01\n\
    \x0fPlayerInfoExtra\x12;\n\x0bplayer_info\x18\x01\x20\x01(\x0b2\x1a.SC2A\
    PIProtocol.PlayerInfoR\nplayerInfo\x12A\n\rplayer_result\x18\x02\x20\x01\
    (\x0b2\x1c.SC2APIProtocol.PlayerResultR\x0cplayerResult\x12\x1d\n\nplaye\
    r_mmr\x18\x03\x20\x01(\x05R\tplayerMmr\x12\x1d\n\nplayer_apm\x18\x04\x20\
    \x01(\x05R\tplayerApm\"\xd3\x04\n\x12ResponseReplayInfo\x12\x19\n\x08map\
    _name\x18\x01\x20\x01(\tR\x07mapName\x12$\n\x0elocal_map_path\x18\x02\
    \x20\x01(\tR\x0clocalMapPath\x12@\n\x0bplayer_info\x18\x03\x20\x03(\x0b2\
    \x1f.SC2APIProtocol.PlayerInfoExtraR\nplayerInfo\x12.\n\x13game_duration\
    _loops\x18\x04\x20\x01(\rR\x11gameDurationLoops\x122\n\x15game_duration_\
    seconds\x18\x05\x20\x01(\x02R\x13gameDurationSeconds\x12!\n\x0cgame_vers\
    ion\x18\x06\x20\x01(\tR\x0bgameVersion\x12!\n\x0cdata_version\x18\x0b\
    \x20\x01(\tR\x0bdataVersion\x12\x1d\n\ndata_build\x18\x07\x20\x01(\rR\td\
    ataBuild\x12\x1d\n\nbase_build\x18\x08\x20\x01(\rR\tbaseBuild\x12>\n\x05\
    error\x18\t\x20\x01(\x0e2(.SC2APIProtocol.ResponseReplayInfo.ErrorR\x05e\
    rror\x12#\n\rerror_details\x18\n\x20\x01(\tR\x0cerrorDetails\"m\n\x05Err\
    or\x12\x11\n\rMissingReplay\x10\x01\x12\x15\n\x11InvalidReplayPath\x10\
    \x02\x12\x15\n\x11InvalidReplayData\x10\x03\x12\x10\n\x0cParsingError\
    \x10\x04\x12\x11\n\rDownloadError\x10\x05\"\x16\n\x14RequestAvailableMap\
    s\"o\n\x15ResponseAvailableMaps\x12&\n\x0flocal_map_paths\x18\x01\x20\
    \x03(\tR\rlocalMapPaths\x12.\n\x13battlenet_map_names\x18\x02\x20\x03(\t\
    R\x11battlenetMapNames\"F\n\x0eRequestSaveMap\x12\x19\n\x08map_path\x18\
    \x01\x20\x01(\tR\x07mapPath\x12\x19\n\x08map_data\x18\x02\x20\x01(\x0cR\
    \x07mapData\"k\n\x0fResponseSaveMap\x12;\n\x05error\x18\x01\x20\x01(\x0e\
    2%.SC2APIProtocol.ResponseSaveMap.ErrorR\x05error\"\x1b\n\x05Error\x12\
    \x12\n\x0eInvalidMapData\x10\x01\"\r\n\x0bRequestPing\"\x92\x01\n\x0cRes\
    ponsePing\x12!\n\x0cgame_version\x18\x01\x20\x01(\tR\x0bgameVersion\x12!\
    \n\x0cdata_version\x18\x02\x20\x01(\tR\x0bdataVersion\x12\x1d\n\ndata_bu\
    ild\x18\x03\x20\x01(\rR\tdataBuild\x12\x1d\n\nbase_build\x18\x04\x20\x01\
    (\rR\tbaseBuild\"B\n\x0cRequestDebug\x122\n\x05debug\x18\x01\x20\x03(\
    \x0b2\x1c.SC2APIProtocol.DebugCommandR\x05debug\"\x0f\n\rResponseDebug\"\
    \xa3\x01\n\x0bPlayerSetup\x12.\n\x04type\x18\x01\x20\x01(\x0e2\x1a.SC2AP\
    IProtocol.PlayerTypeR\x04type\x12(\n\x04race\x18\x02\x20\x01(\x0e2\x14.S\
    C2APIProtocol.RaceR\x04race\x12:\n\ndifficulty\x18\x03\x20\x01(\x0e2\x1a\
    .SC2APIProtocol.DifficultyR\ndifficulty\"\xab\x01\n\x12SpatialCameraSetu\
    p\x12\x14\n\x05width\x18\x01\x20\x01(\x02R\x05width\x127\n\nresolution\
    \x18\x02\x20\x01(\x0b2\x17.SC2APIProtocol.Size2DIR\nresolution\x12F\n\
    \x12minimap_resolution\x18\x03\x20\x01(\x0b2\x17.SC2APIProtocol.Size2DIR\
    \x11minimapResolution\"\xbf\x01\n\x10InterfaceOptions\x12\x10\n\x03raw\
    \x18\x01\x20\x01(\x08R\x03raw\x12\x14\n\x05score\x18\x02\x20\x01(\x08R\
    \x05score\x12G\n\rfeature_layer\x18\x03\x20\x01(\x0b2\".SC2APIProtocol.S\
    patialCameraSetupR\x0cfeatureLayer\x12:\n\x06render\x18\x04\x20\x01(\x0b\
    2\".SC2APIProtocol.SpatialCameraSetupR\x06render\"\x89\x02\n\nPlayerInfo\
    \x12\x1b\n\tplayer_id\x18\x01\x20\x01(\rR\x08playerId\x12.\n\x04type\x18\
    \x02\x20\x01(\x0e2\x1a.SC2APIProtocol.PlayerTypeR\x04type\x12;\n\x0erace\
    _requested\x18\x03\x20\x01(\x0e2\x14.SC2APIProtocol.RaceR\rraceRequested\
    \x125\n\x0brace_actual\x18\x04\x20\x01(\x0e2\x14.SC2APIProtocol.RaceR\nr\
    aceActual\x12:\n\ndifficulty\x18\x05\x20\x01(\x0e2\x1a.SC2APIProtocol.Di\
    fficultyR\ndifficulty\"\xed\x02\n\x0cPlayerCommon\x12\x1b\n\tplayer_id\
    \x18\x01\x20\x01(\rR\x08playerId\x12\x1a\n\x08minerals\x18\x02\x20\x01(\
    \rR\x08minerals\x12\x18\n\x07vespene\x18\x03\x20\x01(\rR\x07vespene\x12\
    \x19\n\x08food_cap\x18\x04\x20\x01(\rR\x07foodCap\x12\x1b\n\tfood_used\
    \x18\x05\x20\x01(\rR\x08foodUsed\x12\x1b\n\tfood_army\x18\x06\x20\x01(\r\
    R\x08foodArmy\x12!\n\x0cfood_workers\x18\x07\x20\x01(\rR\x0bfoodWorkers\
    \x12*\n\x11idle_worker_count\x18\x08\x20\x01(\rR\x0fidleWorkerCount\x12\
    \x1d\n\narmy_count\x18\t\x20\x01(\rR\tarmyCount\x12&\n\x0fwarp_gate_coun\
    t\x18\n\x20\x01(\rR\rwarpGateCount\x12\x1f\n\x0blarva_count\x18\x0b\x20\
    \x01(\rR\nlarvaCount\"\x97\x04\n\x0bObservation\x12\x1b\n\tgame_loop\x18\
    \t\x20\x01(\rR\x08gameLoop\x12A\n\rplayer_common\x18\x01\x20\x01(\x0b2\
    \x1c.SC2APIProtocol.PlayerCommonR\x0cplayerCommon\x12-\n\x06alerts\x18\n\
    \x20\x03(\x0e2\x15.SC2APIProtocol.AlertR\x06alerts\x12>\n\tabilities\x18\
    \x03\x20\x03(\x0b2\x20.SC2APIProtocol.AvailableAbilityR\tabilities\x12+\
    \n\x05score\x18\x04\x20\x01(\x0b2\x15.SC2APIProtocol.ScoreR\x05score\x12\
    9\n\x08raw_data\x18\x05\x20\x01(\x0b2\x1e.SC2APIProtocol.ObservationRawR\
    \x07rawData\x12U\n\x12feature_layer_data\x18\x06\x20\x01(\x0b2'.SC2APIPr\
    otocol.ObservationFeatureLayerR\x10featureLayerData\x12B\n\x0brender_dat\
    a\x18\x07\x20\x01(\x0b2!.SC2APIProtocol.ObservationRenderR\nrenderData\
    \x126\n\x07ui_data\x18\x08\x20\x01(\x0b2\x1d.SC2APIProtocol.ObservationU\
    IR\x06uiData\"\xbe\x02\n\x06Action\x128\n\naction_raw\x18\x01\x20\x01(\
    \x0b2\x19.SC2APIProtocol.ActionRawR\tactionRaw\x12O\n\x14action_feature_\
    layer\x18\x02\x20\x01(\x0b2\x1d.SC2APIProtocol.ActionSpatialR\x12actionF\
    eatureLayer\x12B\n\raction_render\x18\x03\x20\x01(\x0b2\x1d.SC2APIProtoc\
    ol.ActionSpatialR\x0cactionRender\x125\n\taction_ui\x18\x04\x20\x01(\x0b\
    2\x18.SC2APIProtocol.ActionUIR\x08actionUi\x12.\n\x04chat\x18\x05\x20\
    \x03(\x0b2\x1a.SC2APIProtocol.ActionChatR\x04chat\"\x88\x01\n\nActionCha\
    t\x12<\n\x07channel\x18\x01\x20\x01(\x0e2\".SC2APIProtocol.ActionChat.Ch\
    annelR\x07channel\x12\x18\n\x07message\x18\x02\x20\x01(\tR\x07message\"\
    \"\n\x07Channel\x12\r\n\tBroadcast\x10\x01\x12\x08\n\x04Team\x10\x02\"}\
    \n\x0bActionError\x12\x19\n\x08unit_tag\x18\x01\x20\x01(\x04R\x07unitTag\
    \x12\x1d\n\nability_id\x18\x02\x20\x01(\x04R\tabilityId\x124\n\x06result\
    \x18\x03\x20\x01(\x0e2\x1c.SC2APIProtocol.ActionResultR\x06result\"[\n\
    \x0cPlayerResult\x12\x1b\n\tplayer_id\x18\x01\x20\x01(\rR\x08playerId\
    \x12.\n\x06result\x18\x02\x20\x01(\x0e2\x16.SC2APIProtocol.ResultR\x06re\
    sult*c\n\x06Status\x12\x0c\n\x08launched\x10\x01\x12\r\n\tinit_game\x10\
    \x02\x12\x0b\n\x07in_game\x10\x03\x12\r\n\tin_replay\x10\x04\x12\t\n\x05\
    ended\x10\x05\x12\x08\n\x04quit\x10\x06\x12\x0b\n\x07unknown\x10c*\x96\
    \x01\n\nDifficulty\x12\x0c\n\x08VeryEasy\x10\x01\x12\x08\n\x04Easy\x10\
    \x02\x12\n\n\x06Medium\x10\x03\x12\x0e\n\nMediumHard\x10\x04\x12\x08\n\
    \x04Hard\x10\x05\x12\n\n\x06Harder\x10\x06\x12\x0c\n\x08VeryHard\x10\x07\
    \x12\x0f\n\x0bCheatVision\x10\x08\x12\x0e\n\nCheatMoney\x10\t\x12\x0f\n\
    \x0bCheatInsane\x10\n*9\n\nPlayerType\x12\x0f\n\x0bParticipant\x10\x01\
    \x12\x0c\n\x08Computer\x10\x02\x12\x0c\n\x08Observer\x10\x03*9\n\x05Aler\
    t\x12\x19\n\x15NuclearLaunchDetected\x10\x01\x12\x15\n\x11NydusWormDetec\
    ted\x10\x02*9\n\x06Result\x12\x0b\n\x07Victory\x10\x01\x12\n\n\x06Defeat\
    \x10\x02\x12\x07\n\x03Tie\x10\x03\x12\r\n\tUndecided\x10\x04J\x9e\xda\
    \x01\n\x07\x12\x05\x01\0\xce\x04\x01\n\x08\n\x01\x0c\x12\x03\x01\0\x12\n\
    \x08\n\x01\x02\x12\x03\x03\x08\x16\n\t\n\x02\x03\0\x12\x03\x05\x07&\n\t\
    \n\x02\x03\x01\x12\x03\x06\x07$\n\t\n\x02\x03\x02\x12\x03\x07\x07%\n\t\n\
    \x02\x03\x03\x12\x03\x08\x07%\n\t\n\x02\x03\x04\x12\x03\t\x07%\n\t\n\x02\
    \x03\x05\x12\x03\n\x07#\n\t\n\x02\x03\x06\x12\x03\x0b\x07%\n\t\n\x02\x03\
    \x07\x12\x03\x0c\x07'\n\t\n\x02\x03\x08\x12\x03\r\x07\"\n\xa9#\n\x02\x04\
    \0\x12\x04O\0o\x012\x86#\n\x20Notes:\n\x20\x20Single\x20player\x20flow:\
    \n\x20\x20\x20\x201)\x20Call\x20Request.create_game\x20with\x20a\x20vali\
    d\x20single\x20player\x20map\x20(a\x20multiplayer\x20map\x20will\x20end\
    \x20right\x20away).\n\x20\x20\x20\x202)\x20Call\x20Request.join_game,\
    \x20wait\x20for\x20the\x20response.\n\x20\x20\x20\x203)\x20Request.end\
    \x20will\x20terminate\x20the\x20game.\x20Observations\x20can\x20still\
    \x20be\x20made.\n\x20\x20Multi-player\x20flow:\n\x20\x20\x20\x201)\x20La\
    unch\x20two\x20game\x20instances\x20with\x20separate\x20ports.\n\x20\x20\
    \x20\x202)\x20Designate\x20a\x20host,\x20and\x20Request.create_game\x20w\
    ith\x20a\x20multiplayer\x20map.\n\x20\x20\x20\x203)\x20Call\x20Request.j\
    oin\x20on\x20BOTH\x20clients.\x20Join\x20will\x20block\x20until\x20both\
    \x20clients\x20connect.\n\x20\x20\x20\x204)\x20Wait\x20for\x20a\x20respo\
    nse\x20from\x20both\x20clients.\x20They\x20can\x20now\x20play/step.\n\
    \x20\x20\x20\x205)\x20Steps\x20should\x20be\x20syncronized.\x20One\x20cl\
    ient\x20may\x20time\x20out\x20if\x20they\x20are\x20not.\x20Multiple\x20s\
    tep\x20sizes\x20are\x20ok.\n\x20\x20\x20\x204)\x20Call\x20Request.leave\
    \x20at\x20any\x20point\x20or\x20when\x20the\x20game\x20ends.\x20Observat\
    ions\x20will\x20not\x20be\x20valid\x20after\x20this.\n\n\x20States:\n\n-\
    -----------------|---------------------------------------------------|--\
    ---------------------|\n\x20\x20Request\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20|\x20Valid\x20in\x20State\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20|\x20Transition\x20to\x20State\x20\x20\
    \x20|\n------------------|----------------------------------------------\
    -----|-----------------------|\n\x20create_game\x20\x20\x20\x20\x20\x20|\
    \x20launched\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20init_game\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20|\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20ended\x20(singleplayer\x20only)\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20|\x20init_game\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20|\n\x20join_game*\x20\x20\x20\x20\x20\x20\x20|\
    \x20init_game\x20(singleplayer\x20or\x20multiplayer\x20host\x20only)\x20\
    |\x20in_game\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    |\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20|\x20launched\x20(multiplayer\x20client\x20only)\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20in_game\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20restart_game\x20\x20\
    \x20\x20\x20|\x20ended\x20(singleplayer\x20only)\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20|\x20in_game\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20|\n\x20start_replay\x20\x20\x20\x20\x20|\x20launched\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20|\x20in_replay\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20|\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20|\x20ended\x20(singleplayer\x20only)\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    |\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20|\n\x20leave_game\x20\x20\x20\x20\x20\x20\x20|\
    \x20in_game\x20(required\x20when\x20finishing\x20multiplayer)\x20\x20\
    \x20\x20\x20|\x20launched\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20|\n\x20quick_save\x20\x20\x20\x20\x20\x20\x20|\x20in_game\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20quick_load\
    \x20\x20\x20\x20\x20\x20\x20|\x20in_game\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20|\n\x20quit\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20|\x20any\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20qui\
    t\x20(not\x20sent)\x20\x20\x20\x20\x20\x20\x20|\n\x20game_info\x20\x20\
    \x20\x20\x20\x20\x20\x20|\x20in_game\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20|\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20|\x20in_replay\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20|\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20|\x20ended\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20|\n\x20observation\x20\x20\x20\x20\x20\x20|\x20in_game\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20in_replay\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20ended\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20step*\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20|\x20in_game\x20(not\x20available\x20in\
    \x20realtime\x20mode)\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20in_gam\
    e\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20in_\
    replay\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20|\x20ended\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20action\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20|\x20in_game\x20(not\x20available\x20to\x20obser\
    vers)\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20|\n\x20data\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20|\x20in_game\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    |\x20in_replay\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20end\
    ed\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20q\
    uery\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20in_game\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20in_replay\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20ended\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20save_replay\x20\x20\x20\
    \x20\x20\x20|\x20in_game\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20|\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20|\x20ended\x20(only\x20after\x20a\x20game)\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20|\n\x20replay_info\x20\x20\x20\x20\x20\
    \x20|\x20any\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20|\n\x20available_maps\x20\x20\x20|\x20any\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n\x20save_map\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20|\x20any\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20|\n\x20ping\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20|\x20any\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20|\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20|\n\x20debug\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20|\x20in_game\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\x20various\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20|\n-------------\
    -----|---------------------------------------------------|--------------\
    ---------|\n\n\x20*\x20In\x20multiplayer,\x20these\x20require\x20synchro\
    nization\x20between\x20clients.\n\n\x20Notes:\n\x20\x20\x20\x20\x20\x20-\
    \x20if\x20a\x20request\x20fails,\x20the\x20game\x20remains\x20in\x20the\
    \x20current\x20state.\n\n2\x14\n\x20Request/Response\n\n\n\n\n\x03\x04\0\
    \x01\x12\x03O\x08\x0f\n\x0c\n\x04\x04\0\x08\0\x12\x04P\x02n\x03\n\x0c\n\
    \x05\x04\0\x08\0\x01\x12\x03P\x08\x0f\n=\n\x04\x04\0\x02\0\x12\x03R\x04&\
    \x1a\x0c\x20Game\x20Setup\n\"\"\x20Send\x20to\x20host\x20to\x20initializ\
    e\x20game.\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03R\x04\x15\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03R\x16!\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03R$%\n>\
    \n\x04\x04\0\x02\x01\x12\x03S\x04\"\"1\x20Send\x20to\x20host\x20and\x20a\
    ll\x20clients\x20for\x20game\x20to\x20begin.\n\n\x0c\n\x05\x04\0\x02\x01\
    \x06\x12\x03S\x04\x13\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03S\x14\x1d\n\
    \x0c\n\x05\x04\0\x02\x01\x03\x12\x03S\x20!\nU\n\x04\x04\0\x02\x02\x12\
    \x03T\x04(\"H\x20Single\x20player\x20only.\x20Reinitializes\x20the\x20ga\
    me\x20with\x20the\x20same\x20player\x20setup.\n\n\x0c\n\x05\x04\0\x02\
    \x02\x06\x12\x03T\x04\x16\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03T\x17#\n\
    \x0c\n\x05\x04\0\x02\x02\x03\x12\x03T&'\n&\n\x04\x04\0\x02\x03\x12\x03U\
    \x04(\"\x19\x20Start\x20playing\x20a\x20replay.\n\n\x0c\n\x05\x04\0\x02\
    \x03\x06\x12\x03U\x04\x16\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03U\x17#\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03U&'\n^\n\x04\x04\0\x02\x04\x12\x03V\
    \x04$\"Q\x20Multiplayer\x20only.\x20Disconnects\x20from\x20a\x20multipla\
    yer\x20game,\x20equivalent\x20to\x20surrender.\n\n\x0c\n\x05\x04\0\x02\
    \x04\x06\x12\x03V\x04\x14\n\x0c\n\x05\x04\0\x02\x04\x01\x12\x03V\x15\x1f\
    \n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03V\"#\nD\n\x04\x04\0\x02\x05\x12\
    \x03X\x04$\"7\x20Not\x20implemented.\x20Saves\x20game\x20to\x20an\x20in-\
    memory\x20bookmark.\n\n\x0c\n\x05\x04\0\x02\x05\x06\x12\x03X\x04\x14\n\
    \x0c\n\x05\x04\0\x02\x05\x01\x12\x03X\x15\x1f\n\x0c\n\x05\x04\0\x02\x05\
    \x03\x12\x03X\"#\nA\n\x04\x04\0\x02\x06\x12\x03Y\x04$\"4\x20Not\x20imple\
    mented.\x20Loads\x20from\x20an\x20in-memory\x20bookmark.\n\n\x0c\n\x05\
    \x04\0\x02\x06\x06\x12\x03Y\x04\x14\n\x0c\n\x05\x04\0\x02\x06\x01\x12\
    \x03Y\x15\x1f\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03Y\"#\n*\n\x04\x04\0\
    \x02\x07\x12\x03[\x04\x19\"\x1d\x20Terminates\x20the\x20application.\n\n\
    \x0c\n\x05\x04\0\x02\x07\x06\x12\x03[\x04\x0f\n\x0c\n\x05\x04\0\x02\x07\
    \x01\x12\x03[\x10\x14\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03[\x17\x18\nI\
    \n\x04\x04\0\x02\x08\x12\x03^\x04\"\x1a\r\x20During\x20Game\n\"-\x20Stat\
    ic\x20data\x20about\x20the\x20current\x20game\x20and\x20map.\n\n\x0c\n\
    \x05\x04\0\x02\x08\x06\x12\x03^\x04\x13\n\x0c\n\x05\x04\0\x02\x08\x01\
    \x12\x03^\x14\x1d\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03^\x20!\n2\n\x04\
    \x04\0\x02\t\x12\x03_\x04(\"%\x20Snapshot\x20of\x20the\x20current\x20gam\
    e\x20state.\n\n\x0c\n\x05\x04\0\x02\t\x06\x12\x03_\x04\x16\n\x0c\n\x05\
    \x04\0\x02\t\x01\x12\x03_\x17\"\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03_%'\n\
    \"\n\x04\x04\0\x02\n\x12\x03`\x04\x1e\"\x15\x20Executes\x20an\x20action.\
    \n\n\x0c\n\x05\x04\0\x02\n\x06\x12\x03`\x04\x11\n\x0c\n\x05\x04\0\x02\n\
    \x01\x12\x03`\x12\x18\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03`\x1b\x1d\n,\n\
    \x04\x04\0\x02\x0b\x12\x03a\x04\x1a\"\x1f\x20Advances\x20the\x20game\x20\
    simulation.\n\n\x0c\n\x05\x04\0\x02\x0b\x06\x12\x03a\x04\x0f\n\x0c\n\x05\
    \x04\0\x02\x0b\x01\x12\x03a\x10\x14\n\x0c\n\x05\x04\0\x02\x0b\x03\x12\
    \x03a\x17\x19\n\\\n\x04\x04\0\x02\x0c\x12\x03b\x04\x1a\"O\x20Data\x20abo\
    ut\x20different\x20gameplay\x20elements.\x20May\x20be\x20different\x20fo\
    r\x20different\x20games.\n\n\x0c\n\x05\x04\0\x02\x0c\x06\x12\x03b\x04\
    \x0f\n\x0c\n\x05\x04\0\x02\x0c\x01\x12\x03b\x10\x14\n\x0c\n\x05\x04\0\
    \x02\x0c\x03\x12\x03b\x17\x19\n<\n\x04\x04\0\x02\r\x12\x03c\x04\x1c\"/\
    \x20Additional\x20methods\x20for\x20inspecting\x20game\x20state.\n\n\x0c\
    \n\x05\x04\0\x02\r\x06\x12\x03c\x04\x10\n\x0c\n\x05\x04\0\x02\r\x01\x12\
    \x03c\x11\x16\n\x0c\n\x05\x04\0\x02\r\x03\x12\x03c\x19\x1b\n\"\n\x04\x04\
    \0\x02\x0e\x12\x03d\x04'\"\x15\x20Generates\x20a\x20replay.\n\n\x0c\n\
    \x05\x04\0\x02\x0e\x06\x12\x03d\x04\x15\n\x0c\n\x05\x04\0\x02\x0e\x01\
    \x12\x03d\x16!\n\x0c\n\x05\x04\0\x02\x0e\x03\x12\x03d$&\nX\n\x04\x04\0\
    \x02\x0f\x12\x03g\x04'\x1a\x08\x20Other.\n\"A\x20Returns\x20metadata\x20\
    about\x20a\x20replay\x20file.\x20Does\x20not\x20load\x20the\x20replay.\n\
    \n\x0c\n\x05\x04\0\x02\x0f\x06\x12\x03g\x04\x15\n\x0c\n\x05\x04\0\x02\
    \x0f\x01\x12\x03g\x16!\n\x0c\n\x05\x04\0\x02\x0f\x03\x12\x03g$&\n?\n\x04\
    \x04\0\x02\x10\x12\x03h\x04-\"2\x20Returns\x20directory\x20of\x20maps\
    \x20that\x20can\x20be\x20played\x20on.\n\n\x0c\n\x05\x04\0\x02\x10\x06\
    \x12\x03h\x04\x18\n\x0c\n\x05\x04\0\x02\x10\x01\x12\x03h\x19'\n\x0c\n\
    \x05\x04\0\x02\x10\x03\x12\x03h*,\nA\n\x04\x04\0\x02\x11\x12\x03i\x04!\"\
    4\x20Saves\x20binary\x20map\x20data\x20to\x20the\x20local\x20temp\x20dir\
    ectory.\n\n\x0c\n\x05\x04\0\x02\x11\x06\x12\x03i\x04\x12\n\x0c\n\x05\x04\
    \0\x02\x11\x01\x12\x03i\x13\x1b\n\x0c\n\x05\x04\0\x02\x11\x03\x12\x03i\
    \x1e\x20\n@\n\x04\x04\0\x02\x12\x12\x03l\x04\x1a\x1a\x0b\x20Debugging\n\
    \"&\x20Network\x20ping\x20for\x20testing\x20connection.\n\n\x0c\n\x05\
    \x04\0\x02\x12\x06\x12\x03l\x04\x0f\n\x0c\n\x05\x04\0\x02\x12\x01\x12\
    \x03l\x10\x14\n\x0c\n\x05\x04\0\x02\x12\x03\x12\x03l\x17\x19\nC\n\x04\
    \x04\0\x02\x13\x12\x03m\x04\x1c\"6\x20Display\x20debug\x20information\
    \x20and\x20execute\x20debug\x20actions.\n\n\x0c\n\x05\x04\0\x02\x13\x06\
    \x12\x03m\x04\x10\n\x0c\n\x05\x04\0\x02\x13\x01\x12\x03m\x11\x16\n\x0c\n\
    \x05\x04\0\x02\x13\x03\x12\x03m\x19\x1b\n\x0b\n\x02\x04\x01\x12\x05q\0\
    \x90\x01\x01\n\n\n\x03\x04\x01\x01\x12\x03q\x08\x10\n\r\n\x04\x04\x01\
    \x08\0\x12\x05r\x02\x8d\x01\x03\n\x0c\n\x05\x04\x01\x08\0\x01\x12\x03r\
    \x08\x10\n\x0b\n\x04\x04\x01\x02\0\x12\x03s\x04'\n\x0c\n\x05\x04\x01\x02\
    \0\x06\x12\x03s\x04\x16\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03s\x17\"\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03s%&\n\x0b\n\x04\x04\x01\x02\x01\x12\
    \x03t\x04#\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03t\x04\x14\n\x0c\n\x05\
    \x04\x01\x02\x01\x01\x12\x03t\x15\x1e\n\x0c\n\x05\x04\x01\x02\x01\x03\
    \x12\x03t!\"\n\x0b\n\x04\x04\x01\x02\x02\x12\x03u\x04)\n\x0c\n\x05\x04\
    \x01\x02\x02\x06\x12\x03u\x04\x17\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\
    \x03u\x18$\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03u'(\n\x0b\n\x04\x04\
    \x01\x02\x03\x12\x03v\x04)\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03v\x04\
    \x17\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03v\x18$\n\x0c\n\x05\x04\x01\
    \x02\x03\x03\x12\x03v'(\n\x0b\n\x04\x04\x01\x02\x04\x12\x03w\x04%\n\x0c\
    \n\x05\x04\x01\x02\x04\x06\x12\x03w\x04\x15\n\x0c\n\x05\x04\x01\x02\x04\
    \x01\x12\x03w\x16\x20\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03w#$\n\x0b\n\
    \x04\x04\x01\x02\x05\x12\x03y\x04%\n\x0c\n\x05\x04\x01\x02\x05\x06\x12\
    \x03y\x04\x15\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03y\x16\x20\n\x0c\n\
    \x05\x04\x01\x02\x05\x03\x12\x03y#$\n\x0b\n\x04\x04\x01\x02\x06\x12\x03z\
    \x04%\n\x0c\n\x05\x04\x01\x02\x06\x06\x12\x03z\x04\x15\n\x0c\n\x05\x04\
    \x01\x02\x06\x01\x12\x03z\x16\x20\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\
    \x03z#$\n\x0b\n\x04\x04\x01\x02\x07\x12\x03|\x04\x1a\n\x0c\n\x05\x04\x01\
    \x02\x07\x06\x12\x03|\x04\x10\n\x0c\n\x05\x04\x01\x02\x07\x01\x12\x03|\
    \x11\x15\n\x0c\n\x05\x04\x01\x02\x07\x03\x12\x03|\x18\x19\n\x0b\n\x04\
    \x04\x01\x02\x08\x12\x03~\x04#\n\x0c\n\x05\x04\x01\x02\x08\x06\x12\x03~\
    \x04\x14\n\x0c\n\x05\x04\x01\x02\x08\x01\x12\x03~\x15\x1e\n\x0c\n\x05\
    \x04\x01\x02\x08\x03\x12\x03~!\"\n\x0b\n\x04\x04\x01\x02\t\x12\x03\x7f\
    \x04)\n\x0c\n\x05\x04\x01\x02\t\x06\x12\x03\x7f\x04\x17\n\x0c\n\x05\x04\
    \x01\x02\t\x01\x12\x03\x7f\x18#\n\x0c\n\x05\x04\x01\x02\t\x03\x12\x03\
    \x7f&(\n\x0c\n\x04\x04\x01\x02\n\x12\x04\x80\x01\x04\x1f\n\r\n\x05\x04\
    \x01\x02\n\x06\x12\x04\x80\x01\x04\x12\n\r\n\x05\x04\x01\x02\n\x01\x12\
    \x04\x80\x01\x13\x19\n\r\n\x05\x04\x01\x02\n\x03\x12\x04\x80\x01\x1c\x1e\
    \n\x0c\n\x04\x04\x01\x02\x0b\x12\x04\x81\x01\x04\x1b\n\r\n\x05\x04\x01\
    \x02\x0b\x06\x12\x04\x81\x01\x04\x10\n\r\n\x05\x04\x01\x02\x0b\x01\x12\
    \x04\x81\x01\x11\x15\n\r\n\x05\x04\x01\x02\x0b\x03\x12\x04\x81\x01\x18\
    \x1a\n\x0c\n\x04\x04\x01\x02\x0c\x12\x04\x82\x01\x04\x1b\n\r\n\x05\x04\
    \x01\x02\x0c\x06\x12\x04\x82\x01\x04\x10\n\r\n\x05\x04\x01\x02\x0c\x01\
    \x12\x04\x82\x01\x11\x15\n\r\n\x05\x04\x01\x02\x0c\x03\x12\x04\x82\x01\
    \x18\x1a\n\x0c\n\x04\x04\x01\x02\r\x12\x04\x83\x01\x04\x1d\n\r\n\x05\x04\
    \x01\x02\r\x06\x12\x04\x83\x01\x04\x11\n\r\n\x05\x04\x01\x02\r\x01\x12\
    \x04\x83\x01\x12\x17\n\r\n\x05\x04\x01\x02\r\x03\x12\x04\x83\x01\x1a\x1c\
    \n\x0c\n\x04\x04\x01\x02\x0e\x12\x04\x84\x01\x04(\n\r\n\x05\x04\x01\x02\
    \x0e\x06\x12\x04\x84\x01\x04\x16\n\r\n\x05\x04\x01\x02\x0e\x01\x12\x04\
    \x84\x01\x17\"\n\r\n\x05\x04\x01\x02\x0e\x03\x12\x04\x84\x01%'\n\x0c\n\
    \x04\x04\x01\x02\x0f\x12\x04\x86\x01\x04(\n\r\n\x05\x04\x01\x02\x0f\x06\
    \x12\x04\x86\x01\x04\x16\n\r\n\x05\x04\x01\x02\x0f\x01\x12\x04\x86\x01\
    \x17\"\n\r\n\x05\x04\x01\x02\x0f\x03\x12\x04\x86\x01%'\n\x0c\n\x04\x04\
    \x01\x02\x10\x12\x04\x87\x01\x04.\n\r\n\x05\x04\x01\x02\x10\x06\x12\x04\
    \x87\x01\x04\x19\n\r\n\x05\x04\x01\x02\x10\x01\x12\x04\x87\x01\x1a(\n\r\
    \n\x05\x04\x01\x02\x10\x03\x12\x04\x87\x01+-\n\x0c\n\x04\x04\x01\x02\x11\
    \x12\x04\x88\x01\x04\"\n\r\n\x05\x04\x01\x02\x11\x06\x12\x04\x88\x01\x04\
    \x13\n\r\n\x05\x04\x01\x02\x11\x01\x12\x04\x88\x01\x14\x1c\n\r\n\x05\x04\
    \x01\x02\x11\x03\x12\x04\x88\x01\x1f!\n\x19\n\x04\x04\x01\x02\x12\x12\
    \x04\x8b\x01\x04\x1b\x1a\x0b\x20Debugging\n\n\r\n\x05\x04\x01\x02\x12\
    \x06\x12\x04\x8b\x01\x04\x10\n\r\n\x05\x04\x01\x02\x12\x01\x12\x04\x8b\
    \x01\x11\x15\n\r\n\x05\x04\x01\x02\x12\x03\x12\x04\x8b\x01\x18\x1a\n\x0c\
    \n\x04\x04\x01\x02\x13\x12\x04\x8c\x01\x04\x1d\n\r\n\x05\x04\x01\x02\x13\
    \x06\x12\x04\x8c\x01\x04\x11\n\r\n\x05\x04\x01\x02\x13\x01\x12\x04\x8c\
    \x01\x12\x17\n\r\n\x05\x04\x01\x02\x13\x03\x12\x04\x8c\x01\x1a\x1c\nm\n\
    \x04\x04\x01\x02\x14\x12\x04\x8e\x01\x02\x1d\"_\x20If\x20command\x20is\
    \x20missing,\x20this\x20will\x20contain\x20the\x20error.\x20Otherwise\
    \x20this\x20will\x20contain\x20any\x20warnings.\n\n\r\n\x05\x04\x01\x02\
    \x14\x04\x12\x04\x8e\x01\x02\n\n\r\n\x05\x04\x01\x02\x14\x05\x12\x04\x8e\
    \x01\x0b\x11\n\r\n\x05\x04\x01\x02\x14\x01\x12\x04\x8e\x01\x12\x17\n\r\n\
    \x05\x04\x01\x02\x14\x03\x12\x04\x8e\x01\x1a\x1c\n7\n\x04\x04\x01\x02\
    \x15\x12\x04\x8f\x01\x02\x1e\")\x20Should\x20be\x20sent\x20back\x20with\
    \x20all\x20responses.\n\n\r\n\x05\x04\x01\x02\x15\x04\x12\x04\x8f\x01\
    \x02\n\n\r\n\x05\x04\x01\x02\x15\x06\x12\x04\x8f\x01\x0b\x11\n\r\n\x05\
    \x04\x01\x02\x15\x01\x12\x04\x8f\x01\x12\x18\n\r\n\x05\x04\x01\x02\x15\
    \x03\x12\x04\x8f\x01\x1b\x1d\n\x0c\n\x02\x05\0\x12\x06\x92\x01\0\x9a\x01\
    \x01\n\x0b\n\x03\x05\0\x01\x12\x04\x92\x01\x05\x0b\nC\n\x04\x05\0\x02\0\
    \x12\x04\x93\x01\x02\x0f\"5\x20Game\x20has\x20been\x20launch\x20and\x20i\
    s\x20not\x20yet\x20doing\x20anything.\n\n\r\n\x05\x05\0\x02\0\x01\x12\
    \x04\x93\x01\x02\n\n\r\n\x05\x05\0\x02\0\x02\x12\x04\x93\x01\r\x0e\nN\n\
    \x04\x05\0\x02\x01\x12\x04\x94\x01\x02\x10\"@\x20Create\x20game\x20has\
    \x20been\x20called,\x20and\x20the\x20host\x20is\x20awaiting\x20players.\
    \n\n\r\n\x05\x05\0\x02\x01\x01\x12\x04\x94\x01\x02\x0b\n\r\n\x05\x05\0\
    \x02\x01\x02\x12\x04\x94\x01\x0e\x0f\n0\n\x04\x05\0\x02\x02\x12\x04\x95\
    \x01\x02\x0e\"\"\x20In\x20a\x20single\x20or\x20multiplayer\x20game.\n\n\
    \r\n\x05\x05\0\x02\x02\x01\x12\x04\x95\x01\x02\t\n\r\n\x05\x05\0\x02\x02\
    \x02\x12\x04\x95\x01\x0c\r\n\x1c\n\x04\x05\0\x02\x03\x12\x04\x96\x01\x02\
    \x10\"\x0e\x20In\x20a\x20replay.\n\n\r\n\x05\x05\0\x02\x03\x01\x12\x04\
    \x96\x01\x02\x0b\n\r\n\x05\x05\0\x02\x03\x02\x12\x04\x96\x01\x0e\x0f\nV\
    \n\x04\x05\0\x02\x04\x12\x04\x97\x01\x02\x0c\"H\x20Game\x20has\x20ended,\
    \x20can\x20still\x20request\x20game\x20info,\x20but\x20ready\x20for\x20a\
    \x20new\x20game.\n\n\r\n\x05\x05\0\x02\x04\x01\x12\x04\x97\x01\x02\x07\n\
    \r\n\x05\x05\0\x02\x04\x02\x12\x04\x97\x01\n\x0b\n-\n\x04\x05\0\x02\x05\
    \x12\x04\x98\x01\x02\x0b\"\x1f\x20Application\x20is\x20shutting\x20down.\
    \n\n\r\n\x05\x05\0\x02\x05\x01\x12\x04\x98\x01\x02\x06\n\r\n\x05\x05\0\
    \x02\x05\x02\x12\x04\x98\x01\t\n\nG\n\x04\x05\0\x02\x06\x12\x04\x99\x01\
    \x02\x0f\"9\x20Should\x20not\x20happen,\x20but\x20indicates\x20an\x20err\
    or\x20if\x20it\x20occurs.\n\n\r\n\x05\x05\0\x02\x06\x01\x12\x04\x99\x01\
    \x02\t\n\r\n\x05\x05\0\x02\x06\x02\x12\x04\x99\x01\x0c\x0e\n\xe3\x01\n\
    \x02\x04\x02\x12\x06\x9f\x01\0\xab\x01\x01\x1a\xd4\x01------------------\
    -----------------------------------------------------------\n\x20If\x20s\
    uccessful,\x20puts\x20the\x20game\x20into\x20the\x20status:\x20init_game\
    .\n\x20The\x20next\x20expected\x20request\x20should\x20be\x20RequestJoin\
    Game.\x20Can\x20also\x20quit\x20(exit).\n\n\x0b\n\x03\x04\x02\x01\x12\
    \x04\x9f\x01\x08\x19\n\x0e\n\x04\x04\x02\x08\0\x12\x06\xa0\x01\x02\xa3\
    \x01\x03\n\r\n\x05\x04\x02\x08\0\x01\x12\x04\xa0\x01\x08\x0b\n\"\n\x04\
    \x04\x02\x02\0\x12\x04\xa1\x01\x04\x1b\"\x14\x20Local\x20.SC2Map\x20file\
    \n\n\r\n\x05\x04\x02\x02\0\x06\x12\x04\xa1\x01\x04\x0c\n\r\n\x05\x04\x02\
    \x02\0\x01\x12\x04\xa1\x01\r\x16\n\r\n\x05\x04\x02\x02\0\x03\x12\x04\xa1\
    \x01\x19\x1a\n*\n\x04\x04\x02\x02\x01\x12\x04\xa2\x01\x04\"\"\x1c\x20Map\
    \x20published\x20to\x20BattleNet\n\n\r\n\x05\x04\x02\x02\x01\x05\x12\x04\
    \xa2\x01\x04\n\n\r\n\x05\x04\x02\x02\x01\x01\x12\x04\xa2\x01\x0b\x1d\n\r\
    \n\x05\x04\x02\x02\x01\x03\x12\x04\xa2\x01\x20!\n\x0c\n\x04\x04\x02\x02\
    \x02\x12\x04\xa5\x01\x02(\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\xa5\x01\
    \x02\n\n\r\n\x05\x04\x02\x02\x02\x06\x12\x04\xa5\x01\x0b\x16\n\r\n\x05\
    \x04\x02\x02\x02\x01\x12\x04\xa5\x01\x17#\n\r\n\x05\x04\x02\x02\x02\x03\
    \x12\x04\xa5\x01&'\n\x0c\n\x04\x04\x02\x02\x03\x12\x04\xa7\x01\x02\x20\n\
    \r\n\x05\x04\x02\x02\x03\x04\x12\x04\xa7\x01\x02\n\n\r\n\x05\x04\x02\x02\
    \x03\x05\x12\x04\xa7\x01\x0b\x0f\n\r\n\x05\x04\x02\x02\x03\x01\x12\x04\
    \xa7\x01\x10\x1b\n\r\n\x05\x04\x02\x02\x03\x03\x12\x04\xa7\x01\x1e\x1f\n\
    9\n\x04\x04\x02\x02\x04\x12\x04\xa9\x01\x02\"\"+\x20Sets\x20the\x20pseud\
    o-random\x20seed\x20for\x20the\x20game.\n\n\r\n\x05\x04\x02\x02\x04\x04\
    \x12\x04\xa9\x01\x02\n\n\r\n\x05\x04\x02\x02\x04\x05\x12\x04\xa9\x01\x0b\
    \x11\n\r\n\x05\x04\x02\x02\x04\x01\x12\x04\xa9\x01\x12\x1d\n\r\n\x05\x04\
    \x02\x02\x04\x03\x12\x04\xa9\x01\x20!\n4\n\x04\x04\x02\x02\x05\x12\x04\
    \xaa\x01\x02\x1d\"&\x20If\x20set,\x20the\x20game\x20plays\x20in\x20real\
    \x20time.\n\n\r\n\x05\x04\x02\x02\x05\x04\x12\x04\xaa\x01\x02\n\n\r\n\
    \x05\x04\x02\x02\x05\x05\x12\x04\xaa\x01\x0b\x0f\n\r\n\x05\x04\x02\x02\
    \x05\x01\x12\x04\xaa\x01\x10\x18\n\r\n\x05\x04\x02\x02\x05\x03\x12\x04\
    \xaa\x01\x1b\x1c\n\x0c\n\x02\x04\x03\x12\x06\xad\x01\0\xb3\x01\x01\n\x0b\
    \n\x03\x04\x03\x01\x12\x04\xad\x01\x08\x10\n\xd2\x01\n\x04\x04\x03\x02\0\
    \x12\x04\xb1\x01\x02\x1f\x1a\xc3\x01\x20A\x20map\x20can\x20be\x20specifi\
    ed\x20either\x20by\x20a\x20file\x20path\x20or\x20the\x20data\x20of\x20th\
    e\x20.SC2Map\x20file.\n\x20If\x20you\x20provide\x20both,\x20it\x20will\
    \x20play\x20the\x20game\x20using\x20map_data\x20and\x20store\x20map_path\
    \n\x20into\x20the\x20replay.\x20(260\x20character\x20max)\n\n\r\n\x05\
    \x04\x03\x02\0\x04\x12\x04\xb1\x01\x02\n\n\r\n\x05\x04\x03\x02\0\x05\x12\
    \x04\xb1\x01\x0b\x11\n\r\n\x05\x04\x03\x02\0\x01\x12\x04\xb1\x01\x12\x1a\
    \n\r\n\x05\x04\x03\x02\0\x03\x12\x04\xb1\x01\x1d\x1e\n\x0c\n\x04\x04\x03\
    \x02\x01\x12\x04\xb2\x01\x02\x1e\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04\
    \xb2\x01\x02\n\n\r\n\x05\x04\x03\x02\x01\x05\x12\x04\xb2\x01\x0b\x10\n\r\
    \n\x05\x04\x03\x02\x01\x01\x12\x04\xb2\x01\x11\x19\n\r\n\x05\x04\x03\x02\
    \x01\x03\x12\x04\xb2\x01\x1c\x1d\n\x0c\n\x02\x04\x04\x12\x06\xb5\x01\0\
    \xc2\x01\x01\n\x0b\n\x03\x04\x04\x01\x12\x04\xb5\x01\x08\x1a\n\x0e\n\x04\
    \x04\x04\x04\0\x12\x06\xb6\x01\x02\xbf\x01\x03\n\r\n\x05\x04\x04\x04\0\
    \x01\x12\x04\xb6\x01\x07\x0c\n\x0e\n\x06\x04\x04\x04\0\x02\0\x12\x04\xb7\
    \x01\x04\x13\n\x0f\n\x07\x04\x04\x04\0\x02\0\x01\x12\x04\xb7\x01\x04\x0e\
    \n\x0f\n\x07\x04\x04\x04\0\x02\0\x02\x12\x04\xb7\x01\x11\x12\n\x0e\n\x06\
    \x04\x04\x04\0\x02\x01\x12\x04\xb8\x01\x04\x17\n\x0f\n\x07\x04\x04\x04\0\
    \x02\x01\x01\x12\x04\xb8\x01\x04\x12\n\x0f\n\x07\x04\x04\x04\0\x02\x01\
    \x02\x12\x04\xb8\x01\x15\x16\n\x0e\n\x06\x04\x04\x04\0\x02\x02\x12\x04\
    \xb9\x01\x04\x17\n\x0f\n\x07\x04\x04\x04\0\x02\x02\x01\x12\x04\xb9\x01\
    \x04\x12\n\x0f\n\x07\x04\x04\x04\0\x02\x02\x02\x12\x04\xb9\x01\x15\x16\n\
    \x0e\n\x06\x04\x04\x04\0\x02\x03\x12\x04\xba\x01\x04\x17\n\x0f\n\x07\x04\
    \x04\x04\0\x02\x03\x01\x12\x04\xba\x01\x04\x12\n\x0f\n\x07\x04\x04\x04\0\
    \x02\x03\x02\x12\x04\xba\x01\x15\x16\n\x0e\n\x06\x04\x04\x04\0\x02\x04\
    \x12\x04\xbb\x01\x04\x19\n\x0f\n\x07\x04\x04\x04\0\x02\x04\x01\x12\x04\
    \xbb\x01\x04\x14\n\x0f\n\x07\x04\x04\x04\0\x02\x04\x02\x12\x04\xbb\x01\
    \x17\x18\n\x0e\n\x06\x04\x04\x04\0\x02\x05\x12\x04\xbc\x01\x04\x1b\n\x0f\
    \n\x07\x04\x04\x04\0\x02\x05\x01\x12\x04\xbc\x01\x04\x16\n\x0f\n\x07\x04\
    \x04\x04\0\x02\x05\x02\x12\x04\xbc\x01\x19\x1a\n\x0e\n\x06\x04\x04\x04\0\
    \x02\x06\x12\x04\xbd\x01\x04\x1b\n\x0f\n\x07\x04\x04\x04\0\x02\x06\x01\
    \x12\x04\xbd\x01\x04\x16\n\x0f\n\x07\x04\x04\x04\0\x02\x06\x02\x12\x04\
    \xbd\x01\x19\x1a\nD\n\x06\x04\x04\x04\0\x02\x07\x12\x04\xbe\x01\x04\x1f\
    \"4\x20Multiplayer\x20is\x20not\x20supported\x20in\x20the\x20current\x20\
    build.\n\n\x0f\n\x07\x04\x04\x04\0\x02\x07\x01\x12\x04\xbe\x01\x04\x1a\n\
    \x0f\n\x07\x04\x04\x04\0\x02\x07\x02\x12\x04\xbe\x01\x1d\x1e\n\x0c\n\x04\
    \x04\x04\x02\0\x12\x04\xc0\x01\x02\x1b\n\r\n\x05\x04\x04\x02\0\x04\x12\
    \x04\xc0\x01\x02\n\n\r\n\x05\x04\x04\x02\0\x06\x12\x04\xc0\x01\x0b\x10\n\
    \r\n\x05\x04\x04\x02\0\x01\x12\x04\xc0\x01\x11\x16\n\r\n\x05\x04\x04\x02\
    \0\x03\x12\x04\xc0\x01\x19\x1a\n\x0c\n\x04\x04\x04\x02\x01\x12\x04\xc1\
    \x01\x02$\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04\xc1\x01\x02\n\n\r\n\x05\
    \x04\x04\x02\x01\x05\x12\x04\xc1\x01\x0b\x11\n\r\n\x05\x04\x04\x02\x01\
    \x01\x12\x04\xc1\x01\x12\x1f\n\r\n\x05\x04\x04\x02\x01\x03\x12\x04\xc1\
    \x01\"#\n\xd7\x01\n\x02\x04\x05\x12\x06\xc7\x01\0\xd2\x01\x01\x1a\xc8\
    \x01--------------------------------------------------------------------\
    ---------\n\x20If\x20successful,\x20puts\x20the\x20game\x20into\x20the\
    \x20status:\x20in_game.\x20Will\x20be\x20able\x20to\n\x20request\x20acti\
    ons,\x20observations\x20and\x20step\x20the\x20game.\n\n\x0b\n\x03\x04\
    \x05\x01\x12\x04\xc7\x01\x08\x17\n\x0e\n\x04\x04\x05\x08\0\x12\x06\xc8\
    \x01\x02\xcb\x01\x03\n\r\n\x05\x04\x05\x08\0\x01\x12\x04\xc8\x01\x08\x15\
    \n#\n\x04\x04\x05\x02\0\x12\x04\xc9\x01\x04\x12\"\x15\x20Join\x20as\x20p\
    articipant\n\n\r\n\x05\x04\x05\x02\0\x06\x12\x04\xc9\x01\x04\x08\n\r\n\
    \x05\x04\x05\x02\0\x01\x12\x04\xc9\x01\t\r\n\r\n\x05\x04\x05\x02\0\x03\
    \x12\x04\xc9\x01\x10\x11\n\x20\n\x04\x04\x05\x02\x01\x12\x04\xca\x01\x04\
    \"\"\x12\x20Join\x20as\x20observer\n\n\r\n\x05\x04\x05\x02\x01\x05\x12\
    \x04\xca\x01\x04\n\n\r\n\x05\x04\x05\x02\x01\x01\x12\x04\xca\x01\x0b\x1d\
    \n\r\n\x05\x04\x05\x02\x01\x03\x12\x04\xca\x01\x20!\n|\n\x04\x04\x05\x02\
    \x02\x12\x04\xcd\x01\x02(\"n\x20This\x20is\x20limited\x20to\x20what\x20i\
    s\x20specified\x20in\x20RequestCreateGame,\x20but\x20you\x20can\x20reque\
    st\x20less\x20information\x20if\x20you\x20want.\n\n\r\n\x05\x04\x05\x02\
    \x02\x04\x12\x04\xcd\x01\x02\n\n\r\n\x05\x04\x05\x02\x02\x06\x12\x04\xcd\
    \x01\x0b\x1b\n\r\n\x05\x04\x05\x02\x02\x01\x12\x04\xcd\x01\x1c#\n\r\n\
    \x05\x04\x05\x02\x02\x03\x12\x04\xcd\x01&'\nY\n\x04\x04\x05\x02\x03\x12\
    \x04\xce\x01\x02$\"K\x20Do\x20not\x20set\x20in\x20the\x20single-player\
    \x20case.\x20This\x20is\x20the\x20port\x20a\x20server\x20will\x20use.\n\
    \n\r\n\x05\x04\x05\x02\x03\x04\x12\x04\xce\x01\x02\n\n\r\n\x05\x04\x05\
    \x02\x03\x06\x12\x04\xce\x01\x0b\x12\n\r\n\x05\x04\x05\x02\x03\x01\x12\
    \x04\xce\x01\x13\x1f\n\r\n\x05\x04\x05\x02\x03\x03\x12\x04\xce\x01\"#\nw\
    \n\x04\x04\x05\x02\x04\x12\x04\xcf\x01\x02$\"i\x20Do\x20not\x20set\x20in\
    \x20the\x20single-player\x20case.\x20These\x20are\x20the\x20ports\x20cli\
    ents\x20will\x20use\x20to\x20initialize\x20communication.\n\n\r\n\x05\
    \x04\x05\x02\x04\x04\x12\x04\xcf\x01\x02\n\n\r\n\x05\x04\x05\x02\x04\x06\
    \x12\x04\xcf\x01\x0b\x12\n\r\n\x05\x04\x05\x02\x04\x01\x12\x04\xcf\x01\
    \x13\x1f\n\r\n\x05\x04\x05\x02\x04\x03\x12\x04\xcf\x01\"#\n;\n\x04\x04\
    \x05\x02\x05\x12\x04\xd1\x01\x02!\x1a-\x20Currently\x20only\x20a\x20sing\
    e\x20client\x20is\x20supported.\n\n\r\n\x05\x04\x05\x02\x05\x04\x12\x04\
    \xd1\x01\x02\n\n\r\n\x05\x04\x05\x02\x05\x05\x12\x04\xd1\x01\x0b\x10\n\r\
    \n\x05\x04\x05\x02\x05\x01\x12\x04\xd1\x01\x11\x1c\n\r\n\x05\x04\x05\x02\
    \x05\x03\x12\x04\xd1\x01\x1f\x20\n\x0c\n\x02\x04\x06\x12\x06\xd4\x01\0\
    \xd7\x01\x01\n\x0b\n\x03\x04\x06\x01\x12\x04\xd4\x01\x08\x0f\nh\n\x04\
    \x04\x06\x02\0\x12\x04\xd5\x01\x02\x1f\"Z\x20Game\x20right\x20now\x20nee\
    ds\x20two\x20internal\x20ports\x20to\x20establish\x20a\x20multiplay\x20g\
    ame\x20on\x20the\x20local\x20host.\n\n\r\n\x05\x04\x06\x02\0\x04\x12\x04\
    \xd5\x01\x02\n\n\r\n\x05\x04\x06\x02\0\x05\x12\x04\xd5\x01\x0b\x10\n\r\n\
    \x05\x04\x06\x02\0\x01\x12\x04\xd5\x01\x11\x1a\n\r\n\x05\x04\x06\x02\0\
    \x03\x12\x04\xd5\x01\x1d\x1e\n\x0c\n\x04\x04\x06\x02\x01\x12\x04\xd6\x01\
    \x02\x1f\n\r\n\x05\x04\x06\x02\x01\x04\x12\x04\xd6\x01\x02\n\n\r\n\x05\
    \x04\x06\x02\x01\x05\x12\x04\xd6\x01\x0b\x10\n\r\n\x05\x04\x06\x02\x01\
    \x01\x12\x04\xd6\x01\x11\x1a\n\r\n\x05\x04\x06\x02\x01\x03\x12\x04\xd6\
    \x01\x1d\x1e\n\x0c\n\x02\x04\x07\x12\x06\xd9\x01\0\xef\x01\x01\n\x0b\n\
    \x03\x04\x07\x01\x12\x04\xd9\x01\x08\x18\n\x0c\n\x04\x04\x07\x02\0\x12\
    \x04\xda\x01\x02\x20\n\r\n\x05\x04\x07\x02\0\x04\x12\x04\xda\x01\x02\n\n\
    \r\n\x05\x04\x07\x02\0\x05\x12\x04\xda\x01\x0b\x11\n\r\n\x05\x04\x07\x02\
    \0\x01\x12\x04\xda\x01\x12\x1b\n\r\n\x05\x04\x07\x02\0\x03\x12\x04\xda\
    \x01\x1e\x1f\n\x0e\n\x04\x04\x07\x04\0\x12\x06\xdc\x01\x02\xec\x01\x03\n\
    \r\n\x05\x04\x07\x04\0\x01\x12\x04\xdc\x01\x07\x0c\n\x0e\n\x06\x04\x07\
    \x04\0\x02\0\x12\x04\xdd\x01\x04\x1d\n\x0f\n\x07\x04\x07\x04\0\x02\0\x01\
    \x12\x04\xdd\x01\x04\x18\n\x0f\n\x07\x04\x07\x04\0\x02\0\x02\x12\x04\xdd\
    \x01\x1b\x1c\n\x0e\n\x06\x04\x07\x04\0\x02\x01\x12\x04\xde\x01\x04\x20\n\
    \x0f\n\x07\x04\x07\x04\0\x02\x01\x01\x12\x04\xde\x01\x04\x1b\n\x0f\n\x07\
    \x04\x07\x04\0\x02\x01\x02\x12\x04\xde\x01\x1e\x1f\n\x0e\n\x06\x04\x07\
    \x04\0\x02\x02\x12\x04\xdf\x01\x04\x17\n\x0f\n\x07\x04\x07\x04\0\x02\x02\
    \x01\x12\x04\xdf\x01\x04\x12\n\x0f\n\x07\x04\x07\x04\0\x02\x02\x02\x12\
    \x04\xdf\x01\x15\x16\n\x0e\n\x06\x04\x07\x04\0\x02\x03\x12\x04\xe0\x01\
    \x04\x15\n\x0f\n\x07\x04\x07\x04\0\x02\x03\x01\x12\x04\xe0\x01\x04\x10\n\
    \x0f\n\x07\x04\x07\x04\0\x02\x03\x02\x12\x04\xe0\x01\x13\x14\n\x0e\n\x06\
    \x04\x07\x04\0\x02\x04\x12\x04\xe1\x01\x04\x11\n\x0f\n\x07\x04\x07\x04\0\
    \x02\x04\x01\x12\x04\xe1\x01\x04\x0c\n\x0f\n\x07\x04\x07\x04\0\x02\x04\
    \x02\x12\x04\xe1\x01\x0f\x10\n\x0e\n\x06\x04\x07\x04\0\x02\x05\x12\x04\
    \xe2\x01\x04\x14\n\x0f\n\x07\x04\x07\x04\0\x02\x05\x01\x12\x04\xe2\x01\
    \x04\x0f\n\x0f\n\x07\x04\x07\x04\0\x02\x05\x02\x12\x04\xe2\x01\x12\x13\n\
    x\n\x06\x04\x07\x04\0\x02\x06\x12\x04\xe5\x01\x04\x1b\x1a\x17\x20Multipl\
    ayer\x20specific.\n\"O\x20Multiplayer\x20is\x20not\x20supported\x20in\
    \x20the\x20current\x20build\x20for\x20the\x20requested\x20features.\n\n\
    \x0f\n\x07\x04\x07\x04\0\x02\x06\x01\x12\x04\xe5\x01\x04\x16\n\x0f\n\x07\
    \x04\x07\x04\0\x02\x06\x02\x12\x04\xe5\x01\x19\x1a\n\x0e\n\x06\x04\x07\
    \x04\0\x02\x07\x12\x04\xe6\x01\x04\x17\n\x0f\n\x07\x04\x07\x04\0\x02\x07\
    \x01\x12\x04\xe6\x01\x04\x12\n\x0f\n\x07\x04\x07\x04\0\x02\x07\x02\x12\
    \x04\xe6\x01\x15\x16\n\x0e\n\x06\x04\x07\x04\0\x02\x08\x12\x04\xe7\x01\
    \x04\x18\n\x0f\n\x07\x04\x07\x04\0\x02\x08\x01\x12\x04\xe7\x01\x04\x13\n\
    \x0f\n\x07\x04\x07\x04\0\x02\x08\x02\x12\x04\xe7\x01\x16\x17\n\x0e\n\x06\
    \x04\x07\x04\0\x02\t\x12\x04\xe8\x01\x04\x17\n\x0f\n\x07\x04\x07\x04\0\
    \x02\t\x01\x12\x04\xe8\x01\x04\x11\n\x0f\n\x07\x04\x07\x04\0\x02\t\x02\
    \x12\x04\xe8\x01\x14\x16\n\x0e\n\x06\x04\x07\x04\0\x02\n\x12\x04\xe9\x01\
    \x04\x17\n\x0f\n\x07\x04\x07\x04\0\x02\n\x01\x12\x04\xe9\x01\x04\x11\n\
    \x0f\n\x07\x04\x07\x04\0\x02\n\x02\x12\x04\xe9\x01\x14\x16\n\x0e\n\x06\
    \x04\x07\x04\0\x02\x0b\x12\x04\xea\x01\x04\x16\n\x0f\n\x07\x04\x07\x04\0\
    \x02\x0b\x01\x12\x04\xea\x01\x04\x10\n\x0f\n\x07\x04\x07\x04\0\x02\x0b\
    \x02\x12\x04\xea\x01\x13\x15\n\x0e\n\x06\x04\x07\x04\0\x02\x0c\x12\x04\
    \xeb\x01\x04\x14\n\x0f\n\x07\x04\x07\x04\0\x02\x0c\x01\x12\x04\xeb\x01\
    \x04\x0e\n\x0f\n\x07\x04\x07\x04\0\x02\x0c\x02\x12\x04\xeb\x01\x11\x13\n\
    \x0c\n\x04\x04\x07\x02\x01\x12\x04\xed\x01\x02\x1b\n\r\n\x05\x04\x07\x02\
    \x01\x04\x12\x04\xed\x01\x02\n\n\r\n\x05\x04\x07\x02\x01\x06\x12\x04\xed\
    \x01\x0b\x10\n\r\n\x05\x04\x07\x02\x01\x01\x12\x04\xed\x01\x11\x16\n\r\n\
    \x05\x04\x07\x02\x01\x03\x12\x04\xed\x01\x19\x1a\n\x0c\n\x04\x04\x07\x02\
    \x02\x12\x04\xee\x01\x02$\n\r\n\x05\x04\x07\x02\x02\x04\x12\x04\xee\x01\
    \x02\n\n\r\n\x05\x04\x07\x02\x02\x05\x12\x04\xee\x01\x0b\x11\n\r\n\x05\
    \x04\x07\x02\x02\x01\x12\x04\xee\x01\x12\x1f\n\r\n\x05\x04\x07\x02\x02\
    \x03\x12\x04\xee\x01\"#\n\\\n\x02\x04\x08\x12\x06\xf2\x01\0\xf3\x01\x01\
    \x1aN-------------------------------------------------------------------\
    ----------\n\n\x0b\n\x03\x04\x08\x01\x12\x04\xf2\x01\x08\x1a\n\x0c\n\x02\
    \x04\t\x12\x06\xf5\x01\0\xfb\x01\x01\n\x0b\n\x03\x04\t\x01\x12\x04\xf5\
    \x01\x08\x1b\n\x0e\n\x04\x04\t\x04\0\x12\x06\xf6\x01\x02\xf8\x01\x03\n\r\
    \n\x05\x04\t\x04\0\x01\x12\x04\xf6\x01\x07\x0c\n\x0e\n\x06\x04\t\x04\0\
    \x02\0\x12\x04\xf7\x01\x04\x14\n\x0f\n\x07\x04\t\x04\0\x02\0\x01\x12\x04\
    \xf7\x01\x04\x0f\n\x0f\n\x07\x04\t\x04\0\x02\0\x02\x12\x04\xf7\x01\x12\
    \x13\n\x0c\n\x04\x04\t\x02\0\x12\x04\xf9\x01\x02\x1b\n\r\n\x05\x04\t\x02\
    \0\x04\x12\x04\xf9\x01\x02\n\n\r\n\x05\x04\t\x02\0\x06\x12\x04\xf9\x01\
    \x0b\x10\n\r\n\x05\x04\t\x02\0\x01\x12\x04\xf9\x01\x11\x16\n\r\n\x05\x04\
    \t\x02\0\x03\x12\x04\xf9\x01\x19\x1a\n\x0c\n\x04\x04\t\x02\x01\x12\x04\
    \xfa\x01\x02$\n\r\n\x05\x04\t\x02\x01\x04\x12\x04\xfa\x01\x02\n\n\r\n\
    \x05\x04\t\x02\x01\x05\x12\x04\xfa\x01\x0b\x11\n\r\n\x05\x04\t\x02\x01\
    \x01\x12\x04\xfa\x01\x12\x1f\n\r\n\x05\x04\t\x02\x01\x03\x12\x04\xfa\x01\
    \"#\n\\\n\x02\x04\n\x12\x06\xfe\x01\0\x88\x02\x01\x1aN------------------\
    -----------------------------------------------------------\n\n\x0b\n\
    \x03\x04\n\x01\x12\x04\xfe\x01\x08\x1a\n\x0e\n\x04\x04\n\x08\0\x12\x06\
    \xff\x01\x02\x82\x02\x03\n\r\n\x05\x04\n\x08\0\x01\x12\x04\xff\x01\x08\
    \x0e\n\x0c\n\x04\x04\n\x02\0\x12\x04\x80\x02\x04\x1b\n\r\n\x05\x04\n\x02\
    \0\x05\x12\x04\x80\x02\x04\n\n\r\n\x05\x04\n\x02\0\x01\x12\x04\x80\x02\
    \x0b\x16\n\r\n\x05\x04\n\x02\0\x03\x12\x04\x80\x02\x19\x1a\n\x0c\n\x04\
    \x04\n\x02\x01\x12\x04\x81\x02\x04\x1a\n\r\n\x05\x04\n\x02\x01\x05\x12\
    \x04\x81\x02\x04\t\n\r\n\x05\x04\n\x02\x01\x01\x12\x04\x81\x02\n\x15\n\r\
    \n\x05\x04\n\x02\x01\x03\x12\x04\x81\x02\x18\x19\n<\n\x04\x04\n\x02\x02\
    \x12\x04\x83\x02\x02\x1e\".\x20Overrides\x20the\x20map\x20path\x20stored\
    \x20in\x20the\x20replay.\n\n\r\n\x05\x04\n\x02\x02\x04\x12\x04\x83\x02\
    \x02\n\n\r\n\x05\x04\n\x02\x02\x05\x12\x04\x83\x02\x0b\x10\n\r\n\x05\x04\
    \n\x02\x02\x01\x12\x04\x83\x02\x11\x19\n\r\n\x05\x04\n\x02\x02\x03\x12\
    \x04\x83\x02\x1c\x1d\n\x0c\n\x04\x04\n\x02\x03\x12\x04\x84\x02\x02(\n\r\
    \n\x05\x04\n\x02\x03\x04\x12\x04\x84\x02\x02\n\n\r\n\x05\x04\n\x02\x03\
    \x05\x12\x04\x84\x02\x0b\x10\n\r\n\x05\x04\n\x02\x03\x01\x12\x04\x84\x02\
    \x11#\n\r\n\x05\x04\n\x02\x03\x03\x12\x04\x84\x02&'\n\x0c\n\x04\x04\n\
    \x02\x04\x12\x04\x85\x02\x02(\n\r\n\x05\x04\n\x02\x04\x04\x12\x04\x85\
    \x02\x02\n\n\r\n\x05\x04\n\x02\x04\x06\x12\x04\x85\x02\x0b\x1b\n\r\n\x05\
    \x04\n\x02\x04\x01\x12\x04\x85\x02\x1c#\n\r\n\x05\x04\n\x02\x04\x03\x12\
    \x04\x85\x02&'\n\x0c\n\x04\x04\n\x02\x05\x12\x04\x87\x02\x02\x20\n\r\n\
    \x05\x04\n\x02\x05\x04\x12\x04\x87\x02\x02\n\n\r\n\x05\x04\n\x02\x05\x05\
    \x12\x04\x87\x02\x0b\x0f\n\r\n\x05\x04\n\x02\x05\x01\x12\x04\x87\x02\x10\
    \x1b\n\r\n\x05\x04\n\x02\x05\x03\x12\x04\x87\x02\x1e\x1f\n\x0c\n\x02\x04\
    \x0b\x12\x06\x8a\x02\0\x96\x02\x01\n\x0b\n\x03\x04\x0b\x01\x12\x04\x8a\
    \x02\x08\x1b\n\x0e\n\x04\x04\x0b\x04\0\x12\x06\x8b\x02\x02\x93\x02\x03\n\
    \r\n\x05\x04\x0b\x04\0\x01\x12\x04\x8b\x02\x07\x0c\n\x0e\n\x06\x04\x0b\
    \x04\0\x02\0\x12\x04\x8c\x02\x04\x16\n\x0f\n\x07\x04\x0b\x04\0\x02\0\x01\
    \x12\x04\x8c\x02\x04\x11\n\x0f\n\x07\x04\x0b\x04\0\x02\0\x02\x12\x04\x8c\
    \x02\x14\x15\n\x0e\n\x06\x04\x0b\x04\0\x02\x01\x12\x04\x8d\x02\x04\x1a\n\
    \x0f\n\x07\x04\x0b\x04\0\x02\x01\x01\x12\x04\x8d\x02\x04\x15\n\x0f\n\x07\
    \x04\x0b\x04\0\x02\x01\x02\x12\x04\x8d\x02\x18\x19\n\x0e\n\x06\x04\x0b\
    \x04\0\x02\x02\x12\x04\x8e\x02\x04\x1a\n\x0f\n\x07\x04\x0b\x04\0\x02\x02\
    \x01\x12\x04\x8e\x02\x04\x15\n\x0f\n\x07\x04\x0b\x04\0\x02\x02\x02\x12\
    \x04\x8e\x02\x18\x19\n\x0e\n\x06\x04\x0b\x04\0\x02\x03\x12\x04\x8f\x02\
    \x04\x17\n\x0f\n\x07\x04\x0b\x04\0\x02\x03\x01\x12\x04\x8f\x02\x04\x12\n\
    \x0f\n\x07\x04\x0b\x04\0\x02\x03\x02\x12\x04\x8f\x02\x15\x16\n\x0e\n\x06\
    \x04\x0b\x04\0\x02\x04\x12\x04\x90\x02\x04\x20\n\x0f\n\x07\x04\x0b\x04\0\
    \x02\x04\x01\x12\x04\x90\x02\x04\x1b\n\x0f\n\x07\x04\x0b\x04\0\x02\x04\
    \x02\x12\x04\x90\x02\x1e\x1f\n\x0e\n\x06\x04\x0b\x04\0\x02\x05\x12\x04\
    \x91\x02\x04\x17\n\x0f\n\x07\x04\x0b\x04\0\x02\x05\x01\x12\x04\x91\x02\
    \x04\x12\n\x0f\n\x07\x04\x0b\x04\0\x02\x05\x02\x12\x04\x91\x02\x15\x16\n\
    \x0e\n\x06\x04\x0b\x04\0\x02\x06\x12\x04\x92\x02\x04\x14\n\x0f\n\x07\x04\
    \x0b\x04\0\x02\x06\x01\x12\x04\x92\x02\x04\x0f\n\x0f\n\x07\x04\x0b\x04\0\
    \x02\x06\x02\x12\x04\x92\x02\x12\x13\n\x0c\n\x04\x04\x0b\x02\0\x12\x04\
    \x94\x02\x02\x1b\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04\x94\x02\x02\n\n\r\n\
    \x05\x04\x0b\x02\0\x06\x12\x04\x94\x02\x0b\x10\n\r\n\x05\x04\x0b\x02\0\
    \x01\x12\x04\x94\x02\x11\x16\n\r\n\x05\x04\x0b\x02\0\x03\x12\x04\x94\x02\
    \x19\x1a\n\x0c\n\x04\x04\x0b\x02\x01\x12\x04\x95\x02\x02$\n\r\n\x05\x04\
    \x0b\x02\x01\x04\x12\x04\x95\x02\x02\n\n\r\n\x05\x04\x0b\x02\x01\x05\x12\
    \x04\x95\x02\x0b\x11\n\r\n\x05\x04\x0b\x02\x01\x01\x12\x04\x95\x02\x12\
    \x1f\n\r\n\x05\x04\x0b\x02\x01\x03\x12\x04\x95\x02\"#\n\\\n\x02\x04\x0c\
    \x12\x06\x99\x02\0\x9a\x02\x01\x1aN-------------------------------------\
    ----------------------------------------\n\n\x0b\n\x03\x04\x0c\x01\x12\
    \x04\x99\x02\x08\x18\n\x0c\n\x02\x04\r\x12\x06\x9c\x02\0\x9d\x02\x01\n\
    \x0b\n\x03\x04\r\x01\x12\x04\x9c\x02\x08\x19\n\\\n\x02\x04\x0e\x12\x06\
    \xa0\x02\0\xa1\x02\x01\x1aN---------------------------------------------\
    --------------------------------\n\n\x0b\n\x03\x04\x0e\x01\x12\x04\xa0\
    \x02\x08\x18\n\\\n\x02\x04\x0f\x12\x06\xa4\x02\0\xa5\x02\x01\x1aN-------\
    ----------------------------------------------------------------------\n\
    \n\x0b\n\x03\x04\x0f\x01\x12\x04\xa4\x02\x08\x19\n\\\n\x02\x04\x10\x12\
    \x06\xa8\x02\0\xa9\x02\x01\x1aN-----------------------------------------\
    ------------------------------------\n\n\x0b\n\x03\x04\x10\x01\x12\x04\
    \xa8\x02\x08\x18\n\\\n\x02\x04\x11\x12\x06\xac\x02\0\xad\x02\x01\x1aN---\
    ------------------------------------------------------------------------\
    --\n\n\x0b\n\x03\x04\x11\x01\x12\x04\xac\x02\x08\x19\n\\\n\x02\x04\x12\
    \x12\x06\xb0\x02\0\xb1\x02\x01\x1aN-------------------------------------\
    ----------------------------------------\n\n\x0b\n\x03\x04\x12\x01\x12\
    \x04\xb0\x02\x08\x13\n\x0c\n\x02\x04\x13\x12\x06\xb3\x02\0\xb4\x02\x01\n\
    \x0b\n\x03\x04\x13\x01\x12\x04\xb3\x02\x08\x14\n\\\n\x02\x04\x14\x12\x06\
    \xb7\x02\0\xb8\x02\x01\x1aN---------------------------------------------\
    --------------------------------\n\n\x0b\n\x03\x04\x14\x01\x12\x04\xb7\
    \x02\x08\x17\n\x0c\n\x02\x04\x15\x12\x06\xba\x02\0\xc1\x02\x01\n\x0b\n\
    \x03\x04\x15\x01\x12\x04\xba\x02\x08\x18\n\x0c\n\x04\x04\x15\x02\0\x12\
    \x04\xbb\x02\x02\x1f\n\r\n\x05\x04\x15\x02\0\x04\x12\x04\xbb\x02\x02\n\n\
    \r\n\x05\x04\x15\x02\0\x05\x12\x04\xbb\x02\x0b\x11\n\r\n\x05\x04\x15\x02\
    \0\x01\x12\x04\xbb\x02\x12\x1a\n\r\n\x05\x04\x15\x02\0\x03\x12\x04\xbb\
    \x02\x1d\x1e\n\x0c\n\x04\x04\x15\x02\x01\x12\x04\xbc\x02\x02\x20\n\r\n\
    \x05\x04\x15\x02\x01\x04\x12\x04\xbc\x02\x02\n\n\r\n\x05\x04\x15\x02\x01\
    \x05\x12\x04\xbc\x02\x0b\x11\n\r\n\x05\x04\x15\x02\x01\x01\x12\x04\xbc\
    \x02\x12\x1b\n\r\n\x05\x04\x15\x02\x01\x03\x12\x04\xbc\x02\x1e\x1f\n\x0c\
    \n\x04\x04\x15\x02\x02\x12\x04\xbd\x02\x02%\n\r\n\x05\x04\x15\x02\x02\
    \x04\x12\x04\xbd\x02\x02\n\n\r\n\x05\x04\x15\x02\x02\x05\x12\x04\xbd\x02\
    \x0b\x11\n\r\n\x05\x04\x15\x02\x02\x01\x12\x04\xbd\x02\x12\x20\n\r\n\x05\
    \x04\x15\x02\x02\x03\x12\x04\xbd\x02#$\n\x0c\n\x04\x04\x15\x02\x03\x12\
    \x04\xbe\x02\x02&\n\r\n\x05\x04\x15\x02\x03\x04\x12\x04\xbe\x02\x02\n\n\
    \r\n\x05\x04\x15\x02\x03\x06\x12\x04\xbe\x02\x0b\x15\n\r\n\x05\x04\x15\
    \x02\x03\x01\x12\x04\xbe\x02\x16!\n\r\n\x05\x04\x15\x02\x03\x03\x12\x04\
    \xbe\x02$%\n6\n\x04\x04\x15\x02\x04\x12\x04\xbf\x02\x02\"\"(\x20Populate\
    d\x20if\x20Raw\x20interface\x20is\x20enabled.\n\n\r\n\x05\x04\x15\x02\
    \x04\x04\x12\x04\xbf\x02\x02\n\n\r\n\x05\x04\x15\x02\x04\x06\x12\x04\xbf\
    \x02\x0b\x13\n\r\n\x05\x04\x15\x02\x04\x01\x12\x04\xbf\x02\x14\x1d\n\r\n\
    \x05\x04\x15\x02\x04\x03\x12\x04\xbf\x02\x20!\n\x0c\n\x04\x04\x15\x02\
    \x05\x12\x04\xc0\x02\x02(\n\r\n\x05\x04\x15\x02\x05\x04\x12\x04\xc0\x02\
    \x02\n\n\r\n\x05\x04\x15\x02\x05\x06\x12\x04\xc0\x02\x0b\x1b\n\r\n\x05\
    \x04\x15\x02\x05\x01\x12\x04\xc0\x02\x1c#\n\r\n\x05\x04\x15\x02\x05\x03\
    \x12\x04\xc0\x02&'\n\\\n\x02\x04\x16\x12\x06\xc4\x02\0\xc6\x02\x01\x1aN-\
    ------------------------------------------------------------------------\
    ----\n\n\x0b\n\x03\x04\x16\x01\x12\x04\xc4\x02\x08\x1a\n\x0c\n\x04\x04\
    \x16\x02\0\x12\x04\xc5\x02\x02\x20\n\r\n\x05\x04\x16\x02\0\x04\x12\x04\
    \xc5\x02\x02\n\n\r\n\x05\x04\x16\x02\0\x05\x12\x04\xc5\x02\x0b\x0f\n\r\n\
    \x05\x04\x16\x02\0\x01\x12\x04\xc5\x02\x10\x1b\n\r\n\x05\x04\x16\x02\0\
    \x03\x12\x04\xc5\x02\x1e\x1f\n\x0c\n\x02\x04\x17\x12\x06\xc8\x02\0\xce\
    \x02\x01\n\x0b\n\x03\x04\x17\x01\x12\x04\xc8\x02\x08\x1b\nC\n\x04\x04\
    \x17\x02\0\x12\x04\xc9\x02\x02\x1e\"5\x20Actions\x20this\x20player\x20di\
    d\x20since\x20the\x20last\x20Observation.\n\n\r\n\x05\x04\x17\x02\0\x04\
    \x12\x04\xc9\x02\x02\n\n\r\n\x05\x04\x17\x02\0\x06\x12\x04\xc9\x02\x0b\
    \x11\n\r\n\x05\x04\x17\x02\0\x01\x12\x04\xc9\x02\x12\x19\n\r\n\x05\x04\
    \x17\x02\0\x03\x12\x04\xc9\x02\x1c\x1d\n3\n\x04\x04\x17\x02\x01\x12\x04\
    \xca\x02\x02)\"%\x20Equivalent\x20of\x20UI\x20\"red\x20text\"\x20errors.\
    \n\n\r\n\x05\x04\x17\x02\x01\x04\x12\x04\xca\x02\x02\n\n\r\n\x05\x04\x17\
    \x02\x01\x06\x12\x04\xca\x02\x0b\x16\n\r\n\x05\x04\x17\x02\x01\x01\x12\
    \x04\xca\x02\x17$\n\r\n\x05\x04\x17\x02\x01\x03\x12\x04\xca\x02'(\n\x0c\
    \n\x04\x04\x17\x02\x02\x12\x04\xcb\x02\x02'\n\r\n\x05\x04\x17\x02\x02\
    \x04\x12\x04\xcb\x02\x02\n\n\r\n\x05\x04\x17\x02\x02\x06\x12\x04\xcb\x02\
    \x0b\x16\n\r\n\x05\x04\x17\x02\x02\x01\x12\x04\xcb\x02\x17\"\n\r\n\x05\
    \x04\x17\x02\x02\x03\x12\x04\xcb\x02%&\nB\n\x04\x04\x17\x02\x03\x12\x04\
    \xcc\x02\x02*\"4\x20Only\x20populated\x20if\x20the\x20game\x20ended\x20d\
    uring\x20this\x20step.\n\n\r\n\x05\x04\x17\x02\x03\x04\x12\x04\xcc\x02\
    \x02\n\n\r\n\x05\x04\x17\x02\x03\x06\x12\x04\xcc\x02\x0b\x17\n\r\n\x05\
    \x04\x17\x02\x03\x01\x12\x04\xcc\x02\x18%\n\r\n\x05\x04\x17\x02\x03\x03\
    \x12\x04\xcc\x02()\n\x0c\n\x04\x04\x17\x02\x04\x12\x04\xcd\x02\x02!\n\r\
    \n\x05\x04\x17\x02\x04\x04\x12\x04\xcd\x02\x02\n\n\r\n\x05\x04\x17\x02\
    \x04\x06\x12\x04\xcd\x02\x0b\x17\n\r\n\x05\x04\x17\x02\x04\x01\x12\x04\
    \xcd\x02\x18\x1c\n\r\n\x05\x04\x17\x02\x04\x03\x12\x04\xcd\x02\x1f\x20\n\
    \x0c\n\x02\x04\x18\x12\x06\xd0\x02\0\xd3\x02\x01\n\x0b\n\x03\x04\x18\x01\
    \x12\x04\xd0\x02\x08\x14\n\x0c\n\x04\x04\x18\x02\0\x12\x04\xd1\x02\x02\
    \x1f\n\r\n\x05\x04\x18\x02\0\x04\x12\x04\xd1\x02\x02\n\n\r\n\x05\x04\x18\
    \x02\0\x05\x12\x04\xd1\x02\x0b\x10\n\r\n\x05\x04\x18\x02\0\x01\x12\x04\
    \xd1\x02\x11\x1a\n\r\n\x05\x04\x18\x02\0\x03\x12\x04\xd1\x02\x1d\x1e\n\
    \x0c\n\x04\x04\x18\x02\x01\x12\x04\xd2\x02\x02\x1e\n\r\n\x05\x04\x18\x02\
    \x01\x04\x12\x04\xd2\x02\x02\n\n\r\n\x05\x04\x18\x02\x01\x05\x12\x04\xd2\
    \x02\x0b\x11\n\r\n\x05\x04\x18\x02\x01\x01\x12\x04\xd2\x02\x12\x19\n\r\n\
    \x05\x04\x18\x02\x01\x03\x12\x04\xd2\x02\x1c\x1d\n\\\n\x02\x04\x19\x12\
    \x06\xd6\x02\0\xd8\x02\x01\x1aN-----------------------------------------\
    ------------------------------------\n\n\x0b\n\x03\x04\x19\x01\x12\x04\
    \xd6\x02\x08\x15\n\x0c\n\x04\x04\x19\x02\0\x12\x04\xd7\x02\x03\x1f\n\r\n\
    \x05\x04\x19\x02\0\x04\x12\x04\xd7\x02\x03\x0b\n\r\n\x05\x04\x19\x02\0\
    \x06\x12\x04\xd7\x02\x0c\x12\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\xd7\x02\
    \x13\x1a\n\r\n\x05\x04\x19\x02\0\x03\x12\x04\xd7\x02\x1d\x1e\n\x0c\n\x02\
    \x04\x1a\x12\x06\xda\x02\0\xdc\x02\x01\n\x0b\n\x03\x04\x1a\x01\x12\x04\
    \xda\x02\x08\x16\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\xdb\x02\x03$\n\r\n\
    \x05\x04\x1a\x02\0\x04\x12\x04\xdb\x02\x03\x0b\n\r\n\x05\x04\x1a\x02\0\
    \x06\x12\x04\xdb\x02\x0c\x18\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\xdb\x02\
    \x19\x1f\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\xdb\x02\"#\n\\\n\x02\x04\
    \x1b\x12\x06\xdf\x02\0\xe1\x02\x01\x1aN---------------------------------\
    --------------------------------------------\n\n\x0b\n\x03\x04\x1b\x01\
    \x12\x04\xdf\x02\x08\x13\nD\n\x04\x04\x1b\x02\0\x12\x04\xe0\x02\x02\x1c\
    \"6\x20Number\x20of\x20game\x20loops\x20to\x20simulate\x20for\x20the\x20\
    next\x20frame.\n\n\r\n\x05\x04\x1b\x02\0\x04\x12\x04\xe0\x02\x02\n\n\r\n\
    \x05\x04\x1b\x02\0\x05\x12\x04\xe0\x02\x0b\x11\n\r\n\x05\x04\x1b\x02\0\
    \x01\x12\x04\xe0\x02\x12\x17\n\r\n\x05\x04\x1b\x02\0\x03\x12\x04\xe0\x02\
    \x1a\x1b\n\x0c\n\x02\x04\x1c\x12\x06\xe3\x02\0\xe4\x02\x01\n\x0b\n\x03\
    \x04\x1c\x01\x12\x04\xe3\x02\x08\x14\n\\\n\x02\x04\x1d\x12\x06\xe7\x02\0\
    \xec\x02\x01\x1aN-------------------------------------------------------\
    ----------------------\n\n\x0b\n\x03\x04\x1d\x01\x12\x04\xe7\x02\x08\x13\
    \n\x0c\n\x04\x04\x1d\x02\0\x12\x04\xe8\x02\x02\x1f\n\r\n\x05\x04\x1d\x02\
    \0\x04\x12\x04\xe8\x02\x02\n\n\r\n\x05\x04\x1d\x02\0\x05\x12\x04\xe8\x02\
    \x0b\x0f\n\r\n\x05\x04\x1d\x02\0\x01\x12\x04\xe8\x02\x10\x1a\n\r\n\x05\
    \x04\x1d\x02\0\x03\x12\x04\xe8\x02\x1d\x1e\n\x0c\n\x04\x04\x1d\x02\x01\
    \x12\x04\xe9\x02\x02!\n\r\n\x05\x04\x1d\x02\x01\x04\x12\x04\xe9\x02\x02\
    \n\n\r\n\x05\x04\x1d\x02\x01\x05\x12\x04\xe9\x02\x0b\x0f\n\r\n\x05\x04\
    \x1d\x02\x01\x01\x12\x04\xe9\x02\x10\x1c\n\r\n\x05\x04\x1d\x02\x01\x03\
    \x12\x04\xe9\x02\x1f\x20\n\x0c\n\x04\x04\x1d\x02\x02\x12\x04\xea\x02\x02\
    \x1f\n\r\n\x05\x04\x1d\x02\x02\x04\x12\x04\xea\x02\x02\n\n\r\n\x05\x04\
    \x1d\x02\x02\x05\x12\x04\xea\x02\x0b\x0f\n\r\n\x05\x04\x1d\x02\x02\x01\
    \x12\x04\xea\x02\x10\x1a\n\r\n\x05\x04\x1d\x02\x02\x03\x12\x04\xea\x02\
    \x1d\x1e\n\x0c\n\x04\x04\x1d\x02\x03\x12\x04\xeb\x02\x02\x1c\n\r\n\x05\
    \x04\x1d\x02\x03\x04\x12\x04\xeb\x02\x02\n\n\r\n\x05\x04\x1d\x02\x03\x05\
    \x12\x04\xeb\x02\x0b\x0f\n\r\n\x05\x04\x1d\x02\x03\x01\x12\x04\xeb\x02\
    \x10\x17\n\r\n\x05\x04\x1d\x02\x03\x03\x12\x04\xeb\x02\x1a\x1b\n\x0c\n\
    \x02\x04\x1e\x12\x06\xee\x02\0\xf3\x02\x01\n\x0b\n\x03\x04\x1e\x01\x12\
    \x04\xee\x02\x08\x14\n\x0c\n\x04\x04\x1e\x02\0\x12\x04\xef\x02\x02%\n\r\
    \n\x05\x04\x1e\x02\0\x04\x12\x04\xef\x02\x02\n\n\r\n\x05\x04\x1e\x02\0\
    \x06\x12\x04\xef\x02\x0b\x16\n\r\n\x05\x04\x1e\x02\0\x01\x12\x04\xef\x02\
    \x17\x20\n\r\n\x05\x04\x1e\x02\0\x03\x12\x04\xef\x02#$\n\x0c\n\x04\x04\
    \x1e\x02\x01\x12\x04\xf0\x02\x02\"\n\r\n\x05\x04\x1e\x02\x01\x04\x12\x04\
    \xf0\x02\x02\n\n\r\n\x05\x04\x1e\x02\x01\x06\x12\x04\xf0\x02\x0b\x17\n\r\
    \n\x05\x04\x1e\x02\x01\x01\x12\x04\xf0\x02\x18\x1d\n\r\n\x05\x04\x1e\x02\
    \x01\x03\x12\x04\xf0\x02\x20!\n\x0c\n\x04\x04\x1e\x02\x02\x12\x04\xf1\
    \x02\x02$\n\r\n\x05\x04\x1e\x02\x02\x04\x12\x04\xf1\x02\x02\n\n\r\n\x05\
    \x04\x1e\x02\x02\x06\x12\x04\xf1\x02\x0b\x16\n\r\n\x05\x04\x1e\x02\x02\
    \x01\x12\x04\xf1\x02\x17\x1f\n\r\n\x05\x04\x1e\x02\x02\x03\x12\x04\xf1\
    \x02\"#\n\x0c\n\x04\x04\x1e\x02\x03\x12\x04\xf2\x02\x02\x1e\n\r\n\x05\
    \x04\x1e\x02\x03\x04\x12\x04\xf2\x02\x02\n\n\r\n\x05\x04\x1e\x02\x03\x06\
    \x12\x04\xf2\x02\x0b\x13\n\r\n\x05\x04\x1e\x02\x03\x01\x12\x04\xf2\x02\
    \x14\x19\n\r\n\x05\x04\x1e\x02\x03\x03\x12\x04\xf2\x02\x1c\x1d\n\\\n\x02\
    \x04\x1f\x12\x06\xf6\x02\0\xf7\x02\x01\x1aN-----------------------------\
    ------------------------------------------------\n\n\x0b\n\x03\x04\x1f\
    \x01\x12\x04\xf6\x02\x08\x19\n\x0c\n\x02\x04\x20\x12\x06\xf9\x02\0\xfb\
    \x02\x01\n\x0b\n\x03\x04\x20\x01\x12\x04\xf9\x02\x08\x1a\n\x0c\n\x04\x04\
    \x20\x02\0\x12\x04\xfa\x02\x02\x1a\n\r\n\x05\x04\x20\x02\0\x04\x12\x04\
    \xfa\x02\x02\n\n\r\n\x05\x04\x20\x02\0\x05\x12\x04\xfa\x02\x0b\x10\n\r\n\
    \x05\x04\x20\x02\0\x01\x12\x04\xfa\x02\x11\x15\n\r\n\x05\x04\x20\x02\0\
    \x03\x12\x04\xfa\x02\x18\x19\n\\\n\x02\x04!\x12\x06\xfe\x02\0\x84\x03\
    \x01\x1aN---------------------------------------------------------------\
    --------------\n\n\x0b\n\x03\x04!\x01\x12\x04\xfe\x02\x08\x19\n\x0e\n\
    \x04\x04!\x08\0\x12\x06\xff\x02\x02\x82\x03\x03\n\r\n\x05\x04!\x08\0\x01\
    \x12\x04\xff\x02\x08\x0e\nN\n\x04\x04!\x02\0\x12\x04\x80\x03\x04\x1b\"@\
    \x20Limitation:\x20might\x20fail\x20if\x20the\x20replay\x20file\x20is\
    \x20currently\x20loaded.\n\n\r\n\x05\x04!\x02\0\x05\x12\x04\x80\x03\x04\
    \n\n\r\n\x05\x04!\x02\0\x01\x12\x04\x80\x03\x0b\x16\n\r\n\x05\x04!\x02\0\
    \x03\x12\x04\x80\x03\x19\x1a\n\x0c\n\x04\x04!\x02\x01\x12\x04\x81\x03\
    \x04\x1a\n\r\n\x05\x04!\x02\x01\x05\x12\x04\x81\x03\x04\t\n\r\n\x05\x04!\
    \x02\x01\x01\x12\x04\x81\x03\n\x15\n\r\n\x05\x04!\x02\x01\x03\x12\x04\
    \x81\x03\x18\x19\n[\n\x04\x04!\x02\x02\x12\x04\x83\x03\x02\"\"M\x20Ensur\
    e\x20the\x20data\x20and\x20binary\x20are\x20downloaded\x20if\x20this\x20\
    is\x20an\x20old\x20version\x20replay.\n\n\r\n\x05\x04!\x02\x02\x04\x12\
    \x04\x83\x03\x02\n\n\r\n\x05\x04!\x02\x02\x05\x12\x04\x83\x03\x0b\x0f\n\
    \r\n\x05\x04!\x02\x02\x01\x12\x04\x83\x03\x10\x1d\n\r\n\x05\x04!\x02\x02\
    \x03\x12\x04\x83\x03\x20!\n\x0c\n\x02\x04\"\x12\x06\x86\x03\0\x8b\x03\
    \x01\n\x0b\n\x03\x04\"\x01\x12\x04\x86\x03\x08\x17\n\x0c\n\x04\x04\"\x02\
    \0\x12\x04\x87\x03\x02&\n\r\n\x05\x04\"\x02\0\x04\x12\x04\x87\x03\x02\n\
    \n\r\n\x05\x04\"\x02\0\x06\x12\x04\x87\x03\x0b\x15\n\r\n\x05\x04\"\x02\0\
    \x01\x12\x04\x87\x03\x16!\n\r\n\x05\x04\"\x02\0\x03\x12\x04\x87\x03$%\n\
    \x0c\n\x04\x04\"\x02\x01\x12\x04\x88\x03\x02*\n\r\n\x05\x04\"\x02\x01\
    \x04\x12\x04\x88\x03\x02\n\n\r\n\x05\x04\"\x02\x01\x06\x12\x04\x88\x03\
    \x0b\x17\n\r\n\x05\x04\"\x02\x01\x01\x12\x04\x88\x03\x18%\n\r\n\x05\x04\
    \"\x02\x01\x03\x12\x04\x88\x03()\n\x0c\n\x04\x04\"\x02\x02\x12\x04\x89\
    \x03\x02\x20\n\r\n\x05\x04\"\x02\x02\x04\x12\x04\x89\x03\x02\n\n\r\n\x05\
    \x04\"\x02\x02\x05\x12\x04\x89\x03\x0b\x10\n\r\n\x05\x04\"\x02\x02\x01\
    \x12\x04\x89\x03\x11\x1b\n\r\n\x05\x04\"\x02\x02\x03\x12\x04\x89\x03\x1e\
    \x1f\n\x0c\n\x04\x04\"\x02\x03\x12\x04\x8a\x03\x02\x20\n\r\n\x05\x04\"\
    \x02\x03\x04\x12\x04\x8a\x03\x02\n\n\r\n\x05\x04\"\x02\x03\x05\x12\x04\
    \x8a\x03\x0b\x10\n\r\n\x05\x04\"\x02\x03\x01\x12\x04\x8a\x03\x11\x1b\n\r\
    \n\x05\x04\"\x02\x03\x03\x12\x04\x8a\x03\x1e\x1f\n\x0c\n\x02\x04#\x12\
    \x06\x8d\x03\0\xa1\x03\x01\n\x0b\n\x03\x04#\x01\x12\x04\x8d\x03\x08\x1a\
    \n\x0c\n\x04\x04#\x02\0\x12\x04\x8e\x03\x02\x1f\n\r\n\x05\x04#\x02\0\x04\
    \x12\x04\x8e\x03\x02\n\n\r\n\x05\x04#\x02\0\x05\x12\x04\x8e\x03\x0b\x11\
    \n\r\n\x05\x04#\x02\0\x01\x12\x04\x8e\x03\x12\x1a\n\r\n\x05\x04#\x02\0\
    \x03\x12\x04\x8e\x03\x1d\x1e\n\x0c\n\x04\x04#\x02\x01\x12\x04\x8f\x03\
    \x02%\n\r\n\x05\x04#\x02\x01\x04\x12\x04\x8f\x03\x02\n\n\r\n\x05\x04#\
    \x02\x01\x05\x12\x04\x8f\x03\x0b\x11\n\r\n\x05\x04#\x02\x01\x01\x12\x04\
    \x8f\x03\x12\x20\n\r\n\x05\x04#\x02\x01\x03\x12\x04\x8f\x03#$\n\x0c\n\
    \x04\x04#\x02\x02\x12\x04\x90\x03\x02+\n\r\n\x05\x04#\x02\x02\x04\x12\
    \x04\x90\x03\x02\n\n\r\n\x05\x04#\x02\x02\x06\x12\x04\x90\x03\x0b\x1a\n\
    \r\n\x05\x04#\x02\x02\x01\x12\x04\x90\x03\x1b&\n\r\n\x05\x04#\x02\x02\
    \x03\x12\x04\x90\x03)*\n\x0c\n\x04\x04#\x02\x03\x12\x04\x91\x03\x02*\n\r\
    \n\x05\x04#\x02\x03\x04\x12\x04\x91\x03\x02\n\n\r\n\x05\x04#\x02\x03\x05\
    \x12\x04\x91\x03\x0b\x11\n\r\n\x05\x04#\x02\x03\x01\x12\x04\x91\x03\x12%\
    \n\r\n\x05\x04#\x02\x03\x03\x12\x04\x91\x03()\n\x0c\n\x04\x04#\x02\x04\
    \x12\x04\x92\x03\x02+\n\r\n\x05\x04#\x02\x04\x04\x12\x04\x92\x03\x02\n\n\
    \r\n\x05\x04#\x02\x04\x05\x12\x04\x92\x03\x0b\x10\n\r\n\x05\x04#\x02\x04\
    \x01\x12\x04\x92\x03\x11&\n\r\n\x05\x04#\x02\x04\x03\x12\x04\x92\x03)*\n\
    \x0c\n\x04\x04#\x02\x05\x12\x04\x93\x03\x02#\n\r\n\x05\x04#\x02\x05\x04\
    \x12\x04\x93\x03\x02\n\n\r\n\x05\x04#\x02\x05\x05\x12\x04\x93\x03\x0b\
    \x11\n\r\n\x05\x04#\x02\x05\x01\x12\x04\x93\x03\x12\x1e\n\r\n\x05\x04#\
    \x02\x05\x03\x12\x04\x93\x03!\"\n\x0c\n\x04\x04#\x02\x06\x12\x04\x94\x03\
    \x02$\n\r\n\x05\x04#\x02\x06\x04\x12\x04\x94\x03\x02\n\n\r\n\x05\x04#\
    \x02\x06\x05\x12\x04\x94\x03\x0b\x11\n\r\n\x05\x04#\x02\x06\x01\x12\x04\
    \x94\x03\x12\x1e\n\r\n\x05\x04#\x02\x06\x03\x12\x04\x94\x03!#\n\x0c\n\
    \x04\x04#\x02\x07\x12\x04\x95\x03\x02!\n\r\n\x05\x04#\x02\x07\x04\x12\
    \x04\x95\x03\x02\n\n\r\n\x05\x04#\x02\x07\x05\x12\x04\x95\x03\x0b\x11\n\
    \r\n\x05\x04#\x02\x07\x01\x12\x04\x95\x03\x12\x1c\n\r\n\x05\x04#\x02\x07\
    \x03\x12\x04\x95\x03\x1f\x20\n\x0c\n\x04\x04#\x02\x08\x12\x04\x96\x03\
    \x02!\n\r\n\x05\x04#\x02\x08\x04\x12\x04\x96\x03\x02\n\n\r\n\x05\x04#\
    \x02\x08\x05\x12\x04\x96\x03\x0b\x11\n\r\n\x05\x04#\x02\x08\x01\x12\x04\
    \x96\x03\x12\x1c\n\r\n\x05\x04#\x02\x08\x03\x12\x04\x96\x03\x1f\x20\n\
    \x0e\n\x04\x04#\x04\0\x12\x06\x98\x03\x02\x9e\x03\x03\n\r\n\x05\x04#\x04\
    \0\x01\x12\x04\x98\x03\x07\x0c\n\x0e\n\x06\x04#\x04\0\x02\0\x12\x04\x99\
    \x03\x04\x16\n\x0f\n\x07\x04#\x04\0\x02\0\x01\x12\x04\x99\x03\x04\x11\n\
    \x0f\n\x07\x04#\x04\0\x02\0\x02\x12\x04\x99\x03\x14\x15\n\x0e\n\x06\x04#\
    \x04\0\x02\x01\x12\x04\x9a\x03\x04\x1a\n\x0f\n\x07\x04#\x04\0\x02\x01\
    \x01\x12\x04\x9a\x03\x04\x15\n\x0f\n\x07\x04#\x04\0\x02\x01\x02\x12\x04\
    \x9a\x03\x18\x19\n\x0e\n\x06\x04#\x04\0\x02\x02\x12\x04\x9b\x03\x04\x1a\
    \n\x0f\n\x07\x04#\x04\0\x02\x02\x01\x12\x04\x9b\x03\x04\x15\n\x0f\n\x07\
    \x04#\x04\0\x02\x02\x02\x12\x04\x9b\x03\x18\x19\n\x0e\n\x06\x04#\x04\0\
    \x02\x03\x12\x04\x9c\x03\x04\x15\n\x0f\n\x07\x04#\x04\0\x02\x03\x01\x12\
    \x04\x9c\x03\x04\x10\n\x0f\n\x07\x04#\x04\0\x02\x03\x02\x12\x04\x9c\x03\
    \x13\x14\n\x0e\n\x06\x04#\x04\0\x02\x04\x12\x04\x9d\x03\x04\x16\n\x0f\n\
    \x07\x04#\x04\0\x02\x04\x01\x12\x04\x9d\x03\x04\x11\n\x0f\n\x07\x04#\x04\
    \0\x02\x04\x02\x12\x04\x9d\x03\x14\x15\n\x0c\n\x04\x04#\x02\t\x12\x04\
    \x9f\x03\x02\x1b\n\r\n\x05\x04#\x02\t\x04\x12\x04\x9f\x03\x02\n\n\r\n\
    \x05\x04#\x02\t\x06\x12\x04\x9f\x03\x0b\x10\n\r\n\x05\x04#\x02\t\x01\x12\
    \x04\x9f\x03\x11\x16\n\r\n\x05\x04#\x02\t\x03\x12\x04\x9f\x03\x19\x1a\n\
    \x0c\n\x04\x04#\x02\n\x12\x04\xa0\x03\x02%\n\r\n\x05\x04#\x02\n\x04\x12\
    \x04\xa0\x03\x02\n\n\r\n\x05\x04#\x02\n\x05\x12\x04\xa0\x03\x0b\x11\n\r\
    \n\x05\x04#\x02\n\x01\x12\x04\xa0\x03\x12\x1f\n\r\n\x05\x04#\x02\n\x03\
    \x12\x04\xa0\x03\"$\n\\\n\x02\x04$\x12\x06\xa4\x03\0\xa5\x03\x01\x1aN---\
    ------------------------------------------------------------------------\
    --\n\n\x0b\n\x03\x04$\x01\x12\x04\xa4\x03\x08\x1c\n\xc6\x01\n\x02\x04%\
    \x12\x06\xaa\x03\0\xad\x03\x01\x1a\xb7\x01\x20This\x20will\x20only\x20co\
    ntain\x20locally\x20cached\x20BattleNet\x20maps.\n\x20To\x20download\x20\
    all\x20ladder\x20maps,\x20log\x20in\x20and\x20queue\x20into\x20a\x20ladd\
    er\x20match.\n\x20To\x20download\x20any\x20other\x20map,\x20play\x20a\
    \x20custom\x20game\x20on\x20that\x20map.\n\n\x0b\n\x03\x04%\x01\x12\x04\
    \xaa\x03\x08\x1d\n6\n\x04\x04%\x02\0\x12\x04\xab\x03\x02&\"(\x20All\x20t\
    he\x20maps\x20in\x20the\x20\"Maps/\"\x20directory.\n\n\r\n\x05\x04%\x02\
    \0\x04\x12\x04\xab\x03\x02\n\n\r\n\x05\x04%\x02\0\x05\x12\x04\xab\x03\
    \x0b\x11\n\r\n\x05\x04%\x02\0\x01\x12\x04\xab\x03\x12!\n\r\n\x05\x04%\
    \x02\0\x03\x12\x04\xab\x03$%\n4\n\x04\x04%\x02\x01\x12\x04\xac\x03\x02*\
    \"&\x20All\x20the\x20maps\x20in\x20the\x20BattleNet\x20cache.\n\n\r\n\
    \x05\x04%\x02\x01\x04\x12\x04\xac\x03\x02\n\n\r\n\x05\x04%\x02\x01\x05\
    \x12\x04\xac\x03\x0b\x11\n\r\n\x05\x04%\x02\x01\x01\x12\x04\xac\x03\x12%\
    \n\r\n\x05\x04%\x02\x01\x03\x12\x04\xac\x03()\n\x86\x01\n\x02\x04&\x12\
    \x06\xb1\x03\0\xb4\x03\x01\x1ax-----------------------------------------\
    ------------------------------------\n\x20Copies\x20map\x20data\x20into\
    \x20the\x20path\x20specified.\n\n\x0b\n\x03\x04&\x01\x12\x04\xb1\x03\x08\
    \x16\nh\n\x04\x04&\x02\0\x12\x04\xb2\x03\x02\x1f\"Z\x20Path\x20the\x20ga\
    me\x20process\x20will\x20write\x20to,\x20relative\x20to\x20the\x20temp\
    \x20directory.\x20(260\x20character\x20max)\n\n\r\n\x05\x04&\x02\0\x04\
    \x12\x04\xb2\x03\x02\n\n\r\n\x05\x04&\x02\0\x05\x12\x04\xb2\x03\x0b\x11\
    \n\r\n\x05\x04&\x02\0\x01\x12\x04\xb2\x03\x12\x1a\n\r\n\x05\x04&\x02\0\
    \x03\x12\x04\xb2\x03\x1d\x1e\n-\n\x04\x04&\x02\x01\x12\x04\xb3\x03\x02\
    \x1e\"\x1f\x20Binary\x20map\x20data\x20of\x20a\x20.SC2Map.\n\n\r\n\x05\
    \x04&\x02\x01\x04\x12\x04\xb3\x03\x02\n\n\r\n\x05\x04&\x02\x01\x05\x12\
    \x04\xb3\x03\x0b\x10\n\r\n\x05\x04&\x02\x01\x01\x12\x04\xb3\x03\x11\x19\
    \n\r\n\x05\x04&\x02\x01\x03\x12\x04\xb3\x03\x1c\x1d\n\x0c\n\x02\x04'\x12\
    \x06\xb6\x03\0\xbb\x03\x01\n\x0b\n\x03\x04'\x01\x12\x04\xb6\x03\x08\x17\
    \n\x0e\n\x04\x04'\x04\0\x12\x06\xb7\x03\x02\xb9\x03\x03\n\r\n\x05\x04'\
    \x04\0\x01\x12\x04\xb7\x03\x07\x0c\n\x0e\n\x06\x04'\x04\0\x02\0\x12\x04\
    \xb8\x03\x04\x17\n\x0f\n\x07\x04'\x04\0\x02\0\x01\x12\x04\xb8\x03\x04\
    \x12\n\x0f\n\x07\x04'\x04\0\x02\0\x02\x12\x04\xb8\x03\x15\x16\n\x0c\n\
    \x04\x04'\x02\0\x12\x04\xba\x03\x02\x1b\n\r\n\x05\x04'\x02\0\x04\x12\x04\
    \xba\x03\x02\n\n\r\n\x05\x04'\x02\0\x06\x12\x04\xba\x03\x0b\x10\n\r\n\
    \x05\x04'\x02\0\x01\x12\x04\xba\x03\x11\x16\n\r\n\x05\x04'\x02\0\x03\x12\
    \x04\xba\x03\x19\x1a\n\\\n\x02\x04(\x12\x06\xbe\x03\0\xbf\x03\x01\x1aN--\
    ------------------------------------------------------------------------\
    ---\n\n\x0b\n\x03\x04(\x01\x12\x04\xbe\x03\x08\x13\n\x0c\n\x02\x04)\x12\
    \x06\xc1\x03\0\xc6\x03\x01\n\x0b\n\x03\x04)\x01\x12\x04\xc1\x03\x08\x14\
    \n\x0c\n\x04\x04)\x02\0\x12\x04\xc2\x03\x02#\n\r\n\x05\x04)\x02\0\x04\
    \x12\x04\xc2\x03\x02\n\n\r\n\x05\x04)\x02\0\x05\x12\x04\xc2\x03\x0b\x11\
    \n\r\n\x05\x04)\x02\0\x01\x12\x04\xc2\x03\x12\x1e\n\r\n\x05\x04)\x02\0\
    \x03\x12\x04\xc2\x03!\"\n\x0c\n\x04\x04)\x02\x01\x12\x04\xc3\x03\x02#\n\
    \r\n\x05\x04)\x02\x01\x04\x12\x04\xc3\x03\x02\n\n\r\n\x05\x04)\x02\x01\
    \x05\x12\x04\xc3\x03\x0b\x11\n\r\n\x05\x04)\x02\x01\x01\x12\x04\xc3\x03\
    \x12\x1e\n\r\n\x05\x04)\x02\x01\x03\x12\x04\xc3\x03!\"\n\x0c\n\x04\x04)\
    \x02\x02\x12\x04\xc4\x03\x02!\n\r\n\x05\x04)\x02\x02\x04\x12\x04\xc4\x03\
    \x02\n\n\r\n\x05\x04)\x02\x02\x05\x12\x04\xc4\x03\x0b\x11\n\r\n\x05\x04)\
    \x02\x02\x01\x12\x04\xc4\x03\x12\x1c\n\r\n\x05\x04)\x02\x02\x03\x12\x04\
    \xc4\x03\x1f\x20\n\x0c\n\x04\x04)\x02\x03\x12\x04\xc5\x03\x02!\n\r\n\x05\
    \x04)\x02\x03\x04\x12\x04\xc5\x03\x02\n\n\r\n\x05\x04)\x02\x03\x05\x12\
    \x04\xc5\x03\x0b\x11\n\r\n\x05\x04)\x02\x03\x01\x12\x04\xc5\x03\x12\x1c\
    \n\r\n\x05\x04)\x02\x03\x03\x12\x04\xc5\x03\x1f\x20\n\\\n\x02\x04*\x12\
    \x06\xc9\x03\0\xcb\x03\x01\x1aN-----------------------------------------\
    ------------------------------------\n\n\x0b\n\x03\x04*\x01\x12\x04\xc9\
    \x03\x08\x14\n\x0c\n\x04\x04*\x02\0\x12\x04\xca\x03\x02\"\n\r\n\x05\x04*\
    \x02\0\x04\x12\x04\xca\x03\x02\n\n\r\n\x05\x04*\x02\0\x06\x12\x04\xca\
    \x03\x0b\x17\n\r\n\x05\x04*\x02\0\x01\x12\x04\xca\x03\x18\x1d\n\r\n\x05\
    \x04*\x02\0\x03\x12\x04\xca\x03\x20!\n\x0c\n\x02\x04+\x12\x06\xcd\x03\0\
    \xce\x03\x01\n\x0b\n\x03\x04+\x01\x12\x04\xcd\x03\x08\x15\n\x1c\n\x02\
    \x05\x01\x12\x06\xd5\x03\0\xe0\x03\x012\x0e\n\x20Game\x20Setup\n\n\n\x0b\
    \n\x03\x05\x01\x01\x12\x04\xd5\x03\x05\x0f\n\x0c\n\x04\x05\x01\x02\0\x12\
    \x04\xd6\x03\x02\x0f\n\r\n\x05\x05\x01\x02\0\x01\x12\x04\xd6\x03\x02\n\n\
    \r\n\x05\x05\x01\x02\0\x02\x12\x04\xd6\x03\r\x0e\n\x0c\n\x04\x05\x01\x02\
    \x01\x12\x04\xd7\x03\x02\x0b\n\r\n\x05\x05\x01\x02\x01\x01\x12\x04\xd7\
    \x03\x02\x06\n\r\n\x05\x05\x01\x02\x01\x02\x12\x04\xd7\x03\t\n\n\x0c\n\
    \x04\x05\x01\x02\x02\x12\x04\xd8\x03\x02\r\n\r\n\x05\x05\x01\x02\x02\x01\
    \x12\x04\xd8\x03\x02\x08\n\r\n\x05\x05\x01\x02\x02\x02\x12\x04\xd8\x03\
    \x0b\x0c\n\x0c\n\x04\x05\x01\x02\x03\x12\x04\xd9\x03\x02\x11\n\r\n\x05\
    \x05\x01\x02\x03\x01\x12\x04\xd9\x03\x02\x0c\n\r\n\x05\x05\x01\x02\x03\
    \x02\x12\x04\xd9\x03\x0f\x10\n\x0c\n\x04\x05\x01\x02\x04\x12\x04\xda\x03\
    \x02\x0b\n\r\n\x05\x05\x01\x02\x04\x01\x12\x04\xda\x03\x02\x06\n\r\n\x05\
    \x05\x01\x02\x04\x02\x12\x04\xda\x03\t\n\n\x0c\n\x04\x05\x01\x02\x05\x12\
    \x04\xdb\x03\x02\r\n\r\n\x05\x05\x01\x02\x05\x01\x12\x04\xdb\x03\x02\x08\
    \n\r\n\x05\x05\x01\x02\x05\x02\x12\x04\xdb\x03\x0b\x0c\n\x0c\n\x04\x05\
    \x01\x02\x06\x12\x04\xdc\x03\x02\x0f\n\r\n\x05\x05\x01\x02\x06\x01\x12\
    \x04\xdc\x03\x02\n\n\r\n\x05\x05\x01\x02\x06\x02\x12\x04\xdc\x03\r\x0e\n\
    \x0c\n\x04\x05\x01\x02\x07\x12\x04\xdd\x03\x02\x12\n\r\n\x05\x05\x01\x02\
    \x07\x01\x12\x04\xdd\x03\x02\r\n\r\n\x05\x05\x01\x02\x07\x02\x12\x04\xdd\
    \x03\x10\x11\n\x0c\n\x04\x05\x01\x02\x08\x12\x04\xde\x03\x02\x11\n\r\n\
    \x05\x05\x01\x02\x08\x01\x12\x04\xde\x03\x02\x0c\n\r\n\x05\x05\x01\x02\
    \x08\x02\x12\x04\xde\x03\x0f\x10\n\x0c\n\x04\x05\x01\x02\t\x12\x04\xdf\
    \x03\x02\x13\n\r\n\x05\x05\x01\x02\t\x01\x12\x04\xdf\x03\x02\r\n\r\n\x05\
    \x05\x01\x02\t\x02\x12\x04\xdf\x03\x10\x12\n\x0c\n\x02\x05\x02\x12\x06\
    \xe2\x03\0\xe6\x03\x01\n\x0b\n\x03\x05\x02\x01\x12\x04\xe2\x03\x05\x0f\n\
    \x0c\n\x04\x05\x02\x02\0\x12\x04\xe3\x03\x02\x12\n\r\n\x05\x05\x02\x02\0\
    \x01\x12\x04\xe3\x03\x02\r\n\r\n\x05\x05\x02\x02\0\x02\x12\x04\xe3\x03\
    \x10\x11\n\x0c\n\x04\x05\x02\x02\x01\x12\x04\xe4\x03\x02\x0f\n\r\n\x05\
    \x05\x02\x02\x01\x01\x12\x04\xe4\x03\x02\n\n\r\n\x05\x05\x02\x02\x01\x02\
    \x12\x04\xe4\x03\r\x0e\n\x0c\n\x04\x05\x02\x02\x02\x12\x04\xe5\x03\x02\
    \x0f\n\r\n\x05\x05\x02\x02\x02\x01\x12\x04\xe5\x03\x02\n\n\r\n\x05\x05\
    \x02\x02\x02\x02\x12\x04\xe5\x03\r\x0e\n\x0c\n\x02\x04,\x12\x06\xe8\x03\
    \0\xee\x03\x01\n\x0b\n\x03\x04,\x01\x12\x04\xe8\x03\x08\x13\n\x0c\n\x04\
    \x04,\x02\0\x12\x04\xe9\x03\x02\x1f\n\r\n\x05\x04,\x02\0\x04\x12\x04\xe9\
    \x03\x02\n\n\r\n\x05\x04,\x02\0\x06\x12\x04\xe9\x03\x0b\x15\n\r\n\x05\
    \x04,\x02\0\x01\x12\x04\xe9\x03\x16\x1a\n\r\n\x05\x04,\x02\0\x03\x12\x04\
    \xe9\x03\x1d\x1e\n0\n\x04\x04,\x02\x01\x12\x04\xec\x03\x02\x19\x1a\"\x20\
    Only\x20used\x20for\x20a\x20computer\x20player.\n\n\r\n\x05\x04,\x02\x01\
    \x04\x12\x04\xec\x03\x02\n\n\r\n\x05\x04,\x02\x01\x06\x12\x04\xec\x03\
    \x0b\x0f\n\r\n\x05\x04,\x02\x01\x01\x12\x04\xec\x03\x10\x14\n\r\n\x05\
    \x04,\x02\x01\x03\x12\x04\xec\x03\x17\x18\n\x0c\n\x04\x04,\x02\x02\x12\
    \x04\xed\x03\x02%\n\r\n\x05\x04,\x02\x02\x04\x12\x04\xed\x03\x02\n\n\r\n\
    \x05\x04,\x02\x02\x06\x12\x04\xed\x03\x0b\x15\n\r\n\x05\x04,\x02\x02\x01\
    \x12\x04\xed\x03\x16\x20\n\r\n\x05\x04,\x02\x02\x03\x12\x04\xed\x03#$\n\
    \x0c\n\x02\x04-\x12\x06\xf0\x03\0\xf4\x03\x01\n\x0b\n\x03\x04-\x01\x12\
    \x04\xf0\x03\x08\x1a\n\x0c\n\x04\x04-\x02\0\x12\x04\xf1\x03\x02\x1b\n\r\
    \n\x05\x04-\x02\0\x04\x12\x04\xf1\x03\x02\n\n\r\n\x05\x04-\x02\0\x05\x12\
    \x04\xf1\x03\x0b\x10\n\r\n\x05\x04-\x02\0\x01\x12\x04\xf1\x03\x11\x16\n\
    \r\n\x05\x04-\x02\0\x03\x12\x04\xf1\x03\x19\x1a\n\x0c\n\x04\x04-\x02\x01\
    \x12\x04\xf2\x03\x02\"\n\r\n\x05\x04-\x02\x01\x04\x12\x04\xf2\x03\x02\n\
    \n\r\n\x05\x04-\x02\x01\x06\x12\x04\xf2\x03\x0b\x12\n\r\n\x05\x04-\x02\
    \x01\x01\x12\x04\xf2\x03\x13\x1d\n\r\n\x05\x04-\x02\x01\x03\x12\x04\xf2\
    \x03\x20!\n\x0c\n\x04\x04-\x02\x02\x12\x04\xf3\x03\x02*\n\r\n\x05\x04-\
    \x02\x02\x04\x12\x04\xf3\x03\x02\n\n\r\n\x05\x04-\x02\x02\x06\x12\x04\
    \xf3\x03\x0b\x12\n\r\n\x05\x04-\x02\x02\x01\x12\x04\xf3\x03\x13%\n\r\n\
    \x05\x04-\x02\x02\x03\x12\x04\xf3\x03()\n\x0c\n\x02\x04.\x12\x06\xf6\x03\
    \0\xfc\x03\x01\n\x0b\n\x03\x04.\x01\x12\x04\xf6\x03\x08\x18\n!\n\x04\x04\
    .\x02\0\x12\x04\xf8\x03\x02\x18\x1a\x13\x20Interface\x20options\n\n\r\n\
    \x05\x04.\x02\0\x04\x12\x04\xf8\x03\x02\n\n\r\n\x05\x04.\x02\0\x05\x12\
    \x04\xf8\x03\x0b\x0f\n\r\n\x05\x04.\x02\0\x01\x12\x04\xf8\x03\x10\x13\n\
    \r\n\x05\x04.\x02\0\x03\x12\x04\xf8\x03\x16\x17\n\x0c\n\x04\x04.\x02\x01\
    \x12\x04\xf9\x03\x02\x1a\n\r\n\x05\x04.\x02\x01\x04\x12\x04\xf9\x03\x02\
    \n\n\r\n\x05\x04.\x02\x01\x05\x12\x04\xf9\x03\x0b\x0f\n\r\n\x05\x04.\x02\
    \x01\x01\x12\x04\xf9\x03\x10\x15\n\r\n\x05\x04.\x02\x01\x03\x12\x04\xf9\
    \x03\x18\x19\n\x20\n\x04\x04.\x02\x02\x12\x04\xfa\x03\x020\"\x12\x20Omit\
    \x20to\x20disable.\n\n\r\n\x05\x04.\x02\x02\x04\x12\x04\xfa\x03\x02\n\n\
    \r\n\x05\x04.\x02\x02\x06\x12\x04\xfa\x03\x0b\x1d\n\r\n\x05\x04.\x02\x02\
    \x01\x12\x04\xfa\x03\x1e+\n\r\n\x05\x04.\x02\x02\x03\x12\x04\xfa\x03./\n\
    \x20\n\x04\x04.\x02\x03\x12\x04\xfb\x03\x02)\"\x12\x20Not\x20implemented\
    .\n\n\r\n\x05\x04.\x02\x03\x04\x12\x04\xfb\x03\x02\n\n\r\n\x05\x04.\x02\
    \x03\x06\x12\x04\xfb\x03\x0b\x1d\n\r\n\x05\x04.\x02\x03\x01\x12\x04\xfb\
    \x03\x1e$\n\r\n\x05\x04.\x02\x03\x03\x12\x04\xfb\x03'(\n\x0c\n\x02\x04/\
    \x12\x06\xfe\x03\0\x86\x04\x01\n\x0b\n\x03\x04/\x01\x12\x04\xfe\x03\x08\
    \x12\n\xb6\x01\n\x04\x04/\x02\0\x12\x04\x81\x04\x02\x20\x1a\xa7\x01\x20I\
    dentifier\x20that\x20will\x20be\x20used\x20to\x20reference\x20this\x20pl\
    ayer.\n\x20SC2\x20will\x20always\x20assign\x20playerIds\x20starting\x20f\
    rom\x201\x20in\x20standard\x20Melee\x20maps.\x20This\x20may\x20not\x20be\
    \x20true\x20in\x20custom\x20maps.\n\n\r\n\x05\x04/\x02\0\x04\x12\x04\x81\
    \x04\x02\n\n\r\n\x05\x04/\x02\0\x05\x12\x04\x81\x04\x0b\x11\n\r\n\x05\
    \x04/\x02\0\x01\x12\x04\x81\x04\x12\x1b\n\r\n\x05\x04/\x02\0\x03\x12\x04\
    \x81\x04\x1e\x1f\n\x0c\n\x04\x04/\x02\x01\x12\x04\x82\x04\x02\x1f\n\r\n\
    \x05\x04/\x02\x01\x04\x12\x04\x82\x04\x02\n\n\r\n\x05\x04/\x02\x01\x06\
    \x12\x04\x82\x04\x0b\x15\n\r\n\x05\x04/\x02\x01\x01\x12\x04\x82\x04\x16\
    \x1a\n\r\n\x05\x04/\x02\x01\x03\x12\x04\x82\x04\x1d\x1e\n\x0c\n\x04\x04/\
    \x02\x02\x12\x04\x83\x04\x02#\n\r\n\x05\x04/\x02\x02\x04\x12\x04\x83\x04\
    \x02\n\n\r\n\x05\x04/\x02\x02\x06\x12\x04\x83\x04\x0b\x0f\n\r\n\x05\x04/\
    \x02\x02\x01\x12\x04\x83\x04\x10\x1e\n\r\n\x05\x04/\x02\x02\x03\x12\x04\
    \x83\x04!\"\nF\n\x04\x04/\x02\x03\x12\x04\x84\x04\x02\x20\"8\x20Only\x20\
    populated\x20for\x20your\x20player\x20or\x20when\x20watching\x20replay\n\
    \n\r\n\x05\x04/\x02\x03\x04\x12\x04\x84\x04\x02\n\n\r\n\x05\x04/\x02\x03\
    \x06\x12\x04\x84\x04\x0b\x0f\n\r\n\x05\x04/\x02\x03\x01\x12\x04\x84\x04\
    \x10\x1b\n\r\n\x05\x04/\x02\x03\x03\x12\x04\x84\x04\x1e\x1f\n\x0c\n\x04\
    \x04/\x02\x04\x12\x04\x85\x04\x02%\n\r\n\x05\x04/\x02\x04\x04\x12\x04\
    \x85\x04\x02\n\n\r\n\x05\x04/\x02\x04\x06\x12\x04\x85\x04\x0b\x15\n\r\n\
    \x05\x04/\x02\x04\x01\x12\x04\x85\x04\x16\x20\n\r\n\x05\x04/\x02\x04\x03\
    \x12\x04\x85\x04#$\n\x1d\n\x02\x040\x12\x06\x8d\x04\0\x99\x04\x012\x0f\n\
    \x20During\x20Game\n\n\n\x0b\n\x03\x040\x01\x12\x04\x8d\x04\x08\x14\n\
    \x0c\n\x04\x040\x02\0\x12\x04\x8e\x04\x02\x20\n\r\n\x05\x040\x02\0\x04\
    \x12\x04\x8e\x04\x02\n\n\r\n\x05\x040\x02\0\x05\x12\x04\x8e\x04\x0b\x11\
    \n\r\n\x05\x040\x02\0\x01\x12\x04\x8e\x04\x12\x1b\n\r\n\x05\x040\x02\0\
    \x03\x12\x04\x8e\x04\x1e\x1f\n\x0c\n\x04\x040\x02\x01\x12\x04\x8f\x04\
    \x02\x1f\n\r\n\x05\x040\x02\x01\x04\x12\x04\x8f\x04\x02\n\n\r\n\x05\x040\
    \x02\x01\x05\x12\x04\x8f\x04\x0b\x11\n\r\n\x05\x040\x02\x01\x01\x12\x04\
    \x8f\x04\x12\x1a\n\r\n\x05\x040\x02\x01\x03\x12\x04\x8f\x04\x1d\x1e\n\
    \x0c\n\x04\x040\x02\x02\x12\x04\x90\x04\x02\x1e\n\r\n\x05\x040\x02\x02\
    \x04\x12\x04\x90\x04\x02\n\n\r\n\x05\x040\x02\x02\x05\x12\x04\x90\x04\
    \x0b\x11\n\r\n\x05\x040\x02\x02\x01\x12\x04\x90\x04\x12\x19\n\r\n\x05\
    \x040\x02\x02\x03\x12\x04\x90\x04\x1c\x1d\n\x0c\n\x04\x040\x02\x03\x12\
    \x04\x91\x04\x02\x1f\n\r\n\x05\x040\x02\x03\x04\x12\x04\x91\x04\x02\n\n\
    \r\n\x05\x040\x02\x03\x05\x12\x04\x91\x04\x0b\x11\n\r\n\x05\x040\x02\x03\
    \x01\x12\x04\x91\x04\x12\x1a\n\r\n\x05\x040\x02\x03\x03\x12\x04\x91\x04\
    \x1d\x1e\n\x0c\n\x04\x040\x02\x04\x12\x04\x92\x04\x02\x20\n\r\n\x05\x040\
    \x02\x04\x04\x12\x04\x92\x04\x02\n\n\r\n\x05\x040\x02\x04\x05\x12\x04\
    \x92\x04\x0b\x11\n\r\n\x05\x040\x02\x04\x01\x12\x04\x92\x04\x12\x1b\n\r\
    \n\x05\x040\x02\x04\x03\x12\x04\x92\x04\x1e\x1f\n\x0c\n\x04\x040\x02\x05\
    \x12\x04\x93\x04\x02\x20\n\r\n\x05\x040\x02\x05\x04\x12\x04\x93\x04\x02\
    \n\n\r\n\x05\x040\x02\x05\x05\x12\x04\x93\x04\x0b\x11\n\r\n\x05\x040\x02\
    \x05\x01\x12\x04\x93\x04\x12\x1b\n\r\n\x05\x040\x02\x05\x03\x12\x04\x93\
    \x04\x1e\x1f\n\x0c\n\x04\x040\x02\x06\x12\x04\x94\x04\x02#\n\r\n\x05\x04\
    0\x02\x06\x04\x12\x04\x94\x04\x02\n\n\r\n\x05\x040\x02\x06\x05\x12\x04\
    \x94\x04\x0b\x11\n\r\n\x05\x040\x02\x06\x01\x12\x04\x94\x04\x12\x1e\n\r\
    \n\x05\x040\x02\x06\x03\x12\x04\x94\x04!\"\n\x0c\n\x04\x040\x02\x07\x12\
    \x04\x95\x04\x02(\n\r\n\x05\x040\x02\x07\x04\x12\x04\x95\x04\x02\n\n\r\n\
    \x05\x040\x02\x07\x05\x12\x04\x95\x04\x0b\x11\n\r\n\x05\x040\x02\x07\x01\
    \x12\x04\x95\x04\x12#\n\r\n\x05\x040\x02\x07\x03\x12\x04\x95\x04&'\n\x0c\
    \n\x04\x040\x02\x08\x12\x04\x96\x04\x02!\n\r\n\x05\x040\x02\x08\x04\x12\
    \x04\x96\x04\x02\n\n\r\n\x05\x040\x02\x08\x05\x12\x04\x96\x04\x0b\x11\n\
    \r\n\x05\x040\x02\x08\x01\x12\x04\x96\x04\x12\x1c\n\r\n\x05\x040\x02\x08\
    \x03\x12\x04\x96\x04\x1f\x20\n\x0c\n\x04\x040\x02\t\x12\x04\x97\x04\x02'\
    \n\r\n\x05\x040\x02\t\x04\x12\x04\x97\x04\x02\n\n\r\n\x05\x040\x02\t\x05\
    \x12\x04\x97\x04\x0b\x11\n\r\n\x05\x040\x02\t\x01\x12\x04\x97\x04\x12!\n\
    \r\n\x05\x040\x02\t\x03\x12\x04\x97\x04$&\n\x0c\n\x04\x040\x02\n\x12\x04\
    \x98\x04\x02#\n\r\n\x05\x040\x02\n\x04\x12\x04\x98\x04\x02\n\n\r\n\x05\
    \x040\x02\n\x05\x12\x04\x98\x04\x0b\x11\n\r\n\x05\x040\x02\n\x01\x12\x04\
    \x98\x04\x12\x1d\n\r\n\x05\x040\x02\n\x03\x12\x04\x98\x04\x20\"\n\x0c\n\
    \x02\x041\x12\x06\x9b\x04\0\xa6\x04\x01\n\x0b\n\x03\x041\x01\x12\x04\x9b\
    \x04\x08\x13\n\x0c\n\x04\x041\x02\0\x12\x04\x9c\x04\x02\x20\n\r\n\x05\
    \x041\x02\0\x04\x12\x04\x9c\x04\x02\n\n\r\n\x05\x041\x02\0\x05\x12\x04\
    \x9c\x04\x0b\x11\n\r\n\x05\x041\x02\0\x01\x12\x04\x9c\x04\x12\x1b\n\r\n\
    \x05\x041\x02\0\x03\x12\x04\x9c\x04\x1e\x1f\n\x0c\n\x04\x041\x02\x01\x12\
    \x04\x9d\x04\x02*\n\r\n\x05\x041\x02\x01\x04\x12\x04\x9d\x04\x02\n\n\r\n\
    \x05\x041\x02\x01\x06\x12\x04\x9d\x04\x0b\x17\n\r\n\x05\x041\x02\x01\x01\
    \x12\x04\x9d\x04\x18%\n\r\n\x05\x041\x02\x01\x03\x12\x04\x9d\x04()\n\x0c\
    \n\x04\x041\x02\x02\x12\x04\x9e\x04\x02\x1d\n\r\n\x05\x041\x02\x02\x04\
    \x12\x04\x9e\x04\x02\n\n\r\n\x05\x041\x02\x02\x06\x12\x04\x9e\x04\x0b\
    \x10\n\r\n\x05\x041\x02\x02\x01\x12\x04\x9e\x04\x11\x17\n\r\n\x05\x041\
    \x02\x02\x03\x12\x04\x9e\x04\x1a\x1c\nb\n\x04\x041\x02\x03\x12\x04\x9f\
    \x04\x02*\"T\x20Abilities\x20available\x20in\x20the\x20selection.\x20Ena\
    bled\x20if\x20in\x20this\x20list,\x20disabled\x20otherwise.\n\n\r\n\x05\
    \x041\x02\x03\x04\x12\x04\x9f\x04\x02\n\n\r\n\x05\x041\x02\x03\x06\x12\
    \x04\x9f\x04\x0b\x1b\n\r\n\x05\x041\x02\x03\x01\x12\x04\x9f\x04\x1c%\n\r\
    \n\x05\x041\x02\x03\x03\x12\x04\x9f\x04()\n\x0c\n\x04\x041\x02\x04\x12\
    \x04\xa0\x04\x02\x1b\n\r\n\x05\x041\x02\x04\x04\x12\x04\xa0\x04\x02\n\n\
    \r\n\x05\x041\x02\x04\x06\x12\x04\xa0\x04\x0b\x10\n\r\n\x05\x041\x02\x04\
    \x01\x12\x04\xa0\x04\x11\x16\n\r\n\x05\x041\x02\x04\x03\x12\x04\xa0\x04\
    \x19\x1a\n6\n\x04\x041\x02\x05\x12\x04\xa2\x04\x02'\"(\x20Populated\x20i\
    f\x20Raw\x20interface\x20is\x20enabled.\n\n\r\n\x05\x041\x02\x05\x04\x12\
    \x04\xa2\x04\x02\n\n\r\n\x05\x041\x02\x05\x06\x12\x04\xa2\x04\x0b\x19\n\
    \r\n\x05\x041\x02\x05\x01\x12\x04\xa2\x04\x1a\"\n\r\n\x05\x041\x02\x05\
    \x03\x12\x04\xa2\x04%&\n@\n\x04\x041\x02\x06\x12\x04\xa3\x04\x02:\"2\x20\
    Populated\x20if\x20Feature\x20Layer\x20interface\x20is\x20enabled.\n\n\r\
    \n\x05\x041\x02\x06\x04\x12\x04\xa3\x04\x02\n\n\r\n\x05\x041\x02\x06\x06\
    \x12\x04\xa3\x04\x0b\"\n\r\n\x05\x041\x02\x06\x01\x12\x04\xa3\x04#5\n\r\
    \n\x05\x041\x02\x06\x03\x12\x04\xa3\x0489\n9\n\x04\x041\x02\x07\x12\x04\
    \xa4\x04\x02-\"+\x20Populated\x20if\x20Render\x20interface\x20is\x20enab\
    led.\n\n\r\n\x05\x041\x02\x07\x04\x12\x04\xa4\x04\x02\n\n\r\n\x05\x041\
    \x02\x07\x06\x12\x04\xa4\x04\x0b\x1c\n\r\n\x05\x041\x02\x07\x01\x12\x04\
    \xa4\x04\x1d(\n\r\n\x05\x041\x02\x07\x03\x12\x04\xa4\x04+,\nJ\n\x04\x041\
    \x02\x08\x12\x04\xa5\x04\x02%\"<\x20Populated\x20if\x20Feature\x20Layer\
    \x20or\x20Render\x20interface\x20is\x20enabled.\n\n\r\n\x05\x041\x02\x08\
    \x04\x12\x04\xa5\x04\x02\n\n\r\n\x05\x041\x02\x08\x06\x12\x04\xa5\x04\
    \x0b\x18\n\r\n\x05\x041\x02\x08\x01\x12\x04\xa5\x04\x19\x20\n\r\n\x05\
    \x041\x02\x08\x03\x12\x04\xa5\x04#$\n\x0c\n\x02\x042\x12\x06\xa8\x04\0\
    \xae\x04\x01\n\x0b\n\x03\x042\x01\x12\x04\xa8\x04\x08\x0e\n6\n\x04\x042\
    \x02\0\x12\x04\xa9\x04\x02$\"(\x20Populated\x20if\x20Raw\x20interface\
    \x20is\x20enabled.\n\n\r\n\x05\x042\x02\0\x04\x12\x04\xa9\x04\x02\n\n\r\
    \n\x05\x042\x02\0\x06\x12\x04\xa9\x04\x0b\x14\n\r\n\x05\x042\x02\0\x01\
    \x12\x04\xa9\x04\x15\x1f\n\r\n\x05\x042\x02\0\x03\x12\x04\xa9\x04\"#\n@\
    \n\x04\x042\x02\x01\x12\x04\xaa\x04\x022\"2\x20Populated\x20if\x20Featur\
    e\x20Layer\x20interface\x20is\x20enabled.\n\n\r\n\x05\x042\x02\x01\x04\
    \x12\x04\xaa\x04\x02\n\n\r\n\x05\x042\x02\x01\x06\x12\x04\xaa\x04\x0b\
    \x18\n\r\n\x05\x042\x02\x01\x01\x12\x04\xaa\x04\x19-\n\r\n\x05\x042\x02\
    \x01\x03\x12\x04\xaa\x0401\nJ\n\x04\x042\x02\x02\x12\x04\xab\x04\x02+\"<\
    \x20Not\x20implemented.\x20Populated\x20if\x20Render\x20interface\x20is\
    \x20enabled.\n\n\r\n\x05\x042\x02\x02\x04\x12\x04\xab\x04\x02\n\n\r\n\
    \x05\x042\x02\x02\x06\x12\x04\xab\x04\x0b\x18\n\r\n\x05\x042\x02\x02\x01\
    \x12\x04\xab\x04\x19&\n\r\n\x05\x042\x02\x02\x03\x12\x04\xab\x04)*\nJ\n\
    \x04\x042\x02\x03\x12\x04\xac\x04\x02\"\"<\x20Populated\x20if\x20Feature\
    \x20Layer\x20or\x20Render\x20interface\x20is\x20enabled.\n\n\r\n\x05\x04\
    2\x02\x03\x04\x12\x04\xac\x04\x02\n\n\r\n\x05\x042\x02\x03\x06\x12\x04\
    \xac\x04\x0b\x13\n\r\n\x05\x042\x02\x03\x01\x12\x04\xac\x04\x14\x1d\n\r\
    \n\x05\x042\x02\x03\x03\x12\x04\xac\x04\x20!\nG\n\x04\x042\x02\x04\x12\
    \x04\xad\x04\x02\x1f\"9\x20Chat\x20messages\x20as\x20a\x20player\x20typi\
    ng\x20into\x20the\x20chat\x20channel.\n\n\r\n\x05\x042\x02\x04\x04\x12\
    \x04\xad\x04\x02\n\n\r\n\x05\x042\x02\x04\x06\x12\x04\xad\x04\x0b\x15\n\
    \r\n\x05\x042\x02\x04\x01\x12\x04\xad\x04\x16\x1a\n\r\n\x05\x042\x02\x04\
    \x03\x12\x04\xad\x04\x1d\x1e\n\x0c\n\x02\x043\x12\x06\xb0\x04\0\xb7\x04\
    \x01\n\x0b\n\x03\x043\x01\x12\x04\xb0\x04\x08\x12\n\x0e\n\x04\x043\x04\0\
    \x12\x06\xb1\x04\x02\xb4\x04\x03\n\r\n\x05\x043\x04\0\x01\x12\x04\xb1\
    \x04\x07\x0e\n\x0e\n\x06\x043\x04\0\x02\0\x12\x04\xb2\x04\x04\x12\n\x0f\
    \n\x07\x043\x04\0\x02\0\x01\x12\x04\xb2\x04\x04\r\n\x0f\n\x07\x043\x04\0\
    \x02\0\x02\x12\x04\xb2\x04\x10\x11\n\x0e\n\x06\x043\x04\0\x02\x01\x12\
    \x04\xb3\x04\x04\r\n\x0f\n\x07\x043\x04\0\x02\x01\x01\x12\x04\xb3\x04\
    \x04\x08\n\x0f\n\x07\x043\x04\0\x02\x01\x02\x12\x04\xb3\x04\x0b\x0c\n\
    \x0c\n\x04\x043\x02\0\x12\x04\xb5\x04\x02\x1f\n\r\n\x05\x043\x02\0\x04\
    \x12\x04\xb5\x04\x02\n\n\r\n\x05\x043\x02\0\x06\x12\x04\xb5\x04\x0b\x12\
    \n\r\n\x05\x043\x02\0\x01\x12\x04\xb5\x04\x13\x1a\n\r\n\x05\x043\x02\0\
    \x03\x12\x04\xb5\x04\x1d\x1e\n\x0c\n\x04\x043\x02\x01\x12\x04\xb6\x04\
    \x02\x1e\n\r\n\x05\x043\x02\x01\x04\x12\x04\xb6\x04\x02\n\n\r\n\x05\x043\
    \x02\x01\x05\x12\x04\xb6\x04\x0b\x11\n\r\n\x05\x043\x02\x01\x01\x12\x04\
    \xb6\x04\x12\x19\n\r\n\x05\x043\x02\x01\x03\x12\x04\xb6\x04\x1c\x1d\n\
    \x0c\n\x02\x044\x12\x06\xb9\x04\0\xbd\x04\x01\n\x0b\n\x03\x044\x01\x12\
    \x04\xb9\x04\x08\x13\n8\n\x04\x044\x02\0\x12\x04\xba\x04\x02\x1f\"*\x20O\
    nly\x20populated\x20when\x20using\x20raw\x20interface.\n\n\r\n\x05\x044\
    \x02\0\x04\x12\x04\xba\x04\x02\n\n\r\n\x05\x044\x02\0\x05\x12\x04\xba\
    \x04\x0b\x11\n\r\n\x05\x044\x02\0\x01\x12\x04\xba\x04\x12\x1a\n\r\n\x05\
    \x044\x02\0\x03\x12\x04\xba\x04\x1d\x1e\n\x0c\n\x04\x044\x02\x01\x12\x04\
    \xbb\x04\x02!\n\r\n\x05\x044\x02\x01\x04\x12\x04\xbb\x04\x02\n\n\r\n\x05\
    \x044\x02\x01\x05\x12\x04\xbb\x04\x0b\x11\n\r\n\x05\x044\x02\x01\x01\x12\
    \x04\xbb\x04\x12\x1c\n\r\n\x05\x044\x02\x01\x03\x12\x04\xbb\x04\x1f\x20\
    \n\x0c\n\x04\x044\x02\x02\x12\x04\xbc\x04\x02#\n\r\n\x05\x044\x02\x02\
    \x04\x12\x04\xbc\x04\x02\n\n\r\n\x05\x044\x02\x02\x06\x12\x04\xbc\x04\
    \x0b\x17\n\r\n\x05\x044\x02\x02\x01\x12\x04\xbc\x04\x18\x1e\n\r\n\x05\
    \x044\x02\x02\x03\x12\x04\xbc\x04!\"\n\x0c\n\x02\x05\x03\x12\x06\xbf\x04\
    \0\xc2\x04\x01\n\x0b\n\x03\x05\x03\x01\x12\x04\xbf\x04\x05\n\n\x0c\n\x04\
    \x05\x03\x02\0\x12\x04\xc0\x04\x02\x1c\n\r\n\x05\x05\x03\x02\0\x01\x12\
    \x04\xc0\x04\x02\x17\n\r\n\x05\x05\x03\x02\0\x02\x12\x04\xc0\x04\x1a\x1b\
    \n\x0c\n\x04\x05\x03\x02\x01\x12\x04\xc1\x04\x02\x18\n\r\n\x05\x05\x03\
    \x02\x01\x01\x12\x04\xc1\x04\x02\x13\n\r\n\x05\x05\x03\x02\x01\x02\x12\
    \x04\xc1\x04\x16\x17\n\x0c\n\x02\x05\x04\x12\x06\xc4\x04\0\xc9\x04\x01\n\
    \x0b\n\x03\x05\x04\x01\x12\x04\xc4\x04\x05\x0b\n\x0c\n\x04\x05\x04\x02\0\
    \x12\x04\xc5\x04\x02\x0e\n\r\n\x05\x05\x04\x02\0\x01\x12\x04\xc5\x04\x02\
    \t\n\r\n\x05\x05\x04\x02\0\x02\x12\x04\xc5\x04\x0c\r\n\x0c\n\x04\x05\x04\
    \x02\x01\x12\x04\xc6\x04\x02\r\n\r\n\x05\x05\x04\x02\x01\x01\x12\x04\xc6\
    \x04\x02\x08\n\r\n\x05\x05\x04\x02\x01\x02\x12\x04\xc6\x04\x0b\x0c\n\x0c\
    \n\x04\x05\x04\x02\x02\x12\x04\xc7\x04\x02\n\n\r\n\x05\x05\x04\x02\x02\
    \x01\x12\x04\xc7\x04\x02\x05\n\r\n\x05\x05\x04\x02\x02\x02\x12\x04\xc7\
    \x04\x08\t\n\x0c\n\x04\x05\x04\x02\x03\x12\x04\xc8\x04\x02\x10\n\r\n\x05\
    \x05\x04\x02\x03\x01\x12\x04\xc8\x04\x02\x0b\n\r\n\x05\x05\x04\x02\x03\
    \x02\x12\x04\xc8\x04\x0e\x0f\n\x0c\n\x02\x045\x12\x06\xcb\x04\0\xce\x04\
    \x01\n\x0b\n\x03\x045\x01\x12\x04\xcb\x04\x08\x14\n\x0c\n\x04\x045\x02\0\
    \x12\x04\xcc\x04\x02\x20\n\r\n\x05\x045\x02\0\x04\x12\x04\xcc\x04\x02\n\
    \n\r\n\x05\x045\x02\0\x05\x12\x04\xcc\x04\x0b\x11\n\r\n\x05\x045\x02\0\
    \x01\x12\x04\xcc\x04\x12\x1b\n\r\n\x05\x045\x02\0\x03\x12\x04\xcc\x04\
    \x1e\x1f\n\x0c\n\x04\x045\x02\x01\x12\x04\xcd\x04\x02\x1d\n\r\n\x05\x045\
    \x02\x01\x04\x12\x04\xcd\x04\x02\n\n\r\n\x05\x045\x02\x01\x06\x12\x04\
    \xcd\x04\x0b\x11\n\r\n\x05\x045\x02\x01\x01\x12\x04\xcd\x04\x12\x18\n\r\
    \n\x05\x045\x02\x01\x03\x12\x04\xcd\x04\x1b\x1c\
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
