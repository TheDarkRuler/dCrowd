import { Principal } from "/home/formazione/Desktop/testICP/icrc7/node_modules/@dfinity/principal/lib/cjs/index";
import { icp_ledger_canister } from "../../../declarations/icp_ledger_canister"
import { _SERVICE as LEDG_SERVICE } from "../../../declarations/icp_ledger_canister/icp_ledger_canister.did"
import { _SERVICE as BACK_SERVICE } from "../../../declarations/marketplace_backend/marketplace_backend.did";
import { ActorSubclass } from "/home/formazione/Desktop/testICP/icrc7/node_modules/@dfinity/agent";


export async function PurchaseNft(actorBackend: ActorSubclass<BACK_SERVICE>, actorLedger: ActorSubclass<LEDG_SERVICE>) {
    const collectionId = document.querySelector<HTMLInputElement>("#canisterIDforBuy")!.value // Get collection canister id from app
    const nftId = document.querySelector<HTMLInputElement>("#NFTforBuy")!.value // Get nft token id from app

    if (collectionId == "" || nftId == "") {
        console.log("collection id or Nft id empty")
        return 
    }

    // Checks balance of the caller and if ( balance >= fee + nftPrice ) then it returns the nftPrice 
    let nftPrice = await actorBackend.check_balance([], BigInt(nftId), collectionId)

    if ("Err" in nftPrice) {
        console.log(nftPrice.Err)
        return
    }

    // Approves the backend canister to transfer token (ex. ICP) on caller behalf
    let approval = await actorLedger.icrc2_approve({
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
    }

    let transferRes = await actorBackend.transfer_nft({
        amount: nftPrice.Ok,
        tkn_id: BigInt(nftId),
        collection_id: collectionId
    })

    console.log(transferRes)
  }