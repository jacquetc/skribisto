#ifndef SKRCLIPBOARD_H
#define SKRCLIPBOARD_H

#include <QObject>
#include <QTextBlockFormat>
#include <QTextDocumentFragment>
#include "documenthandler.h"

class SKRClipboard : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString fontFamily MEMBER m_fontFamily READ fontFamily WRITE setFontFamily NOTIFY fontFamilyChanged)
    Q_PROPERTY(int fontPointSize MEMBER m_fontPointSize READ fontPointSize WRITE setFontPointSize NOTIFY fontPointSizeChanged)
    Q_PROPERTY(int textTopMargin MEMBER m_textTopMargin READ textTopMargin WRITE setTextTopMargin NOTIFY textTopMarginChanged)
    Q_PROPERTY(int textIndent MEMBER m_textIndent READ textIndent WRITE setTextIndent NOTIFY textIndentChanged)
    Q_PROPERTY(DocumentHandler *documentHandler MEMBER m_documentHandler READ documentHandler WRITE setDocumentHandler NOTIFY documentHandlerChanged)

public:
    explicit SKRClipboard(QObject *parent = nullptr);

    Q_INVOKABLE void  prepareAndSendLastClipboardText();

    QString fontFamily() const;
    void setFontFamily(const QString &fontFamily);

    int fontPointSize() const;
    void setFontPointSize(int fontPointSize);

    int textTopMargin() const;
    void setTextTopMargin(int textTopMargin);

    int textIndent() const;
    void setTextIndent(int textIndent);

    DocumentHandler *documentHandler() const;
    void setDocumentHandler(DocumentHandler *documentHandler);

signals:
    void fontFamilyChanged(QString fontFamily);
    void fontPointSizeChanged(int pointSize);
    void textTopMarginChanged(int value);
    void textIndentChanged(int value);
    void documentHandlerChanged(DocumentHandler *documentHandler);

    void lastClipboardTextPrepared(QTextDocumentFragment textFragment);

private:
    void createFormats();
private:
    DocumentHandler *m_documentHandler;
    QTextCharFormat m_charFormat;
    QTextBlockFormat m_blockFormat;
    QString m_fontFamily;
    int m_fontPointSize;
    int m_textTopMargin;
    int m_textIndent;

};

#endif // SKRCLIPBOARD_H
