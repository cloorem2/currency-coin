import * as anchor from "@project-serum/anchor";
import { CurrencyCoin } from "../target/types/currency_coin";

const sleep = require("sleep");
const execSync = require('child_process').execSync;

// const provider = anchor.AnchorProvider.local("http://127.0.0.1:8899");
const provider = anchor.AnchorProvider.env();
// provider.opts.commitment = 'confirmed';
anchor.setProvider(provider);
console.log('commitment ' + provider.opts.commitment);
const payer = provider.wallet as anchor.Wallet;
const program = anchor.workspace.CurrencyCoin as anchor.Program<CurrencyCoin>;

const ccKey = "5jGmQSUMvQ5dKrLXuh3oUjczWWLefUEQHX3EGKXnLjjS";
const b0Key = "GVWmAjaond6PrChG4bPZRWRbQAs2gwFbHE5t3NfUhbft";
const b1Key = "7c72g38GCvGyNqBL7zEdQS7nHq9DfiRAUQcRiSDH5FDx";
const s0Key = "EBj4FTsN2BHbNtbctvzCX96JS3ksB8z1ukWcZQjqCujr";

var mintAuth, mintAuthBump;
var ccMint, ccMintBump;
var ccb0Mint, ccb0MintBump;
var ccb1Mint, ccb1MintBump;
var ccs0Mint, ccs0MintBump;
var cc_ata,ccb0_ata,ccb1_ata,ccs0_ata;
var owner_cc_ata,owner_ccb0_ata,owner_ccb1_ata,owner_ccs0_ata;
var state;

let balances = {};
let pbalances = {};
let crank_count = 0;
let buyAm = 0;
async function start() {
  await sleep.sleep(10);
  await init();
  await getBalances();
  state = await program.account.mintAuth.fetch(mintAuth);
  print_state();
  while (true) {
    state = await program.account.mintAuth.fetch(mintAuth);
    if (state.maturityState == 0) {
      if (balances[b1Key] > 0) {
        await redeem1();
        await getBalances();
        buyAm = balances[ccKey];
        // await buy_bonds0();
        // await getBalances();
        /*
        var ima = state.ima0*60*60*24*365;
        if (ima < 5) {
          await sell_bonds0();
          await getBalances();
        }
        if (ima > 50.0) {
          buyAm = balances[ccKey] / 100;
          if (ima > 60.0) { buyAm = balances[ccKey] / 10; }
          if (ima > 70.0) { buyAm = balances[ccKey]; }
          await buy_bonds0();
          await getBalances();
        }
        */
      }
      await crank0();
      // print_state();
      // await getPBalances();
      crank_count++;
    }
    if (state.maturityState == 1) {
      print_state();
      await crank1();
      console.log();
      console.log('new run');
      state = await program.account.mintAuth.fetch(mintAuth);
      crank_count = 0;
      print_state();
    }
    if (state.maturityState == 2) {
      if (balances[b0Key] > 0) {
        await redeem0();
        await getBalances();
        buyAm = balances[ccKey];
        // await buy_bonds1();
        // await getBalances();
        /*
        var ima = state.ima0*60*60*24*365;
        if (ima < 5) {
          await sell_bonds1();
          await getBalances();
        }
        if (ima > 50.0) {
          buyAm = balances[ccKey] / 100;
          if (ima > 60.0) { buyAm = balances[ccKey] / 10; }
          if (ima > 70.0) { buyAm = balances[ccKey]; }
          await buy_bonds1();
          await getBalances();
        }
        */
      }
      await crank2();
      // print_state();
      // await getPBalances();
      crank_count++;
    }
    if (state.maturityState == 3) {
      print_state();
      await crank3();
      console.log();
      console.log('new run');
      state = await program.account.mintAuth.fetch(mintAuth);
      crank_count = 0;
      print_state();
    }
  }
}

async function print_state() {
    let tstr = '';
    tstr = 'imod ' + state.imod.toExponential(5);
    while (tstr.length < 38) tstr += ' ';
    tstr += 'ima0 ' + (state.ima0*60*60*24*365).toExponential(5);
    console.log(tstr);

    tstr = 'rmod ' + state.rmod.toExponential(5);
    while (tstr.length < 38) tstr += ' ';
    tstr += 'ima1 ' + (state.ima1*60*60*24*365).toExponential(5);
    console.log(tstr);

    tstr = 'smod ' + state.smod.toExponential(5);
    console.log(tstr);

    tstr = 'cmod ' + state.cmod.toExponential(5);
    console.log(tstr);

    tstr = 'isum ' + state.isum.toExponential(5);
    console.log(tstr);

    console.log();
    tstr = 'timestamp ' + state.timestamp.toString();
    while (tstr.length < 26) tstr += ' ';
    tstr += 'crank_count ' + crank_count;
    while (tstr.length < 48) tstr += ' ';
    tstr += 'maturity  ' + state.maturityState.toString();
    console.log(tstr);
    console.log();

    await getPBalances();
}

async function sell_bonds1() {
  const x0 = new anchor.BN(balances[b1Key]/200);
  console.log('   selling ' + x0 + ' b1s');
  if (x0.eq(new anchor.BN(0))) { return; }
    try {
      await program.methods.sellBonds1(x0,
        mintAuthBump,
        ccMintBump,
        ccb1MintBump,
        ccs0MintBump,
      ).accounts({
        mintAuthority: mintAuth,

        ccMint: ccMint,
        ccb1Mint: ccb1Mint,
        ccs0Mint: ccs0Mint,

        ownerCcToken: owner_cc_ata,
        ownerCcb1Token: owner_ccb1_ata,

        ccToken: cc_ata,
        ccb1Token: ccb1_ata,
        ccs0Token: ccs0_ata,

        owner: payer.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).signers([payer.payer]).rpc();
      await sleep.sleep(5);
    } catch { console.log('sellBonds1 fail'); }
}

async function sell_bonds0() {
  const x0 = new anchor.BN(balances[b0Key]/200);
  console.log('   selling ' + x0 + ' b0s');
  if (x0.eq(new anchor.BN(0))) { return; }
    try {
      await program.methods.sellBonds0(x0,
        mintAuthBump,
        ccMintBump,
        ccb0MintBump,
        ccs0MintBump,
      ).accounts({
        mintAuthority: mintAuth,

        ccMint: ccMint,
        ccb0Mint: ccb0Mint,
        ccs0Mint: ccs0Mint,

        ownerCcToken: owner_cc_ata,
        ownerCcb0Token: owner_ccb0_ata,

        ccToken: cc_ata,
        ccb0Token: ccb0_ata,
        ccs0Token: ccs0_ata,

        owner: payer.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).signers([payer.payer]).rpc();
      await sleep.sleep(5);
    } catch { console.log('sellBonds0 fail'); }
}

async function buy_bonds1() {
  let maxx0 = -balances[s0Key];
  let maxx1 = buyAm - maxx0 / state.ccsAmount * state.cc1Amount;
  for (let x0 = 0; x0 < buyAm; x0++) {
    let x1 = (buyAm - x0) * (state.cc1Amount + x0) / state.cc1Amount;
    if (x1 > maxx1) {
      maxx1 = x1;
      maxx0 = x0;
    }
  }
  console.log('   buying ' + maxx0 + ' shorts and '
    + (buyAm - maxx0).toString() + ' b1s');
  if (maxx0 > 0) {
    try {
      await program.methods.buyShorts1(
        new anchor.BN(maxx0),
        mintAuthBump,
        ccMintBump,
        ccb1MintBump,
        ccs0MintBump,
      ).accounts({
        mintAuthority: mintAuth,

        ccMint: ccMint,
        ccb1Mint: ccb1Mint,
        ccs0Mint: ccs0Mint,

        ownerCcToken: owner_cc_ata,
        ownerCcs0Token: owner_ccs0_ata,

        ccToken: cc_ata,
        ccb1Token: ccb1_ata,
        ccs0Token: ccs0_ata,

        owner: payer.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).signers([payer.payer]).rpc();
      await sleep.sleep(5);
    } catch { console.log('buyShorts1 fail'); }
  }

  if (maxx0 < 0) {
    try {
      await program.methods.sellShorts1(
        new anchor.BN(-maxx0),
        mintAuthBump,
        ccMintBump,
        ccb1MintBump,
        ccs0MintBump,
      ).accounts({
        mintAuthority: mintAuth,

        ccMint: ccMint,
        ccb1Mint: ccb1Mint,
        ccs0Mint: ccs0Mint,

        ownerCcToken: owner_cc_ata,
        ownerCcs0Token: owner_ccs0_ata,

        ccToken: cc_ata,
        ccb1Token: ccb1_ata,
        ccs0Token: ccs0_ata,

        owner: payer.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).signers([payer.payer]).rpc();
      await sleep.sleep(5);
      maxx0 = 0;
      await getBalances();
    } catch { console.log('sellShorts1 fail'); }
  }

  if (buyAm - maxx0 > 0) {
    try {
      await program.methods.buyBonds1(
        new anchor.BN(buyAm - maxx0),
        mintAuthBump,
        ccMintBump,
        ccb1MintBump,
        ccs0MintBump,
      ).accounts({
        mintAuthority: mintAuth,

        ccMint: ccMint,
        ccb1Mint: ccb1Mint,
        ccs0Mint: ccs0Mint,

        ownerCcToken: owner_cc_ata,
        ownerCcb1Token: owner_ccb1_ata,

        ccToken: cc_ata,
        ccb1Token: ccb1_ata,
        ccs0Token: ccs0_ata,

        owner: payer.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).signers([payer.payer]).rpc();
      await sleep.sleep(5);
    } catch { console.log('buyBonds1 fail'); }
  }
}

async function buy_bonds0() {
  let maxx0 = -balances[s0Key];
  let maxx1 = buyAm - maxx0 / state.ccsAmount * state.cc1Amount;
  for (let x0 = 0; x0 < buyAm; x0++) {
    let x1 = (buyAm - x0) * (state.cc1Amount + x0) / state.cc1Amount;
    if (x1 > maxx1) {
      maxx1 = x1;
      maxx0 = x0;
    }
  }
  console.log('   buying ' + maxx0 + ' shorts and '
    + (buyAm - maxx0).toString() + ' b0s');
  if (maxx0 > 0) {
    try {
      await program.methods.buyShorts0(
        new anchor.BN(maxx0),
        mintAuthBump,
        ccMintBump,
        ccb0MintBump,
        ccs0MintBump,
      ).accounts({
        mintAuthority: mintAuth,

        ccMint: ccMint,
        ccb0Mint: ccb0Mint,
        ccs0Mint: ccs0Mint,

        ownerCcToken: owner_cc_ata,
        ownerCcs0Token: owner_ccs0_ata,

        ccToken: cc_ata,
        ccb0Token: ccb0_ata,
        ccs0Token: ccs0_ata,

        owner: payer.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).signers([payer.payer]).rpc();
      await sleep.sleep(5);
    } catch { console.log('buyShorts0 fail'); }
  }

  if (maxx0 < 0) {
    try {
      await program.methods.sellShorts0(
        new anchor.BN(-maxx0),
        mintAuthBump,
        ccMintBump,
        ccb0MintBump,
        ccs0MintBump,
      ).accounts({
        mintAuthority: mintAuth,

        ccMint: ccMint,
        ccb0Mint: ccb0Mint,
        ccs0Mint: ccs0Mint,

        ownerCcToken: owner_cc_ata,
        ownerCcs0Token: owner_ccs0_ata,

        ccToken: cc_ata,
        ccb0Token: ccb0_ata,
        ccs0Token: ccs0_ata,

        owner: payer.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).signers([payer.payer]).rpc();
      await sleep.sleep(5);
      maxx0 = 0;
      await getBalances();
    } catch { console.log('sellShorts0 fail'); }
  }

  if (buyAm - maxx0 > 0) {
    try {
      await program.methods.buyBonds0(
        new anchor.BN(buyAm - maxx0),
        mintAuthBump,
        ccMintBump,
        ccb0MintBump,
        ccs0MintBump,
      ).accounts({
        mintAuthority: mintAuth,

        ccMint: ccMint,
        ccb0Mint: ccb0Mint,
        ccs0Mint: ccs0Mint,

        ownerCcToken: owner_cc_ata,
        ownerCcb0Token: owner_ccb0_ata,

        ccToken: cc_ata,
        ccb0Token: ccb0_ata,
        ccs0Token: ccs0_ata,

        owner: payer.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      }).signers([payer.payer]).rpc();
      await sleep.sleep(5);
    } catch { console.log('buyBonds0 fail'); }
  }
}

async function init() {
  [ mintAuth, mintAuthBump ] =
    await anchor.web3.PublicKey.findProgramAddress(
      [ Buffer.from("mint_auth_") ], program.programId
    );
  console.log(`mintAuth ${mintAuthBump} ${mintAuth}`);

  [ ccMint, ccMintBump ] =
    await anchor.web3.PublicKey.findProgramAddress(
      [ Buffer.from("cc_mint_") ], program.programId
    );
  console.log(`ccMint ${ccMintBump} ${ccMint}`);

  [ ccb0Mint, ccb0MintBump ] =
    await anchor.web3.PublicKey.findProgramAddress(
      [ Buffer.from("ccb0_mint_") ], program.programId
    );
  console.log(`ccb0Mint ${ccb0MintBump} ${ccb0Mint}`);

  [ ccb1Mint, ccb1MintBump ] =
    await anchor.web3.PublicKey.findProgramAddress(
      [ Buffer.from("ccb1_mint_") ], program.programId
    );
  console.log(`ccb1Mint ${ccb1MintBump} ${ccb1Mint}`);

  [ ccs0Mint, ccs0MintBump ] =
    await anchor.web3.PublicKey.findProgramAddress(
      [ Buffer.from("ccs0_mint_") ], program.programId
    );
  console.log(`ccs0Mint ${ccs0MintBump} ${ccs0Mint}`);

  cc_ata = await anchor.utils.token.associatedAddress({
    mint: ccMint,
    owner: mintAuth
  });
  console.log(`cc_ata ${cc_ata}`);

  ccb0_ata = await anchor.utils.token.associatedAddress({
    mint: ccb0Mint,
    owner: mintAuth
  });
  console.log(`ccb0_ata ${ccb0_ata}`);

  ccb1_ata = await anchor.utils.token.associatedAddress({
    mint: ccb1Mint,
    owner: mintAuth
  });
  console.log(`ccb1_ata ${ccb1_ata}`);

  ccs0_ata = await anchor.utils.token.associatedAddress({
    mint: ccs0Mint,
    owner: mintAuth
  });
  console.log(`ccs0_ata ${ccs0_ata}`);

  owner_cc_ata = await anchor.utils.token.associatedAddress({
    mint: ccMint,
    owner: payer.publicKey
  });
  console.log(`owner_cc_ata ${owner_cc_ata}`);

  owner_ccb0_ata = await anchor.utils.token.associatedAddress({
    mint: ccb0Mint,
    owner: payer.publicKey
  });
  console.log(`owner_ccb0_ata ${owner_ccb0_ata}`);

  owner_ccb1_ata = await anchor.utils.token.associatedAddress({
    mint: ccb1Mint,
    owner: payer.publicKey
  });
  console.log(`owner_ccb1_ata ${owner_ccb1_ata}`);

  owner_ccs0_ata = await anchor.utils.token.associatedAddress({
    mint: ccs0Mint,
    owner: payer.publicKey
  });
  console.log(`owner_ccs0_ata ${owner_ccs0_ata}`);
}

async function getBalances() {
  const res = execSync('spl-token accounts',{encoding:'utf8'});
  const lines = res.split('\n');
  for (var i=2;i<lines.length;i++) {
    const lst = lines[i].split(' ');
    if (lst.length < 2) continue;
    for (var ii=0;ii<lst.length;ii++) {
      balances[lst[0].toString()] = Number(lst[2]);
    }
  }
  console.log('ccKey ',balances[ccKey]);
  console.log('b0Key ',balances[b0Key]);
  console.log('b1Key ',balances[b1Key]);
  console.log('s0Key ',balances[s0Key]);
  await getPBalances();
}

async function getPBalances() {
  const res = execSync('spl-token accounts --owner 5Ju8Dax7SgVsygfwkkuDX1eoJHwCQFgjpiCSctjrPZoC',{encoding:'utf8'});
  const lines = res.split('\n');
  for (var i=2;i<lines.length;i++) {
    const lst = lines[i].split(' ');
    if (lst.length < 2) continue;
    for (var ii=0;ii<lst.length;ii++) {
      pbalances[lst[0].toString()] = Number(lst[2]);
    }
  }
  let ccb = pbalances[b0Key] + pbalances[b1Key];
  let cc0 = pbalances[ccKey] - ccb;
  let ccs = pbalances[s0Key];
  console.log('program balances');
  console.log('cc0   ',cc0,'      ccKey ',pbalances[ccKey]);
  console.log('ccb   ',ccb,'      b0Key ',pbalances[b0Key]);
  console.log('cc1   ',ccb,'      b1Key ',pbalances[b1Key]);
  console.log('ccs   ',ccs,'      s0Key ',pbalances[s0Key]);
}

async function redeem0() {
    console.log('   redeeming b0s  ' + new Date());
  try {
    await program.methods.redeemBonds0(
      mintAuthBump,
      ccMintBump,
      ccb0MintBump,
      ccb1MintBump,
    ).accounts({
      mintAuthority: mintAuth,

      ccMint: ccMint,
      ccb0Mint: ccb0Mint,
      ccb1Mint: ccb1Mint,

      ownerCcToken: owner_cc_ata,
      ownerCcb0Token: owner_ccb0_ata,
      ownerCcb1Token: owner_ccb1_ata,

      owner: payer.publicKey,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
  } catch { console.log('redeem0 fail'); }
  await sleep.sleep(5);
}

async function redeem1() {
    console.log('   redeeming b1s  ' + new Date());
  try {
    await program.methods.redeemBonds1(
      mintAuthBump,
      ccMintBump,
      ccb0MintBump,
      ccb1MintBump,
    ).accounts({
      mintAuthority: mintAuth,

      ccMint: ccMint,
      ccb0Mint: ccb0Mint,
      ccb1Mint: ccb1Mint,

      ownerCcToken: owner_cc_ata,
      ownerCcb0Token: owner_ccb0_ata,
      ownerCcb1Token: owner_ccb1_ata,

      owner: payer.publicKey,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
  } catch { console.log('redeem1 fail'); }
  await sleep.sleep(5);
}

async function crank0() {
  try {
    await program.methods.crank0(
      mintAuthBump, ccMintBump,
      ccb0MintBump,
      ccs0MintBump
    ).accounts({
      mintAuthority: mintAuth,
      ccMint: ccMint,
      ccb0Mint: ccb0Mint,
      ccs0Mint: ccs0Mint,
      ccToken: cc_ata,
      ccb0Token: ccb0_ata,
      ccs0Token: ccs0_ata,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    await sleep.sleep(5);
  } catch {
    // console.log('crank0 fail',state.timestamp.toString(),Date.now());
  }
}
async function crank2() {
  try {
    await program.methods.crank2(
      mintAuthBump, ccMintBump,
      ccb1MintBump,
      ccs0MintBump
    ).accounts({
      mintAuthority: mintAuth,
      ccMint: ccMint,
      ccb1Mint: ccb1Mint,
      ccs0Mint: ccs0Mint,
      ccToken: cc_ata,
      ccb1Token: ccb1_ata,
      ccs0Token: ccs0_ata,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
    await sleep.sleep(5);
  } catch {
    // console.log('crank2 fail');
  }
}

async function crank1() {
  try {
    await program.methods.crank1(
      mintAuthBump,
      ccMintBump,
      ccb0MintBump,
      ccb1MintBump,
      ccs0MintBump,
    ).accounts({
      mintAuthority: mintAuth,

      ccMint: ccMint,
      ccb0Mint: ccb0Mint,
      ccb1Mint: ccb1Mint,
      ccs0Mint: ccs0Mint,

      ccToken: cc_ata,
      ccb0Token: ccb0_ata,
      ccb1Token: ccb1_ata,
      ccs0Token: ccs0_ata,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
  } catch { console.log('crank1 fail'); }
  await sleep.sleep(5);
}

async function crank3() {
  try {
    await program.methods.crank3(
      mintAuthBump,
      ccMintBump,
      ccb0MintBump,
      ccb1MintBump,
      ccs0MintBump,
    ).accounts({
      mintAuthority: mintAuth,

      ccMint: ccMint,
      ccb0Mint: ccb0Mint,
      ccb1Mint: ccb1Mint,
      ccs0Mint: ccs0Mint,

      ccToken: cc_ata,
      ccb0Token: ccb0_ata,
      ccb1Token: ccb1_ata,
      ccs0Token: ccs0_ata,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
    }).signers([payer.payer]).rpc();
  } catch { console.log('crank3 fail'); }
  await sleep.sleep(5);
}

// async function start_loop() {
  // while (true) {
    try {
      start();
    } catch (err) {
      console.log(err);
      console.log('starting over');
    }
  // }
// }

// start_loop();
