# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

settings-page-templates = Modelos
settings-templates-description = Textos reutilizáveis que pode inserir em qualquer coisa que esteja a escrever: uma ficha de personagem em branco, um perfil de lugar, uma grelha de cenas. São guardados neste projeto, por isso quem o abrir recebe o mesmo conjunto.
settings-templates-filter = Filtrar modelos
settings-templates-count = { $n ->
    [0] Sem modelos
    [one] 1 modelo
   *[other] { $n } modelos
}
settings-templates-empty = Este projeto ainda não tem modelos.
settings-templates-empty-hint = Adicione um a partir de uma predefinição, importe um ficheiro .md ou .djot, ou escreva algo e escolha Documento ▸ Guardar como modelo.
settings-templates-presets = Adicionar uma predefinição
settings-templates-import = Importar…
settings-templates-export = Exportar…
settings-templates-star = Mostrar primeiro no menu de inserção
settings-templates-unstar = Parar de mostrar primeiro
settings-templates-move-up = Subir
settings-templates-move-down = Descer
settings-templates-delete = Eliminar modelo
settings-templates-name = Nome
settings-templates-name-placeholder = Nome do modelo
settings-templates-body-placeholder = O texto que este modelo insere
settings-templates-duplicate-name = Já existe outro modelo chamado «{ $name }»
settings-templates-words = { $n ->
    [one] 1 palavra
   *[other] { $n } palavras
}

templates-imported = { $added ->
    [one] 1 modelo importado
   *[other] { $added } modelos importados
}
templates-imported-renamed = Nome alterado para evitar um conflito: { $names }
templates-imported-skipped = Não foi possível ler: { $names }
templates-import-filter = Modelos (.md, .djot)
templates-exported = { $n ->
    [one] 1 modelo exportado
   *[other] { $n } modelos exportados
}
templates-preset-applied = «{ $name }» adicionado

templates-delete-title = Eliminar este modelo?
templates-delete-body = «{ $name }» será removido deste projeto. As notas que já criou a partir dele não são afetadas.
templates-delete-confirm = Eliminar

menu-document = &Documento
menu-insert-template = &Inserir modelo
menu-insert-template-none = Sem modelos neste projeto
menu-save-as-template = &Guardar como modelo…

save-as-template-title = Guardar como modelo
save-as-template-explain = O texto que está a editar torna-se um modelo que pode inserir em qualquer outro lugar.
save-as-template-name = Nome
save-as-template-placeholder = Ficha de personagem
save-as-template-duplicate = Já existe um modelo chamado «{ $name }»
save-as-template-empty-editor = Este editor está vazio. Não há nada para guardar.
save-as-template-confirm = Guardar o modelo
save-as-template-saved = «{ $name }» guardado como modelo
template-inserted = «{ $name }» inserido

note-template-preset-character-sheet = Ficha de personagem
note-template-preset-location = Lugar
note-template-preset-artifact = Objeto
note-template-preset-beat-sheet = Grelha de cenas
note-template-preset-faction = Fação
note-template-preset-research-note = Nota de pesquisa

note-template-set-essentials = Essenciais
note-template-set-everything = Todos os modelos

note-template-section-identity = Identidade
note-template-section-appearance = Aparência
note-template-section-voice = Voz
note-template-section-psychology = Vida interior
note-template-section-history = Passado
note-template-section-arc = Arco narrativo
note-template-section-first-impression = Primeira impressão
note-template-section-in-the-story = Na história
note-template-section-the-thing-itself = O próprio objeto
note-template-section-the-scene = A cena
note-template-section-the-shape = A forma
note-template-section-what-it-is = O que é
note-template-section-source = Origem
note-template-section-what-it-says = O que está escrito

note-template-field-full-name = Nome completo
note-template-field-known-as = Também conhecido como (registado no campo Também conhecido como do Inspetor)
note-template-field-age = Idade
note-template-field-role-in-story = Papel na história
note-template-field-build-and-features = Constituição e traços distintivos
note-template-field-habitual-bearing = Postura habitual
note-template-field-speech-patterns = Padrões de fala e tiques verbais
note-template-field-what-they-never-say = O que nunca diz
note-template-field-want = O que quer
note-template-field-need = Aquilo de que precisa sem o saber
note-template-field-fear = O que teme
note-template-field-flaw = O defeito que lhe sai caro
note-template-field-formative-event = Evento formativo
note-template-field-relationships = Relações
note-template-field-starts-as = Começa como
note-template-field-ends-as = Termina como
note-template-field-what-you-notice-first = O que se nota primeiro
note-template-field-sound-and-smell = Som e olfato
note-template-field-light-and-weather = Luz e clima
note-template-field-what-happened-here = O que aconteceu aqui
note-template-field-who-lives-or-works-here = Quem vive ou trabalha aqui
note-template-field-scenes-set-here = Cenas passadas aqui
note-template-field-why-it-matters = Porque é importante
note-template-field-appearance = Aparência
note-template-field-age-and-origin = Idade e origem
note-template-field-what-it-does = O que faz
note-template-field-who-holds-it = Quem o tem
note-template-field-who-wants-it = Quem o quer
note-template-field-pov = Ponto de vista
note-template-field-time-and-place = Tempo e lugar
note-template-field-goal = Objetivo
note-template-field-conflict = Conflito
note-template-field-turn = A viragem
note-template-field-exit-emotion = Emoção à saída
note-template-field-purpose = Propósito
note-template-field-who-leads-it = Quem a lidera
note-template-field-resources = Recursos
note-template-field-allies-and-enemies = Aliados e inimigos
note-template-field-what-it-wants-now = O que quer agora
note-template-field-where-from = De onde veio
note-template-field-page-or-link = Página ou ligação
note-template-field-key-facts = Factos essenciais
note-template-field-how-it-is-used = Como é usado na história
