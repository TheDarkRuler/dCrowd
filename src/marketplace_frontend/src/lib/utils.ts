import { Principal } from "@dfinity/principal";
import { icp_ledger_canister, createActor as createLedgerActor } from "../../../declarations/icp_ledger_canister"
import { marketplace_backend, createActor as createBackendActor } from "../../../declarations/marketplace_backend";
import { CanisterArg, DiscountWindowArg, Errors, NftMetadata } from "../../../declarations/marketplace_backend/marketplace_backend.did";
import { HttpAgent, Identity, Agent } from "@dfinity/agent";
import { isSafari } from "react-device-detect";
import { AuthClient } from "@dfinity/auth-client";


/**
 * Utility class for requests to the backend.
 * This class provides methods for connecting to a backend using the candid interface,
 * Connection to the backend of the marketplace and to the Mainnet ledger.
 * 
 * @class Connection
 */
export class Connection {

  actorBackend = marketplace_backend;
  actorLedger = icp_ledger_canister;
  identity!: Identity;
  agent!: Agent;

  /**
   * Init method that let the user log with internet identity opening a window of login provided by ICP.
   * After that, it creates the actors for the backend and the ledger canisters.
   */
  init = async () => {
    const local_ii_url = isSafari ? 
    `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`: 
    `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`;


    const iiUrl = process.env.DFX_NETWORK === "ic" ?
      `https://${process.env.CANISTER_ID_INTERNET_IDENTITY}.ic0.app`: 
      local_ii_url;
      
    const authClient = await AuthClient.create();

    const canisterBackendId = process.env.CANISTER_ID_MARKETPLACE_BACKEND;
    const canisterLedger = process.env.CANISTER_ID_ICP_LEDGER_CANISTER;

    await new Promise<void>((resolve, reject) => {
      authClient.login({
        identityProvider: iiUrl,
        onSuccess: resolve,
        onError: reject,
      });
    });

    this.identity = authClient.getIdentity();

    this.agent = new HttpAgent({ identity: this.identity });

    this.actorBackend = createBackendActor(canisterBackendId as string, {
      agent: this.agent
    });

    this.actorLedger = createLedgerActor(canisterLedger as string, {
      agent: this.agent
    }); 
  }

  /**
   * 
   * Method that needs collectionId and nftId of the NFT to buy, (set and get them as you like)
   * 
   * 1) Firstly it checks if the collectionId and nftId are not empty strings.
   * 2) Checks the balance of the caller, returning the right price of the NFT (fee included).
   * 3) Calls ICRC-2 approve method to approve the backend canister to use exactly the nft price amount of tokens (ICP) as their behalf (on this method the caller pays a fee).
   * 4) If everything went well transfer_nft is called, where it happends exactly:
   *    1) The caller sends exactly the nftPrice to the owner of the NFT (a fee is paid in this operation).
   *    2) The NFT is transfered from the owner of it to the caller of the function.
   * 
   * @param actorBackend backend actor used to call api
   * @param actorLedger mainnet ledger actor used to call the ICRC-1 and ICRC-2 methods
   * @returns 
   */
  async purchaseNft() {
    // Create them as you wish
    const collectionId = document.querySelector<HTMLInputElement>("#canisterIDforBuy")!.value // Get collection canister id from app
    const nftId = document.querySelector<HTMLInputElement>("#NFTforBuy")!.value // Get nft token id from app
  
    if (collectionId == "" || nftId == "") {
        console.log("collection id or Nft id empty")
        return 
    }
  
   /* // Checks balance of the caller and if ( balance >= fee + nftPrice ) then it returns the nftPrice 
    let nftPrice = await this.actorBackend.check_balance([], BigInt(nftId), collectionId)
  
    if ("Err" in nftPrice) {
        console.log(nftPrice.Err)
        return
    }
  
    // Approves the backend canister to transfer token (ex. ICP) on caller behalf
    let approval = await this.actorLedger.icrc2_approve({
      from_subaccount: [],
      spender: {
        owner: Principal.fromText(process.env.CANISTER_ID_MARKETPLACE_BACKEND as string),
        subaccount: []
      },
      amount: nftPrice.Ok + await icp_ledger_canister.icrc1_fee(),
      expected_allowance: [],
      expires_at: [],
      fee: [],
      memo: [],
      created_at_time: [],
    })
  
    if ("Err" in approval) {
        console.log(approval.Err)
        return
    }*/
  
    let transferRes = await this.actorBackend.transfer_nft({
        tkn_id: BigInt(nftId),
        collection_id: collectionId
    })
  
    console.log(transferRes)
  }

  /**
   * Method that gets from backend all the collections assigned to the caller.
   * 
   * @param caller Optional -> if undefined then the caller of the method will be used
   * @param offset Offset of the first element to retrieve
   * @param limit Number of elements to retrieve
   * @returns List of collections assigned to caller
   */
  async getCollectionsByCaller(caller: [] | [string], offset: number, limit: number): Promise<string | string[]> {
    let res = await this.actorBackend.get_collection_ids(caller, offset, limit)

    if ("Ok" in res) {
      return res.Ok
    }
    return res.Err
  }

  /**
   * Method to create a collection canister
   * 
   * @param arg CanisterArgs
   * @param nfts Vector of NftMetadata the total sum of the nfts quantity needs to be equal to the supply_cap in CanisterArg
   * @param expire_date expire date in nanosecods of the collection
   * @param discount_windows array of windows of discount, giving in nanoseconds the expire date of the discount and the discount percentage (10 = 10%)
   * @returns success message or an error of type Errors
   */
  async createCanister(arg: CanisterArg, nfts: NftMetadata[], expire_date: bigint, discount_windows: DiscountWindowArg[]): Promise<string | Errors> {
    const res = await this.actorBackend.create_collection_nfts({
      canister_arg: arg,
      nfts,
      expire_date,
      discount_windows
    });

    if ("Ok" in res) {
      return res.Ok
    }
    return res.Err
  }

  /**
   * Gets an array of nfts from the marketplace
   * 
   * @param offset Offset of the first element to retrieve
   * @param limit Number of elements to retrieve
   * @returns Array of (token id, the collection canister principal it is assigned to, a boolean saying if it is on sale or not, the owner and the price)
   */
  async getNfts(offset: number, limit: number) {
    const res = await this.actorBackend.get_all_nfts(offset, limit);

    if ("Ok" in res) {
      return res.Ok
    }
    return res.Err
  }

  /**
   * Gets all collections and their full info
   * 
   * @param offset Offset of the first element to retrieve
   * @param limit Number of elements to retrieve
   * @returns Array of (available boolean saying if the collection is expired, c
   * anister_id, discount window array, expire_date, array of Nfts giving the metadata and all the ids of that type on the collection, owner of the collection)
   */
  async getAllCollections(offset: number, limit: number) {
    const res = await this.actorBackend.get_all_collections(offset, limit)

    if ("Ok" in res) {
      return res.Ok.map(x => {
        return {...x, canister_id: x.canister_id.toText(), owner: x.owner.toText()}
      })
    }
    return res.Err
  }

}

