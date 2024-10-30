use schemars::JsonSchema;
use serde::{Serialize,Deserialize};

use crate::ContractError;
use crate::consts::{HOUR, CPU_UNIT_PRICE, MEM_UNIT_PRICE, DISK_UNIT_PRICE};


#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct Resource {
    pub cpu: u32,
    // CPU 核数
    pub memory: u32,
    // 内存大小 单位G
    pub disk: u32,
    // 硬盘大小 单位G
}

impl Resource {
    pub fn calc_price(&self, duration: u64) -> Result<u128, ContractError> {
        let uint_price = self.calc_cpu_price()? + self.calc_mem_price()? + self.calc_disk_price()?;

        let duration_coefficient = Resource::calc_duration_coefficient(duration)?;
        return Ok(uint_price * u128::from(duration) * duration_coefficient / 10)
    }


    fn calc_cpu_price(&self) -> Result<u128, ContractError> {
        let price = u128::from(self.cpu) * CPU_UNIT_PRICE * match self.cpu {
            1..4 => 10,
            4..8 => 9,
            9..16 => 8,
            17..32 => 7,
            _ => {return Err(ContractError::BadRequest)}
        } / 10;

        Ok(price)
    }

    fn calc_mem_price(&self) -> Result<u128, ContractError> {
        let price = u128::from(self.memory) * MEM_UNIT_PRICE * match self.memory {
            1..4 => 10,
            5..16 => 9,
            17..32 => 8,
            33..64 => 7,
            _ => {return Err(ContractError::BadRequest)}
        } / 10;

        Ok(price)
    }

    fn calc_disk_price(&self) -> Result<u128, ContractError> {
        let price = u128::from(self.disk) * DISK_UNIT_PRICE * match self.disk {
            40..100 => 10,
            101..500 => 8,
            501..2000 => 6,
            2001..4000 => 5,
            _ => {return Err(ContractError::BadRequest)}
        } / 10;

        Ok(price)
    }

    // 计算乘 10 后的时间系数,使用时需要除 10
    pub fn calc_duration_coefficient(duration: u64) -> Result<u128, ContractError> {
        if duration > HOUR && duration <= HOUR * 24 {
            return Ok(10)
        } else if duration <= HOUR * 72 {
            return Ok(9)
        } else if duration <= HOUR * 168 {
            return Ok(8)
        } else if duration <= HOUR * 720 {
            return Ok(7)
        }

        Ok(0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_price() {
        let resources: Vec<_> = vec![
            (Resource{cpu:4, memory: 4, disk: 50}, 12 * HOUR, 10800),
            (Resource{cpu:16, memory: 32, disk: 100}, 720 * HOUR, 2540160)];

        for (resource, duration, price) in resources {
            let calc_price = resource.calc_price(duration)?;
            assert_eq!(calc_price, price);
        }
    }
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct TotalResource {
    pub total: Resource,
    pub used: Resource,
}

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
