use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

use crate::ContractError;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub enum Region {
    USA,
    Germany,
    Singapore,
    HongKong,
    Other(String),

    #[default]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub enum NAT {
    NoNat,
    FullCone,
    RestrictedCone,
    PortRestrictedCone,
    Symmetric,

    #[default]
    UnKnown,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct ResourceDetails {
    total: u128,
    used: u128,
    unit_price: u128,
}

impl ResourceDetails {
    pub fn new(total: u128, unit_price: u128) -> Self {
        ResourceDetails{total, used: 0, unit_price}
    }

    pub fn default() -> Self {
        ResourceDetails{
            total: 0,
            used: 0,
            unit_price: 0
        }
    }

    pub fn get_total(&self) -> u128 {
        return self.total
    }

    pub fn set_total(&mut self, total: u128) {
        self.total = total
    }

    pub fn set_price(&mut self, price: u128) {
        self.unit_price = price
    }

    pub fn get_used(&self) -> u128 {
        return self.used
    }

    pub fn add_used(&mut self, used: u128) -> Result<(), ContractError> {
        if used + self.used > self.total {
            // 超出资源界限
            return Err(ContractError::ResourceOverflow)
        }

        self.total = self.used + used;

        Ok(())
    }

    pub fn release_used(&mut self, used: u128) -> Result<(), ContractError> {
        if self.used < used {
            // 超出资源界限
            return Err(ContractError::ResourceOverflow)
        }

        self.used -= used;

        Ok(())
    }
}

// 枚举，用于表示不同的资源类型
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub enum ResourceType {
    Cpu(u128),          // 传递 CPU 的总量
    Memory(u128),       // 传递内存的总量
    Bandwidth(u128),    // 传递带宽的总量
    Nat(NAT),          // 传递 NAT 类型
    Region(Region),    // 传递 Region 类型
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ResourceAttr {
    pub cpu: ResourceDetails,
    pub memory: ResourceDetails,
    pub bandwidth: ResourceDetails,
    pub region: Region,
    pub nat: NAT,
}

impl ResourceAttr {
    pub fn new(cpu_detail: ResourceDetails,
               mem_detail: ResourceDetails,
               bandwidth_detail: ResourceDetails,
               region: Region,
               nat: NAT
    ) -> Self {
        ResourceAttr{ cpu: cpu_detail, memory: mem_detail, bandwidth: bandwidth_detail, region, nat }
    }

    pub fn reset(&mut self) {
        self.cpu.used = 0;
        self.memory.used = 0;
        self.bandwidth.used = 0;
    }

    // 更新函数，适用于所有类型的资源
    pub fn update_resource(&mut self, resource_type: ResourceType) {
        match resource_type {
            ResourceType::Cpu(total) => self.cpu.set_total(total),
            ResourceType::Memory(total) => self.memory.set_total(total),
            ResourceType::Bandwidth(total) => self.bandwidth.set_total(total),
            ResourceType::Nat(nat) => self.nat = nat,
            ResourceType::Region(region) => self.region = region,
        }
    }

    pub fn use_resource(&mut self, resource_type: ResourceType) -> Result<(), ContractError>{
        match resource_type {
            ResourceType::Cpu(used) => self.cpu.add_used(used),
            ResourceType::Memory(used) => self.memory.add_used(used),
            ResourceType::Bandwidth(used) => self.bandwidth.add_used(used),
            _ => Err(ContractError::OtherError)
        }
    }

    pub fn release_resource(&mut self, resource_type: ResourceType) -> Result<(), ContractError>{
        match resource_type {
            ResourceType::Cpu(used) => self.cpu.release_used(used),
            ResourceType::Memory(used) => self.memory.release_used(used),
            ResourceType::Bandwidth(used) => self.bandwidth.release_used(used),
            _ => Err(ContractError::OtherError)
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub enum Status {
    Unused,                  // 未使用
    //UsedRemaining,           // 已使用但还有空余
    Suspended,               // 暂停接单

    #[default]
    Exception,               // 异常, 不能提供服务
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct Resource {
    id: String,                         // 机器ID
    owner: Addr,                        // 机器所有者
    base_price: u128,                   // 机器基础价格
    resources_attr: ResourceAttr,       // 资源属性
    status: Status,             // 机器状态
}

impl Resource {
    // Constructor that checks if all required resources are present
    pub fn new(id: String, owner: Addr, base_price: u128, resources_attr: ResourceAttr, status: Status) -> Self {
        Self { id, owner, base_price, resources_attr, status}
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_resource_price(&self) -> u128 {
        let cpu_price = self.resources_attr.cpu.unit_price * self.resources_attr.cpu.total;
        let memory_price = self.resources_attr.memory.unit_price * self.resources_attr.memory.total;
        let bandwidth_price =self.resources_attr.bandwidth.unit_price *  self.resources_attr.bandwidth.total;

        self.base_price + cpu_price + memory_price + bandwidth_price
    }

    pub fn get_resource_attr(&self) -> ResourceAttr {
        self.resources_attr.clone()
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_base_price(&mut self, price: u128) -> Result<(), ContractError> {
        if self.status != Status::Unused {
            return Err(ContractError::OtherError)
        }

        self.base_price = price;

        Ok(())
    }

    pub fn set_price(&mut self, resource_type: ResourceType, price: u128) -> Result<(), ContractError> {
        if self.status != Status::Unused {
            return Err(ContractError::OtherError)
        }

        match resource_type {
            ResourceType::Cpu(_) => {self.resources_attr.cpu.set_price(price); Ok(())},
            ResourceType::Memory(_) => {self.resources_attr.memory.set_price(price); Ok(())},
            ResourceType::Bandwidth(_) => {self.resources_attr.bandwidth.set_price(price); Ok(())},
            _ => Err(ContractError::OtherError)
        }
    }

    pub fn use_resource(&mut self) -> Result<(), ContractError>{
        if self.status != Status::Unused {
            return Err(ContractError::OtherError)
        }

        self.resources_attr.cpu.add_used(self.resources_attr.cpu.get_total())?;
        self.resources_attr.memory.add_used(self.resources_attr.memory.get_total())?;
        self.resources_attr.bandwidth.add_used(self.resources_attr.bandwidth.get_total())?;

        self.status = Status::Suspended;

        Ok(())
    }

    pub fn update_resource(&mut self, resources_type: ResourceType) -> Result<(), ContractError> {
        if self.status != Status::Unused {
            return Err(ContractError::OtherError)
        }

        self.resources_attr.update_resource(resources_type);

        Ok(())
    }

    pub fn release_resource(&mut self) -> Result<(), ContractError>{
        if self.get_status() != Status::Suspended {
            return Err(ContractError::OtherError)
        }

        self.resources_attr.release_resource(ResourceType::Cpu(self.resources_attr.cpu.get_used()))?;
        self.resources_attr.release_resource(ResourceType::Memory(self.resources_attr.memory.get_used()))?;
        self.resources_attr.release_resource(ResourceType::Bandwidth(self.resources_attr.bandwidth.get_used()))?;

        self.set_status(Status::Unused);

        Ok(())
    }
}

