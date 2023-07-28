const { providers, Wallet } = require("ethers");
const { parseEther} = require("ethers/lib/utils");
const {sleep} = require("./utils")


 const nativeTokenTransfer = async (context, ee, next) => {
    const targetUrl = context.vars.target;
    const privateKey = context.vars.privateKey;

    const provider = new providers.JsonRpcProvider(targetUrl);

    const wallet = new Wallet(privateKey, provider);
    const wallet2 = Wallet.createRandom();

    const recepientAddress = wallet2.address;
    let nonce = await wallet.getTransactionCount();

    try {
    for (let i =0 ; i < 500 ; i +=1 ) {

    let tx = {
        to: recepientAddress,
        value: parseEther('0.00001'),
        nonce
    }

    const populatedTx = await wallet.populateTransaction(tx);
    const signedTx = await wallet.signTransaction(populatedTx);

    provider.sendTransaction(signedTx).then().catch((e)=>{
        // console.log('e ---->', e);
    })

    nonce +=1;
    console.log("transaction sent, current nonce ---->", nonce);

    // break to make sure transactions arrive in order to RPC.
    // NOTE: reduce it if you are skipping validate
    await sleep(350);
}
}
catch(err) {
    // the rpc is killed once metrics are collected, and hence we will land to this error
    if(err.code === "NETWORK_ERROR")
    {
       console.log("kakarot rpc is down");
       process.exit(0);
    }
    else {
        throw err
    }
}

    next();
}

module.exports = {
    nativeTokenTransfer,
}