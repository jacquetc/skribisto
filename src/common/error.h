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

#include "QtCore/qobject.h"
#include "skribisto_common_export.h"
#include <QList>
#include <QString>

#define QLN_ERROR_1(func_info, err_type, error_code) Error(func_info, err_type, error_code, __FILE__, __LINE__)
#define QLN_ERROR_2(func_info, err_type, error_code, msg)                                                              \
    Error(func_info, err_type, error_code, msg, __FILE__, __LINE__)
#define QLN_ERROR_3(func_info, err_type, error_code, msg, data)                                                        \
    Error(func_info, err_type, error_code, msg, data, __FILE__, __LINE__)

using namespace Qt::Literals::StringLiterals;

namespace Skribisto::Common
{
/**
 * @brief The Error class represents an error with an associated status, error code, and optional error message and
 * data.
 */
class SKRIBISTO_COMMON_EXPORT Error
{
    Q_GADGET
    Q_PROPERTY(Status status READ status)
    Q_PROPERTY(QString code READ code)
    Q_PROPERTY(QString message READ message)
    Q_PROPERTY(QString data READ data)
    Q_PROPERTY(QString className READ className)

  public:
    /**
     * @brief The Status enum defines the possible error statuses.
     */
    enum Status
    {
        Ok,       ///< The operation succeeded without any issues.
        Warning,  ///< The operation succeeded but with some non-fatal issues or warnings.
        Critical, ///< The operation failed with a critical error.
        Fatal,    ///< The operation failed with a fatal error.
        Empty     ///< The error is empty or uninitialized.
    };
    Q_ENUM(Status)

    /**
     * @brief Constructs an Error object with the given QObject, status, and error code.
     *
     * @param object The QObject associated with the error.
     * @param status The error status.
     * @param code The error code.
     */
    explicit Error(const QObject *object, const Error::Status &status, const char *code, const char *file, int line)
        : m_status(status), m_code(code), m_message(""), m_data(""), m_file(file), m_line(line)
    {
        m_className = object->metaObject()->className();
    }
    explicit Error(const QObject *object, const Error::Status &status, const char *code, const QString &file, int line)
        : m_status(status), m_code(code), m_message(""), m_data(""), m_file(file.toLatin1().constData()), m_line(line)
    {
        m_className = object->metaObject()->className();
    }

    /**
     * @brief Constructs an Error object with the given QObject, status, error code, and error message.
     *
     * @param object The QObject associated with the error.
     * @param status The error status.
     * @param code The error code.
     * @param message The error message.
     */
    explicit Error(const QObject *object, const Error::Status &status, const char *code, const char *message,
                   const char *file, int line)
        : m_status(status), m_code(code), m_message(message), m_data(""), m_file(file), m_line(line)
    {
        m_className = object->metaObject()->className();
    }
    explicit Error(const QObject *object, const Error::Status &status, const char *code, const QString &message,
                   const char *file, int line)
        : m_status(status), m_code(code), m_message(message.toLatin1().constData()), m_data(""), m_file(file),
          m_line(line)
    {
        m_className = object->metaObject()->className();
    }

    //--------------------------------------------------------------
    explicit Error(const QObject *object, const Error::Status &status, const char *code, const char *message,
                   const char *data, const char *file, int line)
        : m_status(status), m_code(code), m_message(message), m_data(data), m_file(file), m_line(line)
    {
        m_className = object->metaObject()->className();
    }
    //--------------------------------------------------------------
    explicit Error(const QObject *object, const Error::Status &status, const char *code, const QString &message,
                   const QString &data, const char *file, int line)
        : m_status(status), m_code(code), m_message(message.toLatin1().constData()),
          m_data(data.toLatin1().constData()), m_file(file), m_line(line)
    {
        m_className = object->metaObject()->className();
    }

    //--------------------------------------------------------------
    explicit Error(const char *className, const Error::Status &status, const char *code, const char *file, int line)
        : m_className(className), m_status(status), m_code(code), m_message(""), m_data(""), m_file(file), m_line(line)
    {
    }

    //--------------------------------------------------------------
    explicit Error(const char *className, const Error::Status &status, const char *code, const char *message,
                   const char *file, int line)
        : m_className(className), m_status(status), m_code(code), m_message(message), m_data(""), m_file(file),
          m_line(line)
    {
    }
    explicit Error(const char *className, const Error::Status &status, const char *code, const QString &message,
                   const char *file, int line)
        : m_className(className), m_status(status), m_code(code), m_message(message.toLatin1().constData()), m_data(""),
          m_file(file), m_line(line)
    {
    }

    //--------------------------------------------------------------
    explicit Error(const char *className, const Error::Status &status, const char *code, const char *message,
                   const char *data, const char *file, int line)
        : m_className(className), m_status(status), m_code(code), m_message(message), m_data(data), m_file(file),
          m_line(line)
    {
    }

    //--------------------------------------------------------------
    explicit Error(const char *className, const Error::Status &status, const char *code, const char *message,
                   const QString &data, const char *file, int line)
        : m_className(className), m_status(status), m_code(code), m_message(message),
          m_data(data.toLatin1().constData()), m_file(file), m_line(line)
    {
    }

    //--------------------------------------------------------------
    explicit Error(const char *className, const Error::Status &status, const char *code, const QString &message,
                   const QString &data, const char *file, int line)
        : m_className(className), m_status(status), m_code(code), m_message(message.toLatin1().constData()),
          m_data(data.toLatin1().constData()), m_file(file), m_line(line)
    {
    }

    /**
     *         @brief Constructs an empty Error object.
     *   Initializes the Error with an empty error status.
     */
    explicit Error()
    {
        m_status = Status::Empty;
    }

    //--------------------------------------------------------------
    Error(const Error &other)
        : m_status(other.m_status), m_className(other.m_className), m_code(other.m_code), m_message(other.m_message),
          m_data(other.m_data), m_file(other.m_file), m_line(other.m_line), m_trace(other.m_trace)
    {
    }

    // Copy assignment operator
    Error &operator=(const Error &other)
    {
        if (this != &other)
        {
            m_status = other.m_status;
            m_className = other.m_className;
            m_code = other.m_code;
            m_message = other.m_message;
            m_data = other.m_data;
            m_file = other.m_file;
            m_line = other.m_line;
            m_trace = other.m_trace;
        }
        return *this;
    }

    bool operator==(const Error &otherError) const
    {
        return m_status == otherError.m_status && m_className == otherError.m_className &&
               m_code == otherError.m_code && m_message == otherError.m_message;
    }

    bool operator!=(const Error &otherError) const
    {
        return m_status != otherError.m_status || m_className != otherError.m_className ||
               m_code != otherError.m_code || m_message != otherError.m_message;
    }

    /**
     * @brief Returns the error message.
     *
     * @return The error message.
     */
    Q_INVOKABLE QString message() const
    {
        return QString::fromLatin1(m_message);
    }
    /**
     * @brief Returns the error data.
     *
     * @return The error data.
     */
    Q_INVOKABLE QString data() const
    {
        return QString::fromLatin1(m_data);
    }

    QString stackTrace() const
    {
        return QString::fromLatin1(m_file) + ":"_L1 + QString::number(m_line);
    }

    /**
     * @brief Returns true if the error status is Ok, false otherwise.
     *
     * @return True if the error status is Ok, false otherwise.
     */
    bool isOk() const
    {
        return m_status == Status::Ok;
    }

    /**
     * @brief Returns true if the error status is Empty, false otherwise.
     *
     * @return True if the error status is Empty, false otherwise.
     */
    bool isEmpty() const
    {
        return m_status == Status::Empty;
    }

    /**
     * @brief Returns the error status.
     *
     * @return The error status.
     */
    Error::Status status() const
    {
        return m_status;
    }

    /**
     * @brief Sets the error status.
     *
     * @param status The new error status.
     */
    void setStatus(const Error::Status &status)
    {
        m_status = status;
    }

    //--------------------------------------------------------------

    Q_INVOKABLE QString code() const
    {
        return QString::fromLatin1(m_code);
    }

    Q_INVOKABLE QString className() const
    {
        return QString::fromLatin1(m_className);
    }

    QList<Error> trace() const
    {
        return m_trace;
    }
    void setTrace(const QList<Error> &newTrace)
    {
        m_trace = newTrace;
    }

  private:
    const char *m_code;
    const char *m_message;
    const char *m_data;
    const char *m_className;
    Error::Status m_status;
    const char *m_file;
    int m_line;
    QList<Error> m_trace;
};

}; // namespace Skribisto::Common
Q_DECLARE_METATYPE(Skribisto::Common::Error)
