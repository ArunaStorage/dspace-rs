## Documentation for API Endpoints

| Class                         | Method                                                                                                               | HTTP request                                      | Note       |
|-------------------------------|----------------------------------------------------------------------------------------------------------------------|---------------------------------------------------|------------|
| *ApplicationObservabilityApi* | [**check_health**](../../docs/edc_client/ApplicationObservabilityApi.md#check_health)                                | **GET** /check/health                             | deprecated |
| *ApplicationObservabilityApi* | [**get_liveness**](../../docs/edc_client/ApplicationObservabilityApi.md#get_liveness)                                | **GET** /check/liveness                           | deprecated |
| *ApplicationObservabilityApi* | [**get_readiness**](../../docs/edc_client/ApplicationObservabilityApi.md#get_readiness)                              | **GET** /check/readiness                          | deprecated |
| *ApplicationObservabilityApi* | [**get_startup**](../../docs/edc_client/ApplicationObservabilityApi.md#get_startup)                                  | **GET** /check/startup                            | deprecated |
| *AssetApi*                    | [**create_asset**](../../docs/edc_client/AssetApi.md#create_asset)                                                   | **POST** /v3/assets                               |            |
| *AssetApi*                    | [**get_asset**](../../docs/edc_client/AssetApi.md#get_asset)                                                         | **GET** /v3/assets/{id}                           |            |
| *AssetApi*                    | [**remove_asset**](../../docs/edc_client/AssetApi.md#remove_asset)                                                   | **DELETE** /v3/assets/{id}                        |            |
| *AssetApi*                    | [**request_assets**](../../docs/edc_client/AssetApi.md#request_assets)                                               | **POST** /v3/assets/request                       |            |
| *AssetApi*                    | [**update_asset**](../../docs/edc_client/AssetApi.md#update_asset)                                                   | **PUT** /v3/assets                                |            |
| *CatalogApi*                  | [**get_dataset**](../../docs/edc_client/CatalogApi.md#get_dataset)                                                   | **POST** /v2/catalog/dataset/request              |            |
| *CatalogApi*                  | [**request_catalog**](../../docs/edc_client/CatalogApi.md#request_catalog)                                           | **POST** /v2/catalog/request                      |            |
| *ContractAgreementApi*        | [**get_agreement_by_id**](../../docs/edc_client/ContractAgreementApi.md#get_agreement_by_id)                         | **GET** /v2/contractagreements/{id}               |            |
| *ContractAgreementApi*        | [**get_negotiation_by_agreement_id**](../../docs/edc_client/ContractAgreementApi.md#get_negotiation_by_agreement_id) | **GET** /v2/contractagreements/{id}/negotiation   |            |
| *ContractAgreementApi*        | [**query_all_agreements**](../../docs/edc_client/ContractAgreementApi.md#query_all_agreements)                       | **POST** /v2/contractagreements/request           |            |
| *ContractDefinitionApi*       | [**create_contract_definition**](../../docs/edc_client/ContractDefinitionApi.md#create_contract_definition)          | **POST** /v2/contractdefinitions                  |            |
| *ContractDefinitionApi*       | [**delete_contract_definition**](../../docs/edc_client/ContractDefinitionApi.md#delete_contract_definition)          | **DELETE** /v2/contractdefinitions/{id}           |            |
| *ContractDefinitionApi*       | [**get_contract_definition**](../../docs/edc_client/ContractDefinitionApi.md#get_contract_definition)                | **GET** /v2/contractdefinitions/{id}              |            |
| *ContractDefinitionApi*       | [**query_all_contract_definitions**](../../docs/edc_client/ContractDefinitionApi.md#query_all_contract_definitions)  | **POST** /v2/contractdefinitions/request          |            |
| *ContractDefinitionApi*       | [**update_contract_definition**](../../docs/edc_client/ContractDefinitionApi.md#update_contract_definition)          | **PUT** /v2/contractdefinitions                   |            |
| *ContractNegotiationApi*      | [**get_agreement_for_negotiation**](../../docs/edc_client/ContractNegotiationApi.md#get_agreement_for_negotiation)   | **GET** /v2/contractnegotiations/{id}/agreement   |            |
| *ContractNegotiationApi*      | [**get_negotiation**](../../docs/edc_client/ContractNegotiationApi.md#get_negotiation)                               | **GET** /v2/contractnegotiations/{id}             |            |
| *ContractNegotiationApi*      | [**get_negotiation_state**](../../docs/edc_client/ContractNegotiationApi.md#get_negotiation_state)                   | **GET** /v2/contractnegotiations/{id}/state       |            |
| *ContractNegotiationApi*      | [**initiate_contract_negotiation**](../../docs/edc_client/ContractNegotiationApi.md#initiate_contract_negotiation)   | **POST** /v2/contractnegotiations                 |            |
| *ContractNegotiationApi*      | [**query_negotiations**](../../docs/edc_client/ContractNegotiationApi.md#query_negotiations)                         | **POST** /v2/contractnegotiations/request         |            |
| *ContractNegotiationApi*      | [**terminate_negotiation**](../../docs/edc_client/ContractNegotiationApi.md#terminate_negotiation)                   | **POST** /v2/contractnegotiations/{id}/terminate  |            |
| *DataplaneSelectorApi*        | [**add_entry**](../../docs/edc_client/DataplaneSelectorApi.md#add_entry)                                             | **POST** /v2/dataplanes                           | deprecated |
| *DataplaneSelectorApi*        | [**find**](../../docs/edc_client/DataplaneSelectorApi.md#find)                                                       | **POST** /v2/dataplanes/select                    | deprecated |
| *DataplaneSelectorApi*        | [**get_all**](../../docs/edc_client/DataplaneSelectorApi.md#get_all)                                                 | **GET** /v2/dataplanes                            |            |
| *EDRCacheApi*                 | [**delete_edr**](../../docs/edc_client/EDRCacheApi.md#delete_edr)                                                    | **DELETE** /v1/edrs/{transferProcessId}           |            |
| *EDRCacheApi*                 | [**get_edr_data_address**](../../docs/edc_client/EDRCacheApi.md#get_edr_data_address)                                | **POST** /v1/edrs/{transferProcessId}/dataaddress |            |
| *EDRCacheApi*                 | [**query_edrs**](../../docs/edc_client/EDRCacheApi.md#query_edrs)                                                    | **GET** /v1/edrs/request                          |            |
| *HttpProvisionerWebhookApi*   | [**call_deprovision_webhook**](../../docs/edc_client/HttpProvisionerWebhookApi.md#call_deprovision_webhook)          | **POST** /callback/{processId}/deprovision        |            |
| *HttpProvisionerWebhookApi*   | [**call_provision_webhook**](../../docs/edc_client/HttpProvisionerWebhookApi.md#call_provision_webhook)              | **POST** /callback/{processId}/provision          |            |
| *PolicyDefinitionApi*         | [**create_policy_definition**](../../docs/edc_client/PolicyDefinitionApi.md#create_policy_definition)                | **POST** /v2/policydefinitions                    |            |
| *PolicyDefinitionApi*         | [**delete_policy_definition**](../../docs/edc_client/PolicyDefinitionApi.md#delete_policy_definition)                | **DELETE** /v2/policydefinitions/{id}             |            |
| *PolicyDefinitionApi*         | [**get_policy_definition**](../../docs/edc_client/PolicyDefinitionApi.md#get_policy_definition)                      | **GET** /v2/policydefinitions/{id}                |            |
| *PolicyDefinitionApi*         | [**query_policy_definitions**](../../docs/edc_client/PolicyDefinitionApi.md#query_policy_definitions)                | **POST** /v2/policydefinitions/request            |            |
| *PolicyDefinitionApi*         | [**update_policy_definition**](../../docs/edc_client/PolicyDefinitionApi.md#update_policy_definition)                | **PUT** /v2/policydefinitions/{id}                |            |
| *SecretApi*                   | [**create_secret**](../../docs/edc_client/SecretApi.md#create_secret)                                                | **POST** /v1/secrets                              |            |
| *SecretApi*                   | [**delete_secret**](../../docs/edc_client/SecretApi.md#delete_secret)                                                | **DELETE** /v1/secrets/{id}                       |            |
| *SecretApi*                   | [**get_secret**](../../docs/edc_client/SecretApi.md#get_secret)                                                      | **GET** /v1/secrets/{id}                          |            |
| *SecretApi*                   | [**update_secret**](../../docs/edc_client/SecretApi.md#update_secret)                                                | **PUT** /v1/secrets                               |            |
| *TransferProcessApi*          | [**deprovision_transfer_process**](../../docs/edc_client/TransferProcessApi.md#deprovision_transfer_process)         | **POST** /v2/transferprocesses/{id}/deprovision   |            |
| *TransferProcessApi*          | [**get_transfer_process**](../../docs/edc_client/TransferProcessApi.md#get_transfer_process)                         | **GET** /v2/transferprocesses/{id}                |            |
| *TransferProcessApi*          | [**get_transfer_process_state**](../../docs/edc_client/TransferProcessApi.md#get_transfer_process_state)             | **GET** /v2/transferprocesses/{id}/state          |            |
| *TransferProcessApi*          | [**initiate_transfer_process**](../../docs/edc_client/TransferProcessApi.md#initiate_transfer_process)               | **POST** /v2/transferprocesses                    |            |
| *TransferProcessApi*          | [**query_transfer_processes**](../../docs/edc_client/TransferProcessApi.md#query_transfer_processes)                 | **POST** /v2/transferprocesses/request            |            |
| *TransferProcessApi*          | [**resume_transfer_process**](../../docs/edc_client/TransferProcessApi.md#resume_transfer_process)                   | **POST** /v2/transferprocesses/{id}/resume        |            |
| *TransferProcessApi*          | [**suspend_transfer_process**](../../docs/edc_client/TransferProcessApi.md#suspend_transfer_process)                 | **POST** /v2/transferprocesses/{id}/suspend       |            |
| *TransferProcessApi*          | [**terminate_transfer_process**](../../docs/edc_client/TransferProcessApi.md#terminate_transfer_process)             | **POST** /v2/transferprocesses/{id}/terminate     |            | 