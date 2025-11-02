use anchor_lang::prelude::*;

declare_id!("A2NKgCASKEFVLMLZ2rjghx65R9mkMQHom7KEo8PzPihn");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}




#[derive(Accounts)]
pub struct Initialize <'info> {
    pub user:Signer<'info>,
    
}
#[account]
#[derive(InitSpace)]
pub struct vaultState{
    pub vault_bump:u8,
    pub state_bump:u8,

}
// impl Space for vaultState{
//     const INIT_SPACE:usize=8+1+1;


// }