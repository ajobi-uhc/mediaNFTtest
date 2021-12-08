const anchor = require('@project-serum/anchor');
const { token } = require('@project-serum/anchor/dist/cjs/utils');
describe('something', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    const provider = anchor.Provider.env();
    anchor.setProvider(provider);
    const program = anchor.workspace.Something;
    const mintAccount= new anchor.web3.PublicKey("BHGdnvMcmRmZeTYUcsF6zu7yoKL9VBrxzayxfSVDMTz7");
    const tokenMetadataProgram = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
    const tokenAccount = new anchor.web3.PublicKey("2GSwe1ng1LVPpxao2CGbzTcKjFeJ3BJptmQtKQp7h1pL")
    let [metadataAccount, bumpForMetadata] = await anchor.web3.PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("metadata"),tokenMetadataProgram.toBuffer(),mintAccount.toBuffer()],
      tokenMetadataProgram

    )
      console.log("metadataaccount",metadataAccount)
    // const metadataAccount = new anchor.web3.PublicKey("GUA7uT7YRTBg2FcQz8opwWxDJuNeZGcVs7Cn3uUMcmrr")
   
      console.log('metadataaccount',metadataAccount.toString());

    const ownerWallet = anchor.web3.Keypair.fromSecretKey(
      new Uint8Array(
        JSON.parse(require("fs").readFileSync('/Users/aryajakkli/.config/solana/id.json', "utf8"))
      )
    );
      
      // const ownerWallet = new anchor.web3.PublicKey("FKviGeU7L1djHrbf21z2ijK2N9KAREagtTjhoc9KQZn6")

    let [programPda, myBump] = await anchor.web3.PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("new_update_auth")],
      program.programId,
    )
    

    // let [findAssociatedToken, associatedBump] = await anchor.web3.PublicKey.findProgramAddress(
    //   [anchor],
    //   program.programId
    // )


    
//PDA: AGKVw4WA8A84BzGVFZquVuVyrTDWPTXgaJwpVr2CYQfD
      console.log('program id', program.programId.toString())
      console.log('created pda:',programPda.toString())
      // console.log('owner wallet', ownerWallet.publicKey.toString())

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


  //   const tx = await program.rpc.initialize(
  //     myBump,
  //     programPda.programId,
      
  //   {

  //     accounts:{
  //     programPda: programPda,
  //     ownerWallet:ownerWallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
      
      
      
  //   },
  //     signers:[ownerWallet]
  
  // });
  //   console.log("Your transaction signature", tx);

    const newtx = await program.rpc.lock(myBump,bumpForMetadata,{

      accounts:{
        tokenMetadataProgram:tokenMetadataProgram,
        mintAccount:mintAccount,
        ownerWallet:ownerWallet.publicKey,
        tokenAccount:tokenAccount,
        programPdaSigner: programPda,
        metadataAccount:metadataAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        
      },
        signers:[ownerWallet]

    })

    console.log("new tx worked!!",newtx)
  });
   
 
    });
