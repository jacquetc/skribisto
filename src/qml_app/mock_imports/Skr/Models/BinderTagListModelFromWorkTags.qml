/*
 * Copyright (C) 2025 by Cyril Jacquet
 * cyril.jacquet@skribisto.eu
 *
 * This file is part of Skribisto.
 *
 * Skribisto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Skribisto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.
 */

import QtQuick

ListModel {
    // BinderTag list for a given Work.tags relation
    property int workId: 1

    ListElement {
        createdAt: "2020-01-01T00:00:00"
        updatedAt: "2020-01-01T00:00:00"
        itemId: 1
        name: "Tag 1"
        color: "#3366ff"
        textColor: "#ffffff"
    }
    ListElement {
        createdAt: "2020-01-02T00:00:00"
        updatedAt: "2020-01-02T00:00:00"
        itemId: 2
        name: "Tag 2"
        color: "#ff9933"
        textColor: "#000000"
    }
}