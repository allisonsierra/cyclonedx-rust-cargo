use crate::external_models::{
    normalized_string::NormalizedString,
    spdx::SpdxIdentifier,
    uri::{Purl, Uri},
};

#[derive(Debug, PartialEq)]
pub struct Bom {
    pub version: u32,
    pub serial_number: Option<UrnUuid>,
    pub metadata: Option<Metadata>,
    pub components: Option<Vec<Component>>,
    pub services: Option<Vec<Service>>,
    pub external_references: Option<Vec<ExternalReference>>,
    pub dependencies: Option<Vec<Dependency>>,
    pub compositions: Option<Vec<Composition>>,
    pub properties: Option<Properties>,
}

impl Default for Bom {
    fn default() -> Self {
        Self {
            version: 1,
            serial_number: Some(UrnUuid(uuid::Uuid::new_v4())),
            metadata: None,
            components: None,
            services: None,
            external_references: None,
            dependencies: None,
            compositions: None,
            properties: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AttachedText {
    pub(crate) content_type: Option<NormalizedString>,
    pub(crate) encoding: Option<Encoding>,
    pub(crate) content: String,
}

impl AttachedText {
    /// Construct a new `AttachedText`
    ///
    /// - `content_type` - Content type of the attached text (default: `"text/plain"`)
    /// - `content` - Raw content, which will be base64 encoded when added to the BOM
    pub fn new<T: AsRef<[u8]>>(content_type: Option<NormalizedString>, content: T) -> Self {
        Self {
            content_type,
            encoding: Some(Encoding::Base64),
            content: base64::encode(content),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BomReference(pub(crate) String);

#[derive(Debug, PartialEq)]
pub struct Commit {
    pub uid: Option<NormalizedString>,
    pub url: Option<Uri>,
    pub author: Option<IdentifiableAction>,
    pub committer: Option<IdentifiableAction>,
    pub message: Option<NormalizedString>,
}

#[derive(Debug, PartialEq)]
pub struct Component {
    pub component_type: Classification,
    pub mime_type: Option<MimeType>,
    pub bom_ref: Option<String>,
    pub supplier: Option<OrganizationalEntity>,
    pub author: Option<NormalizedString>,
    pub publisher: Option<NormalizedString>,
    pub group: Option<NormalizedString>,
    pub name: NormalizedString,
    pub version: NormalizedString,
    pub description: Option<NormalizedString>,
    pub scope: Option<Scope>,
    pub hashes: Option<Vec<Hash>>,
    pub licenses: Option<Vec<LicenseChoice>>,
    pub copyright: Option<NormalizedString>,
    pub cpe: Option<Cpe>,
    pub purl: Option<Purl>,
    pub swid: Option<Swid>,
    pub modified: Option<bool>,
    pub pedigree: Option<Pedigree>,
    pub external_references: Option<Vec<ExternalReference>>,
    pub properties: Option<Properties>,
    pub components: Option<Vec<Component>>,
    pub evidence: Option<ComponentEvidence>,
}

#[derive(Debug, PartialEq)]
pub struct ComponentEvidence {
    pub licenses: Option<Vec<LicenseChoice>>,
    pub copyright: Option<Vec<Copyright>>,
}

#[derive(Debug, PartialEq)]
pub struct Composition {
    pub aggregate: AggregateType,
    pub assemblies: Option<Vec<BomReference>>,
    pub dependencies: Option<Vec<BomReference>>,
}

#[derive(Debug, PartialEq)]
pub struct Copyright(Vec<String>);

#[derive(Debug, PartialEq)]
pub struct DataClassification {
    pub flow: DataFlowType,
    pub classification: NormalizedString,
}

#[derive(Debug, PartialEq)]
pub struct Dependency {
    pub dependency_ref: String,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, PartialEq)]
pub struct Diff {
    pub text: Option<AttachedText>,
    pub url: Option<Uri>,
}

#[derive(Debug, PartialEq)]
pub struct ExternalReference {
    pub external_reference_type: ExternalReferenceType,
    pub url: Uri,
    pub comment: Option<String>,
    pub hashes: Option<Vec<Hash>>,
}

#[derive(Debug, PartialEq)]
pub struct Hash {
    pub alg: HashAlgorithm,
    pub value: HashValue,
}

#[derive(Debug, PartialEq)]
pub struct IdentifiableAction {
    pub timestamp: Option<time::PrimitiveDateTime>,
    pub name: Option<NormalizedString>,
    pub email: Option<NormalizedString>,
}

#[derive(Debug, PartialEq)]
pub struct Issue {
    pub issue_type: IssueClassification,
    pub id: Option<NormalizedString>,
    pub name: Option<NormalizedString>,
    pub description: Option<NormalizedString>,
    pub source: Option<Source>,
    pub references: Option<Vec<Uri>>,
}

#[derive(Debug, PartialEq)]
pub enum LicenseChoice {
    License(Option<Vec<License>>),
    Expression(Option<NormalizedString>),
}

#[derive(Debug, PartialEq)]
pub struct License {
    pub license_identifier: LicenseIdentifier,
    pub text: Option<AttachedText>,
    pub url: Option<Uri>,
}

#[derive(Debug, PartialEq)]
pub enum LicenseIdentifier {
    SpdxId(SpdxIdentifier),
    Name(NormalizedString),
}

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub timestamp: Option<time::PrimitiveDateTime>,
    pub tools: Option<Vec<Tool>>,
    pub authors: Option<Vec<OrganizationalContact>>,
    pub component: Option<Component>,
    pub manufacture: Option<OrganizationalEntity>,
    pub supplier: Option<OrganizationalEntity>,
    pub licenses: Option<Vec<LicenseChoice>>,
    pub properties: Option<Properties>,
}

#[derive(Debug, PartialEq)]
pub struct OrganizationalContact {
    pub name: Option<NormalizedString>,
    pub email: Option<NormalizedString>,
    pub phone: Option<NormalizedString>,
}

#[derive(Debug, PartialEq)]
pub struct OrganizationalEntity {
    pub name: Option<NormalizedString>,
    pub url: Option<Vec<Uri>>,
    pub contact: Option<Vec<OrganizationalContact>>,
}

#[derive(Debug, PartialEq)]
pub struct Patch {
    pub patch_type: PatchClassification,
    pub diff: Diff,
    pub resolves: Option<Vec<Issue>>,
}

#[derive(Debug, PartialEq)]
pub struct Pedigree {
    pub ancestors: Option<Vec<Component>>,
    pub decendants: Option<Vec<Component>>,
    pub variants: Option<Vec<Component>>,
    pub commits: Option<Vec<Commit>>,
    pub patches: Option<Vec<Patch>>,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Properties(Vec<Property>);

#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: NormalizedString,
}

#[derive(Debug, PartialEq)]
pub struct Source {
    pub name: Option<NormalizedString>,
    pub url: Option<Uri>,
}

#[derive(Debug, PartialEq)]
pub struct Swid {
    pub tag_id: String,
    pub name: String,
    pub version: Option<String>,
    pub tag_version: Option<u32>,
    pub patch: Option<bool>,
    pub text: Option<AttachedText>,
    pub url: Option<Uri>,
}

#[derive(Debug, PartialEq)]
pub struct Service {
    pub bom_ref: Option<String>,
    pub provider: Option<OrganizationalEntity>,
    pub group: Option<NormalizedString>,
    pub name: NormalizedString,
    pub version: Option<NormalizedString>,
    pub description: Option<NormalizedString>,
    pub endpoints: Option<Vec<Uri>>,
    pub authenticated: Option<bool>,
    pub x_trust_boundary: Option<bool>,
    pub data: Option<Vec<DataClassification>>,
    pub licenses: Option<Vec<LicenseChoice>>,
    pub external_references: Option<Vec<ExternalReference>>,
    pub properties: Option<Properties>,
    pub services: Option<Vec<Service>>,
}

#[derive(Debug, PartialEq)]
pub struct Tool {
    pub vendor: Option<NormalizedString>,
    pub name: Option<NormalizedString>,
    pub version: Option<NormalizedString>,
    pub hashes: Option<Vec<Hash>>,
}

#[derive(Debug, PartialEq)]
pub enum AggregateType {
    Complete,
    Incomplete,
    IncompleteFirstPartyOnly,
    IncompleteThirdPartyOnly,
    Unknown,
    NotSpecified,
}

#[derive(Debug, PartialEq)]
pub enum Classification {
    Application,
    Framework,
    Library,
    Container,
    OperatingSystem,
    Device,
    Firmware,
    File,
}

#[derive(Debug, PartialEq)]
pub struct Cpe(String); // TODO: validate

#[derive(Debug, PartialEq)]
pub enum DataFlowType {
    Inbound,
    Outbound,
    BiDirectional,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Encoding {
    Base64,
    UnknownEncoding(String),
}

impl From<String> for Encoding {
    fn from(s: String) -> Self {
        match &*s {
            "base64" => Self::Base64,
            _ => Self::UnknownEncoding(s),
        }
    }
}

impl From<Encoding> for String {
    fn from(e: Encoding) -> Self {
        match e {
            Encoding::Base64 => "base64".to_string(),
            Encoding::UnknownEncoding(ue) => ue,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExternalReferenceType {
    Vcs,
    IssueTracker,
    Website,
    Advisories,
    Bom,
    MailingList,
    Social,
    Chat,
    Documentation,
    Support,
    Distribution,
    License,
    BuildMeta,
    BuildSystem,
    Other,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum HashAlgorithm {
    MD5,
    SHA1,
    SHA256,
    SHA384,
    SHA512,
    SHA3_256,
    SHA3_384,
    SHA3_512,
    BLAKE2b_256,
    BLAKE2b_384,
    BLAKE2b_512,
    BLAKE3,
}

#[derive(Debug, PartialEq)]
pub struct HashValue(String); // TODO: validate

#[derive(Debug, PartialEq)]
pub enum IssueClassification {
    Defect,
    Enhancement,
    Security,
}

#[derive(Debug, PartialEq)]
pub struct MimeType(String); // TODO: validate regex

#[derive(Debug, PartialEq)]
pub enum PatchClassification {
    Unofficial,
    Monkey,
    Backport,
    CherryPick,
}

#[derive(Debug, PartialEq)]
pub enum Scope {
    Required,
    Optional,
    Excluded,
}

#[derive(Debug, PartialEq)]
pub struct UrnUuid(uuid::Uuid);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_construct_attached_text() {
        let actual = AttachedText::new(
            Some(NormalizedString::new("text/plain")),
            "this text is plain",
        );
        assert_eq!(
            actual,
            AttachedText {
                content_type: Some(NormalizedString::new("text/plain")),
                encoding: Some(Encoding::Base64),
                content: "dGhpcyB0ZXh0IGlzIHBsYWlu".to_string(),
            }
        )
    }
}
