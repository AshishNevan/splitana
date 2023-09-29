import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Splitana } from "../target/types/splitana";

describe("splitana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Splitana as Program<Splitana>;

  const wallet = anchor.web3.Keypair.generate();

  const splitanaPK = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("splitana-user"), wallet.publicKey.toBuffer()], program.programId)[0];
  const groupTrackerPK = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("splitana-user-gtracker"), wallet.publicKey.toBuffer()], program.programId)[0];
  let groupIndex = 0;
  const getGroupData = (groupIndex: number) => {
    /**
     *@return group pubkey, group expense tracker pubkey 
     */
    return [anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("splitana-group"), wallet.publicKey.toBuffer()], program.programId)[0], anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("splitana-group-etracker"), wallet.publicKey.toBuffer()], program.programId)[0], anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("splitana-group-ptracker"), wallet.publicKey.toBuffer()], program.programId)[0]];
  }
  it("request airdrop", async () => {
    const tx = await anchor.AnchorProvider.env().connection.requestAirdrop(wallet.publicKey, 1000000000);
    await anchor.AnchorProvider.env().connection.confirmTransaction(tx);
  })
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
      .accounts(
        {
          user: splitanaPK,
          groupTracker: groupTrackerPK,
          payer: wallet.publicKey,
        }
      )
      .signers([wallet])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  // it("add group", async () => {
  //   const tx = await program.methods.addGroup()
  //     .accounts(
  //       {
  //         user: splitanaPK,
  //         group: getGroupData(groupIndex)[0],
  //         expenseTracker: getGroupData(groupIndex++)[1],
  //         payer: wallet.publicKey
  //       }
  //     )
  //     .signers([wallet])
  //     .rpc();
  //   console.log("add group tx:", tx);
  // })

  it("add group", async () => {
    const tx = await program.methods.addGroup()
      .accounts(
        {
          user: splitanaPK,
          groupTracker: groupTrackerPK,
          group: getGroupData(1)[0],
          expenseTracker: getGroupData(1)[1],
          // participantTracker: getGroupData(1)[2],
          payer: wallet.publicKey
        }
      )
      .signers([wallet])
      .rpc();
    console.log("add group tx:", tx);
  })

  it("get group", async () => {
    const tx = await program.methods.getGroup()
      .accounts(
        {
          user: splitanaPK,
          groupTracker: groupTrackerPK
        }
      )
      .signers([])
      .rpc();
    console.log("get group tx:", tx);
  })
});
