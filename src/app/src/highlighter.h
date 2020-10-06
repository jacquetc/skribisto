#ifndef HIGHLIGHTER_H
#define HIGHLIGHTER_H

#include <QSyntaxHighlighter>
#include <SonnetCore/Sonnet/Speller>

class Highlighter : public QSyntaxHighlighter
{
    Q_OBJECT

public:
    Highlighter(QTextDocument *parent);

protected:
    void highlightBlock(const QString &text) override;


private:
    Sonnet::Speller *m_speller;

};

#endif // HIGHLIGHTER_H
