// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// ═══════════════════════════════════════════════════════════════════════
// Leaf entity (no forward relationships) — macro implementation
// ═══════════════════════════════════════════════════════════════════════

use super::resource_repository::ResourceTable;
use super::resource_repository::ResourceTableRO;
use crate::entities::Resource;
crate::impl_leaf_entity_table!(
    entity: Resource,
    entity_name: "resource",
    store_field: resources,
    table_trait: ResourceTable,
    table_ro_trait: ResourceTableRO,
    table_struct: ResourceHashMapTable,
    table_ro_struct: ResourceHashMapTableRO,
    backward_junctions: [

        (jn_resource_from_binder_item_resources),
        (jn_resource_from_work_resources),
    ],
);
