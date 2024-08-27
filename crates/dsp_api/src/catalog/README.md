## [Catalog](https://docs.internationaldataspaces.org/ids-knowledgebase/v/dataspace-protocol/catalog/catalog.protocol)

The Catalog Protocol defines how a Catalog is requested from a Catalog Service by a Consumer using an abstract message exchange format. 
The concrete message exchange wire format is defined in the binding specifications.

### DCAT Vocabulary Mapping

This section describes how the DSP Information Model maps to DCAT resources.

* **Dataset**:<br>
  A Dataset is a DCAT Dataset with the following attributes:<br>
  <br>
  **odrl:hasPolicy**<br>
  <br>
  A Dataset must have 1..N hasPolicy attributes that contain an ODRL Offer defining the Usage Policy associated with the Catalog. 
  Offers must NOT contain any explicit target attributes. The target of an Offer is the associated Dataset. 
  This is in line with the semantics of hasPolicy as defined in the ODRL Information Model, explaining that the subject (here the Dataset) 
  is automatically the target of each Rule. To prevent conflicts, the target attribute must not be set explicitly, for example, in the Offer or Rules.<br>
  <br>
* **Distribution**<br>
  A Dataset may contain 0..N DCAT Distributions. Each distribution must have at least one DataService which specifies where the distribution is obtained. 
  Specifically, a DataService specifies the endpoint for initiating a Contract Negotiation and Transfer Process.<br>
  <br>
  A Distribution may have 0..N hasPolicy attributes that contain an ODRL Offer defining the Usage Policy associated with the Dataset and this explicit Distribution. 
  Offers must NOT contain any target attributes. The target of an Offer is the Dataset that contains the distribution.<br>
  <br>
  Support for hasPolicy attributes on a Distribution is optional. Implementations may choose not to support this feature, 
  in which case they should return an appropriate error message to clients.<br>
  <br>
* **Data Service**<br>
  A Data Service may specify an endpoint supporting the Dataspace Protocol such as a Connector.<br>
  <br>
  **dspace:dataServiceType**<br>
  If the Data Service refers to an endpoint that supports the Dataspace Protocol, it must include the property ```dspace:dataServiceType```:

  | **Category** | **Description**                                                           |
  |--------------|---------------------------------------------------------------------------|
  | Definition   | Specifies the service type                                                |
  | Domain       | dcat:DataService                                                          |
  | Type         | xsd:string                                                                |
  | Note         | The value of this field is left intentionally open for future extensions. |

  The following table lists well-know endpoint types:<br>

  | **Value**        | **Description**      |
  |------------------|----------------------|
  | dspace:connector | A Connector endpoint |

  **dcat:servesDataset**<br>
  Note that the property ```dcat:servesDataset``` should be omitted from the DataService since Datasets are included as top-level entries. 
  Clients are not required to process the contents of ```dcat:servesDataset```.<br>
  <br>
* **Participant Id**<br>
  The identifier of the participant providing the Catalog is specified using the ```dspace:participantId``` attribute on that DCAT Catalog.

### DCAT and ODRL Profiles

The Catalog is a DCAT Catalog with the following restrictions:

1. Each ODRL Offer must be unique to a Dataset since the target of the Offer is derived from its enclosing context.
2. A Catalog must not have an ```odrl:hasPolicy``` attribute, since it is not intended to negotiate on the access to Catalog objects. An implementation might however regulate the visibility and/or the content of its Catalog dependent of the requester.

### Message Types

All messages must be serialized in JSON-LD compact form as specified in the [JSON-LD 1.1 Processing Algorithms and API](https://www.w3.org/TR/json-ld11-api/#compaction-algorithms). 
Further Dataspace specifications may define additional optional serialization formats.

### Technical Considerations

* **Queries and Filter Expressions**<br>
  A Catalog Service may support Catalog queries or filter expressions as an implementation-specific feature. 
  However, it is expected that query capabilities will be implemented by the Consumer against the results of a Catalog Request Message,
  as the latter is an RDF vocabulary. Client-side querying can be scaled by periodically crawling the Provider's Catalog Services, 
  caching the results, and executing queries against the locally-stored Catalogs.<br>
  <br>
* **Replication Protocol**<br>
  The Catalog Protocol is designed to be used by federated services without the need for a replication protocol. 
  Each Consumer is responsible for issuing requests to 1..N Catalog Services, and managing the results. 
  It follows that a specific replication protocol is not needed, or more precisely, each Consumer replicates data from 
  catalog services by issuing Catalog Request Messages.<br>
  <br>
  The discovery protocol adopted by a particular Dataspace defines how a Consumer discovers Catalog Services.<br>
  <br>
* **Security**<br>
  It is expected (although not required) that Catalog Services implement access control. 
  A Catalog as well as individual Datasets may be restricted to trusted parties. 
  The Catalog Service may require Consumers to include a security token along with a Catalog Request Message. 
  The specifics of how this is done can be found in the relevant protocol binding, e.g., [Catalog HTTPS Binding](https://docs.internationaldataspaces.org/ids-knowledgebase/v/dataspace-protocol/catalog/catalog.binding.https). 
  The semantics of such tokens are not part of this specification.<br>
  <br>
  **The Proof Metadata Endpoint**<br>
  <br>
  When a Catalog contains protected Datasets the Provider has two options: include all Datasets in the Catalog response 
  and restrict access when a contract is negotiated; or, require one or more proofs when the Catalog Request is made and 
  filter the Datasets accordingly. The latter option requires a mechanism for clients to discover the type of proofs that 
  may be presented at request time. The specifics of proof types and presenting a proof during a Catalog request is outside 
  the scope of the Dataspace Protocol. However, Catalog Protocol bindings should define a proof data endpoint for obtaining 
  this information.<br>
  <br>
* **Catalog Brokers**<br>
  A Dataspace may include Catalog Brokers. A Catalog Broker is a Consumer that has trusted access to 1..N upstream Catalog Services 
  and advertises their respective Catalogs as a single Catalog Service. The broker is expected to honor upstream access control requirements.