// filepath: d:\Wuwa\wicked_waifus_rs\wicked-waifus-game-server\src\logic\handler\inventory.rs
use wicked_waifus_protocol::{ItemExchangeInfo, ItemExchangeInfoRequest, ItemExchangeInfoResponse, NormalItemRequest, NormalItemResponse, PhantomItemRequest, PhantomItemResponse, WeaponItem, WeaponItemRequest, WeaponItemResponse};

use crate::logic::player::Player;

pub fn on_normal_item_request(
    player: &mut Player,
    _: NormalItemRequest,
    response: &mut NormalItemResponse,
) {
    tracing::debug!("Received NormalItemRequest, returning player inventory");
    response.normal_item_list = player.inventory.to_normal_item_list();
}

pub fn on_weapon_item_request(
    player: &mut Player,
    _: WeaponItemRequest,
    response: &mut WeaponItemResponse,
) {
    tracing::debug!("Received WeaponItemRequest, returning player weapons");

    // Collect all weapons from roles
    let mut weapons: Vec<WeaponItem> = Vec::new();
    let mut incr_id = 1; // Start incr_id from 1 for unique identification in the response

    // Debug log the role weapons
    for role in player.role_list.values() {
        tracing::debug!(
            "Role ID: {}, has weapon ID: {}",
            role.role_id,
            role.equip_weapon
        );
    }

    // Create weapon items
    for role in player.role_list.values() {
        // Only add if weapon ID is valid (typically >= 20000000 and < 30000000 for actual weapons)
        // This check ensures we only process actual weapons and not placeholders like 0.
        if role.equip_weapon >= 20000000 && role.equip_weapon < 30000000 {
            // Create basic weapon item
            let weapon_item = WeaponItem {
                id: role.equip_weapon,
                incr_id,
                func_value: 0,          // Default or specific logic if needed
                weapon_level: 90,       // Hardcoded max level
                weapon_exp: 0,          // Default or specific logic if needed
                weapon_breach: 6,       // Hardcoded max breakthrough
                weapon_reson_level: 6,  // Hardcoded max resonance (constellation)
                role_id: role.role_id,  // Associate weapon with role
                ..Default::default()    // Fill other fields with default values
            };

            tracing::debug!(
                "Adding weapon to response - ID: {}, incr_id: {}, role_id: {}",
                weapon_item.id,
                weapon_item.incr_id,
                weapon_item.role_id
            );

            weapons.push(weapon_item);
            incr_id += 1; // Increment incr_id for the next weapon
        }
    }

    tracing::debug!("Returning {} weapons in response", weapons.len());
    response.weapon_item_list = weapons;
}

pub fn on_phantom_item_request(
    _player: &mut Player,
    _: PhantomItemRequest,
    response: &mut PhantomItemResponse,
) {
    // Set an empty response rather than leaving it unhandled
    tracing::debug!("Received PhantomItemRequest, returning empty response");
    response.phantom_item_list = Vec::new(); // Return an empty list
}

pub fn on_item_exchange_info_request(
    _player: &mut Player,
    _: ItemExchangeInfoRequest,
    response: &mut ItemExchangeInfoResponse,
) {
    response.item_exchange_infos = wicked_waifus_data::item_exchange_content_data::iter()
        .map(|item_exchange_content_data| ItemExchangeInfo {
            item_id: item_exchange_content_data.item_id,
            today_times: 0, // TODO: For stats only, not used for PS so far
            total_times: 0, // TODO: For stats only, not used for PS so far
            daily_limit: 0, // At the time of writing there is no limits
            total_limit: 0, // At the time of writing there is no limits
        })
        .collect();
}

