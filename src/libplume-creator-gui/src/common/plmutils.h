/***************************************************************************
 *   Copyright (C) 2012 by Cyril Jacquet                                   *
 *   cyril.jacquet@plume-creator.eu                                                 *
 *                                                                         *
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
#ifndef PLMUTILS_H
#define PLMUTILS_H

#include <QString>
#include <QStringList>
#include <QHash>
#include <QDir>
#include <QModelIndex>
#include <QTranslator>

// c = class name; e = enum name; v = enum value
#define ENUM_TO_STRING(c, e, v) \
    (c::staticMetaObject.enumerator(c::staticMetaObject.indexOfEnumerator(#e)).valueToKey(v))
// c = class name; e = enum name; s = string value
#define STRING_TO_ENUM(c, e, s) \
    (c::e)(c::staticMetaObject.enumerator(c::staticMetaObject.indexOfEnumerator(#e)).keyToValue(s.toAscii()))
// c = class name; f = flag name, v = flag value
#define FLAG_TO_STRING(c, f, v) \
    (c::staticMetaObject.enumerator(c::staticMetaObject.indexOfEnumerator(#f)).valueToKeys(v))
// c = class name; f = flag name; s = string value
#define STRING_TO_FLAG(c, f, s) \
    (c::f)(c::staticMetaObject.enumerator(c::staticMetaObject.indexOfEnumerator(#f)).keysToValue(s.toAscii()))
//c = class name; e = enum name
#define ENUM_KEY_COUNT(c,e) \
    (c::staticMetaObject.enumerator(c::staticMetaObject.indexOfEnumerator(#e)).keyCount())


//    static int addProjectToSettingsArray(QString fileName);


//    static bool isProjectFromOldSystem(QString file);

//    static QString projectRealName(QString fileName);

//    static QString parseHtmlText(QString htmlText);

//    static void applyAttributeRecursively(QDomElement element, QString attribute, QString value);

//    static QList<QDomElement> allChildElements(QDomElement element);



namespace PLMUtils {



class Dir
{
public:

    static bool removeDir(const QString &dirName);
    static QStringList addonsPathsList();
    static void createPath(QStringList paths);
    static void createPath(QString path);

};

//---------------------------------------------------------------------

class Models
{
public:

    static QModelIndexList allChildIndexes(QModelIndex index);
    static QModelIndexList allParentIndexes(QModelIndex index);

};

//---------------------------------------------------------------------

class ProjectsArrayInSettings
{
public:
    static bool modifyProjectModifiedDateInSettingsArray(int arrayNumber, QString date);
    static int findProjectInSettingArray(QString fileName);
    static bool isProjectExistingInSettingArray(QString fileName);
    static int adaptSettingArrayToProject(QString fileName);
    static void sortSettingsArray();
    static QHash<QString, QString> listAllProjectsInSettingsArray(); // QHash< path, name>
};

//---------------------------------------------------------------------

class Misc
{
public:

    static QString spaceInNumber(const QString numberString = "",const QString symbol = " ");
    static QString updateProjectIfOldSystem(QString file);



    static int uniqueId(){
        ++id;
        return id;
    }

private:
    static int id;

};


class Lang
{
public:


    static QString getUserLang();
    static void setUserLang(const QString &value);
    static void setUserLangFile(const QString &fileName);


private:

    static QString projectLang;
    static QTranslator projectTranslator;
    static QString userLang;
    static QTranslator userTranslator;

};

}

#endif // PLMUTILS_H
