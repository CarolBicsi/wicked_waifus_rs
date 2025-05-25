use wicked_waifus_protocol::{ErrorCode, InputSettingRequest, InputSettingResponse, InputSettingUpdateRequest, InputSettingUpdateResponse, LanguageSettingUpdateRequest, LanguageSettingUpdateResponse, MonthCardRequest, MonthCardResponse, ServerPlayStationPlayOnlyStateRequest, ServerPlayStationPlayOnlyStateResponse, UpdateVoxelEnvRequest, UpdateVoxelEnvResponse, VersionInfoPush, WebSignRequest, WebSignResponse, TimeStopPush, Zih};

use crate::logic::player::Player;

pub fn on_month_card_request(
    player: &mut Player,
    _: MonthCardRequest,
    response: &mut MonthCardResponse,
) {
    // TODO: Check if we should send MonthCardUseNotify
    response.days = player.month_card.days;
    response.is_daily_got = wicked_waifus_commons::time_util::unix_days() == player.month_card.last_received_day;
    response.error_code = ErrorCode::Success.into();
}

pub fn on_web_sign_request(
    _: &mut Player,
    _: WebSignRequest,
    response: &mut WebSignResponse,
) {
    response.notice_sign = "Welcome to Wicked Waifus PS provided by Reversed Rooms Dev Team".to_string();
}

pub fn on_input_setting_request(
    _: &Player,
    _: InputSettingRequest,
    response: &mut InputSettingResponse,
) {
    response.zih = Some(Zih::default());
}

pub fn on_input_setting_update_request(
    _: &Player,
    _: InputSettingUpdateRequest,
    response: &mut InputSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_language_setting_update_request(
    _: &Player,
    _: LanguageSettingUpdateRequest,
    response: &mut LanguageSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_server_play_station_play_only_state_request(
    _: &Player,
    _: ServerPlayStationPlayOnlyStateRequest,
    response: &mut ServerPlayStationPlayOnlyStateResponse,
) {
    response.cross_play_enabled = false;
}

pub fn on_version_info_push(_player: &Player, push: VersionInfoPush) {
    // TODO: Shall we do safety check and ensure we have compatible versions?
    tracing::debug!(
        "Client versions: launcher: {}, app: {}, resources: {}",
        push.launcher_version,
        push.app_version,
        push.resource_version
    );
}

pub fn on_update_voxel_env_request(
    _: &Player,
    request: UpdateVoxelEnvRequest,
    response: &mut UpdateVoxelEnvResponse,
) {
    response.server_cave_mode = request.server_cave_mode;
    response.error_code = ErrorCode::Success.into();
}

pub fn on_time_check_request(
    _: &Player,
    request: wicked_waifus_protocol::TimeCheckRequest,
    response: &mut wicked_waifus_protocol::TimeCheckResponse,
) {
    // Set response fields using the available properties
    response.client_time = request.client_time;
    response.server_time = wicked_waifus_commons::time_util::unix_timestamp() as i64;
    response.server_stop_time = 0;  // Not used
    response.scene_stop_time = 0;   // Not used
    response.xo_2 = 0;              // Not used
    response.error_code = wicked_waifus_protocol::ErrorCode::Success.into();
}

pub fn on_time_stop_push(_player: &Player, push: TimeStopPush) {
    // Just log the push for now
    tracing::debug!(
        "Received TimeStopPush with time_dilation: {}",
        push.time_dilation
    );
}
