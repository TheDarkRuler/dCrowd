import { HttpAgent, Identity, Agent } from "/home/formazione/Desktop/testICP/icrc7/node_modules/@dfinity/agent/lib/cjs/index";
import { AuthClient } from "@dfinity/auth-client";
import { createActor, icrc7 } from '../../declarations/icrc7';
import { createActor as createFactoryActor, factory } from "../../declarations/factory";
import { isSafari } from 'react-device-detect';
import { useState } from "react";

function App() {

  const [identity, setIdentity] = useState<Identity>();
  const [iiUrl, setIiUrl] = useState("");
  const [agent, setAgent] = useState<Agent>();
  const [actorFactory, setActorFactory] = useState(factory);
  const [actorIcrc7, setActorIcrc7] = useState(icrc7);


  // The <canisterId>.localhost URL is used as opposed to setting the canister id as a parameter
  // since the latter is brittle with regards to transitively loaded resources.
  const local_ii_url = isSafari ? 
    `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`: 
    `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;


  setIiUrl(process.env.DFX_NETWORK === "ic" ?
    `https://${process.env.CANISTER_ID_INTERNET_IDENTITY}.ic0.app`: 
    local_ii_url);

  async function handleLogin() {
      // When the user clicks, we start the login process.
      // First we have to create and AuthClient.
      const authClient = await AuthClient.create();

      // Find out which URL should be used for login.
      const iiUrl = document.querySelector<HTMLInputElement>("#iiUrl")!.value;
      const canisterFactoryId = process.env.CANISTER_ID_FACTORY;

      // Call authClient.login(...) to login with Internet Identity. This will open a new tab
      // with the login prompt. The code has to wait for the login process to complete.
      // We can either use the callback functions directly or wrap in a promise.
      await new Promise<void>((resolve, reject) => {
        authClient.login({
          identityProvider: iiUrl,
          onSuccess: resolve,
          onError: reject,
        });
      });

      // At this point we're authenticated, and we can get the identity from the auth client:
      setIdentity(authClient.getIdentity());
      // Using the identity obtained from the auth client, we can create an agent to interact with the IC.
      // Using the interface description of our webapp, we create an actor that we use to call the service methods.

      setAgent(new HttpAgent({ identity: identity as unknown as Identity }));

      setActorFactory(createFactoryActor(canisterFactoryId as string, {
        agent
      }));

      // Call whoami which returns the principal (user id) of the current user.
      // show the principal on the page
      

  }

  async function display_canister() {
    console.log(await actorFactory.get_principal())
  }
  
  async function mint() {
    let result = await actorIcrc7.icrc7_mint({to: {owner: identity!.getPrincipal(), subaccount: []}, token_id: BigInt(12), memo: [], 
      from_subaccount: [], token_description: [], token_logo: [], token_name: []}).then((a) => {
        //document.getElementById("loginStatus")!.innerText = icrc7.Result
        console.log(a)
        console.log(identity!.getPrincipal().toString())
      });
    console.log(result);
  }

  async function changeCollection() {

    const canisterIcircId = document.querySelector<HTMLInputElement>("#canisterID")!.value
    console.log(canisterIcircId);
    setActorIcrc7(createActor(canisterIcircId, {
      agent,
    }));
  }

  async function createCanister() {
    let _ = await actorFactory.mint_collection_canister({    
      icrc7_symbol: "c",
      icrc7_name: "aasd",
      icrc7_description: [],
      icrc7_logo: [],
      icrc7_supply_cap: [],
      icrc7_max_query_batch_size: [],
      icrc7_max_update_batch_size: [],
      icrc7_max_take_value: [],
      icrc7_default_take_value: [],
      icrc7_max_memo_size: [],
      icrc7_atomic_batch_transfers: [],
      tx_window: [],
      permitted_drift: []
    });
  }
  
  function lesgo() {

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
        <button onClick={lesgo}>get all nfts</button><br/><br/>
        <button onClick={lesgo}>show token metadata</button><br/><br/>
        <button onClick={lesgo}>minting authority</button><br/><br/>
        <button onClick={lesgo}>supply cap</button><br/><br/>
        <button onClick={lesgo}>total supply</button><br/><br/>
        <button onClick={lesgo}>transfer</button><br/><br/>
        <button onClick={lesgo}>supported standard</button><br/><br/>
        <button onClick={lesgo}>burn</button><br/><br/>
        <button onClick={lesgo}>balance of</button><br/><br/>
        <button onClick={lesgo}>logo</button><br/><br/>
        <button onClick={lesgo}>name</button><br/><br/>
        <button onClick={lesgo}>description</button><br/><br/>
        <button onClick={mint}>mint</button><br/><br/>
        <button onClick={lesgo}>name</button><br/><br/>
        <button onClick={lesgo}>set minting authority</button><br/><br/>
        <button onClick={lesgo}>owner of</button><br/><br/>
        <button onClick={lesgo}>approve</button><br/><br/>
        <input id="canisterID" type="text"/><br/>
        <button onClick={changeCollection}>set collection</button><br/><br/>

      </section>
      <section id="loginStatus">
        <p>Not logged in</p>
      </section>
    </main>
  );
}

export default App;
