/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: projectpage.h                                                   *
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
#ifndef PROJECTPAGE_H
#define PROJECTPAGE_H

#include <QObject>
#include "skrpageinterface.h"
#include "skrexporterinterface.h"

class ProjectPage : public QObject,
                    public SKRPageInterface,
                    public SKRExporterInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.ProjectPagePlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRPageInterface SKRExporterInterface)

public:

    explicit ProjectPage(QObject *parent = nullptr);
    ~ProjectPage();
    QString name() const override {
        return "ProjectPage";
    }

    QString displayedName() const override {
        return tr("Project page", "plugin name");
    }

    QString use() const override {
        return "Display a page for the project item";
    }

    QString pluginGroup() const override {
        return "Page";
    }

    QString pluginSelectionGroup() const override {
        return "Mandatory";
    }

    // Page
    QString pageType() const override {
        return "PROJECT";
    }

    int weight() const override {
        return 500;
    }

    QString visualText() const override {
        return tr("Project");
    }

    QString pageDetailText() const override {
        return ""; // useless
    }

    QString pageUrl() const override {
        return "qrc:///qml/plugins/ProjectPage/ProjectPage.qml";
    }

    bool isConstructible() const override {
        return false;
    }

    QString pageTypeIconUrl(int projectId, int treeItemId) const override {
        Q_UNUSED(projectId)
        Q_UNUSED(treeItemId)
        return "qrc:///icons/backup/address-book-new.svg";
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

#endif // PROJECTPAGE_H
