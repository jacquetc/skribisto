/******************************************************************************
 Copyright (C) 2025 by Cyril Jacquet                                          *
 cyril.jacquet@skribisto.eu                                                   *
                                                                              *
 This file is part of Skribisto.                                              *
                                                                              *
 Skribisto is free software: you can redistribute it and/or modify            *
 it under the terms of the GNU General Public License as published by         *
 the Free Software Foundation, either version 3 of the License, or            *
 (at your option) any later version.                                          *
                                                                              *
 Skribisto is distributed in the hope that it will be useful,                 *
 but WITHOUT ANY WARRANTY; without even the implied warranty of               *
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                *
 GNU General Public License for more details.                                 *
                                                                              *
 You should have received a copy of the GNU General Public License            *
 along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.           *
 ******************************************************************************/

#include <QObject>
#include <QTest>

// namespace SCU = Skribisto::Common::UndoRedo;

class TestUndoRedo : public QObject
{
    Q_OBJECT

  private Q_SLOTS:
    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void testInstance();

  private:
};

void TestUndoRedo::initTestCase()
{
}

void TestUndoRedo::cleanupTestCase()
{
}

void TestUndoRedo::init()
{
}

void TestUndoRedo::cleanup()
{
}
void TestUndoRedo::testInstance()
{
    // Skribisto::Common::UndoRedo::Scopes scopes(QStringList() << "scope_1"_L1
    //                                                          << "scope_2"_L1);

    QVERIFY(true);
}

QTEST_APPLESS_MAIN(TestUndoRedo)
#include "tst_undo_redo.moc"
