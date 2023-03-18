#pragma once
#include <QSettings>

namespace Infrastructure::Skrib
{

class SkribFileConfiguration
{
  public:
    void setFilePath(const QString &filePath)
    {
        m_settings.setValue("file/path", filePath);
    }

    QString getFilePath() const
    {
        return m_settings.value("file/path").toString();
    }

  private:
    QSettings m_settings;
};
} // namespace Infrastructure::Skrib
