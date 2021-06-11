#ifndef SKRDOWNLOAD_H
#define SKRDOWNLOAD_H

#include <QObject>

#include <QUrl>
#include <QNetworkReply>
#include <QNetworkAccessManager>
#include <QNetworkRequest>
#include <QDir>
#include <QSaveFile>
#include <QFileInfo>
#include "qqml.h"

#define skrNetworkAccessManager SKRNetworkAccessManager::instance()


class SKRNetworkAccessManager : public QNetworkAccessManager {
public:

    explicit SKRNetworkAccessManager(QObject *parent = nullptr);
    ~SKRNetworkAccessManager();
    static SKRNetworkAccessManager* instance()
    {
        if (m_instance == nullptr) m_instance = new SKRNetworkAccessManager();
        return m_instance;
    }

private:

    static SKRNetworkAccessManager *m_instance;
    Q_DISABLE_COPY(SKRNetworkAccessManager)
};


class SKRDownload : public QObject, public QQmlParserStatus {
    Q_OBJECT
    Q_INTERFACES(QQmlParserStatus)
    Q_PROPERTY(QUrl url READ url WRITE setUrl NOTIFY urlChanged)
    Q_PROPERTY(bool running READ running WRITE setRunning NOTIFY runningChanged)
    Q_PROPERTY(qreal progress READ progress NOTIFY progressChanged)
    Q_PROPERTY(QUrl destination READ destination WRITE setDestination NOTIFY destinationChanged)

public:

    enum Error
    {
        ErrorUnknown,
        ErrorUrl,
        ErrorDestination,
        ErrorNetwork
    };
    Q_ENUM(Error)

    explicit SKRDownload(QObject *parent = nullptr);
    ~SKRDownload();


    QUrl  url() const;
    void  setUrl(const QUrl& url);

    bool  running();
    void  setRunning(bool running);

    qreal progress() const;

    QUrl  destination() const;
    void  setDestination(const QUrl& destination);

    void  classBegin() {}

    void  componentComplete();

public slots:

    void start();
    void stop();

signals:

    void urlChanged();
    void runningChanged();
    void progressChanged();
    void destinationChanged();

    void started();
    void finished();
    void canceled();

    void updateKBReceivedAndTotal(int received,
                                  int total);
    void error(int     errorCode,
               QString errorString);

private slots:

    void saveFile();
    void onFinished();
    void onDownloadProgress(qint64 bytesReceived,
                            qint64 bytesTotal);

private:

    void setProgress(qreal progress);
    QUrl m_url;
    bool m_running;
    qreal m_progress;
    QUrl m_destination;
    QNetworkReply *m_networkReply;


    QSaveFile *m_saveFile;

    bool m_componentComplete;
    Q_DISABLE_COPY(SKRDownload)
    void stopNetworkReply();
    void stopSaveFile();
};

#endif // SKRDOWNLOAD_H
