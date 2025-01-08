use anchor_lang::prelude::*;

declare_id!("BskoijGEHEMBdEGJuwi8JfYySWpzTYTcgLGzaYxuv9Pk");

#[program]
pub mod voting_program {

    use super::*;

    pub fn initialize_poll(ctx: Context<InitializePoll>,title:String,options:Vec<String>,) -> Result<()> {
        require!(options.len() >= 2, VotingError::NotEnoughOptions);
        require!(options.len() <= 5, VotingError::TooManyOptions);
        let poll = &mut ctx.accounts.poll;
        poll.title = title;
        poll.options = options;
        poll.vote_counts = vec![0; poll.options.len()];
        poll.authority = *ctx.accounts.authority.key;
        poll.is_active = true;
        poll.total_votes = 0;
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>,option_index:u8) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        require!(poll.is_active, VotingError::PollNotActive);
        require!(option_index < poll.options.len() as u8, VotingError::InvalidOption);
        let voter = &mut ctx.accounts.voter;
        require!(!voter.has_voted, VotingError::AlreadyVoted);
        poll.vote_counts[option_index as usize] += 1;
        poll.total_votes += 1;
        voter.has_voted = true;
        Ok(())
    }

    pub fn end_poll(ctx: Context<EndPoll>) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        require!(poll.authority == *ctx.accounts.authority.key, VotingError::UnauthorizedAccess);
        require!(poll.is_active, VotingError::PollNotActive);
        poll.is_active = false;
        Ok(())

    }
}

#[derive(Accounts)]
pub struct InitializePoll<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 200 + 4 + (4*50) + 1 + 8, seeds = [b"poll", authority.key().as_ref()],bump)]//discriminator + pubkey + title + options vector + is_active + total_votes
    pub poll: Account<'info,Poll>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    #[account(init_if_needed, payer = voter_account, space = 8 + 32 + 1, seeds = [b"voter", poll.key().as_ref(), voter_account.key().as_ref()], bump)]// discriminator + voter pubkey + has_voted
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub voter_account: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndPoll<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Poll {
    pub authority: Pubkey,
    pub title: String,
    pub options: Vec<String>,
    pub vote_counts: Vec<u64>,
    pub is_active: bool,
    pub total_votes: u64,
}

#[account]
pub struct Voter {
    pub voter: Pubkey,
    pub has_voted: bool,
}

#[error_code]
pub enum VotingError {
    #[msg("At least two options are required for voting")]
    NotEnoughOptions,
    #[msg("There can be a maximum of 5 options for voting")]
    TooManyOptions,
    #[msg("The poll isn't active")]
    PollNotActive,
    #[msg("Invalid option")]
    InvalidOption,
    #[msg("You have already voted")]
    AlreadyVoted,
    #[msg("You are not authorised for this operation")]
    UnauthorizedAccess,
}
