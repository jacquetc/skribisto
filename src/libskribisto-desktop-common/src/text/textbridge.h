#ifndef TEXTBRIDGE_H
#define TEXTBRIDGE_H

#include "QtWidgets/qapplication.h"
#include <QObject>
#include <QPointer>
#include <QTextDocument>
#include <QString>
#define textBridge TextBridge::instance()

class TextBridge;
struct SyncDocument;

struct SyncDocument {
  Q_GADGET

public:
  explicit SyncDocument();
  SyncDocument(const QString &uniqueReference,
               const QString &skrTextAreaUniqueObjectName,
               QTextDocument *textDocument);
  SyncDocument(const SyncDocument &syncDocument);

  bool operator!() const;
  operator bool() const;
  SyncDocument &operator=(const SyncDocument &otherSyncDocument);
  bool operator==(const SyncDocument &otherSyncDocument) const;

  QString skrTextAreaUniqueObjectName() const;
  void
  setSkrTextAreaUniqueObjectName(const QString &skrTextAreaUniqueObjectName);

  QTextDocument *textDocument() const;
  void setTextDocument(QTextDocument *textDocument);

  QString uniqueDocumentReference() const;
  void setUniqueDocumentReference(const QString &uniqueDocumentReference);

private:
  QString m_uniqueDocumentReference;
  QString m_skrTextAreaUniqueObjectName;
  QPointer<QTextDocument> m_textDocumentPtr;
};
Q_DECLARE_METATYPE(SyncDocument)



class TextBridge : public QObject {
  Q_OBJECT

public:

    static TextBridge *instance() {
      if (!m_instance)
        m_instance = new TextBridge(qApp);
      return m_instance;
    }

  Q_INVOKABLE void
  subscribeTextDocument(const QString &uniqueDocumentReference,
                        const QString &skrTextAreaUniqueObjectName,
                        QTextDocument *textDocument);
  Q_INVOKABLE void
  unsubscribeTextDocument(const QString &uniqueDocumentReference,
                          const QString &skrTextAreaUniqueObjectName,
                          QTextDocument *textDocument);

signals:

private:
    explicit TextBridge(QObject *parent = nullptr);
    static TextBridge *m_instance;
  bool isThereAnyOtherSimilarSyncDoc(
      const QString &uniqueDocumentReference,
      const QString &skrTextAreaUniqueObjectName) const;

  bool isThereAnyOtherSimilarSyncDoc(const SyncDocument &syncDoc) const;

  QList<SyncDocument>
  listOtherSimilarSyncDocs(const QString &uniqueDocumentReference,
                           const QString &skrTextAreaUniqueObjectName) const;

  QList<SyncDocument>
  listOtherSimilarSyncDocs(const SyncDocument &syncDoc) const;

  void connectContentsChangeSignal(const SyncDocument &syncDoc);
  void disconnectContentsChangeSignal(const SyncDocument &syncDoc);

private slots:

  void useTextBridge(int position, int charsRemoved, int charsAdded);

private:
  QList<SyncDocument> m_syncDocList;
};

#endif // TEXTBRIDGE_H
