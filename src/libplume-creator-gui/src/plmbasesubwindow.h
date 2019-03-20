/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmbasesubwindow.h
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
#ifndef PLMBASESUBWINDOW_H
#define PLMBASESUBWINDOW_H
#include <QMainWindow>
#include <QMouseEvent>
#include <QDebug>
#include "plmbasedocument.h"

namespace Ui {
class PLMBaseSubWindow;
}
class PLMBaseSubWindow : public QMainWindow {
    Q_OBJECT

public:

    PLMBaseSubWindow(int      id,
                     QWidget *parent = nullptr);

    ~PLMBaseSubWindow();
    int id() {
        return m_id;
    }

    void addDocument(PLMBaseDocument *document);

public slots:

    void clearProject(int projectId);

protected:

    void mousePressEvent(QMouseEvent *event);

signals:

    void subWindowClosed(int id);
    void splitCalled(Qt::Orientation orientation,
                     int             id);

    void subWindowFocusActived(int id);
    void documentAdded(int projectId,
                       int documentId);

private:

    int m_id;
    Ui::PLMBaseSubWindow *ui;
    QList<PLMBaseDocument *>m_documentList;
    void setupActions();
};

#endif // PLMBASESUBWINDOW_H
