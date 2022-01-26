// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

export const types = {
    BlockCid: "Vec<u8>",
    BlockSubmissionProposal: {
        proposer: "AccountId",
        status: "ProposalStatus",
        start_block: "BlockNumber",
        end_block: "BlockNumber",
    },
    MessageRootCid: "Vec<u8>",
};

/**
 * rpc methods
 */
 export const rpc = {
    filecoindot: {
        setRpcEndpoint: {
            description: "set filecoin rpc http endpoint",
            params: [
                {
                    name: "urls",
                    type: "Vec<String>",
                },
            ],
            type: "()",
        },
        verifyReceipt: {
            description: "verify filecoin receipt",
            params: [
                {
                    name: "proof",
                    type: "String",
                },
                {
                    name: "cid",
                    type: "String",
                },
            ],
            type: "bool",
        },
        verifyState: {
            description: "verify filecoin state",
            params: [
                {
                    name: "proof",
                    type: "String",
                },
                {
                    name: "cid",
                    type: "String",
                },
            ],
            type: "bool",
        },
    },
};
exports.rpc = rpc;