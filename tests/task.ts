import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Farming } from "../target/types/farming";
import {
  closeAccountInstructionData,
  TOKEN_PROGRAM_ID,
  transfer,
} from "@solana/spl-token";
import {
  Account,
  createMint,
  getMint,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import fs from "fs";
import { Keypair } from "@solana/web3.js";

import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
// import { TokenHelper } from "./token_helper";
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import { BN } from "bn.js";
import { assert, expect } from "chai";
import { publicKey } from "@project-serum/anchor/dist/cjs/utils";
const connection = new anchor.web3.Connection(
  anchor.web3.clusterApiUrl("devnet")
);

anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Farming as Program<Farming>;

//const connection = anchor.getProvider().connection;
const finoMintAddress = new anchor.web3.PublicKey(
  "54qAckkqAqr2qCVup8SYgxkUyPvx1q8yuyaHPvXY7tAc"
);
const rinoMintAddress = new anchor.web3.PublicKey(
  "GM8YLBaVraVrwmbuEHpVPYGxkVVtXvq2DdHrZUvpX6VY"
);
const lpMint = "MNLj3omEgCVysqQyiAu2LcUPTzbssjQAGSSvw5rEJk4";

describe("Farming", () => {
  // Configure the client to use the local cluster.

  const user5 = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array([
      34, 190, 48, 111, 97, 113, 83, 136, 188, 13, 70, 81, 53, 181, 218, 57,
      168, 214, 4, 120, 233, 81, 7, 231, 159, 7, 35, 60, 15, 172, 36, 33, 109,
      172, 88, 169, 223, 83, 74, 187, 20, 220, 71, 102, 178, 161, 60, 199, 251,
      203, 28, 9, 206, 104, 107, 252, 45, 211, 127, 132, 0, 123, 151, 232,
    ])
  );
  //anchor.setProvider(provider);

  // it("Create Reward Mint ", async () => {
  //   const rewardData = JSON.parse(
  //     fs.readFileSync("./.keys/reward_mint.json", {
  //       encoding: "utf8",
  //       flag: "r",
  //     })
  //   );
  //   const rewardMintKeypair = Keypair.fromSecretKey(new Uint8Array(rewardData));
  //   const rewardMintAddress = rewardMintKeypair.publicKey;
  //   const [rewardPDA, rewardPDABump] = await PublicKey.findProgramAddress(
  //     [rewardMintAddress.toBuffer()],
  //     program.programId
  //   );

  //   const rewardMint = await createMintAcct(
  //     rewardMintKeypair,
  //     rewardPDA,
  //     user5
  //   );

  //   console.log(`Reward's  Mint Address: ${rewardMint}`);
  // });

  it("It creates the program LP💰 token bag", async () => {
    const [lpBagPDA, bagBump] = await PublicKey.findProgramAddress(
      [new anchor.web3.PublicKey(lpMint).toBuffer()],
      program.programId
    );

    // await program.methods
    //   .createLpTokenBag(new anchor.web3.PublicKey(lpMint))
    //   .accounts({
    //     lpMint: new anchor.web3.PublicKey(lpMint),
    //     programLpTokenBag: lpBagPDA,
    //     payer: user5.publicKey,
    //     systemProgram: SystemProgram.programId,
    //     tokenProgram: TOKEN_PROGRAM_ID,
    //     rent: SYSVAR_RENT_PUBKEY,
    //   })
    //   .signers([user5])
    //   .rpc();
    console.log("Program Account address is  ", lpBagPDA);
  });

  // it("intialize user Lp mint profile ", async () => {
  //   const [userProfilePDA] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       utf8.encode("FarmingProfile"),
  //       new anchor.web3.PublicKey(lpMint).toBuffer(),
  //       user5.publicKey.toBuffer(),
  //     ],
  //     program.programId
  //   );
  //   console.log("The stake Account pda is ", userProfilePDA);
  //   await program.methods
  //     .intializeUserProfile(new anchor.web3.PublicKey(lpMint))
  //     .accounts({
  //       user: user5.publicKey,
  //       userFarmingProfile: userProfilePDA,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       lpMint: new anchor.web3.PublicKey(lpMint),
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([user5])
  //     .rpc();
  // });
  // it("Stake LP !", async () => {
  //   const [userProfilePDA, userProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         utf8.encode("FarmingProfile"),
  //         new anchor.web3.PublicKey(lpMint).toBuffer(),
  //         user5.publicKey.toBuffer(),
  //       ],
  //       program.programId
  //     );
  //   console.log(userProfilePDA);
  //   const fetchedStakingAccount =
  //     await program.account.userLpStakingProfile.fetch(userProfilePDA);
  //   let stakedAccounts = fetchedStakingAccount.stakedAccounts;
  //   stakedAccounts += 1;
  //   const index = stakedAccounts.toString();
  //   console.log("Stake index is ", index);
  //   const [LpStakeTransactionPDA] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         utf8.encode("LpStakeTransaction"),
  //         userProfilePDA.toBuffer(),
  //         utf8.encode(index),
  //       ],
  //       program.programId
  //     );
  //   console.log("stakePDA", LpStakeTransactionPDA.toBase58());
  //   const userLpAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     user5,
  //     new anchor.web3.PublicKey(lpMint),
  //     user5.publicKey,
  //     false
  //   );
  //   const [programLpPDA, programPDAbump] = await PublicKey.findProgramAddress(
  //     [new anchor.web3.PublicKey(lpMint).toBuffer()],
  //     program.programId
  //   );
  //   await program.methods
  //     .stakeLp(
  //       programPDAbump,
  //       index,
  //       new anchor.web3.PublicKey(lpMint),
  //       userProfilePDA,
  //       userProfileBump,
  //       new anchor.BN(5.0)
  //     )
  //     .accounts({
  //       user: user5.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       lpStakeTransaction: LpStakeTransactionPDA,
  //       userProfile: userProfilePDA,
  //       userLpTokenBag: userLpAccount.address,
  //       userLpTokenBagAuthority: user5.publicKey,
  //       programLpTokenBag: programLpPDA,
  //       lpMint: new anchor.web3.PublicKey(lpMint),
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //     })
  //     .signers([user5])
  //     .rpc();
  // });

  // it("It Harvest ", async () => {
  //   const index = "1"; // change according to stake index
  //   const userRinoAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     user5,
  //     rinoMintAddress,
  //     user5.publicKey,
  //     false
  //   );
  //   const [userProfilePDA, userProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         utf8.encode("FarmingProfile"),
  //         new anchor.web3.PublicKey(lpMint).toBuffer(),
  //         user5.publicKey.toBuffer(),
  //       ],
  //       program.programId
  //     );

  //   const [lpStakeTransactionPDA, lpStakeTransactionPDABump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         utf8.encode("LpStakeTransaction"),
  //         userProfilePDA.toBuffer(),
  //         utf8.encode(index),
  //       ],
  //       program.programId
  //     );
  //   const [rinoPDA, rinoPDABump] = await PublicKey.findProgramAddress(
  //     [rinoMintAddress.toBuffer()],
  //     program.programId
  //   );
  //   await program.methods
  //     .harvest(
  //       rinoPDABump,
  //       new anchor.web3.PublicKey(lpMint),
  //       userProfileBump,
  //       lpStakeTransactionPDABump,
  //       index
  //     )
  //     .accounts({
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       lpStakeTransaction: lpStakeTransactionPDA,
  //       userProfile: userProfilePDA,
  //       lpMint: lpMint,
  //       rinoMint: rinoMintAddress,
  //       rinoMintAuthority: rinoPDA,
  //       userRinoTokenBag: userRinoAccount.address,
  //       user: user5.publicKey,
  //     })
  //     .signers([user5])
  //     .rpc();
  // });

  // it("It Withdraw Lp Token ", async () => {
  //   const user = user5;
  //   const index = "1"; //stake index
  //   const userLpAccount = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     user,
  //     new anchor.web3.PublicKey(lpMint),
  //     user.publicKey,
  //     false
  //   );

  //   const [userProfilePDA, userProfileBump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         utf8.encode("FarmingProfile"),
  //         new anchor.web3.PublicKey(lpMint).toBuffer(),
  //         user5.publicKey.toBuffer(),
  //       ],
  //       program.programId
  //     );

  //   const [lpStakeTransactionPDA, lpStakeTransactionPDABump] =
  //     await anchor.web3.PublicKey.findProgramAddress(
  //       [
  //         utf8.encode("LpStakeTransaction"),
  //         userProfilePDA.toBuffer(),
  //         utf8.encode(index),
  //       ],
  //       program.programId
  //     );

  //   const [rinoPDA, rinoPDABump] = await PublicKey.findProgramAddress(
  //     [rinoMintAddress.toBuffer()],
  //     program.programId
  //   );
  //   const [programlpBagPDA, programlpBagBump] =
  //     await PublicKey.findProgramAddress(
  //       [new anchor.web3.PublicKey(lpMint).toBuffer()],
  //       program.programId
  //     );
  //   await program.methods
  //     .withdraw(
  //       programlpBagBump,
  //       rinoPDABump,
  //       userProfileBump,
  //       lpStakeTransactionPDABump,
  //       new anchor.web3.PublicKey(lpMint),
  //       index
  //     )
  //     .accounts({
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       lpStakeTransaction: lpStakeTransactionPDA,
  //       userProfile: userProfilePDA,
  //       lpMint: new anchor.web3.PublicKey(lpMint),
  //       programLpTokenBag: programlpBagPDA,
  //       userLpTokenBag: userLpAccount.address,
  //       user: user5.publicKey,
  //     })
  //     .signers([user5])
  //     .rpc();
  // });
});

const getProgramLpTokenBagPDA = async (seed): Promise<[PublicKey, number]> => {
  seed;

  return await PublicKey.findProgramAddress([seed], program.programId);
};

const createMintAcct = async (
  keypairToAssign: Keypair,
  authorityToAssign: PublicKey,
  payer: Keypair
): Promise<PublicKey> => {
  return await createMint(
    connection,
    payer,
    authorityToAssign, // mint authority
    null, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
    8, // decimals
    keypairToAssign // address of the mint
  );
};
