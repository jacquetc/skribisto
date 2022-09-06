#ifndef SHORTCUTBACKEND_H
#define SHORTCUTBACKEND_H

#include <QObject>
#include <QQmlEngine>

class ShortcutBackend : public QObject
{
    Q_OBJECT
    QML_ELEMENT
    Q_PROPERTY(QString name READ name WRITE setName NOTIFY nameChanged)
    Q_PROPERTY(QString group READ group WRITE setGroup NOTIFY groupChanged)
    Q_PROPERTY(QString sequence READ sequence WRITE setSequence NOTIFY sequenceChanged)
    Q_PROPERTY(bool enabled READ enabled WRITE setEnabled NOTIFY enabledChanged)
    Q_PROPERTY(bool visible READ visible WRITE setVisible NOTIFY visibleChanged)
    Q_PROPERTY(int priority READ priority WRITE setPriority NOTIFY priorityChanged)
    Q_PROPERTY(QString finalSequence READ finalSequence NOTIFY finalSequenceChanged)

public:
    explicit ShortcutBackend(QObject *parent = nullptr);
    ~ShortcutBackend();

    const QString &name() const;
    void setName(const QString &newName);

    const QString &group() const;


    const QString &sequence() const;
    void setSequence(const QString &newSequence);

    void setGroup(const QString &newGroup);

    bool enabled() const;
    void setEnabled(bool newEnabled);

    bool visible() const;
    void setVisible(bool newVisible);

    int priority() const;
    void setPriority(int newPriority);


    const QString &finalSequence() const;

    Q_INVOKABLE void processAmbiguousActivation();

signals:
    void activated();

    void nameChanged();

    void sequenceChanged();

    void groupChanged();

    void enabledChanged();

    void visibleChanged();

    void priorityChanged();

    void finalSequenceChanged();

private:
    void setFinalSequence(const QString &newFinalSequence);
    QString m_name;
    QString m_group;
    QString m_sequence;
    QString m_finalSequence;
    bool m_enabled;
    bool m_visible;
    int m_priority;

};

#endif // SHORTCUTBACKEND_H
