use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::common::{send, transfer};
use crate::consts::{ORDER_MAP, ORDER_MIN_DURATION};
use crate::ContractError;
use crate::type_order::{Order, Resource};

pub fn execute_create_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    order_id: String,
    resource: Resource,
    duration: u64,
) -> Result<Response, ContractError> {
    //确认 Order id 唯一
    if ORDER_MAP.may_load(deps.storage, order_id.clone())?.is_some() {
        return Err(ContractError::AlreadyExists {});
    }

    // 资源最少使用权限
    if duration < ORDER_MIN_DURATION {
        return Err(ContractError::BadRequest);
        //return Err(StdError::generic_err("Resource is not available"));
    }

    // 计算总费用
    let total_cost = Order::calc_price(resource.clone(), duration);

    // 订单所有者
    let initiator = info.sender.clone();

    let height = env.block.height;

    // 创建订单
    // 默认是创建状态,转账成功后变为活跃状态
    let order = Order::new(
        order_id.clone(),
        height,
        duration,
        total_cost,
        initiator,
        resource,
    );

    // 保存订单
    ORDER_MAP.save(deps.storage, order_id, &order)?;

    // 构建转账消息
    let wasm_msg = send(
        "".to_string(),
        env.contract.address.to_string(),
        Uint128(total_cost),
        Binary::new(Vec::new()),
    )?;

    // 返回响应，确认订单创建成功
    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "create_order")
        .add_attribute("order_id", order.id)
        .add_attribute("locked_funds", total_cost.to_string())
    )
}

pub fn execute_release_order(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    order_id: String,
) -> Result<Response, ContractError> {
    // 加载订单
    let mut order = ORDER_MAP.load(deps.storage, order_id.clone())?;

    // 计算实际消费并修改订单状态
    let price = order.release(env.block.height)?;

    // 退还余额
    let overage = order.locked_funds - price;

    // 保存订单状态
    ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

    // 构建退还余额消息
    let wasm_msg = transfer(
        env.contract.address.to_string(),
        order.initiator.to_string(),
        Uint128::from(overage));

    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "end_order")
        .add_attribute("order_id", order_id)
        .add_attribute("cost", price)
        .add_attribute("overage", overage)
    )
}