pub fn create_order_id(env: Env, index: usize) -> String {
    let mut hasher = Sha256::new();

    // 将区块高度和发送者地址作为输入进行哈希
    hasher.update(env.block.height.to_be_bytes());
    hasher.update(env.transaction.unwrap().index.to_be_bytes());
    hasher.update(index.to_be_bytes());

    let result = hasher.finalize();

    format!("{:?}", result)
}

pub fn send(cw20_contract: String, contract: String, amount: Uint128, msg: Binary) -> StdResult<WasmMsg> {
    wasm_execute(
        cw20_contract,
        &ExecuteMsg::Send { contract, amount, msg },
        Vec::new(),
    )
}

pub fn transfer(cw20_contract: String, recipient: String, amount: Uint128) -> StdResult<WasmMsg> {
    wasm_execute(
        cw20_contract,
        &ExecuteMsg::Transfer { recipient, amount },
        Vec::new(),
    )
}