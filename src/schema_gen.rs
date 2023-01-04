// Copyright 2023 - The Bardasz Group & etp-rs authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//  http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// ETP Schemas from Energistics Organisation are licenced under the Energistics Licence.
// You may not use those schema's except in compliance with the license.
// You can find a copy of the License at: schema/ENERGISTICS_LICENCE
//
// The following Energistics (c) products were used in the creation of this work: ETP 1.2 Specification.
//
// Author: Mark Farnan

// This File is Auto-generated with rsgen-avro,  then manually edited

// ------------------------------------------------------------------------------------------------------------
// Things that need changing after pasting a Generated output from rsgen-Avro
// ------------------------------------------------------------------------------------------------------------
// command line:  rsgen-avro --union-deser "./**/*.avsc" struct.rs
// 1. Remove all 'serde_bytes'
// 2. Remove the Option <> from DataValue Item
// 3. Rename UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray
//    to: DataValueEnum
// 4. Add 'Null' to the top of DataValueEnum
// 5. Rename UnionArrayOfBooleanArrayOfIntArrayOfLongArrayOfFloatArrayOfDoubleArrayOfStringBytes to AnyArrayUnion
// 6. Put #[serde(with = "serde_bytes")]  above EVERY use of Uuid   (search for ': Uuid,' some 27 entries.)
// 7. Rename 'Self' to 'Self_'  in ContextScopeKind   (Self is a reserved word)
// 8. Remove Types ChunkSq and ChunkSn  (Just use Schema/Type 'Chunk' for all 3 protocol uses)
// 9. Update Protocol enum with absolute protocol numbers. (See struct below)

// pub enum Protocol {
//     Core = 0,
//     ChannelStreaming = 1,
//     ChannelDataFrame = 2,
//     Discovery = 3,
//     Store = 4,
//     StoreNotification = 5,
//     GrowingObject = 6,
//     GrowingObjectNotification = 7,
//     #[serde(rename = "DEPRECATED_8")]
//     Deprecated8 = 8,
//     DataArray = 9,
//     #[serde(rename = "RESERVED_10")]
//     Reserved10 = 10,
//     #[serde(rename = "RESERVED_11")]
//     Reserved11 = 11,
//     #[serde(rename = "RESERVED_12")]
//     Reserved12 = 12,
//     DiscoveryQuery = 13,
//     StoreQuery = 14,
//     #[serde(rename = "RESERVED_15")]
//     Reserved15 = 15,
//     GrowingObjectQuery = 16,
//     #[serde(rename = "RESERVED_17")]
//     Reserved17 = 17,
//     Transaction = 18,
//     #[serde(rename = "RESERVED_19")]
//     Reserved19 = 19,
//     #[serde(rename = "RESERVED_20")]
//     Reserved20 = 20,
//     ChannelSubscribe = 21,
//     ChannelDataLoad = 22,
//     #[serde(rename = "RESERVED_23")]
//     Reserved23 = 23,
//     Dataspace = 24,
//     SupportedTypes = 25,
// }

pub type Uuid = [u8; 16];

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct StartTransactionResponse {
    #[serde(rename = "transactionUuid")]
    #[serde(with = "serde_bytes")]
    pub transaction_uuid: Uuid,
    #[serde(default = "default_starttransactionresponse_successful")]
    pub successful: bool,
    #[serde(rename = "failureReason")]
    #[serde(default = "default_starttransactionresponse_failure_reason")]
    pub failure_reason: String,
}

#[inline(always)]
fn default_starttransactionresponse_successful() -> bool {
    true
}

#[inline(always)]
fn default_starttransactionresponse_failure_reason() -> String {
    "".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct StartTransaction {
    #[serde(rename = "readOnly")]
    pub read_only: bool,
    #[serde(default = "default_starttransaction_message")]
    pub message: String,
    #[serde(rename = "dataspaceUris")]
    #[serde(default = "default_starttransaction_dataspace_uris")]
    pub dataspace_uris: Vec<String>,
}

#[inline(always)]
fn default_starttransaction_message() -> String {
    "".to_owned()
}

#[inline(always)]
fn default_starttransaction_dataspace_uris() -> Vec<String> {
    vec!["".to_owned()]
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct RollbackTransactionResponse {
    #[serde(rename = "transactionUuid")]
    #[serde(with = "serde_bytes")]
    pub transaction_uuid: Uuid,
    #[serde(default = "default_rollbacktransactionresponse_successful")]
    pub successful: bool,
    #[serde(rename = "failureReason")]
    #[serde(default = "default_rollbacktransactionresponse_failure_reason")]
    pub failure_reason: String,
}

#[inline(always)]
fn default_rollbacktransactionresponse_successful() -> bool {
    true
}

#[inline(always)]
fn default_rollbacktransactionresponse_failure_reason() -> String {
    "".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct RollbackTransaction {
    #[serde(rename = "transactionUuid")]
    #[serde(with = "serde_bytes")]
    pub transaction_uuid: Uuid,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct CommitTransactionResponse {
    #[serde(rename = "transactionUuid")]
    #[serde(with = "serde_bytes")]
    pub transaction_uuid: Uuid,
    #[serde(default = "default_committransactionresponse_successful")]
    pub successful: bool,
    #[serde(rename = "failureReason")]
    #[serde(default = "default_committransactionresponse_failure_reason")]
    pub failure_reason: String,
}

#[inline(always)]
fn default_committransactionresponse_successful() -> bool {
    true
}

#[inline(always)]
fn default_committransactionresponse_failure_reason() -> String {
    "".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct CommitTransaction {
    #[serde(rename = "transactionUuid")]
    #[serde(with = "serde_bytes")]
    pub transaction_uuid: Uuid,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetSupportedTypesResponse {
    #[serde(rename = "supportedTypes")]
    pub supported_types: Vec<SupportedType>,
}

#[inline(always)]
fn default_getsupportedtypesresponse_supported_types() -> Vec<SupportedType> {
    vec![]
}

impl Default for GetSupportedTypesResponse {
    fn default() -> GetSupportedTypesResponse {
        GetSupportedTypesResponse {
            supported_types: default_getsupportedtypesresponse_supported_types(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetSupportedTypes {
    pub uri: String,
    pub scope: ContextScopeKind,
    #[serde(rename = "returnEmptyTypes")]
    #[serde(default = "default_getsupportedtypes_return_empty_types")]
    pub return_empty_types: bool,
    #[serde(rename = "countObjects")]
    #[serde(default = "default_getsupportedtypes_count_objects")]
    pub count_objects: bool,
}

#[inline(always)]
fn default_getsupportedtypes_return_empty_types() -> bool {
    false
}

#[inline(always)]
fn default_getsupportedtypes_count_objects() -> bool {
    false
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct FindDataObjectsResponse {
    #[serde(rename = "dataObjects")]
    #[serde(default = "default_finddataobjectsresponse_data_objects")]
    pub data_objects: Vec<DataObject>,
    #[serde(rename = "serverSortOrder")]
    pub server_sort_order: String,
}

#[inline(always)]
fn default_finddataobjectsresponse_data_objects() -> Vec<DataObject> {
    vec![]
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct FindDataObjects {
    pub context: ContextInfo,
    pub scope: ContextScopeKind,
    #[serde(rename = "storeLastWriteFilter")]
    pub store_last_write_filter: Option<i64>,
    #[serde(rename = "activeStatusFilter")]
    pub active_status_filter: Option<ActiveStatusKind>,
    #[serde(default = "default_finddataobjects_format")]
    pub format: String,
}

#[inline(always)]
fn default_finddataobjects_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct UnsubscribeNotifications {
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct UnsolicitedStoreNotifications {
    pub subscriptions: Vec<SubscriptionInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionEnded {
    pub reason: String,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubscribeNotificationsResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubscribeNotifications {
    pub request: ::std::collections::HashMap<String, SubscriptionInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ObjectDeleted {
    pub uri: String,
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ObjectChanged {
    pub change: ObjectChange,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ObjectActiveStatusChanged {
    #[serde(rename = "activeStatus")]
    pub active_status: ActiveStatusKind,
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    pub resource: Resource,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ObjectAccessRevoked {
    pub uri: String,
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataObjectsResponse {
    pub success: ::std::collections::HashMap<String, PutResponse>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataObjects {
    #[serde(rename = "dataObjects")]
    pub data_objects: ::std::collections::HashMap<String, DataObject>,
    #[serde(rename = "pruneContainedObjects")]
    #[serde(default = "default_putdataobjects_prune_contained_objects")]
    pub prune_contained_objects: bool,
}

#[inline(always)]
fn default_putdataobjects_prune_contained_objects() -> bool {
    false
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetDataObjectsResponse {
    #[serde(rename = "dataObjects")]
    pub data_objects: ::std::collections::HashMap<String, DataObject>,
}

#[inline(always)]
fn default_getdataobjectsresponse_data_objects() -> ::std::collections::HashMap<String, DataObject>
{
    ::std::collections::HashMap::new()
}

impl Default for GetDataObjectsResponse {
    fn default() -> GetDataObjectsResponse {
        GetDataObjectsResponse {
            data_objects: default_getdataobjectsresponse_data_objects(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetDataObjects {
    pub uris: ::std::collections::HashMap<String, String>,
    #[serde(default = "default_getdataobjects_format")]
    pub format: String,
}

#[inline(always)]
fn default_getdataobjects_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DeleteDataObjectsResponse {
    #[serde(rename = "deletedUris")]
    pub deleted_uris: ::std::collections::HashMap<String, ArrayOfString>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DeleteDataObjects {
    pub uris: ::std::collections::HashMap<String, String>,
    #[serde(rename = "pruneContainedObjects")]
    #[serde(default = "default_deletedataobjects_prune_contained_objects")]
    pub prune_contained_objects: bool,
}

#[inline(always)]
fn default_deletedataobjects_prune_contained_objects() -> bool {
    false
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Chunk {
    #[serde(rename = "blobId")]
    #[serde(with = "serde_bytes")]
    pub blob_id: Uuid,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub r#final: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct FindPartsResponse {
    pub uri: String,
    #[serde(rename = "serverSortOrder")]
    pub server_sort_order: String,
    #[serde(default = "default_findpartsresponse_format")]
    pub format: String,
    #[serde(default = "default_findpartsresponse_parts")]
    pub parts: Vec<ObjectPart>,
}

#[inline(always)]
fn default_findpartsresponse_format() -> String {
    "xml".to_owned()
}

#[inline(always)]
fn default_findpartsresponse_parts() -> Vec<ObjectPart> {
    vec![]
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct FindParts {
    pub uri: String,
    #[serde(default = "default_findparts_format")]
    pub format: String,
}

#[inline(always)]
fn default_findparts_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct UnsubscribePartNotification {
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct UnsolicitedPartNotifications {
    pub subscriptions: Vec<SubscriptionInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubscribePartNotificationsResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubscribePartNotifications {
    pub request: ::std::collections::HashMap<String, SubscriptionInfo>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PartsReplacedByRange {
    pub uri: String,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    #[serde(rename = "deletedInterval")]
    pub deleted_interval: IndexInterval,
    #[serde(rename = "includeOverlappingIntervals")]
    pub include_overlapping_intervals: bool,
    #[serde(default = "default_partsreplacedbyrange_format")]
    pub format: String,
    pub parts: Vec<ObjectPart>,
}

#[inline(always)]
fn default_partsreplacedbyrange_format() -> String {
    "".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PartsDeleted {
    pub uri: String,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    pub uids: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PartsChanged {
    pub uri: String,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
    #[serde(rename = "changeKind")]
    pub change_kind: ObjectChangeKind,
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    #[serde(default = "default_partschanged_format")]
    pub format: String,
    pub parts: Vec<ObjectPart>,
}

#[inline(always)]
fn default_partschanged_format() -> String {
    "".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PartSubscriptionEnded {
    pub reason: String,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ReplacePartsByRange {
    pub uri: String,
    #[serde(rename = "deleteInterval")]
    pub delete_interval: IndexInterval,
    #[serde(rename = "includeOverlappingIntervals")]
    pub include_overlapping_intervals: bool,
    #[serde(default = "default_replacepartsbyrange_format")]
    pub format: String,
    pub parts: Vec<ObjectPart>,
}

#[inline(always)]
fn default_replacepartsbyrange_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutPartsResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutParts {
    pub uri: String,
    #[serde(default = "default_putparts_format")]
    pub format: String,
    pub parts: ::std::collections::HashMap<String, ObjectPart>,
}

#[inline(always)]
fn default_putparts_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutGrowingDataObjectsHeaderResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutGrowingDataObjectsHeader {
    #[serde(rename = "dataObjects")]
    pub data_objects: ::std::collections::HashMap<String, DataObject>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetPartsResponse {
    pub uri: String,
    #[serde(default = "default_getpartsresponse_format")]
    pub format: String,
    pub parts: ::std::collections::HashMap<String, ObjectPart>,
}

#[inline(always)]
fn default_getpartsresponse_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetPartsMetadataResponse {
    pub metadata: ::std::collections::HashMap<String, PartsMetadataInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetPartsMetadata {
    pub uris: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetPartsByRangeResponse {
    pub uri: String,
    #[serde(default = "default_getpartsbyrangeresponse_format")]
    pub format: String,
    pub parts: Vec<ObjectPart>,
}

#[inline(always)]
fn default_getpartsbyrangeresponse_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetPartsByRange {
    pub uri: String,
    #[serde(default = "default_getpartsbyrange_format")]
    pub format: String,
    #[serde(rename = "indexInterval")]
    pub index_interval: IndexInterval,
    #[serde(rename = "includeOverlappingIntervals")]
    pub include_overlapping_intervals: bool,
}

#[inline(always)]
fn default_getpartsbyrange_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetParts {
    pub uri: String,
    #[serde(default = "default_getparts_format")]
    pub format: String,
    pub uids: ::std::collections::HashMap<String, String>,
}

#[inline(always)]
fn default_getparts_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetGrowingDataObjectsHeaderResponse {
    #[serde(rename = "dataObjects")]
    pub data_objects: ::std::collections::HashMap<String, DataObject>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetGrowingDataObjectsHeader {
    pub uris: ::std::collections::HashMap<String, String>,
    #[serde(default = "default_getgrowingdataobjectsheader_format")]
    pub format: String,
}

#[inline(always)]
fn default_getgrowingdataobjectsheader_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetChangeAnnotationsResponseGo {
    pub changes: ::std::collections::HashMap<String, ChangeResponseInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetChangeAnnotationsGo {
    #[serde(rename = "sinceChangeTime")]
    pub since_change_time: i64,
    pub uris: ::std::collections::HashMap<String, String>,
    #[serde(rename = "latestOnly")]
    #[serde(default = "default_getchangeannotationsgo_latest_only")]
    pub latest_only: bool,
}

#[inline(always)]
fn default_getchangeannotationsgo_latest_only() -> bool {
    false
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DeletePartsResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DeleteParts {
    pub uri: String,
    pub uids: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct FindResourcesResponse {
    #[serde(default = "default_findresourcesresponse_resources")]
    pub resources: Vec<Resource>,
    #[serde(rename = "serverSortOrder")]
    pub server_sort_order: String,
}

#[inline(always)]
fn default_findresourcesresponse_resources() -> Vec<Resource> {
    vec![]
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct FindResources {
    pub context: ContextInfo,
    pub scope: ContextScopeKind,
    #[serde(rename = "storeLastWriteFilter")]
    pub store_last_write_filter: Option<i64>,
    #[serde(rename = "activeStatusFilter")]
    pub active_status_filter: Option<ActiveStatusKind>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetResourcesResponse {
    pub resources: Vec<Resource>,
}

#[inline(always)]
fn default_getresourcesresponse_resources() -> Vec<Resource> {
    vec![]
}

impl Default for GetResourcesResponse {
    fn default() -> GetResourcesResponse {
        GetResourcesResponse {
            resources: default_getresourcesresponse_resources(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetResourcesEdgesResponse {
    pub edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetResources {
    pub context: ContextInfo,
    pub scope: ContextScopeKind,
    #[serde(rename = "countObjects")]
    #[serde(default = "default_getresources_count_objects")]
    pub count_objects: bool,
    #[serde(rename = "storeLastWriteFilter")]
    pub store_last_write_filter: Option<i64>,
    #[serde(rename = "activeStatusFilter")]
    pub active_status_filter: Option<ActiveStatusKind>,
    #[serde(rename = "includeEdges")]
    #[serde(default = "default_getresources_include_edges")]
    pub include_edges: bool,
}

#[inline(always)]
fn default_getresources_count_objects() -> bool {
    false
}

#[inline(always)]
fn default_getresources_include_edges() -> bool {
    false
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetDeletedResourcesResponse {
    #[serde(rename = "deletedResources")]
    pub deleted_resources: Vec<DeletedResource>,
}

#[inline(always)]
fn default_getdeletedresourcesresponse_deleted_resources() -> Vec<DeletedResource> {
    vec![]
}

impl Default for GetDeletedResourcesResponse {
    fn default() -> GetDeletedResourcesResponse {
        GetDeletedResourcesResponse {
            deleted_resources: default_getdeletedresourcesresponse_deleted_resources(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetDeletedResources {
    #[serde(rename = "dataspaceUri")]
    pub dataspace_uri: String,
    #[serde(rename = "deleteTimeFilter")]
    pub delete_time_filter: Option<i64>,
    #[serde(rename = "dataObjectTypes")]
    #[serde(default = "default_getdeletedresources_data_object_types")]
    pub data_object_types: Vec<String>,
}

#[inline(always)]
fn default_getdeletedresources_data_object_types() -> Vec<String> {
    vec![]
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataspacesResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataspaces {
    pub dataspaces: ::std::collections::HashMap<String, Dataspace>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetDataspacesResponse {
    pub dataspaces: Vec<Dataspace>,
}

#[inline(always)]
fn default_getdataspacesresponse_dataspaces() -> Vec<Dataspace> {
    vec![]
}

impl Default for GetDataspacesResponse {
    fn default() -> GetDataspacesResponse {
        GetDataspacesResponse {
            dataspaces: default_getdataspacesresponse_dataspaces(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetDataspaces {
    #[serde(rename = "storeLastWriteFilter")]
    pub store_last_write_filter: Option<i64>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DeleteDataspacesResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DeleteDataspaces {
    pub uris: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutUninitializedDataArraysResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutUninitializedDataArrays {
    #[serde(rename = "dataArrays")]
    pub data_arrays: ::std::collections::HashMap<String, PutUninitializedDataArrayType>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataSubarraysResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataSubarrays {
    #[serde(rename = "dataSubarrays")]
    pub data_subarrays: ::std::collections::HashMap<String, PutDataSubarraysType>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataArraysResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataArrays {
    #[serde(rename = "dataArrays")]
    pub data_arrays: ::std::collections::HashMap<String, PutDataArraysType>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetDataSubarraysResponse {
    #[serde(rename = "dataSubarrays")]
    pub data_subarrays: ::std::collections::HashMap<String, DataArray>,
}

#[inline(always)]
fn default_getdatasubarraysresponse_data_subarrays(
) -> ::std::collections::HashMap<String, DataArray> {
    ::std::collections::HashMap::new()
}

impl Default for GetDataSubarraysResponse {
    fn default() -> GetDataSubarraysResponse {
        GetDataSubarraysResponse {
            data_subarrays: default_getdatasubarraysresponse_data_subarrays(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetDataSubarrays {
    #[serde(rename = "dataSubarrays")]
    pub data_subarrays: ::std::collections::HashMap<String, GetDataSubarraysType>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetDataArraysResponse {
    #[serde(rename = "dataArrays")]
    pub data_arrays: ::std::collections::HashMap<String, DataArray>,
}

#[inline(always)]
fn default_getdataarraysresponse_data_arrays() -> ::std::collections::HashMap<String, DataArray> {
    ::std::collections::HashMap::new()
}

impl Default for GetDataArraysResponse {
    fn default() -> GetDataArraysResponse {
        GetDataArraysResponse {
            data_arrays: default_getdataarraysresponse_data_arrays(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetDataArrays {
    #[serde(rename = "dataArrays")]
    pub data_arrays: ::std::collections::HashMap<String, DataArrayIdentifier>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetDataArrayMetadataResponse {
    #[serde(rename = "arrayMetadata")]
    pub array_metadata: ::std::collections::HashMap<String, DataArrayMetadata>,
}

#[inline(always)]
fn default_getdataarraymetadataresponse_array_metadata(
) -> ::std::collections::HashMap<String, DataArrayMetadata> {
    ::std::collections::HashMap::new()
}

impl Default for GetDataArrayMetadataResponse {
    fn default() -> GetDataArrayMetadataResponse {
        GetDataArrayMetadataResponse {
            array_metadata: default_getdataarraymetadataresponse_array_metadata(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetDataArrayMetadata {
    #[serde(rename = "dataArrays")]
    pub data_arrays: ::std::collections::HashMap<String, DataArrayIdentifier>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct RequestSession {
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[serde(rename = "applicationVersion")]
    pub application_version: String,
    #[serde(rename = "clientInstanceId")]
    //#[serde(serialize_with = "avro_serialize_fixed")]
    #[serde(with = "serde_bytes")]
    pub client_instance_id: Uuid,
    #[serde(rename = "requestedProtocols")]
    pub requested_protocols: Vec<SupportedProtocol>,
    #[serde(rename = "supportedDataObjects")]
    pub supported_data_objects: Vec<SupportedDataObject>,
    #[serde(rename = "supportedCompression")]
    #[serde(default = "default_requestsession_supported_compression")]
    pub supported_compression: Vec<String>,
    #[serde(rename = "supportedFormats")]
    #[serde(default = "default_requestsession_supported_formats")]
    pub supported_formats: Vec<String>,
    #[serde(rename = "currentDateTime")]
    pub current_date_time: i64,
    #[serde(rename = "earliestRetainedChangeTime")]
    pub earliest_retained_change_time: i64,
    #[serde(rename = "serverAuthorizationRequired")]
    #[serde(default = "default_requestsession_server_authorization_required")]
    pub server_authorization_required: bool,
    #[serde(rename = "endpointCapabilities")]
    #[serde(default = "default_requestsession_endpoint_capabilities")]
    pub endpoint_capabilities: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_requestsession_supported_compression() -> Vec<String> {
    vec![]
}

#[inline(always)]
fn default_requestsession_supported_formats() -> Vec<String> {
    vec!["xml".to_owned()]
}

#[inline(always)]
fn default_requestsession_server_authorization_required() -> bool {
    false
}

#[inline(always)]
fn default_requestsession_endpoint_capabilities() -> ::std::collections::HashMap<String, DataValue>
{
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProtocolException {
    pub error: Option<ErrorInfo>,
    #[serde(default = "default_protocolexception_errors")]
    pub errors: ::std::collections::HashMap<String, ErrorInfo>,
}

#[inline(always)]
fn default_protocolexception_errors() -> ::std::collections::HashMap<String, ErrorInfo> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Pong {
    #[serde(rename = "currentDateTime")]
    pub current_date_time: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Ping {
    #[serde(rename = "currentDateTime")]
    pub current_date_time: i64,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct OpenSession {
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[serde(rename = "applicationVersion")]
    pub application_version: String,
    #[serde(rename = "serverInstanceId")]
    #[serde(with = "serde_bytes")]
    pub server_instance_id: Uuid,
    #[serde(rename = "supportedProtocols")]
    pub supported_protocols: Vec<SupportedProtocol>,
    #[serde(rename = "supportedDataObjects")]
    pub supported_data_objects: Vec<SupportedDataObject>,
    #[serde(rename = "supportedCompression")]
    #[serde(default = "default_opensession_supported_compression")]
    pub supported_compression: String,
    #[serde(rename = "supportedFormats")]
    #[serde(default = "default_opensession_supported_formats")]
    pub supported_formats: Vec<String>,
    #[serde(rename = "currentDateTime")]
    pub current_date_time: i64,
    #[serde(rename = "earliestRetainedChangeTime")]
    pub earliest_retained_change_time: i64,
    #[serde(rename = "sessionId")]
    #[serde(with = "serde_bytes")]
    pub session_id: Uuid,
    #[serde(rename = "endpointCapabilities")]
    #[serde(default = "default_opensession_endpoint_capabilities")]
    pub endpoint_capabilities: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_opensession_supported_compression() -> String {
    "".to_owned()
}

#[inline(always)]
fn default_opensession_supported_formats() -> Vec<String> {
    vec!["xml".to_owned()]
}

#[inline(always)]
fn default_opensession_endpoint_capabilities() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct CloseSession {
    pub reason: String,
}

#[inline(always)]
fn default_closesession_reason() -> String {
    "".to_owned()
}

impl Default for CloseSession {
    fn default() -> CloseSession {
        CloseSession {
            reason: default_closesession_reason(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct AuthorizeResponse {
    pub success: bool,
    pub challenges: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Authorize {
    pub authorization: String,
    #[serde(rename = "supplementalAuthorization")]
    pub supplemental_authorization: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct UnsubscribeChannels {
    #[serde(rename = "channelIds")]
    pub channel_ids: ::std::collections::HashMap<String, i64>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionsStopped {
    pub reason: String,
    #[serde(rename = "channelIds")]
    #[serde(default = "default_subscriptionsstopped_channel_ids")]
    pub channel_ids: ::std::collections::HashMap<String, i64>,
}

#[inline(always)]
fn default_subscriptionsstopped_channel_ids() -> ::std::collections::HashMap<String, i64> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubscribeChannelsResponse {
    pub success: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubscribeChannels {
    pub channels: ::std::collections::HashMap<String, ChannelSubscribeInfo>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct RangeReplaced {
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    #[serde(rename = "channelIds")]
    pub channel_ids: Vec<i64>,
    #[serde(rename = "changedInterval")]
    pub changed_interval: IndexInterval,
    pub data: Vec<DataItem>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetRangesResponse {
    pub data: Vec<DataItem>,
}

#[inline(always)]
fn default_getrangesresponse_data() -> Vec<DataItem> {
    vec![]
}

impl Default for GetRangesResponse {
    fn default() -> GetRangesResponse {
        GetRangesResponse {
            data: default_getrangesresponse_data(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetRanges {
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
    #[serde(rename = "channelRanges")]
    pub channel_ranges: Vec<ChannelRangeInfo>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct GetChannelMetadataResponse {
    pub metadata: ::std::collections::HashMap<String, ChannelMetadataRecord>,
}

#[inline(always)]
fn default_getchannelmetadataresponse_metadata(
) -> ::std::collections::HashMap<String, ChannelMetadataRecord> {
    ::std::collections::HashMap::new()
}

impl Default for GetChannelMetadataResponse {
    fn default() -> GetChannelMetadataResponse {
        GetChannelMetadataResponse {
            metadata: default_getchannelmetadataresponse_metadata(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetChannelMetadata {
    pub uris: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetChangeAnnotationsResponse {
    pub changes: ::std::collections::HashMap<String, ChangeResponseInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetChangeAnnotations {
    pub channels: ::std::collections::HashMap<String, ChannelChangeRequestInfo>,
    #[serde(rename = "latestOnly")]
    #[serde(default = "default_getchangeannotations_latest_only")]
    pub latest_only: bool,
}

#[inline(always)]
fn default_getchangeannotations_latest_only() -> bool {
    false
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelsTruncated {
    pub channels: Vec<TruncateInfo>,
    #[serde(rename = "changeTime")]
    pub change_time: i64,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelData {
    pub data: Vec<DataItem>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct CancelGetRanges {
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct TruncateChannelsCs {
    pub channels: Vec<TruncateInfo>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelMetadata {
    pub channels: Vec<ChannelMetadataRecord>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelDataCs {
    pub data: Vec<DataItem>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct TruncateChannelsResponse {
    #[serde(rename = "channelsTruncatedTime")]
    pub channels_truncated_time: ::std::collections::HashMap<String, i64>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct TruncateChannels {
    pub channels: ::std::collections::HashMap<String, TruncateInfo>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ReplaceRangeResponse {
    #[serde(rename = "channelChangeTime")]
    pub channel_change_time: i64,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ReplaceRange {
    #[serde(rename = "changedInterval")]
    pub changed_interval: IndexInterval,
    #[serde(rename = "channelIds")]
    pub channel_ids: Vec<i64>,
    pub data: Vec<DataItem>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct OpenChannels {
    pub uris: ::std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct OpenChannelsResponse {
    pub channels: ::std::collections::HashMap<String, OpenChannelInfo>,
}

#[inline(always)]
fn default_openchannelsresponse_channels() -> ::std::collections::HashMap<String, OpenChannelInfo> {
    ::std::collections::HashMap::new()
}

impl Default for OpenChannelsResponse {
    fn default() -> OpenChannelsResponse {
        OpenChannelsResponse {
            channels: default_openchannelsresponse_channels(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct CloseChannels {
    pub id: ::std::collections::HashMap<String, i64>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelsClosed {
    pub reason: String,
    pub id: ::std::collections::HashMap<String, i64>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelDataCdl {
    pub data: Vec<DataItem>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetFrameResponseRows {
    pub frame: Vec<FrameRow>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetFrameResponseHeader {
    #[serde(rename = "channelUris")]
    pub channel_uris: Vec<String>,
    pub indexes: Vec<IndexMetadataRecord>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetFrameMetadataResponse {
    pub uri: String,
    pub indexes: Vec<IndexMetadataRecord>,
    pub channels: Vec<FrameChannelMetadataRecord>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetFrameMetadata {
    pub uri: String,
    #[serde(rename = "includeAllChannelSecondaryIndexes")]
    #[serde(default = "default_getframemetadata_include_all_channel_secondary_indexes")]
    pub include_all_channel_secondary_indexes: bool,
}

#[inline(always)]
fn default_getframemetadata_include_all_channel_secondary_indexes() -> bool {
    false
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetFrame {
    pub uri: String,
    #[serde(rename = "includeAllChannelSecondaryIndexes")]
    #[serde(default = "default_getframe_include_all_channel_secondary_indexes")]
    pub include_all_channel_secondary_indexes: bool,
    #[serde(rename = "requestedInterval")]
    pub requested_interval: IndexInterval,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
    #[serde(rename = "requestedSecondaryIntervals")]
    #[serde(default = "default_getframe_requested_secondary_intervals")]
    pub requested_secondary_intervals: Vec<IndexInterval>,
}

#[inline(always)]
fn default_getframe_include_all_channel_secondary_indexes() -> bool {
    false
}

#[inline(always)]
fn default_getframe_requested_secondary_intervals() -> Vec<IndexInterval> {
    vec![]
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct CancelGetFrame {
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsUpdateInStoreResponse {
    #[serde(rename = "Result")]
    pub result: i32,
    #[serde(rename = "SuppMsgOut")]
    pub supp_msg_out: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsUpdateInStore {
    #[serde(rename = "WMLtypeIn")]
    pub wm_ltype_in: String,
    #[serde(rename = "XMLin")]
    pub xm_lin: String,
    #[serde(rename = "OptionsIn")]
    pub options_in: String,
    #[serde(rename = "CapabilitiesIn")]
    pub capabilities_in: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsGetVersionResponse {
    #[serde(rename = "Result")]
    pub result: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WmlsGetVersion {}

impl Default for WmlsGetVersion {
    fn default() -> WmlsGetVersion {
        WmlsGetVersion {}
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsGetFromStoreResponse {
    #[serde(rename = "Result")]
    pub result: i32,
    #[serde(rename = "XMLout")]
    pub xm_lout: String,
    #[serde(rename = "SuppMsgOut")]
    pub supp_msg_out: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsGetFromStore {
    #[serde(rename = "WMLtypeIn")]
    pub wm_ltype_in: String,
    #[serde(rename = "XMLin")]
    pub xm_lin: String,
    #[serde(rename = "OptionsIn")]
    pub options_in: String,
    #[serde(rename = "CapabilitiesIn")]
    pub capabilities_in: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsGetCapResponse {
    #[serde(rename = "Result")]
    pub result: i32,
    #[serde(rename = "CapabilitiesOut")]
    pub capabilities_out: String,
    #[serde(rename = "SuppMsgOut")]
    pub supp_msg_out: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsGetCap {
    #[serde(rename = "OptionsIn")]
    pub options_in: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsGetBaseMsgResponse {
    #[serde(rename = "Result")]
    pub result: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsGetBaseMsg {
    #[serde(rename = "ReturnValueIn")]
    pub return_value_in: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsDeleteFromStoreResponse {
    #[serde(rename = "Result")]
    pub result: i32,
    #[serde(rename = "SuppMsgOut")]
    pub supp_msg_out: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsDeleteFromStore {
    #[serde(rename = "WMLtypeIn")]
    pub wm_ltype_in: String,
    #[serde(rename = "XMLin")]
    pub xm_lin: String,
    #[serde(rename = "OptionsIn")]
    pub options_in: String,
    #[serde(rename = "CapabilitiesIn")]
    pub capabilities_in: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsAddToStoreResponse {
    #[serde(rename = "Result")]
    pub result: i32,
    #[serde(rename = "SuppMsgOut")]
    pub supp_msg_out: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct WmlsAddToStore {
    #[serde(rename = "WMLtypeIn")]
    pub wm_ltype_in: String,
    #[serde(rename = "XMLin")]
    pub xm_lin: String,
    #[serde(rename = "OptionsIn")]
    pub options_in: String,
    #[serde(rename = "CapabilitiesIn")]
    pub capabilities_in: String,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub revision: i32,
    pub patch: i32,
}

#[inline(always)]
fn default_version_major() -> i32 {
    0
}

#[inline(always)]
fn default_version_minor() -> i32 {
    0
}

#[inline(always)]
fn default_version_revision() -> i32 {
    0
}

#[inline(always)]
fn default_version_patch() -> i32 {
    0
}

impl Default for Version {
    fn default() -> Version {
        Version {
            major: default_version_major(),
            minor: default_version_minor(),
            revision: default_version_revision(),
            patch: default_version_patch(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SupportedProtocol {
    pub protocol: i32,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: Version,
    pub role: String,
    #[serde(rename = "protocolCapabilities")]
    #[serde(default = "default_supportedprotocol_protocol_capabilities")]
    pub protocol_capabilities: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_supportedprotocol_protocol_capabilities(
) -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SupportedDataObject {
    #[serde(rename = "qualifiedType")]
    pub qualified_type: String,
    #[serde(rename = "dataObjectCapabilities")]
    #[serde(default = "default_supporteddataobject_data_object_capabilities")]
    pub data_object_capabilities: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_supporteddataobject_data_object_capabilities(
) -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ServerCapabilities {
    #[serde(rename = "applicationName")]
    pub application_name: String,
    #[serde(rename = "applicationVersion")]
    pub application_version: String,
    #[serde(rename = "contactInformation")]
    pub contact_information: Contact,
    #[serde(rename = "supportedCompression")]
    #[serde(default = "default_servercapabilities_supported_compression")]
    pub supported_compression: Vec<String>,
    #[serde(rename = "supportedEncodings")]
    #[serde(default = "default_servercapabilities_supported_encodings")]
    pub supported_encodings: Vec<String>,
    #[serde(rename = "supportedFormats")]
    #[serde(default = "default_servercapabilities_supported_formats")]
    pub supported_formats: Vec<String>,
    #[serde(rename = "supportedDataObjects")]
    pub supported_data_objects: Vec<SupportedDataObject>,
    #[serde(rename = "supportedProtocols")]
    pub supported_protocols: Vec<SupportedProtocol>,
    #[serde(rename = "endpointCapabilities")]
    #[serde(default = "default_servercapabilities_endpoint_capabilities")]
    pub endpoint_capabilities: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_servercapabilities_supported_compression() -> Vec<String> {
    vec![]
}

#[inline(always)]
fn default_servercapabilities_supported_encodings() -> Vec<String> {
    vec!["binary".to_owned()]
}

#[inline(always)]
fn default_servercapabilities_supported_formats() -> Vec<String> {
    vec!["xml".to_owned()]
}

#[inline(always)]
fn default_servercapabilities_endpoint_capabilities(
) -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum ProtocolCapabilityKind {
    FrameChangeDetectionPeriod,
    MaxDataArraySize,
    MaxDataObjectSize,
    MaxFrameResponseRowCount,
    MaxIndexCount,
    MaxRangeChannelCount,
    MaxRangeDataItemCount,
    MaxResponseCount,
    MaxStreamingChannelsSessionCount,
    MaxSubscriptionSessionCount,
    MaxTransactionCount,
    SupportsSecondaryIndexFiltering,
    TransactionTimeoutPeriod,
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum Protocol {
    Core = 0,
    ChannelStreaming = 1,
    ChannelDataFrame = 2,
    Discovery = 3,
    Store = 4,
    StoreNotification = 5,
    GrowingObject = 6,
    GrowingObjectNotification = 7,
    #[serde(rename = "DEPRECATED_8")]
    Deprecated8 = 8,
    DataArray = 9,
    #[serde(rename = "RESERVED_10")]
    Reserved10 = 10,
    #[serde(rename = "RESERVED_11")]
    Reserved11 = 11,
    #[serde(rename = "RESERVED_12")]
    Reserved12 = 12,
    DiscoveryQuery = 13,
    StoreQuery = 14,
    #[serde(rename = "RESERVED_15")]
    Reserved15 = 15,
    GrowingObjectQuery = 16,
    #[serde(rename = "RESERVED_17")]
    Reserved17 = 17,
    Transaction = 18,
    #[serde(rename = "RESERVED_19")]
    Reserved19 = 19,
    #[serde(rename = "RESERVED_20")]
    Reserved20 = 20,
    ChannelSubscribe = 21,
    ChannelDataLoad = 22,
    #[serde(rename = "RESERVED_23")]
    Reserved23 = 23,
    Dataspace = 24,
    SupportedTypes = 25,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SupportedType {
    #[serde(rename = "dataObjectType")]
    pub data_object_type: String,
    #[serde(rename = "objectCount")]
    pub object_count: Option<i32>,
    #[serde(rename = "relationshipKind")]
    pub relationship_kind: RelationshipKind,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionInfo {
    pub context: ContextInfo,
    pub scope: ContextScopeKind,
    #[serde(rename = "requestUuid")]
    #[serde(with = "serde_bytes")]
    pub request_uuid: Uuid,
    #[serde(rename = "includeObjectData")]
    pub include_object_data: bool,
    #[serde(default = "default_subscriptioninfo_format")]
    pub format: String,
}

#[inline(always)]
fn default_subscriptioninfo_format() -> String {
    "xml".to_owned()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Resource {
    pub uri: String,
    #[serde(rename = "alternateUris")]
    #[serde(default = "default_resource_alternate_uris")]
    pub alternate_uris: Vec<String>,
    pub name: String,
    #[serde(rename = "sourceCount")]
    #[serde(default = "default_resource_source_count")]
    pub source_count: Option<i32>,
    #[serde(rename = "targetCount")]
    #[serde(default = "default_resource_target_count")]
    pub target_count: Option<i32>,
    #[serde(rename = "lastChanged")]
    pub last_changed: i64,
    #[serde(rename = "storeLastWrite")]
    pub store_last_write: i64,
    #[serde(rename = "storeCreated")]
    pub store_created: i64,
    #[serde(rename = "activeStatus")]
    pub active_status: ActiveStatusKind,
    #[serde(rename = "customData")]
    #[serde(default = "default_resource_custom_data")]
    pub custom_data: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_resource_alternate_uris() -> Vec<String> {
    vec![]
}

#[inline(always)]
fn default_resource_source_count() -> Option<i32> {
    None
}

#[inline(always)]
fn default_resource_target_count() -> Option<i32> {
    None
}

#[inline(always)]
fn default_resource_custom_data() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum RelationshipKind {
    Primary,
    Secondary,
    Both,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PutResponse {
    #[serde(rename = "createdContainedObjectUris")]
    pub created_contained_object_uris: Vec<String>,
    #[serde(rename = "deletedContainedObjectUris")]
    pub deleted_contained_object_uris: Vec<String>,
    #[serde(rename = "joinedContainedObjectUris")]
    pub joined_contained_object_uris: Vec<String>,
    #[serde(rename = "unjoinedContainedObjectUris")]
    pub unjoined_contained_object_uris: Vec<String>,
}

#[inline(always)]
fn default_putresponse_created_contained_object_uris() -> Vec<String> {
    vec![]
}

#[inline(always)]
fn default_putresponse_deleted_contained_object_uris() -> Vec<String> {
    vec![]
}

#[inline(always)]
fn default_putresponse_joined_contained_object_uris() -> Vec<String> {
    vec![]
}

#[inline(always)]
fn default_putresponse_unjoined_contained_object_uris() -> Vec<String> {
    vec![]
}

impl Default for PutResponse {
    fn default() -> PutResponse {
        PutResponse {
            created_contained_object_uris: default_putresponse_created_contained_object_uris(),
            deleted_contained_object_uris: default_putresponse_deleted_contained_object_uris(),
            joined_contained_object_uris: default_putresponse_joined_contained_object_uris(),
            unjoined_contained_object_uris: default_putresponse_unjoined_contained_object_uris(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PartsMetadataInfo {
    pub uri: String,
    pub name: String,
    pub index: IndexMetadataRecord,
    #[serde(rename = "customData")]
    #[serde(default = "default_partsmetadatainfo_custom_data")]
    pub custom_data: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_partsmetadatainfo_custom_data() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ObjectPart {
    pub uid: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum ObjectChangeKind {
    #[serde(rename = "insert")]
    Insert,
    #[serde(rename = "update")]
    Update,
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "joined")]
    Joined,
    #[serde(rename = "unjoined")]
    Unjoined,
    #[serde(rename = "joinedSubscription")]
    JoinedSubscription,
    #[serde(rename = "unjoinedSubscription")]
    UnjoinedSubscription,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ObjectChange {
    #[serde(rename = "changeKind")]
    pub change_kind: ObjectChangeKind,
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    #[serde(rename = "dataObject")]
    pub data_object: DataObject,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct IndexInterval {
    #[serde(rename = "startIndex")]
    pub start_index: IndexValue,
    #[serde(rename = "endIndex")]
    pub end_index: IndexValue,
    pub uom: String,
    #[serde(rename = "depthDatum")]
    #[serde(default = "default_indexinterval_depth_datum")]
    pub depth_datum: String,
}

#[inline(always)]
fn default_indexinterval_depth_datum() -> String {
    "".to_owned()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Edge {
    #[serde(rename = "sourceUri")]
    pub source_uri: String,
    #[serde(rename = "targetUri")]
    pub target_uri: String,
    #[serde(rename = "relationshipKind")]
    pub relationship_kind: RelationshipKind,
    #[serde(rename = "customData")]
    #[serde(default = "default_edge_custom_data")]
    pub custom_data: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_edge_custom_data() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DeletedResource {
    pub uri: String,
    #[serde(rename = "deletedTime")]
    pub deleted_time: i64,
    #[serde(rename = "customData")]
    #[serde(default = "default_deletedresource_custom_data")]
    pub custom_data: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_deletedresource_custom_data() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct Dataspace {
    pub uri: String,
    #[serde(default = "default_dataspace_path")]
    pub path: String,
    #[serde(rename = "storeLastWrite")]
    pub store_last_write: i64,
    #[serde(rename = "storeCreated")]
    pub store_created: i64,
    #[serde(rename = "customData")]
    #[serde(default = "default_dataspace_custom_data")]
    pub custom_data: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_dataspace_path() -> String {
    "".to_owned()
}

#[inline(always)]
fn default_dataspace_custom_data() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataObject {
    pub resource: Resource,
    #[serde(default = "default_dataobject_format")]
    pub format: String,
    #[serde(rename = "blobId")]
    pub blob_id: Option<Uuid>,
    #[serde(default = "default_dataobject_data")]
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[inline(always)]
fn default_dataobject_format() -> String {
    "xml".to_owned()
}

#[inline(always)]
fn default_dataobject_data() -> Vec<u8> {
    vec![]
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum ContextScopeKind {
    #[serde(rename = "self")]
    Self_, // Renamed with underscore as 'Self' is a reserved word
    #[serde(rename = "sources")]
    Sources,
    #[serde(rename = "targets")]
    Targets,
    #[serde(rename = "sourcesOrSelf")]
    SourcesOrSelf,
    #[serde(rename = "targetsOrSelf")]
    TargetsOrSelf,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ContextInfo {
    pub uri: String,
    pub depth: i32,
    #[serde(rename = "dataObjectTypes")]
    #[serde(default = "default_contextinfo_data_object_types")]
    pub data_object_types: Vec<String>,
    #[serde(rename = "navigableEdges")]
    pub navigable_edges: RelationshipKind,
    #[serde(rename = "includeSecondaryTargets")]
    #[serde(default = "default_contextinfo_include_secondary_targets")]
    pub include_secondary_targets: bool,
    #[serde(rename = "includeSecondarySources")]
    #[serde(default = "default_contextinfo_include_secondary_sources")]
    pub include_secondary_sources: bool,
}

#[inline(always)]
fn default_contextinfo_data_object_types() -> Vec<String> {
    vec![]
}

#[inline(always)]
fn default_contextinfo_include_secondary_targets() -> bool {
    false
}

#[inline(always)]
fn default_contextinfo_include_secondary_sources() -> bool {
    false
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChangeResponseInfo {
    #[serde(rename = "responseTimestamp")]
    pub response_timestamp: i64,
    pub changes: ::std::collections::HashMap<String, Vec<ChangeAnnotation>>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChangeAnnotation {
    #[serde(rename = "changeTime")]
    pub change_time: i64,
    pub interval: IndexInterval,
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum ActiveStatusKind {
    Active,
    Inactive,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MessageHeaderExtension {
    pub extension: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_messageheaderextension_extension() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

impl Default for MessageHeaderExtension {
    fn default() -> MessageHeaderExtension {
        MessageHeaderExtension {
            extension: default_messageheaderextension_extension(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct MessageHeader {
    pub protocol: i32,
    #[serde(rename = "messageType")]
    pub message_type: i32,
    #[serde(rename = "correlationId")]
    pub correlation_id: i64,
    #[serde(rename = "messageId")]
    pub message_id: i64,
    #[serde(rename = "messageFlags")]
    pub message_flags: i32,
}

/// Auto-generated type for unnamed Avro union variants.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum UnionLongDoublePassIndexedDepth {
    Long(i64),
    Double(f64),
    PassIndexedDepth(PassIndexedDepth),
}

impl From<i64> for UnionLongDoublePassIndexedDepth {
    fn from(v: i64) -> Self {
        Self::Long(v)
    }
}

impl TryFrom<UnionLongDoublePassIndexedDepth> for i64 {
    type Error = UnionLongDoublePassIndexedDepth;

    fn try_from(v: UnionLongDoublePassIndexedDepth) -> Result<Self, Self::Error> {
        if let UnionLongDoublePassIndexedDepth::Long(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<f64> for UnionLongDoublePassIndexedDepth {
    fn from(v: f64) -> Self {
        Self::Double(v)
    }
}

impl TryFrom<UnionLongDoublePassIndexedDepth> for f64 {
    type Error = UnionLongDoublePassIndexedDepth;

    fn try_from(v: UnionLongDoublePassIndexedDepth) -> Result<Self, Self::Error> {
        if let UnionLongDoublePassIndexedDepth::Double(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<PassIndexedDepth> for UnionLongDoublePassIndexedDepth {
    fn from(v: PassIndexedDepth) -> Self {
        Self::PassIndexedDepth(v)
    }
}

impl TryFrom<UnionLongDoublePassIndexedDepth> for PassIndexedDepth {
    type Error = UnionLongDoublePassIndexedDepth;

    fn try_from(v: UnionLongDoublePassIndexedDepth) -> Result<Self, Self::Error> {
        if let UnionLongDoublePassIndexedDepth::PassIndexedDepth(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl<'de> serde::Deserialize<'de> for UnionLongDoublePassIndexedDepth {
    fn deserialize<D>(deserializer: D) -> Result<UnionLongDoublePassIndexedDepth, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Serde visitor for the auto-generated unnamed Avro union type.
        struct UnionLongDoublePassIndexedDepthVisitor;

        impl<'de> serde::de::Visitor<'de> for UnionLongDoublePassIndexedDepthVisitor {
            type Value = UnionLongDoublePassIndexedDepth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a UnionLongDoublePassIndexedDepth")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(UnionLongDoublePassIndexedDepth::Long(value.into()))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(UnionLongDoublePassIndexedDepth::Double(value.into()))
            }
        }

        deserializer.deserialize_any(UnionLongDoublePassIndexedDepthVisitor)
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct IndexValue {
    pub item: Option<UnionLongDoublePassIndexedDepth>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ErrorInfo {
    pub message: String,
    pub code: i32,
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum EndpointCapabilityKind {
    ActiveTimeoutPeriod,
    AuthorizationDetails,
    ChangePropagationPeriod,
    ChangeRetentionPeriod,
    MaxConcurrentMultipart,
    MaxDataObjectSize,
    MaxPartSize,
    MaxSessionClientCount,
    MaxSessionGlobalCount,
    MaxWebSocketFramePayloadSize,
    MaxWebSocketMessagePayloadSize,
    MultipartMessageTimeoutPeriod,
    ResponseTimeoutPeriod,
    RequestSessionTimeoutPeriod,
    SessionEstablishmentTimeoutPeriod,
    SupportsAlternateRequestUris,
    SupportsMessageHeaderExtensions,
}

/// Auto-generated type for unnamed Avro union variants.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum DataValueEnum {
    Null,
    Boolean(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    ArrayOfBoolean(ArrayOfBoolean),
    ArrayOfNullableBoolean(ArrayOfNullableBoolean),
    ArrayOfInt(ArrayOfInt),
    ArrayOfNullableInt(ArrayOfNullableInt),
    ArrayOfLong(ArrayOfLong),
    ArrayOfNullableLong(ArrayOfNullableLong),
    ArrayOfFloat(ArrayOfFloat),
    ArrayOfDouble(ArrayOfDouble),
    ArrayOfString(ArrayOfString),
    ArrayOfBytes(ArrayOfBytes),
    Bytes(Vec<u8>),
    AnySparseArray(AnySparseArray),
}

impl From<bool> for DataValueEnum {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl TryFrom<DataValueEnum> for bool {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::Boolean(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<i32> for DataValueEnum {
    fn from(v: i32) -> Self {
        Self::Int(v)
    }
}

impl TryFrom<DataValueEnum> for i32 {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::Int(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<i64> for DataValueEnum {
    fn from(v: i64) -> Self {
        Self::Long(v)
    }
}

impl TryFrom<DataValueEnum> for i64 {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::Long(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<f32> for DataValueEnum {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}

impl TryFrom<DataValueEnum> for f32 {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::Float(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<f64> for DataValueEnum {
    fn from(v: f64) -> Self {
        Self::Double(v)
    }
}

impl TryFrom<DataValueEnum> for f64 {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::Double(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<String> for DataValueEnum {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl TryFrom<DataValueEnum> for String {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::String(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfBoolean> for DataValueEnum {
    fn from(v: ArrayOfBoolean) -> Self {
        Self::ArrayOfBoolean(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfBoolean {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfBoolean(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfNullableBoolean> for DataValueEnum {
    fn from(v: ArrayOfNullableBoolean) -> Self {
        Self::ArrayOfNullableBoolean(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfNullableBoolean {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfNullableBoolean(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfInt> for DataValueEnum {
    fn from(v: ArrayOfInt) -> Self {
        Self::ArrayOfInt(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfInt {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfInt(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfNullableInt> for DataValueEnum {
    fn from(v: ArrayOfNullableInt) -> Self {
        Self::ArrayOfNullableInt(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfNullableInt {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfNullableInt(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfLong> for DataValueEnum {
    fn from(v: ArrayOfLong) -> Self {
        Self::ArrayOfLong(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfLong {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfLong(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfNullableLong> for DataValueEnum {
    fn from(v: ArrayOfNullableLong) -> Self {
        Self::ArrayOfNullableLong(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfNullableLong {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfNullableLong(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfFloat> for DataValueEnum {
    fn from(v: ArrayOfFloat) -> Self {
        Self::ArrayOfFloat(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfFloat {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfFloat(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfDouble> for DataValueEnum {
    fn from(v: ArrayOfDouble) -> Self {
        Self::ArrayOfDouble(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfDouble {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfDouble(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfString> for DataValueEnum {
    fn from(v: ArrayOfString) -> Self {
        Self::ArrayOfString(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfString {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfString(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfBytes> for DataValueEnum {
    fn from(v: ArrayOfBytes) -> Self {
        Self::ArrayOfBytes(v)
    }
}

impl TryFrom<DataValueEnum> for ArrayOfBytes {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::ArrayOfBytes(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<AnySparseArray> for DataValueEnum {
    fn from(v: AnySparseArray) -> Self {
        Self::AnySparseArray(v)
    }
}

impl TryFrom<DataValueEnum> for AnySparseArray {
    type Error = DataValueEnum;

    fn try_from(v: DataValueEnum) -> Result<Self, Self::Error> {
        if let DataValueEnum::AnySparseArray(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl<'de> serde::Deserialize<'de> for DataValueEnum {
    fn deserialize<D>(deserializer: D) -> Result<DataValueEnum, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Serde visitor for the auto-generated unnamed Avro union type.
        struct UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArrayVisitor;

        impl<'de> serde::de::Visitor<'de> for UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArrayVisitor {
            type Value = DataValueEnum;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArray")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DataValueEnum::Boolean(value.into()))
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DataValueEnum::Int(value.into()))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DataValueEnum::Long(value.into()))
            }

            fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DataValueEnum::Float(value.into()))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DataValueEnum::Double(value.into()))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(DataValueEnum::String(value.into()))
            }
        }

        deserializer.deserialize_any(UnionBooleanIntLongFloatDoubleStringArrayOfBooleanArrayOfNullableBooleanArrayOfIntArrayOfNullableIntArrayOfLongArrayOfNullableLongArrayOfFloatArrayOfDoubleArrayOfStringArrayOfBytesBytesAnySparseArrayVisitor)
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataValue {
    pub item: DataValueEnum,
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum DataObjectCapabilityKind {
    ActiveTimeoutPeriod,
    MaxContainedDataObjectCount,
    MaxDataObjectSize,
    OrphanedChildrenPrunedOnDelete,
    SupportsGet,
    SupportsPut,
    SupportsDelete,
    MaxSecondaryIndexCount,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataAttribute {
    #[serde(rename = "attributeId")]
    pub attribute_id: i32,
    #[serde(rename = "attributeValue")]
    pub attribute_value: DataValue,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutUninitializedDataArrayType {
    pub uid: DataArrayIdentifier,
    pub metadata: DataArrayMetadata,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataSubarraysType {
    pub uid: DataArrayIdentifier,
    pub data: AnyArray,
    pub starts: Vec<i64>,
    pub counts: Vec<i64>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PutDataArraysType {
    pub uid: DataArrayIdentifier,
    pub array: DataArray,
    #[serde(rename = "customData")]
    #[serde(default = "default_putdataarraystype_custom_data")]
    pub custom_data: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_putdataarraystype_custom_data() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct GetDataSubarraysType {
    pub uid: DataArrayIdentifier,
    #[serde(default = "default_getdatasubarraystype_starts")]
    pub starts: Vec<i64>,
    #[serde(default = "default_getdatasubarraystype_counts")]
    pub counts: Vec<i64>,
}

#[inline(always)]
fn default_getdatasubarraystype_starts() -> Vec<i64> {
    vec![]
}

#[inline(always)]
fn default_getdatasubarraystype_counts() -> Vec<i64> {
    vec![]
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataArrayMetadata {
    pub dimensions: Vec<i64>,
    #[serde(rename = "preferredSubarrayDimensions")]
    #[serde(default = "default_dataarraymetadata_preferred_subarray_dimensions")]
    pub preferred_subarray_dimensions: Vec<i64>,
    #[serde(rename = "transportArrayType")]
    pub transport_array_type: AnyArrayType,
    #[serde(rename = "logicalArrayType")]
    pub logical_array_type: AnyLogicalArrayType,
    #[serde(rename = "storeLastWrite")]
    pub store_last_write: i64,
    #[serde(rename = "storeCreated")]
    pub store_created: i64,
    #[serde(rename = "customData")]
    #[serde(default = "default_dataarraymetadata_custom_data")]
    pub custom_data: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_dataarraymetadata_preferred_subarray_dimensions() -> Vec<i64> {
    vec![]
}

#[inline(always)]
fn default_dataarraymetadata_custom_data() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataArrayIdentifier {
    pub uri: String,
    #[serde(rename = "pathInResource")]
    pub path_in_resource: String,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataArray {
    pub dimensions: Vec<i64>,
    pub data: AnyArray,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Contact {
    #[serde(rename = "organizationName")]
    pub organization_name: String,
    #[serde(rename = "contactName")]
    pub contact_name: String,
    #[serde(rename = "contactPhone")]
    pub contact_phone: String,
    #[serde(rename = "contactEmail")]
    pub contact_email: String,
}

#[inline(always)]
fn default_contact_organization_name() -> String {
    "".to_owned()
}

#[inline(always)]
fn default_contact_contact_name() -> String {
    "".to_owned()
}

#[inline(always)]
fn default_contact_contact_phone() -> String {
    "".to_owned()
}

#[inline(always)]
fn default_contact_contact_email() -> String {
    "".to_owned()
}

impl Default for Contact {
    fn default() -> Contact {
        Contact {
            organization_name: default_contact_organization_name(),
            contact_name: default_contact_contact_name(),
            contact_phone: default_contact_contact_phone(),
            contact_email: default_contact_contact_email(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct TruncateInfo {
    #[serde(rename = "channelId")]
    pub channel_id: i64,
    #[serde(rename = "newEndIndex")]
    pub new_end_index: IndexValue,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct PassIndexedDepth {
    pub pass: i64,
    pub direction: PassDirection,
    pub depth: f64,
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum PassDirection {
    Up,
    HoldingSteady,
    Down,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct OpenChannelInfo {
    pub metadata: ChannelMetadataRecord,
    #[serde(rename = "preferRealtime")]
    #[serde(default = "default_openchannelinfo_prefer_realtime")]
    pub prefer_realtime: bool,
    #[serde(rename = "dataChanges")]
    #[serde(default = "default_openchannelinfo_data_changes")]
    pub data_changes: bool,
}

#[inline(always)]
fn default_openchannelinfo_prefer_realtime() -> bool {
    true
}

#[inline(always)]
fn default_openchannelinfo_data_changes() -> bool {
    true
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct IndexMetadataRecord {
    #[serde(rename = "indexKind")]
    #[serde(default = "default_indexmetadatarecord_index_kind")]
    pub index_kind: ChannelIndexKind,
    pub interval: IndexInterval,
    #[serde(default = "default_indexmetadatarecord_direction")]
    pub direction: IndexDirection,
    #[serde(default = "default_indexmetadatarecord_name")]
    pub name: String,
    pub uom: String,
    #[serde(rename = "depthDatum")]
    #[serde(default = "default_indexmetadatarecord_depth_datum")]
    pub depth_datum: String,
    #[serde(rename = "indexPropertyKindUri")]
    pub index_property_kind_uri: String,
    #[serde(default = "default_indexmetadatarecord_filterable")]
    pub filterable: bool,
}

#[inline(always)]
fn default_indexmetadatarecord_index_kind() -> ChannelIndexKind {
    ChannelIndexKind::DateTime
}

#[inline(always)]
fn default_indexmetadatarecord_direction() -> IndexDirection {
    IndexDirection::Increasing
}

#[inline(always)]
fn default_indexmetadatarecord_name() -> String {
    "".to_owned()
}

#[inline(always)]
fn default_indexmetadatarecord_depth_datum() -> String {
    "".to_owned()
}

#[inline(always)]
fn default_indexmetadatarecord_filterable() -> bool {
    true
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum IndexDirection {
    Increasing,
    Decreasing,
    Unordered,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct FrameRow {
    pub indexes: Vec<IndexValue>,
    pub points: Vec<FramePoint>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct FramePoint {
    pub value: DataValue,
    #[serde(rename = "valueAttributes")]
    #[serde(default = "default_framepoint_value_attributes")]
    pub value_attributes: Vec<DataAttribute>,
}

#[inline(always)]
fn default_framepoint_value_attributes() -> Vec<DataAttribute> {
    vec![]
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct FrameChannelMetadataRecord {
    pub uri: String,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "dataKind")]
    pub data_kind: ChannelDataKind,
    pub uom: String,
    #[serde(rename = "depthDatum")]
    pub depth_datum: String,
    #[serde(rename = "channelPropertyKindUri")]
    pub channel_property_kind_uri: String,
    pub status: ActiveStatusKind,
    pub source: String,
    #[serde(rename = "axisVectorLengths")]
    pub axis_vector_lengths: Vec<i32>,
    #[serde(rename = "attributeMetadata")]
    #[serde(default = "default_framechannelmetadatarecord_attribute_metadata")]
    pub attribute_metadata: Vec<AttributeMetadataRecord>,
    #[serde(rename = "customData")]
    #[serde(default = "default_framechannelmetadatarecord_custom_data")]
    pub custom_data: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_framechannelmetadatarecord_attribute_metadata() -> Vec<AttributeMetadataRecord> {
    vec![]
}

#[inline(always)]
fn default_framechannelmetadatarecord_custom_data() -> ::std::collections::HashMap<String, DataValue>
{
    ::std::collections::HashMap::new()
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataItem {
    #[serde(rename = "channelId")]
    pub channel_id: i64,
    pub indexes: Vec<IndexValue>,
    pub value: DataValue,
    #[serde(rename = "valueAttributes")]
    #[serde(default = "default_dataitem_value_attributes")]
    pub value_attributes: Vec<DataAttribute>,
}

#[inline(always)]
fn default_dataitem_value_attributes() -> Vec<DataAttribute> {
    vec![]
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelSubscribeInfo {
    #[serde(rename = "channelId")]
    pub channel_id: i64,
    #[serde(rename = "startIndex")]
    pub start_index: IndexValue,
    #[serde(rename = "dataChanges")]
    #[serde(default = "default_channelsubscribeinfo_data_changes")]
    pub data_changes: bool,
    #[serde(rename = "requestLatestIndexCount")]
    pub request_latest_index_count: Option<i32>,
}

#[inline(always)]
fn default_channelsubscribeinfo_data_changes() -> bool {
    true
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelRangeInfo {
    #[serde(rename = "channelIds")]
    pub channel_ids: Vec<i64>,
    pub interval: IndexInterval,
    #[serde(rename = "secondaryIntervals")]
    #[serde(default = "default_channelrangeinfo_secondary_intervals")]
    pub secondary_intervals: Vec<IndexInterval>,
}

#[inline(always)]
fn default_channelrangeinfo_secondary_intervals() -> Vec<IndexInterval> {
    vec![]
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelMetadataRecord {
    pub uri: String,
    pub id: i64,
    pub indexes: Vec<IndexMetadataRecord>,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "dataKind")]
    pub data_kind: ChannelDataKind,
    pub uom: String,
    #[serde(rename = "depthDatum")]
    pub depth_datum: String,
    #[serde(rename = "channelClassUri")]
    pub channel_class_uri: String,
    pub status: ActiveStatusKind,
    pub source: String,
    #[serde(rename = "axisVectorLengths")]
    pub axis_vector_lengths: Vec<i32>,
    #[serde(rename = "attributeMetadata")]
    #[serde(default = "default_channelmetadatarecord_attribute_metadata")]
    pub attribute_metadata: Vec<AttributeMetadataRecord>,
    #[serde(rename = "customData")]
    #[serde(default = "default_channelmetadatarecord_custom_data")]
    pub custom_data: ::std::collections::HashMap<String, DataValue>,
}

#[inline(always)]
fn default_channelmetadatarecord_attribute_metadata() -> Vec<AttributeMetadataRecord> {
    vec![]
}

#[inline(always)]
fn default_channelmetadatarecord_custom_data() -> ::std::collections::HashMap<String, DataValue> {
    ::std::collections::HashMap::new()
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum ChannelIndexKind {
    DateTime,
    ElapsedTime,
    MeasuredDepth,
    TrueVerticalDepth,
    PassIndexedDepth,
    Pressure,
    Temperature,
    Scalar,
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum ChannelDataKind {
    DateTime,
    ElapsedTime,
    MeasuredDepth,
    PassIndexedDepth,
    TrueVerticalDepth,
    #[serde(rename = "typeBoolean")]
    TypeBoolean,
    #[serde(rename = "typeInt")]
    TypeInt,
    #[serde(rename = "typeLong")]
    TypeLong,
    #[serde(rename = "typeFloat")]
    TypeFloat,
    #[serde(rename = "typeDouble")]
    TypeDouble,
    #[serde(rename = "typeString")]
    TypeString,
    #[serde(rename = "typeBytes")]
    TypeBytes,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ChannelChangeRequestInfo {
    #[serde(rename = "sinceChangeTime")]
    pub since_change_time: i64,
    #[serde(rename = "channelIds")]
    pub channel_ids: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct AttributeMetadataRecord {
    #[serde(rename = "attributeId")]
    pub attribute_id: i32,
    #[serde(rename = "attributeName")]
    pub attribute_name: String,
    #[serde(rename = "dataKind")]
    pub data_kind: ChannelDataKind,
    pub uom: String,
    #[serde(rename = "depthDatum")]
    pub depth_datum: String,
    #[serde(rename = "attributePropertyKindUri")]
    pub attribute_property_kind_uri: String,
    #[serde(rename = "axisVectorLengths")]
    pub axis_vector_lengths: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfString {
    pub values: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfNullableLong {
    pub values: Vec<Option<i64>>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfNullableInt {
    pub values: Vec<Option<i32>>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfNullableBoolean {
    pub values: Vec<Option<bool>>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfLong {
    pub values: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfInt {
    pub values: Vec<i32>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfFloat {
    pub values: Vec<f32>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfDouble {
    pub values: Vec<f64>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfBytes {
    pub values: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub struct ArrayOfBoolean {
    pub values: Vec<bool>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct AnySubarray {
    pub start: i64,
    pub slice: AnyArray,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct AnySparseArray {
    pub slices: Vec<AnySubarray>,
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum AnyLogicalArrayType {
    #[serde(rename = "arrayOfBoolean")]
    ArrayOfBoolean,
    #[serde(rename = "arrayOfInt8")]
    ArrayOfInt8,
    #[serde(rename = "arrayOfUInt8")]
    ArrayOfUInt8,
    #[serde(rename = "arrayOfInt16LE")]
    ArrayOfInt16Le,
    #[serde(rename = "arrayOfInt32LE")]
    ArrayOfInt32Le,
    #[serde(rename = "arrayOfInt64LE")]
    ArrayOfInt64Le,
    #[serde(rename = "arrayOfUInt16LE")]
    ArrayOfUInt16Le,
    #[serde(rename = "arrayOfUInt32LE")]
    ArrayOfUInt32Le,
    #[serde(rename = "arrayOfUInt64LE")]
    ArrayOfUInt64Le,
    #[serde(rename = "arrayOfFloat32LE")]
    ArrayOfFloat32Le,
    #[serde(rename = "arrayOfDouble64LE")]
    ArrayOfDouble64Le,
    #[serde(rename = "arrayOfInt16BE")]
    ArrayOfInt16Be,
    #[serde(rename = "arrayOfInt32BE")]
    ArrayOfInt32Be,
    #[serde(rename = "arrayOfInt64BE")]
    ArrayOfInt64Be,
    #[serde(rename = "arrayOfUInt16BE")]
    ArrayOfUInt16Be,
    #[serde(rename = "arrayOfUInt32BE")]
    ArrayOfUInt32Be,
    #[serde(rename = "arrayOfUInt64BE")]
    ArrayOfUInt64Be,
    #[serde(rename = "arrayOfFloat32BE")]
    ArrayOfFloat32Be,
    #[serde(rename = "arrayOfDouble64BE")]
    ArrayOfDouble64Be,
    #[serde(rename = "arrayOfString")]
    ArrayOfString,
    #[serde(rename = "arrayOfCustom")]
    ArrayOfCustom,
}

#[derive(
    Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, serde::Deserialize, serde::Serialize,
)]
pub enum AnyArrayType {
    #[serde(rename = "arrayOfBoolean")]
    ArrayOfBoolean,
    #[serde(rename = "arrayOfInt")]
    ArrayOfInt,
    #[serde(rename = "arrayOfLong")]
    ArrayOfLong,
    #[serde(rename = "arrayOfFloat")]
    ArrayOfFloat,
    #[serde(rename = "arrayOfDouble")]
    ArrayOfDouble,
    #[serde(rename = "arrayOfString")]
    ArrayOfString,
    #[serde(rename = "bytes")]
    Bytes,
}

/// Auto-generated type for unnamed Avro union variants.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum AnyArrayUnion {
    ArrayOfBoolean(ArrayOfBoolean),
    ArrayOfInt(ArrayOfInt),
    ArrayOfLong(ArrayOfLong),
    ArrayOfFloat(ArrayOfFloat),
    ArrayOfDouble(ArrayOfDouble),
    ArrayOfString(ArrayOfString),
    Bytes(Vec<u8>),
}

impl From<ArrayOfBoolean> for AnyArrayUnion {
    fn from(v: ArrayOfBoolean) -> Self {
        Self::ArrayOfBoolean(v)
    }
}

impl TryFrom<AnyArrayUnion> for ArrayOfBoolean {
    type Error = AnyArrayUnion;

    fn try_from(v: AnyArrayUnion) -> Result<Self, Self::Error> {
        if let AnyArrayUnion::ArrayOfBoolean(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfInt> for AnyArrayUnion {
    fn from(v: ArrayOfInt) -> Self {
        Self::ArrayOfInt(v)
    }
}

impl TryFrom<AnyArrayUnion> for ArrayOfInt {
    type Error = AnyArrayUnion;

    fn try_from(v: AnyArrayUnion) -> Result<Self, Self::Error> {
        if let AnyArrayUnion::ArrayOfInt(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfLong> for AnyArrayUnion {
    fn from(v: ArrayOfLong) -> Self {
        Self::ArrayOfLong(v)
    }
}

impl TryFrom<AnyArrayUnion> for ArrayOfLong {
    type Error = AnyArrayUnion;

    fn try_from(v: AnyArrayUnion) -> Result<Self, Self::Error> {
        if let AnyArrayUnion::ArrayOfLong(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfFloat> for AnyArrayUnion {
    fn from(v: ArrayOfFloat) -> Self {
        Self::ArrayOfFloat(v)
    }
}

impl TryFrom<AnyArrayUnion> for ArrayOfFloat {
    type Error = AnyArrayUnion;

    fn try_from(v: AnyArrayUnion) -> Result<Self, Self::Error> {
        if let AnyArrayUnion::ArrayOfFloat(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfDouble> for AnyArrayUnion {
    fn from(v: ArrayOfDouble) -> Self {
        Self::ArrayOfDouble(v)
    }
}

impl TryFrom<AnyArrayUnion> for ArrayOfDouble {
    type Error = AnyArrayUnion;

    fn try_from(v: AnyArrayUnion) -> Result<Self, Self::Error> {
        if let AnyArrayUnion::ArrayOfDouble(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<ArrayOfString> for AnyArrayUnion {
    fn from(v: ArrayOfString) -> Self {
        Self::ArrayOfString(v)
    }
}

impl TryFrom<AnyArrayUnion> for ArrayOfString {
    type Error = AnyArrayUnion;

    fn try_from(v: AnyArrayUnion) -> Result<Self, Self::Error> {
        if let AnyArrayUnion::ArrayOfString(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl<'de> serde::Deserialize<'de> for AnyArrayUnion {
    fn deserialize<D>(deserializer: D) -> Result<AnyArrayUnion, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Serde visitor for the auto-generated unnamed Avro union type.
        struct UnionArrayOfBooleanArrayOfIntArrayOfLongArrayOfFloatArrayOfDoubleArrayOfStringBytesVisitor;

        impl<'de> serde::de::Visitor<'de> for UnionArrayOfBooleanArrayOfIntArrayOfLongArrayOfFloatArrayOfDoubleArrayOfStringBytesVisitor {
            type Value = AnyArrayUnion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a UnionArrayOfBooleanArrayOfIntArrayOfLongArrayOfFloatArrayOfDoubleArrayOfStringBytes")
            }
        }

        deserializer.deserialize_any(UnionArrayOfBooleanArrayOfIntArrayOfLongArrayOfFloatArrayOfDoubleArrayOfStringBytesVisitor)
    }
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct AnyArray {
    // #[serde(with = "serde_bytes")]
    pub item: AnyArrayUnion,
}
