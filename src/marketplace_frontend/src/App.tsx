import { Connection } from "./lib/utils";
import { Principal } from "@dfinity/principal";
import { icp_ledger_canister, createActor as createLedgerActor } from "../../declarations/icp_ledger_canister"
import { marketplace_backend, createActor as createBackendActor } from "../../declarations/marketplace_backend";
import { CanisterArg, DiscountWindowArg, Errors, NftMetadata } from "../../declarations/marketplace_backend/marketplace_backend.did";
import { HttpAgent, Identity, Agent } from "@dfinity/agent";
import { isSafari } from "react-device-detect";
import { AuthClient } from "@dfinity/auth-client";

function App() {

  let identity: Identity;
  let agent: Agent;
  let actorBackend = marketplace_backend;

  const local_ii_url = isSafari ? 
    `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`: 
    `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;


  const iiUrl = process.env.DFX_NETWORK === "ic" ?
    `https://${process.env.CANISTER_ID_INTERNET_IDENTITY}.ic0.app`: 
    local_ii_url;

  async function handleLogin() {

      const authClient = await AuthClient.create();

      const iiUrl = document.querySelector<HTMLInputElement>("#iiUrl")!.value;
      const canisterBackendId = process.env.CANISTER_ID_MARKETPLACE_BACKEND;

      await new Promise<void>((resolve, reject) => {
        authClient.login({
          identityProvider: iiUrl,
          onSuccess: resolve,
          onError: reject,
        });
      });

      identity = authClient.getIdentity()

      agent = new HttpAgent({ identity: identity as unknown as Identity });

      actorBackend = createBackendActor(canisterBackendId as string, {
        agent
      });

      let canisters = await actorBackend.get_collection_ids([], 0, 100);
  }

  async function display_canister() {
    console.log(await actorBackend.get_collection_ids([], 0, 100))
  }

  async function createCanister() {
    let result = await actorBackend.create_collection_nfts({
      canister_arg: {
        icrc7_symbol: "c",
        icrc7_name: "aasd",
        icrc7_description: [],
        icrc7_logo: [],
        icrc7_supply_cap: 50n,
        icrc7_max_query_batch_size: [],
        icrc7_max_update_batch_size: [],
        icrc7_max_take_value: [],
        icrc7_default_take_value: [],
        icrc7_max_memo_size: [],
        icrc7_atomic_batch_transfers: [],
        tx_window: [],
        permitted_drift: []
      },
      nfts: [{
        token_description: "standard ticket",
        quantity: 30n,
        token_logo: "logo standard",
        token_name: "standard",
        token_privilege_code: 1,
        price: 10
      },
      {
        token_description: "premium ticket",
        quantity: 20n,
        token_logo: "logo",
        token_name: "premium",
        token_privilege_code: 2,
        price: 500
      }],
      expire_date : BigInt(Date.now() * 1_000_000 + 1_000_000_000_000),
      discount_windows: [{
          expire_date: BigInt(Date.now() * 1_000_000 + 1_000_000_000 ),
          discount_percentage: 10
        }]
    });
    console.log(result)

  }
  

  async function symbol() {
    const canisterIcircId = document.querySelector<HTMLInputElement>("#canisterIDforSymbol")!.value
    //console.log(await actorBackend.collection_symbol(canisterIcircId))
  }

  function timestamp() {
    console.log(Date.now() * 1_000_000)
    console.log(window.performance.now())
  }

  return (
    <main>
      <h1>Internet Identity Demo Webapp</h1>
      <section>
        <label htmlFor="iiUrl">Internet Identity URL:</label>
        <input size={50} id="iiUrl" type="text" value={iiUrl} readOnly/>
      </section>
      <section>
        <button onClick={handleLogin}>Login with Internet Identity</button><br/><br/>
        <button onClick={createCanister}>create canister</button><br/><br/>
        <button onClick={display_canister}>display all your canisters</button><br/><br/>
        <button >get all nfts</button><br/><br/>
        <button onClick={timestamp}>get time</button><br/><br/>
        <button >show token metadata</button><br/><br/>
        <button >minting authority</button><br/><br/>
        <button >supply cap</button><br/><br/>
        <button >total supply</button><br/><br/>
        <button >transfer</button><br/><br/>
        <button >supported standard</button><br/><br/>
        <button >burn</button><br/><br/>
        <button >balance of</button><br/><br/>
        <button >show archive</button><br/><br/>
        <button >name</button><br/><br/>
        <input id="canisterIDforSymbol" type="text"/><br/>
        <button onClick={symbol}>symbol</button><br/><br/>
        <button >mint</button><br/><br/>
        <button >name</button><br/><br/>
        <button >set minting authority</button><br/><br/>
        <button >owner of</button><br/><br/>
        <button >approve</button><br/><br/>
        <input id="canisterID" type="text"/><br/>
        <button >set collection</button><br/><br/>


      </section>
      <section id="loginStatus">
        <p>Not logged in</p>
      </section>
    </main>
  );
}

export default App;
