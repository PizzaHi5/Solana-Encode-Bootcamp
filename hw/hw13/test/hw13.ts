import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hw13 } from "../target/types/hw13";
import { assert } from "chai";

describe("hw13", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Hw13 as Program<Hw13>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
    
    //Trying to access the data here!!!
    const accountData = await program.methods.getData().rpc();
    const accountData2 = await program.account.starting.fetch(program.programId);
    console.log("Account Data is: ", accountData);
    console.log("Try #2: ", accountData2.data);
  });
});
