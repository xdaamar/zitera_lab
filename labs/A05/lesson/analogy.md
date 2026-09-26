# The Bank Check Analogy

Imagine writing a paper check:

- Pay to the order of: `Alice`
- Amount: `$50`
- Memo: `For gardening work`

Now imagine you leave the "Memo" line open, and Alice writes:
`For gardening work; and also transfer $1,000,000 to Alice's offshore account.`

When the bank teller reads the check, if they treat the entire check as a continuous instruction stream rather than strictly separated fields, they execute the command written inside the memo line!

In SQL Injection, the database query is like that check:
The application intends user input to be purely data (a name or keyword), but because it glues strings together, user input becomes executable database code.
