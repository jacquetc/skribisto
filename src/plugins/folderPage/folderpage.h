/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: folderpage.h                                                   *
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
#ifndef FOLDERPAGE_H
#define FOLDERPAGE_H

#include <QObject>
#include "skrpageinterface.h"
#include "skrexporterinterface.h"

class FolderPage : public QObject,
                   public SKRPageInterface,
                   public SKRExporterInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.FolderPagePlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRPageInterface SKRExporterInterface)

public:

    explicit FolderPage(QObject *parent = nullptr);
    ~FolderPage();
    QString name() const override {
        return "FolderPage";
    }

    QString displayedName() const override {
        return tr("Folder page");
    }

    QString use() const override {
        return "Display a page for the folder item";
    }

    // Page
    QString pageType() const override {
        return "FOLDER";
    }

    QString visualText() const override {
        return tr("Folder");
    }

    QString pageDetailText() const override {
        return tr("Group your items in folders.");
    }

    QString pageUrl() const override {
        return "qrc:///qml/plugins/FolderPage/FolderPage.qml";
    }

    bool isConstructible() const override {
        return true;
    }

    QString pageTypeIconUrl() const override {
        return "qrc:///icons/backup/document-open.svg";
    }

    SKRResult finaliseAfterCreationOfTreeItem(int projectId,
                                              int treeItemId) override;

    void      updateCharAndWordCount(int  projectId,
                                     int  treeItemId,
                                     bool sameThread = false)  override;

    // exporter
    QTextDocumentFragment generateExporterTextFragment(int                projectId,
                                                       int                treeItemId,
                                                       const QVariantMap& exportProperties,
                                                       SKRResult        & result) const override;

signals:

private:
};

#endif // FOLDERPAGE_H
