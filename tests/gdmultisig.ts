import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Gdmultisig } from "../target/types/gdmultisig";
import { TOKEN_PROGRAM_ID, NATIVE_MINT, createMint, createAccount, mintTo, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";


// consts
const TREASURY_AUTH_PDA_SEED = "treasury_auth_pda_seed";


// utils
function to_lamports(num_sol) {
    return Math.round(num_sol * 1e9);
}

// globals
let receiverUsdcAta;
let owner1;
let usdcMint;
let treasury;
let wsolVault;
let gigsVault;
let usdcVault;
let treasuryAuthPda;


describe("gdmultisig", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Gdmultisig as Program<Gdmultisig>;

    it("Is initialized!", async () => {

        // load up payers
        owner1 = anchor.web3.Keypair.generate();
        await program.provider.connection.confirmTransaction(
            await program.provider.connection.requestAirdrop(owner1.publicKey, to_lamports(1000)),
            "confirmed"
        );
        await program.provider.connection.confirmTransaction(
            await program.provider.connection.requestAirdrop(program.provider.publicKey, to_lamports(10000)),
            "confirmed"
        );

        // create mints
        usdcMint = await createMint(
            program.provider.connection,
            owner1,
            owner1.publicKey,
            null,
            6,
        );

        receiverUsdcAta = await createAccount(
            program.provider.connection,
            owner1,
            usdcMint,
            program.provider.publicKey,
        );

        let gigsMint = await createMint(
            program.provider.connection,
            owner1,
            owner1.publicKey,
            null,
            0,
        );

        treasury = anchor.web3.Keypair.generate();
        wsolVault = anchor.web3.Keypair.generate();
        gigsVault = anchor.web3.Keypair.generate();
        usdcVault = anchor.web3.Keypair.generate();

        let [_treasuryAuthPda, _] = await anchor.web3.PublicKey.findProgramAddress(
            [treasury.publicKey.toBuffer(), Buffer.from(anchor.utils.bytes.utf8.encode(TREASURY_AUTH_PDA_SEED))],
            program.programId);
        treasuryAuthPda = _treasuryAuthPda;

        let councillorsVec = [program.provider.publicKey, owner1.publicKey];

        // @ts-ignore
        const tx = await program.methods.initializeTreasury(councillorsVec)
            .accounts({
                signer: program.provider.publicKey,
                treasury: treasury.publicKey,
                treasuryAuthPda: treasuryAuthPda,
                wsolMint: NATIVE_MINT,
                wsolVault: wsolVault.publicKey,
                usdcMint: usdcMint,
                usdcVault: usdcVault.publicKey,
                gigsMint: gigsMint,
                gigsVault: gigsVault.publicKey,
                system_program: anchor.web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([treasury, wsolVault, usdcVault, gigsVault])
            .rpc();
        console.log("Your transaction signature", tx);
    });

    it("Execute Withdraw", async () => {

        let vault_amount = Math.round(1000 * 1e6)

        let result = await mintTo(
            program.provider.connection,
            owner1,
            usdcMint,
            usdcVault.publicKey,
            owner1,
            vault_amount,
        );
        console.log("minted usdc to vault: ", result);

        let receiverWsolAta = await getOrCreateAssociatedTokenAccount(program.provider.connection, owner1, NATIVE_MINT, program.provider.publicKey);
        console.log("got wsol ata: ", receiverWsolAta.address.toString());

        let amount_usd = new anchor.BN(500);
        let withdraw_usdc = true;

        // @ts-ignore
        const tx = await program.methods.executeWithdrawal(amount_usd, withdraw_usdc)
            .accounts({
                signer: program.provider.publicKey,
                treasury: treasury.publicKey,
                treasuryAuthPda: treasuryAuthPda,
                wsolVault: wsolVault.publicKey,
                usdcVault: usdcVault.publicKey,
                receiverWsolAta: receiverWsolAta.address,
                receiverUsdcAta: receiverUsdcAta,
                system_program: anchor.web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .rpc();
        console.log("Your transaction signature", tx);
    });





});
