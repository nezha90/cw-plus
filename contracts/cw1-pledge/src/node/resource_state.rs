// use cosmwasm_std::{Addr, Uint128};
// use cw_storage_plus::{Item, Map};
//
// use serde::{Deserialize, Serialize};
// use schemars::JsonSchema;
//
// use crate::node::resource_info::ResourceInfo;
//
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct TotalResourceInfo {
//     pub cpu: u128,
//     pub memory: u128,
//     pub disk: u128,
//     pub bandwidth: u128,
// }
//
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct ResourceState {
//     //pub used: TotalResourceInfo,   // 资源使用总量
//     pub total: TotalResourceInfo,  // 资源总量
// }
//
// impl ResourceState {
//     pub fn add(&mut self, resource: ResourceInfo) {
//         self.total.cpu += resource.cpu;
//         self.total.bandwidth += resource.bandwidth;
//         self.total.disk += resource.disk.iter().sum();
//         self.total.memory += resource.memory;
//     }
//
//     pub fn sub(&mut self, resource: ResourceInfo) {
//         self.total.cpu -= resource.cpu;
//         self.total.bandwidth -= resource.bandwidth;
//         self.total.disk -= resource.disk.iter().sum();
//         self.total.memory -= resource.memory;
//     }
// }
