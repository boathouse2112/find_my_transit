use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ProductInformation {
    pub manufacturer_name: String,
    pub model_name: String,
    pub product_identifier: String,
    pub vendor_identifier: String,
    pub antenna_power: u32,
}

#[derive(Debug, Deserialize)]
pub struct ProductType {
    #[serde(rename = "type")]
    pub type_value: String,
    pub product_information: ProductInformation,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub position_type: String,
    pub vertical_accuracy: i32,
    pub longitude: i32,
    pub floor_level: i32,
    pub is_inaccurate: bool,
    pub is_old: bool,
    pub horizontal_accuracy: i32,
    pub latitude: i32,
    pub time_stamp: String,
    pub altitude: i32,
    pub location_finished: bool,
}

#[derive(Debug, Deserialize)]
pub struct Address {
    pub sub_administrative_area: String,
    pub label: String,
    pub street_address: String,
    pub country_code: String,
    pub state_code: String,
    pub administrative_area: String,
    pub street_name: String,
    pub formatted_address_lines: Vec<String>,
    pub map_item_full_address: String,
    pub full_throroughfare: String,
    pub area_of_interest: Vec<String>,
    pub locality: String,
    pub country: String,
}

#[derive(Debug, Deserialize)]
pub struct SafeLocation {
    #[serde(rename = "type")]
    pub type_value: u32,
    pub approval_state: u32,
    pub name: String,
    pub identifier: String,
    pub location: Location,
    pub address: Address,
}

#[derive(Debug, Deserialize)]
pub struct Role {
    pub name: String,
    pub emojie: String,
    pub identifier: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindMyCacheData {
    pub part_info: Option<String>,
    pub is_firmware_update_mandatory: bool,
    pub product_type: ProductType,
    pub safe_locations: Vec<SafeLocation>,
    pub owner: String,
    pub battery_status: String,
    pub serial_number: String,
    pub lost_mode_metadata: Option<String>,
    pub capabilities: String,
    pub identifier: String,
    pub address: Address,
    pub location: Location,
    pub product_identifier: String,
    pub is_apple_audio_accessory: bool,
    pub crowd_sourced_location: Location,
    pub group_identifier: Option<String>,
    pub role: Role,
    pub system_version: String,
    pub name: String,
}
