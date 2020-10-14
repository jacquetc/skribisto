#ifndef SKRFONTS_H
#define SKRFONTS_H

#include <QObject>
#include <QFontDatabase>


class SKRFonts : public QObject {
    Q_OBJECT

public:

    explicit SKRFonts(QObject *parent = nullptr);
    Q_INVOKABLE QStringList fontFamilies();   
    QFont systemFont();

signals:

private:

    QStringList families;
};

#endif // SKRFONTS_H
