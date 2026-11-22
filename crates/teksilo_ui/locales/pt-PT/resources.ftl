# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Recursos: ficheiros e ligações web anexados aos itens do arquivador.

## O painel

resources-title = Recursos
shortcut-name-resources-show = Mostrar os recursos
resources-add-files = Adicionar ficheiros
resources-add-files-title = Adicionar ficheiros a este item
resources-add-link = Adicionar uma ligação web
resources-drop-hint = Largue ficheiros ou endereços web para os anexar
resources-no-item = Abra um item para ver os seus recursos.
resources-empty = Ainda não há nada anexado a este item. Adicione ficheiros ou ligações web com os botões acima, ou largue-os aqui.
resources-actions = Ações
resources-not-found = Não encontrado
resources-row-linked = Ficheiro ligado
resources-row-linked-size = Ficheiro ligado, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, não encontrado

## Os tipos de ficheiro, tal como um leitor de ecrã os anuncia

resources-kind-picture = imagem
resources-kind-pdf = documento PDF
resources-kind-audio = gravação de som
resources-kind-video = vídeo
resources-kind-document = documento
resources-kind-ebook = livro eletrónico
resources-kind-archive = arquivo comprimido
resources-kind-file = ficheiro
resources-kind-web-link = ligação web

## O menu de uma linha

resources-open = Abrir
resources-show-in-folder = Mostrar na pasta
resources-locate = Localizar…
resources-insert-into-text = Inserir no texto
resources-insert-picture = Inserir a imagem no texto
resources-insert-link = Inserir uma ligação no texto
resources-edit = Mudar o nome e descrever…
resources-detach = Desanexar deste item
resources-remove = Remover do projeto…

## As duas caixas de diálogo

resources-link-dialog-title = Adicionar uma ligação web
resources-link-dialog-address = Endereço
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Título
resources-link-dialog-name-placeholder = O nome do site, se o deixar vazio
resources-link-dialog-add = Adicionar
resources-edit-dialog-title = Mudar o nome e descrever
resources-edit-dialog-name = Título
resources-edit-dialog-description = Descrição
resources-edit-dialog-description-placeholder = O que é, e porque o guardou
resources-edit-dialog-save = Guardar
resources-dialog-cancel = Cancelar

## Adicionar

resources-large-title = Um ficheiro grande
resources-large-text = «{ $name }» tem { $size }. Guardado no projeto, viaja com cada cópia e cada cópia de segurança do projeto. Ligado, fica onde está, e o projeto apenas se lembra de onde o encontrar.
resources-large-text-many = «{ $name }» tem { $size }, e { $others ->
    [one] outro ficheiro também é grande
   *[other] outros { $others } ficheiros também são grandes
}. Guardados no projeto, viajam com cada cópia e cada cópia de segurança do projeto. Ligados, ficam onde estão, e o projeto apenas se lembra de onde os encontrar.
resources-large-store = Guardá-lo no projeto
resources-large-link = Ligá-lo em vez disso
resources-large-remember = Não voltar a perguntar
resources-storing-title = A adicionar ficheiros ao projeto
resources-store-cancel = Cancelar
resources-store-cancelled = Nenhum ficheiro foi adicionado.
resources-store-busy = Ainda estão a ser adicionados ficheiros. Tente de novo quando tiverem chegado.
resources-store-failed = Não foi possível adicionar os ficheiros: { $error }
resources-no-project = Abra um projeto antes de anexar ficheiros.
resources-moved-away = O projeto foi movido enquanto o ficheiro era copiado.
resources-some-failed = { $count ->
    [one] Não foi possível adicionar um ficheiro: { $names }
   *[other] Não foi possível adicionar { $count } ficheiros: { $names }
}
resources-add-failed = Não foi possível adicionar os recursos: { $error }
resources-added = { $count ->
    [one] Um recurso adicionado
   *[other] { $count } recursos adicionados
}
resources-dropped-added = { $count ->
    [one] Adicionado como recurso deste item, e ligado no texto
   *[other] { $count } ficheiros adicionados como recursos deste item, e ligados no texto
}
resources-undo = Anular
resources-link-refused = «{ $url }» não é um endereço web que o Skribisto possa guardar. Use um endereço que comece por http:// ou https://, sem nome de utilizador nem palavra-passe.

## O aviso para um projeto num único ficheiro

resources-folder-title = Este projeto está a ficar grande
resources-folder-text = Os seus ficheiros guardados ocupam agora { $size }. Um projeto mantido num único ficheiro é reescrito por inteiro a cada gravação, incluindo os seus ficheiros. Mantido como pasta, só grava o que mudou. Pode guardá-lo como pasta agora, e continuar a trabalhar na pasta a partir daí.
resources-folder-switch = Guardar como pasta…
resources-folder-not-now = Agora não
resources-folder-never = Não voltar a perguntar para este projeto

## Abrir

resources-not-found-toast = «{ $name }» não está onde o projeto o espera.
resources-unsafe-title = Abrir «{ $name }»?
resources-unsafe-text = O Skribisto não sabe se este tipo de ficheiro é seguro. Abri-lo poderia iniciar um programa no seu computador, e um projeto pode vir de outra pessoa. Mostre-o primeiro na sua pasta para ver o que é.
resources-open-anyway = Abrir mesmo assim
resources-open-failed = Não foi possível abrir o ficheiro: { $error }
resources-no-data-folder = este sistema não dá ao Skribisto nenhuma pasta de dados onde abrir uma cópia
resources-link-gone = Esta ligação aponta para um recurso que o projeto já não tem.
resources-locate-title = Onde está agora «{ $name }»?
resources-relinked-more = { $count ->
    [one] Foi encontrado mais um ficheiro em falta na mesma pasta, e foi ligado de novo.
   *[other] Foram encontrados mais { $count } ficheiros em falta na mesma pasta, e foram ligados de novo.
}
resources-changed-elsewhere = «{ $name }» foi alterado noutra aplicação. O projeto mantém a nova versão.

## Editar

resources-edit-failed = Não foi possível alterar o recurso: { $error }
resources-remove-title = Remover «{ $title }» do projeto?
resources-remove-text = É retirado de todos os itens a que está anexado. Um ficheiro guardado no projeto vai com ele; um ficheiro ligado e uma página web ficam onde estão. Anular trá-lo de volta.
resources-remove-confirm = Remover
resources-remove-failed = Não foi possível remover o recurso: { $error }
resources-removed-toast = «{ $title }» foi removido do projeto.
resources-removed-many-toast = { $count } recursos foram removidos do projeto.
resources-detached-toast = «{ $title }» foi desanexado.
resources-attached-one-toast = «{ $title }» adicionado aos recursos de «{ $item }».
resources-attached-many-toast = { $count } recursos adicionados a «{ $item }».

## No texto

resources-no-text = Coloque primeiro o cursor num texto, e depois insira lá o recurso.
resources-drop-insert-picture = Inserir a imagem
resources-drop-insert-link = Inserir uma ligação para o recurso

## Progresso

progress-resources-storing = A copiar os ficheiros
progress-resources-stored = Ficheiros copiados

## A página Recursos de uma pasta

segment-folder = Pasta
resources-folder-count = { $count ->
    [one] Um recurso
   *[other] { $count } recursos
}
resources-view-label = Mostrar os recursos como
resources-view-list = Lista
resources-view-grid = Grelha
resources-grid-label = Recursos
resources-folder-drop-hint = Largue ficheiros ou endereços web para os anexar a esta pasta
resources-folder-empty = Ainda não há nada anexado a esta pasta nem ao que ela contém. Adicione ficheiros ou ligações web com os botões acima, ou largue-os aqui: ficam anexados à pasta.
resources-attached-to = Em
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] e mais um
   *[other] e mais { $count }
}
resources-row-name-attached = { $name }, em { $items }
resources-open-item = Abrir «{ $title }»
resources-detach-from = Desanexar de «{ $title }»

## As imagens que são recursos, na Galeria

gallery-resource-badge = Recurso
gallery-tile-resource = Imagem que é um recurso de { $title }: { $description }

## A página Recursos das definições do projeto

settings-page-resources = Recursos
settings-desc-resources = Os ficheiros e ligações web que nenhum item tem já: anexe-os de novo, remova-os ou exporte cópias
settings-resources-description = Um recurso deixa de estar anexado a algo quando é desanexado do seu último item, ou quando o item que o tinha é eliminado de vez. Continua no projeto, e um ficheiro guardado viaja com cada cópia e cada cópia de segurança do projeto, até o voltar a anexar ou o remover.
settings-resources-group-storage = Armazenamento
settings-resources-group-unattached = Sem item
settings-resources-stored = Ficheiros guardados neste projeto: { $size }
settings-resources-folder-advice = Um projeto mantido num único ficheiro é reescrito por inteiro a cada gravação, incluindo os seus ficheiros guardados. Mantido como pasta, só grava o que mudou, o que é mais rápido.
settings-resources-count = { $count ->
    [one] Um recurso
   *[other] { $count } recursos
}
settings-resources-empty = Cada recurso deste projeto está anexado a um item.
resources-attach-to-items = Anexar a itens…
resources-export-copies = Exportar cópias…
resources-export-copy = Exportar uma cópia…
resources-remove-many-title = Remover { $count } recursos do projeto?
resources-attach-title = Anexar a itens
resources-attach-hint = Escolha os itens a que anexar. Ctrl+clique ou Shift+clique escolhe vários.
resources-attach-empty = Este projeto ainda não tem itens.
resources-attach-confirm = Anexar
resources-attached = { $count ->
    [one] Um recurso anexado
   *[other] { $count } recursos anexados
} a { $items ->
    [one] um item
   *[other] { $items } itens
}
resources-export-title = Exportar cópias para uma pasta
resources-exported = { $count ->
    [one] Um recurso exportado
   *[other] { $count } recursos exportados
}
resources-export-missing = { $count ->
    [one] Um ficheiro não está onde o projeto o espera, e foi deixado de fora: { $names }
   *[other] { $count } ficheiros não estão onde o projeto os espera, e foram deixados de fora: { $names }
}
resources-export-failed = { $count ->
    [one] Um ficheiro não pôde ser copiado: { $names }
   *[other] { $count } ficheiros não puderam ser copiados: { $names }
}

## A preferência dos ficheiros grandes (Comportamento do editor)

settings-resources-large-file = Ficheiros com mais de 10 MB
settings-resources-large-ask = Perguntar de cada vez
settings-resources-large-store = Guardar sempre no projeto
settings-resources-large-link = Ligar sempre ao ficheiro
settings-hint-resources-large-file = O que acontece a um ficheiro com mais de 10 MB que anexa como recurso. Guardado no projeto, viaja com cada cópia e cada cópia de segurança do projeto. Ligado, fica onde está, e o projeto só guarda onde o encontrar.
