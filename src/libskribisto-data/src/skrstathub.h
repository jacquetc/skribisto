#ifndef SKRSTATHUB_H
#define SKRSTATHUB_H

#include <QObject>
#include "skr.h"
#include "skribisto_data_global.h"

class EXPORT SKRStatHub : public QObject
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
    void updateWordStats(SKR::ItemType paperType, int projectId, int paperId, int wordCount = -1, bool triggerProjectModifiedSignal = true);
    void updateCharacterStats(SKR::ItemType paperType, int projectId, int paperId, int characterCount = -1, bool triggerProjectModifiedSignal = true);
    void setSheetTrashed(int projectId, int sheetId, bool isTrashed);
    void setNoteTrashed(int projectId, int noteId, bool isTrashed);
    void removeSheetFromStat(int projectId, int sheetId);
    void removeNoteFromStat(int projectId, int noteId);

signals:
    void statsChanged(SKRStatHub::StatType statType, SKR::ItemType paperType, int projectId, int count);


private:
    QHash<int, QHash<int, QHash<QString, int> > > m_sheetHashByProjectHash;
    QHash<int, QHash<int, QHash<QString, int> > > m_noteHashByProjectHash;
};

#endif // SKRSTATHUB_H
