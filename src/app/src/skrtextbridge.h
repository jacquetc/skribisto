#ifndef SKRTEXTBRIDGE_H
#define SKRTEXTBRIDGE_H

#include <QObject>
#include <QQuickTextDocument>
#include <QString>
#include <QPointer>

class SKRTextBridge;
struct SKRSyncDocument;


struct SKRSyncDocument
{
    Q_GADGET

public:

    explicit SKRSyncDocument();
    SKRSyncDocument(const QString     & paperType,
                    int                 projectId,
                    int                 paperId,
                    const QString     & skrTextAreaUniqueObjectName,
                    QQuickTextDocument *qQuickTextDocument);
    SKRSyncDocument(const SKRSyncDocument& syncDocument);

    bool                operator!() const;
    operator bool() const;
    SKRSyncDocument   & operator=(const SKRSyncDocument& otherSyncDocument);
    bool                operator==(const SKRSyncDocument& otherSyncDocument) const;

    QString             paperType() const;
    void                setPaperType(const QString& paperType);

    QString             skrTextAreaUniqueObjectName() const;
    void                setSkrTextAreaUniqueObjectName(
        const QString& skrTextAreaUniqueObjectName);

    int                 projectId() const;
    void                setProjectId(int projectId);

    int                 paperId() const;
    void                setPaperId(int paperId);

    QQuickTextDocument* qQuickTextDocument() const;
    void                setQQuickTextDocument(QQuickTextDocument *qQuickTextDocument);

private:

    QString                     m_paperType, m_skrTextAreaUniqueObjectName;
    int                         m_projectId, m_paperId;
    QPointer<QQuickTextDocument>m_qQuickTextDocumentPtr;
};
Q_DECLARE_METATYPE(SKRSyncDocument)


class SKRTextBridge : public QObject {
    Q_OBJECT

public:

    explicit SKRTextBridge(QObject *parent);

    Q_INVOKABLE void subscribeTextDocument(const QString     & paperType,
                                           int                 projectId,
                                           int                 paperId,
                                           const QString     & skrTextAreaUniqueObjectName,
                                           QQuickTextDocument *qQuickTextDocument);
    Q_INVOKABLE void unsubscribeTextDocument(const QString     & paperType,
                                             int                 projectId,
                                             int                 paperId,
                                             const QString     & skrTextAreaUniqueObjectName,
                                             QQuickTextDocument *qQuickTextDocument);

signals:

private:

    bool isThereAnyOtherSimilarSyncDoc(const QString& paperType,
                                       int            projectId,
                                       int            paperId,
                                       const QString& skrTextAreaUniqueObjectName) const;


    bool                  isThereAnyOtherSimilarSyncDoc(const SKRSyncDocument& syncDoc)
    const;

    QList<SKRSyncDocument>listOtherSimilarSyncDocs(const QString& paperType,
                                                   int            projectId,
                                                   int            paperId,
                                                   const QString& skrTextAreaUniqueObjectName)
    const;

    QList<SKRSyncDocument>listOtherSimilarSyncDocs(const SKRSyncDocument& syncDoc) const;

    void                  connectContentsChangeSignal(const SKRSyncDocument& syncDoc);
    void                  disconnectContentsChangeSignal(const SKRSyncDocument& syncDoc);

private slots:

    void useTextBridge(int position,
                       int charsRemoved,
                       int charsAdded);

private:

    QList<SKRSyncDocument>m_syncDocList;
};

#endif // SKRTEXTBRIDGE_H
