#pragma once
#include <QHash>
#include <QObject>
#include <QRegularExpression>
#include <QString>

namespace Presenter::UndoRedo
{
using ScopeFlag = int;

// Scope struc to to hold the command scope
struct Scope

{

    int scope() const
    {
        return m_scope;
    }

    void setScope(int newScope)
    {
        m_scope = newScope;

        QList<ScopeFlag> flagList;
        for (int i = 0; i <= 30; i++)
        {
            ScopeFlag flag = 1 << i;
            if ((m_scope & flag) != 0)
            {
                flagList.append(flag);
            }
        }
        m_flags = flagList;
    }
    int m_scope;

    bool hasScopeFlag(ScopeFlag scopeFlag) const
    {
        return (m_scope & scopeFlag) != 0;
    }

    // Returns the list of flag values
    QList<ScopeFlag> flags() const
    {
        return m_flags;
    }

    QList<ScopeFlag> m_flags;
};

// Define the operator== function for Scope
inline bool operator==(const Presenter::UndoRedo::Scope &s1, const Presenter::UndoRedo::Scope &s2)
{
    return s1.scope() == s2.scope();
}

// Define the qHash function for Scope
inline uint qHash(const Presenter::UndoRedo::Scope &scope, uint seed = 0)
{
    return scope.scope();
}

// Scopes class to manage multiple scopes for undo-redo commands
class Scopes
{
  public:
    // Constructor taking a QStringList of scopes
    Scopes(const QStringList &scopeList)
    {
        // Initialize the bit flags to 0x01, which is equivalent to 1
        int n = 0x01;

        // Loop through the list of scopes
        for (const auto &scope : scopeList)
        {
            // If the scope is "all", and exit the loop
            if (scope.toLower() == "all")
            {
                qFatal("do not add All to scopes");
            }

            // Add the scope to the list and map its flag value
            m_scopeList.append(scope);
            m_scopeHash.insert(scope, n);
            m_scopeMap.insert(n, scope);
            m_flags += n;

            // Increment the bit flag to the next power of 2
            n <<= 1;
        }
    }

    // Constructor taking a comma-separated string of scopes
    Scopes(const QString &scopeList) : Scopes(scopeList.split(QRegularExpression("[\\s|,]+"), Qt::SkipEmptyParts))
    {
    }

    // Returns the number of scopes in the list
    int count() const
    {
        return m_scopeList.count();
    }

    // Returns a QStringList of the scopes
    QStringList scopeList() const
    {
        return m_scopeList;
    }

    // Returns the flag value for a given scope
    ScopeFlag flags(const QString &scope) const
    {
        return m_scopeHash.value(scope, 0);
    }

    // Returns the list of flag values for all scopes
    QList<ScopeFlag> flags() const
    {
        return m_scopeMap.keys();
    }

    // Returns the "all" flag;
    ScopeFlag flagForAll() const
    {
        return 0xFFFFFFF;
    }

    // Returns true if the given scope is in the list
    bool hasScope(const QString &scope) const
    {
        if (scope.toLower() == "all")
        {
            return true;
        }
        return m_scopeHash.contains(scope);
    }

    // Returns true if the given scope is in the list
    bool hasScope(const Scope &scope) const
    {
        if (scope.scope() == 0xFFFFFFF)
        {
            return true;
        }

        return (m_flags & scope.scope()) != 0;
    }

    Scope createScopeFromString(const QStringList &scopeStringList)
    {
        int n = 0x00;

        for (const auto &scope : scopeStringList)
        {
            int scopeFlag = 0;
            if (scope == "all")
            {
                n = 0xFFFFFFF;
                break;
            }
            else
                scopeFlag = m_scopeHash.value(scope, 0);
            n += scopeFlag;

            if (scopeFlag == 0)
            {
                QString fatal = QString("At Scopes::createScopeFromString, unknown scope : %1").arg(scope);
                qFatal("%s", fatal.toStdString().c_str());
            }
        }

        Scope scope;
        scope.setScope(n);

        return scope;
    }
    Scope createScopeFromString(const QString &scopeString)
    {
        static auto expr = QRegularExpression("[\\s|,]+");
        return createScopeFromString(scopeString.split(expr, Qt::SkipEmptyParts));
    }

  private:
    // The combined flag value for all scopes
    int m_flags = 0;

    // The list of scopes
    QStringList m_scopeList;

    // The map of scope names to flag values
    QHash<QString, int> m_scopeHash;
    QMap<int, QString> m_scopeMap;
};

} // namespace Presenter::UndoRedo
Q_DECLARE_METATYPE(Presenter::UndoRedo::Scope)
