use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("g64EtQutPeM1tB3d16LWZzBryhT27G2g16pyHVUhXVS");

#[program]
pub mod sol_props {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, name: String) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.name = name;
        user_profile.prop_list = 0;
        user_profile.authority = ctx.accounts.authority.key();

        Ok(())
    }

    pub fn create_property(
        ctx: Context<CreateProperty>,
        name: String,
        description: String,
        price: u64,
    ) -> Result<()> {
        let new_property = &mut ctx.accounts.property;
        let user_profile = &mut ctx.accounts.user_profile;

        new_property.name = name;
        new_property.property_description = description;
        new_property.price = price;
        new_property.property_owner = ctx.accounts.authority.key();
        user_profile.prop_list = user_profile.prop_list.checked_add(1).unwrap();

        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub prop_list: u8,
    pub name: String
}

#[account]
#[derive(Default)]
pub struct Property {
    name: String,
    property_description: String,
    price: u64,
    property_owner: Pubkey,
}

#[derive(Accounts)]
#[instruction()]
pub struct CreateProfile<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        seeds = [b"user_tag".as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = size_of::<UserProfile>() + 8
    )]
    pub user_profile: Account<'info, UserProfile>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction()]
pub struct CreateProperty<'info> {
    #[account(
        mut,
        seeds = [b"user_tag".as_ref(), authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(
        init,
        seeds = [b"create_property".as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = size_of::<Property>() + 8
    )]
    pub property: Account<'info, Property>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}
