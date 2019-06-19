/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmbasedocument.h
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
#ifndef PLMBASEDOCUMENT_H
#define PLMBASEDOCUMENT_H

#include <QWidget>
#include "global.h"

class EXPORT_GUI PLMBaseDocument : public QWidget {
    Q_OBJECT

public:

    explicit PLMBaseDocument(int      projectId,
                             int      documentId, const QString &documentType,
                             const QString documentTableName,
                             QWidget *parent = nullptr);

    void setDocumentId(int documentId);
    int  getDocumentId();
    QString  getDocumentType() const;


    int  getProjectId() const;

    ///
    /// \brief setProjectId
    /// \param value
    /// default : -1 , meaning not project-specific;
    void setProjectId(int value);

    QString getDocumentTableName() const;

signals:

    void documentFocusActived(int id);

public slots:

private:

    int m_documentId;
    int m_projectId;
    QString m_documentType;
    QString m_documentTableName;
};


#endif // PLMBASEDOCUMENT_H
