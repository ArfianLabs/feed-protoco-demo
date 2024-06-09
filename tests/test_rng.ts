import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TestRng } from "../target/types/test_rng";
import { PublicKey } from "@solana/web3.js";
import { deserialize } from "borsh";
import bs58 from "bs58";
import { CurrentFeed, CurrentFeedSchema, rng_program } from "./utils";
import { key } from "./key";
const LAMPORTS_PER_SOL = 1000000000;

const programId = new PublicKey("Eeug9CDxq84xng6vJNKScoWDNQ2vpg865TpqYrhMbGz8");

describe("test_rng", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = new Program<TestRng>(
    require("../target/idl/test_rng.json"),
    programId,
    provider
  );
  //const program = anchor.workspace.TestRng as Program<TestRng>;
  const connection = program.provider.connection;

  const uint8Array = new Uint8Array(key);
  const admin = anchor.web3.Keypair.fromSecretKey(uint8Array);

  it("Is initialized!", async () => {
    /*
    try {
      const [dataPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("data")],
        program.programId
      );

      console.log("dataPDA", dataPDA.toBase58());
      const total_value = new anchor.BN(15);
      await program.methods
        .initialize(total_value)
        .accounts({
          data: dataPDA,
          signer: admin.publicKey,
        })
        .signers([admin])
        .rpc();
    } catch (error) {
      console.log("error ", error);
    }*/
  });

  it("get rand!", async () => {
    try {
      const [dataPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("data")],
        program.programId
      );

      const current_feeds_account = PublicKey.findProgramAddressSync(
        [Buffer.from("c"), Buffer.from([1])],
        rng_program
      );
      console.log();

      const currentFeedsAccountInfo = await connection.getAccountInfo(
        current_feeds_account[0]
      );
      const currentFeedsAccountData = deserialize(
        CurrentFeedSchema,
        CurrentFeed,
        currentFeedsAccountInfo?.data!
      );

      const feedAccount1 = new PublicKey(
        bs58.encode(currentFeedsAccountData.account1).toString()
      );
      const feedAccount2 = new PublicKey(
        bs58.encode(currentFeedsAccountData.account2).toString()
      );
      const feedAccount3 = new PublicKey(
        bs58.encode(currentFeedsAccountData.account3).toString()
      );

      const fallbackAccount = new PublicKey(
        bs58.encode(currentFeedsAccountData.fallback_account).toString()
      );

      const tempKeypair = anchor.web3.Keypair.generate();

      const tx = await program.methods
        .getRandom()
        .accounts({
          data: dataPDA,
          signer: admin.publicKey,
          feedAccount1: feedAccount1,
          feedAccount2: feedAccount2,
          feedAccount3: feedAccount3,
          fallbackAccount: fallbackAccount,
          temp: tempKeypair.publicKey,
          rngProgram: rng_program,
          currentFeedsAccount: current_feeds_account[0],
        })
        .signers([admin, tempKeypair])
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log("error", error);
    }
  });
});
