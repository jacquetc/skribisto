# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# リソース：バインダーの項目に添付したファイルとウェブリンク。

## パネル

resources-title = リソース
shortcut-name-resources-show = リソースを表示
resources-add-files = ファイルを追加
resources-add-files-title = この項目にファイルを追加
resources-add-link = ウェブリンクを追加
resources-drop-hint = ファイルやウェブアドレスをここにドロップすると添付できます
resources-drop-hint-unattached = ファイルやウェブアドレスをここにドロップすると、どの項目にも添付せずに追加できます
resources-no-item = 項目を開くと、そのリソースが表示されます。
resources-empty = この項目にはまだ何も添付されていません。上のボタンでファイルやウェブリンクを追加するか、ここにドロップしてください。
resources-dock-view-label = 表示するリソース
resources-dock-view-item = この項目
resources-dock-view-unattached = 未添付
resources-dock-view-item-hint = いま開いている項目のファイルとリンクです。追加したものはこの項目に添付されます。
resources-dock-view-unattached-hint = どの項目にも添付されていないファイルとリンクです。まだ整理していない資料などです。ここで追加したものはどこにも添付されません。
resources-unattached-empty = このプロジェクトのリソースはすべて、いずれかの項目に添付されています。ここで追加したファイルやリンクは、添付するまでどこにも添付されません。
resources-actions = 操作
resources-not-found = 見つかりません
resources-row-linked = リンクしたファイル
resources-row-linked-size = リンクしたファイル、{ $size }
resources-row-name = { $title }、{ $kind }、{ $detail }
resources-row-name-missing = { $title }、{ $kind }、{ $detail }、見つかりません

## スクリーンリーダーが読み上げるファイルの種類

resources-kind-picture = 画像
resources-kind-pdf = PDF 文書
resources-kind-audio = 録音
resources-kind-video = 動画
resources-kind-document = 文書
resources-kind-ebook = 電子書籍
resources-kind-archive = アーカイブ
resources-kind-file = ファイル
resources-kind-web-link = ウェブリンク

## 行のメニュー

resources-open = 開く
resources-show-in-folder = フォルダーに表示
resources-locate = 場所を指定…
resources-insert-into-text = 本文に挿入
resources-insert-picture = 画像を本文に挿入
resources-insert-link = リンクを本文に挿入
resources-edit = 名前と説明を変更…
resources-detach = この項目から外す
resources-remove = プロジェクトから削除…

## 2 つのダイアログ

resources-link-dialog-title = ウェブリンクを追加
resources-link-dialog-address = アドレス
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = タイトル
resources-link-dialog-name-placeholder = 空欄の場合はサイト名
resources-link-dialog-add = 追加
resources-edit-dialog-title = 名前と説明を変更
resources-edit-dialog-name = タイトル
resources-edit-dialog-description = 説明
resources-edit-dialog-description-placeholder = これが何か、なぜ残しておいたのか
resources-edit-dialog-save = 保存
resources-dialog-cancel = キャンセル

## 追加

resources-large-title = 大きなファイル
resources-large-text = 「{ $name }」のサイズは { $size } です。プロジェクトに保管すると、プロジェクトのすべてのコピーとバックアップに含まれます。リンクにすると、ファイルは今の場所に残り、プロジェクトはその場所だけを記憶します。
resources-large-text-many = 「{ $name }」のサイズは { $size } で、ほかに{ $others ->
   *[other] { $others }個のファイル
}も大きなファイルです。プロジェクトに保管すると、プロジェクトのすべてのコピーとバックアップに含まれます。リンクにすると、ファイルは今の場所に残り、プロジェクトはその場所だけを記憶します。
resources-large-store = プロジェクトに保管
resources-large-link = 代わりにリンクする
resources-large-remember = 今後は確認しない
resources-storing-title = ファイルをプロジェクトに追加しています
resources-store-cancel = キャンセル
resources-store-cancelled = ファイルは追加されませんでした。
resources-store-busy = ファイルの追加がまだ終わっていません。追加が終わってからもう一度お試しください。
resources-store-failed = ファイルを追加できませんでした：{ $error }
resources-no-project = ファイルを添付する前にプロジェクトを開いてください。
resources-moved-away = ファイルのコピー中にプロジェクトが移動されました。
resources-some-failed = { $count ->
   *[other] { $count }個のファイルを追加できませんでした：{ $names }
}
resources-add-failed = リソースを追加できませんでした：{ $error }
resources-added = { $count ->
   *[other] リソースを{ $count }件追加しました
}
resources-dropped-added = { $count ->
   *[other] { $count }個のファイルをこの項目のリソースとして追加し、本文にリンクしました
}
resources-dropped-undo-kept = ドロップ後に本文が変更されたため、何も元に戻していません。
resources-undo = 元に戻す
resources-link-refused = 「{ $url }」は Skribisto が保存できるウェブアドレスではありません。http:// または https:// で始まり、ユーザー名やパスワードを含まないアドレスを使ってください。

## 単一ファイルのプロジェクトへのお知らせ

resources-folder-title = プロジェクトが大きくなっています
resources-folder-text = 保管したファイルが { $size } になりました。単一ファイルとして保存したプロジェクトは、保存のたびにファイルも含めて全体を書き直します。フォルダーとして保存すれば、変更した部分だけを保存します。今すぐフォルダーとして保存し、以後はそのフォルダーで作業を続けることができます。
resources-folder-switch = フォルダーとして保存…
resources-folder-not-now = 後で
resources-folder-never = このプロジェクトでは今後確認しない

## 開く

resources-not-found-toast = 「{ $name }」がプロジェクトの想定する場所にありません。
resources-unsafe-title = 「{ $name }」を開きますか？
resources-unsafe-text = Skribisto には、この種類のファイルが安全かどうかわかりません。開くとコンピューター上でプログラムが起動するおそれがあり、プロジェクトは他人から届いたものかもしれません。まずフォルダーに表示して、中身を確かめてください。
resources-open-anyway = それでも開く
resources-open-failed = ファイルを開けませんでした：{ $error }
resources-no-data-folder = このシステムでは、コピーを開くためのデータフォルダーを Skribisto が使えません
resources-link-gone = このリンクの先のリソースは、もうプロジェクトにありません。
resources-locate-title = 「{ $name }」は今どこにありますか？
resources-relinked-more = { $count ->
   *[other] 同じフォルダーで見つからなかったファイルがほかに{ $count }個見つかり、もう一度リンクしました。
}
resources-changed-elsewhere = 「{ $name }」は別のアプリで変更されました。プロジェクトは新しい版を使います。

## 編集

resources-edit-failed = リソースを変更できませんでした：{ $error }
resources-remove-title = 「{ $title }」をプロジェクトから削除しますか？
resources-remove-text = 添付先のすべての項目から外されます。プロジェクトに保管したファイルは一緒に消えます。リンクしたファイルとウェブページは元の場所に残ります。元に戻すと復元できます。
resources-remove-confirm = 削除
resources-remove-failed = リソースを削除できませんでした：{ $error }
resources-removed-toast = 「{ $title }」をプロジェクトから削除しました。
resources-removed-many-toast = { $count } 件のリソースをプロジェクトから削除しました。
resources-detached-toast = 「{ $title }」の添付を外しました。
resources-attached-one-toast = 「{ $title }」を「{ $item }」のリソースに追加しました。
resources-attached-many-toast = { $count } 件のリソースを「{ $item }」に追加しました。

## 本文へ

resources-no-text = まず本文にカーソルを置いてから、そこにリソースを挿入してください。
resources-drop-insert-picture = 画像を挿入
resources-drop-insert-link = リソースへのリンクを挿入

## 進行状況

progress-resources-storing = ファイルをコピーしています
progress-resources-stored = ファイルをコピーしました

## フォルダーのリソースページ

segment-folder = フォルダー
resources-folder-count = { $count ->
   *[other] リソース{ $count }件
}
resources-view-label = リソースの表示
resources-view-list = リスト
resources-view-grid = グリッド
resources-grid-label = リソース
resources-folder-drop-hint = ファイルやウェブアドレスをここにドロップすると、このフォルダーに添付されます
resources-folder-empty = このフォルダーにも、その中の項目にも、まだ何も添付されていません。上のボタンでファイルやウェブリンクを追加するか、ここにドロップしてください。フォルダーに添付されます。
resources-attached-to = 添付先
resources-attached-separator = 、
resources-attached-more = { $count ->
   *[other] ほか{ $count }件
}
resources-row-name-attached = { $name }、添付先 { $items }
resources-open-item = 「{ $title }」を開く
resources-detach-from = 「{ $title }」から外す

## ギャラリーの画像リソース

gallery-resource-badge = リソース
gallery-tile-resource = { $title }の画像リソース: { $description }

## プロジェクト設定のリソースページ

settings-page-resources = リソース
settings-desc-resources = どの項目にも添付されていないファイルとウェブリンク: もう一度添付する、削除する、コピーを書き出す
settings-resources-description = 最後の項目から外したとき、または添付していた項目を完全に削除したとき、リソースはどこにも添付されていない状態になります。リソースはプロジェクトに残り、保管されたファイルは、もう一度添付するか削除するまで、プロジェクトのすべてのコピーとすべてのバックアップに含まれます。
settings-resources-group-storage = 容量
settings-resources-group-unattached = どこにも添付されていないもの
settings-resources-stored = このプロジェクトに保管されたファイル: { $size }
settings-resources-folder-advice = 1つのファイルとして保存したプロジェクトは、保存のたびに保管ファイルも含めてすべて書き直されます。フォルダーとして保存すれば、変更された部分だけが保存されるので速くなります。
settings-resources-count = { $count ->
   *[other] リソース{ $count }件
}
settings-resources-empty = このプロジェクトのリソースはすべて、いずれかの項目に添付されています。
resources-attach-to-items = 項目に添付…
resources-export-copies = コピーを書き出す…
resources-export-copy = コピーを書き出す…
resources-remove-many-title = { $count }件のリソースをプロジェクトから削除しますか？
resources-attach-title = 項目に添付
resources-attach-hint = 添付先の項目を選んでください。Ctrl+クリックまたは Shift+クリックで複数選べます。
resources-attach-empty = このプロジェクトにはまだ項目がありません。
resources-attach-confirm = 添付
resources-attached = { $count ->
   *[other] { $count }件のリソースを
}{ $items ->
   *[other] { $items }件の項目に添付しました
}
resources-export-title = コピーをフォルダーに書き出す
resources-exported = { $count ->
   *[other] リソースを{ $count }件書き出しました
}
resources-export-missing = { $count ->
   *[other] { $count }件のファイルがプロジェクトの想定する場所にないため、除外しました: { $names }
}
resources-export-failed = { $count ->
   *[other] { $count }件のファイルをコピーできませんでした: { $names }
}

## 大きなファイルの設定 (エディター ▸ 画像とファイル)

settings-resources-large-file = 10 MB を超えるファイル
settings-resources-large-ask = 毎回確認
settings-resources-large-store = 常にプロジェクトに保管
settings-resources-large-link = 常にファイルにリンク
settings-hint-resources-large-file = リソースとして添付する 10 MB を超えるファイルの扱いです。プロジェクトに保管すると、プロジェクトのすべてのコピーとすべてのバックアップに含まれます。リンクすると、ファイルは元の場所に残り、プロジェクトはその場所だけを覚えます。
