/*
 * This file is part of CycloneDX Rust Cargo.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::external_models::normalized_string::NormalizedString;
use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Bom {
    bom_format: BomFormat,
    spec_version: String,
    version: Option<u32>,
    serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<Component>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<Vec<Service>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_references: Option<Vec<ExternalReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<Vec<Dependency>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    compositions: Option<Vec<Composition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Properties>,
}

#[derive(Debug, Deserialize, Serialize)]
enum BomFormat {
    CycloneDX,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AttachedText {
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<String>,
    content: String,
}

impl From<models::AttachedText> for AttachedText {
    fn from(other: models::AttachedText) -> Self {
        Self {
            content_type: other.content_type.map(|n| n.0),
            encoding: other.encoding.map(String::from),
            content: other.content,
        }
    }
}

impl From<AttachedText> for models::AttachedText {
    fn from(other: AttachedText) -> Self {
        Self {
            content_type: other.content_type.map(NormalizedString::new_unchecked),
            encoding: other.encoding.map(models::Encoding::from),
            content: other.content,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BomReference(String);

impl From<models::BomReference> for BomReference {
    fn from(other: models::BomReference) -> Self {
        Self(other.0)
    }
}

impl From<BomReference> for models::BomReference {
    fn from(other: BomReference) -> Self {
        Self(other.0)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Commit {
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<IdentifiableAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    committer: Option<IdentifiableAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Component {
    #[serde(rename = "type")]
    component_type: String,
    #[serde(rename = "mime-type", skip_serializing_if = "Option::is_none")]
    mime_type: Option<MimeType>,
    #[serde(rename = "bom-ref", skip_serializing_if = "Option::is_none")]
    bom_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supplier: Option<OrganizationalEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    name: String,
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hashes: Option<Vec<Hash>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<Vec<LicenseChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpe: Option<Cpe>,
    #[serde(skip_serializing_if = "Option::is_none")]
    purl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    swid: Option<Swid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pedigree: Option<Pedigree>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_references: Option<Vec<ExternalReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Properties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    components: Option<Vec<Component>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<ComponentEvidence>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ComponentEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<Vec<LicenseChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<Vec<Copyright>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Composition {
    aggregate: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    assemblies: Option<Vec<BomReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<Vec<BomReference>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Copyright(Vec<String>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct DataClassification {
    flow: String,
    classification: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Dependency {
    #[serde(rename = "ref")]
    dependency_ref: String,
    depends_on: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Diff {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<AttachedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExternalReference {
    #[serde(rename = "type")]
    external_reference_type: String,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hashes: Option<Vec<Hash>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Hash {
    alg: String,
    content: HashValue,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct IdentifiableAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Issue {
    #[serde(rename = "type")]
    issue_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    references: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum LicenseChoice {
    License(Option<License>),
    Expression(Option<String>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct License {
    #[serde(flatten)]
    license_identifier: LicenseIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<AttachedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum LicenseIdentifier {
    #[serde(rename = "id")]
    SpdxId(String),
    Name(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authors: Option<Vec<OrganizationalContact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    component: Option<Component>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacture: Option<OrganizationalEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supplier: Option<OrganizationalEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<Vec<LicenseChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Properties>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct OrganizationalContact {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct OrganizationalEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact: Option<Vec<OrganizationalContact>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Patch {
    #[serde(rename = "type")]
    patch_type: String,
    diff: Diff,
    #[serde(skip_serializing_if = "Option::is_none")]
    resolves: Option<Vec<Issue>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Pedigree {
    #[serde(skip_serializing_if = "Option::is_none")]
    ancestors: Option<Vec<Component>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    descendants: Option<Vec<Component>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variants: Option<Vec<Component>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commits: Option<Vec<Commit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patches: Option<Vec<Patch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Properties(Vec<Property>);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Property {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Swid {
    tag_id: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag_version: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<AttachedText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Service {
    #[serde(rename = "bom-ref")]
    bom_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<OrganizationalEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    authenticated: Option<bool>,
    #[serde(rename = "x-trust-boundary", skip_serializing_if = "Option::is_none")]
    x_trust_boundary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Vec<DataClassification>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    licenses: Option<Vec<LicenseChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_references: Option<Vec<ExternalReference>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Properties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    services: Option<Vec<Service>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Tool {
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hashes: Option<Vec<Hash>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Cpe(String);

#[derive(Debug, Deserialize, Serialize)]
struct HashValue(String);

#[derive(Debug, Deserialize, Serialize)]
struct MimeType(String);

#[derive(Debug, Deserialize, Serialize)]
struct UrnUuid(String);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_serialize_to_json() {
        let actual = Bom {
            bom_format: BomFormat::CycloneDX,
            spec_version: "1.3".to_string(),
            version: Some(1),
            serial_number: Some("fake-uuid".to_string()),
            metadata: None,
            components: None,
            services: None,
            external_references: None,
            dependencies: None,
            compositions: None,
            properties: None,
        };

        insta::assert_json_snapshot!(actual);
    }

    #[test]
    fn it_should_serialize_a_complex_example_to_json() {
        let actual = Bom {
            bom_format: BomFormat::CycloneDX,
            spec_version: "1.3".to_string(),
            version: Some(1),
            serial_number: Some("fake-uuid".to_string()),
            metadata: Some(Metadata {
                timestamp: Some("time".to_string()),
                tools: Some(vec![Tool {
                    vendor: Some("vendor".to_string()),
                    name: Some("name".to_string()),
                    version: Some("version".to_string()),
                    hashes: Some(vec![Hash {
                        alg: "hash algorithm".to_string(),
                        content: HashValue("hash value".to_string()),
                    }]),
                }]),
                authors: Some(vec![OrganizationalContact {
                    name: Some("name".to_string()),
                    email: Some("email".to_string()),
                    phone: Some("phone".to_string()),
                }]),
                component: None,
                manufacture: Some(OrganizationalEntity {
                    name: Some("name".to_string()),
                    url: Some(vec!["url".to_string()]),
                    contact: Some(vec![OrganizationalContact {
                        name: Some("name".to_string()),
                        email: Some("email".to_string()),
                        phone: Some("phone".to_string()),
                    }]),
                }),
                supplier: Some(OrganizationalEntity {
                    name: Some("name".to_string()),
                    url: Some(vec!["url".to_string()]),
                    contact: Some(vec![OrganizationalContact {
                        name: Some("name".to_string()),
                        email: Some("email".to_string()),
                        phone: Some("phone".to_string()),
                    }]),
                }),
                licenses: Some(vec![LicenseChoice::License(Some(License {
                    license_identifier: LicenseIdentifier::Name("MIT".to_string()),
                    text: Some(AttachedText {
                        content_type: Some("content type".to_string()),
                        encoding: Some("encoding".to_string()),
                        content: "legal document".to_string(),
                    }),
                    url: Some("url".to_string()),
                }))]),
                properties: Some(Properties(vec![Property {
                    name: "property name".to_string(),
                    value: "property value".to_string(),
                }])),
            }),
            components: Some(vec![Component {
                component_type: "component type".to_string(),
                mime_type: Some(MimeType("mime type".to_string())),
                bom_ref: Some("bom ref".to_string()),
                supplier: Some(OrganizationalEntity {
                    name: Some("name".to_string()),
                    url: Some(vec!["url".to_string()]),
                    contact: Some(vec![OrganizationalContact {
                        name: Some("name".to_string()),
                        email: Some("email".to_string()),
                        phone: Some("phone".to_string()),
                    }]),
                }),
                author: Some("author".to_string()),
                publisher: Some("publisher".to_string()),
                group: Some("group".to_string()),
                name: "name".to_string(),
                version: "version".to_string(),
                description: Some("description".to_string()),
                scope: Some("scope".to_string()),
                hashes: Some(vec![Hash {
                    alg: "algorithm".to_string(),
                    content: HashValue("hash value".to_string()),
                }]),
                licenses: Some(vec![LicenseChoice::Expression(Some(
                    "license expression".to_string(),
                ))]),
                copyright: Some("copyright".to_string()),
                cpe: Some(Cpe("cpe".to_string())),
                purl: Some("purl".to_string()),
                swid: Some(Swid {
                    tag_id: "tag ID".to_string(),
                    name: "name".to_string(),
                    version: Some("version".to_string()),
                    tag_version: Some(1),
                    patch: Some(true),
                    text: Some(AttachedText {
                        content_type: Some("content type".to_string()),
                        encoding: Some("encoding".to_string()),
                        content: "content".to_string(),
                    }),
                    url: Some("url".to_string()),
                }),
                modified: Some(true),
                pedigree: Some(Pedigree {
                    ancestors: Some(vec![]),
                    descendants: Some(vec![]),
                    variants: Some(vec![]),
                    commits: Some(vec![Commit {
                        uid: Some("uid".to_string()),
                        url: Some("url".to_string()),
                        author: Some(IdentifiableAction {
                            timestamp: Some("timestamp".to_string()),
                            name: Some("name".to_string()),
                            email: Some("email".to_string()),
                        }),
                        committer: Some(IdentifiableAction {
                            timestamp: Some("timestamp".to_string()),
                            name: Some("name".to_string()),
                            email: Some("email".to_string()),
                        }),
                        message: Some("message".to_string()),
                    }]),
                    patches: Some(vec![Patch {
                        patch_type: "patch type".to_string(),
                        diff: Diff {
                            text: Some(AttachedText {
                                content_type: Some("content type".to_string()),
                                encoding: Some("encoding".to_string()),
                                content: "content".to_string(),
                            }),
                            url: Some("url".to_string()),
                        },
                        resolves: Some(vec![Issue {
                            issue_type: "issue type".to_string(),
                            id: Some("id".to_string()),
                            name: Some("name".to_string()),
                            description: Some("description".to_string()),
                            source: Some(Source {
                                name: Some("name".to_string()),
                                url: Some("url".to_string()),
                            }),
                            references: Some(vec!["reference".to_string()]),
                        }]),
                    }]),
                    notes: Some("notes".to_string()),
                }),
                external_references: Some(vec![ExternalReference {
                    external_reference_type: "external reference type".to_string(),
                    url: "url".to_string(),
                    comment: Some("comment".to_string()),
                    hashes: Some(vec![]),
                }]),
                properties: Some(Properties(vec![Property {
                    name: "property name".to_string(),
                    value: "property value".to_string(),
                }])),
                components: Some(vec![]),
                evidence: Some(ComponentEvidence {
                    licenses: Some(vec![]),
                    copyright: Some(vec![Copyright(vec!["copyright".to_string()])]),
                }),
            }]),
            services: Some(vec![Service {
                bom_ref: Some("bom ref".to_string()),
                provider: Some(OrganizationalEntity {
                    name: Some("name".to_string()),
                    url: Some(vec!["url".to_string()]),
                    contact: Some(vec![OrganizationalContact {
                        name: Some("name".to_string()),
                        email: Some("email".to_string()),
                        phone: Some("phone".to_string()),
                    }]),
                }),
                group: Some("group".to_string()),
                name: "name".to_string(),
                version: Some("version".to_string()),
                description: Some("description".to_string()),
                endpoints: Some(vec!["endpoint".to_string()]),
                authenticated: Some(true),
                x_trust_boundary: Some(true),
                data: Some(vec![DataClassification {
                    flow: "flow".to_string(),
                    classification: "value".to_string(),
                }]),
                licenses: Some(vec![]),
                external_references: Some(vec![]),
                properties: Some(Properties(vec![])),
                services: Some(vec![]),
            }]),
            external_references: Some(vec![ExternalReference {
                external_reference_type: "external reference type".to_string(),
                url: "url".to_string(),
                comment: Some("comment".to_string()),
                hashes: Some(vec![]),
            }]),
            dependencies: Some(vec![Dependency {
                dependency_ref: "dependency ref".to_string(),
                depends_on: vec!["depends on".to_string()],
            }]),
            compositions: Some(vec![Composition {
                aggregate: "aggregate".to_string(),
                assemblies: Some(vec![BomReference("bom reference".to_string())]),
                dependencies: Some(vec![BomReference("bom reference".to_string())]),
            }]),
            properties: Some(Properties(vec![Property {
                name: "property name".to_string(),
                value: "property value".to_string(),
            }])),
        };

        insta::assert_json_snapshot!(actual);
    }

    #[test]
    fn it_should_handle_licenses_correctly() {
        let actual = vec![
            LicenseChoice::Expression(Some("expression".to_string())),
            LicenseChoice::License(Some(License {
                license_identifier: LicenseIdentifier::SpdxId("id".to_string()),
                text: Some(AttachedText {
                    content_type: Some("content type".to_string()),
                    encoding: Some("encoding".to_string()),
                    content: "content".to_string(),
                }),
                url: Some("url".to_string()),
            })),
            LicenseChoice::License(Some(License {
                license_identifier: LicenseIdentifier::Name("name".to_string()),
                text: Some(AttachedText {
                    content_type: Some("content type".to_string()),
                    encoding: Some("encoding".to_string()),
                    content: "content".to_string(),
                }),
                url: Some("url".to_string()),
            })),
        ];

        insta::assert_json_snapshot!(actual);
    }
}
