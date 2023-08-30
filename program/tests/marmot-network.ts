import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MarmotNetwork } from "../target/types/marmot_network";

describe("marmot-network", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MarmotNetwork as Program<MarmotNetwork>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
