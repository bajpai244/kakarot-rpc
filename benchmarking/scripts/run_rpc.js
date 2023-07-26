// the script does the following:
// - run Madara binary
// - deploy Kakarot to Madara
// - run RPC over Madara

const { spawn } = require("child_process");
const { resolve } = require("path");
const util = require('util');


const exec = util.promisify(require('child_process').exec);

const { waitForMadaraInit, waitForRPCInit } = require("./utils");
const { readFileSync } = require("fs");

const runMadara = async () => {
   const madaraPath = resolve(process.cwd(), "../lib/madara/target/release/madara");
   const madara = spawn(madaraPath, ["--dev"]); 

//    madara.on("error", (error) => {
//     console.error("error from madara:\n", error.message)
//    });

//    madara.on("close", (code) => {
//     console.error("madara process exited with code:", code)
//    });

   await waitForMadaraInit();
}

const deploy_kakarot = async () => {
   console.log("kakarot deployment started ...")

   const deployScriptPath = resolve(process.cwd(), "./scripts/deploy_kakarot.sh")
   await exec(deployScriptPath) 

   console.log("kakarot deployed successfully ✅")
}

const run_rpc = async () => {
    const madaraDeploymentFilePath = resolve(process.cwd(), "../lib/kakarot/deployments/madara/deployments.json")
    const madaraDeployments = JSON.parse(readFileSync(madaraDeploymentFilePath).toString());
    const kakarotAddress = madaraDeployments.kakarot.address;

    console.log("starting kakarot rpc ...")

    const rpcPath = resolve(process.cwd(), "../target/release/kakarot-rpc");
    const rpc = spawn(rpcPath, {
        env : {
            "STARKNET_NETWORK": "madara",
            "KAKAROT_HTTP_RPC_ADDRESS":"0.0.0.0:3030",
            "KAKAROT_ADDRESS": kakarotAddress,
            "MADARA_ACCOUNT_ADDRESS":"0x3",
            "MADARA_PRIVATE_KEY":"0x00c1cf1490de1352865301bb8705143f3ef938f97fdf892f1090dcb5ac7bcd1d",
            "PROXY_ACCOUNT_CLASS_HASH":"0x4b9eef81a3f0a582dfed69be93196cedbff063e0fa206b34b4c2f06ac505f0c",
            "RUST_LOG": ""
        }
    }); 
 
//     rpc.on("error", (error) => {
//      console.error("error from rpc:\n", error.message)
//     });
 
//    rpc.on("close", (code) => {
//      console.error("rpc process exited with code:", code)
//     });

    await waitForRPCInit();
}


const main = async () => {
    await runMadara();
    await deploy_kakarot();

    await run_rpc();

}

main()