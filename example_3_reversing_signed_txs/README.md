## Not strictly a REVM related example

If we're working in a production environment, might be sometimes where it's easier to send signed transactions to be simulated rather than creating additional code to send unsigned transactions instead.

Here's an example of how to reverse signed hex data into an unsigned `TransactionRequest` struct