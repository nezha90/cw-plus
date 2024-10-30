use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Serialize,Deserialize};

use crate::type_order::Resource;
use crate::ContractError;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct TotalResource {
    pub total: Resource,
    pub used: Resource,
}

pub const RESOURCE: Item<TotalResource> = Item::new("total_resource");

impl TotalResource {
    pub fn add_used(&mut self, resource: Resource) -> Result<(), ContractError> {
        // 不能使当前使用量+新增使用量 大于 总量
        if self.used.cpu + resource.cpu > self.total.cpu ||
            self.used.memory + resource.memory > self.total.memory ||
            self.used.disk + resource.disk > self.total.disk {

            return Err(ContractError::InsufficientCapacity)
        }

        self.used.cpu += resource.cpu;
        self.used.memory += resource.memory;
        self.used.disk += resource.disk;

        Ok(())
    }

    pub fn sub_used(&mut self, resource: Resource) -> Result<(), ContractError> {
        // 释放量不能大于当前使用量
        if self.used.cpu < resource.cpu || self.used.memory < resource.memory ||  self.used.disk < resource.disk {
            return Err(ContractError::BadRequest)
        }

        self.used.cpu -= resource.cpu;
        self.used.memory -= resource.memory;
        self.used.disk -= resource.disk;

        Ok(())
    }

    pub fn set_total(&mut self, resource: Resource) -> Result<(), ContractError> {
        // 新总量不能小于当使用量
        if self.used.cpu > resource.cpu || self.used.memory > resource.memory ||  self.used.disk > resource.disk {
            return Err(ContractError::BadRequest)
        }

        self.total = resource;

        Ok(())
    }
}
