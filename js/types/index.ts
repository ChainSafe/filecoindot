const types = {
  BlockCid: "Vec<u8>",
  BlockSubmissionProposal: {
    proposer: "AccountId",
    status: "ProposalStatus",
    start_block: "BlockNumber",
    end_block: "BlockNumber",
  },
  MessageRootCid: "Vec<u8>",
};

export default types;
