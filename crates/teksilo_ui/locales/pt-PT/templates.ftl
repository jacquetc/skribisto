# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Modelos
settings-templates-description = Peças reutilizáveis de escrita que você pode usar em qualquer coisa que você está escrevendo: uma folha de personagens em branco, um perfil de localização, uma planilha de batida. Eles são armazenados neste projeto, então qualquer um que o abra recebe o mesmo conjunto.
settings-templates-filter = Modelos de filtro
settings-templates-count = { $n ->
    [0] Sem modelos
    [one] 1 modelo
   *[other] { $n } modelos
}
settings-templates-empty = Este projeto ainda não tem modelos.
settings-templates-empty-hint = Adicione um de um predefinido, importe um ficheiro .md ou .djot, ou escreva algo e escolha Document ▸ Guardar como modelo.
settings-templates-presets = Adicionar uma predefinição
settings-templates-import = Importar...
settings-templates-export = Exportar...
settings-templates-star = Mostrar primeiro no menu de inserção
settings-templates-unstar = Parar de mostrar primeiro
settings-templates-move-up = Subir
settings-templates-move-down = Mover para baixo
settings-templates-delete = Apagar modelo
settings-templates-name = Nome
settings-templates-name-placeholder = Nome do modelo
settings-templates-body-placeholder = O texto que este modelo insere
settings-templates-duplicate-name = Outro modelo já é chamado de "{ $name }"
settings-templates-words = { $n ->
    [one] 1 palavra
   *[other] { $n } palavras
}

templates-imported = { $added ->
    [one] 1 modelo importado
   *[other] { $added } modelos importados
}
templates-imported-renamed = Renomeado para evitar um confronto: { $names }
templates-imported-skipped = Não foi possível ler: { $names }
templates-import-filter = Modelos (.md, .djot)
templates-exported = { $n ->
    [one] 1 modelo exportado
   *[other] { $n } modelos exportados
}
templates-preset-applied = Adicionado "{ $name }"

templates-delete-title = Apagar este modelo?
templates-delete-body = "{ $name }" será removido deste projeto. Notas que você já criou não são afetadas.
templates-delete-confirm = Eliminar

menu-document = &Documento
menu-insert-template = &Inserir modelo
menu-insert-template-none = Sem modelos neste projeto
menu-save-as-template = &Gravar como modelo...

save-as-template-title = Gravar como modelo
save-as-template-explain = O texto que você está editando torna-se um modelo que você pode soltar em qualquer outro lugar.
save-as-template-name = Nome
save-as-template-placeholder = Ficha de personagem
save-as-template-duplicate = Já existe um modelo chamado "{ $name }"
save-as-template-empty-editor = Este editor está vazio. Não há nada para gravar.
save-as-template-confirm = Gravar o modelo
save-as-template-saved = Gravado "{ $name }" como um modelo
template-inserted = Introduzido em "{ $name }"

note-template-preset-character-sheet = Ficha de personagem
note-template-preset-location = Local
note-template-preset-artifact = Objeto
note-template-preset-beat-sheet = Esquema narrativo
note-template-preset-faction = Façção
note-template-preset-research-note = Nota de pesquisa

note-template-set-essentials = Essenciais
note-template-set-everything = Cada modelo

note-template-section-identity = Identidade
note-template-section-appearance = Aparência
note-template-section-voice = Voz
note-template-section-psychology = Vida interior
note-template-section-history = Histórico
note-template-section-arc = Arco narrativo
note-template-section-first-impression = Primeira impressão
note-template-section-in-the-story = Na história
note-template-section-the-thing-itself = O próprio objeto
note-template-section-the-scene = A cena
note-template-section-the-shape = A forma
note-template-section-what-it-is = O que é isso?
note-template-section-source = Origem
note-template-section-what-it-says = O que está escrito

note-template-field-full-name = Nome completo
note-template-field-known-as = Também conhecido como, localizado no campo de Alias do Inspector
note-template-field-age = Idade
note-template-field-role-in-story = Papel na história
note-template-field-build-and-features = Constrói e distingue características
note-template-field-habitual-bearing = Rolamento habitual
note-template-field-speech-patterns = Padrões de fala e tiques verbais
note-template-field-what-they-never-say = O que eles nunca dizem
note-template-field-want = O que eles querem
note-template-field-need = O que eles precisam, mas não sabem disso.
note-template-field-fear = O que eles temem
note-template-field-flaw = A falha que lhes custa
note-template-field-formative-event = Evento formativo
note-template-field-relationships = Relações
note-template-field-starts-as = Começa como
note-template-field-ends-as = Termina como
note-template-field-what-you-notice-first = O que você nota primeiro
note-template-field-sound-and-smell = Som e olfato
note-template-field-light-and-weather = Luz e tempo
note-template-field-what-happened-here = O que aconteceu aqui?
note-template-field-who-lives-or-works-here = Quem vive ou trabalha aqui
note-template-field-scenes-set-here = Cenas definidas aqui
note-template-field-why-it-matters = Por que isso importa?
note-template-field-appearance = Aparência
note-template-field-age-and-origin = Idade e origem
note-template-field-what-it-does = O que ele faz
note-template-field-who-holds-it = Quem o segura?
note-template-field-who-wants-it = Quem o quer?
note-template-field-pov = Ponto de vista
note-template-field-time-and-place = Hora e local
note-template-field-goal = Objetivo
note-template-field-conflict = Conflito
note-template-field-turn = A viragem
note-template-field-exit-emotion = Emoção à saída
note-template-field-purpose = Objecto
note-template-field-who-leads-it = Quem a lidera?
note-template-field-resources = Recursos
note-template-field-allies-and-enemies = Aliados e inimigos
note-template-field-what-it-wants-now = O que ele quer agora
note-template-field-where-from = De onde veio
note-template-field-page-or-link = Página ou link
note-template-field-key-facts = Factos essenciais
note-template-field-how-it-is-used = Como é usado na história
