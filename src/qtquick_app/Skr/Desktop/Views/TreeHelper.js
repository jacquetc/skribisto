.pragma library

function getChildren(model, parentIndex) {
    var children = [];
    if (parentIndex < 0 || parentIndex >= model.count) return children;
    var parentIndent = model.get(parentIndex).indent;
    for (var i = parentIndex + 1; i < model.count; i++) {
        var indent = model.get(i).indent;
        if (indent <= parentIndent) break;
        if (indent === parentIndent + 1) children.push(i);
    }
    return children;
}

function hasChildren(model, index) {
    if (index < 0 || index >= model.count - 1) return false;
    return model.get(index + 1).indent > model.get(index).indent;
}

function isVisible(model, index, expandedIds) {
    var item = model.get(index);
    if (item.indent === 0) return true;
    var targetIndent = item.indent - 1;
    for (var i = index - 1; i >= 0; i--) {
        var ancestor = model.get(i);
        if (ancestor.indent === targetIndent) {
            if (!expandedIds.has(ancestor.itemId)) return false;
            if (targetIndent === 0) return true;
            targetIndent--;
        }
    }
    return true;
}

function getSubtreeEnd(model, index) {
    var indent = model.get(index).indent;
    for (var i = index + 1; i < model.count; i++) {
        if (model.get(i).indent <= indent) return i;
    }
    return model.count;
}

function findIndexByItemId(model, itemId) {
    for (var i = 0; i < model.count; i++) {
        if (model.get(i).itemId === itemId) return i;
    }
    return -1;
}

function getChildItems(model, parentItemId) {
    if (parentItemId === -1) {
        var items = [];
        for (var i = 0; i < model.count; i++) {
            if (model.get(i).indent === 0) {
                var obj = copyItem(model.get(i));
                obj.modelIndex = i;
                items.push(obj);
            }
        }
        return items;
    }
    var parentIndex = findIndexByItemId(model, parentItemId);
    if (parentIndex < 0) return [];
    var childIndices = getChildren(model, parentIndex);
    var result = [];
    for (var j = 0; j < childIndices.length; j++) {
        var obj2 = copyItem(model.get(childIndices[j]));
        obj2.modelIndex = childIndices[j];
        result.push(obj2);
    }
    return result;
}

function copyItem(item) {
    return {
        itemId: item.itemId,
        title: item.title,
        subTitle: item.subTitle,
        role: item.role,
        subRole: item.subRole,
        label: item.label,
        activated: item.activated,
        isFavorite: item.isFavorite,
        isPrintable: item.isPrintable,
        indent: item.indent,
        wordCountGoal: item.wordCountGoal,
        charCountGoal: item.charCountGoal,
        dictLanguage: item.dictLanguage
    };
}

// --- Hierarchy validation ---

// Returns true if candidateItemId is inside the subtree of ancestorItemId
function isDescendant(model, ancestorItemId, candidateItemId) {
    var ancestorIdx = findIndexByItemId(model, ancestorItemId);
    if (ancestorIdx < 0) return false;
    var candidateIdx = findIndexByItemId(model, candidateItemId);
    if (candidateIdx < 0) return false;
    var subtreeEnd = getSubtreeEnd(model, ancestorIdx);
    return candidateIdx > ancestorIdx && candidateIdx < subtreeEnd;
}

// Returns true if dropping draggedIds onto targetId is invalid
// (target is one of the dragged items or a descendant of any)
function isInvalidDropTarget(model, draggedIds, targetId) {
    for (var i = 0; i < draggedIds.length; i++) {
        if (draggedIds[i] === targetId) return true;
        if (isDescendant(model, draggedIds[i], targetId)) return true;
    }
    return false;
}

// Remove children from selection when their ancestor is also selected.
// Returns a new array of pruned item IDs.
function pruneSelection(model, selectedIds) {
    var ids = Array.from(selectedIds);
    var pruned = [];
    for (var i = 0; i < ids.length; i++) {
        var dominated = false;
        for (var j = 0; j < ids.length; j++) {
            if (i !== j && isDescendant(model, ids[j], ids[i])) {
                dominated = true;
                break;
            }
        }
        if (!dominated) pruned.push(ids[i]);
    }
    return pruned;
}

// --- Role helpers ---

function roleIcon(role) {
    switch (role) {
    case "folder": return "\u{1F4C1}"; // folder
    case "item":   return "\u{1F4C4}"; // page
    default:       return "\u{1F4C3}"; // page with curl
    }
}

function acceptsInto(role) {
    return role === "folder";
}

// --- Drop zone computation ---
// Returns "before", "after", or "into"
function computeDropZone(localY, itemHeight, targetRole) {
    var quarter = itemHeight * 0.25;
    if (localY < quarter)
        return "before";
    if (localY > itemHeight - quarter)
        return "after";
    if (acceptsInto(targetRole))
        return "into";
    return (localY < itemHeight * 0.5) ? "before" : "after";
}

// Horizontal variant for card/grid layouts
function computeDropZoneH(localX, itemWidth, targetRole) {
    var quarter = itemWidth * 0.25;
    if (localX < quarter)
        return "before";
    if (localX > itemWidth - quarter)
        return "after";
    if (acceptsInto(targetRole))
        return "into";
    return (localX < itemWidth * 0.5) ? "before" : "after";
}

// --- Selection helpers ---

// Build a range of visible model indices between two indices (inclusive)
function visibleRange(model, fromIndex, toIndex, expandedIds) {
    var start = Math.min(fromIndex, toIndex);
    var end = Math.max(fromIndex, toIndex);
    var ids = [];
    for (var i = start; i <= end; i++) {
        if (isVisible(model, i, expandedIds)) {
            ids.push(model.get(i).itemId);
        }
    }
    return ids;
}
