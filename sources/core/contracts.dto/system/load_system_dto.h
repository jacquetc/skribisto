#pragma once

#include <QObject>
#include <QUrl>
#include <QQmlEngine>

namespace Contracts::DTO::System
{
class LoadSystemDTO
{
    Q_GADGET
    QML_ELEMENT

    Q_PROPERTY(QUrl fileName MEMBER m_fileName)
  public:
    QUrl fileName() const
    {
        return m_fileName;
    }

    void setFileName(const QUrl &fileName)
    {
        m_fileName = fileName;
    }

  private:
    QUrl m_fileName;
};
} // namespace Contracts::DTO::System
