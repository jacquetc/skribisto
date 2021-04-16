/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmmodels.cpp                                                   *
*  This file is part of Skribisto.                                    *
*                                                                         *
*  Skribisto is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Skribisto is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#include "skrmodels.h"

SKRModels::SKRModels(QObject *parent) : QObject(parent)
{
    m_instance      = this;
    m_treeModel     = new SKRTreeModel(this);
    m_treeListModel = new SKRTreeListModel(this);
    m_tagListModel  = new SKRTagListModel(this);

    m_writeDocumentListModel = new PLMWriteDocumentListModel(this);
}

SKRModels::~SKRModels()
{}

SKRModels *SKRModels::m_instance = nullptr;

SKRTreeModel * SKRModels::treeModel()
{
    return m_treeModel;
}

SKRTreeListModel * SKRModels::treeListModel()
{
    return m_treeListModel;
}

SKRTagListModel * SKRModels::tagListModel()
{
    return m_tagListModel;
}

PLMWriteDocumentListModel * SKRModels::writeDocumentListModel()
{
    return m_writeDocumentListModel;
}
