use anchor_lang::prelude::*;
use num_derive::*;
use rand_chacha::rand_core::SeedableRng;
use rand_core::RngCore;
use rand_chacha; // 0.3.0

declare_id!("28MENXAPtRQmZLXmjBEsbVQv6wf1g7KP4KgMHJS5GHuc");

#[program]
pub mod seventy {
    use super::*;
    use anchor_lang::solana_program::{program::invoke, system_instruction::transfer};

    pub fn setup(ctx: Context<Setup>, vendor: Pubkey, bet_amount: u64, bet_amount_jack: u64, player_choice: u64, player_choice_jack: u64, player_seed: i64) -> Result<()> {
        let seventy = &mut ctx.accounts.seventy;

        msg!("setup");
        seventy.players = [vendor, ctx.accounts.player.key()];
        seventy.player_seed = player_seed;
        seventy.bump = *ctx.bumps.get("seventy").unwrap();
        seventy.bet_amount = bet_amount;
        seventy.bet_amount_jack = bet_amount_jack;
        seventy.player_choice = player_choice;
        seventy.player_choice_jack = player_choice_jack;

        //let wallet_key = Pubkey::from_str("EkNURs18Y1kCZ43kBo2tHK56cB6KCKEva8VbhrV1WpUv").unwrap();

        msg!("transfer to game wallet 1");
        invoke(
            &transfer(
                //ctx.accounts.vendor.to_account_info().key,
                ctx.accounts.player.to_account_info().key,
                seventy.to_account_info().key,
                seventy.bet_amount*1035/1000, //(total_bet/10) - seventy.bet_amount,
            ),
            &[
                //ctx.accounts.vendor.to_account_info(),
                ctx.accounts.player.to_account_info(),
                seventy.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
        msg!("transfer to game wallet 2");

        /*invoke(
            &transfer(
                ctx.accounts.player.to_account_info().key,
                ctx.accounts.vault.to_account_info().key,
                seventy.bet_amount*35/1000, //(total_bet/10) - seventy.bet_amount,
            ),
            &[
                ctx.accounts.player.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;*/

        msg!("transfer to game wallet 3");

        if seventy.player_choice_jack != 0 {
            invoke(
                &transfer(
                    ctx.accounts.player.to_account_info().key,
                    seventy.to_account_info().key,
                    seventy.bet_amount_jack*1035/1000, //(total_bet/10) - seventy.bet_amount,
                ),
                &[
                    ctx.accounts.player.to_account_info(),
                    seventy.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
            msg!("jackpot");
            /*invoke(
                &transfer(
                    ctx.accounts.player.to_account_info().key,
                    ctx.accounts.vault.to_account_info().key,
                    seventy.bet_amount_jack*35/1000, //(total_bet/10) - seventy.bet_amount,
                ),
                &[
                    ctx.accounts.player.to_account_info(),
                    ctx.accounts.vault.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;*/
        }
        msg!("end");

        Ok(())
    }


    pub fn play(ctx: Context<Play>, vendor_seed: i64) -> Result<()> {
        let seventy = &mut ctx.accounts.seventy;
        let vendor_seed = vendor_seed;

        let player_side = if seventy.player_choice == 0 {
            Side::Seventy
        } else if seventy.player_choice == 1 {
            Side::UnderSeventy
        } else if seventy.player_choice == 2 {
            Side::OverSeventy
        } else if seventy.player_choice == 3 {
            Side::UnderSeventy50
        } else {
            Side::OverSeventy50
        };


        let player_side_jack = if seventy.player_choice_jack == 0 {
            Side::JackPotNone
        } else if seventy.player_choice_jack == 1 {
            Side::JackPot2
        } else if seventy.player_choice_jack == 2 {
            Side::JackPot12
        } else if seventy.player_choice_jack == 3 {
            Side::JackPot3
        } else  { //4
            Side::JackPot11
        };

        let total_bet = if seventy.player_choice == 0 {
            seventy.bet_amount * 60
        } else if seventy.player_choice == 1 {
            seventy.bet_amount * 24
        } else if seventy.player_choice == 2 {
            seventy.bet_amount * 24
        } else if seventy.player_choice == 3 { //UnderSeventy50
            seventy.bet_amount * 20
        } else { //OverSeventy50
            seventy.bet_amount * 20
        };


        let total_bet_jack = if seventy.player_choice_jack == 0 { // JackPotNone
            seventy.bet_amount_jack * 0
        } else if seventy.player_choice_jack == 1 { // JackPot2
            seventy.bet_amount_jack * 350
        } else if seventy.player_choice_jack == 2 { //JackPot12
            seventy.bet_amount_jack * 350
        } else if seventy.player_choice_jack == 3 { //JackPot3
            seventy.bet_amount_jack * 170
        } else {  //JackPot11
            seventy.bet_amount_jack * 170
        };


        if **ctx.accounts.vendor.to_account_info().try_borrow_lamports()? < (total_bet/10) - seventy.bet_amount {
            **seventy.to_account_info().try_borrow_mut_lamports()? -= seventy.bet_amount;
            **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += seventy.bet_amount;
            return err!(SeventyError::InsuficientRewardBalance);
        }

        if seventy.player_choice_jack != 0 {
            if **ctx.accounts.vendor.to_account_info().try_borrow_lamports()? < (total_bet_jack/10) - seventy.bet_amount_jack {
                **seventy.to_account_info().try_borrow_mut_lamports()? -= seventy.bet_amount_jack;
                **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += seventy.bet_amount_jack;
                return err!(SeventyError::InsuficientRewardBalance);
            }
        }

        msg!("play");

        invoke(
            &transfer(
                ctx.accounts.vendor.to_account_info().key, //ctx.accounts.player.to_account_info().key,
                seventy.to_account_info().key,
                (total_bet/10) - seventy.bet_amount, //seventy.bet_amount,
            ),
            &[
                ctx.accounts.vendor.to_account_info(),
                seventy.to_account_info(), //seventy.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        if seventy.player_choice_jack != 0 {
            invoke(
                &transfer(
                    ctx.accounts.vendor.to_account_info().key, //ctx.accounts.player.to_account_info().key,
                    seventy.to_account_info().key,
                    (total_bet_jack/10) - seventy.bet_amount_jack, //seventy.bet_amount,
                ),
                &[
                    ctx.accounts.vendor.to_account_info(),
                    seventy.to_account_info(), //seventy.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        let (winner, winner_jack)  = seventy.play(vendor_seed, player_side, player_side_jack);

        if winner != *ctx.accounts.vendor.key {
            msg!("Congratulations, You won!");
            **seventy.to_account_info().try_borrow_mut_lamports()? -= total_bet/10;
            **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += total_bet/10;
        } else {
            msg!("You lost!");
            **seventy.to_account_info().try_borrow_mut_lamports()? -= total_bet/10;
            **ctx.accounts.vendor.try_borrow_mut_lamports()? += total_bet/10;
        }

        if winner_jack != *ctx.accounts.vendor.key {
            msg!("Congratulations, You won JackPot!!!!!!!");
            **seventy.to_account_info().try_borrow_mut_lamports()? -= total_bet_jack/10;
            **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += total_bet_jack/10;
        } else {
            msg!("You lost!");
            **seventy.to_account_info().try_borrow_mut_lamports()? -= total_bet_jack/10;
            **ctx.accounts.vendor.try_borrow_mut_lamports()? += total_bet_jack/10;
        }

        Ok(())
    }


    pub fn delete(_ctx: Context<Delete>) -> Result<()> {
        /*let seventy = &mut _ctx.accounts.seventy;
        let amount = **seventy.to_account_info().try_borrow_mut_lamports()?;
        **seventy.to_account_info().try_borrow_mut_lamports()? -= amount;
        **_ctx.accounts.player.try_borrow_mut_lamports()? += amount;
        */
        //**_ctx.accounts.vendor.try_borrow_mut_lamports()? += amount;
        Ok(())
    }
}

//#[instruction(player: Pubkey, bet_amount: u64, player_choice: u64, vendor_seed: i64)]

#[derive(Accounts)]
#[instruction(vendor: Pubkey, bet_amount: u64, player_choice: u64, player_seed: i64)]
pub struct Setup<'info> {
    #[account(
        init,
        payer = player, //payer = vendor,
        space = Seventy::LEN,
        seeds = [b"sixty", player.key().as_ref(), vendor.as_ref()], bump //seeds = [b"seventy", vendor.key().as_ref(), player.as_ref()], bump
    )]
    pub seventy: Account<'info, Seventy>,
    #[account(mut)]
    pub player: Signer<'info>, //pub vendor: Signer<'info>,
    /// CHECK
    pub vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(
        mut,
        seeds = [b"sixty", player.key().as_ref(), vendor.key().as_ref()], bump //  seeds = [b"seventy", vendor.key().as_ref(), player.key().as_ref()], bump
    )]
    pub seventy: Account<'info, Seventy>,
    #[account(mut)]
    pub vendor: Signer<'info>, //pub player: Signer<'info>,
    #[account(mut)]
    /// CHECK
    pub player : AccountInfo<'info>, //pub vendor : AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
//#[instruction(player: Pubkey)] //#[instruction(player: Pubkey)]
pub struct Delete<'info> {
    #[account(
        mut,
        close = vendor, //close = vendor,
        seeds = [b"sixty", player.key().as_ref(), vendor.key().as_ref()], bump         //seeds = [b"seventy", vendor.key().as_ref(), player.as_ref()], bump
    )]
    pub seventy: Account<'info, Seventy>,
    #[account(mut)]
    pub vendor: Signer<'info>, //pub vendor: Signer<'info>,
    /// CHECK
    pub player : AccountInfo<'info>,  //added
    pub system_program: Program<'info, System>,
}


#[account]
#[derive(Default)]
pub struct Seventy {
    players: [Pubkey; 2],
    player_seed: i64, //vendor_seed: i64,
    state: SeventyState,
    bet_amount: u64,
    player_choice: u64,
    bet_amount_jack: u64,
    player_choice_jack: u64,
    bump: u8
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum SeventyState {
    Active,
    Finished { winner: Pubkey,  winner_jack: Pubkey, dice1: u8, dice2: u8},
}

impl Default for SeventyState {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq)]
pub enum Side {
    Seventy,
    UnderSeventy,
    OverSeventy,
    UnderSeventy50,
    OverSeventy50,
    JackPot2,
    JackPot12,
    JackPot3,
    JackPot11,
    JackPotNone
}


impl Seventy {
    const LEN: usize = 64 + 8 + 33 + 8 + 8 + 8 + 10 + 16 + 32 + 64; //

    fn check_winner(&self, player_side: Side, sum: u64) -> bool {

        let result : bool = if player_side == Side::Seventy && sum == 7 {
            true
        } else if player_side == Side::Seventy && sum != 7 {
            false
        } else if player_side == Side::UnderSeventy && sum < 7 {
            true
        } else if player_side == Side::UnderSeventy && sum >= 7 {
            false
        } else if player_side == Side::OverSeventy && sum > 7 {
            true
        } else if player_side == Side::OverSeventy && sum <= 7 {
            false
        } else if player_side == Side::UnderSeventy50 && sum < 7 {
            true
        } else if player_side == Side::UnderSeventy50 && sum >= 7 {
            false
        } else if player_side == Side::OverSeventy50 && sum > 7 {
            true
        } else if player_side == Side::OverSeventy50 && sum <= 7 {
            false
        } else if player_side == Side::JackPot2 && sum == 2 {
            true
        } else if player_side == Side::JackPot2 && sum != 2 {
            false
        } else if player_side == Side::JackPot12 && sum == 12 {
            true
        } else if player_side == Side::JackPot12 && sum != 12 {
            false
        } else if player_side == Side::JackPot3 && sum == 3 {
            true
        } else if player_side == Side::JackPot3 && sum != 3 {
            false
        } else if player_side == Side::JackPot11 && sum == 11 {
            true
        } else if player_side == Side::JackPot11 && sum != 11 {
            false
        } else {
            false
        };

        return result;
    }


    pub fn play(&mut self, vendor_seed: i64, player_side: Side, player_side_jack: Side) -> (Pubkey, Pubkey) {

        let c: Clock = Clock::get().unwrap();
        let mut gen = rand_chacha::ChaCha8Rng::seed_from_u64((self.player_seed + vendor_seed + (c.unix_timestamp as i64)) as u64 );
        let mut dice1 = gen.next_u64()%6 + 1;
        let mut dice2 = gen.next_u64()%6 + 1;
        let mut sum = dice1 + dice2;

        if player_side == Side::UnderSeventy50 || player_side == Side::OverSeventy50 {
            while sum == 7  {
                dice1 = gen.next_u64()%6 + 1;
                dice2 = gen.next_u64()%6 + 1;
                sum = dice1 + dice2;
            }
        }
        let dice_result : bool = self.check_winner(player_side, sum );
        let dice_result_jack : bool = self.check_winner(player_side_jack, sum );

        let winner : Pubkey = if dice_result == true {
            self.players[1]
        } else {
            self.players[0]
        };

        let winner_jack : Pubkey = if dice_result_jack == true {
            self.players[1]
        } else {
            self.players[0]
        };

        self.state = SeventyState::Finished {
            winner: winner,
            winner_jack: winner_jack,
            dice1: dice1 as u8,
            dice2: dice2 as u8
        };
        return (winner, winner_jack);
    }

}


#[error_code]
pub enum SeventyError {
    #[msg("Insuficient rewards balance, try with an smaller amount.")]
    InsuficientRewardBalance,
}
