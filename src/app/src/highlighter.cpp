#include "highlighter.h"
#include <SonnetCore/Sonnet/Speller>
#include <SonnetCore/Sonnet/GuessLanguage>
#include <QDebug>

Highlighter::Highlighter(QTextDocument *parent)
    : QSyntaxHighlighter(parent)
{

    //Sonnet::GuessLanguage

    m_speller = new Sonnet::Speller();
    qDebug() << "dicts: " << m_speller->availableLanguages();
    qDebug() << "langs: " << m_speller->availableLanguageNames();
    qDebug() <<"backends: "<< m_speller->availableBackends();
    qDebug() <<"client: "<< m_speller->defaultClient();

    m_speller->setDefaultClient("Hunspell");
    m_speller->setLanguage("fr_FR");

    qDebug() <<"valid: "<< m_speller->isValid();
    qDebug() <<"current lang: "<< m_speller->language();


}

void Highlighter::highlightBlock(const QString &text)
{

    QTextCharFormat format;
    format.setUnderlineStyle(QTextCharFormat::UnderlineStyle::SpellCheckUnderline);
    format.setUnderlineColor(Qt::GlobalColor::red);

    QTextCharFormat emptyFormat;

    //qDebug() <<  this->currentBlock().text();

    QVector<QStringRef> splitRefVector = text.splitRef(" ", Qt::SkipEmptyParts);

    for(const QStringRef &stringRef : splitRefVector){

        if(m_speller->isMisspelled(stringRef.toString())){
            format.setFontUnderline(true);
            this->setFormat(stringRef.position(), stringRef.length(), format);
        }
        else {
            format.setFontUnderline(false);
            this->setFormat(stringRef.position(), stringRef.length(), format);

        }

    }

}

