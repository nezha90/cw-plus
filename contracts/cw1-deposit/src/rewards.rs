use cosmwasm_std::{Uint128, StdError, Storage};

use crate::types::{StakingInfo};
use crate::consts::{INTEREST_RATE, PRECISION, YEAR, END_TIME, CHANGE_TIME, OLD_INTEREST_RATE};

pub fn update_rewards(
    storage: &mut dyn Storage,
    staking_info: &mut StakingInfo,
    current_time: u64,
) -> Result<(), StdError> {
    let end_time = END_TIME.load(storage)?;

    let current_time = if end_time < current_time {
        end_time
    } else {
        current_time
    };

    // 新订单不需要计算收益
    if current_time <= staking_info.last_update_time {
        return Ok(())
    }

    let (new_reward, new_remainder) = if current_time <= CHANGE_TIME {
        // 计算从上次更新到现在的收益
        let elapsed_time = current_time - staking_info.last_update_time;

        calculate_reward_with_remainder(
            OLD_INTEREST_RATE,
            staking_info.principal.u128(),
            elapsed_time as u128,
            staking_info.remainder,
        )
    } else if staking_info.last_update_time < CHANGE_TIME {
        let elapsed_time = CHANGE_TIME - staking_info.last_update_time;

        let (new_reward_temp_1, new_remainder_temp_1) = calculate_reward_with_remainder(
            OLD_INTEREST_RATE,
            staking_info.principal.u128(),
            elapsed_time as u128,
            staking_info.remainder);


        let elapsed_time = current_time - CHANGE_TIME;

        let (new_reward_temp_2, new_remainder_temp_2) = calculate_reward_with_remainder(
            INTEREST_RATE,
            staking_info.principal.u128(),
            elapsed_time as u128,
            new_remainder_temp_1);

        (new_reward_temp_1 + new_reward_temp_2, new_remainder_temp_2)
    } else {

        let elapsed_time = current_time - staking_info.last_update_time;

       calculate_reward_with_remainder(
            INTEREST_RATE,
            staking_info.principal.u128(),
            elapsed_time as u128,
            staking_info.remainder)
    };


    // 更新用户的待领取收益
    staking_info.pending_reward += Uint128::from(new_reward);
    staking_info.last_update_time = current_time;
    staking_info.remainder = new_remainder;

    Ok(())
}

// 计算收益
pub fn calculate_reward_with_remainder(
    interest_rate: u128,
    principal: u128,
    seconds: u128,
    remainder: u128, // 上一次的余数
) -> (u128, u128) {
    // 计算分子：(本金 * 利率 * 秒) + 余数
    let numerator = principal * interest_rate * seconds + remainder;

    // 分母：年时间单位 * 精度因子
    let denominator = YEAR as u128 * PRECISION;

    // 计算收益
    let reward = numerator.clone() / denominator.clone();

    // 计算新的余数
    let new_remainder = numerator % denominator;

    (reward, new_remainder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::{Addr, Uint128};
    use crate::types::StakingInfo;
    use crate::consts::{END_HEIGHT, INTEREST_RATE, PRECISION, YEAR};

    // 测试辅助函数：初始化 StakingInfo
    fn setup_staking_info(principal: u128, last_update_time: u64, remainder: u128) -> StakingInfo {
        StakingInfo {
            user: Addr::unchecked("user"),
            principal: Uint128::from(principal),
            pending_reward: Uint128::zero(),
            reward: Uint128::zero(),
            last_update_time,
            start_time: 0,
            remainder,
            unstake_requests: vec![],
        }
    }

    // 测试辅助函数：初始化 MockStorage 并设置 END_HEIGHT
    fn setup_storage(end_height: u64) -> MockStorage {
        let mut storage = MockStorage::new();
        END_HEIGHT.save(&mut storage, &end_height).unwrap();
        storage
    }

    #[test]
    fn test_update_rewards_before_end_height() {
        // 初始化 Storage 和 StakingInfo
        let mut storage = setup_storage(1000); // END_HEIGHT = 1000
        let mut staking_info = setup_staking_info(1_000_000, 500, 0); // 本金 1_000_000，上次更新时间 500

        // 调用 update_rewards
        let current_time = 600; // 当前时间 600
        update_rewards(&mut storage, &mut staking_info, current_time).unwrap();

        // 验证收益和更新时间
        assert_eq!(staking_info.pending_reward, Uint128::new(3)); // 预期收益
        assert_eq!(staking_info.last_update_time, 600); // 预期更新时间
        assert_eq!(staking_info.remainder, 539200000); // 预期余数
    }

    #[test]
    fn test_update_rewards_after_end_height() {
        // 初始化 Storage 和 StakingInfo
        let mut storage = setup_storage(1000); // END_HEIGHT = 1000
        let mut staking_info = setup_staking_info(1_000_000, 500, 0); // 本金 1_000_000，上次更新时间 500

        // 调用 update_rewards
        let current_time = 1100; // 当前时间 1100（超过 END_HEIGHT）
        update_rewards(&mut storage, &mut staking_info, current_time).unwrap();

        // 验证收益和更新时间
        assert_eq!(staking_info.pending_reward, Uint128::new(15)); // 预期收益
        assert_eq!(staking_info.last_update_time, 1000); // 预期更新时间（不超过 END_HEIGHT）
        assert_eq!(staking_info.remainder, 2696000000); // 预期余数
    }

    #[test]
    fn test_update_rewards_no_reward_calculation() {
        // 初始化 Storage 和 StakingInfo
        let mut storage = setup_storage(1000); // END_HEIGHT = 1000
        let mut staking_info = setup_staking_info(1_000_000, 500, 0); // 本金 1_000_000，上次更新时间 500

        // 调用 update_rewards
        let current_time = 400; // 当前时间 400（小于上次更新时间）
        update_rewards(&mut storage, &mut staking_info, current_time).unwrap();

        // 验证收益和更新时间未变化
        assert_eq!(staking_info.pending_reward, Uint128::zero()); // 预期收益为 0
        assert_eq!(staking_info.last_update_time, 500); // 预期更新时间未变化
        assert_eq!(staking_info.remainder, 0); // 预期余数为 0
    }

    #[test]
    fn test_calculate_reward_with_remainder() {
        // 测试 calculate_reward_with_remainder 函数
        let principal = 1_000_000; // 本金
        let seconds = 3600; // 时间差
        let remainder = 539200000; // 余数

        // 调用 calculate_reward_with_remainder
        let (reward, new_remainder) = calculate_reward_with_remainder(principal, seconds, remainder);

        // 验证收益和余数
        assert_eq!(reward, 1_000_000); // 预期收益
        assert_eq!(new_remainder, 0); // 预期余数
    }
}