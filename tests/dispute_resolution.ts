import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DisputeResolution } from "../target/types/dispute_resolution";

describe("dispute_resolution", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DisputeResolution as Program<DisputeResolution>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
