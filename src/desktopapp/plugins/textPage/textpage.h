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
#include "interfaces/itemexporterinterface.h"
#include "interfaces/pageinterface.h"
#include "interfaces/pagedesktopinterface.h"
#include "interfaces/settingspanelinterface.h"
#include "interfaces/pagetypeiconinterface.h"
#include "skrwordmeter.h"

class TextPage : public QObject,
                 public PageInterface,
                 public PageDesktopInterface,
                 public PageTypeIconInterface,
                 public ItemExporterInterface,
                 public SettingsPanelInterface {
    Q_OBJECT
    Q_PLUGIN_METADATA(
        IID "eu.skribisto.TextPagePlugin/1.0" FILE
        "plugin_info.json")
    Q_INTERFACES(PageInterface PageDesktopInterface PageTypeIconInterface ItemExporterInterface SettingsPanelInterface)

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

    View *getView() const override;



    bool isConstructible() const override {
        return true;
    }

    QString pageTypeIconUrl(const TreeItemAddress &treeItemAddress) const override {
        Q_UNUSED(treeItemAddress)
        return ":/icons/backup/document-edit-sign.svg";
    }

    QVariantMap propertiesForCreationOfTreeItem(const QVariantMap &customProperties = QVariantMap()) const override;

    void      updateCharAndWordCount(const TreeItemAddress &treeItemAddress,
                                     bool sameThread = false, bool triggerProjectModifiedSignal = true)  override;

    // exporter
    QTextDocumentFragment generateExporterTextFragment(const TreeItemAddress &treeItemAddress,
                                                       const QVariantMap& exportProperties,
                                                       SKRResult        & result) const override;

    // settings:

    SettingsSubPanel *settingsPanel() const override;

    QString settingsPanelButtonText() const override      {
        return tr("Text page");
    }

    QString settingsGroup() const override {
        return "pages";
    }

    QString settingsPanelIconSource() const override {
        return ":/icons/backup/document-edit-sign.svg";
    }

    int settingsPanelWeight() const override     {
        return 600;
    }

signals:

private:

    SKRWordMeter *m_wordMeter;
};

#endif // TEXTPAGE_H
