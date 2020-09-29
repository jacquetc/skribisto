#include "highlighter.h"
#include <SonnetCore/Sonnet/Speller>
#include <QDebug>

Highlighter::Highlighter(QTextDocument *parent)
    : QSyntaxHighlighter(parent), currentTextBlock("")
{

    Sonnet::Speller speller;
    qDebug() << "dicts: " << speller.availableLanguages();
    qDebug() <<"backends: "<< speller.availableBackends();

    m_checker = new Sonnet::BackgroundChecker(speller, this);
    m_checker->setAutoDetectLanguageDisabled (false);
    connect(m_checker,&Sonnet::BackgroundChecker::misspelling, this, &Highlighter::misspelling);
}

void Highlighter::highlightBlock(const QString &text)
{
    m_checker->setText(text);
}

void Highlighter::misspelling(const QString &word, int start){


    qDebug() << "word: " << word << " pos : " << start;

    m_checker->continueChecking();
}
