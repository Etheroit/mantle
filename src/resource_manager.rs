use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    resources::ResourceManager,
    roblox_api::{
        CreateDeveloperProductResponse, CreateExperienceResponse, CreatePlaceResponse,
        ExperienceConfigurationModel, GetDeveloperProductResponse, GetExperienceResponse,
        GetPlaceResponse, PlaceConfigurationModel, RobloxApi, UploadImageResponse,
    },
    roblox_auth::RobloxAuth,
};

pub type AssetId = u64;

pub mod resource_types {
    pub const EXPERIENCE: &str = "experience";
    pub const EXPERIENCE_CONFIGURATION: &str = "experienceConfiguration";
    pub const EXPERIENCE_ACTIVATION: &str = "experienceActivation";
    pub const EXPERIENCE_ICON: &str = "experienceIcon";
    pub const EXPERIENCE_THUMBNAIL: &str = "experienceThumbnail";
    pub const EXPERIENCE_THUMBNAIL_ORDER: &str = "experienceThumbnailOrder";
    pub const EXPERIENCE_DEVELOPER_PRODUCT: &str = "experienceDeveloperProduct";
    pub const EXPERIENCE_DEVELOPER_PRODUCT_ICON: &str = "experienceDeveloperProductIcon";
    pub const PLACE: &str = "place";
    pub const PLACE_FILE: &str = "placeFile";
    pub const PLACE_CONFIGURATION: &str = "placeConfiguration";
}

pub const SINGLETON_RESOURCE_ID: &str = "singleton";

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceInputs {
    asset_id: Option<AssetId>,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceOutputs {
    asset_id: AssetId,
    start_place_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceConfigurationInputs {
    experience_id: AssetId,
    configuration: ExperienceConfigurationModel,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceActivationInputs {
    experience_id: AssetId,
    is_active: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceThumbnailInputs {
    experience_id: AssetId,
    file_path: String,
    file_hash: String,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceThumbnailOutputs {
    asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceIconInputs {
    experience_id: AssetId,
    file_path: String,
    file_hash: String,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceIconOutputs {
    asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceDeveloperProductIconInputs {
    experience_id: AssetId,
    file_path: String,
    file_hash: String,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceDeveloperProductIconOutputs {
    asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceThumbnailOrderInputs {
    experience_id: AssetId,
    asset_ids: Vec<AssetId>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceDeveloperProductInputs {
    experience_id: AssetId,
    name: String,
    price: u32,
    description: String,
    icon_asset_id: Option<AssetId>,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ExperienceDeveloperProductOutputs {
    asset_id: AssetId,
    product_id: AssetId,
    shop_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlaceInputs {
    experience_id: AssetId,
    start_place_id: AssetId,
    asset_id: Option<AssetId>,
    is_start: bool,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlaceOutputs {
    asset_id: AssetId,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlaceFileInputs {
    asset_id: AssetId,
    file_path: String,
    file_hash: String,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlaceFileOutputs {
    #[serde(default)]
    version: u32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlaceConfigurationInputs {
    asset_id: AssetId,
    configuration: PlaceConfigurationModel,
}

pub struct RobloxResourceManager {
    roblox_api: RobloxApi,
    project_path: PathBuf,
}

impl RobloxResourceManager {
    pub fn new(project_path: &Path) -> Self {
        Self {
            roblox_api: RobloxApi::new(RobloxAuth::new()),
            project_path: project_path.to_path_buf(),
        }
    }
}

impl ResourceManager for RobloxResourceManager {
    fn create(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
    ) -> Result<Option<serde_yaml::Value>, String> {
        match resource_type {
            resource_types::EXPERIENCE => {
                let inputs = serde_yaml::from_value::<ExperienceInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                let outputs = match inputs.asset_id {
                    Some(asset_id) => {
                        let GetExperienceResponse { root_place_id } =
                            self.roblox_api.get_experience(asset_id)?;
                        ExperienceOutputs {
                            asset_id,
                            start_place_id: root_place_id,
                        }
                    }
                    None => {
                        let CreateExperienceResponse {
                            universe_id,
                            root_place_id,
                        } = self.roblox_api.create_experience()?;
                        ExperienceOutputs {
                            asset_id: universe_id,
                            start_place_id: root_place_id,
                        }
                    }
                };

                Ok(Some(serde_yaml::to_value(outputs).map_err(|e| {
                    format!("Failed to serialize outputs: {}", e)
                })?))
            }
            resource_types::EXPERIENCE_CONFIGURATION => {
                let inputs =
                    serde_yaml::from_value::<ExperienceConfigurationInputs>(resource_inputs)
                        .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                self.roblox_api
                    .configure_experience(inputs.experience_id, &inputs.configuration)?;

                Ok(None)
            }
            resource_types::EXPERIENCE_ACTIVATION => {
                let inputs = serde_yaml::from_value::<ExperienceActivationInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                self.roblox_api
                    .set_experience_active(inputs.experience_id, inputs.is_active)?;

                Ok(None)
            }
            resource_types::EXPERIENCE_ICON => {
                let inputs = serde_yaml::from_value::<ExperienceIconInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                let UploadImageResponse { target_id } = self.roblox_api.upload_icon(
                    inputs.experience_id,
                    self.project_path.join(inputs.file_path).as_path(),
                )?;

                Ok(Some(
                    serde_yaml::to_value(ExperienceIconOutputs {
                        asset_id: target_id,
                    })
                    .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                ))
            }
            resource_types::EXPERIENCE_THUMBNAIL => {
                let inputs = serde_yaml::from_value::<ExperienceThumbnailInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                let UploadImageResponse { target_id } = self.roblox_api.upload_thumbnail(
                    inputs.experience_id,
                    self.project_path.join(inputs.file_path).as_path(),
                )?;

                Ok(Some(
                    serde_yaml::to_value(ExperienceThumbnailOutputs {
                        asset_id: target_id,
                    })
                    .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                ))
            }
            resource_types::EXPERIENCE_THUMBNAIL_ORDER => {
                let inputs =
                    serde_yaml::from_value::<ExperienceThumbnailOrderInputs>(resource_inputs)
                        .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                self.roblox_api
                    .set_experience_thumbnail_order(inputs.experience_id, &inputs.asset_ids)?;

                Ok(None)
            }
            resource_types::EXPERIENCE_DEVELOPER_PRODUCT_ICON => {
                let inputs =
                    serde_yaml::from_value::<ExperienceDeveloperProductIconInputs>(resource_inputs)
                        .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                let asset_id = self.roblox_api.create_experience_developer_product_icon(
                    inputs.experience_id,
                    self.project_path.join(inputs.file_path).as_path(),
                )?;

                Ok(Some(
                    serde_yaml::to_value(ExperienceDeveloperProductIconOutputs { asset_id })
                        .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                ))
            }
            resource_types::EXPERIENCE_DEVELOPER_PRODUCT => {
                let inputs =
                    serde_yaml::from_value::<ExperienceDeveloperProductInputs>(resource_inputs)
                        .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                let CreateDeveloperProductResponse { id, shop_id } =
                    self.roblox_api.create_experience_developer_product(
                        inputs.experience_id,
                        inputs.name,
                        inputs.price,
                        inputs.description,
                        inputs.icon_asset_id,
                    )?;

                let GetDeveloperProductResponse {
                    product_id,
                    developer_product_id: _,
                } = self
                    .roblox_api
                    .find_experience_developer_product_by_id(inputs.experience_id, id)?;

                Ok(Some(
                    serde_yaml::to_value(ExperienceDeveloperProductOutputs {
                        asset_id: product_id,
                        product_id: id,
                        shop_id,
                    })
                    .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                ))
            }
            resource_types::PLACE => {
                let inputs = serde_yaml::from_value::<PlaceInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                let outputs = match (inputs.is_start, inputs.asset_id) {
                    (false, None) => {
                        let CreatePlaceResponse { place_id, .. } =
                            self.roblox_api.create_place(inputs.experience_id)?;
                        PlaceOutputs { asset_id: place_id }
                    }
                    (true, None) => PlaceOutputs {
                        asset_id: inputs.start_place_id,
                    },
                    (_, Some(asset_id)) => PlaceOutputs { asset_id },
                };

                Ok(Some(serde_yaml::to_value(outputs).map_err(|e| {
                    format!("Failed to serialize outputs: {}", e)
                })?))
            }
            resource_types::PLACE_FILE => {
                let inputs = serde_yaml::from_value::<PlaceFileInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                self.roblox_api.upload_place(
                    self.project_path.join(inputs.file_path).as_path(),
                    inputs.asset_id,
                )?;
                let GetPlaceResponse {
                    current_saved_version,
                } = self.roblox_api.get_place(inputs.asset_id)?;

                Ok(Some(
                    serde_yaml::to_value(PlaceFileOutputs {
                        version: current_saved_version,
                    })
                    .map_err(|e| format!("Failed to serialize outputs: {}", e))?,
                ))
            }
            resource_types::PLACE_CONFIGURATION => {
                let inputs = serde_yaml::from_value::<PlaceConfigurationInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;

                self.roblox_api
                    .configure_place(inputs.asset_id, &inputs.configuration)?;

                Ok(None)
            }
            _ => panic!(
                "Create not implemented for resource type: {}",
                resource_type
            ),
        }
    }

    fn update(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
        resource_outputs: serde_yaml::Value,
    ) -> Result<Option<serde_yaml::Value>, String> {
        match resource_type {
            resource_types::EXPERIENCE => self.create(resource_type, resource_inputs),
            resource_types::EXPERIENCE_CONFIGURATION => self.create(resource_type, resource_inputs),
            resource_types::EXPERIENCE_ACTIVATION => self.create(resource_type, resource_inputs),
            resource_types::EXPERIENCE_ICON => self.create(resource_type, resource_inputs),
            resource_types::EXPERIENCE_THUMBNAIL => {
                self.delete(resource_type, resource_inputs.clone(), resource_outputs)?;
                self.create(resource_type, resource_inputs)
            }
            resource_types::EXPERIENCE_THUMBNAIL_ORDER => {
                self.create(resource_type, resource_inputs)
            }
            // TODO: is this correct?
            resource_types::PLACE => self.create(resource_type, resource_inputs),
            resource_types::PLACE_FILE => self.create(resource_type, resource_inputs),
            resource_types::PLACE_CONFIGURATION => self.create(resource_type, resource_inputs),
            resource_types::EXPERIENCE_DEVELOPER_PRODUCT_ICON => {
                self.create(resource_type, resource_inputs)
            }
            resource_types::EXPERIENCE_DEVELOPER_PRODUCT => {
                let inputs =
                    serde_yaml::from_value::<ExperienceDeveloperProductInputs>(resource_inputs)
                        .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;
                let outputs = serde_yaml::from_value::<ExperienceDeveloperProductOutputs>(
                    resource_outputs.clone(),
                )
                .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;

                self.roblox_api.update_experience_developer_product(
                    inputs.experience_id,
                    outputs.asset_id,
                    inputs.name,
                    inputs.price,
                    inputs.description,
                    inputs.icon_asset_id,
                )?;

                Ok(Some(resource_outputs))
            }
            _ => panic!(
                "Update not implemented for resource type: {}",
                resource_type
            ),
        }
    }

    fn delete(
        &mut self,
        resource_type: &str,
        resource_inputs: serde_yaml::Value,
        resource_outputs: serde_yaml::Value,
    ) -> Result<(), String> {
        match resource_type {
            resource_types::EXPERIENCE => {
                let outputs = serde_yaml::from_value::<ExperienceOutputs>(resource_outputs)
                    .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;

                self.roblox_api.configure_experience(
                    outputs.asset_id,
                    &ExperienceConfigurationModel {
                        genre: None,
                        playable_devices: None,
                        is_friends_only: None,
                        allow_private_servers: None,
                        private_server_price: None,
                        is_for_sale: None,
                        price: None,
                        studio_access_to_apis_allowed: None,
                        permissions: None,
                        universe_avatar_type: None,
                        universe_animation_type: None,
                        universe_collision_type: None,
                        is_archived: Some(true),
                    },
                )?;

                Ok(())
            }
            resource_types::EXPERIENCE_CONFIGURATION => Ok(()),
            resource_types::EXPERIENCE_ICON => {
                // TODO: figure out which endpoint to use to delete an icon
                Ok(())
            }
            resource_types::EXPERIENCE_THUMBNAIL => {
                let inputs = serde_yaml::from_value::<ExperienceThumbnailInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;
                let outputs =
                    serde_yaml::from_value::<ExperienceThumbnailOutputs>(resource_outputs)
                        .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;

                self.roblox_api
                    .delete_experience_thumbnail(inputs.experience_id, outputs.asset_id)
            }
            resource_types::EXPERIENCE_THUMBNAIL_ORDER => Ok(()),
            resource_types::EXPERIENCE_DEVELOPER_PRODUCT_ICON => Ok(()),
            resource_types::EXPERIENCE_DEVELOPER_PRODUCT => {
                let inputs =
                    serde_yaml::from_value::<ExperienceDeveloperProductInputs>(resource_inputs)
                        .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;
                let outputs =
                    serde_yaml::from_value::<ExperienceDeveloperProductOutputs>(resource_outputs)
                        .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;

                let utc = Utc::now();
                self.roblox_api.update_experience_developer_product(
                    inputs.experience_id,
                    outputs.asset_id,
                    format!("zzz_DEPRECATED({})", utc.format("%F %T%.f")),
                    inputs.price,
                    format!(
                        "Name: {}\nDescription:\n{}",
                        inputs.name, inputs.description
                    ),
                    inputs.icon_asset_id,
                )
            }
            resource_types::PLACE => {
                let inputs = serde_yaml::from_value::<PlaceInputs>(resource_inputs)
                    .map_err(|e| format!("Failed to deserialize inputs: {}", e))?;
                let outputs = serde_yaml::from_value::<PlaceOutputs>(resource_outputs)
                    .map_err(|e| format!("Failed to deserialize outputs: {}", e))?;

                if inputs.is_start {
                    return Err("Cannot delete the start place of an experience. Try creating a new experience instead.".to_owned());
                }
                self.roblox_api
                    .remove_place_from_experience(inputs.experience_id, outputs.asset_id)?;

                Ok(())
            }
            resource_types::PLACE_FILE => Ok(()),
            resource_types::PLACE_CONFIGURATION => Ok(()),
            _ => panic!(
                "Delete not implemented for resource type: {}",
                resource_type
            ),
        }
    }
}
