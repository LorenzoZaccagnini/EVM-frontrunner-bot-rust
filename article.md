# Intercept pending transactions

Transactions on blockchain are not instant. They are pending until they are confirmed by the network. This is a security feature of the blockchain. It is not possible to change the history of the blockchain. This is why it is important to wait for the transaction to be confirmed before sending another transaction.

## Transaction lifecycle

An Ethereum transaction lifecycle is as follows:

1. The transaction is created and signed by the sender.
2. The transaction is broadcasted to the network.
3. The transaction is pending until it is confirmed by the network.
4. The transaction is confirmed by the network.

The transaction is confirmed when it is included in a block. On Ethereum proof of stake network, the block is mined by a validator. The validator is a node that is running the Ethereum client and is participating in the consensus. The validator is selected randomly from the network. The validator is selected based on the stake that the validator has in the network.

## The Mempool

The mempool is a pool of pending transactions. The transactions are pending until they are confirmed by the network. On Ethereum we don't have a single universal mempool. Each node has its own mempool. Even different clients use different jargon for the mempool.

- On Geth the mempool is called the transaction pool.
- On Parity the mempool is called the transaction queue.

## Why intercept pending transactions?

There are many reasons and on of them is money, some bots frontrun pending transactions to make a profit. It's possibile to frontrun any transaction by increasing the gas price. The gas price is the amount of money that the sender is willing to pay for the transaction to be confirmed. The higher the gas price, the higher the priority of the transaction. The transaction with the highest gas price is confirmed first.

Let's make an example, you are trading a token on Uniswap and you want to buy 100 tokens. You set the gas price to 10 Gwei and the transaction is pending. A bot sees your transaction in the pool and increases the gas price to 20 Gwei. Your transaction is now pending and the bot's transaction is confirmed before yours. The bot knows that your transaction will be executed and he will sell the tokens to you at a higher price. This is called frontrunning.

## Do you want a sandwich?

The Ethereum network is a public network and anyone can see the pending transactions. It is possible to intercept pending transactions and make a profit. The bot will increase the gas price of the transaction and will execute the transaction before the original sender. **The sandwich attack is a type of frontrunning attack**, [it's about placing a trade before and after a victim trade, in order to exploit the slippage that has been created](https://github.com/Defi-Cartel/salmonella). The bot will buy the token and sell it to the original sender at a higher price. The bot will make a profit by selling the token at a higher price.

In a future article we will see how to make a sandwich attack, but for now let's see how to intercept and decode pending transactions.
