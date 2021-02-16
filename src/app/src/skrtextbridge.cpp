#include "skrtextbridge.h"

#include <QTextCursor>
#include <QTextDocumentFragment>
#include <QTextObject>


SKRSyncDocument::SKRSyncDocument() :  m_uniqueDocumentReference(""), m_skrTextAreaUniqueObjectName("")
{}

// --------------------------------------------------------------------------------------

SKRSyncDocument::SKRSyncDocument(const QString &uniqueDocumentReference,
                                 const QString     & skrTextAreaUniqueObjectName,
                                 QQuickTextDocument *qQuickTextDocument)
{
    m_uniqueDocumentReference = uniqueDocumentReference;
    m_skrTextAreaUniqueObjectName = skrTextAreaUniqueObjectName;
    m_qQuickTextDocumentPtr       = qQuickTextDocument;
}

// --------------------------------------------------------------------------------------

SKRSyncDocument::SKRSyncDocument(const SKRSyncDocument& syncDocument)
{
    m_uniqueDocumentReference = syncDocument.uniqueDocumentReference();
    m_skrTextAreaUniqueObjectName = syncDocument.skrTextAreaUniqueObjectName();
    m_qQuickTextDocumentPtr       = syncDocument.qQuickTextDocument();
}

// --------------------------------------------------------------------------------------

bool SKRSyncDocument::operator!() const
{
    return m_qQuickTextDocumentPtr.isNull();
}

// --------------------------------------------------------------------------------------

SKRSyncDocument& SKRSyncDocument::operator=(const SKRSyncDocument& otherSyncDocument)
{
    if (Q_LIKELY(&otherSyncDocument != this)) {
        m_uniqueDocumentReference = otherSyncDocument.uniqueDocumentReference();
        m_skrTextAreaUniqueObjectName = otherSyncDocument.skrTextAreaUniqueObjectName();
        m_qQuickTextDocumentPtr       = otherSyncDocument.qQuickTextDocument();
    }

    return *this;
}

// --------------------------------------------------------------------------------------

SKRSyncDocument::operator bool() const
{
    return !m_qQuickTextDocumentPtr.isNull();
}

// --------------------------------------------------------------------------------------

bool SKRSyncDocument::operator==(const SKRSyncDocument& otherSyncDocument) const {
    return m_uniqueDocumentReference == otherSyncDocument.uniqueDocumentReference()
           && m_skrTextAreaUniqueObjectName == otherSyncDocument.skrTextAreaUniqueObjectName()
           && m_qQuickTextDocumentPtr == otherSyncDocument.qQuickTextDocument()
    ;
}

// --------------------------------------------------------------------------------------

QString SKRSyncDocument::skrTextAreaUniqueObjectName() const
{
    return m_skrTextAreaUniqueObjectName;
}

// --------------------------------------------------------------------------------------

void SKRSyncDocument::setSkrTextAreaUniqueObjectName(
    const QString& skrTextAreaUniqueObjectName)
{
    m_skrTextAreaUniqueObjectName = skrTextAreaUniqueObjectName;
}

// --------------------------------------------------------------------------------------

QQuickTextDocument * SKRSyncDocument::qQuickTextDocument() const
{
    return m_qQuickTextDocumentPtr.data();
}

// --------------------------------------------------------------------------------------

void SKRSyncDocument::setQQuickTextDocument(QQuickTextDocument *qQuickTextDocument)
{
    m_qQuickTextDocumentPtr.clear();

    m_qQuickTextDocumentPtr = qQuickTextDocument;
}

// --------------------------------------------------------------------------------------

QString SKRSyncDocument::uniqueDocumentReference() const
{
    return m_uniqueDocumentReference;
}

// --------------------------------------------------------------------------------------

void SKRSyncDocument::setUniqueDocumentReference(const QString &uniqueDocumentReference)
{
    m_uniqueDocumentReference = uniqueDocumentReference;
}

// --------------------------------------------------------------------------------------
// ------SKRTextBridge-----------------------------------------------------------------------
// --------------------------------------------------------------------------------------

SKRTextBridge::SKRTextBridge(QObject *parent) : QObject(parent)
{
    qRegisterMetaType<QList<SKRSyncDocument> >("QList<SKRSyncDocument>");
}

// --------------------------------------------------------------------------------------

void SKRTextBridge::subscribeTextDocument(const QString &uniqueDocumentReference,
                                          const QString     & skrTextAreaUniqueObjectName,
                                          QQuickTextDocument *qQuickTextDocument)
{
    SKRSyncDocument syncDoc = SKRSyncDocument(uniqueDocumentReference,
                                              skrTextAreaUniqueObjectName,
                                              qQuickTextDocument);

    if (!m_syncDocList.contains(syncDoc)) {
        m_syncDocList.append(syncDoc);
    }
    else {
        return;
    }


    syncDoc.qQuickTextDocument()->textDocument()->setProperty(
        "skrTextAreaUniqueObjectName",
        skrTextAreaUniqueObjectName);

    this->connectContentsChangeSignal(syncDoc);
}

// --------------------------------------------------------------------------------------

void SKRTextBridge::unsubscribeTextDocument(const QString &uniqueDocumentReference,
                                            const QString     & skrTextAreaUniqueObjectName,
                                            QQuickTextDocument *qQuickTextDocument)
{
    SKRSyncDocument syncDoc = SKRSyncDocument(uniqueDocumentReference,
                                              skrTextAreaUniqueObjectName,
                                              qQuickTextDocument);

    this->disconnectContentsChangeSignal(syncDoc);
    m_syncDocList.removeAll(syncDoc);
}

// --------------------------------------------------------------------------------------

bool SKRTextBridge::isThereAnyOtherSimilarSyncDoc(const QString& uniqueDocumentReference,
                                                  const QString& skrTextAreaUniqueObjectName)
const
{
    bool value = false;

    for (const SKRSyncDocument& syncDoc : m_syncDocList) {
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

bool SKRTextBridge::isThereAnyOtherSimilarSyncDoc(const SKRSyncDocument& syncDoc) const
{
    return this->isThereAnyOtherSimilarSyncDoc(syncDoc.uniqueDocumentReference(),
                                               syncDoc.skrTextAreaUniqueObjectName());
}

// --------------------------------------------------------------------------------------

QList<SKRSyncDocument>SKRTextBridge::listOtherSimilarSyncDocs(const QString &uniqueDocumentReference,
                                                              const QString& skrTextAreaUniqueObjectName)
const
{
    QList<SKRSyncDocument> list;

    for (const SKRSyncDocument& syncDoc : m_syncDocList) {
        if ((syncDoc.uniqueDocumentReference() == uniqueDocumentReference) &&
            (syncDoc.skrTextAreaUniqueObjectName() != skrTextAreaUniqueObjectName)) {
            list.append(syncDoc);
        }
    }

    return list;
}

// --------------------------------------------------------------------------------------

QList<SKRSyncDocument>SKRTextBridge::listOtherSimilarSyncDocs(
    const SKRSyncDocument& syncDoc) const
{
    return this->listOtherSimilarSyncDocs(syncDoc.uniqueDocumentReference(),
                                          syncDoc.skrTextAreaUniqueObjectName());
}

// --------------------------------------------------------------------------------------


void SKRTextBridge::connectContentsChangeSignal(const SKRSyncDocument& syncDoc)
{
    connect(syncDoc.qQuickTextDocument()->textDocument(),
            &QTextDocument::contentsChange,
            this,
            &SKRTextBridge::useTextBridge,
            Qt::UniqueConnection);
}

// --------------------------------------------------------------------------------------


void SKRTextBridge::disconnectContentsChangeSignal(const SKRSyncDocument& syncDoc)
{
    disconnect(syncDoc.qQuickTextDocument()->textDocument(),
               &QTextDocument::contentsChange,
               this,
               &SKRTextBridge::useTextBridge);
}

void SKRTextBridge::useTextBridge(int position, int charsRemoved, int charsAdded)
{
    if (!this->sender()) {
        qDebug() << this->metaObject()->className() << "no sender";
        return;
    }

    // find SKRSyncDocument
    QTextDocument *textDocument         = static_cast<QTextDocument *>(this->sender());
    QString skrTextAreaUniqueObjectName = textDocument->property(
        "skrTextAreaUniqueObjectName").toString();

    SKRSyncDocument senderSyncDoc;

    for (const SKRSyncDocument& syncDoc : qAsConst(m_syncDocList)) {
        if (syncDoc.skrTextAreaUniqueObjectName() == skrTextAreaUniqueObjectName) {
            senderSyncDoc = syncDoc;
            break;
        }
    }

    if (!senderSyncDoc) {
        qDebug() << this->metaObject()->className() << "no synDoc found for " <<
            skrTextAreaUniqueObjectName;
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


    QList<SKRSyncDocument> otherSyncDocs = this->listOtherSimilarSyncDocs(senderSyncDoc);

    // get added text

    QTextCursor selectionCursor = textDocument->rootFrame()->firstCursorPosition();

    selectionCursor.setPosition(position,              QTextCursor::MoveAnchor);
    selectionCursor.setPosition(position + charsAdded, QTextCursor::KeepAnchor);

    //qDebug() << "selectionCursor" << selectionCursor.selectedText();

    QTextDocumentFragment docFragment = selectionCursor.selection();

    for (const SKRSyncDocument& syncDoc : otherSyncDocs) {
        if (!syncDoc.qQuickTextDocument()) {
            continue;
        }

        this->disconnectContentsChangeSignal(syncDoc);

        QTextDocument *otherTextDocument = syncDoc.qQuickTextDocument()->textDocument();

        QTextCursor otherDocSelectionCursor =
            otherTextDocument->rootFrame()->firstCursorPosition();

        // remove
        otherDocSelectionCursor.setPosition(position,                QTextCursor::MoveAnchor);
        otherDocSelectionCursor.setPosition(position + charsRemoved, QTextCursor::KeepAnchor);
        otherDocSelectionCursor.removeSelectedText();

        // add
        otherDocSelectionCursor.setPosition(position, QTextCursor::MoveAnchor);
        otherDocSelectionCursor.insertFragment(docFragment);


        //fail safe :
        otherDocSelectionCursor.setPosition(position + charsAdded, QTextCursor::MoveAnchor);
        if(otherDocSelectionCursor.block().text().size() != selectionCursor.block().text().size()){
            otherTextDocument->setMarkdown(textDocument->toMarkdown());
            qDebug() << "failsafe used for " << senderSyncDoc.uniqueDocumentReference();
        }

        this->connectContentsChangeSignal(syncDoc);
    }
}

// --------------------------------------------------------------------------------------
