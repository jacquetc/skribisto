/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: textpage.h                                                   *
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
#ifndef TEXTPAGE_H
#define TEXTPAGE_H

#include <QObject>
#include "skrpageinterface.h"
#include "skritemexporterinterface.h"
#include "skrsettingspanelinterface.h"
#include "skrwordmeter.h"

class TextPage : public QObject,
                 public SKRPageInterface,
                 public SKRItemExporterInterface,
                 public SKRSettingsPanelInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.TextPagePlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(SKRPageInterface SKRItemExporterInterface SKRSettingsPanelInterface)

public:

    explicit TextPage(QObject *parent = nullptr);
    ~TextPage();
    QString name() const override {
        return "TextPage";
    }

    QString displayedName() const override {
        return tr("Text page", "plugin name");
    }

    QString use() const override {
        return "Display a page for text";
    }

    QString pluginGroup() const override {
        return "Page";
    }

    QString pluginSelectionGroup() const override {
        return "Mandatory";
    }

    // Page
    QString pageType() const override {
        return "TEXT";
    }

    int weight() const override {
        return 500;
    }

    QString visualText() const override {
        return tr("Text");
    }

    QString pageDetailText() const override {
        return tr("Write here any text, be it a scene or a note");
    }

    QString pageUrl() const override {
        return "qrc:///eu.skribisto.skribisto/imports/qml/plugins/eu/skribisto/textPage/TextPage.qml";
    }

    bool isConstructible() const override {
        return true;
    }

    QString pageTypeIconUrl(int projectId, int treeItemId) const override {
        Q_UNUSED(projectId)
        Q_UNUSED(treeItemId)
        return "qrc:///icons/backup/document-edit-sign.svg";
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

    // settings:

    QString settingsPanelQmlUrl() const override    {
        return "qrc:///eu.skribisto.skribisto/imports/qml/plugins/eu/skribisto/textPage/Settings.qml";
    }

    QString settingsPanelButtonText() const override      {
        return tr("Text page");
    }

    QString settingsPanelIconSource() const override {
        return "qrc:///icons/backup/document-edit-sign.svg";
    }

    int settingsPanelWeight() const override     {
        return 600;
    }

signals:

private:

    SKRWordMeter *m_wordMeter;
};

#endif // TEXTPAGE_H
