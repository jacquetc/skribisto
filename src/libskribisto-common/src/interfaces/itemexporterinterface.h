/***************************************************************************
 *   Copyright (C) 2021 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrexporterinterface.h                                                   *
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
#ifndef ITEMEXPORTERINTERFACE_H
#define ITEMEXPORTERINTERFACE_H

#include <QString>
#include <QTextDocumentFragment>
#include <QTextDocument>
#include <QTextCharFormat>
#include "skrresult.h"
#include "treeitemaddress.h"


class ItemExporterInterface  {

public:

    virtual ~ItemExporterInterface() {}

    virtual QString pageType() const = 0;
    virtual QTextDocumentFragment generateExporterTextFragment(const TreeItemAddress &treeItemAddress, const QVariantMap &exportProperties, SKRResult &result) const = 0;

    void setFormat(QTextCharFormat charFormat, QTextBlockFormat blockFormat);
    QTextCharFormat charFormat() const;
    QTextBlockFormat blockFormat() const;

    private:
    QTextCharFormat m_charFormat;
    QTextBlockFormat m_blockFormat;


};

inline void ItemExporterInterface::setFormat(QTextCharFormat charFormat, QTextBlockFormat blockFormat){
    m_charFormat = charFormat;
    m_blockFormat = blockFormat;
}

inline QTextCharFormat ItemExporterInterface::charFormat() const
{
    return m_charFormat;
}

inline QTextBlockFormat ItemExporterInterface::blockFormat() const
{
    return m_blockFormat;
}

#define ItemExporterInterface_iid "com.skribisto.ItemExporterInterface/1.0"


Q_DECLARE_INTERFACE(ItemExporterInterface, ItemExporterInterface_iid)


#endif // ITEMEXPORTERINTERFACE_H
