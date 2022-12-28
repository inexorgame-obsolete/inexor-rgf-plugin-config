use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;

use crate::behaviour::component::config_file::ConfigFile;
use crate::behaviour::component::config_file::CONFIG_FILE;
use crate::di::*;
use crate::model::ComponentContainer;
use crate::model::ReactiveBehaviourContainer;
use crate::model::ReactiveEntityInstance;
use crate::plugins::ComponentBehaviourProvider;

#[wrapper]
pub struct ConfigFileStorage(RwLock<HashMap<Uuid, Arc<ConfigFile>>>);

#[provides]
fn create_config_file_behaviours_storage() -> ConfigFileStorage {
    ConfigFileStorage(RwLock::new(HashMap::new()))
}

#[async_trait]
pub trait ConfigComponentBehaviourProvider: ComponentBehaviourProvider + Send + Sync {
    fn create_config_file_behaviour(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_config_file_behaviour(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

#[component]
pub struct ConfigComponentBehaviourProviderImpl {
    config_file_behaviours: ConfigFileStorage,
}

interfaces!(ConfigComponentBehaviourProviderImpl: dyn ComponentBehaviourProvider);

impl ConfigComponentBehaviourProviderImpl {}

#[async_trait]
#[provides]
impl ConfigComponentBehaviourProvider for ConfigComponentBehaviourProviderImpl {
    fn create_config_file_behaviour(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        if let Ok(config_file) = ConfigFile::new(entity_instance.clone()) {
            self.config_file_behaviours.0.write().unwrap().insert(id, Arc::new(config_file));
            entity_instance.add_behaviour(CONFIG_FILE);
            debug!("Added behaviour {} to entity instance {}", CONFIG_FILE, id);
        }
    }

    fn remove_config_file_behaviour(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if self.config_file_behaviours.0.write().unwrap().remove(&entity_instance.id).is_some() {
            entity_instance.remove_behaviour(CONFIG_FILE);
            debug!("Removed behaviour {} from entity instance {}", CONFIG_FILE, entity_instance.id);
        }
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.config_file_behaviours.0.write().unwrap().contains_key(&id) && self.config_file_behaviours.0.write().unwrap().remove(&id).is_some() {
            debug!("Removed behaviour {} from entity instance {}", CONFIG_FILE, id);
        }
    }
}

impl ComponentBehaviourProvider for ConfigComponentBehaviourProviderImpl {
    fn add_behaviours_to_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.is_a(CONFIG_FILE) {
            self.create_config_file_behaviour(entity_instance);
        }
    }

    fn add_behaviours_to_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        match component.name.as_str() {
            CONFIG_FILE => self.create_config_file_behaviour(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_from_entity(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        if entity_instance.behaves_as(CONFIG_FILE) {
            self.remove_config_file_behaviour(entity_instance);
        }
    }

    fn remove_behaviours_from_entity_component(&self, entity_instance: Arc<ReactiveEntityInstance>, component: crate::model::Component) {
        match component.name.as_str() {
            CONFIG_FILE => self.remove_config_file_behaviour(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}
