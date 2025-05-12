use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::node::level::Level;

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

pub fn check_resource(resource: &ResourceInfo, level: &Level) -> bool {
    return true
}、

