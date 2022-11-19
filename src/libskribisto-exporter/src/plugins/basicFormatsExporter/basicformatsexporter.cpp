/***************************************************************************
 *   Copyright (C) 2021 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: basicformatsexporter.cpp * This file is part of Skribisto. *
 *                                                                         *
 *  Skribisto is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by   *
 *  the Free Software Foundation, either version 3 of the License, or      *
 *  (at your option) any later version.                                    *
 *                                                                         *
 *  Skribisto is distributed in the hope that it will be useful,       *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *  GNU General Public License for more details.                           *
 *                                                                         *
 *  You should have received a copy of the GNU General Public License      *
 *  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/
#include "basicformatsexporter.h"
#include "interfaces/itemexporterinterface.h"
#include "project/plmprojectmanager.h"
#include "skrdata.h"

#include <QFile>
#include <skrpluginhub.h>
#include <QTextDocument>
#include <QTextCursor>
#include <QTextBlock>
#include <QTextDocumentWriter>

BasicFormatsExporter::BasicFormatsExporter(QObject *parent) : QObject(parent) {}

// ---------------------------------------------------

BasicFormatsExporter::~BasicFormatsExporter() {}

SKRResult BasicFormatsExporter::run(QList<TreeItemAddress> treeItemAddresses, const QUrl &url,
                                    const QString &extension,
                                    const QVariantMap &parameters) const {

  QUrl fileName = url;
  SKRResult result(this);



  QList<ItemExporterInterface *> pluginList =
          skrpluginhub->pluginsByType<ItemExporterInterface>();


      QList<QTextDocumentFragment> textDocFragments;

      QFont font;
      font.setFamily(parameters.value("font_family", "Times New Roman").toString());
      int pointSize = parameters.value("font_size", 12).toInt();
      font.setPointSize(pointSize);

      QTextBlockFormat blockFormat;
      blockFormat.setTextIndent(parameters.value("text_block_indent", 0).toInt());
      blockFormat.setTopMargin(parameters.value("text_block_top_margin", 0).toInt());
      blockFormat.setLineHeight(parameters.value("text_space_between_line", 100).toInt(), QTextBlockFormat::ProportionalHeight);
      blockFormat.setHeadingLevel(0);
      blockFormat.setAlignment(Qt::AlignLeft);

      QTextCharFormat charFormat;
      charFormat.setFont(font, QTextCharFormat::FontPropertiesSpecifiedOnly);

      for(const TreeItemAddress &treeItemAddress : treeItemAddresses){

          QString pageType = skrdata->treeHub()->getType(treeItemAddress);



          for(auto *plugin : pluginList){
              if(plugin->pageType() == pageType){
                  textDocFragments.append(plugin->generateExporterTextFragment(treeItemAddress, parameters, result));
              }
          }


      }


      QTextDocument document;
      QTextCursor cursor(&document);

      for(const QTextDocumentFragment &fragment : textDocFragments){
          cursor.movePosition(QTextCursor::MoveOperation::End);
          cursor.insertBlock(blockFormat, charFormat);
          cursor.insertFragment(fragment);

      }

      cursor.movePosition(QTextCursor::MoveOperation::Start);
      if(cursor.block().text().size() == 0){
          cursor.movePosition(QTextCursor::MoveOperation::NextBlock, QTextCursor::KeepAnchor);
          cursor.removeSelectedText();
      }


      QByteArray format;
   if (extension == "odt") {
       format = "ODF";
  }
   else if(extension == "html"){
       format = "HTML";
   }
   else if(extension == "txt"){
       format = "plaintext";
   }


   QTextDocumentWriter writer(url.toLocalFile(), format);
   writer.write(&document);
  return result;
}
