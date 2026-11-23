# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

story-bible-modal-configure-title = 新しい項目を設定
story-bible-modal-name-label = 名前
story-bible-modal-name-placeholder = 人物、場所など、記録したいもの…
story-bible-modal-no-binders = このプロジェクトには、項目を保存するバインダーがまだありません
story-bible-modal-template-label = テンプレートから開始（任意）
story-bible-modal-template-placeholder = テンプレートなし
story-bible-modal-body-label = 本文
story-bible-modal-cancel = キャンセル
story-bible-modal-create = 作成
story-bible-modal-create-and-open = 作成して開く
story-bible-modal-failed = 項目を作成できませんでした
ctx-add-as-note = ノートとして追加(&N)…
ctx-add-as-note-all-tags = すべてのタグ…
ctx-add-as-note-untagged = タグなし
story-bible-grid-label = 設定資料集
story-bible-grid-untagged = タグは未設定です
story-bible-grid-empty-title = 登録された項目はまだありません
story-bible-grid-empty-hint = ノートに本文で検索するタグを付けるか、選択したテキストから追加すると、ここに表示されます。
story-bible-grid-alias-count = { $count ->
    [0] 別名はまだありません
   *[other] 別名{ $count }件
}
story-bible-grid-mention-count = { $count ->
    [0] まだどのシーンにも登場していません
   *[other] プロジェクト全体で{ $count }シーンに登場
}
story-bible-books-filter-all = すべての本
story-bible-grid-not-searched = 未検索：設定資料集のタグが必要です
note-in-prose-pov = 視点
note-in-prose-cast = キャスト
note-in-prose-pov-and-cast = 視点 · キャスト
note-in-prose-no-books = このプロジェクトにはまだ本がありません
# The page lists only scenes where the writer placed the entry in the cast or the point of
# view, never the ones the scan merely found its name in, so an empty page says so: a bare
# "nothing here" read as a search that had failed.
note-in-prose-empty-book = この本には、この項目をキャストか視点に含むシーンがまだありません。
note-in-prose-empty-viewpoint = この本には、この項目の視点で語られるシーンがまだありません。
note-in-prose-scope = 「本文内」には、確認済みのシーンだけが表示されます。この項目の名前が出てくるシーンは、「詳細」またはインスペクターでキャストにこの項目を追加するまで、候補のままです。
# A switch beside the Book bar: keep only the scenes this entry narrates.
note-in-prose-viewpoint-only = 視点シーンのみ
note-in-prose-viewpoint-only-tooltip = この項目の視点で語られるシーンだけを表示し、登場するだけのシーンは非表示にします
story-bible-capture-toast = 「{ $name }」を設定資料集に追加しました
story-bible-capture-open = 開く
story-bible-capture-undo = 元に戻す
story-bible-capture-where-title = このノートの保存先を選んでください
story-bible-capture-where-prompt = 新しいノートを保存するフォルダーを選んでください。
story-bible-capture-where-confirm = ここに保存
story-bible-capture-where-needs-folder = このノートを入れるフォルダーを選んでください。
story-bible-capture-where-tag = この確認は初回のみです。今後「{ $tag }」タグの付いたノートはここに保存されます。設定 ▸ プロジェクト ▸ タグで変更できます。
story-bible-capture-where-untagged = この確認は初回のみです。今後タグのないノートはここに保存されます。設定 ▸ プロジェクト ▸ タグで変更できます。

# What a screen reader hears for a binder row that is a story bible entry: its icon
# says so on screen, and this says it aloud. The second form keeps the row's label.
binder-row-story-bible-entry = 設定資料集の項目
binder-row-story-bible-entry-with-label = 設定資料集の項目、{ $label }
