# Solana Farming 


## Get Started

Clone git repo 📦
```bash
git clone <link>
```

### Connect wallet for deployment
Navigate 👉 Anchor.toml file in root 

Replace your Solana wallet address path 👇
```rust
[provider]
cluster = "devnet" // DEVNET
wallet = "<home/path/WalletAddress>"
```
#### Generate your Solana wallet if you do not own it

In terminal excute below command 🔳
```bash
solana-keygen new
```

### Deployment

```bash
anchor build 
anchor deploy

```
Copy the Program ID from the terminal 🔳

Navigate to Anchot.toml file aviable in root

Replace the program ID 👇
```bash
[programs.devnet]
farming = "<PROGRAM_ID>"

```
Navigate to lib.rs file in program/src/lib.rs

Replace the program ID 👇  declare_id in lib.rs file 

```bash

use std::mem::size_of;
declare_id!("<PROGRAM_ID>");


```
## All Set 🚀

## Testing


Navigate 👉  program/tests/task.ts

This will be our testing file

**You will notice all testing functions are commented. For better understanding, we will uncomment each  one by one, and execute them. 😎** 

Uncomment Create Reward Mint function  

For Reward token we need to genrate a mint key pair.

Excute below command in 🔳
```bash
solana-keygen new --outfile .keys/reward_mint.json --force
```
**Response**
```bash
Generating a new keypair

For added security, enter a BIP39 passphrase

NOTE! This passphrase improves security of the recovery seed phrase NOT the
keypair file itself, which is stored as insecure plain text

BIP39 Passphrase (empty for none): 

Wrote new keypair to .keys/reward_mint.json
========================================================================
pubkey: GM8YLBaVraVrwmbuEHpVPYGxkVVtXvq2DdHrZUvpX6VY
========================================================================
Save this seed phrase and your BIP39 passphrase to recover your new keypair:
XXXX XXXXX XXX XX XXX XXXXX XXXx XXXX XXXXX XXXX XXXX
========================================================================
```
Copy the 📄 pubkey 

We will call reward token as **RINO** 💰

Navigate to lib.rs available in program/src/lib.rs.

Replace RINO_MINT_ADDRESS with copied pubkey 
```bash
pub const RINO_MINT_ADDRESS: &str="GM8YLBaVraVrwmbuEHpVPYGxkVVtXvq2DdHrZUvpX6VY";
```
**Also we need to replace in our testing file task.ts**

Replace rinoMintAddress with copied pubkey 
```bash
const rinoMintAddress = new anchor.web3.PublicKey(
  "GM8YLBaVraVrwmbuEHpVPYGxkVVtXvq2DdHrZUvpX6VY"
);
```
**Great 😃**

Lets move towards our testing file task.ts  program/testing/task.ts


Remember, we earlier uncommented the **'Create Reward Mint'** function in the testing file. It's time  ⌛ to execute it. 


We are generating SPL token and assigning the contract as the minting authority.
```ts
  it("Create Reward Mint ", async () => {
    const rewardData = JSON.parse(
      fs.readFileSync("./.keys/reward_mint.json", {
        encoding: "utf8",
        flag: "r",
      })
    );
    const rewardMintKeypair = Keypair.fromSecretKey(new Uint8Array(rewardData));
    const rewardMintAddress = rewardMintKeypair.publicKey;
    const [rewardPDA, rewardPDABump] = await PublicKey.findProgramAddress(
      [rewardMintAddress.toBuffer()],
      program.programId
    );

    const rewardMint = await createMintAcct(
      rewardMintKeypair,
      rewardPDA,
      user5
    );

    console.log(`Reward's  Mint Address: ${rewardMint}`);
  });

```
**Note: I have imported a private key for a dummy wallet in 'user5' as mentioned in the testing. You can create your own dummy wallet and replace it. Have some Solana Devnet tokens in it**

Excute in terminal 🔳
```bash 
anchor test
```
**Response**

```cmd
 Farming
Reward's  Mint Address: GM8YLBaVraVrwmbuEHpVPYGxkVVtXvq2DdHrZUvpX6VY
    ✔ Create Reward Mint  (2902ms)


  1 passing (3s)

Done in 4.89s.
```
Go head to solana explorer with mint address to verify SPL token. 😎

**Now comment the create reward mint function**

Lets move 🔛 to our next function

uncomment function "It creates the program LP💰 token bag"

We need to deposit our LP(Liquidity Pool) tokens to program. For program to hold these LP token it needs a bag/account.We will be creating it in below function. 

**Replace your Lp mint**

```ts
const lpMint = "MNLj3omEgCVysqQyiAu2LcUPTzbssjQAGSSvw5rEJk4";//Replace your with lp mint

```
```ts
  it("It creates the program LP💰 token bag", async () => {
    const [lpBagPDA, bagBump] = await PublicKey.findProgramAddress(
      [new anchor.web3.PublicKey(lpMint).toBuffer()],
      program.programId
    );

    await program.methods
      .createLpTokenBag(new anchor.web3.PublicKey(lpMint))
      .accounts({
        lpMint: new anchor.web3.PublicKey(lpMint),
        programLpTokenBag: lpBagPDA,
        payer: user5.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      })
      .signers([user5])
      .rpc();
    console.log("Program Account address is  ", lpBagPDA);
  });
```

Excute in terminal 🔳
```bash 
anchor test
```
**Response**
```bash
 Farming
    ✔ It creates the program LP💰 token bag (4329ms)
  1 passing (4s)

Done in 6.11s.

```

**Great 😃**
Lets move 🔛 to our next function

Make sure to comment eariler function 

**Creat User Lp Mint Profile Function**

For storing user Lp token deposit record

Go ahead uncomment the Creat User Lp Mint Profile Function
```ts
  it("intialize user Lp mint profile ", async () => {
    const [userProfilePDA] = await anchor.web3.PublicKey.findProgramAddress(
      [
        utf8.encode("FarmingProfile"),
        new anchor.web3.PublicKey(lpMint).toBuffer(),
        user5.publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log("The stake Account pda is ", userProfilePDA);
    await program.methods
      .intializeUserProfile(new anchor.web3.PublicKey(lpMint))
      .accounts({
        user: user5.publicKey,
        userFarmingProfile: userProfilePDA,
        tokenProgram: TOKEN_PROGRAM_ID,
        lpMint: new anchor.web3.PublicKey(lpMint),
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user5])
      .rpc();
  });
```
Excute in terminal 🔳
```bash 
anchor test
```
**Response**
```bash
  Farming
The stake Account pda is  PublicKey [PublicKey(Fv9t16veJgpDZBzh5v85yeEvBRNGXpBH6AV99wxqG5AW)] {
  _bn: <BN: dda170939e046231ea2642adf0b5e5d2347cb846b93d56143a536c69890128ef>
}
{
  totalStakedAmount: 0,
  stakedAccounts: 0,
  mint: PublicKey [PublicKey(MNLj3omEgCVysqQyiAu2LcUPTzbssjQAGSSvw5rEJk4)] {
    _bn: <BN: 537bed8a714811b4fdfb33ea17f2d2056c56355fec1c91f99638b366379c8bd>
  }
}
    ✔ intialize user Lp mint profile  (2276ms)

  1 passing (2s)

```
Make sure it display your Lp mint in mint PublicKey

Make sure to comment the code before moving next

**Great 😃**
Lets move 🔛 to our next function

Lets Stake Our LP Tokens  ✨

Uncomment the Stake LP function

```ts
  it("Stake LP !", async () => {
    const [userProfilePDA, userProfileBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("FarmingProfile"),
          new anchor.web3.PublicKey(lpMint).toBuffer(),
          user5.publicKey.toBuffer(),
        ],
        program.programId
      );
    console.log(userProfilePDA);
    const fetchedStakingAccount =
      await program.account.userLpStakingProfile.fetch(userProfilePDA);
    let stakedAccounts = fetchedStakingAccount.stakedAccounts;
    stakedAccounts += 1;
    const index = stakedAccounts.toString();
    console.log("Stake index is ", index);
    const [LpStakeTransactionPDA] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("LpStakeTransaction"),
          userProfilePDA.toBuffer(),
          utf8.encode(index),
        ],
        program.programId
      );
    console.log("stakePDA", LpStakeTransactionPDA.toBase58());
    const userLpAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user5,
      new anchor.web3.PublicKey(lpMint),
      user5.publicKey,
      false
    );
    const [programLpPDA, programPDAbump] = await PublicKey.findProgramAddress(
      [new anchor.web3.PublicKey(lpMint).toBuffer()],
      program.programId
    );
    await program.methods
      .stakeLp(
        programPDAbump,
        index,
        new anchor.web3.PublicKey(lpMint),
        userProfilePDA,
        userProfileBump,
        new anchor.BN(5.0)
      )
      .accounts({
        user: user5.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        lpStakeTransaction: LpStakeTransactionPDA,
        userProfile: userProfilePDA,
        userLpTokenBag: userLpAccount.address,
        userLpTokenBagAuthority: user5.publicKey,
        programLpTokenBag: programLpPDA,
        lpMint: new anchor.web3.PublicKey(lpMint),
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user5])
      .rpc();
  });
```
Excute in terminal 🔳
```bash 
anchor test
```
**Response**
```bash
  Farming
PublicKey [PublicKey(GdYTTYJnX3b5iWZGjV6mkBrhBTdzpMYGepUydRABXpT5)] {
  _bn: <BN: e83bc7d6cd8c45dcbcc0a4ae6561217b14d0adaf0a3116e0b7f86e1e3d2ebf74>
}
Stake index is  1
stakePDA FixxMt9mhdb9DaPnLRk3ZNkSyEt3qDbcMbR89apcTqLN
    ✔ Stake LP ! (8185ms)


  1 passing (8s)

Done in 9.88s.
```
Make sure to comment the code before moving next

**Great 😃 We have staked our Lp**

**Lets Harvest our 🏆 Reward**    💸💸💸

Currently reward is calulated on based upon staked time and staked amount

Uncomment Harvest function in testing file ... in task.ts program/tests/task.ts

```ts
  it("It Harvest ", async () => {
    const index = "1"; // change according to stake index
    const userRinoAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user5,
      rinoMintAddress,
      user5.publicKey,
      false
    );
    const [userProfilePDA, userProfileBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("FarmingProfile"),
          new anchor.web3.PublicKey(lpMint).toBuffer(),
          user5.publicKey.toBuffer(),
        ],
        program.programId
      );

    const [lpStakeTransactionPDA, lpStakeTransactionPDABump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("LpStakeTransaction"),
          userProfilePDA.toBuffer(),
          utf8.encode(index),
        ],
        program.programId
      );
    const [rinoPDA, rinoPDABump] = await PublicKey.findProgramAddress(
      [rinoMintAddress.toBuffer()],
      program.programId
    );
    await program.methods
      .harvest(
        rinoPDABump,
        new anchor.web3.PublicKey(lpMint),
        userProfileBump,
        lpStakeTransactionPDABump,
        index
      )
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
        lpStakeTransaction: lpStakeTransactionPDA,
        userProfile: userProfilePDA,
        lpMint: lpMint,
        rinoMint: rinoMintAddress,
        rinoMintAuthority: rinoPDA,
        userRinoTokenBag: userRinoAccount.address,
        user: user5.publicKey,
      })
      .signers([user5])
      .rpc();
  });

```
Excute in terminal  🔳
```bash 
anchor test
```
**Response**
```bash

  Farming
    ✔ It Harvest  (3674ms)


  1 passing (4s)

Done in 5.76s.
```
Go ahead to solana explorer view token against your wallet (in my case it's user5 wallet )

Reward should be refecting in your account 🎉 💸💸💸 🎉

Make sure to comment the code before moving next

**Great 😃 We Harvested our 🏆 Reward**    💸💸💸

**Lets withdraw our staked Amount ↪️ 💰** 

Go head to task.ts file and uncomment the withdraw function

```ts
it("It Withdraw Lp Token ", async () => {
    const user = user5;
    const index = "1"; //stake index
    const userLpAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      user,
      new anchor.web3.PublicKey(lpMint),
      user.publicKey,
      false
    );

    const [userProfilePDA, userProfileBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("FarmingProfilee"),
          new anchor.web3.PublicKey(lpMint).toBuffer(),
          user5.publicKey.toBuffer(),
        ],
        program.programId
      );

    const [lpStakeTransactionPDA, lpStakeTransactionPDABump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [
          utf8.encode("LpStakeTransaction"),
          userProfilePDA.toBuffer(),
          utf8.encode(index),
        ],
        program.programId
      );

    const [rinoPDA, rinoPDABump] = await PublicKey.findProgramAddress(
      [rinoMintAddress.toBuffer()],
      program.programId
    );
    const [programlpBagPDA, programlpBagBump] =
      await PublicKey.findProgramAddress(
        [new anchor.web3.PublicKey(lpMint).toBuffer()],
        program.programId
      );
    await program.methods
      .withdraw(
        programlpBagBump,
        rinoPDABump,
        userProfileBump,
        lpStakeTransactionPDABump,
        new anchor.web3.PublicKey(lpMint),
        index
      )
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
        lpStakeTransaction: lpStakeTransactionPDA,
        userProfile: userProfilePDA,
        lpMint: new anchor.web3.PublicKey(lpMint),
        programLpTokenBag: programlpBagPDA,
        userLpTokenBag: userLpAccount.address,
        user: user5.publicKey,
      })
      .signers([user5])
      .rpc();
  });
```

Excute in terminal 🔳
```bash 
anchor test
```
**Response**
```bash
  Farming
    ✔ It Withdraw Lp Token  (2054ms)


  1 passing (2s)

```


Go ahead to solana explorer view token against your wallet (in my case it's user5 wallet )

Withdraw should be refecting in your account 😎  🎉 🎉 

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[LIT]