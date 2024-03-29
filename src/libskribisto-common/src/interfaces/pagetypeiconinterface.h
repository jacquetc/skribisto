#ifndef PAGETYPEICONINTERFACE_H
#define PAGETYPEICONINTERFACE_H

#include "command.h"
#include "treeitemaddress.h"
#include <QObject>
#include <QString>

class PageTypeIconInterface {

public:
  virtual ~PageTypeIconInterface() {}

    virtual QString pageType() const       = 0;
  virtual QString pageTypeIconUrl(const TreeItemAddress &treeItemAddress) const = 0;
};

#define PageTypeIconInterface_iid "com.skribisto.PageTypeIconInterface/1.0"

Q_DECLARE_INTERFACE(PageTypeIconInterface, PageTypeIconInterface_iid)

#endif // PAGETYPEICONINTERFACE_H
