## Documentation for Models

[TODO: Add documentation for models]

## Documentation for Common Functionalities

Connectors implementing the Dataspace Protocol may operate on different versions. Therefore, it is necessary that they can discover the supported versions of each other reliably and unambiguously. 
Each Connector must expose information of at least one Dataspace Protocol Version it supports. The specifics of how this information is obtained its defined by specific protocol bindings.

A Connector must respond to a respective request by providing a JSON-LD object containing an array of supported versions with at least one item. 
The item connects the version tag (version attribute) with the absolute URL path segment of the root path for all endpoints of this version. 
The following example specifies that this Connector offers version 1.0 endpoints at ```<host>/some/path/v1```.

```json
{
  "@context": "https://w3id.org/dspace/2024/1/context.json",
  "protocolVersions": [
    {
      "version": "1.0",
      "path": "/some/path/v1"
    }
  ]
}
```

This data object must comply to the [JSON Schema](https://github.com/International-Data-Spaces-Association/ids-specification/blob/main/common/schema/version-schema.json) and the [SHACL Shape](https://github.com/International-Data-Spaces-Association/ids-specification/blob/main/common/shape/version-shape.ttl).

The requesting Connector may select from the endpoints in the response. 
If the Connector can't identify a matching Dataspace Protocol Version, it must terminate the communication.

See [Common Functionalities Definition](https://docs.internationaldataspaces.org/ids-knowledgebase/v/dataspace-protocol/common-functionalities) for more information.

## Documentation for Catalog

The Catalog Protocol defines how a Catalog is requested from a Catalog Service by a Consumer using an abstract message exchange format. 
The concrete message exchange wire format is defined in the binding specifications.

See [Catalog Definition](../dsp_api/src/catalog/README.md) for more information.

## Documentation for Contract Negotiations

A Contract Negotiation (CN) involves two parties, a Provider that offers one or more Datasets under a usage contract and Consumer that requests Datasets.
A CN is uniquely identified through an IRI. Each CN requires a newly generated IRI, which may not be used in a CN after a terminal state has been reached.
A CN progresses through a series of states, which are tracked by the Provider and Consumer using messages.
A CN transitions to a state in response to an acknowledged message from the counter-party.
Both parties have the same state of the CN. In case the states differ, the CN is terminated and a new CN has to be initiated.

See [Contract Negotiation Definition](../dsp_api/src/contract_negotiation/README.md) for more information.