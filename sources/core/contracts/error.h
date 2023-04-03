#pragma once

#include "QtCore/qobject.h"
#include "contracts_global.h"
#include <QList>
#include <QString>

/**
 * @brief The Error class represents an error with an associated status, error code, and optional error message and
 * data.
 */
class SKR_CONTRACTS_EXPORT Error
{
    Q_GADGET

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
    explicit Error(const QObject *object, const Error::Status &status, const QString &code)
        : m_status(status), m_code(code), m_message("")
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
    explicit Error(const QObject *object, const Error::Status &status, const QString &code, const QString &message)
        : m_status(status), m_code(code), m_message(message)
    {
        m_className = object->metaObject()->className();
    }

    //--------------------------------------------------------------
    explicit Error(const QObject *object, const Error::Status &status, const QString &code, const QString &message,
                   const QString data)
        : m_status(status), m_code(code), m_message(message), m_data(data)
    {
        m_className = object->metaObject()->className();
    }

    //--------------------------------------------------------------
    explicit Error(const QString &className, const Error::Status &status, const QString &code)
        : m_className(className), m_status(status), m_code(code)
    {
    }

    //--------------------------------------------------------------
    explicit Error(const QString &className, const Error::Status &status, const QString &code, const QString &message)
        : m_className(className), m_status(status), m_code(code), m_message(message)
    {
    }

    //--------------------------------------------------------------
    explicit Error(const QString &className, const Error::Status &status, const QString &code, const QString &message,
                   const QString data)
        : m_className(className), m_status(status), m_code(code), m_message(message), m_data(data)
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
          m_data(other.m_data)
    {
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
    QString message() const
    {
        return m_message;
    }
    /**
     * @brief Returns the error data.
     *
     * @return The error data.
     */
    QString data() const
    {
        return m_data;
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

    QString code() const
    {
        return m_code;
    }

    QString className() const;

  private:
    QString m_code;
    QString m_message;
    QString m_data;
    QString m_className;
    Error::Status m_status;
};

inline QString Error::className() const
{
    return m_className;
}
