use wicked_waifus_protocol::combat_message::{
    combat_notify_data, combat_receive_data, combat_request_data, combat_response_data,
    combat_send_data, CombatNotifyData, CombatReceiveData, CombatRequestData, CombatResponseData,
    CombatSendPackRequest, CombatSendPackResponse,
};
use wicked_waifus_protocol::{
    AttributeChangedNotify, CombatCommon, DErrorResult, DamageExecuteRequest,
    DamageExecuteResponse, EAttributeType, ERemoveEntityType, ErrorCode,
    FsmConditionPassRequest, FsmConditionPassResponse, GameplayAttributeData,
    PlayerBattleStateChangeNotify, SwitchRoleRequest, SwitchRoleResponse,
};

use wicked_waifus_data::damage_data;

use crate::logic::ecs::component::ComponentContainer;
use crate::logic::player::{Element, Player};
use crate::logic::utils::world_util;
use crate::query_components;
use rand::Rng;

#[inline(always)]
fn create_combat_response(
    combat_request: &CombatRequestData,
    message: combat_response_data::Message,
) -> CombatReceiveData {
    CombatReceiveData {
        message: Some(combat_receive_data::Message::CombatResponseData(
            CombatResponseData {
                combat_common: combat_request.combat_common,
                request_id: combat_request.request_id,
                message: Some(message),
            },
        )),
    }
}

#[inline(always)]
fn create_combat_notify(
    combat_common: CombatCommon,
    message: combat_notify_data::Message,
) -> CombatReceiveData {
    CombatReceiveData {
        message: Some(combat_receive_data::Message::CombatNotifyData(
            CombatNotifyData {
                combat_common: Some(combat_common),
                message: Some(message),
            },
        )),
    }
}

pub fn on_combat_message_combat_send_pack_request(
    player: &mut Player,
    request: CombatSendPackRequest,
    response: &mut CombatSendPackResponse,
) {
    for data in request.data.iter() {
        if let Some(combat_send_data::Message::Request(ref request_data)) = data.message {
            if let Some(ref request_message) = request_data.message {
                match request_message {
                    combat_request_data::Message::SwitchRoleRequest(ref request) => {
                        handle_switch_role_request(player, request_data, request, response);
                    }
                    combat_request_data::Message::FsmConditionPassRequest(ref request) => {
                        handle_fsm_condition_request(player, request_data, request, response);
                    }
                    combat_request_data::Message::DamageExecuteRequest(ref request) => {
                        handle_damage_execute_request(player, request_data, request, response);
                    }
                    _ => {}
                }
            }
        }
    }
    response.error_code = ErrorCode::Success.into();
}

fn handle_switch_role_request(
    player: &mut Player,
    combat_request: &CombatRequestData,
    request: &SwitchRoleRequest,
    response: &mut CombatSendPackResponse,
) {
    // Find current formation and update current role
    if let Some(formation) = player.formation_list.values_mut().find(|f| f.is_current) {
        formation.cur_role = request.role_id;

        let receive_pack = response
            .receive_pack_notify
            .get_or_insert_with(Default::default);

        receive_pack.data.push(create_combat_response(
            combat_request,
            combat_response_data::Message::SwitchRoleResponse(SwitchRoleResponse {
                error_code: ErrorCode::Success.into(),
                role_id: request.role_id,
            }),
        ));
    } else {
        tracing::error!("Role with id {} not found", request.role_id);
        response.error_code = ErrorCode::ErrSwitchRoleEntityNotExist.into();
        return;
    }

    response.error_code = ErrorCode::Success.into();
}

fn handle_damage_execute_request(
    player: &mut Player,
    combat_request: &CombatRequestData,
    request: &DamageExecuteRequest,
    response: &mut CombatSendPackResponse,
) {
    let receive_pack = response
        .receive_pack_notify
        .get_or_insert_with(Default::default);

    let mut world_ref = player.world.borrow_mut();
    let world = world_ref.get_mut_world_entity();
    let config_id = world.get_config_id(request.attacker_entity_id.try_into().unwrap());
    let mut damage = 1; // TODO: Fix the formula with real parameters(10 field equation)
    if config_id.to_string().len() == 4 {
        if let Some(damage_data) = damage_data::iter().find(|d| d.id == request.damage_id) {
            let attribute = query_components!(world, request.attacker_entity_id, Attribute)
                .0
                .unwrap();
            if let Ok(related_attribute) = EAttributeType::try_from(damage_data.related_property) {
                if let Some((value, _)) = attribute.attr_map.get(&related_attribute) {
                    if let Some(&rate_lv) = damage_data
                        .rate_lv
                        .iter()
                        .find(|&lvl| *lvl == request.skill_level)
                    {
                        let hardness_lv = damage_data.hardness_lv[0];
                        tracing::info!(
                            "atk: {}, damage_id: {}, role_id: {}, rate_lv: {}, hardness_lv: {}",
                            value,
                            request.damage_id,
                            config_id,
                            rate_lv,
                            hardness_lv
                        );
                        damage = if hardness_lv == 0 || rate_lv <= 0 {
                            1
                        } else {
                            ((rate_lv as f32 / hardness_lv as f32) * 100.0 + (*value as f32)) as i32
                        };
                    }
                }
            };
        }
    }
    // สุ่มดาเมจพื้นฐาน (เช่น 3,000 - 8,000)
    let base_damage = rand::rng().random_range(1..=999999);
    let mut damage = base_damage;

    // คริเรท 80% => สุ่มค่า true ได้ 80%
    let crit_rate = 0.8;
    let crit_multiplier = 2.56;
    let is_crit = rand::rng().random_bool(crit_rate);

    if is_crit {
        damage = (damage as f32 * crit_multiplier).min(i32::MAX as f32) as i32;
    }

    // ดึง element type ถ้าใช้
    let element = damage_data::iter()
        .find(|d| d.id == request.damage_id)
        .map(|d| d.element_power_type)
        .unwrap_or(0);

    // Log ข้อมูล
    tracing::info!(
    "Executing damage: {}, Target: {}, Crit?: {}, Element: {}",
    damage,
    request.target_entity_id,
    is_crit,
    element
);

    // Push combat response
    receive_pack.data.push(create_combat_response(
        combat_request,
        combat_response_data::Message::DamageExecuteResponse(DamageExecuteResponse {
            error_code: ErrorCode::Success.into(),
            attacker_entity_id: request.attacker_entity_id,
            target_entity_id: request.target_entity_id,
            part_index: request.part_index,
            damage,
            e_k: element,
            is_crit,
            ..Default::default()
        }),
    ));

    if let Some((value, _)) = query_components!(world, request.target_entity_id, Attribute)
        .0
        .unwrap()
        .attr_map
        .get(&EAttributeType::Life)
    {
        let updated_value = match value - damage >= 0 {
            true => value - damage,
            false => 0,
        };
        receive_pack.data.push(create_combat_notify(
            CombatCommon {
                entity_id: request.target_entity_id,
                ..Default::default()
            },
            combat_notify_data::Message::AttributeChangedNotify(AttributeChangedNotify {
                id: request.target_entity_id,
                attributes: vec![GameplayAttributeData {
                    current_value: updated_value,
                    value_increment: updated_value,
                    attribute_type: EAttributeType::Life.into(),
                }],
            }),
        ));
        // dont enable this if you dont want to get bugged

        // if updated_value == 0 {
        //     world_util::remove_entity(
        //         player,
        //         request.target_entity_id,
        //         ERemoveEntityType::HpIsZero,
        //     );
        // }
    }

    response.error_code = ErrorCode::Success.into();
}

fn handle_battle(
    player: &mut Player,
    combat_request: &CombatRequestData,
    response: &mut CombatSendPackResponse,
    condition: bool,
) {
    let receive_pack = response
        .receive_pack_notify
        .get_or_insert_with(Default::default);

    receive_pack.data.push(create_combat_notify(
        combat_request.combat_common.unwrap(),
        combat_notify_data::Message::PlayerBattleStateChangeNotify(PlayerBattleStateChangeNotify {
            player_id: player.basic_info.id,
            in_battle: condition,
        }),
    ));
}

fn handle_fsm_condition_request(
    player: &mut Player,
    combat_request: &CombatRequestData,
    request: &FsmConditionPassRequest,
    response: &mut CombatSendPackResponse,
) {
    let receive_pack = response
        .receive_pack_notify
        .get_or_insert_with(Default::default);

    receive_pack.data.push(create_combat_response(
        combat_request,
        combat_response_data::Message::FsmConditionPassResponse(FsmConditionPassResponse {
            fsm_id: request.fsm_id,

            error: Some(DErrorResult {
                error_code: ErrorCode::Success.into(),

                error_params: Vec::new(),
            }),
        }),
    ));
    handle_battle(player, combat_request, response, true);
}
