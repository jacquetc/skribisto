#include "markdowntextdocument.h"
#include "QtGui/qtextcursor.h"
#include "QtGui/qtextobject.h"

#include <QRegularExpression>
#include <QTextList>

MarkdownTextDocument::MarkdownTextDocument(QObject *parent)
    : QTextDocument{parent}
{

}

QString MarkdownTextDocument::toSkribistoMarkdown() const
{

    QStringList stringList;

    for(int i = 0 ; i < this->blockCount(); i++){
        const QTextBlock &block = this->findBlockByNumber(i);

        stringList << convertFromBlockToString(block);
    }

    stringList = this->breakStrings(stringList);

    QString finalPseudoMarkdownString;

    for(int i = 0 ; i < stringList.count(); i++){

            const QString &string = stringList.at(i);

            finalPseudoMarkdownString.append(string);

            if(i != stringList.count() - 1){
                if(string.isEmpty()){
                    finalPseudoMarkdownString.append("\n");
                }
                else {
                    finalPseudoMarkdownString.append("\n\n");
                }
            }




         }

    return finalPseudoMarkdownString;
}

void MarkdownTextDocument::setSkribistoMarkdown(const QString &markdownText)
{
    this->setUndoRedoEnabled(false);
    this->clear();


    const QString text = this->cleanUpMarkdown(markdownText);
    const QStringList blockStringList = this->createBlockListFromMarkdown(text);
    QList<QTextBlock> blocks = this->createBlocks(blockStringList);
    for(const QTextBlock &block : blocks){
        this->formatBlock(block);
        this->formatCharsInBlock(block);
    }

    this->setUndoRedoEnabled(true);
}

//----------------------------------------------------------------------------------

QString MarkdownTextDocument::cleanUpMarkdown(const QString &markdownText) const
{

    QString text = markdownText;
    text.replace("[\\t\\v\\f\\r].", "");

    return text;

}

//----------------------------------------------------------------------------------

QStringList MarkdownTextDocument::createBlockListFromMarkdown(const QString &markdownText) const
{
    QStringList blockList;
    const QStringList lineList = markdownText.split("\n");

    bool hadLastLineEmpty = false;
    QString block = "";
    for(int i = 0; i < lineList.count() ; i++){
        const QString &line = lineList.at(i);

        if(line.isEmpty() && hadLastLineEmpty){
            block.clear();
            blockList << block;
            hadLastLineEmpty = true;
            continue;
        }
        else if (!line.isEmpty() && hadLastLineEmpty){ // line is not empty
            hadLastLineEmpty = false;
        }

        block += line;

        if(line.isEmpty() && !hadLastLineEmpty){
            blockList << block;
            block.clear();
            hadLastLineEmpty = true;
        }
        if(i == lineList.count() - 1){
            blockList << block;
        }

    }

    return blockList;
}

//----------------------------------------------------------------------------------

QList<QTextBlock> MarkdownTextDocument::createBlocks(const QStringList &blockStringList)
{
    QList<QTextBlock> blockList;

    QTextCursor cursor(this);
    cursor.setPosition(0);

    for(int i = 0 ; i < blockStringList.count() ; i++){
        const QString &blockString = blockStringList.at(i);
        cursor.insertText(blockString);
        blockList << cursor.block();

        // if not last string
        if(i != blockStringList.count() - 1){
            cursor.insertBlock();
        }
    }

    return blockList;

}

void MarkdownTextDocument::formatBlock(const QTextBlock &block)
{
    const QString &blockText = block.text();

    if(blockText.isEmpty()){
        return;
    }


    QTextCursor cursor(this);
    cursor.setPosition(block.position());


    // detect list :

    //TODO: implement indent

    if(blockText.length() >= 2 && blockText.first(2) == "* "){

        const QTextBlock previousBlock = block.previous();

        QTextCursor previousBlockCursor(this);
        previousBlockCursor.setPosition(previousBlock.position());
        QTextList *currentList = previousBlockCursor.currentList();

        // add to previous list if there is one
        if(block.previous().isValid() && currentList){

            currentList->add(block);

        }
        else { // create a new list
            QTextListFormat listFormat;
            listFormat.setStyle(QTextListFormat::ListDisc);
            cursor.createList(listFormat);
        }

        cursor.setPosition(block.position());
        cursor.setPosition(block.position() + 2, QTextCursor::KeepAnchor);
        cursor.removeSelectedText();
    }

}

void MarkdownTextDocument::formatCharsInBlock(const QTextBlock &block)
{

    QString blockText = block.text();

    QMap<int, MarkdownTextDocument::CharFormats> markupPositionWithCharFormatsMap;

    QTextCursor cursor(this);
    cursor.setPosition(block.position());

    // find double char markups

    for (const QRegularExpressionMatch &match : QRegularExpression("((?<!\\\\)\\*\\*.+?\\*\\*)").globalMatch(blockText)) {
        //qDebug() << match;

        if (match.hasMatch()) {
            int matchStartPos = match.capturedStart();
            int matchEndPos = match.capturedEnd();

            cursor.setPosition(block.position() + matchStartPos);
            cursor.setPosition(block.position() + matchEndPos, QTextCursor::KeepAnchor);

            markupPositionWithCharFormatsMap.insert(block.position() + matchStartPos, MarkdownTextDocument::Bold);
            markupPositionWithCharFormatsMap.insert(block.position() + matchEndPos - 2, MarkdownTextDocument::Bold);

            QTextCharFormat charFormat;
            charFormat.setFontWeight(700);

            cursor.mergeCharFormat(charFormat);
        }

     }


    //
    for (const QRegularExpressionMatch &match : QRegularExpression("((?<!\\\\)__.+?__)").globalMatch(blockText)) {
        //qDebug() << match;

        if (match.hasMatch()) {
            int matchStartPos = match.capturedStart();
            int matchEndPos = match.capturedEnd();

            cursor.setPosition(block.position() + matchStartPos);
            cursor.setPosition(block.position() + matchEndPos, QTextCursor::KeepAnchor);

            markupPositionWithCharFormatsMap.insert(block.position() + matchStartPos, MarkdownTextDocument::Underline);
            markupPositionWithCharFormatsMap.insert(block.position() + matchEndPos - 2, MarkdownTextDocument::Underline);

            QTextCharFormat charFormat;
            charFormat.setUnderlineStyle(QTextCharFormat::UnderlineStyle::SingleUnderline);
            charFormat.setFontUnderline(true);

            cursor.mergeCharFormat(charFormat);
        }

     }


    for (const QRegularExpressionMatch &match : QRegularExpression("((?<!\\\\)~~.+?~~)").globalMatch(blockText)) {
        //qDebug() << match;

        if (match.hasMatch()) {
            int matchStartPos = match.capturedStart();
            int matchEndPos = match.capturedEnd();

            cursor.setPosition(block.position() + matchStartPos);
            cursor.setPosition(block.position() + matchEndPos, QTextCursor::KeepAnchor);

            markupPositionWithCharFormatsMap.insert(block.position() + matchStartPos, MarkdownTextDocument::Strikethrough);
            markupPositionWithCharFormatsMap.insert(block.position() + matchEndPos - 2, MarkdownTextDocument::Strikethrough);

            QTextCharFormat charFormat;
            charFormat.setFontStrikeOut(true);

            cursor.mergeCharFormat(charFormat);
        }

     }

    int offset = 0;
    QMap<int, MarkdownTextDocument::CharFormats>::const_iterator i = markupPositionWithCharFormatsMap.constBegin();
    while (i != markupPositionWithCharFormatsMap.constEnd()) {

        cursor.setPosition(i.key() - offset);
        cursor.setPosition(i.key() - offset + 2, QTextCursor::KeepAnchor);
        cursor.removeSelectedText();
        offset += 2;
         ++i;
     }
    markupPositionWithCharFormatsMap.clear();


    // find single char markups


blockText = block.text();
    for (const QRegularExpressionMatch &match : QRegularExpression("((?<!\\\\)\\*.+?\\*)").globalMatch(block.text())) {
        //qDebug() << match;

        if (match.hasMatch()) {
            int matchStartPos = match.capturedStart();
            int matchEndPos = match.capturedEnd();

            cursor.setPosition(block.position() + matchStartPos);
            cursor.setPosition(block.position() + matchEndPos, QTextCursor::KeepAnchor);

            markupPositionWithCharFormatsMap.insert(block.position() + matchStartPos, MarkdownTextDocument::Italic);
            markupPositionWithCharFormatsMap.insert(block.position() + matchEndPos - 1, MarkdownTextDocument::Italic);

            QTextCharFormat charFormat;
            charFormat.setFontItalic(true);

            cursor.mergeCharFormat(charFormat);
        }

     }

    offset = 0;
    i = markupPositionWithCharFormatsMap.constBegin();
    while (i != markupPositionWithCharFormatsMap.constEnd()) {

        cursor.setPosition(i.key() - offset);
        cursor.setPosition(i.key() - offset + 1, QTextCursor::KeepAnchor);
        cursor.removeSelectedText();
        offset += 1;
         ++i;
     }



    // replace escaped *_~ by not escaped ones



}

//------------------------------------------------------
//------------------------------------------------------
//------------------------------------------------------

QString MarkdownTextDocument::convertFromBlockToString(const QTextBlock &block) const
{
    QString finalString;
    QTextCursor cursor(block);
    cursor.setPosition(block.position());


    bool wasItalic = false;
    bool wasBold = false;
    bool wasUnderline = false;
    bool wasStrikethrough = false;

    int position = 1;
    for(int i = 0 ; i < block.text().length(); i++){
        const QString &character = block.text().at(i);

        // escape *_~
        if(character.contains("[\\*_~]")){
            finalString.append(QString("\\%1").arg(character));
        }

        cursor.setPosition(block.position() + position++);

        bool isItalic = cursor.charFormat().font().italic();
        bool isBold = cursor.charFormat().font().bold();
        bool isUnderline = cursor.charFormat().font().underline();
        bool isStrikethrough = cursor.charFormat().font().strikeOut();


        if(!wasItalic && isItalic) {
            finalString.append("*");
        }
        if(!wasBold && isBold){
            finalString.append("**");
        }
        if(!wasUnderline && isUnderline){
            finalString.append("__");
        }
        if(!wasStrikethrough && isStrikethrough){
            finalString.append("~~");
        }


        if(wasStrikethrough && !isStrikethrough){
            finalString.append("~~");
        }
        if(wasUnderline && !isUnderline){
            finalString.append("__");
        }
        if(wasBold && !isBold){
            finalString.append("**");
        }
        if(wasItalic && !isItalic){
            finalString.append("*");
        }

        wasItalic = isItalic;
        wasBold = isBold;
        wasUnderline = isUnderline;
        wasStrikethrough = isStrikethrough;


        finalString.append(character);

        // if last
        if(i == block.text().length() - 1){

            if(wasStrikethrough){
                finalString.append("~~");
            }
            if(wasUnderline){
                finalString.append("__");
            }
            if(wasBold){
                finalString.append("**");
            }
            if(wasItalic){
                finalString.append("*");
            }

        }


    }

    QTextList *currentList = cursor.currentList();

    if(currentList){
        finalString.prepend("* ");
    }

    return finalString;

}

//------------------------------------------------------

QStringList MarkdownTextDocument::breakStrings(const QStringList &blockStringList) const
{
    QStringList finalList;

    const int length = 80;

    bool hadPreviousStringEmpty = false;

    for(const QString &string : blockStringList) {

        int startSlice = 0;

        int count = string.length();
        bool haveLeftover = count % length > 0 ;
        bool loops = count / length ;

        for(int i = 0; i < loops  - (haveLeftover ? 1 : 0); i++){
            finalList << string.sliced(startSlice, 80);
            startSlice += 80;
        }

        if(haveLeftover){
            finalList << string.sliced(startSlice);
        }

        if(string.isEmpty()){
            finalList << string;
        }




    }

    return finalList;

}
