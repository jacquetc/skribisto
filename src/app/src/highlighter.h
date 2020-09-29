#ifndef HIGHLIGHTER_H
#define HIGHLIGHTER_H

#include <QSyntaxHighlighter>
#include <SonnetCore/Sonnet/BackgroundChecker>

class Highlighter : public QSyntaxHighlighter
{
    Q_OBJECT

public:
    Highlighter(QTextDocument *parent);

protected:
    void highlightBlock(const QString &text) override;

private slots:
    void misspelling(const QString &word, int start);
private:
    Sonnet::BackgroundChecker *m_checker;

};

#endif // HIGHLIGHTER_H
