#ifndef DOCUMENTHANDLER_H
#define DOCUMENTHANDLER_H

#include <QObject>
#include <QQuickTextDocument>
#include <QTextCursor>

class DocumentHandler : public QObject {
    Q_OBJECT
    Q_PROPERTY(QStringList allFontFamilies READ allFontFamilies CONSTANT)

    Q_PROPERTY(
        QQuickTextDocument *
        textDocument READ textDocument WRITE setTextDocument NOTIFY textDocumentChanged)

    Q_PROPERTY(
        int cursorPosition READ cursorPosition WRITE setCursorPosition NOTIFY cursorPositionChanged)
    Q_PROPERTY(
        int selectionStart READ selectionStart WRITE setSelectionStart NOTIFY cursorPositionChanged)
    Q_PROPERTY(
        int selectionEnd READ selectionEnd WRITE setSelectionEnd NOTIFY cursorPositionChanged)

    Q_PROPERTY(QString fontFamily READ fontFamily WRITE setFontFamily NOTIFY formatChanged)
    Q_PROPERTY(qreal fontSize READ fontSize WRITE setFontSize NOTIFY formatChanged)
    Q_PROPERTY(bool italic READ italic WRITE setItalic NOTIFY formatChanged)
    Q_PROPERTY(bool bold READ bold WRITE setBold NOTIFY formatChanged)
    Q_PROPERTY(bool underline READ underline WRITE setUnderline NOTIFY formatChanged)
    Q_PROPERTY(bool strikeout READ strikeout WRITE setStrikeout NOTIFY formatChanged)
    Q_PROPERTY(QColor color READ color WRITE setColor NOTIFY formatChanged)
    Q_PROPERTY(
        Qt::Alignment alignment READ alignment WRITE setAlignment NOTIFY formatChanged)
    Q_PROPERTY(bool bulletList READ bulletList WRITE setBulletList NOTIFY formatChanged)
    Q_PROPERTY(
        bool numberedList READ numberedList WRITE setNumberedList NOTIFY formatChanged)

    Q_PROPERTY(bool canUndo READ canUndo NOTIFY canUndoChanged)
    Q_PROPERTY(bool canRedo READ canRedo NOTIFY canRedoChanged)
    Q_PROPERTY(int paperId READ paperId NOTIFY idChanged)
    Q_PROPERTY(int projectId READ projectId NOTIFY idChanged)

    Q_PROPERTY(qreal topMarginEverywhere READ topMarginEverywhere WRITE setTopMarginEverywhere NOTIFY topMarginEverywhereChanged)
    Q_PROPERTY(qreal indentEverywhere READ indentEverywhere WRITE setIndentEverywhere NOTIFY indentEverywhereChanged)

public:

    DocumentHandler(QObject *parent = nullptr);

    QStringList         allFontFamilies() const;

    QQuickTextDocument* textDocument() const;
    void                setTextDocument(QQuickTextDocument *textDocument);

    int                 cursorPosition() const;
    void                setCursorPosition(int position);

    int                 selectionStart() const;
    void                setSelectionStart(int selectionStart);

    int                 selectionEnd() const;
    void                setSelectionEnd(int selectionEnd);

    QString             fontFamily() const;
    void                setFontFamily(const QString& fontFamily);

    qreal               fontSize() const;
    void                setFontSize(qreal fontSize);

    bool                italic() const;
    void                setItalic(bool italic);

    bool                bold() const;
    void                setBold(bool bold);

    bool                underline() const;
    void                setUnderline(bool underline);

    bool                strikeout() const;
    void                setStrikeout(bool strikeout);

    QColor              color() const;
    void                setColor(const QColor& color);

    bool                canUndo() const;
    bool                canRedo() const;

    Qt::Alignment       alignment() const;
    void                setAlignment(Qt::Alignment alignment);

    bool                bulletList() const;
    void                setBulletList(bool bulletList);

    bool                numberedList() const;
    void                setNumberedList(bool numberedList);

    void                setId(const int projectId,
                              const int paperId);
    int                 paperId() const;

    int                 projectId() const;

    Q_INVOKABLE int     maxCursorPosition() const;

    qreal topMarginEverywhere() const;
    void setTopMarginEverywhere(qreal topMargin);
    qreal indentEverywhere() const;
    void setIndentEverywhere(qreal indent);

public slots:

    void addHorizontalLine();
    void indentBlock();
    void unindentBlock();

    void undo();
    void redo();

signals:

    void textDocumentChanged();
    void cursorPositionChanged();
    void formatChanged();

    void canUndoChanged(bool canUndo);
    void canRedoChanged(bool canRedo);
    void idChanged();
    void topMarginEverywhereChanged(qreal topMargin);
    void indentEverywhereChanged(qreal indent);

private:

    QQuickTextDocument *m_textDoc;
    QTextCursor m_textCursor;
    QTextCursor m_selectionCursor;

    QTextCharFormat m_nextFormat;
    int m_formatPosition;
    int m_selectionStart;
    int m_selectionEnd;
    int m_projectId;
    int m_paperId;
    qreal m_topMarginEverywhere;
    qreal m_indentEverywhere;
};

#endif // DOCUMENTHANDLER_H
