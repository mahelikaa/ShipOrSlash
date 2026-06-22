pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("D2qZSL5Uuxiqt4fJSjgGDJU6p3ntSCicQBiU1ur2gZz6");

#[program]
pub mod ship_or_slash {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
