import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Helios } from "../target/types/helios";
import { assert } from "chai";

describe("HELIOS-L1-Solana-Registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Helios as Program<Helios>;

  it("Registers an AI Agent", async () => {
    const authority = provider.wallet;
    
    // Derived PDA
    const [agentPda, _] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("agent"), authority.publicKey.toBuffer()],
      program.programId
    );

    const agentName = "VORTEX";
    const agentVersion = "v3.0.0";

    console.log("📝 Registering Agent:", agentName);

    try {
        await program.methods
        .register_agent(agentName, agentVersion)
        .accounts({
            agentAccount: agentPda,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

        // Verification
        const account = await program.account.agentEntry.fetch(agentPda);
        
        console.log("✅ Agent Fetched:", account.name);
        console.log("Authority:", account.authority.toBase58());

        assert.ok(account.name === agentName);
        assert.ok(account.version === agentVersion);
        assert.ok(account.isActive === true);
        assert.ok(account.authority.equals(authority.publicKey));
        
    } catch (e) {
        console.error("Test Failed:", e);
        throw e;
    }
  });

  it("Updates Agent Status", async () => {
      // Test Logic implementation
  });
});
