use std::borrow::BorrowMut;

use cw_storage_plus::Map;
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, BankMsg};

use crate::tx_resource::{RESOURCE_MAP, update_status_by_resource_map};
use crate::type_resource::Status;
use crate::ContractError;
use crate::type_order::{DEFAULT_DENOM, Order, OrderStatus};
use crate::common::{check_deposit, create_order_id, create_batch_order_id};
use crate::tx_order::ORDER_MAP;
use crate::type_batch_order::{BatchOrder};

pub const BATCH_ORDER_MAP: Map<String, BatchOrder> = Map::new("batch_order");


pub fn create_batch_orders(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    resource_ids: Vec<String>,
    duration: u64,
) -> Result<Response,ContractError> {
    let mut total_cost = 0;
    let mut order_ids = Vec::<String>::new();

    resource_ids.iter().enumerate().try_for_each(|(index, resource_id)| {
        // 加载资源
        let resource = RESOURCE_MAP.load(deps.storage, resource_id.clone())?;

        // 确保资源未被使用
        if resource.check_status(Status::Unused) {
            return Err(ContractError::OtherError);
            //return Err(StdError::generic_err("Resource is not available"));
        }

        // 计算单个订单费用
        let cost = resource.get_resource_price() * duration as u128;

        total_cost += cost;

        let order_id = create_order_id(env.clone(), index);
        // 创建订单
        let mut order = Order::new(
            env.block.height,
            env.block.height + duration,
            cost,
            info.sender.clone(),
            resource_id.clone(),
            order_id.clone(),
        );

        order_ids.push(order_id);

        // 保存订单
        ORDER_MAP.save(deps.storage.borrow_mut(), order.id.clone(), &order)?;

        //更新资源状态
        update_status_by_resource_map(deps, resource.get_id(), Status::Used)?;

        Ok(())
    })?;

    // 检查用户是否发送了足够的资金
    check_deposit(info, total_cost)?;

    let batch_id = create_batch_order_id(env.clone());
    let batch_order = BatchOrder::new(
        batch_id.clone(),
        order_ids,
        info.sender.clone(),
        total_cost,
        env.block.height,
        env.block.height+duration,
    );

    BATCH_ORDER_MAP.save(deps.storage.borrow_mut(), batch_id.clone(), &batch_order)?;

    // 返回响应，确认订单创建成功
    Ok(Response::new()
        .add_attribute("action", "create_batch_orders")
        .add_attribute("batch_id", batch_id)
        .add_attribute("locked_funds", total_cost.to_string())
    )
}


pub fn end_orders_by_batch_id(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    batch_id: String,
) -> Result<Response,ContractError> {
    let mut batch_order = BATCH_ORDER_MAP.load(deps.storage, batch_id.clone())?;

    // 检查订单是否已到期
    if env.block.height < batch_order.end_height {
        return  Err(ContractError::OtherError);
        //return Err(StdError::generic_err("Order has not yet expired"));
    }
    let mut payment_msgs = Vec::new();

    batch_order.order_ids.iter().enumerate().try_for_each(|(index, order_id)|{
        // 加载订单
        let mut order = ORDER_MAP.load(deps.storage, order_id.clone())?;

        return if order.status == OrderStatus::Active {
            // 更新订单状态为到期
            order.status = OrderStatus::Expired;

            ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

            let resource = RESOURCE_MAP.load(deps.storage, order.resource_id.clone())?;
            // 更新资源状态为未使用
            update_status_by_resource_map(deps, resource.get_id(), Status::Unused)?;

            payment_msgs.push(BankMsg::Send {
                to_address: resource.get_owner().to_string(),
                amount: vec![Coin {
                    denom: DEFAULT_DENOM.to_string(),
                    amount: order.locked_funds.into(),
                }],
            });

            Ok(())
        } else if order.status == OrderStatus::Terminated {
            Ok(())
        } else {
            Err(ContractError::OtherError)
        }
    })?;

    // 返回响应，确认订单结束成功
    Ok(Response::new()
        .add_messages(payment_msgs)
        .add_attribute("action", "end_batch_orders")
        .add_attribute("batch_id", batch_id)
    )
}