use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Coin, BankMsg};
use cw_storage_plus::Map;

use crate::ContractError;
use crate::state::{ADMIN_LIST};
use crate::resource::{RESOURCE_MAP, update_status_by_RESOURCE_MAP};
use crate::resource_type::{Status};
use crate::order_type::{Order, OrderStatus, HandleAction, DEFAULT_DENOM};

pub const ORDER_MAP: Map<String, Order> = Map::new("orders");

pub fn create_order(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    resource_id: String,
    duration: u64,
) -> Result<Response,ContractError> {
    // 加载资源
    let resource = RESOURCE_MAP.load(deps.storage, resource_id.clone())?;

    // 确保资源未被使用
    if resource.check_status(Status::Unused) {
        return Err(ContractError::OtherError);
        //return Err(StdError::generic_err("Resource is not available"));
    }

    // 计算总费用
    let total_cost = resource.get_resource_price() * duration as u128;

    // 检查用户是否发送了足够的资金
    let sent_funds = info.funds.iter().find(|coin| coin.denom == DEFAULT_DENOM);
    if let Some(Coin { amount, .. }) = sent_funds {
        if *amount < total_cost.into() {
            return Err(ContractError::OtherError);
            //return Err(StdError::generic_err("Insufficient funds sent"));
        }
    } else {
        return Err(ContractError::OtherError);
        //return Err(StdError::generic_err("No funds sent"));
    }

    // 创建订单
    let order = Order {
        id: env.block.height.to_string(),
        resource_id: resource_id.clone(),
        initiator: info.sender.clone(),
        start_height: env.block.height,
        end_height: env.block.height + duration,
        locked_funds: total_cost,
        status: OrderStatus::Active,
    };

    // 保存订单
    ORDER_MAP.save(deps.storage, order.id.clone(), &order)?;

    //更新资源状态
    update_status_by_RESOURCE_MAP(deps, resource.get_id(), Status::Used)?;

    // 返回响应，确认订单创建成功
    Ok(Response::new()
        .add_attribute("action", "create_order")
        .add_attribute("order_id", order.id)
        .add_attribute("locked_funds", total_cost.to_string())
    )
}

pub fn end_order(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    order_id: String,
) -> Result<Response, ContractError> {
    // 加载订单
    let mut order = ORDER_MAP.load(deps.storage, order_id.clone())?;

    // 检查订单是否已到期
    if env.block.height < order.end_height {
        return  Err(ContractError::OtherError);
        //return Err(StdError::generic_err("Order has not yet expired"));
    }

    // 获取资源并支付给资源提供者
    let resource = RESOURCE_MAP.load(deps.storage, order.resource_id.clone())?;
    let payment_msg = BankMsg::Send {
        to_address: resource.get_owner().to_string(),
        amount: vec![Coin {
            denom: DEFAULT_DENOM.to_string(),
            amount: order.locked_funds.into(),
        }],
    };

    // 更新订单状态为到期
    order.status = OrderStatus::Expired;

    ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

    // 更新资源状态为未使用
    update_status_by_RESOURCE_MAP(deps, resource.get_id(), Status::Unused)?;

    // 返回响应，并发送资金
    Ok(Response::new()
        .add_message(payment_msg)
        .add_attribute("action", "end_order")
        .add_attribute("order_id", order_id)
    )
}

pub fn handle_exception(
    deps: DepsMut,
    _evn: Env,
    info: MessageInfo,
    order_id: String,
    action: HandleAction,
) -> Result<Response,ContractError> {
    // 只允许管理员操作
    let admin_list = ADMIN_LIST.load(deps.storage)?;
    if !admin_list.is_admin(info.sender.as_str()) {
        return Err(ContractError::Unauthorized {});
    }

    // 加载订单
    let mut order = ORDER_MAP.load(deps.storage, order_id.clone())?;

    match action {
        HandleAction::MarkAsNormal => {
            // 如果订单当前为异常状态，恢复为正常状态
            if order.status == OrderStatus::Exception {
                order.status = OrderStatus::Active;
                ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

                Ok(Response::new()
                    .add_attribute("action", "mark_as_normal")
                    .add_attribute("order_id", order_id))
            } else {
                //Err(StdError::generic_err("Order is not in exception state"))
                Err(ContractError::OtherError)
            }
        },
        HandleAction::MarkAsException => {
            // 将正常订单标记为异常状态
            if order.status == OrderStatus::Active {
                order.status = OrderStatus::Exception;
                ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

                Ok(Response::new()
                    .add_attribute("action", "mark_as_exception")
                    .add_attribute("order_id", order_id))
            } else {
                // Err(StdError::generic_err("Order is not active"))
                Err(ContractError::OtherError)
            }
        },
        HandleAction::Terminate => {
            // 终止异常订单
            if order.status == OrderStatus::Exception {
                // 更新订单状态为终止
                order.status = OrderStatus::Terminated;
                ORDER_MAP.save(deps.storage, order_id.clone(), &order)?;

                // 处理资源状态，设置为未使用
                update_status_by_RESOURCE_MAP(deps,order.resource_id.clone(),  Status::Unused)?;

                // 处理资金，退还给订单发起者
                let refund_msg = BankMsg::Send {
                    to_address: order.initiator.to_string(),
                    amount: vec![Coin {
                        denom: DEFAULT_DENOM.to_string(),
                        amount: order.locked_funds.into(),
                    }],
                };

                Ok(Response::new()
                    .add_message(refund_msg)
                    .add_attribute("action", "terminate_order")
                    .add_attribute("order_id", order_id)
                    .add_attribute("refund", "true"))
            } else {
                // Err(StdError::generic_err("Order is not in exception state"))
                Err(ContractError::OtherError)
            }
        },
    }
}