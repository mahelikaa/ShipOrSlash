use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("This bond has already been settled")]
    AlreadySettled,

    #[msg("The deadline has not reached.")]
    DeadlineNotReached,

    #[msg("Invalid Switchboard feed account")]
    InvalidFeed,

    #[msg("Feed has no value yet")]
    FeedValueMissing,
}
