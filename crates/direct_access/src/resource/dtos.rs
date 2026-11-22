// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use common::entities::Resource;
pub use common::entities::ResourceKind;
use common::types::EntityId;
use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ResourceDto {
    pub id: EntityId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub uid: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub kind: ResourceKind,
    pub location: String,
    pub file_name: String,
    pub mime_type: String,
    pub byte_size: u64,
    pub content_hash: String,
    pub width: u64,
    pub height: u64,
}

impl From<ResourceDto> for Resource {
    fn from(dto: ResourceDto) -> Self {
        Resource {
            id: dto.id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            uid: dto.uid,
            title: dto.title,
            description: dto.description,
            kind: dto.kind,
            location: dto.location,
            file_name: dto.file_name,
            mime_type: dto.mime_type,
            byte_size: dto.byte_size,
            content_hash: dto.content_hash,
            width: dto.width,
            height: dto.height,
        }
    }
}

impl From<&ResourceDto> for Resource {
    fn from(dto: &ResourceDto) -> Self {
        Resource {
            id: dto.id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            uid: dto.uid,
            title: dto.title.clone(),
            description: dto.description.clone(),
            kind: dto.kind.clone(),
            location: dto.location.clone(),
            file_name: dto.file_name.clone(),
            mime_type: dto.mime_type.clone(),
            byte_size: dto.byte_size,
            content_hash: dto.content_hash.clone(),
            width: dto.width,
            height: dto.height,
        }
    }
}

impl From<Resource> for ResourceDto {
    fn from(entity: Resource) -> Self {
        ResourceDto {
            id: entity.id,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            uid: entity.uid,
            title: entity.title,
            description: entity.description,
            kind: entity.kind,
            location: entity.location,
            file_name: entity.file_name,
            mime_type: entity.mime_type,
            byte_size: entity.byte_size,
            content_hash: entity.content_hash,
            width: entity.width,
            height: entity.height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CreateResourceDto {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub uid: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub kind: ResourceKind,
    pub location: String,
    pub file_name: String,
    pub mime_type: String,
    pub byte_size: u64,
    pub content_hash: String,
    pub width: u64,
    pub height: u64,
}

impl From<CreateResourceDto> for Resource {
    fn from(dto: CreateResourceDto) -> Self {
        Resource {
            id: 0,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            uid: dto.uid,
            title: dto.title,
            description: dto.description,
            kind: dto.kind,
            location: dto.location,
            file_name: dto.file_name,
            mime_type: dto.mime_type,
            byte_size: dto.byte_size,
            content_hash: dto.content_hash,
            width: dto.width,
            height: dto.height,
        }
    }
}

impl From<&CreateResourceDto> for Resource {
    fn from(dto: &CreateResourceDto) -> Self {
        Resource {
            id: 0,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            uid: dto.uid,
            title: dto.title.clone(),
            description: dto.description.clone(),
            kind: dto.kind.clone(),
            location: dto.location.clone(),
            file_name: dto.file_name.clone(),
            mime_type: dto.mime_type.clone(),
            byte_size: dto.byte_size,
            content_hash: dto.content_hash.clone(),
            width: dto.width,
            height: dto.height,
        }
    }
}

impl From<Resource> for CreateResourceDto {
    fn from(entity: Resource) -> Self {
        CreateResourceDto {
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            uid: entity.uid,
            title: entity.title,
            description: entity.description,
            kind: entity.kind,
            location: entity.location,
            file_name: entity.file_name,
            mime_type: entity.mime_type,
            byte_size: entity.byte_size,
            content_hash: entity.content_hash,
            width: entity.width,
            height: entity.height,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct UpdateResourceDto {
    pub id: EntityId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub uid: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub kind: ResourceKind,
    pub location: String,
    pub file_name: String,
    pub mime_type: String,
    pub byte_size: u64,
    pub content_hash: String,
    pub width: u64,
    pub height: u64,
}

impl From<UpdateResourceDto> for Resource {
    fn from(dto: UpdateResourceDto) -> Self {
        Resource {
            id: dto.id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            uid: dto.uid,
            title: dto.title,
            description: dto.description,
            kind: dto.kind,
            location: dto.location,
            file_name: dto.file_name,
            mime_type: dto.mime_type,
            byte_size: dto.byte_size,
            content_hash: dto.content_hash,
            width: dto.width,
            height: dto.height,
        }
    }
}

impl From<&UpdateResourceDto> for Resource {
    fn from(dto: &UpdateResourceDto) -> Self {
        Resource {
            id: dto.id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            uid: dto.uid,
            title: dto.title.clone(),
            description: dto.description.clone(),
            kind: dto.kind.clone(),
            location: dto.location.clone(),
            file_name: dto.file_name.clone(),
            mime_type: dto.mime_type.clone(),
            byte_size: dto.byte_size,
            content_hash: dto.content_hash.clone(),
            width: dto.width,
            height: dto.height,
        }
    }
}

impl From<Resource> for UpdateResourceDto {
    fn from(entity: Resource) -> Self {
        UpdateResourceDto {
            id: entity.id,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
            uid: entity.uid,
            title: entity.title,
            description: entity.description,
            kind: entity.kind,
            location: entity.location,
            file_name: entity.file_name,
            mime_type: entity.mime_type,
            byte_size: entity.byte_size,
            content_hash: entity.content_hash,
            width: entity.width,
            height: entity.height,
        }
    }
}

impl From<ResourceDto> for UpdateResourceDto {
    fn from(dto: ResourceDto) -> Self {
        UpdateResourceDto {
            id: dto.id,
            created_at: dto.created_at,
            updated_at: dto.updated_at,
            uid: dto.uid,
            title: dto.title,
            description: dto.description,
            kind: dto.kind,
            location: dto.location,
            file_name: dto.file_name,
            mime_type: dto.mime_type,
            byte_size: dto.byte_size,
            content_hash: dto.content_hash,
            width: dto.width,
            height: dto.height,
        }
    }
}
