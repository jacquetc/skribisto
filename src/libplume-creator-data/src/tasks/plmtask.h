/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                   *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmtask.h                                 *
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
#ifndef PLMTASK_H
#define PLMTASK_H

#include <QObject>
#include <QVariant>
#include <QString>
#include <QList>
#include <QHash>

class PLMTask : public QObject
{
    Q_OBJECT

public:
    enum Type {Getter, Setter};
    Q_ENUM(Type)

    PLMTask(){
        m_type = Getter;
        m_returnValueQVariant = 0;
        m_returnValueGetAll = QList<QHash<QString, QVariant> >();
        m_returnValueHash = QHash<int, QVariant>();
        m_returnValueList = QList<QVariant>();
    }

    virtual void doTask(bool *ok) = 0;


    Type type() const
    {
        return m_type;
    }

    QVariant returnValue() const
    {
        return m_returnValueQVariant;
    }
    QList<QHash<QString, QVariant> > returnValueGetAll() const
    {
        return m_returnValueGetAll;
    }

    QHash<int, QVariant> returnValueHash() const
    {
        return m_returnValueHash;
    }
    QList<QVariant> returnValueList() const
    {
        return m_returnValueList;
    }

    void setReturnValue(const QVariant &returnValue)
    {
        m_returnValueQVariant = returnValue;
    }
    void setReturnValueGetAll(const QList<QHash<QString, QVariant> > &returnValue)
    {
        m_returnValueGetAll = returnValue;
    }
    void setReturnValueHash(const QHash<int, QVariant> &returnValue)
    {
        m_returnValueHash = returnValue;
    }
    void setReturnValueList(const QList<QVariant> &returnValue)
    {
        m_returnValueList = returnValue;
    }

protected:
    void setType(const Type &_type)
    {
        m_type = _type;
    }

signals:
    void taskFinished();
    void taskStarted();
    void errorSent(const QString &error);

private:
    Type m_type;
    QVariant m_returnValueQVariant;
    QList<QHash<QString, QVariant> > m_returnValueGetAll;
    QHash<int, QVariant> m_returnValueHash;
    QList<QVariant> m_returnValueList;
    QList<PLMTask *> m_subtaskList;


};
#endif // PLMTASK_H










