use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ProtocolConfig {
    pub treasury: Pubkey, //32
    pub feed: Pubkey, //32
    pub slash_bps: u16, //2
    pub bump: u8,//1
}

#[account]
#[derive(InitSpace)]
pub struct Bond {
    pub owner: Pubkey, // 32
    #[max_len(39)]
    pub github_username: String,// 4 + 39
    pub target: u32, //4
    pub deadline: i64, //8
    pub lamports: u64, //8
    pub settled: bool, //1
    pub bump: u8,//1
    pub vault_bump: u8,//1
}