#pragma once
#include "QtCore/quuid.h"
#include "contracts_global.h"
#include "error.h"
#include <QMetaType>
#include <QString>

template <typename T> class SKR_CONTRACTS_EXPORT Result;

template <> class SKR_CONTRACTS_EXPORT Result<void>
{
  public:
    explicit Result() : m_error(Error())
    {
    }

    explicit Result(const Error &error) : m_error(error)
    {
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
        }

        return *this;
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

  private:
    Error m_error; /**< The error message contained in the Result object. */
};

/**
 * @brief A class that represents the result of an operation, which can be either a value or an error message.
 * @tparam T The type of the result value.
 */
template <typename T> class SKR_CONTRACTS_EXPORT Result
{

  public:
    /**
     * @brief Constructs a Result object containing the given value.
     * @param value The value to be contained in the Result object.
     */
    explicit Result(const T &value) : m_value(value), m_error(Error())
    {
        m_error.setStatus(Error::Ok);
    }

    /**
     * @brief Constructs a Result object containing the given error message.
     * @param error The error message to be contained in the Result object.
     */
    explicit Result(const Error &error) : m_error(error)
    {
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
            m_value = result.m_value;
            m_error = result.m_error;
        }

        return *this;
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
        return m_value != otherResult.m_value || m_error != otherResult.m_erro;
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

  private:
    T m_value;     /**< The value contained in the Result object. */
    Error m_error; /**< The error message contained in the Result object. */
};

// Register the Result class with the Qt meta object system
Q_DECLARE_METATYPE(Result<void>)
Q_DECLARE_METATYPE(Result<int>)
Q_DECLARE_METATYPE(Result<QString>)
Q_DECLARE_METATYPE(Result<QUuid>)
