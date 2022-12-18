import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID, NATIVE_MINT} from "@solana/spl-token";
// import { TOKEN_PROGRAM_ID, NATIVE_MINT, createMint, createAccount, mintTo, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
// env consts
const IS_DEVNET = false;
const LOCAL_KEYPAIR_FPATH = "/home/alphaprime8/.config/solana/id.json";
const PROGRAM_ID = '2omcykYnUGQW8tDGKZFMuJHAswrfMDAgMTkBo3Kd6Woj'; // can also load from file as done with localKeypair below



/*
tx = YsLbnCjWUQFXK4RWfaX73JWZm67ZkGYK59Zcey7fjxrBLLWUhSbuh9jojWspxLXHTxV61LqAY6RX9d4NWBym3nu
treasury = CQ7aUoeNfDxGGaohMcbe5XRXvVzcuL3xVA1cr2Z8jKbh
 */

const TREASURY_AUTH_PDA_SEED = "treasury_auth_pda_seed";

// program consts
async function initProgram() {
    // INIT Web3 Connection Objects
    const localKeypair = anchor.web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(require("fs").readFileSync(LOCAL_KEYPAIR_FPATH, {encoding: "utf-8",}))));
    const programId = new anchor.web3.PublicKey(PROGRAM_ID);
    let wallet = new anchor.Wallet(localKeypair);
    let opts = anchor.AnchorProvider.defaultOptions();
    const network = IS_DEVNET ? anchor.web3.clusterApiUrl('devnet') : anchor.web3.clusterApiUrl('mainnet-beta');
    let connection = new anchor.web3.Connection(network, opts.preflightCommitment);
    let provider = new anchor.AnchorProvider(connection, wallet, opts);
    let idl = await anchor.Program.fetchIdl(programId, provider);
    return new anchor.Program(idl, programId, provider);
}


async function initialize_treasury() {
    let program = await initProgram()

    let treasury = anchor.web3.Keypair.generate();
    let wsolVault = anchor.web3.Keypair.generate();
    let gigsVault = anchor.web3.Keypair.generate();
    let usdcVault = anchor.web3.Keypair.generate();

    let [treasuryAuthPda, _] = await anchor.web3.PublicKey.findProgramAddress(
        [treasury.publicKey.toBuffer(), Buffer.from(anchor.utils.bytes.utf8.encode(TREASURY_AUTH_PDA_SEED))],
        program.programId);

    let caleb_pubkey = new anchor.web3.PublicKey("4Ve95MJX83TLPfYvAfek3WCRberwtKqXwsKKqJJKdudo");

    let councillorsVec = [program.provider.publicKey, caleb_pubkey];

    let usdcMint = new anchor.web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    let gigsMint = new anchor.web3.PublicKey("9U8Bn6zAf6Wyp1YHdXtLyfbN7yMvdvW1qQY475iZ5ftZ");

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

    console.log("got tx: ", tx);
}

async function execute_withdraw() {
    let program = await initProgram()

    let treasury = new anchor.web3.PublicKey("CQ7aUoeNfDxGGaohMcbe5XRXvVzcuL3xVA1cr2Z8jKbh");
    let treasuryAuthPda = new anchor.web3.PublicKey("AANh7YvRPYCFbMrc6bK2dqw7VbtcLjFb2mZubA4RQfoM");
    let wsolVault = new anchor.web3.PublicKey("ALbmFM1JnfK5ufcsdvvj5Ap1ohZnZuvC7yHe9iVHrzni");
    let usdcVault = new anchor.web3.PublicKey("9DNedwJg9uge3dUz17hzxGdrt3rBt4wSzWsZdhonRrww");

    let receiverWsolAta = new anchor.web3.PublicKey("EDRiq6ekrJ26dD7tNsYgBGuPnKPp5CPgqUrMR9nyj4ev");
    let receiverUsdcAta = new anchor.web3.PublicKey("235ezs9WfEaArSGg3RK4Xo8NtVjZ6Q6TeJQYBsvNqwZh");

    let amount_usd = new anchor.BN(2);
    let withdraw_usdc = true;

    // @ts-ignore
    const tx = await program.methods.executeWithdrawal(amount_usd, withdraw_usdc)
        .accounts({
            signer: program.provider.publicKey,
            treasury: treasury,
            treasuryAuthPda: treasuryAuthPda,
            wsolVault: wsolVault,
            usdcVault: usdcVault,
            receiverWsolAta: receiverWsolAta,
            receiverUsdcAta: receiverUsdcAta,
            system_program: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .rpc();




    console.log("got tx: ", tx);
}






execute_withdraw()
    .then(()=>{
        console.log("done")
    })
