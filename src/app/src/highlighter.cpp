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
    qDebug() <<"backends: "<< m_speller->availableBackends();
    qDebug() <<"client: "<< m_speller->defaultClient();

    m_speller->setLanguage("fr_FR");

    qDebug() <<"valid: "<< m_speller->isValid();


}

void Highlighter::highlightBlock(const QString &text)
{

    QTextCharFormat format;
    format.setUnderlineStyle(QTextCharFormat::UnderlineStyle::WaveUnderline);
    format.setUnderlineColor(QColor(Qt::GlobalColor::red));

    QTextCharFormat emptyFormat;

    //qDebug() <<  this->currentBlock().text();

    QVector<QStringRef> splitRefVector = text.splitRef(" ", Qt::SkipEmptyParts);

    for(const QStringRef &stringRef : splitRefVector){

        if(m_speller->isMisspelled(stringRef.toString())){
            this->setFormat(stringRef.position(), stringRef.length(), format);
        }
        else {
            this->setFormat(stringRef.position(), stringRef.length(), emptyFormat);

        }

    }

}

