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
#include "QtCore/quuid.h"
#include "QtCore/qvariant.h"
#include "error.h"
#include "skribisto_common_export.h"
#include <QMetaType>
#include <QString>

#define QLN_RETURN_IF_ERROR(return_result_type, result)                                                                \
    if (Q_UNLIKELY(result.hasError()))                                                                                 \
    {                                                                                                                  \
        /*Q_ASSERT(false);*/                                                                                           \
        Result<return_result_type> finalResult;                                                                        \
        finalResult.assign(__FILE__, __LINE__, result.error());                                                        \
        return finalResult;                                                                                            \
    }

#define QLN_RETURN_IF_ERROR_WITH_ACTION(return_result_type, result, action)                                            \
    if (Q_UNLIKELY(result.hasError()))                                                                                 \
    {                                                                                                                  \
        /*Q_ASSERT(false);*/                                                                                           \
        action;                                                                                                        \
        Result<return_result_type> finalResult;                                                                        \
        finalResult.assign(__FILE__, __LINE__, result.error());                                                        \
        return finalResult;                                                                                            \
    }

#define QLN_COMMA ,

namespace Skribisto::Common
{

template <typename T> class Result;

template <> class SKRIBISTO_COMMON_EXPORT Result<void>
{
  public:
    explicit Result() : m_error(Error())
    {
    }

    explicit Result(const Result<void> &result) : m_error(result.m_error)
    {
        m_trace = result.m_trace;
        m_error.setTrace(m_trace);
    }

    explicit Result(Result<void> &&result) : m_error(std::move(result.m_error))
    {
        m_trace = std::move(result.m_trace);
        m_error.setTrace(m_trace);
    }

    // convert form Result<T> to Result<void>
    template <typename T> Result(const Result<T> &result) : m_error(result.error())
    {
        m_trace = result.trace();
        m_error.setTrace(m_trace);
    }

    explicit Result(const Error &error) : m_error(error)
    {
        m_trace.append(error);
    }

    explicit Result(Error &&error) : m_error(std::move(error))
    {
        m_trace.append(error);
    }

    operator bool() const
    {
        return isOk();
    }

    bool operator!() const
    {
        return !isOk();
    }

    Q_INVOKABLE Result &operator=(const Result &result)
    {
        if (Q_LIKELY(&result != this))
        {
            m_error = result.m_error;
            m_trace = result.m_trace;
            m_error.setTrace(m_trace);
        }

        return *this;
    }

    void assign(const char *file, int line, const Error &error)
    {

        m_error = error;
        if (!m_error.isOk())
        {
            m_trace += Error("trace", error.status(), "", file, line);
            m_error.setTrace(m_trace);
        }
        // Append other trace information here
    }

    bool operator==(const Result &otherResult) const
    {
        return m_error == otherResult.m_error;
    }

    bool operator!=(const Result &otherResult) const
    {
        return m_error != otherResult.m_error;
    }

    bool isOk() const
    {
        return m_error.isOk() || m_error.isEmpty();
    }

    bool isSuccess() const
    {
        return m_error.isOk() || m_error.isEmpty();
    }

    bool isError() const
    {
        return !m_error.isOk() && !m_error.isEmpty();
    }

    bool hasError() const
    {
        return !m_error.isOk() && !m_error.isEmpty();
    }

    bool isEmpty() const
    {
        return m_error.isEmpty();
    }

    Error error() const
    {
        return m_error;
    }

    QString errorString() const
    {
        return m_error.message() + QString::fromLatin1(" (") + m_error.code() + QString::fromLatin1(") [") +
               m_error.className() + QString::fromLatin1("] [") + m_error.data() + QString::fromLatin1("]");
    }

    QList<Error> trace() const
    {
        return m_trace;
    }

  private:
    Error m_error; /**< The error message contained in the Result object. */
    QList<Error> m_trace;
};

/**
 * @brief A class that represents the result of an operation, which can be either a value or an error message.
 * @tparam T The type of the result value.
 */
template <typename T> class Result
{

  public:
    /**
     * @brief Constructs a Result object containing the given value.
     * @param value The value to be contained in the Result object.
     */
    explicit Result(const T &value) : m_value(std::move(value)), m_error(Error())
    {
        m_error.setStatus(Error::Ok);
    }

    explicit Result(T &&value) : m_value(std::move(value)), m_error(Error())
    {
        m_error.setStatus(Error::Ok);
    }

    /**
     * @brief Constructs a Result object containing the given error message.
     * @param error The error message to be contained in the Result object.
     */
    explicit Result(const Error &error) : m_error(error)
    {
        m_trace.append(error);
    }

    /**
     * @brief Constructs an empty Result object.
     */
    explicit Result() : m_error(Error())
    {
    }

    /**
     * @brief A boolean conversion operator that returns true if the Result object is ok (i.e., contains a value), false
     * otherwise.
     */
    operator bool() const
    {
        return isOk();
    }

    /**
     * @brief An operator that returns true if the Result object is not ok (i.e., contains an error message), false
     * otherwise.
     */
    bool operator!() const
    {
        return !isOk();
    }

    /**
     * @brief An assignment operator that assigns a Result object to another Result object.
     * @param result The Result object to be assigned.
     * @return The assigned Result object.
     */
    Q_INVOKABLE Result &operator=(const Result &result)
    {
        if (Q_LIKELY(&result != this))
        {
            m_value = std::move(result.m_value);
            m_trace = result.m_trace;
            m_error = result.m_error;
            m_error.setTrace(m_trace);
        }

        return *this;
    }

    void assign(const char *file, int line, const Error &error)
    {
        m_error = error;
        if (!m_error.isOk())
        {
            m_trace += Error("trace", error.status(), "", file, line);
            m_error.setTrace(m_trace);
        }
    }
    /**
     * @brief An equality operator that compares two Result objects for equality.
     * @param otherResult The other Result object to be compared.
     * @return True if the two Result objects are equal, false otherwise.
     */
    bool operator==(const Result &otherResult) const
    {
        return m_value == otherResult.m_value && m_error == otherResult.m_error;
    }

    /**
     * @brief An inequality operator that compares two Result objects for inequality.
     * @param otherResult The other Result object to be compared.
     * @return True if the two Result objects are not equal, false otherwise.
     */
    bool operator!=(const Result &otherResult) const
    {
        return m_value != otherResult.m_value || m_error != otherResult.m_error;
    }

    /**
     * @brief A method that returns true if the Result object is ok (i.e., contains a value), false otherwise.
     * @return True if the Result object is ok, false otherwise.
     */
    bool isOk() const
    {
        return m_error.isOk() || m_error.isEmpty();
    }

    /**
     * @brief A method that returns true if the Result object is a success (i.e., contains a value), false otherwise.
     *
     */
    bool isSuccess() const
    {
        return m_error.isOk() || m_error.isEmpty();
    }
    /**
     * @brief A method that returns true if the Result object contains an error message, false otherwise.
     * @return True if the Result object contains an error message, false otherwise.
     */
    bool isError() const
    {
        return !m_error.isOk() && !m_error.isEmpty();
    }

    /**
     * @brief A method that returns true if the Result object contains an error message, false otherwise.
     * @return True if the Result object contains an error message, false otherwise.
     */
    bool hasError() const
    {
        return !m_error.isOk() && !m_error.isEmpty();
    }

    /**
     * @brief A method that returns true if the Result object is empty, false otherwise.
     * @return True if the Result object is empty, false otherwise.
     */
    bool isEmpty() const
    {
        return m_error.isEmpty();
    }

    /**
     * @brief A method that returns the value contained in the Result object, or throws an error if the Result object
     * contains an error message.
     * @return The value contained in the Result object.
     */
    T value() const
    {
        if (!isOk() && !isEmpty())
        {
            qCritical("Result in error while calling value()");
            throw m_error;
        }
        return m_value;
    }

    /**
     * @brief A method that returns the error message contained in the Result object.
     * @return The error message contained in the Result object.
     */
    Error error() const
    {
        return m_error;
    }

    QString errorString() const
    {
        return m_error.message() + " (" + m_error.code() + ")" + " [" + m_error.className() + "]" + " [" +
               m_error.data() + "]";
    }

    QList<Error> trace() const
    {
        return m_trace;
    }

  private:
    T m_value;     /**< The value contained in the Result object. */
    Error m_error; /**< The errorwmessage contained in the Result object. */
    QList<Error> m_trace;
};

}; // namespace Skribisto::Common
// Register the Result class with the Qt meta object system
Q_DECLARE_METATYPE(Skribisto::Common::Result<void>)
Q_DECLARE_METATYPE(Skribisto::Common::Result<int>)
Q_DECLARE_METATYPE(Skribisto::Common::Result<QString>)
Q_DECLARE_METATYPE(Skribisto::Common::Result<QUuid>)
Q_DECLARE_METATYPE(Skribisto::Common::Result<QVariant>)
