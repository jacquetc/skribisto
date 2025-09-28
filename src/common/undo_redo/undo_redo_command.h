/******************************************************************************
 Copyright (C) 2025 by Cyril Jacquet                                          *
 cyril.jacquet@skribisto.eu                                                   *
                                                                              *
 This file is part of Skribisto.                                              *
                                                                              *
 Skribisto is free software: you can redistribute it and/or modify            *
 it under the terms of the GNU General Public License as published by         *
 the Free Software Foundation, either version 3 of the License, or            *
 (at your option) any later version.                                          *
                                                                              *
 Skribisto is distributed in the hope that it will be useful,                 *
 but WITHOUT ANY WARRANTY; without even the implied warranty of               *
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                *
 GNU General Public License for more details.                                 *
                                                                              *
 You should have received a copy of the GNU General Public License            *
 along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.           *
 ******************************************************************************/

#pragma once

#include <QFutureWatcher>
#include <QObject>
#include <QPromise>
#include <QString>
#include <QVariant>
#include <QDateTime>
#include <QElapsedTimer>
#include <functional>
#include <memory>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::UndoRedo
{

enum class ErrorCategory
{
    None,
    ValidationError,
    ExecutionError,
    DatabaseError,
    NetworkError,
    TimeoutError,
    PermissionError,
    SystemError,
    UserCancelledError,
    UnknownError
};

enum class ErrorSeverity
{
    Info,
    Warning,
    Error,
    Critical,
    Fatal
};

template <typename T> class Result
{
  public:
    Result() : m_success(true), m_category(ErrorCategory::None), m_severity(ErrorSeverity::Info),
               m_errorCode(0), m_timestamp(QDateTime::currentDateTimeUtc())
    {
    }

    explicit Result(const QString &error, ErrorCategory category = ErrorCategory::UnknownError,
                    const QVariant &errorData = QVariant(), ErrorSeverity severity = ErrorSeverity::Error,
                    int errorCode = 0, const QString &context = QString(), const QString &sourceLocation = QString())
        : m_success(false), m_error(error), m_category(category), m_errorData(errorData), 
          m_severity(severity), m_errorCode(errorCode), m_context(context), 
          m_sourceLocation(sourceLocation), m_timestamp(QDateTime::currentDateTimeUtc())
    {
    }

    bool isSuccess() const { return m_success; }
    QString error() const { return m_error; }
    ErrorCategory category() const { return m_category; }
    QVariant errorData() const { return m_errorData; }
    ErrorSeverity severity() const { return m_severity; }
    int errorCode() const { return m_errorCode; }
    QString context() const { return m_context; }
    QString sourceLocation() const { return m_sourceLocation; }
    QDateTime timestamp() const { return m_timestamp; }
    qint64 executionTimeMs() const { return m_executionTimeMs; }

    void setExecutionTime(qint64 milliseconds) { m_executionTimeMs = milliseconds; }
    void setSeverity(ErrorSeverity severity) { m_severity = severity; }
    void setContext(const QString &context) { m_context = context; }
    void setSourceLocation(const QString &sourceLocation) { m_sourceLocation = sourceLocation; }

    // Convenience method to get a detailed error description
    QString detailedError() const
    {
        if (m_success) return "Success"_L1;
        
        QString details = m_error;
        if (!m_context.isEmpty()) details += QString(" [Context: %1]").arg(m_context);
        if (m_errorCode != 0) details += QString(" [Code: %1]").arg(m_errorCode);
        if (!m_sourceLocation.isEmpty()) details += QString(" [Location: %1]").arg(m_sourceLocation);
        if (m_executionTimeMs > 0) details += QString(" [Duration: %1ms]").arg(m_executionTimeMs);
        return details;
    }

  private:
    bool m_success;
    QString m_error;
    ErrorCategory m_category;
    QVariant m_errorData;
    ErrorSeverity m_severity;
    int m_errorCode;
    QString m_context;
    QString m_sourceLocation;
    QDateTime m_timestamp;
    qint64 m_executionTimeMs = 0;
};

class UndoRedoCommand : public QObject
{
    Q_OBJECT

  public:
    explicit UndoRedoCommand(const QString &text, QObject *parent = nullptr);

    void setExecuteFunction(const std::function<void(QPromise<Result<void>> &promise)> &function);
    void setUndoFunction(const std::function<Result<void>()> &function);
    void setRedoFunction(const std::function<Result<void>()> &function);

    virtual void asyncExecute();
    virtual void asyncUndo();
    virtual void asyncRedo();

    // Command merging support
    virtual bool canMergeWith(const std::shared_ptr<UndoRedoCommand> &other) const;
    virtual void mergeWith(const std::shared_ptr<UndoRedoCommand> &other);

    QString text() const;
    void setText(const QString &newText);
    // Cancel any running async operations
    void cancel();
  Q_SIGNALS:
    void finished(bool isSuccessful);
    void finishedWithResult(const Result<void> &result);

  private Q_SLOTS:
    void onExecuteFinished();
    void onUndoFinished();
    void onRedoFinished();

  private:
    QString m_text;
    bool m_wasAlreadyExecuted = false;
    std::function<void(QPromise<Result<void>> &promise)> m_executeFunction;
    std::function<Result<void>()> m_undoFunction;
    std::function<Result<void>()> m_redoFunction;
    QFutureWatcher<Result<void>> *m_executeWatcher;
    QFutureWatcher<Result<void>> *m_redoWatcher;
    QFutureWatcher<Result<void>> *m_undoWatcher;
};

} // namespace Skribisto::Common::UndoRedo