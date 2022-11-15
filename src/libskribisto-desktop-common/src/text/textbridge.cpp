#include "textbridge.h"
#include "text/markdowntextdocument.h"

#include <QTextCursor>
#include <QTextDocumentFragment>
#include <QTextObject>

SyncDocument::SyncDocument()
    : m_uniqueDocumentReference(""), m_skrTextAreaUniqueObjectName("") {}

// --------------------------------------------------------------------------------------

SyncDocument::SyncDocument(const QString &uniqueDocumentReference,
                           const QString &skrTextAreaUniqueObjectName,
                           MarkdownTextDocument *textDocument) {
  m_uniqueDocumentReference = uniqueDocumentReference;
  m_skrTextAreaUniqueObjectName = skrTextAreaUniqueObjectName;
  m_textDocumentPtr = textDocument;
}

// --------------------------------------------------------------------------------------

SyncDocument::SyncDocument(const SyncDocument &syncDocument) {
  m_uniqueDocumentReference = syncDocument.uniqueDocumentReference();
  m_skrTextAreaUniqueObjectName = syncDocument.skrTextAreaUniqueObjectName();
  m_textDocumentPtr = syncDocument.textDocument();
}

// --------------------------------------------------------------------------------------

bool SyncDocument::operator!() const {
  return m_textDocumentPtr.isNull();
}

// --------------------------------------------------------------------------------------

SyncDocument &SyncDocument::operator=(const SyncDocument &otherSyncDocument) {
  if (Q_LIKELY(&otherSyncDocument != this)) {
    m_uniqueDocumentReference = otherSyncDocument.uniqueDocumentReference();
    m_skrTextAreaUniqueObjectName =
        otherSyncDocument.skrTextAreaUniqueObjectName();
    m_textDocumentPtr = otherSyncDocument.textDocument();
  }

  return *this;
}

// --------------------------------------------------------------------------------------

SyncDocument::operator bool() const {
  return !m_textDocumentPtr.isNull();
}

// --------------------------------------------------------------------------------------

bool SyncDocument::operator==(const SyncDocument &otherSyncDocument) const {
  return m_uniqueDocumentReference ==
             otherSyncDocument.uniqueDocumentReference() &&
         m_skrTextAreaUniqueObjectName ==
             otherSyncDocument.skrTextAreaUniqueObjectName() &&
         m_textDocumentPtr == otherSyncDocument.textDocument();
}

// --------------------------------------------------------------------------------------

QString SyncDocument::skrTextAreaUniqueObjectName() const {
  return m_skrTextAreaUniqueObjectName;
}

// --------------------------------------------------------------------------------------

void SyncDocument::setSkrTextAreaUniqueObjectName(
    const QString &skrTextAreaUniqueObjectName) {
  m_skrTextAreaUniqueObjectName = skrTextAreaUniqueObjectName;
}

// --------------------------------------------------------------------------------------

MarkdownTextDocument *SyncDocument::textDocument() const {
  return m_textDocumentPtr.data();
}

// --------------------------------------------------------------------------------------

void SyncDocument::setTextDocument(
    MarkdownTextDocument *textDocument) {
  m_textDocumentPtr.clear();

  m_textDocumentPtr = textDocument;
}

// --------------------------------------------------------------------------------------

QString SyncDocument::uniqueDocumentReference() const {
  return m_uniqueDocumentReference;
}

// --------------------------------------------------------------------------------------

void SyncDocument::setUniqueDocumentReference(
    const QString &uniqueDocumentReference) {
  m_uniqueDocumentReference = uniqueDocumentReference;
}

// --------------------------------------------------------------------------------------
// ------TextBridge-----------------------------------------------------------------------
// --------------------------------------------------------------------------------------

TextBridge::TextBridge(QObject *parent) : QObject(parent) {
  qRegisterMetaType<QList<SyncDocument>>("QList<SyncDocument>");
}

TextBridge *TextBridge::m_instance = nullptr;

// --------------------------------------------------------------------------------------

void TextBridge::subscribeTextDocument(
    const QString &uniqueDocumentReference,
    const QString &skrTextAreaUniqueObjectName,
    MarkdownTextDocument *textDocument) {
  SyncDocument syncDoc = SyncDocument(
      uniqueDocumentReference, skrTextAreaUniqueObjectName, textDocument);

  if (!m_syncDocList.contains(syncDoc)) {
    m_syncDocList.append(syncDoc);
  } else {
    return;
  }

  syncDoc.textDocument()->setProperty(
      "skrTextAreaUniqueObjectName", skrTextAreaUniqueObjectName);

  this->connectContentsChangeSignal(syncDoc);
}

// --------------------------------------------------------------------------------------

void TextBridge::unsubscribeTextDocument(
    const QString &uniqueDocumentReference,
    const QString &skrTextAreaUniqueObjectName,
    MarkdownTextDocument *textDocument) {
  SyncDocument syncDoc = SyncDocument(
      uniqueDocumentReference, skrTextAreaUniqueObjectName, textDocument);

  this->disconnectContentsChangeSignal(syncDoc);
  m_syncDocList.removeAll(syncDoc);
}

// --------------------------------------------------------------------------------------

bool TextBridge::isThereAnyOtherSimilarSyncDoc(
    const QString &uniqueDocumentReference,
    const QString &skrTextAreaUniqueObjectName) const {
  bool value = false;

  for (const SyncDocument &syncDoc : m_syncDocList) {
    value =
        (syncDoc.uniqueDocumentReference() == uniqueDocumentReference &&
         syncDoc.skrTextAreaUniqueObjectName() != skrTextAreaUniqueObjectName);

    if (value) {
      break;
    }
  }

  return value;
}

// --------------------------------------------------------------------------------------

bool TextBridge::isThereAnyOtherSimilarSyncDoc(
    const SyncDocument &syncDoc) const {
  return this->isThereAnyOtherSimilarSyncDoc(
      syncDoc.uniqueDocumentReference(), syncDoc.skrTextAreaUniqueObjectName());
}

// --------------------------------------------------------------------------------------

QList<SyncDocument> TextBridge::listOtherSimilarSyncDocs(
    const QString &uniqueDocumentReference,
    const QString &skrTextAreaUniqueObjectName) const {
  QList<SyncDocument> list;

  for (const SyncDocument &syncDoc : m_syncDocList) {
    if ((syncDoc.uniqueDocumentReference() == uniqueDocumentReference) &&
        (syncDoc.skrTextAreaUniqueObjectName() !=
         skrTextAreaUniqueObjectName)) {
      list.append(syncDoc);
    }
  }

  return list;
}

// --------------------------------------------------------------------------------------

QList<SyncDocument>
TextBridge::listOtherSimilarSyncDocs(const SyncDocument &syncDoc) const {
  return this->listOtherSimilarSyncDocs(syncDoc.uniqueDocumentReference(),
                                        syncDoc.skrTextAreaUniqueObjectName());
}

// --------------------------------------------------------------------------------------

void TextBridge::connectContentsChangeSignal(const SyncDocument &syncDoc) {
  connect(syncDoc.textDocument(),
          &QTextDocument::contentsChange, this, &TextBridge::useTextBridge,
          Qt::QueuedConnection);
}

// --------------------------------------------------------------------------------------

void TextBridge::disconnectContentsChangeSignal(const SyncDocument &syncDoc) {
  disconnect(syncDoc.textDocument(),
             &QTextDocument::contentsChange, this, &TextBridge::useTextBridge);
}

void TextBridge::useTextBridge(int position, int charsRemoved, int charsAdded) {
  if (!this->sender()) {
    qDebug() << this->metaObject()->className() << "no sender";
    return;
  }

  // find SyncDocument
  MarkdownTextDocument *textDocument = static_cast<MarkdownTextDocument *>(this->sender());
  QString skrTextAreaUniqueObjectName =
      textDocument->property("skrTextAreaUniqueObjectName").toString();

  SyncDocument senderSyncDoc;

  for (const SyncDocument &syncDoc : qAsConst(m_syncDocList)) {
    if (syncDoc.skrTextAreaUniqueObjectName() == skrTextAreaUniqueObjectName) {
      senderSyncDoc = syncDoc;
      break;
    }
  }

  if (!senderSyncDoc) {
    qDebug() << this->metaObject()->className() << "no synDoc found for "
             << skrTextAreaUniqueObjectName;
    return;
  }

  // qDebug() << "pos" << position << "rem" << charsRemoved << "add" <<
  // charsAdded;

  // find others similar
  if (!this->isThereAnyOtherSimilarSyncDoc(senderSyncDoc)) {
    // qDebug() << this->metaObject()->className() << "no other doc to sync
    // with";
    return;
  }

  QList<SyncDocument> otherSyncDocs =
      this->listOtherSimilarSyncDocs(senderSyncDoc);

  // get added text

  QTextCursor selectionCursor =
      textDocument->rootFrame()->firstCursorPosition();

  selectionCursor.setPosition(position, QTextCursor::MoveAnchor);
  selectionCursor.setPosition(position + charsAdded, QTextCursor::KeepAnchor);

  // qDebug() << "selectionCursor" << selectionCursor.selectedText();

  QTextDocumentFragment docFragment = selectionCursor.selection();

  for (const SyncDocument &syncDoc : qAsConst(otherSyncDocs)) {
    if (!syncDoc.textDocument()) {
      continue;
    }

    this->disconnectContentsChangeSignal(syncDoc);

    MarkdownTextDocument *otherTextDocument =
        syncDoc.textDocument();

    QTextCursor otherDocSelectionCursor =
        otherTextDocument->rootFrame()->firstCursorPosition();

    // remove
    otherDocSelectionCursor.setPosition(position, QTextCursor::MoveAnchor);
    otherDocSelectionCursor.setPosition(position + charsRemoved,
                                        QTextCursor::KeepAnchor);
    otherDocSelectionCursor.removeSelectedText();

    // add
    otherDocSelectionCursor.setPosition(position, QTextCursor::MoveAnchor);
    otherDocSelectionCursor.insertFragment(docFragment);

    // fail safe :
    otherDocSelectionCursor.setPosition(position + charsAdded,
                                        QTextCursor::MoveAnchor);

    if (otherDocSelectionCursor.block().text().size() !=
        selectionCursor.block().text().size()) {
      otherTextDocument->setSkribistoMarkdown(textDocument->toSkribistoMarkdown());
      qDebug() << "failsafe used for "
               << senderSyncDoc.uniqueDocumentReference();
      qFatal("");
    }

    this->connectContentsChangeSignal(syncDoc);
  }
}

// --------------------------------------------------------------------------------------
