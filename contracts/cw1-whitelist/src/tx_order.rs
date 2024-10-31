use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128, to_json_binary, wasm_execute,Empty};

use crate::common::{send, transfer, money_action, MoneyAction};
use crate::consts::{ORDER_MAP, ORDER_MIN_DURATION, RESOURCE, ORDER_MAX_DURATION, CW20};
use crate::ContractError;
use crate::type_order::{Order,OrderStatus};
use crate::state::ADMIN_LIST;
use crate::receive::ReceiveMsg;
use crate::msg::ExecuteMsg;
use crate::type_resource::{TotalResource, Resource};

// 创建订单
pub fn execute_create_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    order_id: String,
    resource: Resource,
    duration: u64,
) -> Result<Response, ContractError> {
    // 资源最少使用权限
    if duration < ORDER_MIN_DURATION || duration > ORDER_MAX_DURATION {
        return Err(ContractError::BadRequest);
        //return Err(StdError::generic_err("Resource is not available"));
    }

    if !resource.check() {
        return Err(ContractError::BadRequest);
    }

    //确认 Order id 唯一
    if ORDER_MAP.may_load(deps.storage, order_id.clone())?.is_some() {
        return Err(ContractError::AlreadyExists {});
    }

    // 计算总费用
    let total_cost = resource.calc_price(duration)?;

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
    ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

    // 获取合约地址
    let cw20 = CW20.load(deps.storage)?;

    // 构建转账至合约的消息
    let msg = to_json_binary(&ReceiveMsg::CreateOrder { order_id})?;

    let wasm_msg = send(
        cw20,
        env.contract.address.to_string(),
        Uint128::from(total_cost),
        msg,
    )?;

    // 返回响应，确认订单创建成功
    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "create_order")
        .add_attribute("order_id", order.id)
        .add_attribute("locked_funds", Uint128::from(total_cost))
    )
}

// 结束订单
pub fn execute_release_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    order_id: String,
) -> Result<Response, ContractError> {
    // 加载管理员列表
    let admin_list = ADMIN_LIST.load(deps.storage)?;

    // 加载订单
    let mut order = ORDER_MAP.load(deps.storage, order_id.clone())?;

    // 仅管理员和使用者可以结束订单
    if !admin_list.is_admin(info.sender.as_str()) && !order.is_initiator(info.sender){
        return Err(ContractError::Unauthorized {});
    }

    // 得到总金额
    let locked_funds = order.locked_funds;

    // 计算实际消费并修改订单状态
    let price = order.release(env.block.height)?;

    // 退还余额
    let overage = locked_funds - price;

    // 保存订单状态
    ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

    // 减少总锁定金额数量
    money_action(deps.storage, MoneyAction::DelLocked, Uint128::from(locked_funds))?;

    // 增加资源提供者可提币数量
    money_action(deps.storage, MoneyAction::AddEarnings, Uint128::from(price))?;

    // 减少对应的使用量
    RESOURCE.update(deps.storage, |mut total_resource| {
        total_resource.sub_used(order.resource)?;

        Ok::<TotalResource, ContractError>(total_resource)
    })?;

    // 获取合约地址
    let cw20 = CW20.load(deps.storage)?;

    // 构建退还余额消息
    let wasm_msg = transfer(
        cw20,
        order.initiator.to_string(),
        Uint128::from(overage))?;

    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "release_order")
        .add_attribute("order_id", order_id)
        .add_attribute("cost", Uint128::from(price))
        .add_attribute("overage", Uint128::from(overage))
    )
}

// 管理员提币到指定地址
pub fn execute_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    beneficiary: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // 仅管理员可提币
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    // 减少资源提供者可提币数量
    money_action(deps.storage, MoneyAction::DelEarnings, amount)?;

    // 获取合约地址
    let cw20 = CW20.load(deps.storage)?;

    // 构建退还余额消息
    let wasm_msg = transfer(
        cw20,
        beneficiary,
        Uint128::from(amount))?;

    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "withdraw")
        .add_attribute("amount", amount)
    )
}

// 订单续期
pub fn execute_extend(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    order_id: String,
    duration: u64,
) -> Result<Response, ContractError> {
    // 加载订单
    let order = ORDER_MAP.load(deps.storage, order_id.clone())?;

    // 新的总金额
    let price = order.resource.calc_price(duration)?;

    // 需补充的
    let shortage = price - order.locked_funds;

    // 获取合约地址
    let cw20 = CW20.load(deps.storage)?;

    // 构建转账至合约的消息
    // 合约收到对应金额后修改订单状态
    let msg = to_json_binary(&ReceiveMsg::ExtendOrder { order_id: order_id.clone(), locked_funds: price, duration})?;

    let wasm_msg = send(
        cw20,
        env.contract.address.to_string(),
        Uint128::from(shortage),
        msg,
    )?;

    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "extend")
        .add_attribute("order_id", order_id)
        .add_attribute("shortage", Uint128::from(shortage))
    )
}


// 订单升级
pub fn execute_update(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    order_id: String,
    new_order_id: String,
    resource: Resource,
) -> Result<Response, ContractError> {
    // 加载订单
    let order = ORDER_MAP.load(deps.storage, order_id.clone())?;

    // 仅使用者可以升级订单
    if !order.is_initiator(info.sender){
        return Err(ContractError::Unauthorized {});
    }

    // 仅在活跃状态下的订单可升级
    if order.status != OrderStatus::Active || order.start_height + order.duration > env.block.height{
        return Err(ContractError::BadRequest {});
    }

    let mut msgs = Vec::new();

    // 释放旧订单消息
    let release_msg = wasm_execute(
        env.contract.address.clone(),
        &ExecuteMsg::<Empty>::ReleaseOrder {order_id: order_id.clone()},
        Vec::new(),
    )?;

    // 创建新订单消息
    let create_msg = wasm_execute(
        env.contract.address,
        &ExecuteMsg::<Empty>::CreateOrder {order_id:new_order_id.clone(), resource, duration: order.duration},
        Vec::new(),
    )?;

    msgs.push(release_msg);
    msgs.push(create_msg);

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("action", "update")
        .add_attribute("old_order_id", order_id)
        .add_attribute("new_order_id", new_order_id)
    )
}

// 手动结束订单并将代币返回
pub fn execute_handle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    order_id: String
) -> Result<Response, ContractError>  {
    // 仅管理员可操作
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    // 加载订单
    let mut order = ORDER_MAP.load(deps.storage, order_id.clone())?;

    let locked_funds = Uint128::from(order.locked_funds);

    let recipient = order.initiator.clone().to_string();

    // 获取合约地址
    let cw20 = CW20.load(deps.storage)?;

    // 构建退还锁定代币消息
    let wasm_msg = transfer(
        cw20,
        recipient,
        locked_funds,
    )?;

    // 结束订单
    order.status = OrderStatus::Expired;

    ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

    // 减少资源提供者可提币数量
    money_action(deps.storage, MoneyAction::DelLocked, locked_funds)?;

    Ok(Response::new()
        .add_message(wasm_msg)
        .add_attribute("action", "handle_order")
        .add_attribute("order_id", order_id)
        .add_attribute("locked_funds", locked_funds)
    )
}