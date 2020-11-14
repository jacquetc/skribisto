#ifndef SKRSTATHUB_H
#define SKRSTATHUB_H

#include <QObject>
#include "skr.h"

class SKRStatHub : public QObject
{
    Q_OBJECT
public:
    enum StatType {
        Character,
        Word
    };
    Q_ENUM(StatType)

    explicit SKRStatHub(QObject *parent = nullptr);

    Q_INVOKABLE int getSheetTotalCount(SKRStatHub::StatType type, int project);
    Q_INVOKABLE int getNoteTotalCount(SKRStatHub::StatType type, int project);

private slots:
    void updateWordStats(SKR::PaperType paperType, int projectId, int paperId, int wordCount = -1);
    void updateCharacterStats(SKR::PaperType paperType, int projectId, int paperId, int characterCount = -1);
    void setSheetTrashed(int projectId, int sheetId, bool isTrashed);
    void setNoteTrashed(int projectId, int noteId, bool isTrashed);
    void removeSheetFromStat(int projectId, int sheetId);
    void removeNoteFromStat(int projectId, int noteId);

signals:
    void statsChanged(SKRStatHub::StatType statType, SKR::PaperType paperType, int projectId, int count);


private:
    QHash<int, QHash<int, QHash<QString, int> > > m_sheetHashByProjectHash;
    QHash<int, QHash<int, QHash<QString, int> > > m_noteHashByProjectHash;
};

#endif // SKRSTATHUB_H
