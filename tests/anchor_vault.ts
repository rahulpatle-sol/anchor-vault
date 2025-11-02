const anchor = require("@coral-xyz/anchor");

describe("anchor_vault", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.AnchorVault;

  it("Initializes vault", async () => {
    const [vaultStatePda, stateBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault_state"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const [vaultPda, vaultBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods.initialize().accounts({
      user: provider.wallet.publicKey,
      vaultState: vaultStatePda,
      vault: vaultPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    console.log("Vault initialized:", tx);
  });
});
