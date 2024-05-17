import { HttpAgent, Identity, Agent, ActorSubclass } from "/home/formazione/Desktop/testICP/icrc7/node_modules/@dfinity/agent";
import { AuthClient } from "/home/formazione/Desktop/testICP/icrc7/node_modules/@dfinity/auth-client";
import { createActor as createBackendActor, marketplace_backend } from "../../declarations/marketplace_backend";
import { createActor as createIcrcActor} from "../../declarations/icrc7";
import { createActor as createLedgerActor, icp_ledger_canister } from "../../declarations/icp_ledger_canister";
import { _SERVICE } from "../../declarations/icrc7/icrc7.did";
import { isSafari } from 'react-device-detect';
import { Principal } from "/home/formazione/Desktop/testICP/icrc7/node_modules/@dfinity/principal/lib/cjs/index";

function App() {

  let identity: Identity;
  let agent: Agent;
  let actorBackend = marketplace_backend;
  let actorsIcrc7 = new Map<string, ActorSubclass<_SERVICE>>();
  let actorLedger = icp_ledger_canister;

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
      const canisterLedger = process.env.CANISTER_ID_ICP_LEDGER_CANISTER;

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

      actorLedger = createLedgerActor(canisterLedger as string, {
        agent
      });

      let canisters = await actorBackend.get_collection_ids([], 0, 100);

      if ("Ok" in canisters) {
        canisters.Ok.forEach(x => {
          actorsIcrc7.set(x, createIcrcActor(x, {
            agent,
          }));
        })
      }
  }

  async function display_canister() {
    console.log(await actorBackend.get_collection_ids([], 0, 100))
  }
  
  async function mint() {
    let canisters = await actorBackend.get_collection_ids([], 0, 100);
    let canisterId = "";
    if ("Ok" in canisters) {
      canisterId = canisters.Ok[0]
    }
    let res = await actorsIcrc7.get(canisterId)?.icrc7_mint({
      to: { owner: identity!.getPrincipal(), subaccount: [] }, token_id: BigInt(400), memo: [],
      from_subaccount: [], token_description: [], token_logo: [], token_name: [],
      token_privilege_code: []
    }, [])
    console.log(res)
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
        price: 20_000
      },
      {
        token_description: "premium ticket",
        quantity: 20n,
        token_logo: "logo",
        token_name: "premium",
        token_privilege_code: 2,
        price: 5_000
      }],
      expire_date : BigInt(Date.now() * 1_000_000 + 1_000_000_000_000),
      discount_windows: [{
          expire_date: BigInt(Date.now() * 1_000_000 + 1_000_000_000 ),
          discount_percentage: 10
        }]
    });
    console.log(result)

    if ("Ok" in result) {
      actorsIcrc7.set(result.Ok, createIcrcActor(result.Ok, {
        agent,
      }));
    }

  }
  
  async function display_nfts() {
    let canisters = await actorBackend.get_collection_ids([], 0, 100);
    let canisterId = "";
    if ("Ok" in canisters) {
      canisterId = canisters.Ok[0]
    }

    let tokens = await actorsIcrc7.get(canisterId)?.icrc7_tokens([], [])

    if (tokens != undefined) {
      console.log(tokens.length)
      let res = await actorsIcrc7.get(canisterId)?.icrc7_token_metadata(tokens)
      console.log(res)
    }
  }

  async function symbol() {
    const canisterIcircId = document.querySelector<HTMLInputElement>("#canisterIDforSymbol")!.value
    //console.log(await actorBackend.collection_symbol(canisterIcircId))
  }

  async function display_archive() {
    let canisters = await actorBackend.get_collection_ids([], 0, 100);
    let canisterId = "";
    if ("Ok" in canisters) {
      canisterId = canisters.Ok[0]
    }

    console.log(await actorsIcrc7.get(canisterId)?.icrc7_archive_log_canister())

    let res = await actorsIcrc7.get(canisterId)?.icrc3_get_archives({'from' : [Principal.fromText(canisterId)]})
    console.log(res)
    
    let res2 = await actorsIcrc7.get(canisterId)?.icrc3_get_blocks([{ 'start' : 0n, 'length' : 0n }])
    console.log(res2)
  }

  function timestamp() {
    console.log(Date.now() * 1_000_000)
    console.log(window.performance.now())
  }

  async function buy() {
    const canisterIcircId = document.querySelector<HTMLInputElement>("#canisterIDforBuy")!.value
    console.log(await actorLedger.icrc1_name())
    let res = await actorLedger.icrc2_approve({
      from_subaccount: [],
      spender: {
        owner: Principal.fromText(process.env.CANISTER_ID_MARKETPLACE_BACKEND as string),
        subaccount: []
      },
      amount: 999965935000n + await icp_ledger_canister.icrc1_fee(),
      expected_allowance: [],
      expires_at: [],
      fee: [],
      memo: [],
      created_at_time: [],
    })

    if ('Ok' in res) {

      let first = await actorLedger.icrc2_allowance({
        account: {
          owner: Principal.fromText("xgad6-k6unb-6xhhm-tfnjm-5mq6w-sr7p5-fwlau-ggonc-diqie-ngpjd-tqe"),
          subaccount: []
        },
        spender: {
          owner: Principal.fromText(process.env.CANISTER_ID_MARKETPLACE_BACKEND as string),
          subaccount: []
        }
      })

      await actorBackend.transfer_nft({
        amount: 9999659350n,
        to_account: {
          owner: Principal.fromText("pofdv-klrb4-bqcke-d5tgu-tozrk-pdzpy-vzmep-depus-afiqw-s2n5e-nae"),
          subaccount: []
        }
      })

      let second = await actorLedger.icrc2_allowance({
        account: {
          owner: Principal.fromText("xgad6-k6unb-6xhhm-tfnjm-5mq6w-sr7p5-fwlau-ggonc-diqie-ngpjd-tqe"),
          subaccount: []
        },
        spender: {
          owner: Principal.fromText(process.env.CANISTER_ID_MARKETPLACE_BACKEND as string),
          subaccount: []
        }
      })
      console.log(first.allowance)
      console.log(second.allowance)
    } else {
      console.log("nooooooooooooooooooo    asdas das dasd as")
    }

   

  }

  async function displayAllCollections() {
    const res = await actorBackend.get_all_collections(0, 10)
    if ("Ok" in res) {

      let c = res.Ok.map(x => {
        return {...x, canister_id: x.canister_id.toText(), owner: x.owner.toText()}
      })
      console.log(c)
    }
  }

  async function displayNfts() {
    const res = await actorBackend.get_all_nfts(0, 100);
    console.log(res);
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
        <button onClick={display_nfts}>get all nfts</button><br/><br/>
        <button onClick={timestamp}>get time</button><br/><br/>
        <input id="canisterIDforBuy" type="text"/><br/>
        <button onClick={buy}>BUY</button><br/><br/>
        <button onClick={displayAllCollections}>display all collections and NFTs</button><br/><br/>
        <button >show token metadata</button><br/><br/>
        <button onClick={displayNfts}>all nfts</button><br/><br/>
        <button >supply cap</button><br/><br/>
        <button >total supply</button><br/><br/>
        <button >transfer</button><br/><br/>
        <button >supported standard</button><br/><br/>
        <button >burn</button><br/><br/>
        <button >balance of</button><br/><br/>
        <button onClick={display_archive}>show archive</button><br/><br/>
        <button >name</button><br/><br/>
        <input id="canisterIDforSymbol" type="text"/><br/>
        <button onClick={symbol}>symbol</button><br/><br/>
        <button onClick={mint}>mint</button><br/><br/>
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
