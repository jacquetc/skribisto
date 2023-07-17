#pragma once

#include <QObject>
#include <QQmlEngine>
#include <QQuickTextDocument>
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

    Q_PROPERTY(QQuickTextDocument *textDocument READ textDocument WRITE setTextDocument NOTIFY textDocumentChanged)

  public:
    QQuickTextDocument *textDocument() const;
    void setTextDocument(QQuickTextDocument *textDocument);

  signals:
    void textDocumentChanged();

  private:
    QQuickTextDocument *m_quickTextDocument;
};
