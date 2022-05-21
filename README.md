# bytecode-questionaire
Optional Questions: 
Explain some of the ways hashing functions enable blockchain technology? 
- Hashing functions give each block unique identifiers, and immutable block information that help determine valid transactions.
- Hashing functions make it possible for crypto miners to validate transactions and thus form a consensus on the blockchain.
- Hashing functions make it cheaper to store large datasets on the blockchain. And since the data is time-stamped, hashed and     validated by other miners, the data is securely stored and permanent.
- Hashing functions can determine the difficulty levels during mining.
- Hash in eary transaction makes its easier to track transactions on chain. 

Briefly explain Bitcoin's UTXO model of transaction validation
- Anytime a transaction is made, users determine the inputs for UTXOs which will consumed or spent. Next the user is obliged to provide their digital signature to confirm their ownership, then the transaction results in outputs. 

What is the structure of a Block in Bitcoin and how does it relate to the 'blockchain'
- The block is made of a blockheader, containing metadata, followed by a number of transactions
### Block contains:
- Timestamp
- Block number
- Difficulty
- mixHash
- Parent Hash
- Transaction List
- State root
- Nonce

What problem/s are POW/ POS trying to solve? Discuss/compare (byzantine fault tolerance,
reaching a single consensus on a p2p network)
- POW and POS proves that you have spent computational resources to add new blocks in the shared network. The participants maintain a synchronised state and agree on the precise history of transactions.



