const anchor = require('@project-serum/anchor');

describe('something', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.Something;
    const mintAccount = new anchor.web3.PublicKey("bwrs7C1FFaFfySXw1HmgM98ordGXPW5AKBP7YfvUZ4U");
    const metadataAccount = new anchor.web3.PublicKey("H2R9ggkTtLBMDAo3tTiT73x2ZjKKj3G5WPhV1zYmhCce")
    const tokenMetadataProgram = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
    

    const ownerWallet = anchor.web3.Keypair.fromSecretKey(
      new Uint8Array(
        JSON.parse(require("fs").readFileSync('/Users/aryajakkli/.config/solana/id.json', "utf8"))
      )
    );
        
    let [programPda, myBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("new_update_auth")],
      program.programId,
    )

      
    console.log(programPda.toString());
    
    console.log('tis my bump',myBump)

    console.log('accounts', {

      accounts:{
      mintAccount:mintAccount.toString(),
      programPda: programPda.toString(),
      ownerWallet:ownerWallet.publicKey.toString(),
      metadataAccount:metadataAccount.toString(),
      systemProgram: anchor.web3.SystemProgram.programId.toString(),
      tokenMetadataProgram:tokenMetadataProgram.toString(),
      rent: anchor.web3.SYSVAR_RENT_PUBKEY.toString(),
      
    },
      signers:[ownerWallet]
  
    })
    const tx = await program.rpc.initialize(
      myBump,
      
    {

      accounts:{
      mintAccount:mintAccount,
      programPda: programPda,
      ownerWallet:ownerWallet.publicKey,
      metadataAccount:metadataAccount,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenMetadataProgram:tokenMetadataProgram,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      
    },
      signers:[ownerWallet]
  
    });
    console.log("Your transaction signature", tx);
  });
   
 
    });
