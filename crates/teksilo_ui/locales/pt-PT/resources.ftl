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

## No texto

resources-no-text = Coloque primeiro o cursor num texto, e depois insira lá o recurso.
resources-drop-insert-picture = Inserir a imagem
resources-drop-insert-link = Inserir uma ligação para o recurso

## Progresso

progress-resources-storing = A copiar os ficheiros
progress-resources-stored = Ficheiros copiados
