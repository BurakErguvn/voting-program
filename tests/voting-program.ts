import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VotingProgram } from "../target/types/voting_program";
import { assert } from "chai";

describe("voting_program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.VotingProgram as Program<VotingProgram>;

  // Keypairs for testing
  const authority = anchor.web3.Keypair.generate();
  const voter = anchor.web3.Keypair.generate();

  // Poll PDA
  let pollPda: anchor.web3.PublicKey;
  let pollBump: number;

  // Voter PDA
  let voterPda: anchor.web3.PublicKey;
  let voterBump: number;

  before(async () => {
    // Airdrop SOL to authority and voter for testing
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(authority.publicKey, 1_000_000_000) // 1 SOL
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(voter.publicKey, 1_000_000_000) // 1 SOL
    );
  });

  it("Initializes a poll", async () => {
    const [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("poll"), authority.publicKey.toBuffer()],
      program.programId
    );
    pollPda = pda;
    pollBump = bump;

    const title = "Favorite programming language?";
    const options = ["Rust", "TypeScript", "Python"];

    const tx = await program.methods
      .initializePoll(title, options)
      .accounts({
        poll: pollPda,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();
    console.log("Transaction executed successfully: ", tx);
    const pollAccount = await program.account.poll.fetch(pollPda);
    assert.equal(pollAccount.title, title);
    assert.deepEqual(pollAccount.options, options);
    assert.equal(pollAccount.isActive, true);
    assert.equal(pollAccount.totalVotes.toNumber(), 0);
  });

  it("Casts a vote", async () => {
    const [pda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("voter"), pollPda.toBuffer(), voter.publicKey.toBuffer()],
      program.programId
    );
    voterPda = pda;
    voterBump = bump;

    const optionIndex = 1; // Voting for "TypeScript"

    const tx = await program.methods
      .castVote(optionIndex)
      .accounts({
        poll: pollPda,
        voter: voterPda,
        voterAccount: voter.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([voter])
      .rpc();
      console.log("Transaction executed successfully: ", tx);
    const pollAccount = await program.account.poll.fetch(pollPda);
    const voterAccount = await program.account.voter.fetch(voterPda);

    assert.equal(pollAccount.voteCounts[optionIndex].toNumber(), 1);
    assert.equal(pollAccount.totalVotes.toNumber(), 1);
    assert.equal(voterAccount.hasVoted, true);
  });

  it("Ends the poll", async () => {
    const tx = await program.methods
      .endPoll()
      .accounts({
        poll: pollPda,
        authority: authority.publicKey,
      })
      .signers([authority])
      .rpc();
    console.log("Transaction executed successfully: ", tx);
    const pollAccount = await program.account.poll.fetch(pollPda);
    assert.equal(pollAccount.isActive, false);
  });
});
