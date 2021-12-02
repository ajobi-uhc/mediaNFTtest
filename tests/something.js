const anchor = require('@project-serum/anchor');
describe('something', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    const provider = anchor.Provider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Something;
    const mintAccount= new anchor.web3.PublicKey("bwrs7C1FFaFfySXw1HmgM98ordGXPW5AKBP7YfvUZ4U");
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

      console.log('program id', program.programId.toString())
      console.log('created pda:',programPda.toString())
      console.log('owner wallet', ownerWallet.publicKey.toString())

    // console.log(' program pda: ', await program.provider.connection.getAccountInfo(programPda));
    // console.log('ownerWallet', await program.provider.connection.getAccountInfo(ownerWallet));
    
    // console.log(programPda.toString());
    // console.log('program ID',program.programId.toString())
    // console.log('tis my bump',myBump)
    // console.log(program.idl.instructions[0]);
    // console.log('accounts', {

    //   accounts:{
    //   mintAccount:mintAccount.toString(),
    //   programPda: programPda.toString(),
    //   ownerWallet:ownerWallet.publicKey.toString(),
    //   metadataAccount:metadataAccount.toString(),
    //   systemProgram: anchor.web3.SystemProgram.programId.toString(),
    //   tokenMetadataProgram:tokenMetadataProgram.toString(),
    //   rent: anchor.web3.SYSVAR_RENT_PUBKEY.toString(),
      
    // },
    //   signers:[ownerWallet]
  
    // })


    const tx = await program.rpc.initialize(
      myBump,
      
    {

      accounts:{
      programPda: programPda,
      ownerWallet:ownerWallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      
      
      
    },
      signers:[ownerWallet]
  
  });
    console.log("Your transaction signature", tx);

    const newtx = await program.rpc.lock(myBump,{

      accounts:{
        tokenMetadataProgram:tokenMetadataProgram,
        mintAccount:mintAccount,
        programPdaSigner: programPda,
        metadataAccount:metadataAccount,
        ownerWallet:provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        
      },
        signers:[ownerWallet]

    })

    console.log("new tx worked!!",newtx)
  });
   
 
    });
