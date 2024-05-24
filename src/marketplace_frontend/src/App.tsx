import { Connection } from "./lib/utils";

function App() {
  
  const connection = new Connection();

  return (
    <main>
      <h1>Internet Identity Demo Webapp</h1>
      <section>
        <label htmlFor="iiUrl">Internet Identity URL:</label>
      </section>
      <section>
        <button onClick={connection.init}>Login with Internet Identity</button><br/><br/>
        <button onClick={() => connection.createCanister(
            {
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
            [{
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
            BigInt(Date.now() * 1_000_000 + 1_000_000_000_000_000_000),
            [{
              expire_date: BigInt(Date.now() * 1_000_000 + 1_000_000_000_000_000 ),
              discount_percentage: 10
            }]
        )}>create canister</button><br/><br/>
        <button onClick={() => connection.getCollectionsByCaller([], 0, 10)}>display all your canisters</button><br/><br/>
        <button onClick={() => connection.getNfts(0, 100)}>get all nfts</button><br/><br/>
        <input id="canisterIDforBuy" type="text"/><br/>
        <input id="NFTforBuy" type="text"/><br/>
        <button onClick={() => connection.purchaseNft()}>BUY</button><br/><br/>
        <button onClick={() => connection.getAllCollections(0, 10)}>display all collections and NFTs</button><br/><br/>
        <button >show token metadata</button><br/><br/>
        <button >supply cap</button><br/><br/>
        <button >total supply</button><br/><br/>
        <button >transfer</button><br/><br/>
        <button >supported standard</button><br/><br/>
        <button >burn</button><br/><br/>
        <button >balance of</button><br/><br/>
        <button >name</button><br/><br/>
        <input id="canisterIDforSymbol" type="text"/><br/>
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
