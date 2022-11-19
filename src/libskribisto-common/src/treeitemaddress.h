#ifndef TREEITEMADDRESS_H
#define TREEITEMADDRESS_H

#include <QDebug>
#include <QObject>
#include "skribisto_common_global.h"


struct SKRCOMMONEXPORT TreeItemAddress
{
    Q_GADGET
public:
    explicit TreeItemAddress();
    ~TreeItemAddress();
    TreeItemAddress(int projectId, int itemId);
    TreeItemAddress(const TreeItemAddress &treeItemAddress);
    void set(int projectId, int itemId);

//    bool                                        operator!() const;
//    operator bool() const;
    Q_INVOKABLE TreeItemAddress& operator=(const TreeItemAddress& treeItemAddress);
    bool                                        operator==(const TreeItemAddress& treeItemAddress) const;
    bool                                        operator!=(const TreeItemAddress& treeItemAddress) const;


    Q_INVOKABLE bool                            isValid() const;
    Q_INVOKABLE bool hasOnlyProjectId() const;
    int itemId, projectId;
signals:

private:

};
Q_DECLARE_METATYPE(TreeItemAddress)

inline QDebug operator<< (QDebug d, const TreeItemAddress& treeItemAddress){
    d << treeItemAddress.projectId << ':' << treeItemAddress.itemId;
    return d;
}

inline QDataStream &operator<<(QDataStream &stream, const TreeItemAddress &treeItemAddress){
    stream << treeItemAddress.projectId << '/' << treeItemAddress.itemId;
    return stream;
}
inline QDataStream &operator>>(QDataStream &stream, TreeItemAddress &treeItemAddress){
    char separator;
    stream >> treeItemAddress.projectId >> separator >> treeItemAddress.itemId;
    return stream;
}
inline size_t qHash(const TreeItemAddress &key, size_t seed)
{
    return qHashMulti(seed, key.projectId, key.itemId);
}

#endif // TREEITEMADDRESS_H

