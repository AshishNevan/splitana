use anchor_lang::prelude::*;

declare_id!("73qPic8NHyrWJQdX16PSEqESYBKZo4y7awKUYF3kTszY");

#[program]
pub mod splitana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.name = ctx.accounts.payer.key();
        user.money = 0;
        user.group_index = 0;
        user.group_tracker = ctx.accounts.group_tracker.key();
        msg!(
            "user pubkey: {:?}, money: {:?}, groups joined: {:?}",
            user.name,
            user.money,
            user.group_index
        );
        Ok(())
    }

    pub fn add_group(ctx: Context<AddGroup>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        let len = user.group_index;
        user.group_index = len + 1;

        let group_tracker = &mut ctx.accounts.group_tracker;
        group_tracker.groups[len as usize] = ctx.accounts.group.key();

        let group = &mut ctx.accounts.group;
        group.title = "group1".to_string();
        group.expense_index = 0;
        group.expense_tracker = ctx.accounts.expense_tracker.key();
        group.participants[0] = user.key();

        msg!(
            "user: {:?} has joined group {:?} {:?}",
            ctx.accounts.user.name,
            ctx.accounts.user.group_index,
            ctx.accounts.group_tracker.groups[ctx.accounts.user.group_index as usize - 1]
        );
        Ok(())
    }

    pub fn get_group(ctx: Context<GetGroup>) -> Result<()> {
        let user = &ctx.accounts.user;
        let group_tracker = &ctx.accounts.group_tracker;
        msg!("user {:?} is in {:?} groups", user.name, user.group_index);
        for i in 0..user.group_index {
            msg!(
                "group {:?} pubkey = {:?}",
                i + 1,
                group_tracker.groups[i as usize]
            );
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = User::MAX_SIZE, seeds = [b"splitana-user", payer.key().as_ref()], bump)]
    pub user: Box<Account<'info, User>>,
    #[account(init, payer = payer, space = GroupTracker::MAX_SIZE, seeds = [b"splitana-user-gtracker", payer.key().as_ref()], bump)]
    pub group_tracker: Account<'info, GroupTracker>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGroup<'info> {
    #[account(mut)]
    pub user: Box<Account<'info, User>>,
    #[account(mut)]
    pub group_tracker: Box<Account<'info, GroupTracker>>,
    #[account(init, payer = payer, space = Group::MAX_SIZE, seeds = [b"splitana-group", payer.key().as_ref()], bump)]
    pub group: Box<Account<'info, Group>>,
    #[account(init, payer = payer, space = ExpenseTracker::MAX_SIZE, seeds = [b"splitana-group-etracker", payer.key().as_ref()], bump)]
    pub expense_tracker: Box<Account<'info, ExpenseTracker>>,
    // #[account(init, payer = payer, space = ParticipantTracker::MAX_SIZE, seeds = [b"splitana-group-ptracker", payer.key().as_ref()], bump)]
    // pub participant_tracker: Box<Account<'info, ParticipantTracker>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetGroup<'info> {
    pub user: Account<'info, User>,
    pub group_tracker: Account<'info, GroupTracker>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct User {
    pub name: Pubkey,          // 32 bytes
    pub money: u32,            // 4 bytes
    pub group_tracker: Pubkey, // 32 bytes
    pub group_index: u8,       // 1 byte
}
impl User {
    pub const MAX_SIZE: usize = 8 + 32 + 4 + 32 + 1;
}

#[account]
pub struct Group {
    pub title: String,             // 32 bytes
    pub expense_index: u8,         // 1 byte
    pub expense_tracker: Pubkey,   // 32 bytes
    pub participants: [Pubkey; 8], // 8 * 32 bytes
}
impl Group {
    pub const MAX_SIZE: usize = 8 + 32 + 1 + 32 + (8 * 32);
}

#[account]
pub struct GroupTracker {
    pub groups: [Pubkey; 5],
}
impl GroupTracker {
    pub const MAX_SIZE: usize = 8 + (32 * 5);
}

#[account]
pub struct ExpenseTracker {
    pub expenses: [Pubkey; 10],
}
impl ExpenseTracker {
    pub const MAX_SIZE: usize = 8 + (32 * 10);
}

// #[account]
// pub struct ParticipantTracker {
//     pub expenses: Pubkey,
// }
// impl ParticipantTracker {
//     pub const MAX_SIZE: usize = 8 + 32;
// }

#[account]
pub struct Expense {
    pub title: String,           // 8 bytes
    pub amount: u32,             // 4 bytes
    pub payer: Pubkey,           // 32 bytes
    pub participants: [bool; 8], // 1 * 8
}
impl Expense {
    pub const MAX_SIZE: usize = 8 + 8 + 4 + 32 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct Participant {
    pub name: String, // 8 bytes
    pub amount: u32,  // 4 bytes
}
