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
#ifndef SKREXPORTERINTERFACE_H
#define SKREXPORTERINTERFACE_H

#include <QString>
#include <QTextDocumentFragment>
#include <QTextDocument>
#include <QTextCharFormat>
#include "skrresult.h"


class SKRExporterInterface  {

public:

    virtual ~SKRExporterInterface() {}

    virtual QString pageType() const = 0;
    virtual QTextDocumentFragment generateExporterTextFragment(int projectId, int treeItemId, const QVariantMap &exportProperties, SKRResult &result) const = 0;

    void setFormat(QTextCharFormat charFormat, QTextBlockFormat blockFormat);
    QTextCharFormat charFormat() const;
    QTextBlockFormat blockFormat() const;

    private:
    QTextCharFormat m_charFormat;
    QTextBlockFormat m_blockFormat;


};

inline void SKRExporterInterface::setFormat(QTextCharFormat charFormat, QTextBlockFormat blockFormat){
    m_charFormat = charFormat;
    m_blockFormat = blockFormat;
}

inline QTextCharFormat SKRExporterInterface::charFormat() const
{
    return m_charFormat;
}

inline QTextBlockFormat SKRExporterInterface::blockFormat() const
{
    return m_blockFormat;
}

#define SKRExporterInterface_iid "com.skribisto.ExporterInterface/1.0"


Q_DECLARE_INTERFACE(SKRExporterInterface, SKRExporterInterface_iid)


#endif // SKREXPORTERINTERFACE_H
