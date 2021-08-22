#include "skrdownload.h"

#include <QTimer>

SKRNetworkAccessManager *SKRNetworkAccessManager::m_instance = nullptr;

SKRNetworkAccessManager::SKRNetworkAccessManager(QObject *parent) : QNetworkAccessManager(parent)
{
    Q_ASSERT_X(!m_instance, "QuickDownloadMaster", "there should be only one instance of this object");
    m_instance = this;

    this->setRedirectPolicy(QNetworkRequest::NoLessSafeRedirectPolicy);

    qDebug() << QSslSocket::supportsSsl() << QSslSocket::sslLibraryBuildVersionString() << QSslSocket::sslLibraryVersionString();

    this->connect(this, &QNetworkAccessManager::sslErrors, this, [](QNetworkReply *reply, const QList<QSslError> &errors){
        for(const QSslError &error : errors){
            qWarning() << error.errorString();
        }
    }
    );
}

SKRNetworkAccessManager::~SKRNetworkAccessManager()
{}

// ---------------------------------------------------
// ---------------------------------------------------
// ---------------------------------------------------
// ---------------------------------------------------


SKRDownload::SKRDownload(QObject *parent) : QObject(parent),
    m_running(false),
    m_componentComplete(false),
    m_progress(0.0),
    m_saveFile(nullptr),
    m_networkReply(nullptr)
{
}

SKRDownload::~SKRDownload()
{
    if (m_networkReply) {
        if (m_networkReply->isRunning()) m_networkReply->abort();
        this->stopNetworkReply();
    }

    if (m_saveFile) {
        m_saveFile->cancelWriting();
        this->stopSaveFile();
    }
}

QUrl SKRDownload::url() const
{
    return m_url;
}

void SKRDownload::setUrl(const QUrl& url)
{
    if (m_url != url) {
        m_url = url;
        emit urlChanged();
    }
}

bool SKRDownload::running()
{
    return m_running;
}

void SKRDownload::setRunning(bool running)
{
    if (m_running != running) {
        m_running = running;

        if (!m_running) {
            if (m_networkReply) {
                if (m_networkReply->isRunning()) m_networkReply->abort();
                this->stopNetworkReply();
            }

            if (m_saveFile) {
                m_saveFile->cancelWriting();
                this->stopSaveFile();
            }
        } else start();

        emit runningChanged();
    }
}

qreal SKRDownload::progress() const
{
    return m_progress;
}

QUrl SKRDownload::destination() const
{
    return m_destination;
}

void SKRDownload::setDestination(const QUrl& destination)
{
    if (m_destination != destination) {
        m_destination = destination;

        if (m_saveFile && !m_running) {
            QString newDestination = m_destination.toDisplayString(QUrl::PreferLocalFile);

            if (m_saveFile->fileName() != newDestination) m_saveFile->setFileName(newDestination);
        }
        emit destinationChanged();
    }
}

void SKRDownload::componentComplete()
{
    m_componentComplete = true;

    if (m_running) {
        this->start();
    }
}

void SKRDownload::start()
{
    if (!m_componentComplete) return;

    if (m_url.isEmpty()) {
        emit error(Error::ErrorUrl, "No URL");
        return;
    }

    if (m_destination.isEmpty()) {
        emit error(Error::ErrorDestination, "No destination");
        return;
    }

    this->setUrl(m_url);

    QString destination = m_destination.toDisplayString(QUrl::PreferLocalFile);

    if (QFile::exists(destination)) {
        emit error(Error::ErrorDestination, tr("Destination file \"%1\" already exists").arg(destination));
        return;
    }

    if (m_saveFile) m_saveFile->cancelWriting();
    this->stopSaveFile();
    m_saveFile = new QSaveFile(destination);

    if (!m_saveFile->open(QIODevice::WriteOnly)) {
        emit error(Error::ErrorDestination, m_saveFile->errorString());
        this->stopSaveFile();
        return;
    }

    this->stopNetworkReply();
    m_networkReply = skrNetworkAccessManager->get(QNetworkRequest(m_url));

    connect(m_networkReply, SIGNAL(readyRead()),                     this, SLOT(saveFile()));
    connect(m_networkReply, SIGNAL(downloadProgress(qint64,qint64)), this, SLOT(onDownloadProgress(qint64,qint64)));
    connect(m_networkReply, SIGNAL(finished()),                      this, SLOT(onFinished()));

    this->setProgress(0.0);
    m_running = true;
    emit runningChanged();
    emit started();
}

void SKRDownload::stop()
{
    this->setRunning(false);
}

void SKRDownload::saveFile()
{
    if (m_saveFile) m_saveFile->write(m_networkReply->readAll());
}

void SKRDownload::onFinished()
{
    if (!m_running) {
        if (m_saveFile) m_saveFile->cancelWriting();
    }

    if (!m_networkReply) {
        emit error(Error::ErrorNetwork, "Network reply was deleted");

        if (m_saveFile) m_saveFile->cancelWriting();
        this->stopSaveFile();
        return;
    }

    // get redirection url
    QVariant redirectionTarget = m_networkReply->attribute(QNetworkRequest::RedirectionTargetAttribute);

    if (m_networkReply->error()) {
        m_saveFile->cancelWriting();
        emit error(Error::ErrorNetwork, m_networkReply->errorString());
    } else if (!redirectionTarget.isNull()) {
        m_url = m_url.resolved(redirectionTarget.toUrl());


        start();
        return;
    } else {
        if (m_saveFile->commit()) {
            this->stopSaveFile();
            this->setProgress(1.0);
            this->setRunning(false);


            QTimer::singleShot(20, this, [this]() {
                emit finished();
            });
        } else {
            if (m_saveFile) m_saveFile->cancelWriting();
            emit error(Error::ErrorDestination, tr("Error while writing \"%1\"").arg(m_destination.toDisplayString(
                                                                                         QUrl::PreferLocalFile)));
        }
    }

    this->stopNetworkReply();
    this->stopSaveFile();
}

void SKRDownload::onDownloadProgress(qint64 bytesReceived, qint64 bytesTotal)
{
    if (!m_running) return;

    emit updateKBReceivedAndTotal((bytesReceived / 1000), (bytesTotal / 1000));
    this->setProgress(((qreal)bytesReceived / bytesTotal));
}

void SKRDownload::setProgress(qreal progress)
{
    if (m_progress != progress) {
        m_progress = progress;
        emit progressChanged();
    }
}

void SKRDownload::stopNetworkReply()
{
    if (m_networkReply) {
        disconnect(m_networkReply, SIGNAL(readyRead()), this, SLOT(saveFile()));
        disconnect(m_networkReply,
                   SIGNAL(downloadProgress(qint64,qint64)),
                   this,
                   SLOT(onDownloadProgress(qint64,qint64)));
        disconnect(m_networkReply, SIGNAL(finished()), this, SLOT(onFinished()));

        m_networkReply->deleteLater();
        m_networkReply = nullptr;
    }
}

void SKRDownload::stopSaveFile()
{
    if (m_saveFile) {
        m_saveFile->commit();
        delete m_saveFile;
        m_saveFile = nullptr;
    }
}
