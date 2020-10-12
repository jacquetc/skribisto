#include "skrtextbridge.h"

#include <QTextCursor>
#include <QTextDocumentFragment>
#include <QTextObject>


SKRSyncDocument::SKRSyncDocument() : m_paperType(""), m_skrTextAreaUniqueObjectName(""),
    m_projectId(-1), m_paperId(-1)
{}

// --------------------------------------------------------------------------------------

SKRSyncDocument::SKRSyncDocument(const QString     & paperType,
                                 int                 projectId,
                                 int                 paperId,
                                 const QString     & skrTextAreaUniqueObjectName,
                                 QQuickTextDocument *qQuickTextDocument)
{
    m_paperId                     = paperId;
    m_projectId                   = projectId;
    m_paperType                   = paperType;
    m_skrTextAreaUniqueObjectName = skrTextAreaUniqueObjectName;
    m_qQuickTextDocumentPtr       = qQuickTextDocument;
}

// --------------------------------------------------------------------------------------

SKRSyncDocument::SKRSyncDocument(const SKRSyncDocument& syncDocument)
{
    m_paperId                     = syncDocument.paperId();
    m_projectId                   = syncDocument.projectId();
    m_paperType                   = syncDocument.paperType();
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
        m_paperId                     = otherSyncDocument.paperId();
        m_projectId                   = otherSyncDocument.projectId();
        m_paperType                   = otherSyncDocument.paperType();
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
    return m_paperType == otherSyncDocument.paperId()
           && m_projectId == otherSyncDocument.projectId()
           && m_paperId == otherSyncDocument.paperId()
           && m_skrTextAreaUniqueObjectName ==
           otherSyncDocument.skrTextAreaUniqueObjectName()
           && m_qQuickTextDocumentPtr == otherSyncDocument.qQuickTextDocument()
    ;
}

// --------------------------------------------------------------------------------------

QString SKRSyncDocument::paperType() const
{
    return m_paperType;
}

// --------------------------------------------------------------------------------------

void SKRSyncDocument::setPaperType(const QString& paperType)
{
    m_paperType = paperType;
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

int SKRSyncDocument::projectId() const
{
    return m_projectId;
}

// --------------------------------------------------------------------------------------

void SKRSyncDocument::setProjectId(int projectId)
{
    m_projectId = projectId;
}

// --------------------------------------------------------------------------------------


int SKRSyncDocument::paperId() const
{
    return m_paperId;
}

// --------------------------------------------------------------------------------------

void SKRSyncDocument::setPaperId(int paperId)
{
    m_paperId = paperId;
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
// ------SKRTextBridge-----------------------------------------------------------------------
// --------------------------------------------------------------------------------------

SKRTextBridge::SKRTextBridge(QObject *parent) : QObject(parent)
{
    qRegisterMetaType<QList<SKRSyncDocument> >("QList<SKRSyncDocument>");
}

// --------------------------------------------------------------------------------------

void SKRTextBridge::subscribeTextDocument(const QString     & paperType,
                                          int                 projectId,
                                          int                 paperId,
                                          const QString     & skrTextAreaUniqueObjectName,
                                          QQuickTextDocument *qQuickTextDocument)
{
    SKRSyncDocument syncDoc = SKRSyncDocument(paperType,
                                              projectId,
                                              paperId,
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

void SKRTextBridge::unsubscribeTextDocument(const QString     & paperType,
                                            int                 projectId,
                                            int                 paperId,
                                            const QString     & skrTextAreaUniqueObjectName,
                                            QQuickTextDocument *qQuickTextDocument)
{
    SKRSyncDocument syncDoc = SKRSyncDocument(paperType,
                                              projectId,
                                              paperId,
                                              skrTextAreaUniqueObjectName,
                                              qQuickTextDocument);

    this->disconnectContentsChangeSignal(syncDoc);
    m_syncDocList.removeAll(syncDoc);
}

// --------------------------------------------------------------------------------------

bool SKRTextBridge::isThereAnyOtherSimilarSyncDoc(const QString& paperType,
                                                  int            projectId,
                                                  int            paperId,
                                                  const QString& skrTextAreaUniqueObjectName)
const
{
    bool result = false;

    for (const SKRSyncDocument& syncDoc : m_syncDocList) {
        result =
            (syncDoc.paperType() == paperType && syncDoc.projectId() == projectId &&
             syncDoc.paperId() == paperId &&
             syncDoc.skrTextAreaUniqueObjectName() != skrTextAreaUniqueObjectName);

        if (result) {
            break;
        }
    }

    return result;
}

// --------------------------------------------------------------------------------------

bool SKRTextBridge::isThereAnyOtherSimilarSyncDoc(const SKRSyncDocument& syncDoc) const
{
    return this->isThereAnyOtherSimilarSyncDoc(syncDoc.paperType(),
                                               syncDoc.projectId(),
                                               syncDoc.paperId(),
                                               syncDoc.skrTextAreaUniqueObjectName());
}

// --------------------------------------------------------------------------------------

QList<SKRSyncDocument>SKRTextBridge::listOtherSimilarSyncDocs(const QString& paperType,
                                                              int            projectId,
                                                              int            paperId,
                                                              const QString& skrTextAreaUniqueObjectName)
const
{
    QList<SKRSyncDocument> result;

    for (const SKRSyncDocument& syncDoc : m_syncDocList) {
        if ((syncDoc.paperType() == paperType) && (syncDoc.projectId() == projectId) &&
            (syncDoc.paperId() == paperId) &&
            (syncDoc.skrTextAreaUniqueObjectName() != skrTextAreaUniqueObjectName)) {
            result.append(syncDoc);
        }
    }

    return result;
}

// --------------------------------------------------------------------------------------

QList<SKRSyncDocument>SKRTextBridge::listOtherSimilarSyncDocs(
    const SKRSyncDocument& syncDoc) const
{
    return this->listOtherSimilarSyncDocs(syncDoc.paperType(),
                                          syncDoc.projectId(),
                                          syncDoc.paperId(),
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

    for (const SKRSyncDocument& syncDoc : m_syncDocList) {
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
        qDebug() << this->metaObject()->className() << "no other doc to sync with";
        return;
    }


    QList<SKRSyncDocument> otherSyncDocs = this->listOtherSimilarSyncDocs(senderSyncDoc);

    // get added text

    QTextCursor selectionCursor = textDocument->rootFrame()->firstCursorPosition();

    selectionCursor.setPosition(position,              QTextCursor::MoveAnchor);
    selectionCursor.setPosition(position + charsAdded, QTextCursor::KeepAnchor);

    qDebug() << "selectionCursor" << selectionCursor.selectedText();

    QTextDocumentFragment docFragment = selectionCursor.selection();

    for (const SKRSyncDocument& syncDoc : otherSyncDocs) {
        this->disconnectContentsChangeSignal(syncDoc);

        QTextDocument *otherTextDocument = syncDoc.qQuickTextDocument()->textDocument();
        QTextCursor    selectionCursor   =
            otherTextDocument->rootFrame()->firstCursorPosition();

        // remove
        selectionCursor.setPosition(position,                QTextCursor::MoveAnchor);
        selectionCursor.setPosition(position + charsRemoved, QTextCursor::KeepAnchor);
        selectionCursor.removeSelectedText();

        // add
        selectionCursor.setPosition(position, QTextCursor::MoveAnchor);
        selectionCursor.insertFragment(docFragment);

        this->connectContentsChangeSignal(syncDoc);
    }
}

// --------------------------------------------------------------------------------------
