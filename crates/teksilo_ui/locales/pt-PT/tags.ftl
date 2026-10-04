# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

tags-preset-basic = Básico
tags-preset-scifi = Ficção científica
tags-preset-fantasy = Fantasia
tags-preset-mystery = Mistério
tags-preset-historical = Histórico


tags-preset-needs-research = necessita de investigação
tags-preset-continuity-check = verificação da continuidade
tags-preset-plot-point = ponto de gráfico

tags-preset-character = caracter
tags-preset-place = local
tags-preset-item = item

tags-preset-vessel = Navio
tags-preset-planet = planeta
tags-preset-organization = organização
tags-preset-creature = criatura
tags-preset-faction = facção
tags-preset-artifact = artefacto
tags-preset-realm = reino
tags-preset-magic-system = sistema mágico
tags-preset-suspect = suspeito
tags-preset-victim = vítima
tags-preset-clue = pista
tags-preset-red-herring = arenque vermelho
tags-preset-historical-figure = figura histórica
tags-preset-source = fonte
tags-preset-period-detail = detalhe do período

tags-pill-list = Etiquetas
tags-pill-add = Adicionar uma etiqueta
tags-none-yet = Sem etiquetas
tags-pick-list = Etiquetas disponíveis
tags-pill-remove = Remover { $name }
tags-pill-filter-placeholder = Filtrar ou nomear uma nova etiqueta
tags-pill-no-match = Sem correspondência de etiquetas
tags-pill-create = Criar "{ $name }"
tags-pill-create-failed = Não foi possível criar "{ $name }"
tags-pill-new-discoverable = Marca de procura em promoção
tags-pill-new-discoverable-hint = Os itens com esta etiqueta são combinados com a sua prosa para preencher a lista.

tags-alias-list = Outros nomes
tags-alias-add = Adicionar outro nome
tags-alias-remove = Remover { $name }
tags-alias-placeholder = Outro nome, em seguida, Enter
tags-alias-hint = Nomes que isso aparece em sua prosa, ao lado de seu título.
tags-alias-collision = { $name } já responde a este nome.

settings-page-tags = Etiquetas
settings-tags-desc = As etiquetas rotulam os itens em seu fichário. Uma tag find-in-prose também diz Skribisto para procurar os nomes desse item em sua prosa.
settings-tags-add = Adicionar etiqueta
settings-tags-add-placeholder = Nomear uma nova etiqueta
settings-tags-added = Adicionado "{ $name }"
settings-tags-duplicate = "{ $name }" já existe
settings-tags-filter = Marcas de filtro
settings-tags-count = { $n ->
    [one] 1 etiqueta
   *[other] { $n } tags
}
settings-tags-details-placeholder = O que esta etiqueta significa
settings-tags-discoverable = Procurar no texto
settings-tags-creates-in = Novas notas vão para
settings-tags-creates-in-unset = Pergunta-me da primeira vez.
settings-tags-creates-in-untitled = Pasta sem título
settings-tags-creates-in-trashed = { $name } (no lixo)
settings-tags-template = Modelo inicial
settings-tags-template-unset = Nota em branco
settings-tags-delete = Apagar { $name }
settings-tags-deleted = Excluído "{ $name }" e removido de cada item
settings-tags-empty = Ainda não há etiquetas.
settings-tags-apply-preset = Aplicar uma predefinição...
settings-tags-preset-applied = Adicionado { $added }, pulado { $skipped } já presente
settings-tags-csv-filter = CSV ficheiros
settings-tags-import = Importar...
settings-tags-export = Exportar...
settings-tags-imported = Importado { $added }, pulou { $skipped }
settings-tags-exported = Exportado { $n ->
    [one] 1 etiqueta
   *[other] { $n } tags
}

tags-chip-more = { $n ->
    [one] Mais 1 tag
   *[other] { $n } mais tags
}

cast-section = Personagens presentes
cast-add = Adicionar ao elenco...
cast-add-filter-placeholder = Filtrar a bíblia da história...
cast-add-empty = Nenhum item de contas de histórias a adicionar
cast-pin = Adicionar { $name } ao elenco
cast-unpin = Remover { $name } do elenco
cast-empty = Ainda ninguém foi preso. Adicione ou mantenha uma sugestão
cast-unresolved = Já não está na bíblia da história.

mentions-confirm = Adicionar ao elenco de { $name }
mentions-hit-count = { $n ->
    [one] uma vez
   *[other] { $n } vezes
}
mentions-point-of-view-badge = Ponto de vista
mentions-point-of-view-badge-tooltip = Declarado como o ponto de vista aqui, posto à mão, não detectado na prosa.

mentions-roster = Personagens presentes
mentions-pin = Adicionar { $name } ao elenco

pov-section = Ponto de vista
pov-empty = Nenhum ponto de vista definido
pov-add = Definir o ponto de vista...
pov-multiple = Esta cena tem dois pontos de vista.
pov-remove = Remover { $name } como ponto de vista
pov-unresolved = Um ponto de vista foi fixado aqui, mas essa entrada não existe mais ou perdeu sua tag história-bíblia.

books-section = Associado a
books-empty = Não arquivado sob um livro ainda
books-add = Ficheiro sob um livro...
books-remove = Remover de { $name }
books-apply-to-children = Aplicar arquivamento a elementos subordinados

note-details-name-placeholder = Nome…
note-details-tags = Etiquetas
note-details-aliases = Outros nomes
note-details-books = Associado a
note-details-books-empty = Ainda não arquivado sob um Livro
note-details-links = Ligações
note-details-links-empty = Sem links ainda. Adicione um para apontar para outro item na sua bíblia de história.
note-details-links-add = Ligação a uma nota...
note-details-pov = Ponto de vista
note-details-pov-empty = Nenhum ponto de vista definido
note-details-pov-unresolved = Um ponto de vista foi fixado aqui, mas essa entrada não existe mais ou perdeu sua tag história-bíblia.
note-details-pov-multiple = Esta nota tem dois pontos de vista.
note-details-backlinks = Aparece no manuscrito
note-details-backlinks-empty = Quando este nome aparecer na tua prosa, aparecerá aqui.
note-details-backlinks-outside = Fora dos livros
note-details-backlinks-confirm-all = Confirmar cada aparência
note-details-backlinks-confirm-all-tooltip = Adicionar esta entrada ao elenco de cada documento mostrado aqui que ainda não o confirmou
note-details-untitled-document = Sem título

note-in-prose-mentions = { $n ->
    [one] 1 menção
   *[other] { $n } menciona
}
note-in-prose-mention-at = { $current } de { $total }
note-in-prose-mention-previous = Menção anterior
note-in-prose-mention-next = Menção seguinte
