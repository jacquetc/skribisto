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
#ifndef SECTIONPAGE_H
#define SECTIONPAGE_H

#include <QObject>
#include "skrpageinterface.h"
#include "skrexporterinterface.h"

class SectionPage : public QObject,
                    public SKRPageInterface,
                    public SKRExporterInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.SectionPagePlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRPageInterface SKRExporterInterface)

public:

    explicit SectionPage(QObject *parent = nullptr);
    ~SectionPage();
    QString name() const override {
        return "SectionPage";
    }

    QString displayedName() const override {
        return tr("Section page", "plugin name");
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
        return "SECTION";
    }

    QString visualText() const override {
        return tr("Section");
    }

    QString pageDetailText() const override {
        return tr("Create a logical separation between items.");
    }

    QString pageUrl() const override {
        return "qrc:///qml/plugins/SectionPage/SectionPage.qml";
    }

    QString pageCreationParametersUrl() const override {
        return "qrc:///qml/plugins/SectionPage/SectionCreationParameters.qml";
    }

    bool isConstructible() const override {
        return true;
    }

    QString pageTypeIconUrl() const override {
        return "qrc:///icons/backup/artifact.svg";
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

#endif // SECTIONPAGE_H
