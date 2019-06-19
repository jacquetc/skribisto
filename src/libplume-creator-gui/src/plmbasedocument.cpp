/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmbasedocument.cpp
*                                                  *
*  This file is part of Plume Creator.                                    *
*                                                                         *
*  Plume Creator is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Plume Creator is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#include "plmbasedocument.h"

PLMBaseDocument::PLMBaseDocument(int projectId, int documentId, const QString &documentType, const QString documentTableName,
                                 QWidget *parent) : QWidget(parent),
    m_documentId(documentId), m_projectId(projectId), m_documentType(documentType), m_documentTableName(documentTableName)
{}

void PLMBaseDocument::setDocumentId(int documentId)
{
    m_documentId = documentId;
}

int PLMBaseDocument::getDocumentId()
{
    return m_documentId;
}

QString PLMBaseDocument::getDocumentType() const
{
    return m_documentType;
}

int PLMBaseDocument::getProjectId() const
{
    return m_projectId;
}

void PLMBaseDocument::setProjectId(int value)
{
    m_projectId = value;
}

QString PLMBaseDocument::getDocumentTableName() const
{
    return m_documentTableName;
}
