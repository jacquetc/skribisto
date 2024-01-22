// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "presenter_export.h"
#include <QObject>

#include <QDateTime>

#include <QDateTime>

#include <QUuid>

namespace Skribisto::Presenter
{
class SKRIBISTO_PRESENTER_EXPORT SingleAtelier : public QObject
{
    Q_OBJECT
    Q_PROPERTY(int itemId READ id WRITE setId RESET resetId NOTIFY idChanged FINAL)

    Q_PROPERTY(QUuid uuid READ uuid NOTIFY uuidChanged FINAL)
    Q_PROPERTY(QDateTime creationDate READ creationDate NOTIFY creationDateChanged FINAL)
    Q_PROPERTY(QDateTime updateDate READ updateDate NOTIFY updateDateChanged FINAL)
    Q_PROPERTY(bool path READ path NOTIFY pathChanged FINAL)

  public:
    explicit SingleAtelier(QObject *parent = nullptr);

    int id() const;
    void setId(int newId);
    void resetId();

    QUuid uuid() const;

    QDateTime creationDate() const;

    QDateTime updateDate() const;

    bool path() const;

  signals:

    void idChanged();

    void uuidChanged();
    void creationDateChanged();
    void updateDateChanged();
    void pathChanged();

  private:
    int m_id;

    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    bool m_path;
};

} // namespace Skribisto::Presenter