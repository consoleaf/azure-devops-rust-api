// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.
#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Avatar {
    #[serde(
        rename = "isAutoGenerated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_auto_generated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<avatar::Size>,
    #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl Avatar {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod avatar {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Size {
        #[serde(rename = "small")]
        Small,
        #[serde(rename = "medium")]
        Medium,
        #[serde(rename = "large")]
        Large,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphCachePolicies {
    #[doc = "Size of the cache"]
    #[serde(rename = "cacheSize", default, skip_serializing_if = "Option::is_none")]
    pub cache_size: Option<i32>,
}
impl GraphCachePolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subject descriptor of a Graph entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphDescriptorResult {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl GraphDescriptorResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a set of data used to communicate with a federated provider on behalf of a particular user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphFederatedProviderData {
    #[doc = "The access token that can be used to communicated with the federated provider on behalf on the target identity, if we were able to successfully acquire one, otherwise <code>null</code>, if we were not."]
    #[serde(
        rename = "accessToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<String>,
    #[doc = "The name of the federated provider, e.g. \"github.com\"."]
    #[serde(
        rename = "providerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_name: Option<String>,
    #[doc = "The descriptor of the graph subject to which this federated provider data corresponds."]
    #[serde(
        rename = "subjectDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_descriptor: Option<String>,
    #[doc = "The version number of this federated provider data, which corresponds to when it was last updated. Can be used to prevent returning stale provider data from the cache when the caller is aware of a newer version, such as to prevent local cache poisoning from a remote cache or store. This is the app layer equivalent of the data layer sequence ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
impl GraphFederatedProviderData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphGlobalExtendedPropertyBatch {
    #[serde(
        rename = "propertyNameFilters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub property_name_filters: Vec<String>,
    #[serde(
        rename = "subjectDescriptors",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub subject_descriptors: Vec<String>,
}
impl GraphGlobalExtendedPropertyBatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Graph group entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphGroup {
    #[serde(flatten)]
    pub graph_member: GraphMember,
    #[doc = "A short phrase to help human readers disambiguate groups with similar names"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl GraphGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Do not attempt to use this type to create a new group. This type does not contain sufficient fields to create a new group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphGroupCreationContext {
    #[doc = "Optional: If provided, we will use this identifier for the storage key of the created group"]
    #[serde(
        rename = "storageKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
}
impl GraphGroupCreationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphGroupList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GraphGroup>,
}
impl GraphGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Use this type to create a new group using the mail address as a reference to an existing group from an external AD or AAD backed provider. This is the subset of GraphGroup fields required for creation of a group for the AAD and AD use case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphGroupMailAddressCreationContext {
    #[serde(flatten)]
    pub graph_group_creation_context: GraphGroupCreationContext,
    #[doc = "This should be the mail address or the group in the source AD or AAD provider. Example: jamal@contoso.com Team Services will communicate with the source provider to fill all other fields on creation."]
    #[serde(
        rename = "mailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mail_address: Option<String>,
}
impl GraphGroupMailAddressCreationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Use this type to create a new group using the OriginID as a reference to an existing group from an external AD or AAD backed provider. This is the subset of GraphGroup fields required for creation of a group for the AD and AAD use case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphGroupOriginIdCreationContext {
    #[serde(flatten)]
    pub graph_group_creation_context: GraphGroupCreationContext,
    #[doc = "This should be the object id or sid of the group from the source AD or AAD provider. Example: d47d025a-ce2f-4a79-8618-e8862ade30dd Team Services will communicate with the source provider to fill all other fields on creation."]
    #[serde(rename = "originId", default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}
impl GraphGroupOriginIdCreationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Use this type to create a new Vsts group that is not backed by an external provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphGroupVstsCreationContext {
    #[serde(flatten)]
    pub graph_group_creation_context: GraphGroupCreationContext,
    #[doc = "For internal use only in back compat scenarios."]
    #[serde(
        rename = "crossProject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cross_project: Option<bool>,
    #[doc = "Used by VSTS groups; if set this will be the group description, otherwise ignored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[doc = "Used by VSTS groups; if set this will be the group DisplayName, otherwise ignored"]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "For internal use only in back compat scenarios."]
    #[serde(
        rename = "restrictedVisibility",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub restricted_visibility: Option<bool>,
    #[doc = "For internal use only in back compat scenarios."]
    #[serde(
        rename = "specialGroupType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_group_type: Option<String>,
}
impl GraphGroupVstsCreationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphMember {
    #[serde(flatten)]
    pub graph_subject: GraphSubject,
    #[doc = "This represents the name of the container of origin for a graph member. (For MSA this is \"Windows Live ID\", for AD the name of the domain, for AAD the tenantID of the directory, for VSTS groups the ScopeId, etc)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The email address of record for a given graph member. This may be different than the principal name."]
    #[serde(
        rename = "mailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mail_address: Option<String>,
    #[doc = "This is the PrincipalName of this graph member from the source provider. The source provider may change this field over time and it is not guaranteed to be immutable for the life of the graph member by VSTS."]
    #[serde(
        rename = "principalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_name: Option<String>,
}
impl GraphMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Relationship between a container and a member"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphMembership {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(
        rename = "containerDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub container_descriptor: Option<String>,
    #[serde(
        rename = "memberDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub member_descriptor: Option<String>,
}
impl GraphMembership {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphMembershipList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GraphMembership>,
}
impl GraphMembershipList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of a Graph membership (active/inactive)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphMembershipState {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "When true, the membership is active"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
impl GraphMembershipState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphMembershipTraversal {
    #[doc = "Reason why the subject could not be traversed completely"]
    #[serde(
        rename = "incompletenessReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub incompleteness_reason: Option<String>,
    #[doc = "When true, the subject is traversed completely"]
    #[serde(
        rename = "isComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_complete: Option<bool>,
    #[doc = "The traversed subject descriptor"]
    #[serde(
        rename = "subjectDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_descriptor: Option<String>,
    #[doc = "Subject descriptor ids of the traversed members"]
    #[serde(
        rename = "traversedSubjectIds",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub traversed_subject_ids: Vec<String>,
    #[doc = "Subject descriptors of the traversed members"]
    #[serde(
        rename = "traversedSubjects",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub traversed_subjects: Vec<String>,
}
impl GraphMembershipTraversal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Who is the provider for this user and what is the identifier and domain that is used to uniquely identify the user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphProviderInfo {
    #[doc = "The descriptor is the primary way to reference the graph subject while the system is running. This field will uniquely identify the same graph subject across both Accounts and Organizations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[doc = "This represents the name of the container of origin for a graph member. (For MSA this is \"Windows Live ID\", for AAD the tenantID of the directory.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "The type of source provider for the origin identifier (ex: \"aad\", \"msa\")"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The unique identifier from the system of origin. (For MSA this is the PUID in hex notation, for AAD this is the object id.)"]
    #[serde(rename = "originId", default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}
impl GraphProviderInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container where a graph entity is defined (organization, project, team)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphScope {
    #[serde(flatten)]
    pub graph_subject: GraphSubject,
    #[doc = "The subject descriptor that references the administrators group for this scope. Only members of this group can change the contents of this scope or assign other users permissions to access this scope."]
    #[serde(
        rename = "administratorDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub administrator_descriptor: Option<String>,
    #[doc = "When true, this scope is also a securing host for one or more scopes."]
    #[serde(rename = "isGlobal", default, skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    #[doc = "The subject descriptor for the closest account or organization in the ancestor tree of this scope."]
    #[serde(
        rename = "parentDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_descriptor: Option<String>,
    #[doc = "The type of this scope. Typically ServiceHost or TeamProject."]
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<graph_scope::ScopeType>,
    #[doc = "The subject descriptor for the containing organization in the ancestor tree of this scope."]
    #[serde(
        rename = "securingHostDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub securing_host_descriptor: Option<String>,
}
impl GraphScope {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod graph_scope {
    use super::*;
    #[doc = "The type of this scope. Typically ServiceHost or TeamProject."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "generic")]
        Generic,
        #[serde(rename = "serviceHost")]
        ServiceHost,
        #[serde(rename = "teamProject")]
        TeamProject,
    }
}
#[doc = "This type is the subset of fields that can be provided by the user to create a Vsts scope. Scope creation is currently limited to internal back-compat scenarios. End users that attempt to create a scope with this API will fail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphScopeCreationContext {
    #[doc = "Set this field to override the default description of this scope's admin group."]
    #[serde(
        rename = "adminGroupDescription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_group_description: Option<String>,
    #[doc = "All scopes have an Administrator Group that controls access to the contents of the scope. Set this field to use a non-default group name for that administrators group."]
    #[serde(
        rename = "adminGroupName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub admin_group_name: Option<String>,
    #[doc = "Set this optional field if this scope is created on behalf of a user other than the user making the request. This should be the Id of the user that is not the requester."]
    #[serde(rename = "creatorId", default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[doc = "The scope must be provided with a unique name within the parent scope. This means the created scope can have a parent or child with the same name, but no siblings with the same name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of scope being created."]
    #[serde(rename = "scopeType", default, skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<graph_scope_creation_context::ScopeType>,
    #[doc = "An optional ID that uniquely represents the scope within it's parent scope. If this parameter is not provided, Vsts will generate on automatically."]
    #[serde(
        rename = "storageKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
}
impl GraphScopeCreationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod graph_scope_creation_context {
    use super::*;
    #[doc = "The type of scope being created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScopeType {
        #[serde(rename = "generic")]
        Generic,
        #[serde(rename = "serviceHost")]
        ServiceHost,
        #[serde(rename = "teamProject")]
        TeamProject,
    }
}
#[doc = "Storage key of a Graph entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphStorageKeyResult {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl GraphStorageKeyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Top-level graph entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSubject {
    #[serde(flatten)]
    pub graph_subject_base: GraphSubjectBase,
    #[doc = "[Internal Use Only] The legacy descriptor is here in case you need to access old version IMS using identity descriptor."]
    #[serde(
        rename = "legacyDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub legacy_descriptor: Option<String>,
    #[doc = "The type of source provider for the origin identifier (ex:AD, AAD, MSA)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The unique identifier from the system of origin. Typically a sid, object id or Guid. Linking and unlinking operations can cause this value to change for a user because the user is not backed by a different provider and has a different unique id in the new provider."]
    #[serde(rename = "originId", default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    #[doc = "This field identifies the type of the graph subject (ex: Group, Scope, User)."]
    #[serde(
        rename = "subjectKind",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subject_kind: Option<String>,
}
impl GraphSubject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSubjectBase {
    #[doc = "Links"]
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[doc = "The descriptor is the primary way to reference the graph subject while the system is running. This field will uniquely identify the same graph subject across both Accounts and Organizations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[doc = "This is the non-unique display name of the graph subject. To change this field, you must alter its value in the source provider."]
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "This url is the full route to the source resource of this graph subject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl GraphSubjectBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSubjectList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GraphSubject>,
}
impl GraphSubjectList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Batching of subjects to lookup using the Graph API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSubjectLookup {
    #[serde(rename = "lookupKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub lookup_keys: Vec<GraphSubjectLookupKey>,
}
impl GraphSubjectLookup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSubjectLookupKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
}
impl GraphSubjectLookupKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subject to search using the Graph API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSubjectQuery {
    #[doc = "Search term to search for Azure Devops users or/and groups"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "Optional parameter. Specify a non-default scope (collection, project) to search for users or groups within the scope."]
    #[serde(
        rename = "scopeDescriptor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_descriptor: Option<String>,
    #[doc = "\"User\" or \"Group\" can be specified, both or either"]
    #[serde(rename = "subjectKind", default, skip_serializing_if = "Vec::is_empty")]
    pub subject_kind: Vec<String>,
}
impl GraphSubjectQuery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphSystemSubject {
    #[serde(flatten)]
    pub graph_subject: GraphSubject,
}
impl GraphSystemSubject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Graph user entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphUser {
    #[serde(flatten)]
    pub graph_member: GraphMember,
    #[doc = "The short, generally unique name for the user in the backing directory. For AAD users, this corresponds to the mail nickname, which is often but not necessarily similar to the part of the user's mail address before the @ sign. For GitHub users, this corresponds to the GitHub user handle."]
    #[serde(
        rename = "directoryAlias",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub directory_alias: Option<String>,
    #[doc = "When true, the group has been deleted in the identity provider"]
    #[serde(
        rename = "isDeletedInOrigin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_deleted_in_origin: Option<bool>,
    #[doc = "The meta type of the user in the origin, such as \"member\", \"guest\", etc. See UserMetaType for the set of possible values."]
    #[serde(rename = "metaType", default, skip_serializing_if = "Option::is_none")]
    pub meta_type: Option<String>,
}
impl GraphUser {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Do not attempt to use this type to create a new user. Use one of the subclasses instead. This type does not contain sufficient fields to create a new user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphUserCreationContext {
    #[doc = "Optional: If provided, we will use this identifier for the storage key of the created user"]
    #[serde(
        rename = "storageKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_key: Option<String>,
}
impl GraphUserCreationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphUserList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GraphUser>,
}
impl GraphUserList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Use this type to create a new user using the mail address as a reference to an existing user from an external AD or AAD backed provider. This is the subset of GraphUser fields required for creation of a GraphUser for the AD and AAD use case when looking up the user by its mail address in the backing provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphUserMailAddressCreationContext {
    #[serde(flatten)]
    pub graph_user_creation_context: GraphUserCreationContext,
    #[serde(
        rename = "mailAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mail_address: Option<String>,
}
impl GraphUserMailAddressCreationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Use this type to create a new user using the OriginID as a reference to an existing user from an external AD or AAD backed provider. This is the subset of GraphUser fields required for creation of a GraphUser for the AD and AAD use case when looking up the user by its unique ID in the backing provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphUserOriginIdCreationContext {
    #[serde(flatten)]
    pub graph_user_creation_context: GraphUserCreationContext,
    #[doc = "This should be the name of the origin provider. Example: github.com"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "This should be the object id or sid of the user from the source AD or AAD provider. Example: d47d025a-ce2f-4a79-8618-e8862ade30dd Team Services will communicate with the source provider to fill all other fields on creation."]
    #[serde(rename = "originId", default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}
impl GraphUserOriginIdCreationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Use this type to update an existing user using the OriginID as a reference to an existing user from an external AD or AAD backed provider. This is the subset of GraphUser fields required for creation of a GraphUser for the AD and AAD use case when looking up the user by its unique ID in the backing provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphUserOriginIdUpdateContext {
    #[serde(flatten)]
    pub graph_user_update_context: GraphUserUpdateContext,
    #[doc = "This should be the object id or sid of the user from the source AD or AAD provider. Example: d47d025a-ce2f-4a79-8618-e8862ade30dd Azure Devops will communicate with the source provider to fill all other fields on creation."]
    #[serde(rename = "originId", default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}
impl GraphUserOriginIdUpdateContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Use this type to create a new user using the principal name as a reference to an existing user from an external AD or AAD backed provider. This is the subset of GraphUser fields required for creation of a GraphUser for the AD and AAD use case when looking up the user by its principal name in the backing provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphUserPrincipalNameCreationContext {
    #[serde(flatten)]
    pub graph_user_creation_context: GraphUserCreationContext,
    #[doc = "This should be the principal name or upn of the user in the source AD or AAD provider. Example: jamal@contoso.com Team Services will communicate with the source provider to fill all other fields on creation."]
    #[serde(
        rename = "principalName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub principal_name: Option<String>,
}
impl GraphUserPrincipalNameCreationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Do not attempt to use this type to update user. Use one of the subclasses instead. This type does not contain sufficient fields to create a new user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GraphUserUpdateContext {}
impl GraphUserUpdateContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an abstract JSON token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JToken {
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first: Box<Option<JToken>>,
    #[doc = "Gets a value indicating whether this token has child tokens."]
    #[serde(rename = "hasValues", default, skip_serializing_if = "Option::is_none")]
    pub has_values: Option<bool>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Box<Option<JToken>>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last: Box<Option<JToken>>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Box<Option<JToken>>,
    #[doc = "Gets or sets the parent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[doc = "Gets the path of the JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Box<Option<JToken>>,
    #[doc = "Represents an abstract JSON token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Box<Option<JToken>>,
    #[doc = "Gets the node type for this JToken."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl JToken {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON model for JSON Patch Operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonPatchDocument {}
impl JsonPatchDocument {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The JSON model for a JSON Patch operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonPatchOperation {
    #[doc = "The path to copy from for the Move/Copy operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[doc = "The patch operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<json_patch_operation::Op>,
    #[doc = "The path for the operation. In the case of an array, a zero based index can be used to specify the position in the array (e.g. /biscuits/0/name). The \"-\" character can be used instead of an index to insert at the end of the array (e.g. /biscuits/-)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The value for the operation. This is either a primitive or a JToken."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl JsonPatchOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod json_patch_operation {
    use super::*;
    #[doc = "The patch operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Op {
        #[serde(rename = "add")]
        Add,
        #[serde(rename = "remove")]
        Remove,
        #[serde(rename = "replace")]
        Replace,
        #[serde(rename = "move")]
        Move,
        #[serde(rename = "copy")]
        Copy,
        #[serde(rename = "test")]
        Test,
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedGraphGroups {
    #[doc = "This will be non-null if there is another page of data. There will never be more than one continuation token returned by a request."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub continuation_token: Vec<String>,
    #[doc = "The enumerable list of groups found within a page."]
    #[serde(rename = "graphGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub graph_groups: Vec<GraphGroup>,
}
impl PagedGraphGroups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedGraphUsers {
    #[doc = "This will be non-null if there is another page of data. There will never be more than one continuation token returned by a request."]
    #[serde(
        rename = "continuationToken",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub continuation_token: Vec<String>,
    #[doc = "The enumerable set of users found within a page."]
    #[serde(rename = "graphUsers", default, skip_serializing_if = "Vec::is_empty")]
    pub graph_users: Vec<GraphUser>,
}
impl PagedGraphUsers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The class to represent a collection of REST reference links."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceLinks {
    #[doc = "The readonly view of the links.  Because Reference links are readonly, we only want to expose them as read only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
}
impl ReferenceLinks {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestAccessPayLoad {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(
        rename = "projectUri",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub project_uri: Option<String>,
    #[serde(
        rename = "urlRequested",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url_requested: Option<String>,
}
impl RequestAccessPayLoad {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class is used to serialized collections as a single JSON object on the wire, to avoid serializing JSON arrays directly to the client, which can be a security hole"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VssJsonCollectionWrapper {
    #[serde(flatten)]
    pub vss_json_collection_wrapper_base: VssJsonCollectionWrapperBase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl VssJsonCollectionWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VssJsonCollectionWrapperBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}
impl VssJsonCollectionWrapperBase {
    pub fn new() -> Self {
        Self::default()
    }
}