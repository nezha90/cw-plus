use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ContractError;
use crate::consts::{CPU_UNIT_PRICE, MEM_UNIT_PRICE, DISK_UNIT_PRICE};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub enum OrderStatus {
    #[default]
    Created,      // 创建订单

    Active,       // 订单活跃

    Expired,      // 订单到期
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub enum HandleAction {
    MarkAsNormal,
    // 将异常订单标记为正常
    MarkAsException,  // 将正常订单标记为异常

    #[default]
    Terminate,        // 终止异常订单
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct Order {
    pub id: String,
    // 订单ID
    pub initiator: Addr,
    // 订单发起者
    pub start_height: u64,
    // 订单开始区块高度
    pub duration: u64,
    // 订单时常
    pub locked_funds: u128,
    // 锁定的资金
    pub status: OrderStatus,
    // 订单状态
    pub resource: Resource,      // 资源详情
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct Resource {
    pub cpu: u128,
    // CPU 核数
    pub memory: u128,
    // 内存大小 单位G
    pub disk: u128,
    // 硬盘大小 单位G
}

impl Order {
    pub fn new(
        id: String,
        start_height: u64,
        duration: u64,
        locked_funds: u128,
        initiator: Addr,
        resource: Resource,
    ) -> Self {
        Order {
            id,
            initiator,
            start_height,
            duration,
            locked_funds,
            status: OrderStatus::Created,
            resource,
        }
    }

    pub fn is_initiator(&self, sender: Addr) -> bool {
        self.initiator == sender
    }

    #[warn(dead_code)]
    fn change_resource(&mut self, resource: Resource) {
        self.resource = resource
    }

    pub fn calc_unit_price(resource: &Resource) -> u128 {
        (resource.cpu as u128) * CPU_UNIT_PRICE + (resource.memory as u128) * MEM_UNIT_PRICE + (resource.disk as u128) * DISK_UNIT_PRICE
    }

    pub fn calc_price(resource: &Resource, duration: u64) -> u128 {
        Order::calc_unit_price(resource) * u128::from(duration)
    }

    pub fn renew(&mut self, funds: u128, duration: u64) -> Result<(), ContractError> {
        let price = Order::calc_price(&self.resource, duration);

        if funds < price {
            return Err(ContractError::InsufficientFunds);
        }

        self.duration += duration;
        self.locked_funds += funds;

        return Ok(());
    }

    pub fn release(&mut self, current_height: u64) -> Result<u128, ContractError> {
        if self.status != OrderStatus::Active {
            return Err(ContractError::BadRequest);
        }

        self.status = OrderStatus::Expired;

        let unit_price = Order::calc_unit_price(&self.resource);

        let duration = if current_height < self.start_height + self.duration {
            current_height - self.start_height
        } else {
            self.duration
        };


        Ok(u128::from(duration) * unit_price)
    }

    pub fn activation(&mut self) -> Result<(), ContractError> {
        if self.status != OrderStatus::Created {
            return Err(ContractError::BadRequest);
        }

        self.status = OrderStatus::Active;

        Ok(())
    }
}