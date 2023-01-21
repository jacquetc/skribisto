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
#include "interfaces/itemexporterinterface.h"
#include "interfaces/pageinterface.h"
#include "interfaces/pagedesktopinterface.h"
#include "interfaces/pagetypeiconinterface.h"
#include <view.h>

class FolderPage : public QObject,
        public PageInterface,
        public PageDesktopInterface,
        public PageTypeIconInterface,
        public ItemExporterInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
            IID "eu.skribisto.FolderPagePlugin/1.0" FILE
            "plugin_info.json")
    Q_INTERFACES(PageInterface PageDesktopInterface PageTypeIconInterface ItemExporterInterface)

public:

    explicit FolderPage(QObject *parent = nullptr);
    ~FolderPage();
    QString name() const override {
        return "FolderPage";
    }

    QString displayedName() const override {
        return tr("Folder page", "plugin name");
    }

    QString use() const override {
        return "Display a page for the folder item";
    }

    QString pluginGroup() const override {
        return "Page";
    }

    QString pluginSelectionGroup() const override {
        return "Mandatory";
    }

    // Page
    QString pageType() const override {
        return "FOLDER";
    }

    int weight() const override {
        return 600;
    }

    QString visualText() const override {
        return tr("Folder");
    }

    QString pageDetailText() const override {
        return tr("Group your items in folders.");
    }

    View * getView() const override;

    bool isConstructible() const override {
        return true;
    }

    QString pageTypeIconUrl(const TreeItemAddress &treeItemAddress) const override {
        Q_UNUSED(treeItemAddress)
        return ":/icons/backup/document-open.svg";
    }

    QVariantMap propertiesForCreationOfTreeItem(const QVariantMap &customProperties = QVariantMap()) const override;


    // exporter
    QTextDocumentFragment generateExporterTextFragment(const TreeItemAddress &treeItemAddress,
                                                       const QVariantMap& exportProperties,
                                                       SKRResult        & result) const override;

signals:

private:
};

#endif // FOLDERPAGE_H
