#pragma once

#include <QObject>
#include <QQmlEngine>
#include <QQuickTextDocument>
#include <QTextBlock>
#include <QTextBlockUserData>
#include <QTextCursor>
#include <QUuid>

struct BlockUserData : public QTextBlockUserData
{
    int id;
    QUuid uuid;
};

class DocumentHandler : public QObject
{
    QML_ELEMENT
    Q_OBJECT

    Q_PROPERTY(QQuickTextDocument *quickTextDocument READ quickTextDocument WRITE setQuickTextDocument NOTIFY
                   quickTextDocumentChanged)

  public:
    QQuickTextDocument *quickTextDocument() const;
    void setQuickTextDocument(QQuickTextDocument *quickTextDocument);

  signals:
    void quickTextDocumentChanged();

  private slots:
    void onContentsChange(int position, int charsRemoved, int charsAdded);

  private:
    QQuickTextDocument *m_quickTextDocument;
    QTextDocument *m_textDocument;
};
