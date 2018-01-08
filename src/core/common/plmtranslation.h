#ifndef PLMTRANSLATION_H
#define PLMTRANSLATION_H
#include <QVariant>
#include <QString>
#include <QObject>


struct PLMTranslation
{
public:

    PLMTranslation(){}

    PLMTranslation(const QString &langFile, const QString &langCode, const QString &langName ){
        file = langFile;
        code = langCode;
        name = langName;

    }


    PLMTranslation(const PLMTranslation& other){
        file = other.file;
       code = other.code;
       name = other.name;
    }



    ~PLMTranslation() { }




    QString file, code, name;



};
Q_DECLARE_METATYPE(PLMTranslation)




















class PLMTranslations : public QObject
{
    Q_OBJECT
    PLMTranslations(QObject *parent) : QObject(parent)
    {
tr("Language Name", "write here your language name, ex: write 'English - USA' if it's en-US'");


//file and folder names :


tr("Bin");
tr("Book");
tr("Act");
tr("Chapter");
tr("parent");
tr("order");
tr("Informations");
tr("Gallery");
tr("Text");
tr("Notes");
tr("Statistics");


    }
};

#endif // PLMTRANSLATION_H
