#include <QString>
#include <QtTest>
#include <QDebug>
#include <QUrl>
#include <QTextBlock>

#include "markdowntextdocumentproxy.h"

class MarkdownTextDocumentCase : public QObject {
    Q_OBJECT

public:

    MarkdownTextDocumentCase();

public slots:

private Q_SLOTS:

    void initTestCase();
    void cleanupTestCase();
    void init();
    void cleanup();

    void formatCharsInBlock();
    void formatCharsInBlock_nested();
    void formatCharsInBlock_wrongly_nested();
    void formatBlock();

    void setAndFromMarkdown();
    void setAndFromMarkdown_multipleEmptyLines();
    void setAndFromMarkdown_multipleEmptyLines_2();

private:

};

MarkdownTextDocumentCase::MarkdownTextDocumentCase()
{}

void MarkdownTextDocumentCase::initTestCase()
{

}

void MarkdownTextDocumentCase::cleanupTestCase()
{
}

void MarkdownTextDocumentCase::init()
{
}

void MarkdownTextDocumentCase::cleanup()
{

}

//------------------------------------------

void MarkdownTextDocumentCase::formatCharsInBlock()
{
    MarkdownTextDocumentProxy doc;
    QString originalString = "*1***2**__3__~~4~~";
    doc.setPlainText(originalString);


    for(int i = 0 ; i < doc.blockCount(); i++){

        const QTextBlock &block = doc.findBlockByNumber(i);

        doc.formatCharsInBlock(block);

    }

    QTextCursor cursor(&doc);
    cursor.setPosition(1);
    QVERIFY(cursor.charFormat().font().italic());
    cursor.setPosition(2);
    QVERIFY(cursor.charFormat().font().bold());
    cursor.setPosition(3);
    QVERIFY(cursor.charFormat().font().underline());
    cursor.setPosition(4);
    QVERIFY(cursor.charFormat().font().strikeOut());

}

//------------------------------------------

void MarkdownTextDocumentCase::formatCharsInBlock_nested()
{
    MarkdownTextDocumentProxy doc;
    QString originalString = "*1***2**__3__~~__***4***__~~";
    doc.setPlainText(originalString);


    for(int i = 0 ; i < doc.blockCount(); i++){

        const QTextBlock &block = doc.findBlockByNumber(i);

        doc.formatCharsInBlock(block);

    }

    QTextCursor cursor(&doc);
    cursor.setPosition(1);
    QVERIFY(cursor.charFormat().font().italic());
    cursor.setPosition(2);
    QVERIFY(cursor.charFormat().font().bold());
    cursor.setPosition(3);
    QVERIFY(cursor.charFormat().font().underline());
    cursor.setPosition(4);
    QVERIFY(cursor.charFormat().font().strikeOut());
    QVERIFY(cursor.charFormat().font().bold());
    QVERIFY(cursor.charFormat().font().underline());
    QVERIFY(cursor.charFormat().font().italic());

}

//------------------------------------------

void MarkdownTextDocumentCase::formatCharsInBlock_wrongly_nested()
{
    MarkdownTextDocumentProxy doc;
    QString originalString = "*1***2**__3__**~~*__4***__~~";
    doc.setPlainText(originalString);


    for(int i = 0 ; i < doc.blockCount(); i++){

        const QTextBlock &block = doc.findBlockByNumber(i);

        doc.formatCharsInBlock(block);

    }

    QTextCursor cursor(&doc);
    cursor.setPosition(1);
    QVERIFY(cursor.charFormat().font().italic());
    cursor.setPosition(2);
    QVERIFY(cursor.charFormat().font().bold());
    cursor.setPosition(3);
    QVERIFY(cursor.charFormat().font().underline());
    cursor.setPosition(4);
    QVERIFY(cursor.charFormat().font().strikeOut());
    QVERIFY(cursor.charFormat().font().bold());
    QVERIFY(cursor.charFormat().font().underline());
    QVERIFY(cursor.charFormat().font().italic());

}
//------------------------------------------


void MarkdownTextDocumentCase::formatBlock()
{
    MarkdownTextDocumentProxy doc;
    QString originalString = "* list\n* list 2";
    doc.setPlainText(originalString);


    for(int i = 0 ; i < doc.blockCount(); i++){

        const QTextBlock &block = doc.findBlockByNumber(i);

        doc.formatBlock(block);

    }

    QTextCursor cursor(&doc);
    cursor.setPosition(0);
    QTextList *currentList = cursor.currentList();
    QVERIFY(currentList);
    cursor.setPosition(5);
    currentList = cursor.currentList();
    QVERIFY(currentList);


}
//------------------------------------------


void MarkdownTextDocumentCase::setAndFromMarkdown()
{

    MarkdownTextDocumentProxy doc;
    QString originalString = "* list\n\n* list *2*";
    doc.setSkribistoMarkdown(originalString);

    QCOMPARE(doc.toSkribistoMarkdown(), originalString);


    originalString = "* **__l__**is*__~~t~~__*\n\n* list *2*";
        doc.setSkribistoMarkdown(originalString);

        QCOMPARE(doc.toSkribistoMarkdown(), originalString);
}
//------------------------------------------


void MarkdownTextDocumentCase::setAndFromMarkdown_multipleEmptyLines()
{

    MarkdownTextDocumentProxy doc;
    QString originalString = "word\n\n\n\nword";
    doc.setSkribistoMarkdown(originalString);

    QCOMPARE(doc.toSkribistoMarkdown(), originalString);


}
//------------------------------------------


void MarkdownTextDocumentCase::setAndFromMarkdown_multipleEmptyLines_2()
{

    MarkdownTextDocumentProxy doc;
    QString originalString = "word1\n\ntabcdefghij\n\nword";
    doc.setSkribistoMarkdown(originalString);

    QCOMPARE(doc.toSkribistoMarkdown(), originalString);

}
QTEST_MAIN(MarkdownTextDocumentCase)

#include "tst_markdowntextdocumentcase.moc"
