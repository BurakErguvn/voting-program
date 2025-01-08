
# Voting Program

This project is a Solana **Voting Program** implemented using the **Anchor** framework. The program allows users to create polls, cast votes, and close polls in a decentralized environment. It is written in **Rust** and uses the **Anchor** framework for efficient program development and testing on the Solana blockchain.

## Features

- **Create a Poll**: Initialize a poll with a title and up to 5 options. Each poll is associated with an authority (the creator).
- **Cast a Vote**: Users can vote on one of the available options in an active poll. Each user can vote only once per poll.
- **End a Poll**: The poll creator can close the poll to prevent further voting.
- **Account Validation**: All accounts (polls and voters) are validated with PDAs (Program Derived Addresses).

## Requirements

To run this project, you'll need the following tools:
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Anchor Framework](https://project-serum.github.io/anchor/getting-started/introduction.html)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)

## Setup

1. **Clone the Project**:
   ```bash
   git clone https://github.com/your-username/voting-program.git
   cd voting-program
   ```

2. **Install Dependencies**:
   ```bash
   yarn install
   ```

3. **Build the Project**:
   ```bash
   anchor build
   ```

4. **Start the Test Validator**:
   ```bash
   solana-test-validator
   ```

5. **Run Anchor Tests**:
   ```bash
   anchor test
   ```

## Usage

### Anchor Methods

1. **Initialize Poll**
   - `initialize_poll()`: Initializes a new poll.
   
2. **Voting**
   - `cast_vote()`: Allows a user to vote on a poll.
   
3. **Finishing the Poll**
   - `end_poll()`: Ends an active poll.

### Tests

The tests are written using `mocha` and Anchor's testing tools. To run the tests:

```bash
anchor test
```

#### Test Coverage:
- Tests the creation of a poll with a title and options.
- Tests voting for a valid option and ensures only one vote per user.
- Tests the closure of a poll by its creator.

## Security Precautions

1. **Do Not Share Admin Key**: The admin key used in the tests should remain confidential.
2. **Exclude Private Keys (.json)**: Ensure you add sensitive files like Solana wallet `.json` files to `.gitignore` to prevent accidental exposure.

## License

This project is licensed under the [MIT License](LICENSE).

---
