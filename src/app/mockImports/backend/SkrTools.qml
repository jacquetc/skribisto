/****************************************************************************
**
** Copyright (C) 2020 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of Qt Creator.
**
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 3 as published by the Free Software
** Foundation with exceptions as appearing in the file LICENSE.GPL3-EXCEPT
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-3.0.html.
**
****************************************************************************/
pragma Singleton

import QtQuick

QtObject {
    id: root

    property string currentTranslationLanguageCode: "en_US"
    function setCurrentTranslationLanguageCode(langCode){
        return "en_US"
    }

    function applyLanguageFromSettings(){}


    function getLanguageFromSettings(){
        return "en_US"
    }


    function skribistoVersion(){ return "0.0.1" }
    function toLocaleDateTimeFormat(dateTime){ return dateTime }
    function toLocaleIntString(number){ return  "1,000"}

    function getQtVersion(){ return "6" }
    function hasPrintSupport(){ return false }
    function defaultFontFamily(){ return "Arial" }

    function findAvailableTranslationsMap() {

        return ({}) }

    //Q_INVOKABLE static QString cleanUpHtml(const QString& html);

    function createPath(path) { return true }
    function getWritableAddonsPathsListDir() { return "./" }

    function getTempPath() { return "./" }

    function getDictFoldersFromGitHubTree(treeFile) {  return []}

    function removeFile(fileName) { }

    function getOnlyLanguageFromLocale(name){
        return "en"
    }
    function getNativeCountryNameFromLocale(name){
        return "United-States"
    }
    function getNativeLanguageNameFromLocale(name){
        return "english"
    }

}
