/******************************************************************************
 Copyright (C) 2025 by Cyril Jacquet                                          *
 cyril.jacquet@skribisto.eu                                                   *
                                                                              *
 This file is part of Skribisto.                                              *
                                                                              *
 Skribisto is free software: you can redistribute it and/or modify            *
 it under the terms of the GNU General Public License as published by         *
 the Free Software Foundation, either version 3 of the License, or            *
 (at your option) any later version.                                          *
                                                                              *
 Skribisto is distributed in the hope that it will be useful,                 *
 but WITHOUT ANY WARRANTY; without even the implied warranty of               *
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                *
 GNU General Public License for more details.                                 *
                                                                              *
 You should have received a copy of the GNU General Public License            *
 along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.           *
 ******************************************************************************/

#include "get_relationship_ids_in_range_uc.h"

namespace Skribisto::DirectAccess::Root
{

QList<int> GetRelationshipIdsInRangeUseCase::execute(int rootId, RootRelationshipField relationship, int offset, int limit)
{
    m_uow->beginTransaction();
    auto result = m_uow->getRootRelationshipInRange(rootId, DtoMapper::toCommonRelationshipField(relationship), offset, limit);
    m_uow->endTransaction();

    return result;
}

} // namespace Skribisto::DirectAccess::Root