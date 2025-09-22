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

template <typename T> class Result
{
  public:
    Result() : m_success(true), m_category(ErrorCategory::None)
    {
    }
    
    explicit Result(const QString &error, ErrorCategory category = ErrorCategory::UnknownError, const QVariant &errorData = QVariant()) 
        : m_success(false), m_error(error), m_category(category), m_errorData(errorData)
    {
    }

    bool isSuccess() const
    {
        return m_success;
    }
    
    QString error() const
    {
        return m_error;
    }
    
    ErrorCategory category() const
    {
        return m_category;
    }
    
    QVariant errorData() const
    {
        return m_errorData;
    }

  private:
    bool m_success;
    QString m_error;
    ErrorCategory m_category;
    QVariant m_errorData;
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

  Q_SIGNALS:
    void finished(bool isSuccessful);

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