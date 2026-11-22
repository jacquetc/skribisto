# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# 리소스: 문서 트리의 항목에 첨부한 파일과 웹 링크.

## 패널

resources-title = 리소스
shortcut-name-resources-show = 리소스 표시
resources-add-files = 파일 추가
resources-add-files-title = 이 항목에 파일 추가
resources-add-link = 웹 링크 추가
resources-drop-hint = 파일이나 웹 주소를 여기에 놓으면 첨부됩니다
resources-no-item = 리소스를 보려면 항목을 여세요.
resources-empty = 이 항목에는 아직 첨부된 것이 없습니다. 위의 버튼으로 파일이나 웹 링크를 추가하거나 여기에 놓으세요.
resources-actions = 작업
resources-not-found = 찾을 수 없음
resources-row-linked = 연결된 파일
resources-row-linked-size = 연결된 파일, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, 찾을 수 없음

## 화면 낭독기가 읽어 주는 파일 종류

resources-kind-picture = 이미지
resources-kind-pdf = PDF 문서
resources-kind-audio = 녹음
resources-kind-video = 동영상
resources-kind-document = 문서
resources-kind-ebook = 전자책
resources-kind-archive = 압축 파일
resources-kind-file = 파일
resources-kind-web-link = 웹 링크

## 행 메뉴

resources-open = 열기
resources-show-in-folder = 폴더에서 표시
resources-locate = 위치 찾기…
resources-insert-into-text = 본문에 삽입
resources-insert-picture = 이미지를 본문에 삽입
resources-insert-link = 본문에 링크 삽입
resources-edit = 이름 바꾸기 및 설명…
resources-detach = 이 항목에서 떼기
resources-remove = 프로젝트에서 제거…

## 두 대화 상자

resources-link-dialog-title = 웹 링크 추가
resources-link-dialog-address = 주소
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = 제목
resources-link-dialog-name-placeholder = 비워 두면 사이트 이름
resources-link-dialog-add = 추가
resources-edit-dialog-title = 이름 바꾸기 및 설명
resources-edit-dialog-name = 제목
resources-edit-dialog-description = 설명
resources-edit-dialog-description-placeholder = 이것이 무엇이고 왜 보관했는지
resources-edit-dialog-save = 저장
resources-dialog-cancel = 취소

## 추가

resources-large-title = 큰 파일
resources-large-text = "{ $name }"의 크기는 { $size }입니다. 프로젝트에 보관하면 프로젝트의 모든 사본과 백업에 함께 들어갑니다. 연결하면 파일은 지금 있는 곳에 그대로 있고, 프로젝트는 파일이 있는 위치만 기억합니다.
resources-large-text-many = "{ $name }"의 크기는 { $size }이고, 다른 파일 { $others ->
   *[other] { $others }개
}도 큽니다. 프로젝트에 보관하면 프로젝트의 모든 사본과 백업에 함께 들어갑니다. 연결하면 파일은 지금 있는 곳에 그대로 있고, 프로젝트는 파일이 있는 위치만 기억합니다.
resources-large-store = 프로젝트에 보관
resources-large-link = 대신 연결
resources-large-remember = 다시 묻지 않음
resources-storing-title = 프로젝트에 파일을 추가하는 중
resources-store-cancel = 취소
resources-store-cancelled = 추가된 파일이 없습니다.
resources-store-busy = 아직 파일을 추가하는 중입니다. 추가가 끝나면 다시 시도하세요.
resources-store-failed = 파일을 추가할 수 없습니다: { $error }
resources-no-project = 파일을 첨부하려면 먼저 프로젝트를 여세요.
resources-moved-away = 파일을 복사하는 동안 프로젝트가 이동되었습니다.
resources-some-failed = { $count ->
   *[other] 파일 { $count }개를 추가할 수 없습니다: { $names }
}
resources-add-failed = 리소스를 추가할 수 없습니다: { $error }
resources-added = { $count ->
   *[other] 리소스 { $count }개를 추가했습니다
}
resources-dropped-added = { $count ->
   *[other] 파일 { $count }개를 이 항목의 리소스로 추가하고 본문에 연결했습니다
}
resources-undo = 실행 취소
resources-link-refused = "{ $url }"은(는) Skribisto가 보관할 수 있는 웹 주소가 아닙니다. 사용자 이름이나 비밀번호 없이 http:// 또는 https://로 시작하는 주소를 사용하세요.

## 단일 파일 프로젝트 알림

resources-folder-title = 프로젝트가 커지고 있습니다
resources-folder-text = 보관한 파일이 차지하는 공간이 이제 { $size }입니다. 단일 파일로 저장한 프로젝트는 저장할 때마다 파일까지 포함해 전체를 다시 씁니다. 폴더로 저장하면 바뀐 부분만 저장합니다. 지금 폴더로 저장하고 이후에는 그 폴더에서 계속 작업할 수 있습니다.
resources-folder-switch = 폴더로 저장…
resources-folder-not-now = 나중에
resources-folder-never = 이 프로젝트에서는 다시 묻지 않음

## 열기

resources-not-found-toast = "{ $name }"이(가) 프로젝트가 예상하는 위치에 없습니다.
resources-unsafe-title = "{ $name }"을(를) 열까요?
resources-unsafe-text = Skribisto는 이 종류의 파일이 안전한지 알지 못합니다. 열면 컴퓨터에서 프로그램이 실행될 수 있고, 프로젝트는 다른 사람에게서 온 것일 수 있습니다. 먼저 폴더에서 표시해 무엇인지 확인하세요.
resources-open-anyway = 그래도 열기
resources-open-failed = 파일을 열 수 없습니다: { $error }
resources-no-data-folder = 이 시스템은 Skribisto에 사본을 열 데이터 폴더를 제공하지 않습니다
resources-link-gone = 이 링크는 프로젝트에 더 이상 없는 리소스를 가리킵니다.
resources-locate-title = "{ $name }"은(는) 지금 어디에 있나요?
resources-relinked-more = { $count ->
   *[other] 같은 폴더에서 없어진 파일 { $count }개를 더 찾아 다시 연결했습니다.
}
resources-changed-elsewhere = "{ $name }"이(가) 다른 앱에서 변경되었습니다. 프로젝트는 새 버전을 유지합니다.

## 편집

resources-edit-failed = 리소스를 변경할 수 없습니다: { $error }
resources-remove-title = 프로젝트에서 "{ $title }"을(를) 제거할까요?
resources-remove-text = 첨부된 모든 항목에서 떼어 냅니다. 프로젝트에 보관한 파일은 함께 사라지고, 연결된 파일과 웹 페이지는 있던 곳에 그대로 남습니다. 실행 취소하면 되돌릴 수 있습니다.
resources-remove-confirm = 제거
resources-remove-failed = 리소스를 제거할 수 없습니다: { $error }
resources-removed-toast = “{ $title }”을(를) 프로젝트에서 제거했습니다.
resources-removed-many-toast = 리소스 { $count }개를 프로젝트에서 제거했습니다.
resources-detached-toast = “{ $title }”의 첨부를 해제했습니다.
resources-attached-one-toast = “{ $title }”을(를) “{ $item }”의 리소스에 추가했습니다.
resources-attached-many-toast = 리소스 { $count }개를 “{ $item }”에 추가했습니다.

## 본문에

resources-no-text = 먼저 본문에 커서를 놓은 다음 그곳에 리소스를 삽입하세요.
resources-drop-insert-picture = 이미지 삽입
resources-drop-insert-link = 리소스 링크 삽입

## 진행 상황

progress-resources-storing = 파일 복사 중
progress-resources-stored = 파일 복사 완료

## 폴더의 리소스 페이지

segment-folder = 폴더
resources-folder-count = { $count ->
   *[other] 리소스 { $count }개
}
resources-view-label = 리소스 보기 방식
resources-view-list = 목록
resources-view-grid = 격자
resources-grid-label = 리소스
resources-folder-drop-hint = 파일이나 웹 주소를 여기에 놓으면 이 폴더에 첨부됩니다
resources-folder-empty = 이 폴더와 그 안의 항목에는 아직 첨부된 것이 없습니다. 위의 버튼으로 파일이나 웹 링크를 추가하거나 여기에 놓으세요. 폴더에 첨부됩니다.
resources-attached-to = 첨부된 곳
resources-attached-separator = ,
resources-attached-more = { $count ->
   *[other] 외 { $count }개
}
resources-row-name-attached = { $name }, 첨부된 곳 { $items }
resources-open-item = "{ $title }" 열기
resources-detach-from = "{ $title }"에서 떼기

## 갤러리의 이미지 리소스

gallery-resource-badge = 리소스
gallery-tile-resource = { $title }의 이미지 리소스: { $description }

## 프로젝트 설정의 리소스 페이지

settings-page-resources = 리소스
settings-desc-resources = 더 이상 어떤 항목에도 첨부되지 않은 파일과 웹 링크: 다시 첨부하거나, 제거하거나, 사본을 내보냅니다
settings-resources-description = 리소스를 마지막 항목에서 떼거나, 리소스가 첨부된 항목을 완전히 삭제하면 그 리소스는 어디에도 첨부되지 않은 상태가 됩니다. 리소스는 프로젝트에 남고, 보관된 파일은 다시 첨부하거나 제거할 때까지 프로젝트의 모든 사본과 모든 백업에 함께 들어갑니다.
settings-resources-group-storage = 저장 공간
settings-resources-group-unattached = 어디에도 첨부되지 않음
settings-resources-stored = 이 프로젝트에 보관된 파일: { $size }
settings-resources-folder-advice = 파일 하나로 저장한 프로젝트는 저장할 때마다 보관된 파일까지 모두 다시 씁니다. 폴더로 저장하면 바뀐 부분만 저장하므로 더 빠릅니다.
settings-resources-count = { $count ->
   *[other] 리소스 { $count }개
}
settings-resources-empty = 이 프로젝트의 모든 리소스가 어떤 항목에 첨부되어 있습니다.
resources-attach-to-items = 항목에 첨부…
resources-export-copies = 사본 내보내기…
resources-export-copy = 사본 내보내기…
resources-remove-many-title = 프로젝트에서 리소스 { $count }개를 제거할까요?
resources-attach-title = 항목에 첨부
resources-attach-hint = 첨부할 항목을 고르세요. Ctrl+클릭이나 Shift+클릭으로 여러 개를 고를 수 있습니다.
resources-attach-empty = 이 프로젝트에는 아직 항목이 없습니다.
resources-attach-confirm = 첨부
resources-attached = { $count ->
   *[other] 리소스 { $count }개를
} { $items ->
   *[other] 항목 { $items }개에 첨부했습니다
}
resources-export-title = 사본을 폴더로 내보내기
resources-exported = { $count ->
   *[other] 리소스 { $count }개를 내보냈습니다
}
resources-export-missing = { $count ->
   *[other] 파일 { $count }개가 프로젝트가 예상한 곳에 없어 제외했습니다: { $names }
}
resources-export-failed = { $count ->
   *[other] 파일 { $count }개를 복사하지 못했습니다: { $names }
}

## 큰 파일 설정 (편집기 동작)

settings-resources-large-file = 10 MB가 넘는 파일
settings-resources-large-ask = 매번 묻기
settings-resources-large-store = 항상 프로젝트에 보관
settings-resources-large-link = 항상 파일에 연결
settings-hint-resources-large-file = 리소스로 첨부하는 10 MB가 넘는 파일을 어떻게 할지 정합니다. 프로젝트에 보관하면 프로젝트의 모든 사본과 모든 백업에 함께 들어갑니다. 연결하면 파일은 원래 자리에 그대로 있고, 프로젝트는 그 위치만 기억합니다.
