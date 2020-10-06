#include "highlighter.h"
#include <SonnetCore/Sonnet/Speller>
#include <SonnetCore/Sonnet/GuessLanguage>
#include <QDebug>

Highlighter::Highlighter(QTextDocument *parent)
    : QSyntaxHighlighter(parent)
{

    //Sonnet::GuessLanguage

    Sonnet::Speller speller;
    qDebug() << "dicts: " << speller.availableLanguages();
    qDebug() <<"backends: "<< speller.availableBackends();

    speller.setLanguage("fr_FR");

    m_checker = new Sonnet::BackgroundChecker(speller, this);
    m_checker->setAutoDetectLanguageDisabled (true);
    connect(m_checker,&Sonnet::BackgroundChecker::misspelling, this, &Highlighter::misspelling, Qt::DirectConnection);
}

void Highlighter::highlightBlock(const QString &text)
{
    m_checker->setText(text);
}

void Highlighter::misspelling(const QString &word, int start){

//    QTextBlock textBlock = this->currentBlock();
//    textBlock.position();
    qDebug() << "word: " << word << " pos : " << start;

    QTextCharFormat format;
    format.setUnderlineStyle(QTextCharFormat::UnderlineStyle::WaveUnderline);
    format.setUnderlineColor(QColor(Qt::GlobalColor::red));
    format.setFontUnderline(true);


    this->setFormat(start, word.count(), format);

    m_checker->continueChecking();
}
