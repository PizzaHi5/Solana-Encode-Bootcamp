import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Hw13 } from "../target/types/hw13";
import { assert } from "chai";
import { PublicKey, Keypair } from '@solana/web3.js';
import { readFileSync } from 'fs';
import * as bs58 from 'bs58';

describe("hw13", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Hw13 as Program<Hw13>;

  it("Is initialized!", async () => {
    // Add your test here.
    const programAddress = "GyxSxejrutiCUwT1hfGMmTMYtRNUwvvMAB18nYBCcvgo";
    console.log(programAddress);

    //nope
    const programKeyPair = Keypair.generate();

    // Did a lot of work trying to change the signer to my personal testnet
    // wallet. I figured that would solve the problem since I was initially
    // using the programAddress as the signer. I got the same error for both.
    // Error: Unknown signer: <key>
    // Everything below is me learning how to use my own wallet for signing
    // this transaction
    const keypairPath = 'wallet/keypair.json';
    console.log("error here?");
    const keypairFileContents = readFileSync(keypairPath, 'utf-8');
    console.log("error here?2");
    const privateKeyArray = JSON.parse(keypairFileContents);
    console.log("error here?3");
    // Tried this, ran into error
    //const privateKey = Uint8Array.from(Buffer.from(privateKeything, 'base64'));
    // Tried this too
    //const privateKeyString = privateKeything.secretKey;
    console.log("error here?4");
    // Learned key is not base64 encoded and is not base58 encoded
    //const privKeyBuffer = Buffer.from(privateKeyString);
    // Learned that my key is an array and must be made into a uint8array
    const privateKeyBytes = new Uint8Array(privateKeyArray.length);
    for (let i = 0; i < privateKeyArray.length; i++) {
      privateKeyBytes[i] = privateKeyArray[i];
    }
    console.log("6?");
    //const privateKey = bs58.decode(privKeyBuffer);
    console.log("7?");
    const keyPair = Keypair.fromSecretKey(privateKeyBytes); 
    console.log("error here?5");

    //running into problems here for some reason
    // - learned that the dataAccount needed to be specified
    // - specifying the signer has kept leading to "unknown signer" error
    const tx = await program.methods.initialize()
      .accounts({
        dataAccount: programAddress,
        //putting in keypair.publicKey.toBase58() got "program already initialized" error
        // Learned that the 32byte representation of the address is the base58 of the publicKey
        //dataAccount: keyPair.publicKey.toBase58(),
      })
      //.signers([keyPair])
      .rpc();
    console.log("Your transaction signature", tx);
    
    //Trying to access the data here using 2 different approaches
    const accountData = await program.methods.getData().rpc();
    const accountData2 = await program.account.starting.fetch(program.programId);
    console.log("Account Data is: ", accountData);
    console.log("Try #2: ", accountData2.data);
  });
});
