use schemars::JsonSchema;

use std::fmt;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{CosmosMsg, Empty};

use crate::resource_type::{ResourceType,Resource};
use crate::order_type::{HandleAction};

#[cw_serde]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
    pub mutable: bool,
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

    ///B
    //UseResources { ids: Vec<String>},

    ///B
    UpdateResources { ids: Vec<String>, resource_types: Vec<ResourceType>},

    ///B
    AddResources { resources: Vec<Resource>},

    ///B
    //ReleaseResources { ids: Vec<String>},

    ///B
    DeleteResources { ids: Vec<String>},

    /// B
    CreateOrder{resource_id: String, duration: u64},

    /// B
    EndOrder{order_id: String},

    /// B
    HandleException{order_id: String, action: HandleAction},
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

    #[returns(Vec<Resource>)]
    QueryResources { ids: Vec<String>},
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
