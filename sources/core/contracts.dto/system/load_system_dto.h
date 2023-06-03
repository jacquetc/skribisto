#pragma once

#include <QObject>
#include <QUrl>

namespace Contracts::DTO::System
{
class LoadSystemDTO : public QObject
{
    Q_OBJECT

    Q_PROPERTY(QUrl fileName MEMBER m_fileName)
  public:
    LoadSystemDTO(QObject *parent = nullptr)
    {
    }

    LoadSystemDTO(const LoadSystemDTO &other) : m_fileName(other.m_fileName)
    {
    }

    LoadSystemDTO &operator=(const LoadSystemDTO &other)
    {
        if (this != &other)
        {
            m_fileName = other.m_fileName;
        }
        return *this;
    }

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
