import { IDL } from "./../target/types/dispute_resolution";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DisputeResolution } from "../target/types/dispute_resolution";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("dispute_resolution", () => {
  const SETTINGS_PDA = Buffer.from(anchor.utils.bytes.utf8.encode("settings"));
  const COURT_TREASURY_PDA = Buffer.from(
    anchor.utils.bytes.utf8.encode("court_treasury_token_account")
  );
  const COURT_TREASURY_AUTHORITY_PDA = Buffer.from(
    anchor.utils.bytes.utf8.encode("court_treasury_authority")
  );
  const JUROR_PDA = Buffer.from(anchor.utils.bytes.utf8.encode("juror"));
  const METADATA_PDA = Buffer.from(anchor.utils.bytes.utf8.encode("metadata"));
  const EDITION_PDA = Buffer.from(anchor.utils.bytes.utf8.encode("edition"));
  const MINT = new anchor.web3.PublicKey(
    "CotjBMa7GVLUP6ajjDbCxoNZBAu9zfkLZzcU5wCLC2Hx"
  );
  const JUROR_CREATOR = new anchor.web3.PublicKey(
    "6UFFhicUhTdXFQDmdvdBhBxDtaTWtH5XAvJAiEe2sZKx"
  );
  const TOKEN_METADATA_PROGRAM = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .DisputeResolution as Program<DisputeResolution>;

  const wallet = anchor.Wallet.local();

  it("Initialize settings", async () => {
    const [settings, _settingsBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [SETTINGS_PDA],
        program.programId
      );

    const [courtTreasuryTokenAccount, _courtTreasuryTokenAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [COURT_TREASURY_PDA],
        program.programId
      );

    const tx = program.methods.initializeSettings(
      wallet.publicKey,
      JUROR_CREATOR,
      new anchor.BN(1000)
    );

    tx.accounts({
      settings,
      courtTreasuryTokenAccount,
      mint: MINT,
      admin: wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      systemProgram: anchor.web3.SystemProgram.programId,
    });

    await tx.rpc();

    console.log("Your transaction signature", tx);
  });

  it("Set settings", async () => {
    const [settings, _settingsBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [SETTINGS_PDA],
        program.programId
      );

    const settingsData = {
      masterAdmin: wallet.publicKey,
      admin: wallet.publicKey,
      jurorCreator: wallet.publicKey,
      raiseDisputeFee: new anchor.BN(12323),
    };

    const tx = program.methods.setSettings(settingsData);

    tx.accounts({
      settings,
      masterAdmin: wallet.publicKey,
    });

    await tx.rpc();

    console.log("Your transaction signature", tx);
  });

  it("Raise dispute", async () => {
    const dispute = anchor.web3.Keypair.generate();

    const [payerTokenAccount, _payerTokenAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          wallet.publicKey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          MINT.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

    const [settings, _settingsBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [SETTINGS_PDA],
        program.programId
      );

    const [courtTreasuryTokenAccount, _courtTreasuryTokenAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [COURT_TREASURY_PDA],
        program.programId
      );

    const disputeData = {
      disputeValue: new anchor.BN(10000),
      requiredStakeAmount: new anchor.BN(5000),
      joinJurorDeadline: new anchor.BN(1662395731),
      drawJurorDeadline: new anchor.BN(1662396731),
      closureDeadline: new anchor.BN(1662397731),
      applicants: [
        {
          address: wallet.publicKey,
          share: 100,
          evidenceUri: "sad",
          fingerprint: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
          ],
        },
      ],
      respondents: [
        {
          address: wallet.publicKey,
          share: 100,
          evidenceUri: "sad",
          fingerprint: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
          ],
        },
      ],
    };

    const tx = program.methods.raiseDispute(disputeData);

    tx.accounts({
      dispute: dispute.publicKey,
      payer: wallet.publicKey,
      payerTokenAccount,
      settings,
      courtTreasuryTokenAccount,
      mint: MINT,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    });

    tx.signers([dispute]);

    await tx.rpc();

    console.log("Your transaction signature", tx);
  });

  it("Join party", async () => {
    let disputes = await program.account.dispute.all();

    const tx = program.methods.joinParty(
      "my evidence2",
      [
        1, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
      ]
    );

    tx.accounts({
      dispute: disputes[0].publicKey,
      payer: wallet.publicKey,
    });

    await tx.rpc();

    console.log("Your transaction signature", tx);
  });

  it("Approve dispute", async () => {
    let disputes = await program.account.dispute.all();

    const [settings, _settingsBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [SETTINGS_PDA],
        program.programId
      );

    const tx = program.methods.approveDispute(
      new anchor.BN(10000),
      new anchor.BN(10000)
    );

    tx.accounts({
      dispute: disputes[0].publicKey,
      payer: wallet.publicKey,
      settings,
    });

    await tx.rpc();

    console.log("Your transaction signature", tx);
  });

  it("Join juror", async () => {
    let disputes = await program.account.dispute.all();

    const [jurorReservationEntry, _jurorReservationEntryBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          JUROR_PDA,
          disputes[0].publicKey.toBuffer(),
          Buffer.from(
            anchor.utils.bytes.utf8.encode(
              disputes[0].account.readyJurors.toString()
            )
          ),
        ],
        program.programId
      );

    const jurorNftMint = new anchor.web3.PublicKey(
      "6LPrB8YiNGwaNKG9XPKJf5iyroUMcx2iK4YMPy2fLchd"
    );

    const [jurorNftTokenAccount, _jurorNftTokenAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          wallet.publicKey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          jurorNftMint.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

    const [jurorNftMetadataAccount, _jurorNftMetadataAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          METADATA_PDA,
          TOKEN_METADATA_PROGRAM.toBuffer(),
          jurorNftMint.toBuffer(),
        ],
        TOKEN_METADATA_PROGRAM
      );

    const [jurorNftMasterEditionAccount, _jurorNftMasterEditionAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          METADATA_PDA,
          TOKEN_METADATA_PROGRAM.toBuffer(),
          jurorNftMint.toBuffer(),
          EDITION_PDA,
        ],
        TOKEN_METADATA_PROGRAM
      );

    const [jurorTokenAccount, _jurorTokenAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          wallet.publicKey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          MINT.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

    const [settings, _settingsBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [SETTINGS_PDA],
        program.programId
      );

    const [courtTreasuryTokenAccount, _courtTreasuryTokenAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [COURT_TREASURY_PDA],
        program.programId
      );

    const tx = program.methods.joinJuror();

    tx.accounts({
      jurorReservationEntry,
      dispute: disputes[0].publicKey,
      juror: wallet.publicKey,
      jurorNftMint,
      jurorNftTokenAccount,
      jurorNftMetadataAccount,
      jurorNftMasterEditionAccount,
      jurorTokenAccount,
      settings,
      courtTreasuryTokenAccount,
      mint: MINT,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM,
    });

    await tx.rpc();

    console.log("Your transaction signature", tx);
  });

  it("Draw juror", async () => {
    let disputes = await program.account.dispute.all();

    const [settings, _settingsBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [SETTINGS_PDA],
        program.programId
      );

    const jurors = [wallet.publicKey]; // TODO change

    const tx = program.methods.drawJurors(jurors);

    tx.accounts({
      dispute: disputes[0].publicKey,
      payer: wallet.publicKey,
      settings,
    });

    await tx.rpc();

    // console.log("Your transaction signature", tx);
  });

  it("Cast vote", async () => {
    let disputes = await program.account.dispute.all();

    const [jurorReservationEntry, _jurorReservationEntryBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          JUROR_PDA,
          disputes[0].publicKey.toBuffer(),
          Buffer.from(
            anchor.utils.bytes.utf8.encode(
              (disputes[0].account.readyJurors - 1).toString()
            )
          ),
        ],
        program.programId
      );

    const jurors = [TOKEN_PROGRAM_ID]; // TODO change

    const tx = program.methods.castVote(1, { respondent: {} });

    tx.accounts({
      jurorReservationEntry,
      dispute: disputes[0].publicKey,
      payer: wallet.publicKey,
    });

    await tx.rpc();

    console.log("Your transaction signature", tx);
  });

  it.only("Claim stake", async () => {
    let disputes = await program.account.dispute.all();

    const [jurorReservationEntry, _jurorReservationEntryBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          JUROR_PDA,
          disputes[0].publicKey.toBuffer(),
          Buffer.from(
            anchor.utils.bytes.utf8.encode(
              (disputes[0].account.readyJurors - 1).toString()
            )
          ),
        ],
        program.programId
      );

    const [jurorTokenAccount, _jurorTokenAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          wallet.publicKey.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          MINT.toBuffer(),
        ],
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

    const [settings, _settingsBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [SETTINGS_PDA],
        program.programId
      );

    const [courtTreasuryTokenAccount, _courtTreasuryTokenAccountBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [COURT_TREASURY_PDA],
        program.programId
      );

    const [courtTreasuryAuthority, _courtTreasuryAuthorityBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [COURT_TREASURY_AUTHORITY_PDA],
        program.programId
      );

    const tx = program.methods.claimStake(1);

    tx.accounts({
      jurorReservationEntry,
      dispute: disputes[0].publicKey,
      juror: wallet.publicKey,
      jurorTokenAccount,
      settings,
      courtTreasuryTokenAccount,
      treasuryAuthority: courtTreasuryAuthority,
      mint: MINT,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    });

    await tx.rpc();

    console.log("Your transaction signature", tx);
  });
});
