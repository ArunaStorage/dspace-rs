## Documentation for Models

[TODO: Add documentation for models]

## Documentation for Contract Negotiations

A Contract Negotiation (CN) involves two parties, a Provider that offers one or more Datasets under a usage contract and Consumer that requests Datasets.
A CN is uniquely identified through an IRI. Each CN requires a newly generated IRI, which may not be used in a CN after a terminal state has been reached.
A CN progresses through a series of states, which are tracked by the Provider and Consumer using messages.
A CN transitions to a state in response to an acknowledged message from the counter-party.
Both parties have the same state of the CN. In case the states differ, the CN is terminated and a new CN has to be initiated.

See [Contract Negotiation Definition](../dsp_api/src/contract_negotiation/README.md) for more information.