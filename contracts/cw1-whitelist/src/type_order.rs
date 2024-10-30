use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ContractError;
use crate::consts::{CPU_UNIT_PRICE, MEM_UNIT_PRICE, DISK_UNIT_PRICE, HOUR};
use crate::type_resource::Resource;

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


    pub fn renew(&mut self, funds: u128, duration: u64) -> Result<(), ContractError> {
        let price = self.resource.calc_price(duration)?;

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

        if self.start_height == current_height {
            return Err(ContractError::BadRequest);
        }

        self.status = OrderStatus::Expired;

        // 按实际使用时间扣费
        let mut duration = if current_height < self.start_height + self.duration {
            current_height - self.start_height
        } else {
            self.duration
        };

        // 不足一小时则按一小时计算
        if duration % HOUR != 0 {
            duration = ((duration / HOUR) + 1) * HOUR;
        }

        let price = self.resource.calc_price(duration)?;

        Ok(price)
    }

    pub fn activation(&mut self) -> Result<(), ContractError> {
        if self.status != OrderStatus::Created {
            return Err(ContractError::BadRequest);
        }

        self.status = OrderStatus::Active;

        Ok(())
    }
}