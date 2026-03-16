// Adapted for ExportWork use case

#include "export_work_uc.h"
#include "direct_access/mapper_tools.h"

#include <QFile>
#include <QPdfWriter>
#include <QTextCursor>
#include <QTextDocument>
#include <QTextDocumentWriter>
#include <QTextStream>

using namespace Qt::StringLiterals;

namespace Skribisto::ExportManagement
{

ExportWorkUseCase::ExportWorkUseCase(std::unique_ptr<IExportWorkUnitOfWork> uow, const ExportWorkDto &exportWorkDto)
    : m_uow(std::move(uow)), m_exportWorkDto(exportWorkDto)
{
}

QJsonObject ExportWorkUseCase::execute(std::function<void(Common::LongOperation::OperationProgress)> progressCallback,
                                       const std::atomic<bool> &cancelFlag)
{
    ExportResultDto resultDto;

    try
    {
        if (!m_uow->beginTransaction())
            throw std::runtime_error("Failed to begin transaction");

        // Get the Work
        auto works = m_uow->getWork({m_exportWorkDto.workId});
        if (works.isEmpty())
            throw std::runtime_error("Work not found");
        const auto &work = works.first();

        // Collect items to export
        QList<int> itemIdsToExport;

        if (!m_exportWorkDto.binderItemIds.isEmpty())
        {
            itemIdsToExport = m_exportWorkDto.binderItemIds;
        }
        else
        {
            auto binderIds = m_uow->getWorkRelationship(work.id, SCDWork::WorkRelationshipField::Binders);
            for (int binderId : binderIds)
            {
                auto binders = m_uow->getBinder({binderId});
                if (binders.isEmpty() || !binders.first().activated)
                    continue;

                auto binderItemIds =
                    m_uow->getBinderRelationship(binderId, SCDBinder::BinderRelationshipField::BinderItems);
                itemIdsToExport.append(binderItemIds);
            }
        }

        // Filter to only activated and printable items
        auto allItems = m_uow->getBinderItem(itemIdsToExport);
        QList<SCE::BinderItem> exportableItems;
        for (const auto &item : allItems)
        {
            if (item.activated && item.isPrintable)
                exportableItems.append(item);
        }

        const int totalItems = exportableItems.size();
        progressCallback({0, totalItems, u"Starting export..."_s});

        // Build a single QTextDocument from all items' primary content.
        // Content is stored as Skribisto Markdown (converted from HTML during legacy upgrade).
        QTextDocument document;
        QTextCursor cursor(&document);
        int exportedCount = 0;

        for (int i = 0; i < exportableItems.size(); ++i)
        {
            if (cancelFlag.load())
            {
                m_uow->rollback();
                throw std::runtime_error("Export cancelled");
            }

            const auto &item = exportableItems[i];

            // Get primary content
            auto contentIds =
                m_uow->getBinderItemRelationship(item.id, SCDBinderItem::BinderItemRelationshipField::Contents);
            auto contents = m_uow->getContent(contentIds);

            QString primaryContent;
            for (const auto &c : contents)
            {
                if (c.role == u"primary"_s && c.activated)
                {
                    primaryContent = c.data;
                    break;
                }
            }

            if (primaryContent.isEmpty())
                continue;

            // Append markdown content to the document
            if (exportedCount > 0)
                cursor.insertBlock();

            cursor.insertMarkdown(primaryContent);
            exportedCount++;
            progressCallback({i + 1, totalItems, u"Exporting: "_s + item.title});
        }

        // Write to file based on format
        const QString &format = m_exportWorkDto.format;
        const QString &outputPath = m_exportWorkDto.outputPath;

        if (format == u"txt"_s)
        {
            QFile file(outputPath);
            if (!file.open(QIODevice::WriteOnly | QIODevice::Text))
                throw std::runtime_error("Cannot open output file: " + outputPath.toStdString());
            file.write(document.toPlainText().toUtf8());
            file.close();
        }
        else if (format == u"md"_s)
        {
            QFile file(outputPath);
            if (!file.open(QIODevice::WriteOnly | QIODevice::Text))
                throw std::runtime_error("Cannot open output file: " + outputPath.toStdString());
            file.write(document.toMarkdown().toUtf8());
            file.close();
        }
        else if (format == u"html"_s)
        {
            QFile file(outputPath);
            if (!file.open(QIODevice::WriteOnly | QIODevice::Text))
                throw std::runtime_error("Cannot open output file: " + outputPath.toStdString());
            file.write(document.toHtml().toUtf8());
            file.close();
        }
        else if (format == u"odt"_s)
        {
            QTextDocumentWriter writer(outputPath, "ODF");
            if (!writer.write(&document))
                throw std::runtime_error("Failed to write ODT file: " + outputPath.toStdString());
        }
        else if (format == u"pdf"_s)
        {
            QPdfWriter pdfWriter(outputPath);
            pdfWriter.setPageSize(QPageSize::A4);
            pdfWriter.setTitle(work.title);
            pdfWriter.setCreator(u"Skribisto"_s);
            document.print(&pdfWriter);
        }
        else
        {
            throw std::runtime_error("Unsupported export format: " + format.toStdString());
        }

        resultDto.exportedCount = exportedCount;
        resultDto.outputPath = outputPath;

        if (!m_uow->commit())
            throw std::runtime_error("Failed to commit transaction");
    }
    catch (...)
    {
        m_uow->rollback();
        throw;
    }

    m_uow->publishExportWorkSignal();
    progressCallback({resultDto.exportedCount, resultDto.exportedCount, u"Export complete"_s});
    return Common::DirectAccess::gadgetToJson(resultDto);
}

} // namespace Skribisto::ExportManagement
