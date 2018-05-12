/*
 *   Copyright 2017 Marco Martin <mart@kde.org>
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU Library General Public License as
 *   published by the Free Software Foundation; either version 2 or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU Library General Public License for more details
 *
 *   You should have received a copy of the GNU Library General Public
 *   License along with this program; if not, write to the
 *   Free Software Foundation, Inc.,
 *   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

import QtQuick 2.6
import org.kde.kirigami 2.4 as Kirigami

import "templates" as T

/**
 * This is the base class for Form layouts conforming to the
 * Kirigami Human interface guidelines. The layout will
 * be divided in two columns: on the right there will be a column
 * of fields, on the left their labels specified in the FormData attached
 * property.
 *
 * Example:
 * @code
 * import org.kde.kirigami 2.3 as Kirigami
 * Kirigami.FormLayout {
 *    TextField {
 *       Kirigami.FormData.label: "Label:"
 *    }
 *    Kirigami.Separator {
 *        Kirigami.FormData.label: "Section Title"
 *        Kirigami.FormData.isSection: true
 *    }
 *    TextField {
 *       Kirigami.FormData.label: "Label:"
 *    }
 *    TextField {
 *    }
 * }
 * @endcode
 * @inherits T.FormLayout
 * @since 2.3
 */
T.FormLayout {
    
}
