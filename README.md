![marmot logo](https://raw.githubusercontent.com/vitabread/marmot-network-file/main/marmot_logo_m.png)
# Marmot Network（土拨鼠网络）
_Marmot Network是一款基于Solana网络的Meme GameFi游戏。玩家需要猜测土拨鼠在洞外，还是洞内？猜对获得1枚新铸造的$MARMOT，猜错烧毁1枚已获得的$MARMOT。_

![marmot shoutout](https://raw.githubusercontent.com/vitabread/marmot-network-file/main/marmot_home.gif)

## 技术堆栈

- 前端：Next.js
- 后端：Anchor Framework
- RPC呼叫：IDL介面
- 网络：devnet

## 功能介绍
- 首页：玩家可以通过Wallet Adapter连接Phantom钱包
![homepage](https://raw.githubusercontent.com/vitabread/marmot-network-file/main/marmot_sc1.png)

- 代币创建Rust代码：通过CPI向Token Program请求创建代币$MARMOT (PDA 地址：697VNng2qcwd4dBrninqF6eyVS3KU9BNnK9miYGKzx5)
```
pub fn create_mint(
       ctx: Context<CreateMint>,
       uri: String,
       name: String,
       symbol: String,
   ) -> Result<()> {
       // PDA seeds and bump to "sign" for CPI
       let seeds = b"reward";
       let bump = *ctx.bumps.get("reward_token_mint").unwrap();
       let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

       // On-chain token metadata for the mint
       let data_v2 = DataV2 {
           name: name,
           symbol: symbol,
           uri: uri,
           seller_fee_basis_points: 0,
           creators: None,
           collection: None,
           uses: None,
       };

       // CPI Context
       let cpi_ctx = CpiContext::new_with_signer(
           ctx.accounts.token_metadata_program.to_account_info(),
           CreateMetadataAccountsV3 {
                // the metadata account being created
               metadata: ctx.accounts.metadata_account.to_account_info(),
                // the mint account of the metadata account
               mint: ctx.accounts.reward_token_mint.to_account_info(),
               // the mint authority of the mint account
               mint_authority: ctx.accounts.reward_token_mint.to_account_info(),
               // the update authority of the metadata account
               update_authority: ctx.accounts.reward_token_mint.to_account_info(),
               // the payer for creating the metadata account
               payer: ctx.accounts.admin.to_account_info(),
               // the system program account
               system_program: ctx.accounts.system_program.to_account_info(),
               // the rent sysvar account
               rent: ctx.accounts.rent.to_account_info(),
           },
           signer,
       );

       create_metadata_accounts_v3(
           cpi_ctx, // cpi context
           data_v2, // token metadata
           true,    // is_mutable
           true,    // update_authority_is_signer
           None,    // collection details
       )?;

       Ok(())
   }
```

- 玩家帐户初始化Rust代码，玩家帐户记录总共的获胜次数和失败次数
```
   // Create new player account
   pub fn init_player(ctx: Context<InitPlayer>) -> Result<()> {
       ctx.accounts.player_data.win = 0;
       ctx.accounts.player_data.lose = 0;
       Ok(())
   }
```
- 获取空投页面：$MARMOT代币空投，玩家可以在游戏前领取代币以参与游戏
![airdrop page](https://raw.githubusercontent.com/vitabread/marmot-network-file/main/marmot_sc2.png)
- 铸造1枚$MARMOT到玩家关联代币帐户
```
// Mint tokens to player token account
   pub fn air_drop(ctx: Context<AirDrop>) -> Result<()> {

       // PDA seeds and bump to "sign" for CPI
       let seeds = b"reward";
       let bump = *ctx.bumps.get("reward_token_mint").unwrap();
       let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

       // CPI Context
       let cpi_ctx = CpiContext::new_with_signer(
           ctx.accounts.token_program.to_account_info(),
           MintTo {
               mint: ctx.accounts.reward_token_mint.to_account_info(),
               to: ctx.accounts.player_token_account.to_account_info(),
               authority: ctx.accounts.reward_token_mint.to_account_info(),
           },
           signer,
       );

       // Mint 1 token, accounting for decimals of mint
       let amount = (1u64)
           .checked_mul(10u64.pow(ctx.accounts.reward_token_mint.decimals as u32))
           .unwrap();

       mint_to(cpi_ctx, amount)?;
       Ok(())
   }
```   
- 游戏页面：玩家可以猜土拨鼠在洞外，还是洞内。猜对获得1枚新铸造的$MARMOT，猜错烧毁1枚已获得的$MARMOT
![game page](https://raw.githubusercontent.com/vitabread/marmot-network-file/main/marmot_sc3.png)
猜对调用MintTo指令
![game page](https://raw.githubusercontent.com/vitabread/marmot-network-file/main/marmot_sc4.png)
猜错调用Burn指令
![game page](https://raw.githubusercontent.com/vitabread/marmot-network-file/main/marmot_sc5.png)
使用时钟当下Slot作为种子值以位元移位方式生成伪随基数，以判断玩家猜对或猜错
```
       // Get current slot
       let slot = Clock::get()?.slot;
       // Generate pseudo-random number using XORShift with the current slot as seed
       let xorshift_output = xorshift64(slot);
       // Calculate random result
       let random_result = xorshift_output % 2 + 1;
```
```
pub fn xorshift64(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}
```
- 未来开发
1. 增加代币质押功能，帮助土拨鼠挖洞，以获得更多代币
2. NFT铸造功能，使用$MARMOT铸造铲子等工具NFT以帮助挖洞，增加游戏胜利机率
