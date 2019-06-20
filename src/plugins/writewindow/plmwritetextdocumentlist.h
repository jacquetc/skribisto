#ifndef PLMWRITETEXTDOCUMENTLIST_H
#define PLMWRITETEXTDOCUMENTLIST_H

#include <plmtextdocumentlist.h>



class PLMWriteTextDocumentList : public PLMTextDocumentList
{
public:
    PLMWriteTextDocumentList(QObject       *parent);

protected slots:
    void saveTextDocument(QTextDocument *textDocument) override;

};

#endif // PLMWRITETEXTDOCUMENTLIST_H
