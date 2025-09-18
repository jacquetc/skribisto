#pragma once


#include <QObject>
#include <QFontDatabase>


class SKRFonts : public QObject {
    Q_OBJECT

public:

    explicit SKRFonts(QObject *parent = nullptr);
    Q_INVOKABLE QStringList fontFamilies();
    QFont                   systemFont();

signals:

private:

    QStringList families;
};


