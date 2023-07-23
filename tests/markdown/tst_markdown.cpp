#include "from_markdown.h"
#include "to_markdown.h"
#include <QTextBlock>
#include <QTextCursor>
#include <QTextDocument>
#include <QtTest>

class TestMarkdown : public QObject
{
    Q_OBJECT

  private slots:
    void testFromMarkdownToPlainText_data();
    void testFromMarkdownToPlainText();
    void testFromMarkdown_data();
    void testFromMarkdown();
    void testToMarkdown_data();
    void testToMarkdown();

  private:
    QTextDocument *doc = new QTextDocument();

    QString wrapHtml(const QString &html) const;
};

void TestMarkdown::testFromMarkdownToPlainText_data()
{
    QTest::addColumn<QString>("markdownInput");
    QTest::addColumn<QString>("expectedOutput");

    // Test cases
    QTest::newRow("Basic Bold") << "**Hello**"
                                << "Hello";
    QTest::newRow("Basic Italic") << "*Hello*"
                                  << "Hello";
    QTest::newRow("Basic Underline") << "_Hello_"
                                     << "Hello";
    QTest::newRow("Basic Strikeout") << "~~Hello~~"
                                     << "Hello";
    QTest::newRow("Nested Formatting") << "***Hello***"
                                       << "Hello";
    QTest::newRow("Image") << "![Alt Text](image.png)"
                           << "\uFFFC";
    QTest::newRow("Hyperlink") << "[Link Text](https://www.example.com)"
                               << "Link Text";
    QTest::newRow("Mixed Formatting") << "_*Hello*_ ~~World~~ [Link](https://www.example.com)"
                                      << "Hello World Link";
    QTest::newRow("List Mixed Formatting") << "1. _*Hello*_"
                                           << "Hello";
    QTest::newRow("Indent List Mixed Formatting") << "    1. _*Hello*_"
                                                  << "Hello";
    QTest::newRow("Indent 2 List Mixed Formatting") << "        1. _*Hello*_"
                                                    << "Hello";
    QTest::newRow("False Indent List Mixed Formatting") << "1.     _*Hello*_"
                                                        << "    Hello";
    QTest::newRow("Unordered List Mixed Formatting") << "+ _*Hello*_"
                                                     << "Hello";
}

void TestMarkdown::testFromMarkdownToPlainText()
{
    QFETCH(QString, markdownInput);
    QFETCH(QString, expectedOutput);

    QTextDocument doc;
    QTextCursor cursor(&doc);
    QTextBlock block = doc.begin();
    Markdown::From::fromMarkdown(block, markdownInput);

    QString actualText;
    for (QTextBlock currentBlock = doc.begin(); currentBlock != doc.end(); currentBlock = currentBlock.next())
    {
        actualText += currentBlock.text();
    }

    QCOMPARE(actualText, expectedOutput);
}

QString TestMarkdown::wrapHtml(const QString &html) const
{
    QString string =
        "<!DOCTYPE HTML PUBLIC \"-//W3C//DTD HTML 4.0//EN\" "
        "\"http://www.w3.org/TR/REC-html40/strict.dtd\">\n<html><head><meta name=\"qrichtext\" content=\"1\" /><meta "
        "charset=\"utf-8\" /><style type=\"text/css\">\np, li { white-space: pre-wrap; }\nhr { height: 1px; "
        "border-width: 0; }\nli.unchecked::marker { content: \"\\2610\"; }\nli.checked::marker { content: \"\\2612\"; "
        "}\n</style></head><body style=\" font-family:'.AppleSystemUIFont'; font-size:13pt; font-weight:400; "
        "font-style:normal;\">\n<p style=\" margin-top:0px; margin-bottom:0px; margin-left:0px; margin-right:0px; "
        "-qt-block-indent:0; text-indent:0px;\">" +
        html + "</p></body></html>";
    string = string.replace("<b>", "<span style=\" font-weight:700;\">");
    string = string.replace("</b>", "</span>");

    return string;
}

void TestMarkdown::testFromMarkdown_data()
{
}

void TestMarkdown::testFromMarkdown()
{
}

void TestMarkdown::testToMarkdown_data()
{
    QTest::addColumn<QString>("richTextInput");
    QTest::addColumn<QString>("expectedMarkdown");

    // Test cases
    QTest::newRow("Basic Bold") << "<b>Hello</b>"
                                << "**Hello**";
    QTest::newRow("Basic Italic") << "<i>Hello</i>"
                                  << "*Hello*";
    QTest::newRow("Basic Underline") << "<u>Hello</u>"
                                     << "_Hello_";
    QTest::newRow("Basic Strikeout") << "<s>Hello</s>"
                                     << "~~Hello~~";
    QTest::newRow("Nested Formatting") << "<b><i>Hello</i></b>"
                                       << "***Hello***";
    QTest::newRow("Image") << "<img src=\"image.png\" alt=\"Alt Text\">"
                           << "![](image.png)";
    QTest::newRow("Hyperlink") << "<a href=\"https://www.example.com\">Link Text</a>"
                               << "_[Link Text](https://www.example.com)_";
    QTest::newRow("Mixed Formatting") << "<u><i>Hello</i></u> <s>World</s> <a href=\"https://www.example.com\">Link</a>"
                                      << "*_Hello_* ~~World~~ _[Link](https://www.example.com)_";
}

void TestMarkdown::testToMarkdown()
{
    QFETCH(QString, richTextInput);
    QFETCH(QString, expectedMarkdown);

    QTextDocument doc;
    doc.setHtml(richTextInput);

    QTextBlock block = doc.begin();
    QString actualMarkdown = Markdown::To::toMarkdown(block);

    QCOMPARE(actualMarkdown, expectedMarkdown);
}

QTEST_MAIN(TestMarkdown)

#include "tst_markdown.moc"
