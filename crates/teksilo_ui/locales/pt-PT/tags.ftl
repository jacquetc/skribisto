# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Básico
tags-preset-scifi = Ficção científica
tags-preset-fantasy = Fantasia
tags-preset-mystery = Mistério
tags-preset-historical = Histórico


tags-preset-needs-research = precisa de pesquisa
tags-preset-continuity-check = verificação da continuidade
tags-preset-plot-point = ponto de viragem

tags-preset-character = personagem
tags-preset-place = lugar
tags-preset-item = objeto

tags-preset-vessel = nave
tags-preset-planet = planeta
tags-preset-organization = organização
tags-preset-creature = criatura
tags-preset-faction = fação
tags-preset-artifact = artefacto
tags-preset-realm = reino
tags-preset-magic-system = sistema mágico
tags-preset-suspect = suspeito
tags-preset-victim = vítima
tags-preset-clue = pista
tags-preset-red-herring = pista falsa
tags-preset-historical-figure = figura histórica
tags-preset-source = fonte
tags-preset-period-detail = pormenor de época

tags-pill-list = Etiquetas
tags-pill-add = Adicionar uma etiqueta
tags-none-yet = Sem etiquetas
tags-pick-list = Etiquetas disponíveis
tags-pill-remove = Remover { $name }
tags-pill-filter-placeholder = Filtrar ou nomear uma nova etiqueta
tags-pill-no-match = Nenhuma etiqueta corresponde
tags-pill-create = Criar «{ $name }»
tags-pill-create-failed = Não foi possível criar «{ $name }»
tags-pill-new-discoverable = Etiqueta procurar na prosa
tags-pill-new-discoverable-hint = Os itens com esta etiqueta são procurados na sua prosa para preencher o elenco.

tags-alias-list = Também conhecido como
tags-alias-add = Adicionar outro nome
tags-alias-remove = Remover { $name }
tags-alias-placeholder = Outro nome e depois Enter
tags-alias-hint = Nomes com que aparece na sua prosa, além do título.
tags-alias-collision = { $name } já responde a este nome.

settings-page-tags = Etiquetas
settings-tags-desc = As etiquetas marcam os itens do seu arquivador. Uma etiqueta procurar na prosa diz também ao Skribisto para procurar os nomes desse item na sua prosa.
settings-tags-add = Adicionar etiqueta
settings-tags-add-placeholder = Dar nome a uma nova etiqueta
settings-tags-added = «{ $name }» adicionada
settings-tags-duplicate = «{ $name }» já existe
settings-tags-filter = Filtrar etiquetas
settings-tags-count = { $n ->
    [one] 1 etiqueta
   *[other] { $n } etiquetas
}
settings-tags-details-placeholder = O que esta etiqueta significa
settings-tags-discoverable = Procurar na prosa
settings-tags-creates-in = Novas notas vão para
settings-tags-creates-in-unset = Perguntar da primeira vez
settings-tags-creates-in-untitled = Pasta sem título
settings-tags-creates-in-trashed = { $name } (no lixo)
settings-tags-template = Modelo inicial
settings-tags-template-unset = Nota em branco
settings-tags-delete = Eliminar { $name }
settings-tags-deleted = «{ $name }» eliminada e removida de todos os itens
settings-tags-empty = Ainda não há etiquetas.
settings-tags-apply-preset = Aplicar uma predefinição…
settings-tags-preset-applied = { $added } adicionadas, { $skipped } já presentes ignoradas
settings-tags-csv-filter = Ficheiros CSV
settings-tags-import = Importar…
settings-tags-export = Exportar…
settings-tags-imported = { $added } importadas, { $skipped } ignoradas
settings-tags-exported = { $n ->
    [one] 1 etiqueta exportada
   *[other] { $n } etiquetas exportadas
}

tags-chip-more = { $n ->
    [one] Mais 1 etiqueta
   *[other] Mais { $n } etiquetas
}

cast-section = Elenco
cast-add = Adicionar ao elenco…
cast-add-filter-placeholder = Filtrar a bíblia da história…
cast-add-empty = Nenhuma entrada da bíblia da história para adicionar
cast-pin = Adicionar { $name } ao elenco
cast-unpin = Remover { $name } do elenco
cast-empty = Ainda ninguém foi fixado. Adicione ou mantenha uma sugestão
cast-unresolved = Já não está na bíblia da história

mentions-confirm = Adicionar ao elenco de { $name }
mentions-hit-count = { $n ->
    [one] uma vez
   *[other] { $n } vezes
}
mentions-point-of-view-badge = Ponto de vista
mentions-point-of-view-badge-tooltip = Declarado aqui como ponto de vista, definido à mão, não detetado na prosa.

mentions-roster = Elenco
mentions-pin = Adicionar { $name } ao elenco

pov-section = Ponto de vista
pov-empty = Nenhum ponto de vista definido
pov-add = Definir o ponto de vista…
pov-multiple = Esta cena tem dois pontos de vista.
pov-remove = Remover { $name } como ponto de vista
pov-unresolved = Foi fixado aqui um ponto de vista, mas essa entrada já não existe ou perdeu a sua etiqueta da bíblia da história.

books-section = Associado a
books-empty = Ainda não associado a nenhum livro
books-add = Associar a um livro…
books-remove = Remover de { $name }
books-apply-to-children = Aplicar a associação aos filhos

note-details-name-placeholder = Nome…
note-details-tags = Etiquetas
note-details-aliases = Outros nomes
note-details-books = Associado a
note-details-books-empty = Ainda não associado a nenhum livro
note-details-links = Ligações
note-details-links-empty = Ainda não há ligações. Adicione uma para apontar para outra entrada da sua bíblia da história.
note-details-links-add = Ligar a uma nota…
note-details-pov = Ponto de vista
note-details-pov-empty = Nenhum ponto de vista definido
note-details-pov-unresolved = Foi fixado aqui um ponto de vista, mas essa entrada já não existe ou perdeu a sua etiqueta da bíblia da história.
note-details-pov-multiple = Esta nota tem dois pontos de vista.
note-details-backlinks = Aparece no manuscrito
note-details-backlinks-empty = Ainda nada. Assim que este nome aparecer na sua prosa, aparecerá aqui.
note-details-backlinks-outside = Fora dos livros
note-details-backlinks-confirm-all = Confirmar todas as aparições
note-details-backlinks-confirm-all-tooltip = Adicionar esta entrada ao elenco de todos os documentos mostrados aqui que ainda não a confirmaram
note-details-untitled-document = Sem título

note-in-prose-mentions = { $n ->
    [one] 1 menção
   *[other] { $n } menções
}
note-in-prose-mention-at = { $current } de { $total }
note-in-prose-mention-previous = Menção anterior
note-in-prose-mention-next = Menção seguinte
