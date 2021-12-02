use anchor_lang::prelude::*;
// use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token::{self, Mint, TokenAccount, Token, SetAuthority, ID as SPL_TOKEN_ID};
// use metaplex_token_metadata::instruction::{
//     mint_new_edition_from_master_edition_via_token,
//     create_master_edition,
//     create_metadata_accounts,
// };

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/*
Code to create a mediaNFT locker:
One instruction :lock_nft that takes in the NFT wallet owner as a pubkey
and the NFT mint address. 

Using mint address and owner it then performs validations and then rederives the metadata struct of the NFT
It checks to make sure locked isnt true and then proceeds to generate a PDA based on 
the currently executing program id and then signs the update_metadata instruction

Q:what seeds do I use for the PDA, and how can I then transfer authority back to another wallet
after program is done executing
*/
#[program]
pub mod something {
    use metaplex_token_metadata::utils::{assert_initialized, assert_owned_by, assert_signer};
    use metaplex_token_metadata::state::{Metadata,Data};
    use metaplex_token_metadata::instruction::{update_metadata_accounts};
    use super::*;
    

    pub fn initialize(ctx: Context<Initialize>, init_nonce:u8) -> ProgramResult {
        let program_pda_account = &mut ctx.accounts.program_pda;
        program_pda_account.signerA = "true".to_string();
        Ok(())
    }



    pub fn lock(ctx: Context<Lock>, init_nonce:u8) -> ProgramResult {

        // let user_mint = &ctx.accounts.mint_account;
        
        // let owner_wallet = &ctx.accounts.owner_wallet;
        let mint_metadata = & Metadata::from_account_info(&ctx.accounts.metadata_account)?;
        // let token_metadata_program = &ctx.accounts.token_metadata_program;
        
        
        // assert_eq!(token_metadata_program.key,&ID);//token metadata program should be valid
        // assert_owned_by(mint_metadata_account, ctx.accounts.token_metadata_program.key)?; //metadata accunt must be owned by metadata program
        // assert_owned_by(&user_mint.to_account_info(),owner_wallet.key)?; //mint authority must be users wallet
        // assert_signer(&owner_wallet)?;

        //On the client side generate a program address using this programs id as a seed

      
        //Maybe implement creator wallet check
        // assert_eq!(mint_metadata.data.creators.,eluuneCreator);

        let new_data = Data {
            name: mint_metadata.data.name.clone(),
            symbol: mint_metadata.data.symbol.clone(),
            uri: "https://ifgv27za22gw26hddoplvrj6yp6jn2wprr26oqe3wu6hsoqins3q.arweave.net/QU1dfyDWjW144xueusU-w_yW6s-MdedAm7U8eToIbLc".to_string(),
            seller_fee_basis_points: mint_metadata.data.seller_fee_basis_points,
            creators: mint_metadata.data.creators.clone(),
        }; 
        let metadata_infos = [
            ctx.accounts.metadata_account.clone(),
            ctx.accounts.program_pda_signer.to_account_info()

        ];
        let update_auth_seeds = [
            b"new_update_auth".as_ref(),
            b"Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS".as_ref(),
            &[init_nonce]
        ];
       

        anchor_lang::solana_program::program::invoke_signed(
            &update_metadata_accounts(
                *ctx.accounts.token_metadata_program.key, 
               *ctx.accounts.metadata_account.key, 
               * ctx.accounts.program_pda_signer.to_account_info().key,
            None,
            Some(new_data),
            None,
             
                ), 

               &metadata_infos,

              &[&update_auth_seeds],
        
        )?;
     


       
        Ok(())
    }
}


//Token account and mint is taken in with constraints as seeds 
//Creating  ['metaplex', metaplex_program_id, mint_id].
#[derive(Accounts)]
#[instruction(my_bump:u8)]
pub struct Initialize<'info> {
    


#[account( 
    init,
    seeds = [
    b"new_update_auth".as_ref(),
    b"Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS".as_ref()],
    bump = my_bump,
    payer = owner_wallet,
    space = 400
)]  
pub program_pda: Account<'info,SignerA>,

#[account(mut)]
pub owner_wallet: Signer<'info>,
pub system_program: Program<'info, System>,




}

#[derive(Accounts)]
#[instruction(my_bump:u8)]
pub struct Lock<'info> {
    pub token_metadata_program: AccountInfo<'info>,
    pub mint_account: Account<'info,Mint>,
    

    #[account(seeds = [b"new_update_auth".as_ref(),b"Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS".as_ref()],bump=my_bump)]
    pub program_pda_signer: Account<'info,SignerA>,

    #[account(
        mut, 
        seeds = ["metaplex".as_bytes(),
        ID.as_ref(),
        mint_account.to_account_info().key.as_ref()],
        bump = my_bump
    )]
    pub metadata_account: AccountInfo<'info>,  

    pub owner_wallet: Signer<'info>,

    pub system_program: Program<'info, System>,
   
}

#[account]
pub struct SignerA{
    pub signerA: String,
}