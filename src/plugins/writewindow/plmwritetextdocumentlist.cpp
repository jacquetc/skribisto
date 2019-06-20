#include "plmwritetextdocumentlist.h"
#include "plmdata.h"

PLMWriteTextDocumentList::PLMWriteTextDocumentList(QObject *parent) : PLMTextDocumentList(parent)
{

}

void PLMWriteTextDocumentList::saveTextDocument(QTextDocument *textDocument)
{
    int projectId = textDocument->property("projectId").toInt();
    int paperId = textDocument->property("paperId").toInt();
    plmdata->sheetHub()->setContent(projectId, paperId, textDocument->toHtml("utf-8").toUtf8());
}
