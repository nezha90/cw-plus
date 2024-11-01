use std::fmt;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Empty, Uint128};
use schemars::JsonSchema;
use cw20::Cw20ReceiveMsg;

use crate::type_resource::{TotalResource, Resource};
use crate::type_order::Order;

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub cw20_contract: String,
    pub mutable: bool,
    pub resource: Option<Resource>,
}

#[cw_serde]
pub enum ExecuteMsg<T = Empty>
    where
        T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    /// Execute requests the contract to re-dispatch all these messages with the
    /// contract's address as sender. Every implementation has it's own logic to
    /// determine in
    Execute { msgs: Vec<CosmosMsg<T>> },
    /// Freeze will make a mutable contract immutable, must be called by an admin
    Freeze {},
    /// UpdateAdmins will change the admin set of the contract, must be called by an existing admin,
    /// and only works if the contract is mutable
    UpdateAdmins { admins: Vec<String> },

    /// cw20
    Receive(Cw20ReceiveMsg),
    ///Create Order
    //CreateOrder { order_id: String, resource: Resource, duration: u64 },

    //Release Order
    // 结束订单
    ReleaseOrder { order_id: String },

    //Withdraw
    // 提取资源提供方收益
    WithDraw {beneficiary: String, amount: Uint128},

    //Handle
    // 手动终止订单并退还
    Handle {order_id: String},

    // Extend
    //Extend {order_id: String, duration: u64},
    // Update Resource
    //Update {order_id: String, new_order_id: String, resource: Resource},

    // Set total resource
    // 设置资源总量
    SetResource {cpu: u32, memory: u32, disk: u32},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg<T = Empty>
    where
        T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    /// Shows all admins and whether or not it is mutable
    #[returns(AdminListResponse)]
    AdminList {},
    /// Checks permissions of the caller on this proxy.
    /// If CanExecute returns true then a call to `Execute` with the same message,
    /// before any further state changes, should also succeed.
    #[returns(cw1::CanExecuteResponse)]
    CanExecute { sender: String, msg: CosmosMsg<T> },

    #[returns(Vec<Order>)]
    Orders {order_ids: Vec<String>},

    #[returns(TotalResource)]
    Resources {},
}

#[cw_serde]
pub struct AdminListResponse {
    pub admins: Vec<String>,
    pub mutable: bool,
}

#[cfg(any(test, feature = "test-utils"))]
impl AdminListResponse {
    /// Utility function for converting message to its canonical form, so two messages with
    /// different representation but same semantic meaning can be easily compared.
    ///
    /// It could be encapsulated in custom `PartialEq` implementation, but `PartialEq` is expected
    /// to be quickly, so it seems to be reasonable to keep it as representation-equality, and
    /// canonicalize message only when it is needed
    ///
    /// Example:
    ///
    /// ```
    /// # use cw1_whitelist::msg::AdminListResponse;
    ///
    /// let resp1 = AdminListResponse {
    ///   admins: vec!["admin1".to_owned(), "admin2".to_owned()],
    ///   mutable: true,
    /// };
    ///
    /// let resp2 = AdminListResponse {
    ///   admins: vec!["admin2".to_owned(), "admin1".to_owned(), "admin2".to_owned()],
    ///   mutable: true,
    /// };
    ///
    /// assert_eq!(resp1.canonical(), resp2.canonical());
    /// ```
    pub fn canonical(mut self) -> Self {
        self.admins.sort();
        self.admins.dedup();
        self
    }
}
