use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ResourceInfo {
    pub cpu: u16,
    pub memory: u16,
    pub disk: Vec<u32>,
    pub bandwidth: u16,
}



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
impl ResourceInfo {
    pub fn new(cpu: u16, memory: u16, disk: Vec<u32>, bandwidth: u16) -> ResourceInfo {
        ResourceInfo{
            cpu,
            memory,
            disk,
            bandwidth,
        }
    }

    pub fn set(&mut self, cpu: u16, memory: u16, disk: Vec<u32>, bandwidth: u16) {
        self.cpu = cpu;
        self.memory = memory;
        self.disk = disk;
        self.bandwidth = bandwidth;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalResourceInfo {
    pub cpu: u128,
    pub memory: u128,
    pub disk: u128,
    pub bandwidth: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ResourceState {
    //pub used: TotalResourceInfo,   // 资源使用总量
    pub total: TotalResourceInfo,  // 资源总量
}

pub const RESOURCE_LIMIT: Item<ResourceLimit> = Item::new("resource_limit");

pub const RESOURCE_STATE: Item<ResourceState> = Item::new("resource_state");

impl ResourceState {
    pub fn add(&mut self, resource: ResourceInfo) {
        self.total.cpu += resource.cpu;
        self.total.bandwidth += resource.bandwidth;
        self.total.disk += resource.disk;
        self.total.memory += resource.memory;
    }

    pub fn sub(&mut self, resource: ResourceInfo) {
        self.total.cpu -= resource.cpu;
        self.total.bandwidth -= resource.bandwidth;
        self.total.disk -= resource.disk;
        self.total.memory -= resource.memory;
    }
}
