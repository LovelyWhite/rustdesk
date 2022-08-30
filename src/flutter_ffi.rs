use std::{
    collections::HashMap,
    ffi::{CStr, CString},
    os::raw::c_char,
};

use flutter_rust_bridge::{StreamSink, SyncReturn, ZeroCopyBuffer};
use serde_json::{json, Number, Value};

use hbb_common::{
    config::{self, Config, LocalConfig, PeerConfig, ONLINE},
    fs, log,
};
use hbb_common::{password_security, ResultType};

use crate::client::file_trait::FileManager;
use crate::common::make_fd_to_json;
use crate::flutter::connection_manager::{self, get_clients_length, get_clients_state};
use crate::flutter::{self, Session, SESSIONS};
use crate::start_server;
use crate::ui_interface;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use crate::ui_interface::{change_id, check_connect_status, is_ok_change_id};
use crate::ui_interface::{
    check_mouse_time, check_super_user_permission, discover, forget_password, get_api_server,
    get_app_name, get_async_job_status, get_connect_status, get_fav, get_id, get_lan_peers,
    get_langs, get_license, get_local_option, get_mouse_time, get_option, get_options, get_peer,
    get_peer_option, get_socks, get_sound_inputs, get_uuid, get_version, has_hwcodec,
    has_rendezvous_service, post_request, set_local_option, set_option, set_options,
    set_peer_option, set_permanent_password, set_socks, store_fav, test_if_valid_server,
    update_temporary_password, using_public_server,
};

fn initialize(app_dir: &str) {
    *config::APP_DIR.write().unwrap() = app_dir.to_owned();
    #[cfg(feature = "cli")]
    {
        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            crate::common::test_rendezvous_server();
            crate::common::test_nat_type();
        }
    }
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_min_level(log::Level::Debug) // limit log level
                .with_tag("ffi"), // logs will show under mytag tag
        );
    }
    #[cfg(target_os = "ios")]
    {
        use hbb_common::env_logger::*;
        init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "debug"));
    }
    #[cfg(target_os = "android")]
    {
        crate::common::check_software_update();
    }
    #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
    {
        use hbb_common::env_logger::*;
        if let Err(e) = try_init_from_env(Env::default().filter_or(DEFAULT_FILTER_ENV, "debug")) {
            log::debug!("{}", e);
        }
    }
}

/// FFI for rustdesk core's main entry.
/// Return true if the app should continue running with UI(possibly Flutter), false if the app should exit.
#[no_mangle]
pub extern "C" fn rustdesk_core_main() -> bool {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    return crate::core_main::core_main();
    #[cfg(any(target_os = "android", target_os = "ios"))]
    false
}

pub enum EventToUI {
    Event(String),
    Rgba(ZeroCopyBuffer<Vec<u8>>),
}

pub fn start_global_event_stream(s: StreamSink<String>, app_type: String) -> ResultType<()> {
    if let Some(_) = flutter::GLOBAL_EVENT_STREAM
        .write()
        .unwrap()
        .insert(app_type.clone(), s)
    {
        log::warn!(
            "Global event stream of type {} is started before, but now removed",
            app_type
        );
    }
    Ok(())
}

pub fn stop_global_event_stream(app_type: String) {
    let _ = flutter::GLOBAL_EVENT_STREAM
        .write()
        .unwrap()
        .remove(&app_type);
}

pub fn host_stop_system_key_propagate(stopped: bool) {
    #[cfg(windows)]
    crate::platform::windows::stop_system_key_propagate(stopped);
}

// FIXME: -> ResultType<()> cannot be parsed by frb_codegen
// thread 'main' panicked at 'Failed to parse function output type `ResultType<()>`', $HOME\.cargo\git\checkouts\flutter_rust_bridge-ddba876d3ebb2a1e\e5adce5\frb_codegen\src\parser\mod.rs:151:25
pub fn session_add_sync(id: String, is_file_transfer: bool, is_port_forward: bool) -> SyncReturn<String> {
    if let Err(e) = Session::add(&id, is_file_transfer, is_port_forward) {
        SyncReturn(format!("Failed to add session with id {}, {}", &id, e))
    } else {
        SyncReturn("".to_owned())
    }
}

pub fn session_start(events2ui: StreamSink<EventToUI>, id: String) -> ResultType<()> {
    Session::start(&id, events2ui)
}

pub fn session_get_remember(id: String) -> Option<bool> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_remember())
    } else {
        None
    }
}

pub fn session_get_toggle_option(id: String, arg: String) -> Option<bool> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_toggle_option(&arg))
    } else {
        None
    }
}

pub fn session_get_toggle_option_sync(id: String, arg: String) -> SyncReturn<bool> {
    let res = session_get_toggle_option(id, arg) == Some(true);
    SyncReturn(res)
}

pub fn session_get_image_quality(id: String) -> Option<String> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_image_quality())
    } else {
        None
    }
}

pub fn session_get_option(id: String, arg: String) -> Option<String> {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        Some(session.get_option(&arg))
    } else {
        None
    }
}

pub fn session_login(id: String, password: String, remember: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.login(&password, remember);
    }
}

pub fn session_close(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.close();
    }
    let _ = SESSIONS.write().unwrap().remove(&id);
}

pub fn session_refresh(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.refresh();
    }
}

pub fn session_reconnect(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.reconnect();
    }
}

pub fn session_toggle_option(id: String, value: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.toggle_option(&value);
    }
}

pub fn session_set_image_quality(id: String, value: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.set_image_quality(&value);
    }
}

pub fn session_lock_screen(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.lock_screen();
    }
}

pub fn session_ctrl_alt_del(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.ctrl_alt_del();
    }
}

pub fn session_switch_display(id: String, value: i32) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.switch_display(value);
    }
}

pub fn session_input_key(
    id: String,
    name: String,
    down: bool,
    press: bool,
    alt: bool,
    ctrl: bool,
    shift: bool,
    command: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.input_key(&name, down, press, alt, ctrl, shift, command);
    }
}

pub fn session_input_string(id: String, value: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.input_string(&value);
    }
}

// chat_client_mode
pub fn session_send_chat(id: String, text: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.send_chat(text);
    }
}

pub fn session_peer_option(id: String, name: String, value: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.set_option(name, value);
    }
}

pub fn session_get_peer_option(id: String, name: String) -> String {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        return session.get_option(&name);
    }
    "".to_string()
}

pub fn session_input_os_password(id: String, value: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.input_os_password(value, true);
    }
}

// File Action
pub fn session_read_remote_dir(id: String, path: String, include_hidden: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.read_remote_dir(path, include_hidden);
    }
}

pub fn session_send_files(
    id: String,
    act_id: i32,
    path: String,
    to: String,
    file_num: i32,
    include_hidden: bool,
    is_remote: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.send_files(act_id, path, to, file_num, include_hidden, is_remote);
    }
}

pub fn session_set_confirm_override_file(
    id: String,
    act_id: i32,
    file_num: i32,
    need_override: bool,
    remember: bool,
    is_upload: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.set_confirm_override_file(act_id, file_num, need_override, remember, is_upload);
    }
}

pub fn session_remove_file(id: String, act_id: i32, path: String, file_num: i32, is_remote: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.remove_file(act_id, path, file_num, is_remote);
    }
}

pub fn session_read_dir_recursive(
    id: String,
    act_id: i32,
    path: String,
    is_remote: bool,
    show_hidden: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.remove_dir_all(act_id, path, is_remote, show_hidden);
    }
}

pub fn session_remove_all_empty_dirs(id: String, act_id: i32, path: String, is_remote: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.remove_dir(act_id, path, is_remote);
    }
}

pub fn session_cancel_job(id: String, act_id: i32) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.cancel_job(act_id);
    }
}

pub fn session_create_dir(id: String, act_id: i32, path: String, is_remote: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.create_dir(act_id, path, is_remote);
    }
}

pub fn session_read_local_dir_sync(id: String, path: String, show_hidden: bool) -> String {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        if let Ok(fd) = fs::read_dir(&fs::get_path(&path), show_hidden) {
            return make_fd_to_json(fd);
        }
    }
    "".to_string()
}

pub fn session_get_platform(id: String, is_remote: bool) -> String {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        return session.get_platform(is_remote);
    }
    "".to_string()
}

pub fn session_load_last_transfer_jobs(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        return session.load_last_jobs();
    } else {
        // a tip for flutter dev
        eprintln!(
            "cannot load last transfer job from non-existed session. Please ensure session \
        is connected before calling load last transfer jobs."
        );
    }
}

pub fn session_add_job(
    id: String,
    act_id: i32,
    path: String,
    to: String,
    file_num: i32,
    include_hidden: bool,
    is_remote: bool,
) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.add_job(act_id, path, to, file_num, include_hidden, is_remote);
    }
}

pub fn session_resume_job(id: String, act_id: i32, is_remote: bool) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.resume_job(act_id, is_remote);
    }
}

pub fn main_get_sound_inputs() -> Vec<String> {
    get_sound_inputs()
}

pub fn main_change_id(new_id: String) {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    change_id(new_id)
}

pub fn main_get_async_status() -> String {
    get_async_job_status()
}

pub fn main_get_option(key: String) -> String {
    get_option(key)
}

pub fn main_set_option(key: String, value: String) {
    if key.eq("custom-rendezvous-server") {
        set_option(key, value);
        #[cfg(target_os = "android")]
        crate::rendezvous_mediator::RendezvousMediator::restart();
        #[cfg(any(target_os = "android", target_os = "ios", feature = "cli"))]
        crate::common::test_rendezvous_server();
    } else {
        set_option(key, value);
    }
}

pub fn main_get_options() -> String {
    get_options()
}

pub fn main_set_options(json: String) {
    let map: HashMap<String, String> = serde_json::from_str(&json).unwrap_or(HashMap::new());
    if !map.is_empty() {
        set_options(map)
    }
}

pub fn main_test_if_valid_server(server: String) -> String {
    test_if_valid_server(server)
}

pub fn main_set_socks(proxy: String, username: String, password: String) {
    set_socks(proxy, username, password)
}

pub fn main_get_socks() -> Vec<String> {
    get_socks()
}

pub fn main_get_app_name() -> String {
    get_app_name()
}

pub fn main_get_license() -> String {
    get_license()
}

pub fn main_get_version() -> String {
    get_version()
}

pub fn main_get_fav() -> Vec<String> {
    get_fav()
}

pub fn main_store_fav(favs: Vec<String>) {
    store_fav(favs)
}

pub fn main_get_peer(id: String) -> String {
    let conf = get_peer(id);
    serde_json::to_string(&conf).unwrap_or("".to_string())
}

pub fn main_get_lan_peers() -> String {
    serde_json::to_string(&get_lan_peers()).unwrap_or_default()
}

pub fn main_get_connect_status() -> String {
    let status = get_connect_status();
    // (status_num, key_confirmed, mouse_time, id)
    let mut m = serde_json::Map::new();
    m.insert("status_num".to_string(), json!(status.0));
    m.insert("key_confirmed".to_string(), json!(status.1));
    m.insert("mouse_time".to_string(), json!(status.2));
    m.insert("id".to_string(), json!(status.3));
    serde_json::to_string(&m).unwrap_or("".to_string())
}

pub fn main_check_connect_status() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    check_mouse_time(); // avoid multi calls
}

pub fn main_is_using_public_server() -> bool {
    using_public_server()
}

pub fn main_discover() {
    discover();
}

pub fn main_has_rendezvous_service() -> bool {
    has_rendezvous_service()
}

pub fn main_get_api_server() -> String {
    get_api_server()
}

pub fn main_post_request(url: String, body: String, header: String) {
    post_request(url, body, header)
}

pub fn main_get_local_option(key: String) -> String {
    get_local_option(key)
}

pub fn main_set_local_option(key: String, value: String) {
    set_local_option(key, value)
}

pub fn main_get_my_id() -> String {
    get_id()
}

pub fn main_get_uuid() -> String {
    get_uuid()
}

pub fn main_get_peer_option(id: String, key: String) -> String {
    get_peer_option(id, key)
}

pub fn main_set_peer_option(id: String, key: String, value: String) {
    set_peer_option(id, key, value)
}

pub fn main_forget_password(id: String) {
    forget_password(id)
}

// TODO APP_DIR & ui_interface
pub fn main_get_recent_peers() -> String {
    if !config::APP_DIR.read().unwrap().is_empty() {
        let peers: Vec<(String, config::PeerInfoSerde)> = PeerConfig::peers()
            .drain(..)
            .map(|(id, _, p)| (id, p.info))
            .collect();
        serde_json::ser::to_string(&peers).unwrap_or("".to_owned())
    } else {
        String::new()
    }
}

pub fn main_load_recent_peers() {
    if !config::APP_DIR.read().unwrap().is_empty() {
        let peers: Vec<(String, config::PeerInfoSerde)> = PeerConfig::peers()
            .drain(..)
            .map(|(id, _, p)| (id, p.info))
            .collect();
        if let Some(s) = flutter::GLOBAL_EVENT_STREAM
            .read()
            .unwrap()
            .get(flutter::APP_TYPE_MAIN)
        {
            let data = HashMap::from([
                ("name", "load_recent_peers".to_owned()),
                (
                    "peers",
                    serde_json::ser::to_string(&peers).unwrap_or("".to_owned()),
                ),
            ]);
            s.add(serde_json::ser::to_string(&data).unwrap_or("".to_owned()));
        };
    }
}

pub fn main_load_fav_peers() {
    if !config::APP_DIR.read().unwrap().is_empty() {
        let favs = get_fav();
        let peers: Vec<(String, config::PeerInfoSerde)> = PeerConfig::peers()
            .into_iter()
            .filter_map(|(id, _, peer)| {
                if favs.contains(&id) {
                    Some((id, peer.info))
                } else {
                    None
                }
            })
            .collect();
        if let Some(s) = flutter::GLOBAL_EVENT_STREAM
            .read()
            .unwrap()
            .get(flutter::APP_TYPE_MAIN)
        {
            let data = HashMap::from([
                ("name", "load_fav_peers".to_owned()),
                (
                    "peers",
                    serde_json::ser::to_string(&peers).unwrap_or("".to_owned()),
                ),
            ]);
            s.add(serde_json::ser::to_string(&data).unwrap_or("".to_owned()));
        };
    }
}

pub fn main_load_lan_peers() {
    if let Some(s) = flutter::GLOBAL_EVENT_STREAM
        .read()
        .unwrap()
        .get(flutter::APP_TYPE_MAIN)
    {
        let data = HashMap::from([
            ("name", "load_lan_peers".to_owned()),
            (
                "peers",
                serde_json::to_string(&get_lan_peers()).unwrap_or_default(),
            ),
        ]);
        s.add(serde_json::ser::to_string(&data).unwrap_or("".to_owned()));
    };
}

pub fn session_add_port_forward(
    id: String,
    local_port: i32,
    remote_host: String,
    remote_port: i32,
) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.add_port_forward(local_port, remote_host, remote_port);
    }
}

pub fn session_remove_port_forward(id: String, local_port: i32) {
    if let Some(session) = SESSIONS.write().unwrap().get_mut(&id) {
        session.remove_port_forward(local_port);
    }
}

pub fn main_get_last_remote_id() -> String {
    // if !config::APP_DIR.read().unwrap().is_empty() {
    //     res = LocalConfig::get_remote_id();
    // }
    LocalConfig::get_remote_id()
}

pub fn main_get_software_update_url() -> String {
    crate::common::SOFTWARE_UPDATE_URL.lock().unwrap().clone()
}

pub fn main_get_home_dir() -> String {
    fs::get_home_as_string()
}

pub fn main_get_langs() -> String {
    get_langs()
}

pub fn main_get_temporary_password() -> String {
    ui_interface::temporary_password()
}

pub fn main_get_permanent_password() -> String {
    ui_interface::permanent_password()
}

pub fn main_get_online_statue() -> i64 {
    ONLINE.lock().unwrap().values().max().unwrap_or(&0).clone()
}

pub fn main_get_clients_state() -> String {
    get_clients_state()
}

pub fn main_check_clients_length(length: usize) -> Option<String> {
    if length != get_clients_length() {
        Some(get_clients_state())
    } else {
        None
    }
}

pub fn main_init(app_dir: String) {
    initialize(&app_dir);
}

pub fn main_device_id(id: String) {
    *crate::common::DEVICE_ID.lock().unwrap() = id;
}

pub fn main_device_name(name: String) {
    *crate::common::DEVICE_NAME.lock().unwrap() = name;
}

pub fn main_remove_peer(id: String) {
    PeerConfig::remove(&id);
}

pub fn main_has_hwcodec() -> bool {
    has_hwcodec()
}

// TODO
pub fn session_send_mouse(id: String, msg: String) {
    if let Ok(m) = serde_json::from_str::<HashMap<String, String>>(&msg) {
        let alt = m.get("alt").is_some();
        let ctrl = m.get("ctrl").is_some();
        let shift = m.get("shift").is_some();
        let command = m.get("command").is_some();
        let x = m
            .get("x")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .unwrap_or(0);
        let y = m
            .get("y")
            .map(|x| x.parse::<i32>().unwrap_or(0))
            .unwrap_or(0);
        let mut mask = 0;
        if let Some(_type) = m.get("type") {
            mask = match _type.as_str() {
                "down" => 1,
                "up" => 2,
                "wheel" => 3,
                _ => 0,
            };
        }
        if let Some(buttons) = m.get("buttons") {
            mask |= match buttons.as_str() {
                "left" => 1,
                "right" => 2,
                "wheel" => 4,
                _ => 0,
            } << 3;
        }
        if let Some(session) = SESSIONS.read().unwrap().get(&id) {
            session.send_mouse(mask, x, y, alt, ctrl, shift, command);
        }
    }
}

pub fn session_restart_remote_device(id: String) {
    if let Some(session) = SESSIONS.read().unwrap().get(&id) {
        session.restart_remote_device();
    }
}

pub fn main_set_home_dir(home: String) {
    *config::APP_HOME_DIR.write().unwrap() = home;
}

pub fn main_stop_service() {
    #[cfg(target_os = "android")]
    {
        Config::set_option("stop-service".into(), "Y".into());
        crate::rendezvous_mediator::RendezvousMediator::restart();
    }
}

pub fn main_start_service() {
    #[cfg(target_os = "android")]
    {
        Config::set_option("stop-service".into(), "".into());
        crate::rendezvous_mediator::RendezvousMediator::restart();
    }
    #[cfg(not(target_os = "android"))]
    std::thread::spawn(move || start_server(true));
}

pub fn main_update_temporary_password() {
    update_temporary_password();
}

pub fn main_set_permanent_password(password: String) {
    set_permanent_password(password);
}

pub fn main_check_super_user_permission() -> bool {
    check_super_user_permission()
}

pub fn main_check_mouse_time() {
    check_mouse_time();
}

pub fn main_get_mouse_time() -> f64 {
    get_mouse_time()
}

pub fn cm_send_chat(conn_id: i32, msg: String) {
    connection_manager::send_chat(conn_id, msg);
}

pub fn cm_login_res(conn_id: i32, res: bool) {
    connection_manager::on_login_res(conn_id, res);
}

pub fn cm_close_connection(conn_id: i32) {
    connection_manager::close_conn(conn_id);
}

pub fn cm_check_click_time(conn_id: i32) {
    connection_manager::check_click_time(conn_id)
}

pub fn cm_get_click_time() -> f64 {
    connection_manager::get_click_time() as _
}

pub fn cm_switch_permission(conn_id: i32, name: String, enabled: bool) {
    connection_manager::switch_permission(conn_id, name, enabled)
}

pub fn main_get_icon() -> String {
    #[cfg(not(any(target_os = "android", target_os = "ios", feature = "cli")))]
    return ui_interface::get_icon();
    #[cfg(any(target_os = "android", target_os = "ios", feature = "cli"))]
    return String::new();
}

#[no_mangle]
unsafe extern "C" fn translate(name: *const c_char, locale: *const c_char) -> *const c_char {
    let name = CStr::from_ptr(name);
    let locale = CStr::from_ptr(locale);
    let res = if let (Ok(name), Ok(locale)) = (name.to_str(), locale.to_str()) {
        crate::client::translate_locale(name.to_owned(), locale)
    } else {
        String::new()
    };
    CString::from_vec_unchecked(res.into_bytes()).into_raw()
}

fn handle_query_onlines(onlines: Vec<String>, offlines: Vec<String>) {
    if let Some(s) = flutter::GLOBAL_EVENT_STREAM
        .read()
        .unwrap()
        .get(flutter::APP_TYPE_MAIN)
    {
        let data = HashMap::from([
            ("name", "callback_query_onlines".to_owned()),
            ("onlines", onlines.join(",")),
            ("offlines", offlines.join(",")),
        ]);
        s.add(serde_json::ser::to_string(&data).unwrap_or("".to_owned()));
    };
}

pub fn query_onlines(ids: Vec<String>) {
    crate::rendezvous_mediator::query_online_states(ids, handle_query_onlines)
}

#[cfg(target_os = "android")]
pub mod server_side {
    use jni::{
        objects::{JClass, JString},
        sys::jstring,
        JNIEnv,
    };

    use hbb_common::{config::Config, log};

    use crate::start_server;

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_carriez_flutter_1hbb_MainService_startServer(
        env: JNIEnv,
        _class: JClass,
    ) {
        log::debug!("startServer from java");
        std::thread::spawn(move || start_server(true));
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_carriez_flutter_1hbb_MainService_translateLocale(
        env: JNIEnv,
        _class: JClass,
        locale: JString,
        input: JString,
    ) -> jstring {
        let res = if let (Ok(input), Ok(locale)) = (env.get_string(input), env.get_string(locale)) {
            let input: String = input.into();
            let locale: String = locale.into();
            crate::client::translate_locale(input, &locale)
        } else {
            "".into()
        };
        return env.new_string(res).unwrap_or(input).into_inner();
    }

    #[no_mangle]
    pub unsafe extern "system" fn Java_com_carriez_flutter_1hbb_MainService_refreshScreen(
        _env: JNIEnv,
        _class: JClass,
    ) {
        crate::server::video_service::refresh()
    }
}
