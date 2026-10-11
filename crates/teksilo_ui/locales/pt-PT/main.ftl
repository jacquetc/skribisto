# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Projeto
menu-new-work = N&ovo projeto
menu-open-work = &Abrir projeto…
menu-new-window = &Nova janela
menu-import-from = &Importar de
menu-create-from = &Criar a partir de
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documentos (Markdown, Word, ODT)…
menu-import-manuskript = &Manuskript (.msk)…
menu-import-scrivener = &Scrivener (.scriv)…
menu-export = &Exportar
menu-export-book = Exportar livro
menu-export-part = Exportar parte
menu-export-chapter = Exportar capítulo
menu-export-scene = Exportar cena
menu-export-note = Exportar nota
menu-export-paratext = Exportar paratexto
menu-export-folder = Exportar pasta
menu-export-choose = Escolher…
menu-export-none = Abrir um documento para exportar
menu-save = &Guardar
menu-save-as-file = Guardar co&mo ficheiro único…
menu-save-as-folder = G&uardar como pasta…
menu-backup = C&riar cópia de segurança agora
menu-close-work = &Fechar projeto
menu-welcome = &Boas-vindas…
menu-settings = &Definições
menu-quit = &Sair

menu-view = &Ver
menu-outline = &Estrutura
menu-search = &Pesquisar no projeto
menu-timeline = &Voltar atrás no tempo
menu-search-preview = Pr&é-visualização da pesquisa
menu-fullscreen = E&crã inteiro
menu-focus-mode = M&odo sem distrações
menu-format = &Formato
menu-scene-break = Inserir &quebra de cena
menu-major-scene-break = Inserir quebra &de cena principal

menu-go = &Ir
menu-go-next-scene = &Cena seguinte
menu-go-prev-scene = C&ena anterior
menu-go-next-chapter = Ca&pítulo seguinte
menu-go-prev-chapter = Capítu&lo anterior
menu-go-next-note = &Nota seguinte
menu-go-prev-note = N&ota anterior

menu-tools = Fe&rramentas
menu-help = &Ajuda

ctx-add = &Adicionar
ctx-new-item = &Novo item
ctx-new-folder = &Nova pasta
ctx-rename = &Mudar o nome
ctx-duplicate = &Duplicar
ctx-import-here = &Importar aqui…
ctx-indent = &Aumentar avanço
ctx-outdent = Dimin&uir avanço
ctx-trash = M&over para o lixo
ctx-open-to-side = A&brir ao lado
ctx-open = &Abrir
ctx-reveal-in-outline = &Revelar na estrutura
ctx-move-up = &Subir
ctx-move-down = &Descer

ctx-tab-close = &Fechar
ctx-tab-close-others = F&echar os outros
ctx-tab-close-all = Fe&char todos
ctx-tab-open-to-side = &Abrir ao lado
ctx-tab-open-in-main = &Abrir no painel principal
ctx-tab-move-to-side = &Mover para o lado
ctx-tab-move-to-main = &Mover para o painel principal
ctx-tab-move-to-new-window = M&over para uma nova janela
ctx-tab-move-window-unsaved = Guarde primeiro este projeto: uma segunda janela abre um ficheiro no disco
ctx-tab-pin = Fi&xar este separador
ctx-tab-unpin = &Desafixar este separador
tab-pinned-tooltip = Fixado: «Fechar os outros» e «Fechar todos» deixam este separador aberto

create-book = Livro
create-part = Parte
create-chapter = Capítulo
create-scene = Cena
create-note = Nota
create-note-folder = Pasta de notas
create-folder = Pasta
create-paratext = Paratexto
create-paratext-folder = Pasta de paratextos
create-book-end = Fim do livro
create-story-bible-entry = Entrada da bíblia da história…
type-book-start = Início do livro
type-text = Texto

new-item-book = Novo livro
new-item-part = Nova parte
new-item-chapter = Novo capítulo
new-item-note = Nova nota
new-item-note-folder = Nova pasta de notas
new-item-folder = Nova pasta
new-item-paratext = Novo paratexto
new-item-paratext-folder = Nova pasta de paratextos
new-item-story-bible-entry = Nova entrada da bíblia da história

placement-inside = dentro
placement-after = depois
placement-after-parent = depois do pai
placement-top-level = nível superior


ctx-promote = &Converter em
promote-chapter-folder = Pasta de capítulo
promote-flat-chapter = Capítulo simples
promote-lossy-title = Não há onde guardar o texto
promote-lossy-text = O tipo { $target } não tem onde guardar o seguinte: { $kinds }. Mova ou apague esse texto primeiro e depois converta.
content-scene-text = Texto da cena
content-note-text = Texto da nota
content-book-title = Título do livro
content-book-subtitle = Subtítulo do livro
content-part-title = Título da parte
content-chapter-title = Título do capítulo
content-epigraph-text = Epígrafe
content-paratext-text = Paratexto
promote-blocked-title = O capítulo não está vazio
promote-blocked-text = { $count ->
    [one] Este capítulo ainda contém { $count } item. Mova-o ou envie-o para o lixo antes de o converter num capítulo simples.
   *[other] Este capítulo ainda contém { $count } itens. Mova-os ou envie-os para o lixo antes de o converter num capítulo simples.
}

inspector = Inspetor
inspector-empty = Abra um item para o inspecionar.
inspector-promote = Converter em…
statusbar-toggle-outline = Mostrar ou ocultar o arquivador
statusbar-toggle-inspector = Mostrar ou ocultar o inspetor
statusbar-save-unsaved = Alterações por guardar. Clique para guardar
statusbar-save-saved = Todas as alterações guardadas
statusbar-save-autosave = A gravação automática está ativa. As alterações são guardadas à medida que escreve
statusbar-saving = A guardar…
statusbar-word-count = { $count ->
    [one] { $count } palavra
   *[other] { $count } palavras
}
statusbar-word-count-tooltip = Palavras na cena que está a editar
statusbar-word-char-count = { $words ->
    [one] { $words } palavra
   *[other] { $words } palavras
} · { $chars ->
    [one] { $chars } carácter
   *[other] { $chars } caracteres
}
session-toggle = Sessão de escrita: iniciar ou pausar um sprint de concentração
session-configure = Definir o objetivo de palavras e o limite de tempo da sessão
session-configure-title = Sessão de escrita
session-word-goal = Objetivo de palavras
session-time-limit = Limite de tempo
session-time-limit-unit = min
session-no-goal = Sem objetivo
session-no-limit = Sem limite
session-reset = Repor a sessão
session-readout = { $words ->
    [one] { $words } palavra
   *[other] { $words } palavras
} · { $time }
session-readout-timed = { $words ->
    [one] { $words } palavra
   *[other] { $words } palavras
} · faltam { $time }
statusbar-focus-exit = Sair do modo sem distrações
statusbar-focus-synopsis = Sinopse
statusbar-focus-go-prev = Item anterior (Alt+Cima)
statusbar-focus-go-next = Item seguinte (Alt+Baixo)

rail-settings = Definições

language = Idioma
theme = Tema
english = Inglês
french = Francês
light = Claro
dark = Escuro
settings-text-width = Largura do texto
settings-preview-width = Largura da pré-visualização da pesquisa
settings-autosave = Guardar automaticamente no disco
settings-show-welcome = Mostrar a janela inicial no arranque
settings-show-welcome-tip = Quando desativado, o último projeto é reaberto.
settings-check-for-updates = Procurar novas versões
settings-check-for-updates-tip = No máximo uma vez por dia, o Skribisto pergunta ao site do projeto qual é a versão atual. O pedido não leva nenhum identificador, nem sequer a versão que está a utilizar: a comparação é feita neste computador.

settings-title = Definições
settings-close = Fechar
settings-search = Pesquisar nas definições
settings-reset = Repor predefinições
settings-done = Concluído
settings-cancel = Cancelar
settings-apply = Aplicar
settings-ok = OK
settings-reset-confirm-title = Repor todas as definições predefinidas?
settings-reset-confirm-body = Repõe as definições de aparência, de editor e de escrita desta aplicação. As definições próprias do projeto, os atalhos de teclado e os estilos guardados não são afetados. Esta ação não pode ser anulada.
settings-empty-title = Ainda não há definições aqui
settings-empty-hint = Esta secção terá opções numa atualização futura.

settings-sec-appearance-behaviour = Aparência e comportamento
settings-sec-editor = Editor
settings-sec-spelling = Ortografia
settings-sec-backup = Cópias de segurança e sincronização
settings-sec-compile = Compilação e exportação
settings-page-appearance = Aparência
settings-field-image-size-policy = Imagens grandes
settings-image-policy-ask = Perguntar de cada vez
settings-image-policy-keep = Manter original
settings-image-policy-downscale = Otimizar
settings-hint-image-size-policy = O que fazer quando uma imagem inserida é maior do que o manuscrito precisa.
settings-page-notifications = Notificações
settings-page-scene = Cena
settings-page-synopsis = Sinopse
settings-page-notes = Notas
settings-page-editor-behavior = Comportamento do editor
settings-page-goals = Objetivos e contagem de palavras
settings-page-games = Jogos de escrita
settings-page-corkboard = Quadro de cortiça
settings-page-distraction-free = Sem distrações
settings-page-dictionaries = Dicionários
settings-page-autosave = Gravação automática
settings-page-export = Formatos de exportação
settings-page-paratext = Estruturas de paratexto
settings-page-keymap = Atalhos de teclado
settings-keymap-filter = Filtrar atalhos

settings-desc-sec-appearance-behaviour = O aspeto da própria aplicação e o que ela faz ao arrancar.
settings-desc-sec-editor = A superfície de escrita: o seu aspeto e o que faz enquanto escreve.
settings-desc-sec-spelling = A verificação ortográfica e os dicionários que a suportam.
settings-desc-sec-backup = Como o seu trabalho chega ao disco e que cópias são mantidas.
settings-desc-sec-compile = O que sai do Skribisto, e em que forma.
settings-desc-sec-work = Definições que pertencem a este projeto e viajam dentro do seu ficheiro.
settings-desc-sec-extensions = Páginas fornecidas pelas extensões aqui instaladas.
settings-desc-group-typography = Uma página por tipo de texto: tipo de letra, tamanho, altura da linha e espaçamento.
settings-desc-appearance = Idioma da interface, tema, tamanho do texto e a janela inicial no arranque.
settings-desc-notifications = Todas as mensagens mostradas nesta sessão e as ações que pode repetir.
settings-desc-scene = Como é composta a prosa das cenas: tipo de letra, tamanho, altura da linha, avanços e espaçamento.
settings-desc-synopsis = Como é composto o painel da sinopse, independentemente do manuscrito.
settings-desc-notes = Como são compostas as notas, independentemente do manuscrito.
settings-desc-corkboard = Tamanho dos cartões, o que cada cartão mostra e como o quadro os organiza.
settings-desc-distraction-free = Tipografia, largura da coluna e barra de controlo para a escrita em ecrã inteiro.
settings-desc-distraction-free-themes = A biblioteca de temas para a escrita em ecrã inteiro: os incluídos e os seus.
settings-desc-editor-behavior = Largura do texto, posição da sinopse, deslocamento tipo máquina de escrever e realce do cursor.
settings-desc-punctuation = O estilo tipográfico da casa com que os novos projetos começam. Todos os projetos o seguem, salvo se definirem o seu.
settings-desc-goals = Metas de palavras e de caracteres, e como as palavras são contadas.
settings-desc-games = Restrições que impõe a si mesmo ao escrever o rascunho, como Sempre em frente.
settings-desc-spellcheck = O interruptor único que liga e desliga a verificação ortográfica em todo o lado.
settings-desc-dictionaries = Instalar, remover e consultar os dicionários ortográficos deste computador.
settings-desc-autosave = Se as suas edições são escritas no disco automaticamente.
settings-desc-backup = Quando são feitas as cópias, onde são guardadas e quantas. Todos os projetos seguem isto, salvo se definirem a sua própria política.
settings-desc-export = Os estilos pelos quais passam todas as exportações: os incluídos e os seus.
settings-desc-paratext = Os elementos pré-textuais e pós-textuais com que um novo projeto pode começar.
settings-desc-keymap = Todos os atalhos e a ação a que cada um está associado.
settings-page-user = Utilizador
settings-desc-user = Quem é, para os comentários que escreve.
settings-group-identity = Identidade
settings-field-user-name = O seu nome
settings-field-user-name-placeholder = Opcional
settings-field-user-name-hint = Assina os comentários e as respostas que escreve. Distinto do nome de autor de um projeto, que é a assinatura do livro e viaja dentro do ficheiro.
settings-field-user-initials = As suas iniciais
settings-field-user-initials-placeholder = A partir do seu nome
settings-field-user-initials-hint = O que um processador de texto mostra junto ao seu comentário na margem. Deixe em branco para usar as apresentadas, retiradas do seu nome.
settings-field-user-hint = Ambos são opcionais e aplicam-se a todos os projetos neste computador. Alterá-los muda a assinatura dos comentários que escrever a seguir. Os comentários já escritos mantêm o nome com que foram escritos.
settings-desc-author = O nome que figura neste projeto.
settings-desc-structure = Se os capítulos deste projeto são pastas ou itens individuais.
settings-desc-language = O idioma em que a prosa deste projeto é verificada.
settings-desc-work-backup = A política de cópias de segurança própria deste projeto, ou a geral.
settings-desc-personal-dictionary = As palavras que este projeto considera corretamente escritas.
settings-desc-tags = As etiquetas coloridas com que este projeto marca os seus itens.
settings-desc-templates = Os modelos de notas que pode inserir ao escrever neste projeto.
settings-desc-text-replacements = Atalhos que se expandem à medida que escreve neste projeto.
settings-desc-work-punctuation = As aspas, os travessões e os espaçamentos que a prosa deste projeto segue.

settings-group-typography = Tipografia
settings-group-writing-column = Coluna de escrita
settings-group-writing-view = Vista de escrita
settings-group-distraction-free-strip = Barra de controlo
settings-group-theme = Tema
settings-group-language = Idioma
settings-group-startup = Arranque
settings-group-updates = Atualizações
settings-group-autosave = Gravação automática
settings-field-typeface = Tipo de letra
settings-field-size = Tamanho do texto

editor-size-changed-manuscript = Tamanho do texto do manuscrito: { $percent }
editor-size-changed-synopsis = Tamanho do texto da sinopse: { $percent }
editor-size-changed-notes = Tamanho do texto das notas: { $percent }
editor-size-changed-corkboard = Tamanho do texto dos cartões do quadro de cortiça: { $percent }
editor-size-changed-corkboard-expanded = Tamanho do texto do editor expandido: { $percent }
editor-size-changed-distraction-free = Tamanho do texto sem distrações: { $percent }

menu-text-size-increase = &Aumentar o tamanho do texto
menu-text-size-decrease = &Diminuir o tamanho do texto
menu-text-size-reset = &Repor o tamanho do texto
settings-field-line-height = Altura da linha
settings-field-first-line-indent = Avanço da primeira linha
settings-field-paragraph-spacing-before = Espaço antes do parágrafo
settings-field-paragraph-spacing-after = Espaço após o parágrafo
settings-field-column-width = Largura da coluna
settings-distraction-free-width-hint = Aplica-se apenas com o modo sem distrações ativo. As larguras das colunas de Cena, Sinopse e Notas não são alteradas.
settings-distraction-free-title = Manter o nome do item
settings-distraction-free-word-count = Manter a contagem de palavras
settings-distraction-free-session = Manter a sessão de escrita
settings-distraction-free-go-to = Manter o botão Ir para…
settings-distraction-free-go = Manter os botões Anterior e Seguinte
settings-distraction-free-chrome-hint = O botão Sair fica sempre, seja qual for a sua escolha aqui. É a sua saída se a tecla Esc estiver ocupada.
settings-field-app-theme = Tema
settings-theme-light = Claro
settings-theme-dark = Escuro
settings-field-text-scale = Tamanho do texto da interface
settings-field-language = Idioma da interface
settings-synopsis-placement = Posição da sinopse
settings-synopsis-placement-none = Nenhuma
settings-synopsis-placement-top = Acima
settings-synopsis-placement-side = Ao lado
settings-typewriter = Deslocamento tipo máquina de escrever
settings-typewriter-tip = Mantém a linha que está a escrever a uma altura fixa enquanto o manuscrito se desloca por baixo dela. Clicar continua a colocar o cursor onde clica.
settings-typewriter-position = Posição da linha
settings-typewriter-position-top-third = Terço superior
settings-typewriter-position-middle = Meio
settings-typewriter-position-bottom-quarter = Quarto inferior
settings-highlight-scope = Realce em torno do cursor
settings-highlight-scope-none = Nenhum
settings-highlight-scope-sentence = Frase
settings-highlight-scope-paragraph = Parágrafo
settings-highlight-scope-tip-none = Deixar a página simples. Nada é sombreado enquanto escreve.
settings-highlight-scope-tip-sentence = Sombrear a frase que está a escrever, para que se destaque das que a rodeiam.
settings-highlight-scope-tip-paragraph = Sombrear todo o parágrafo que está a escrever, para manter à vista a passagem em que está a trabalhar.
settings-group-container-views = Vistas dos contentores
settings-remember-view = Recordar a última vista para cada tipo de item
settings-remember-view-tip = Abrir um contentor na vista que usou pela última vez para esse tipo
settings-remember-view-tip-more =
    Um livro, uma parte e um capítulo oferecem várias vistas cada (a sua própria página,
    o manuscrito completo, a sinopse completa). Ative isto e cada tipo reabre na
    vista que escolheu da última vez para ele: por exemplo, mude um capítulo para Capítulo completo
    e o próximo capítulo que abrir mostra também Capítulo completo. Cada tipo recorda a sua própria vista.
settings-group-counting = Contagem de palavras
settings-counting-auto = Automático (por idioma)
settings-counting-whitespace = Dividir nos espaços
settings-counting-unicode-words = Palavras Unicode
settings-counting-cjk-hybrid = Adaptado a CJK (por carácter)
settings-counting-hint = O modo automático conta o chinês e o japonês por carácter e todas as outras línguas por palavra. Altere-o apenas se a contagem parecer errada para a sua língua.
settings-group-goals-display = Visualização
settings-show-characters = Mostrar o número de caracteres na barra de estado
settings-autosave-hint = As alterações são escritas no disco automaticamente à medida que escreve.

settings-sec-work = Projeto
settings-page-structure = Estrutura
settings-page-author = Autor
settings-field-author-name = Nome do autor
settings-field-author-placeholder = Opcional
settings-field-author-hint = Aparece na página de título compilada e nos metadados dos ficheiros exportados. Deixe em branco para o omitir. É a assinatura do livro. O nome com que os seus comentários são assinados está em Definições ▸ Utilizador.
settings-group-chapters = Capítulos
tidy-titles-title = Arrumar os títulos dos capítulos
tidy-titles-none = Nenhum título de capítulo ou de parte se limita a repetir o seu próprio número.
tidy-titles-lead = { $count ->
        [one] Um capítulo ou parte tem como título apenas o seu próprio número.
       *[other] { $count } capítulos e partes têm como título apenas o seu próprio número.
    }
tidy-titles-explain = Limpar esses títulos deixa cada um designado pelo número que o livro já conhece, o mesmo que a exportação imprime. Nada mais muda, e uma única anulação repõe-nos todos.
menu-document-tidy-titles = Arrumar os &títulos dos capítulos…
settings-group-numbering = Numeração
settings-number-chapters = Numerar capítulos e partes
settings-number-chapters-tip = Os capítulos e as partes têm um número calculado a partir da sua posição no livro, mostrado aqui junto ao título e impresso pela exportação.
settings-number-chapters-tip-more = O número nunca é guardado no título, por isso continua correto quando reordena, insere ou elimina. Desative isto e a exportação imprime apenas os títulos, seja o que for que o estilo de exportação peça. Para deixar um capítulo de fora (um prólogo, um interlúdio), use antes o interruptor Numeração do Inspetor: mantém-no no livro, mas impede-o de receber um número.
settings-part-resets-chapter = Reiniciar os números dos capítulos em cada parte
settings-part-resets-chapter-tip = Desativado por predefinição: a numeração dos capítulos continua ao longo das partes de um livro, por isso a «Parte dois» abre no Capítulo onze.
settings-part-resets-chapter-tip-more = É a prática habitual na edição e o que um leitor espera. Ative-o para um livro cujas partes se leem como volumes separados, em que cada uma abre no Capítulo um.
settings-chapter-flat = Capítulos simples
settings-chapter-flat-hint = Ativado: um capítulo é um item único. Escreve nele e não contém cenas. Desativado: um capítulo é uma pasta. Continua a escrever nele, mas também pode conter cenas. Os novos capítulos seguem esta definição; os existentes convertem-se com Converter em.

settings-styles-builtin = Estilos integrados
settings-styles-user = Os meus estilos
settings-styles-builtin-badge = Integrado
settings-styles-duplicate = Duplicar
settings-styles-edit = Editar
settings-styles-delete = Eliminar
settings-styles-import = Importar…
settings-styles-export = Exportar…
settings-styles-copy-suffix = (cópia)
settings-styles-json-filter = Estilo de exportação
settings-styles-editor-title = Editar o estilo
settings-styles-editor-none = Selecione um estilo personalizado para o editar ou duplique um estilo integrado.
settings-styles-imported = Estilo importado
settings-styles-import-failed = Não foi possível importar o estilo
settings-styles-exported = Estilo exportado
settings-styles-export-failed = Não foi possível exportar o estilo
settings-styles-field-name = Nome
settings-styles-field-chapters = Cabeçalhos dos capítulos
settings-styles-field-parts = Cabeçalhos das partes
settings-styles-field-scene-break = Quebra de cena
settings-styles-field-major-scene-break = Quebra de cena principal
settings-styles-field-spacing = Espaçamento entre linhas
settings-styles-field-justify = Justificar texto
settings-styles-field-notes = Incluir notas
settings-styles-field-synopses = Incluir as sinopses
settings-styles-field-scene-titles = Incluir os títulos das cenas
settings-styles-field-epigraphs = Incluir epígrafes
settings-styles-field-paratexts = Incluir paratextos
settings-styles-field-epigraph-placement = Posição da epígrafe
settings-styles-epigraph-after = Depois do título
settings-styles-epigraph-before = Antes do título
settings-styles-field-footnotes = Incluir notas de rodapé
settings-styles-field-footnote-numbering = Numeração das notas de rodapé
settings-styles-footnote-numbering-continuous = Contínua
settings-styles-footnote-numbering-per-chapter = Recomeçar em cada capítulo
settings-styles-footnote-numbering-per-book = Recomeçar em cada livro
settings-styles-field-images = Imagens
menu-image = I&magem
settings-styles-images-beside = Ao lado do documento
settings-styles-images-embed = Dentro do documento
settings-styles-images-omit = Omitir
settings-styles-group-round-trip = Envio a um revisor
settings-styles-field-comments = Incluir comentários
settings-styles-field-round-trip-marks = Incluir marcadores de ida e volta
settings-styles-round-trip-hint = Apenas DOCX e ODT. Os marcadores são identificadores invisíveis que permitem que um ficheiro devolvido atualize este projeto, em vez de ficar ao lado dele como uma segunda cópia.
settings-styles-group-pages = Páginas
settings-styles-field-cover = Abrir com a capa
settings-styles-field-word-count = Contagem de palavras na página de título
settings-styles-field-page-books = Nova página em cada livro
settings-styles-field-page-parts = Nova página em cada parte
settings-styles-field-page-chapters = Nova página em cada capítulo
settings-styles-field-page-paratexts = Nova página em cada paratexto
settings-styles-heading-none = Sem cabeçalho
settings-styles-heading-numbered = Apenas o número
settings-styles-heading-title = Apenas o título
settings-styles-heading-both = Número + título
settings-styles-break-blank = Linha em branco
settings-styles-break-none = Nenhuma
settings-styles-spacing-single = Simples
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Duplo

welcome-title = Bem-vindo ao Skribisto
welcome-version = Versão { $version }
welcome-update-available = A versão { $version } está disponível
welcome-update-tip = Abre a página de transferência no seu navegador.
welcome-search = Pesquisar projetos
welcome-open = Abrir
welcome-new-work = Novo projeto
welcome-create-from = Criar a partir de…
welcome-recent-works = Projetos recentes
welcome-empty-recents = Ainda não há projetos recentes.
welcome-no-matches = Nenhum projeto recente corresponde à sua pesquisa.
welcome-learn-soon = Guias e dicas em breve.
welcome-about-blurb = Skribisto, uma reescrita em Rust + Teksilo da aplicação de escrita.
welcome-tagline = *Um lugar tranquilo para escrever coisas longas.*
welcome-github = GitHub
welcome-discord = Discord
nav-works = Projetos
nav-examples = Exemplos
nav-learn = Aprender
nav-about = Acerca

synopsis = Sinopse
epigraph = Epígrafe
pane-manuscript = Manuscrito
corkboard = Quadro de cortiça
overview = Vista geral

corkboard-card-count = { $count ->
    [one] { $count } cartão
   *[other] { $count } cartões
}
corkboard-child-count = { $count ->
    [one] { $count } item
   *[other] { $count } itens
}
corkboard-view-nested = Aninhado
corkboard-view-flat = Plano
corkboard-layout-hint = Aninhado mostra os filhos diretos de um contentor. Abra um cartão de pasta para entrar nela. Plano mostra todas as cenas de todo o contentor ao mesmo tempo.
corkboard-card-size = Tamanho dos cartões
corkboard-search-placeholder = Filtrar cartões…
corkboard-empty-title = Ainda não há nada aqui
corkboard-empty-hint = Use «＋ Novo» acima para adicionar o primeiro.
corkboard-new = Novo
corkboard-show-card-numbers = Numerar os cartões
corkboard-modal-size = Tamanho do editor expandido
corkboard-scope-hint = Aplica-se a todos os quadros de cortiça abertos, em todos os projetos.
corkboard-sort-manuscript = Ordem do manuscrito
corkboard-sort-title-asc = Título A–Z
corkboard-sort-title-desc = Título Z–A
corkboard-card-number = Cartão { $number }

duplicate = Duplicar
duplicate-n = { $count ->
    [one] Duplicar { $count } cartão
   *[other] Duplicar { $count } cartões
}
set-label-n = { $count ->
    [one] Definir o rótulo de { $count } cartão
   *[other] Definir o rótulo de { $count } cartões
}
move-to-trash-n = { $count ->
    [one] Mover { $count } cartão para o lixo
   *[other] Mover { $count } cartões para o lixo
}
reveal-in-outline = Revelar na estrutura

corkboard-move-to = Mover para…
corkboard-move-to-n = { $count ->
    [one] Mover { $count } cartão para…
   *[other] Mover { $count } cartões para…
}
corkboard-move-picker-title = Mover para…
corkboard-move-picker-empty = Ainda não há arquivadores. Crie um primeiro.
corkboard-move-picker-cancel = Cancelar
corkboard-move-here = Mover para aqui
corkboard-moved-ok = { $count ->
    [one] { $count } cartão movido
   *[other] { $count } cartões movidos
}
corkboard-move-into-self = Um contentor não pode ser movido para dentro de si próprio. Escolha um destino fora dele.
corkboard-move-failed = Não foi possível mover esses cartões para lá.

overview-col-title = Título
overview-col-type = Tipo
overview-col-label = Rótulo
overview-col-tags = Etiquetas
overview-col-books = Livros
overview-col-own-words = Palavras
overview-col-total-words = Total
overview-row-count = { $count ->
    [one] { $count } item
   *[other] { $count } itens
}
overview-search-placeholder = Filtrar itens…
overview-expand-all = Expandir tudo
overview-collapse-all = Recolher tudo
overview-table-label = Conteúdo
overview-empty-title = Ainda não há nada aqui
overview-gone-title = Este contentor desapareceu
overview-gone-hint = Foi movido para o lixo. Restaure-o ou feche este separador.
overview-empty-hint = Use «＋ Novo» acima para adicionar o primeiro.
corkboard-grid-label = Cartões do quadro de cortiça
corkboard-rename-field = Mudar o nome do item
corkboard-expand-synopsis = Expandir a sinopse
corkboard-synopsis-modal-title = Sinopse
corkboard-badge-scene = Cena
corkboard-badge-chapter = Capítulo
corkboard-badge-part = Parte
corkboard-badge-book = Livro
corkboard-badge-note = Nota
corkboard-badge-folder = Pasta
corkboard-badge-paratext = Paratexto
corkboard-badge-text = Texto
corkboard-badge-end = Fim
settings-group-corkboard-layout = Disposição
settings-group-corkboard-cards = Cartões
corkboard-show-word-count = Mostrar a contagem de palavras nos cartões
text-heading = Texto
no-content = Este item não tem conteúdo editável.
untitled = Sem título
placeholder-title = Título…
placeholder-subtitle = Subtítulo…
placeholder-chapter-title = Título do capítulo…
split-editor = Dividir editor
close-split-view = Fechar a vista dividida
drop-open-here = Abrir aqui
drop-open-to-side = Abrir ao lado

segment-chapter = Capítulo
segment-part = Parte
segment-book = Livro
segment-pace = Ritmo
segment-notes = Notas
segment-story-bible = Bíblia da história
segment-note-own = Nota
segment-note-details = Detalhes
segment-note-in-prose = Em prosa
pace-placeholder = O planeador de ritmo aparece aqui.
pace-empty-title = Planear o ritmo deste livro
pace-empty-body = Defina um objetivo de palavras e um prazo, e o Skribisto calcula o ritmo diário para lá chegar.
pace-start-planning = Iniciar o planeamento
pace-section-schedule = Calendário
pace-section-progress = Progresso
pace-advancement = Avanço
pace-goal = Objetivo de palavras
pace-deadline = Prazo
pace-active = Ritmo ativo
pace-day-mon = seg
pace-day-tue = ter
pace-day-wed = qua
pace-day-thu = qui
pace-day-fri = sex
pace-day-sat = sáb
pace-day-sun = dom
pace-card-written = palavras escritas
pace-card-of-goal = do objetivo
pace-card-rate = palavras / dia de escrita
pace-card-days-left = dias de escrita restantes
pace-card-streak = dias seguidos
pace-card-ahead = palavras à frente do ritmo
pace-card-behind = palavras atrás do ritmo
pace-charts-empty = Os gráficos de progresso aparecem aqui assim que a sua contagem de palavras for registada numa gravação.
pace-chart-progression = Palavras escritas vs. meta
pace-chart-words-per-day = Palavras por dia
pace-series-actual = Real
pace-series-target = Meta
pace-series-words-per-day = Palavras/dia
pace-daily-target-line = Ritmo constante: { $count ->
    [one] { $count } palavra/dia
   *[other] { $count } palavras/dia
}
pace-section-holidays = Feriados
pace-section-milestones = Marcos
pace-holidays-none = Sem feriados. Todos os dias agendados contam.
pace-holiday-label = Nome do feriado
pace-add-holiday = Adicionar
pace-remove = Remover
pace-milestones-none = Ainda não há marcos. Defina um numa parte ou num capítulo no Inspetor.
full-chapter = Capítulo completo
full-part = Parte completa
full-book = Livro completo
full-synopsis = Sinopse completa
rename = Mudar o nome
set-label = Definir o rótulo
insert-scene = Inserir cena
insert-chapter = Inserir capítulo
split-scene = Dividir a cena
move-up = Subir
move-down = Descer
merge-with-previous = Juntar com o anterior
move-to-trash = Mover para o lixo
rename-chapter = Mudar o nome do capítulo
add-scene = Adicionar uma nova cena
add-chapter = Adicionar um novo capítulo
new-scene-title = Nova cena
placeholder-scene-name = Nome da cena…
menu-cut = Cortar
menu-copy = Copiar
menu-paste = Colar
menu-paste-unformatted = Colar sem formatação
menu-select-all = Selecionar tudo
format-bold = Negrito
format-italic = Itálico
format-underline = Sublinhado
format-strikethrough = Rasurado

binder = Arquivador
no-work = Nenhum projeto carregado
no-recent-works = Nenhum projeto recente
switcher-open-section = Aberto atualmente
switcher-recent-section = Recente
switcher-this-window = esta janela
open-project-title = Abrir projeto
open-project-question = Como pretende abrir «{ $title }»?
open-in-new-window = Abrir numa nova janela
open-here = Abrir aqui

binder-all = Todos os arquivadores
binder-show-all = Mostrar todos os arquivadores
binder-new = Novo arquivador…
binder-default-name = Novo arquivador
binder-item-count = { $count ->
    [one] { $count } item
   *[other] { $count } itens
}
binder-search-placeholder = Filtrar a estrutura…
binder-search-scope = Pesquisar em todos os arquivadores
binder-trash-confirm-title = Mover o arquivador para o lixo?
binder-trash-confirm-text = «{ $name }» e todos os seus itens serão movidos para o lixo.

dialog-rename = Mudar o nome
dialog-new-binder = Novo arquivador
dialog-set-label = Definir o rótulo
dialog-new-scene = Nova cena
close-work-question = Guardar as alterações antes de fechar o projeto?
quit-question = Guardar as alterações antes de sair?
quit-save-work-question = Guardar as alterações a { $title } antes de sair?
unsaved-changes = Este projeto tem alterações por guardar.
new-work-unsaved-question = Guardar as alterações antes de criar um novo projeto?
open-work-unsaved-question = Guardar as alterações antes de abrir outro projeto?
switch-backup-discard-title = Descartar alterações nesta cópia de segurança?
switch-backup-discard-text = As alterações a uma cópia de segurança não podem ser guardadas nela. Use Guardar como ou Restaurar para as manter, ou descarte-as e abra o outro projeto.
switch-save-failed = Não foi possível guardar o projeto, por isso não foi substituído: { $error }
switch-save-not-started = Não foi possível guardar o projeto, por isso não foi substituído.
close-save-failed = Não foi possível guardar o projeto, por isso não foi fechado: { $error }
close-save-not-started = Não foi possível guardar o projeto, por isso não foi fechado.
save-not-started = Não foi possível guardar o projeto.

could-not-open-work = Não foi possível abrir «{ $file }»
could-not-open-work-too-new = «{ $file }» precisa de uma versão mais recente do Skribisto
could-not-open-work-too-new-detail = Guardado no formato Skribisto { $written_by }; para o abrir é preciso o formato { $requires } ou mais recente, e esta compilação suporta até ao formato { $supported }. Atualize o Skribisto para o abrir.
could-not-open-work-importing = «{ $file }» está a ser escrito por uma importação
could-not-open-work-importing-detail = Uma importação está a escrever sobre este projeto e ainda não terminou. Abra-o quando a importação estiver concluída: se o abrir agora, mostraria o projeto que a importação está a substituir, e a próxima gravação escreveria esse projeto de volta por cima da importação.
could-not-open-example = Não foi possível abrir o exemplo: { $error }
could-not-open-link = Não foi possível abrir { $url }: { $error }
could-not-create-work = Não foi possível criar o projeto: { $error }
saving-as-file = A guardar como { $target }…
saving-as-folder = A guardar como { $target }/…
saved-as = Guardado em { $target }
save-error = Não foi possível guardar: { $error }
save-flush-failed = A gravação foi feita, mas estes documentos não lhe puderam ser entregues e continuam por guardar: { $titles } ({ $error })
backup-error = Não foi possível criar a cópia de segurança: { $error }
backing-up = A criar cópia de segurança…
backup-nothing-open = Nenhum projeto aberto para copiar.
backup-already-running = Já está em curso uma cópia de segurança.
backup-complete = Cópia de segurança concluída ({ $ok } guardadas, { $skipped } já atualizadas)
backup-partial = Cópia de segurança terminada: { $ok } guardadas, { $failed ->
    [one] { $failed } destino falhou
   *[other] { $failed } destinos falharam
}
backup-no-destination-title = Nenhum local de cópia de segurança disponível
backup-no-destination-text = Não é possível aceder a nenhum dos destinos de cópia de segurança configurados (por exemplo, um disco externo pode estar desligado). Ligue-o e tente novamente, ou saia sem criar uma cópia de segurança.
backup-failed-close-title = Não foi possível guardar a cópia de segurança
backup-failed-close-text = Não foi possível escrever nenhuma cópia de segurança antes de fechar. Todos os destinos falharam (o disco pode ter sido removido, estar cheio ou protegido contra escrita). Corrija o problema e tente novamente, ou saia sem criar uma cópia de segurança.
backup-failed-close-error-text = Não foi possível escrever nenhuma cópia de segurança antes de fechar: { $error }. Corrija o problema e tente novamente, ou saia sem criar uma cópia de segurança.
backup-no-result = A cópia de segurança terminou sem indicar se fez uma cópia. Consulte a lista de cópias de segurança para ver se há uma nova.

backup-choice-title = Ficheiro de cópia de segurança
backup-choice-heading = Abriu uma cópia de segurança
backup-choice-subtitle = Esta é uma cópia de segurança de um projeto num determinado momento.
backup-choice-subtitle-dated = Cópia de segurança feita em { $date }.
backup-choice-body = Pode abri-la e editá-la livremente, mas as alterações só podem ser mantidas com Guardar como. O ficheiro original do projeto não é alterado. Ou então restaure este projeto exatamente para esta cópia de segurança.
backup-choice-open = Abrir a cópia de segurança
backup-choice-restore = Restaurar este projeto para este ponto…
backup-choice-not-a-backup = Não, abrir normalmente
backup-banner-title = Cópia de segurança: as alterações não podem ser guardadas aqui
backup-banner-description = Use Guardar como para manter as suas edições num novo ficheiro, ou Restaurar para substituir o projeto original por esta cópia de segurança.
backup-banner-restore = Restaurar…
backup-banner-save-as = Guardar como…
backup-restore-original-missing = Não foi possível encontrar o projeto original a restaurar. Use antes Guardar como para manter esta cópia de segurança como um novo projeto.
backup-restore-close-elsewhere-title = Projeto aberto noutra janela
backup-restore-close-elsewhere-text = O projeto que está a restaurar está aberto noutra janela. Feche-o primeiro nessa janela e tente novamente.
backup-restore-focus-window = Ir para essa janela
backup-restore-confirm-title = Restaurar esta cópia de segurança?
backup-restore-confirm-text = A versão atual do projeto será copiada à parte como cópia de segurança antes de ser substituída por esta.
backup-restore-confirm-ok = Restaurar
backup-restore-error = Não foi possível restaurar: { $error }
backup-restored-ok = Projeto restaurado.
backup-restored-with-safety = Projeto restaurado. A sua versão anterior foi guardada em { $path }.
close-backup-discard-title = Descartar alterações nesta cópia de segurança?
close-backup-discard-text = As alterações a uma cópia de segurança não podem ser guardadas nela. Use Guardar como para as manter, ou descarte-as e feche.
quit-backup-discard-title = Descartar as alterações e sair?
quit-backup-discard-work-question = Descartar as alterações a { $title } e sair?
quit-backup-discard-text = As alterações a uma cópia de segurança não podem ser guardadas nela. Use Guardar como para as manter, ou descarte-as e saia.
backup-nudge-text = Não há cópias de segurança configuradas para este projeto.
backup-nudge-action = Configurar cópias de segurança…

menu-backups-list = C&ópias de segurança…
backups-title = Cópias de segurança
backups-loading = A procurar cópias de segurança…
backups-empty = Ainda não foram encontradas cópias de segurança para este projeto.
backups-open = Abrir
backups-reveal = Mostrar localização
backups-delete = Eliminar esta cópia de segurança
backups-delete-confirm-title = Eliminar esta cópia de segurança?
backups-delete-confirm-text = «{ $name }» será eliminada permanentemente. Esta ação não pode ser anulada.
backups-delete-error = Não foi possível eliminar a cópia de segurança: { $error }
backups-refresh = Atualizar
backups-close = Fechar

settings-page-backup = Predefinições das cópias de segurança
settings-page-work-backup = Cópias de segurança
settings-backup-general-title = Definições predefinidas das cópias de segurança
settings-backup-work-title = Cópias de segurança para este projeto
settings-backup-inherit = Usar as definições gerais das cópias de segurança
settings-backup-inheriting = Este projeto usa as definições gerais das cópias de segurança.
settings-backup-none-hint = Não há cópias de segurança automáticas configuradas (todos os acionadores estão desativados).
settings-backup-last = Última cópia de segurança: { $date }
settings-backup-last-never = Ainda não há cópias de segurança.
settings-backup-open-list = Abrir a lista de cópias de segurança…
settings-backup-triggers = Quando criar cópias de segurança
settings-backup-on-close = Criar uma cópia de segurança ao fechar o projeto
settings-backup-on-open = Criar uma cópia de segurança ao abrir o projeto
settings-backup-interval = Criar uma cópia de segurança periodicamente, a cada
settings-backup-destinations = Destinos das cópias de segurança
settings-backup-default-location = Localização predefinida
settings-backup-usage-since = { $count ->
    [one] { $count } cópia de segurança · { $size } · mais antiga { $oldest }
   *[other] { $count } cópias de segurança · { $size } · mais antiga { $oldest }
}

settings-backup-usage = { $count ->
    [one] { $count } cópia de segurança · { $size }
   *[other] { $count } cópias de segurança · { $size }
}

settings-backup-usage-empty = Ainda não há nada guardado aqui
settings-backup-usage-measuring = A medir…
settings-backup-reveal-root = Mostrar pasta
settings-backup-dest-none = Nenhum destino. As cópias de segurança são guardadas junto ao projeto.
settings-backup-dest-remove = Remover
settings-backup-dest-add = Adicionar pasta…
settings-backup-dest-refresh = Atualizar
settings-backup-retention = Quantas manter
settings-backup-retention-tiered = Por níveis
settings-backup-retention-keep-n = Manter as últimas N
settings-backup-retention-tip = Como são eliminadas as cópias de segurança antigas.
settings-backup-retention-tip-more = Por níveis mantém uma cópia de segurança por hora durante um dia, uma por dia durante uma semana, uma por semana durante um mês e uma por mês depois disso, para que o histórico recente fique denso e o antigo se vá rarefazendo. «Manter as últimas N» mantém simplesmente as N cópias mais recentes. Em ambos os modos, as cópias mais recentes (o mínimo abaixo) são sempre mantidas.
settings-backup-gfs-hourly = De hora a hora (últimas 24 h)
settings-backup-gfs-daily = Diárias (última semana)
settings-backup-gfs-weekly = Semanais (último mês)
settings-backup-gfs-monthly = Mensais
settings-backup-keep-n = Número a manter
settings-backup-min-keep = Manter sempre pelo menos
settings-backup-dedup = Ignorar uma cópia de segurança quando nada mudou

new-work-title = Novo projeto
new-work-close = Fechar
new-work-name = Nome do projeto
new-work-name-placeholder = Sem título
new-work-author = Autor
new-work-author-placeholder = Opcional
new-work-format = Formato
new-work-single-file = Ficheiro único
new-work-bundle = Pasta de projeto
new-work-convert-later = Pode converter entre formatos mais tarde.
new-work-location = Local
new-work-will-create = Será criado
new-work-language = Idioma predefinido
new-work-language-hint = Aplicado aos novos textos e à verificação ortográfica. Cada texto pode ser mudado para outro idioma.
new-work-template = Modelo
new-work-template-none = Nenhum
new-work-template-empty-novel = Romance vazio
new-work-template-light-novel = Light novel
new-work-template-novel = Romance
new-work-template-novel-in-parts = Romance em partes
new-work-template-notebook = Caderno
new-work-cancel = Cancelar
new-work-create = Criar projeto
new-work-step-details = Detalhes
new-work-step-language = Idioma e estrutura
new-work-step-template = Modelo
new-work-back = Voltar
new-work-next = Seguinte
new-work-documents-title = Novo projeto a partir de documentos
new-work-step-import = Importar
new-work-create-and-import = Criar e importar…
new-work-documents-next-title = Os seus documentos vêm a seguir
new-work-documents-next-body = Criar o projeto abre sobre ele o assistente de importação, onde escolhe os documentos, revê a estrutura que o Skribisto lê neles e escolhe onde ela fica. Nada é escrito no projeto até que o confirme aí.
new-work-documents-no-template = Este projeto começa vazio de propósito: sem modelo, para que os documentos importados sejam a única coisa nele.
new-work-documents-chapter-scene-hint = Aplica-se aos capítulos que a importação cria. Pode alterá-lo mais tarde nas definições do projeto.
new-work-single-file-desc = Um arquivo .skrib (zip). Portátil e fácil de copiar para uma cópia de segurança.
new-work-bundle-desc = Uma pasta com todos os textos e recursos. Mais adequada ao controlo de versões.
new-work-template-none-count = arquivador vazio
new-work-template-empty-novel-count = 1 capítulo
new-work-template-light-novel-count = 15 capítulos
new-work-template-novel-count = 20 capítulos
new-work-template-novel-in-parts-count = 3 partes, 24 capítulos
new-work-template-notebook-count = notas livres
new-work-chapter-scene = Capítulos simples
new-work-chapter-scene-tip = Cada capítulo é um item único em que escreve diretamente, sem cenas por baixo. Deixe isto desativado para a disposição clássica, em que um capítulo é uma pasta: também escreve diretamente nela, mas pode conter cenas.
new-work-chapter-scene-tip-more = Escreve num capítulo em ambos os casos. A única diferença é se pode *conter* cenas. A árvore do arquivador do Skribisto serve apenas para organizar, por isso as duas disposições compilam o mesmo livro, pode misturá-las livremente, e Converter em converte um capítulo entre as duas sem perder uma palavra.
new-work-name-required = Introduza um nome para o projeto
new-work-name-invalid = Este nome não tem caracteres utilizáveis
new-work-location-required = Escolha uma localização
new-work-location-missing = Esta pasta não existe
new-work-location-not-folder = Este caminho não é uma pasta
new-work-location-readonly = Não é possível escrever nesta pasta
new-work-target-exists = Já existe um projeto com este nome nesta pasta. Ao criar o projeto, será pedida confirmação antes de o substituir
new-work-target-is-folder = Já existe uma pasta com este nome nesta localização. Escolha outro nome
new-work-overwrite-title = Substituir o projeto existente?
new-work-overwrite-text = «{ $name }» já existe nesta pasta. Substituí-lo pelo novo projeto?
new-work-target-open-title = Este projeto está aberto
new-work-target-open-text = «{ $name }» está aberto no Skribisto. Criar um novo projeto no seu lugar substituiria o projeto em que está a trabalhar, e a próxima gravação escreveria esse projeto de volta por cima do novo. Escolha outro nome ou pasta, ou feche primeiro o projeto.
target-importing-title = Uma importação está a escrever este projeto
target-importing-text = Uma importação ainda está a escrever «{ $name }». Aguarde até que termine, ou escolha outro nome.
backup-restore-importing-text = Uma importação ainda está a escrever «{ $name }», o projeto sobre o qual esta cópia de segurança seria restaurada. Restaure a cópia de segurança quando a importação terminar.

new-work-manuscript = Manuscrito
new-work-notes = Notas
new-work-research = Pesquisa
new-work-notebook = Caderno
new-work-scene = Cena
new-work-note = Nota
new-work-front-matter = Elementos pré-textuais
new-work-back-matter = Elementos pós-textuais
new-work-paratext = Estrutura do livro
new-work-paratext-none = Sem estrutura
new-work-paratext-hint = Os elementos pré-textuais e pós-textuais com que uma tradição abre e fecha um livro. Pode mover, mudar o nome ou eliminar qualquer um deles depois.
new-work-tags = Etiquetas
new-work-tags-none = Sem etiquetas
new-work-tags-hint = Uma paleta inicial para etiquetar personagens, lugares e o resto. Opcional, e cada etiqueta pode ser renomeada, recolorida ou eliminada depois.
new-work-note-templates = Modelos de notas
new-work-note-templates-none = Sem modelos
new-work-note-templates-hint = A forma com que começa uma nota da bíblia da história. Opcional, e cada modelo pode ser editado ou eliminado depois.
new-work-characters = Personagens
new-work-places = Lugares

import-plume-title = Importar um projeto Plume Creator
import-plume-source = Projeto Plume
import-plume-source-hint = Escolha um ficheiro .plume ou .plume_backup (qualquer versão do Plume Creator).
import-plume-trash-warning = ⚠ Os itens no lixo ou eliminados não são migrados.

import-plume-source-required = Escolha um ficheiro de projeto Plume
import-plume-source-missing = Este ficheiro não existe
import-plume-source-not-file = Este caminho não é um ficheiro
import-plume-manuscript-binder = Manuscrito
import-plume-story-bible-binder = Bíblia da história
import-plume-notes-of = { $name } (notas)
import-plume-content-of = { $name } (texto)
import-plume-progress-title = A importar o projeto Plume…
import-plume-done = { $imported ->
    [one] { $imported } item importado.
   *[other] { $imported } itens importados.
} { $skipped ->
    [one] { $skipped } item no lixo não foi migrado.
   *[other] { $skipped } itens no lixo não foram migrados.
}
import-plume-nested-too-deep = O projeto não foi importado: a sua parte «{ $part }» tem mais de { $limit } níveis de aninhamento. Nenhum projeto Plume Creator se aproxima disso, e lê-lo poderia fazer o Skribisto falhar. Nada foi escrito.
import-plume-declares-entities = O projeto não foi importado: a sua parte «{ $part }» declara entidades XML, o que nenhum projeto Plume Creator faz. Expandi-las poderia exigir mais memória do que o computador tem e fazer o Skribisto falhar. Nada foi escrito.
import-plume-no-outline = Este ficheiro é um arquivo comprimido, mas não é um projeto Plume Creator: não tem estrutura. Nada foi escrito.
import-plume-empty-outline = O ficheiro de estrutura deste projeto Plume Creator está vazio, por isso não há nada para importar. Nada foi escrito.
import-plume-wrong-document = Isto não é um projeto Plume Creator: a sua parte «{ $part }» é um documento «{ $tag }». Nada foi escrito.
import-plume-too-new = A parte «{ $part }» deste projeto foi escrita por um Plume Creator mais recente (versão { $version }) do que o Skribisto consegue ler (até { $supported }). Nada foi escrito.

import-manuskript-title = Importar um projeto Manuskript
import-manuskript-source = Projeto Manuskript
import-manuskript-source-hint = Escolha o ficheiro .msk ou a pasta do projeto (qualquer versão do Manuskript).
import-manuskript-source-file = Escolher ficheiro…
import-manuskript-source-folder = Escolher pasta…
import-manuskript-source-required = Escolha um projeto Manuskript
import-manuskript-source-missing = Este ficheiro ou pasta não existe
import-manuskript-manuscript-binder = Manuscrito
import-manuskript-story-bible-binder = Bíblia da história
import-manuskript-characters-group = Personagens
import-manuskript-world-group = Mundo
import-manuskript-plots-group = Enredos
import-manuskript-project-info-note = Informações do projeto
import-manuskript-summary-note = Resumo
import-manuskript-importance-minor = Menor
import-manuskript-importance-secondary = Secundária
import-manuskript-importance-main = Principal
import-manuskript-notes-of = { $title } (notas)
import-manuskript-progress-title = A importar o projeto Manuskript…
import-manuskript-done = { $imported ->
    [one] { $imported } item importado.
   *[other] { $imported } itens importados.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } versão anterior veio com ele.
   *[other] { $revisions } versões anteriores vieram com eles.
}
import-manuskript-nested-too-deep = O projeto não foi importado: o seu ficheiro «{ $part }» tem mais de { $limit } níveis de aninhamento. Nenhum projeto Manuskript se aproxima disso, e lê-lo poderia fazer o Skribisto falhar. Nada foi escrito.
import-manuskript-folders-too-deep = O projeto não foi importado: um dos seus ficheiros está a mais de { $limit } pastas de profundidade. Nenhum projeto Manuskript se aproxima disso, e lê-lo poderia fazer o Skribisto falhar. Nada foi escrito.
import-manuskript-declares-entities = O projeto não foi importado: o seu ficheiro «{ $part }» declara entidades XML, o que nenhum projeto Manuskript faz. Expandi-las poderia exigir mais memória do que o computador tem e fazer o Skribisto falhar. Nada foi escrito.
import-manuskript-not-a-project-file = «{ $file }» não é um arquivo de projeto Manuskript nem o pequeno ficheiro .msk que aponta para a pasta de um projeto. Nada foi escrito.
import-manuskript-folder-missing = «{ $stub }» aponta para uma pasta de projeto chamada «{ $folder }» ao seu lado, e essa pasta não está lá. Um projeto Manuskript em modo pasta é formado pelos dois, que têm de viajar juntos. Nada foi escrito.
import-manuskript-not-a-project = Isto não parece um projeto Manuskript: não tem a marca do Manuskript nem estrutura. Nada foi escrito.
import-manuskript-format-too-new = Este projeto está no formato Manuskript { $version }, mais recente do que o Skribisto consegue ler (até { $supported }). Nada foi escrito.
import-manuskript-too-many-files = O projeto não foi importado: contém mais de { $limit } ficheiros, muito mais do que qualquer projeto Manuskript. Nada foi escrito.
import-manuskript-too-large = O projeto não foi importado: é maior do que os { $limit } MB que o Skribisto lê de um projeto Manuskript. Nada foi escrito.

project-import-close = Fechar
project-import-location = Pasta de destino
project-import-location-title = Escolha uma pasta de destino
project-import-name = Nome do ficheiro
project-import-name-placeholder = Nome do projeto
project-import-will-create = Será criado
project-import-cancel = Cancelar
project-import-import = Importar
project-import-location-required = Escolha uma pasta de destino
project-import-location-missing = Esta pasta não existe
project-import-location-not-folder = Este caminho não é uma pasta
project-import-location-readonly = Não é possível escrever nesta pasta
project-import-name-required = Introduza um nome de ficheiro
project-import-name-exists = Já existe aqui um ficheiro com este nome. A importação pedirá confirmação para o substituir
project-import-name-is-folder = Já existe aqui uma pasta com este nome, talvez um projeto guardado como pasta. Uma importação não a pode substituir: escolha outro nome
project-import-overwrite-title = Substituir o ficheiro existente?
project-import-overwrite-text = «{ $name }» já existe. Substituí-lo pelo projeto importado?
project-import-busy-title = Outro projeto está a ser importado
project-import-busy-text = Os projetos são importados um de cada vez. Aguarde até que a importação em curso termine, ou cancele-a, e depois inicie esta novamente.
project-import-progress-announcement = { $title } { $phase }
project-import-stopping = A parar a importação…
project-import-cancelled = Importação cancelada
project-import-open-now = Abrir agora
project-import-details = Detalhes
project-import-report = { $count ->
    [one] 1 coisa a saber sobre esta importação
   *[other] { $count } coisas a saber sobre esta importação
}
project-import-report-title = Sobre esta importação
project-import-error-title = Não foi possível importar o projeto
project-import-failed-cancelled = A importação foi cancelada. Nada foi escrito.
project-import-failed-target-exists = «{ $file }» já existe, e a importação não estava autorizada a substituí-lo. Nada foi escrito.
project-import-failed-target-folder = «{ $file }» é uma pasta, talvez um projeto guardado como pasta, e uma importação não a pode substituir. Escolha outro nome. Nada foi escrito.
project-import-failed-not-written = Não foi possível escrever «{ $file }» em «{ $folder }» ({ $reason }). Nada foi escrito.
project-import-failed-disk-full = Não há espaço suficiente em «{ $folder }» para escrever «{ $file }». Liberte algum espaço aí e depois importe novamente. Nada foi escrito.
project-import-failed-source-missing = «{ $file }» já não está lá. Nada foi escrito.
project-import-failed-unreadable = O Skribisto não conseguiu ler «{ $file }» ({ $reason }). Nada foi escrito.
project-import-failed-archive-damaged = «{ $file }» é um ficheiro comprimido que não pôde ser lido, provavelmente por estar danificado ({ $reason }). Nada foi escrito.
project-import-failed-archive-entries = O projeto não foi importado: o seu arquivo lista { $entries } ficheiros, mais do que os { $limit } que um projeto pode conter, o que só acontece com um ficheiro feito para esgotar a memória. Nada foi escrito.
project-import-failed-archive-member = O projeto não foi importado: a sua parte «{ $part }» descomprime-se em mais de { $limit } MB, o que só acontece com um ficheiro feito para esgotar a memória. Nada foi escrito.
project-import-failed-archive-size = O projeto não foi importado: descomprime-se em mais de { $limit } MB, o que só acontece com um ficheiro feito para esgotar a memória. Nada foi escrito.
project-import-failed-nested = O projeto não foi importado: a sua parte «{ $part }» tem mais de { $limit } níveis de aninhamento, e lê-la poderia fazer o Skribisto falhar. Nada foi escrito.
project-import-failed-entities = O projeto não foi importado: a sua parte «{ $part }» declara entidades XML. Expandi-las poderia exigir mais memória do que o computador tem e fazer o Skribisto falhar. Nada foi escrito.
project-import-failed-folders = O projeto não foi importado: um dos seus ficheiros está a mais de { $limit } pastas de profundidade, e lê-lo poderia fazer o Skribisto falhar. Nada foi escrito.
project-import-failed-malformed = O projeto não foi importado: a sua parte «{ $part }» está danificada e não pode ser lida ({ $reason }). Nada foi escrito.
project-import-failed-scrivener-refused = O Skribisto não consegue importar este projeto Scrivener. Nada foi escrito.
project-import-failed-scrivener-question = O projeto Scrivener coloca uma pergunta a que a sua revisão não respondeu. Reveja-o novamente e depois importe. Nada foi escrito.
project-import-failed-scrivener-not-accepted = O projeto Scrivener levanta uma questão que ainda não aceitou. Reveja-o novamente e depois importe. Nada foi escrito.
project-import-failed-scrivener-changed = O projeto Scrivener mudou depois de o ter revisto. Reveja-o novamente e depois importe. Nada foi escrito.
project-import-failed-internal = A importação parou num erro que o Skribisto não esperava: { $detail }

import-target-open-title = Este projeto está aberto
import-target-open-text = «{ $name }» está aberto no Skribisto. Importar por cima dele substituiria o projeto em que está a trabalhar, e a próxima gravação escreveria esse projeto de volta por cima da importação. Escolha outro nome de ficheiro, ou feche primeiro o projeto.
import-warnings-announcement = { $result } { $notice }.
quit-import-running-title = Uma importação ainda está em curso
quit-import-running-text = O Skribisto ainda está a importar «{ $name }». Se sair agora, a importação é interrompida: nada é escrito, e o projeto de onde está a importar fica como estava.
quit-import-stop = Parar a importação e sair

## Import a Scrivener project: the wizard
import-scrivener-title = Importar um projeto Scrivener
import-scrivener-step-project = Projeto
import-scrivener-step-review = Revisão
import-scrivener-step-save = Guardar
import-scrivener-back = Voltar
import-scrivener-next = Seguinte
import-scrivener-import = Importar
import-scrivener-cancel = Cancelar
import-scrivener-help = Ajuda

## Project step: choosing the project
import-scrivener-drop-title = Largue aqui um projeto Scrivener
import-scrivener-drop-hint = Um projeto .scriv, o ficheiro .scrivx que ele contém, uma cópia de segurança .zip ou um .scrivtemplate
import-scrivener-source = Projeto Scrivener
import-scrivener-source-placeholder = Caminho do projeto
import-scrivener-choose-file = Escolher um ficheiro…
import-scrivener-choose-folder = Escolher uma pasta…
import-scrivener-source-hint = Escolha a pasta .scriv do projeto, ou abra-a e escolha o ficheiro .scrivx que ela contém. Uma cópia de segurança .zip também serve.
import-scrivener-source-hint-mac = Se o projeto aparecer como um único ficheiro, use «Escolher um ficheiro…». Se aparecer como uma pasta, use «Escolher uma pasta…». Uma cópia de segurança .zip também serve.
import-scrivener-dialog-file-title = Escolha um projeto Scrivener
import-scrivener-dialog-folder-title = Escolha a pasta de um projeto Scrivener
import-scrivener-dialog-filter = Projeto Scrivener
import-scrivener-source-missing = Não há nada neste caminho
import-scrivener-source-unreadable-name = Não é possível ler este nome. Mude o nome do projeto no seu gestor de ficheiros e depois escolha-o novamente.
import-scrivener-empty-hint = Escolha um projeto para ver o que o Skribisto encontra nele.
import-scrivener-reading = A ler o projeto…
import-scrivener-stop-reading = Parar de ler
import-scrivener-read-failed = Não foi possível ler o projeto
import-scrivener-read-failed-body = Algo correu mal durante a leitura. Verifique novamente, ou consulte os detalhes.
import-scrivener-details = Detalhes
import-scrivener-check-again = Verificar novamente
import-scrivener-refusal-unknown-title = Este projeto não pode ser importado
import-scrivener-refusal-unknown-body = O Skribisto não sabe como o trazer. Os detalhes indicam o motivo.

## Project step: the project card
import-scrivener-made-with-mac = Guardado pela última vez pelo Scrivener { $version } para macOS
import-scrivener-made-with-windows = Guardado pela última vez pelo Scrivener { $version } para Windows
import-scrivener-made-with-linux = Guardado pela última vez pelo Scrivener { $version } para Linux
import-scrivener-made-with-ios = Guardado pela última vez pelo Scrivener { $version } para iPhone e iPad
import-scrivener-made-with-version = Guardado pela última vez pelo Scrivener { $version }
import-scrivener-made-with-unknown = Projeto Scrivener, formato { $format }
import-scrivener-saved-on = Guardado em { $date } no dispositivo «{ $device }»
import-scrivener-saved-on-no-device = Guardado em { $date }
import-scrivener-card-counts = { $documents ->
    [one] 1 documento
   *[other] { $documents } documentos
}, { $manuscript } no manuscrito

## Review step
import-scrivener-review-hint = É assim que o seu arquivador vai ser trazido. Nada é escrito até importar, e muitos itens podem depois mudar de tipo no arquivador, com Converter em.
import-scrivener-choices-structure = Níveis
import-scrivener-top-level = Pastas de nível superior
import-scrivener-choices-text = Texto
import-scrivener-choice-containers = Pastas no nível { $depth }
import-scrivener-choice-texts = Documentos no nível { $depth }
import-scrivener-choice-count = { $count ->
    [one] 1 item
   *[other] { $count } itens
}
import-scrivener-choice-count-example = { $count ->
    [one] 1 item: «{ $example }»
   *[other] { $count } itens, como «{ $example }»
}
import-scrivener-rung-part = Partes
import-scrivener-rung-chapter = Capítulos
import-scrivener-rung-plain = Pastas simples
import-scrivener-texts-scenes = Cenas
import-scrivener-texts-chapters = Capítulos
import-scrivener-choice-books = Livros no manuscrito
import-scrivener-books-one = Um só livro
import-scrivener-books-several = Um livro por pasta
import-scrivener-choice-split = { $count ->
    [one] Dividir «{ $title }» no seu título interno
   *[other] Dividir «{ $title }» nos seus { $count } títulos internos
}
import-scrivener-root-items = { $count ->
    [one] 1 item
   *[other] { $count } itens
}
import-scrivener-root-items-manuscript = { $count ->
    [one] 1 item, parece manuscrito
   *[other] { $count } itens, parece manuscrito
}
import-scrivener-dest-notes = Notas
import-scrivener-dest-story-bible = Bíblia da história
import-scrivener-dest-manuscript = Outro livro
import-scrivener-dest-leave-out = Deixar de fora
import-scrivener-choice-trash = { $count ->
    [one] Trazer o lixo (1 item)
   *[other] Trazer o lixo ({ $count } itens)
}
import-scrivener-trash-detail = { $count ->
    [0] Nenhum deles contém escrita.
    [one] 1 deles contém escrita.
   *[other] { $count } deles contêm escrita.
}
import-scrivener-choice-blank-lines = As linhas em branco tornam-se quebras de cena
import-scrivener-choice-ornaments = As linhas de ornamentos tornam-se quebras de cena
import-scrivener-preview-label = Estrutura do novo projeto
import-scrivener-col-title = Título
import-scrivener-col-becomes = Torna-se
import-scrivener-col-exports = Nas exportações
import-scrivener-in-exports-yes = Sim
import-scrivener-in-exports-no = Não
import-scrivener-becomes-item = Item
import-scrivener-becomes-trashed = No lixo
import-scrivener-before-title = Antes de importar
import-scrivener-review-error-unreadable = Não foi possível preparar a revisão. Escolha novamente o projeto.
import-scrivener-review-error-choice = Esta escolha não se adequa ao projeto. Escolha novamente o projeto.
import-scrivener-review-error-unmappable = Com esta escolha, não foi possível trazer o arquivador sem perder alguma coisa. Faça outra escolha, ou comunique o problema.

## Save step
import-scrivener-save-hint = A importação cria um novo projeto Skribisto. O seu projeto Scrivener é apenas lido, nunca alterado.
import-scrivener-title-field = Título
import-scrivener-author = Autor
import-scrivener-language = Idioma
import-scrivener-location-inside-project = Escolha uma pasta fora do projeto Scrivener. O Scrivener só guarda lá dentro os seus próprios ficheiros.
import-scrivener-copy-research-label = Copiar os ficheiros de pesquisa para junto do novo projeto
import-scrivener-copy-research = { $count ->
    [one] 1 ficheiro de pesquisa que o Skribisto não consegue abrir, como um PDF ou um som, vai para uma pasta ao lado do projeto. Se desmarcar esta opção, a sua nota indica onde ele está no seu projeto Scrivener.
   *[other] { $count } ficheiros de pesquisa que o Skribisto não consegue abrir, como PDF ou sons, vão para uma pasta ao lado do projeto. Se desmarcar esta opção, as suas notas indicam onde eles estão no seu projeto Scrivener.
}
import-scrivener-default-name = Projeto Scrivener

## Progress and result
import-scrivener-progress-title = A importar o projeto Scrivener…
import-scrivener-done-title = Projeto Scrivener importado
import-scrivener-done = { $items ->
    [one] O novo projeto contém 1 item.
   *[other] O novo projeto contém { $items } itens.
}
import-scrivener-done-check = { $items ->
    [one] O novo projeto contém 1 item.
   *[other] O novo projeto contém { $items } itens.
} { $findings ->
    [one] 1 coisa merece a sua atenção.
   *[other] { $findings } coisas merecem a sua atenção.
}
import-scrivener-review-again = Rever novamente
## The import's report (the dialog See report opens)
import-scrivener-see-report = Ver relatório
import-scrivener-report-from = Lido de { $path }
import-scrivener-report-to = Escrito em { $path }
import-scrivener-report-came-across = O que foi trazido
import-scrivener-report-nothing-counted = Nada foi contado.
import-scrivener-report-items = { $count ->
    [one] 1 item
   *[other] { $count } itens
}
import-scrivener-report-words = { $count ->
    [one] 1 palavra
   *[other] { $count } palavras
}
import-scrivener-report-synopses = { $count ->
    [one] 1 sinopse
   *[other] { $count } sinopses
}
import-scrivener-report-document-notes = { $count ->
    [one] notas de 1 documento
   *[other] notas de { $count } documentos
}
import-scrivener-report-comments = { $count ->
    [one] 1 comentário
   *[other] { $count } comentários
}
import-scrivener-report-footnotes = { $count ->
    [one] 1 nota de rodapé
   *[other] { $count } notas de rodapé
}
import-scrivener-report-pictures = { $count ->
    [one] 1 imagem
   *[other] { $count } imagens
}
import-scrivener-report-templates = { $count ->
    [one] 1 modelo de nota
   *[other] { $count } modelos de notas
}
import-scrivener-report-tags = { $count ->
    [one] 1 etiqueta
   *[other] { $count } etiquetas
}
import-scrivener-report-statuses = { $count ->
    [one] 1 estado
   *[other] { $count } estados
}
import-scrivener-report-trashed = { $count ->
    [one] 1 item no lixo
   *[other] { $count } itens no lixo
}
import-scrivener-report-research-files = { $count ->
    [one] 1 ficheiro de pesquisa
   *[other] { $count } ficheiros de pesquisa
}
import-scrivener-report-choices = O que escolheu
import-scrivener-report-accepted-mobile = Importou o projeto sem as alterações feitas no Scrivener para iPhone ou iPad que ainda não estavam nele.
import-scrivener-report-accepted-no-text = Importou esta cópia com os documentos vazios, já que nenhum dos seus textos estava nela.
import-scrivener-report-choice = { $subject }: { $answer }
import-scrivener-report-yes = sim
import-scrivener-report-no = não
import-scrivener-report-research-copied = Os ficheiros de pesquisa que o Skribisto não consegue abrir foram copiados para «{ $folder }», ao lado do novo projeto.
import-scrivener-report-research-not-copied = Os ficheiros de pesquisa que o Skribisto não consegue abrir não foram copiados. As suas notas indicam onde estão no seu projeto Scrivener.
import-scrivener-report-research-copied-except = { $count ->
    [one] Os ficheiros de pesquisa que o Skribisto não consegue abrir foram copiados para «{ $folder }», ao lado do novo projeto, exceto 1, que fica no seu projeto Scrivener. A sua nota indica onde está.
   *[other] Os ficheiros de pesquisa que o Skribisto não consegue abrir foram copiados para «{ $folder }», ao lado do novo projeto, exceto { $count }, que ficam no seu projeto Scrivener. As suas notas indicam onde estão.
}
import-scrivener-report-research-none-copied = { $count ->
    [one] Pediu que os ficheiros de pesquisa fossem copiados, mas 1 não pôde ser copiado. Fica no seu projeto Scrivener, e a sua nota indica onde está.
   *[other] Pediu que os ficheiros de pesquisa fossem copiados, mas { $count } não puderam ser copiados. Ficam no seu projeto Scrivener, e as suas notas indicam onde estão.
}
import-scrivener-report-chose-project = Escolheu importar «{ $project }», um dos projetos que estavam no que selecionou.
import-scrivener-report-chose-manifest = Escolheu o ficheiro do arquivador «{ $file }».
import-scrivener-report-chose-enclosing = Selecionou algo dentro de «{ $project }», por isso foi importado o projeto inteiro.
import-scrivener-report-chose-upgraded = Importou «{ $current }», o projeto em que continuou a escrever, e não a cópia que o Scrivener guardou antes de uma atualização.
import-scrivener-report-chose-backup = Importou «{ $backup }», a cópia mais antiga que o Scrivener guardou antes de uma atualização.
import-scrivener-report-title-placeholder-rejected = O título nas definições do projeto era um marcador de posição de um modelo, por isso o Skribisto propôs antes o nome do projeto.
import-scrivener-report-author-placeholder-rejected = O nome do autor nas definições do projeto era um marcador de posição de um modelo, por isso o Skribisto não propôs nenhum.
import-scrivener-report-title-from-compile-settings = O Skribisto propôs o título indicado nas definições de compilação do projeto: o nome que o projeto tinha quando foi criado, a menos que lá tenha sido definido um título no Scrivener.
import-scrivener-report-title-from-template = O Skribisto propôs o título do próprio modelo.
import-scrivener-report-title-from-folder-name = O Skribisto propôs o nome do projeto como título.
import-scrivener-report-title-from-project-properties = O Skribisto propôs o título indicado nas propriedades do projeto.
import-scrivener-report-author-from-project = O Skribisto propôs o nome do autor indicado nas definições do projeto. O autor de um modelo pode ter ficado lá, e seria impresso numa página de título.
import-scrivener-report-language-not-stated = O projeto não indicava o seu idioma, por isso o Skribisto propôs o idioma em que ele próprio é apresentado. O idioma decide como as palavras são contadas e como as aspas são escritas.
import-scrivener-report-language-from-script = O Skribisto propôs o idioma a partir do sistema de escrita do manuscrito ({ $tag }).
import-scrivener-report-you-changed-it = { $hint } Alterou essa proposta.
import-scrivener-report-content-salvaged-text = O texto de «{ $item }» estava em falta no projeto. As suas palavras vêm do índice de pesquisa do Scrivener, em texto simples e talvez de uma versão mais antiga: verifique-as.
import-scrivener-report-content-salvaged-notes = As notas de «{ $item }» estavam em falta no projeto. As suas palavras vêm do índice de pesquisa do Scrivener, em texto simples e talvez de uma versão mais antiga: verifique-as.
import-scrivener-report-content-salvaged-synopsis = A sinopse de «{ $item }» estava em falta no projeto. As suas palavras vêm do índice de pesquisa do Scrivener, em texto simples e talvez de uma versão mais antiga: verifique-as.
import-scrivener-report-document-others = { $others ->
    [one] { $sentence } O mesmo acontece em mais 1 lugar, indicado abaixo.
   *[other] { $sentence } O mesmo acontece em mais { $others } lugares, indicados abaixo.
}
import-scrivener-report-item-notes = { $path } (notas do documento)
import-scrivener-report-item-synopsis = { $path } (sinopse)
import-scrivener-report-item-project-note = { $path } (nota do projeto)
import-scrivener-report-item-recovered = { $path } (texto recuperado)
import-scrivener-report-untitled = Sem título
import-scrivener-report-no-findings = A importação não tem mais nada a reportar.
import-scrivener-report-group-unread = Não foi possível ler
import-scrivener-report-group-check = A verificar
import-scrivener-report-group-changed = Trazido de outra forma
import-scrivener-report-group-left = Deixado para trás
import-scrivener-report-group-count = { $group } ({ $count })
import-scrivener-report-items-toggle = { $count ->
    [one] 1 item
   *[other] { $count } itens
}
import-scrivener-report-items-toggle-named = { $count ->
    [one] 1 item: { $line }
   *[other] { $count } itens: { $line }
}
import-scrivener-report-names-toggle = { $count ->
    [one] 1 nome
   *[other] { $count } nomes
}
import-scrivener-report-names-toggle-named = { $count ->
    [one] 1 nome: { $line }
   *[other] { $count } nomes: { $line }
}
import-scrivener-report-more = { $count ->
    [one] E mais 1.
   *[other] E mais { $count }.
}
import-scrivener-report-open-item = Abrir no editor
import-scrivener-report-links-hint = Abra o novo projeto e depois volte a mostrar este relatório a partir da paleta de comandos: cada item aqui indicado passa então a abrir-se com um clique.
import-scrivener-copy-report = Copiar relatório
import-scrivener-report-copied = Relatório copiado
import-scrivener-report-copy-failed = Não foi possível copiar o relatório.
import-scrivener-report-close = Fechar
import-scrivener-report-none-title = Nenhum relatório de importação
import-scrivener-report-none = Nenhuma importação do Scrivener terminou desde que o Skribisto foi iniciado. O Skribisto guarda os relatórios das últimas importações até ser fechado.
import-scrivener-report-gone = O relatório desta importação já não está guardado. O Skribisto guarda { $count ->
    [one] o relatório da última importação
   *[other] os relatórios das últimas { $count } importações
} até ser fechado.

## Words the import writes into the new project, in the language Skribisto shows itself in
import-scrivener-manuscript-binder = Manuscrito
import-scrivener-notes-binder = Notas
import-scrivener-notes-of = { $title } (notas)
import-scrivener-project-notes = Notas do projeto
import-scrivener-recovered = Textos recuperados
import-scrivener-untitled-template = Modelo sem título
import-scrivener-research-folder = Ficheiros de pesquisa de { $name }
import-scrivener-list-separator = {", "}
import-scrivener-label-details = Do campo «{ $field }» do Scrivener
import-scrivener-status-details = Do campo «{ $field }» do Scrivener
import-scrivener-unnamed-label = Rótulo { $number }
import-scrivener-unnamed-status = Estado { $number }
import-scrivener-label-tag = { $name } (rótulo)
import-scrivener-untitled-tag = Sem título
import-scrivener-keyword-details = A palavra-chave «{ $path }» no Scrivener
import-scrivener-unnamed-keyword = Palavra-chave { $number }
import-scrivener-keyword-tag = { $name } (palavra-chave)
import-scrivener-collection-details = Uma coleção no Scrivener
import-scrivener-collection-tag = { $name } (coleção)
import-scrivener-field-details = Do campo «{ $field }» do Scrivener
import-scrivener-field-value = { $field }: { $option }
import-scrivener-field-tag = { $name } (campo)
import-scrivener-deadline-day = Prazo
import-scrivener-from-scrivener = Do Scrivener
import-scrivener-note-project = Projeto
import-scrivener-note-project-bookmarks = Marcadores do projeto
import-scrivener-note-replacements = Substituições de compilação
import-scrivener-note-auto-complete = Entradas de preenchimento automático
import-scrivener-note-saved-searches = Pesquisas guardadas
import-scrivener-note-custom-metadata = Metadados personalizados
import-scrivener-note-targets = Objetivos
import-scrivener-note-bookmarks = Marcadores
import-scrivener-abbreviated-title = Título abreviado
import-scrivener-subjects = Assuntos
import-scrivener-company = Empresa
import-scrivener-session-target = Objetivo da sessão
import-scrivener-session-target-calculated = Objetivo da sessão, calculado pelo Scrivener a partir do prazo
import-scrivener-writing-days = Dias de escrita
import-scrivener-weekday-monday = segunda-feira
import-scrivener-weekday-tuesday = terça-feira
import-scrivener-weekday-wednesday = quarta-feira
import-scrivener-weekday-thursday = quinta-feira
import-scrivener-weekday-friday = sexta-feira
import-scrivener-weekday-saturday = sábado
import-scrivener-weekday-sunday = domingo
import-scrivener-draft-target = Objetivo do rascunho
import-scrivener-deadline = Prazo
import-scrivener-unit-words = palavras
import-scrivener-unit-characters = caracteres
import-scrivener-unit-pages = páginas
import-scrivener-search-query = Termos pesquisados
import-scrivener-search-in = Pesquisado em
import-scrivener-search-operator = Correspondência
import-scrivener-search-in-all = tudo
import-scrivener-search-in-title = títulos
import-scrivener-search-in-text = texto
import-scrivener-search-in-notes = notas
import-scrivener-search-in-synopsis = sinopses
import-scrivener-search-in-keywords = palavras-chave
import-scrivener-search-in-label = rótulos
import-scrivener-search-in-status = estados
import-scrivener-search-in-section-type = tipos de secção
import-scrivener-search-in-custom-metadata = metadados personalizados
import-scrivener-search-any = qualquer palavra
import-scrivener-search-all = todas as palavras
import-scrivener-search-exact = expressão exata
import-scrivener-label-value = { $label }: { $value }
import-scrivener-replacement-regex = expressão regular
import-scrivener-replacement-off = desativada
import-scrivener-in-own-note = Indicados na sua própria nota.


## Import a Scrivener project: what the import says about the project, in the writer's words.
## Every key the converter can give has a line here (see `import_scrivener::messages`).

import-scrivener-paragraph = { $title }. { $body }

## When the project cannot be imported
import-scrivener-refusal-missing-title = Não há nada neste caminho
import-scrivener-refusal-missing-body = «{ $file }» já não está lá. Escolha novamente o projeto.
import-scrivener-refusal-unreadable-title = Não é possível ler este projeto
import-scrivener-refusal-unreadable-body = O Skribisto não conseguiu ler «{ $file }» ({ $reason }). Verifique se o consegue abrir e tente novamente.
import-scrivener-refusal-not-a-scrivener-project-title = Não há aqui nenhum projeto Scrivener
import-scrivener-refusal-not-a-scrivener-project-body = Escolha a pasta .scriv do projeto, o ficheiro .scrivx que ela contém, uma cópia de segurança .zip ou um .scrivtemplate.
import-scrivener-refusal-manifest-without-files-title = Os documentos não estão nesta cópia
import-scrivener-refusal-manifest-without-files-body = «{ $manifest }» está aqui, mas não a pasta Files que contém os documentos que ele lista. Um serviço de sincronização pode não ter acabado de copiar o projeto, ou só o ficheiro .scrivx foi copiado. Se escolheu o ficheiro .scrivx, escolha antes a pasta do projeto com «Escolher uma pasta…».
import-scrivener-refusal-search-too-wide-title = Esta pasta contém demasiado para pesquisar
import-scrivener-refusal-search-too-wide-body = Tem mais de { $limit } pastas a percorrer. Escolha o próprio projeto Scrivener.
import-scrivener-refusal-scrivener-one-for-mac-title = Este projeto foi criado pelo Scrivener 1 para Mac
import-scrivener-refusal-scrivener-one-for-mac-body = O Skribisto lê projetos do Scrivener 2 e posteriores. Abra-o uma vez no Scrivener 2 ou 3 para o atualizar, ou compile-o para um documento Word e importe esse documento.
import-scrivener-refusal-ios-stub-without-mobile-data-title = Isto é apenas um marcador de posição do Scrivener para iPhone ou iPad
import-scrivener-refusal-ios-stub-without-mobile-data-body = O projeto propriamente dito não está aqui. Sincronize-o a partir do seu iPhone ou iPad, ou partilhe-o a partir daí como .zip, e importe esse ficheiro.
import-scrivener-refusal-newer-title = Este projeto foi criado por um Scrivener mais recente
import-scrivener-refusal-unknown-format-version-body = O seu formato, { $version }, não é um que o Skribisto conheça. Procure uma atualização do Skribisto. Até lá, compile-o para um documento Word no Scrivener e importe esse documento.
import-scrivener-refusal-newer-format-stamp-body = Os seus ficheiros estão marcados como versão { $stamp }, e o Skribisto lê até à versão { $newest }. Procure uma atualização do Skribisto. Até lá, compile-o para um documento Word no Scrivener e importe esse documento.
import-scrivener-refusal-version-stamp-unreadable-title = Não é possível ler a versão do projeto
import-scrivener-refusal-version-stamp-unreadable-body = O seu ficheiro Files/version.txt não contém nenhum número de versão, por isso o Skribisto não consegue saber se lê este projeto. Abra o projeto uma vez no Scrivener e depois importe-o novamente.
import-scrivener-refusal-damaged-manifest-title = Não é possível ler o ficheiro do arquivador
import-scrivener-refusal-damaged-manifest-body = «{ $manifest }» está danificado. { $spare_copies ->
    [0] Abra o projeto no Scrivener, que o consegue reparar, e depois importe-o novamente.
    [one] O Scrivener guarda uma cópia de reserva dele dentro do projeto, que o Skribisto não lê. Abra o projeto no Scrivener, que o consegue reparar, e depois importe-o novamente.
   *[other] O Scrivener guarda { $spare_copies } cópias de reserva dele dentro do projeto, que o Skribisto não lê. Abra o projeto no Scrivener, que o consegue reparar, e depois importe-o novamente.
}
import-scrivener-refusal-damaged-manifest-new-project-placeholder-body = «{ $manifest }» é o ficheiro vazio que o Scrivener escreve para um projeto que ainda não guardou. Abra o projeto no Scrivener e deixe-o guardar, e depois importe-o novamente.
import-scrivener-refusal-manifest-too-large-title = Este ficheiro do arquivador é demasiado grande
import-scrivener-refusal-manifest-too-large-body = «{ $manifest }» é maior do que os { $limit } MB que o Skribisto lê. Nenhum projeto real se aproxima disso, por isso o ficheiro pode estar danificado.
import-scrivener-refusal-no-item-identifiers-title = O arquivador não lista nenhum documento que o Skribisto consiga encontrar
import-scrivener-refusal-no-item-identifiers-body = Nenhum dos seus itens tem um identificador, por isso não é possível encontrar os respetivos ficheiros. Abra o projeto uma vez no Scrivener e depois importe-o novamente.
import-scrivener-refusal-incomplete-copy-title = Faltam os documentos nesta cópia
import-scrivener-refusal-incomplete-copy-body = A pasta Files está aqui, mas os documentos que devia conter não estão. Um serviço de sincronização pode não ter acabado de copiar o projeto. Aguarde que termine, ou copie novamente toda a pasta .scriv, e depois verifique novamente.
import-scrivener-refusal-archive-title = Não é possível abrir este arquivo
import-scrivener-refusal-archive-unreadable-body = Não foi possível ler «{ $file }» ({ $reason }). Pode estar danificado. Descomprima-o primeiro e depois escolha a pasta .scriv que ele contém.
import-scrivener-refusal-archive-unsafe-name-title = Não é possível abrir este arquivo em segurança
import-scrivener-refusal-archive-unsafe-name-body = «{ $file }» contém um ficheiro chamado «{ $member }», que ficaria fora do arquivo, por isso o Skribisto não o abre.
import-scrivener-refusal-archive-unsafe-name-empty-body = «{ $file }» contém um ficheiro sem nome, por isso o Skribisto não o abre.
import-scrivener-refusal-archive-unsafe-name-control-character-body = «{ $file }» contém um ficheiro cujo nome inclui um carácter de controlo, por isso o Skribisto não o abre.
import-scrivener-refusal-archive-too-large-title = Este arquivo é demasiado grande para ser aberto em segurança
import-scrivener-refusal-archive-too-many-members-body = Contém mais de { $limit } ficheiros. Descomprima-o primeiro e depois escolha a pasta .scriv que ele contém.
import-scrivener-refusal-archive-member-too-large-body = «{ $member }» descomprime-se em mais de { $limit } MB. Descomprima o arquivo primeiro e depois escolha a pasta .scriv que ele contém.
import-scrivener-refusal-archive-too-large-body = Descomprime-se em mais de { $limit } MB. Descomprima-o primeiro e depois escolha a pasta .scriv que ele contém.
import-scrivener-refusal-archive-encrypted-title = Este arquivo está protegido por uma palavra-passe
import-scrivener-refusal-archive-encrypted-body = Descomprima «{ $file }» primeiro e depois escolha a pasta .scriv que ele contém.
import-scrivener-refusal-archive-unsupported-compression-title = Este arquivo está comprimido de uma forma que o Skribisto não consegue ler
import-scrivener-refusal-archive-unsupported-compression-body = «{ $member }» está comprimido com { $method }. Descomprima o arquivo primeiro e depois escolha a pasta .scriv que ele contém.
import-scrivener-refusal-duplicate-archive-members-title = Este arquivo lista alguns ficheiros duas vezes
import-scrivener-refusal-duplicate-archive-members-body = «{ $file }» contém mais de um ficheiro com o mesmo nome, por isso o Skribisto não consegue saber qual pertence ao projeto. Descomprima-o primeiro e depois escolha a pasta .scriv que ele contém.
import-scrivener-refusal-template-undecodable-title = Este ficheiro de modelo está incompleto
import-scrivener-refusal-template-undecodable-too-large-title = Este ficheiro de modelo é demasiado grande para ser aberto
import-scrivener-refusal-template-undecodable-not-a-template-title = Este ficheiro não é um modelo do Scrivener
import-scrivener-refusal-template-undecodable-unknown-encoding-title = Este ficheiro de modelo está guardado num formato que o Skribisto não conhece
import-scrivener-refusal-template-undecodable-body = Não é possível descodificar «{ $file }»: pode terminar antes do projeto que contém. Transfira-o novamente, ou abra-o no Scrivener e crie um projeto a partir dele.
import-scrivener-refusal-template-undecodable-too-large-body = «{ $file }» é maior do que { $limit } MB. Abra-o no Scrivener, crie um projeto a partir dele e depois importe esse projeto.
import-scrivener-refusal-template-undecodable-not-a-template-body = «{ $file }» não contém nenhum projeto Scrivener. Escolha o ficheiro de modelo que o Scrivener guardou, ou um projeto criado a partir dele.
import-scrivener-refusal-template-undecodable-unknown-encoding-body = «{ $file }» guarda o seu projeto de uma forma que o Skribisto não consegue descodificar. Abra-o no Scrivener, crie um projeto a partir dele e depois importe esse projeto.
import-scrivener-refusal-cloud-placeholders-title = { $count ->
    [one] 1 ficheiro deste projeto ainda está na nuvem
   *[other] { $count } ficheiros deste projeto ainda estão na nuvem
}
import-scrivener-refusal-cloud-placeholders-body = Não foram transferidos para este computador, por exemplo { $examples }. Torne o projeto disponível offline na sua aplicação de sincronização e depois verifique novamente.
import-scrivener-refusal-unreadable-members-title = { $count ->
    [one] Não é possível ler 1 ficheiro deste projeto
   *[other] Não é possível ler { $count } ficheiros deste projeto
}
import-scrivener-refusal-unreadable-members-body = Por exemplo { $examples }. Verifique se tem permissão para ler todo o projeto e se o disco está em bom estado, e depois verifique novamente.
import-scrivener-refusal-needed-member-is-link-title = Este projeto aponta para fora da sua própria pasta
import-scrivener-refusal-needed-member-is-link-body = «{ $member }» é uma ligação para outro local deste computador, e o Skribisto não segue ligações. Copie o projeto de forma que todos os ficheiros fiquem dentro dele e tente novamente.
import-scrivener-refusal-project-changed-during-read-title = O projeto mudou enquanto estava a ser lido
import-scrivener-refusal-project-changed-during-read-body = O Scrivener ou um serviço de sincronização ainda está a escrever nele. Feche-o no Scrivener, aguarde que a sincronização termine e depois verifique novamente.
import-scrivener-refusal-too-many-members-title = Este projeto contém demasiados ficheiros
import-scrivener-refusal-too-many-members-body = Tem mais de { $limit } ficheiros, mais do que qualquer projeto real. Pode não ser um projeto Scrivener, ou pode estar danificado.
import-scrivener-refusal-too-large-title = Este projeto é demasiado grande
import-scrivener-refusal-too-large-body = A sua escrita ultrapassa os { $limit } MB, mais do que o Skribisto consegue importar em segurança.
import-scrivener-refusal-structure-unmappable-title = O Skribisto não consegue transpor este arquivador
import-scrivener-refusal-structure-unmappable-body = Não foi possível trazer o arquivador sem perder ou deslocar alguma coisa. Isto é uma falha do Skribisto, não do seu projeto. Comunique-a, juntando o que «Detalhes» mostra.

## When there is a choice to make
import-scrivener-roots-title = Há aqui { $count } projetos. Qual deles?
import-scrivener-roots-hint = Esta pasta ou arquivo contém mais de um projeto Scrivener. Escolha o que quer importar.
import-scrivener-manifests-title = Este projeto tem { $count } ficheiros do arquivador
import-scrivener-manifests-hint = O Scrivener guarda o arquivador, a lista de todos os documentos e pastas, num ficheiro .scrivx. Os ficheiros a mais foram muito provavelmente deixados por um serviço de sincronização. Escolha aquele em que trabalhou por último.
import-scrivener-manifests-hint-preselected = O Scrivener guarda o arquivador, a lista de todos os documentos e pastas, num ficheiro .scrivx. Os ficheiros a mais foram muito provavelmente deixados por um serviço de sincronização. O que está selecionado é aquele em que muito provavelmente trabalhou por último: verifique-o antes de continuar.
import-scrivener-manifest-row = { $file } · guardado em { $date } · { $documents ->
    [one] 1 item
   *[other] { $documents } itens
}
import-scrivener-manifest-row-undated = { $file } · { $documents ->
    [one] 1 item
   *[other] { $documents } itens
}
import-scrivener-manifest-row-unread = { $file } · não é possível ler
import-scrivener-enclosing-title = Isto faz parte do projeto Scrivener «{ $project }»
import-scrivener-enclosing-body = Escolheu um ficheiro ou uma pasta dentro de um projeto. O Skribisto importa o projeto inteiro.
import-scrivener-enclosing-use = Importar «{ $project }»
import-scrivener-upgrade-title = Esta é a cópia que o Scrivener guardou antes de uma atualização
import-scrivener-upgrade-body = Quando o Scrivener atualiza um projeto para uma versão mais recente, guarda o antigo como «{ $backup }». O projeto em que continuou a escrever é «{ $current }», ao lado dele.
import-scrivener-upgrade-body-unclear = Quando o Scrivener atualiza um projeto para uma versão mais recente, guarda o antigo como «{ $backup }». O projeto ao lado dele, «{ $current }», não pôde ser lido corretamente, por isso escolha qual importar.
import-scrivener-upgrade-use-current = Importar «{ $current }», o projeto em que continuou a escrever
import-scrivener-upgrade-keep-backup = Importar esta cópia mais antiga, «{ $backup }»

## What must be accepted before going on
import-scrivener-ios-pending-title = As alterações feitas no Scrivener para iPhone ou iPad ainda não estão nesta cópia
import-scrivener-ios-pending-body = O Scrivener num computador incorpora essas alterações da próxima vez que abrir o projeto. Abra o projeto uma vez no Scrivener no seu computador e depois importe-o novamente.
import-scrivener-ios-pending-accept = Importá-lo sem essas alterações
import-scrivener-no-text-title = { $count ->
    [one] 1 documento, e nem uma palavra de texto nesta cópia
   *[other] { $count } documentos, e nem uma palavra de texto nesta cópia
}
import-scrivener-no-text-body = Um serviço de sincronização pode ainda estar a copiar o projeto, ou a cópia perdeu os seus documentos. Se o importar agora, todos os documentos chegam vazios.
import-scrivener-no-text-accept = Importar os documentos vazios

## Things to know, nothing to decide
import-scrivener-notice-project-found-below-pick-title = O projeto foi encontrado dentro da pasta que escolheu
import-scrivener-notice-project-found-below-pick-body = Está em «{ $path }».
import-scrivener-notice-source-is-template-title = Isto é um modelo do Scrivener
import-scrivener-notice-source-is-template-named = Isto é o modelo do Scrivener «{ $title }»
import-scrivener-notice-source-is-template-body = Importá-lo cria um novo projeto a partir do que o modelo contém, incluindo as suas instruções.
import-scrivener-notice-source-is-backup-title = Isto parece uma cópia de segurança feita pelo Scrivener
import-scrivener-notice-source-is-backup-body = Contém o projeto tal como estava quando a cópia de segurança foi feita. Se continuou a escrever depois disso, escolha o próprio projeto.
import-scrivener-notice-importing-pre-upgrade-copy-title = Está a importar a cópia que o Scrivener guardou antes de uma atualização
import-scrivener-notice-importing-pre-upgrade-copy-body = O projeto em que continuou a escrever está ao lado dela e contém tudo o que foi escrito desde então.
import-scrivener-notice-imported-from-ios-stub-title = Este projeto foi criado no Scrivener para iPhone ou iPad
import-scrivener-notice-imported-from-ios-stub-body = Nunca foi aberto num computador, por isso é lido a partir da cópia que o iPhone ou iPad guarda.
import-scrivener-notice-stale-mobile-binder-ignored-title = Um arquivador antigo de um iPhone ou iPad é posto de lado
import-scrivener-notice-stale-mobile-binder-ignored-body = O Scrivener num computador já incorporou as suas alterações.
import-scrivener-notice-identical-manifest-copies-title = { $count ->
    [one] Está aqui 1 cópia do ficheiro do arquivador
   *[other] Estão aqui { $count } cópias idênticas do ficheiro do arquivador
}
import-scrivener-notice-identical-manifest-copies-body = Têm o mesmo conteúdo, por isso qualquer uma delas dá o mesmo projeto.
import-scrivener-notice-manifest-copies-ignored-title = As cópias a mais do ficheiro do arquivador são postas de lado
import-scrivener-notice-manifest-copies-ignored-body = «{ $chosen }» é aquele em que o Scrivener trabalhou por último. { $count ->
    [one] 1 cópia deixada por um serviço de sincronização é posta de lado.
   *[other] { $count } cópias deixadas por um serviço de sincronização são postas de lado.
}
import-scrivener-notice-pinned-manifest-looks-like-a-copy-title = O ficheiro do arquivador que escolheu parece uma cópia de sincronização
import-scrivener-notice-pinned-manifest-looks-like-a-copy-body = «{ $name }» tem o nome de uma cópia de «{ $original }», que está ao lado dele. Se a importação parecer antiga, escolha antes «{ $original }».
import-scrivener-notice-pinned-manifest-has-siblings-title = { $count ->
    [one] Há 1 outro ficheiro do arquivador ao lado daquele que escolheu
   *[other] Há outros { $count } ficheiros do arquivador ao lado daquele que escolheu
}
import-scrivener-notice-pinned-manifest-has-siblings-body = Só aquele que escolheu é lido.
import-scrivener-notice-legacy-v1-files-beside-title = Os ficheiros de um Scrivener mais antigo são postos de lado
import-scrivener-notice-legacy-v1-files-beside-body = Foram deixados para trás quando o projeto foi atualizado. Só os ficheiros da versão atual são lidos.
import-scrivener-notice-mixed-payload-trees-title = Este projeto contém ficheiros de duas versões do Scrivener
import-scrivener-notice-mixed-payload-trees-body = Os documentos são lidos onde o ficheiro do arquivador diz que estão.
import-scrivener-notice-version-disagreement-title = Os ficheiros deste projeto não estão todos de acordo quanto à sua versão
import-scrivener-notice-id-scheme-mismatch-body = O ficheiro do arquivador indica a versão { $version }, mas dá nome aos seus documentos à maneira de outra versão. São lidos onde ele aponta. Se a importação parecer errada, abra o projeto uma vez no Scrivener e importe-o novamente.
import-scrivener-notice-version-stamp-mismatch-body = É lido da forma que o seu ficheiro do arquivador descreve. Se a importação parecer errada, abra o projeto uma vez no Scrivener e importe-o novamente.
import-scrivener-notice-unrecognised-writer-title = Este projeto foi guardado por um programa que o Skribisto não reconhece
import-scrivener-notice-unrecognised-writer-named = Identifica-se como «{ $creator }». É lido como um projeto Scrivener.
import-scrivener-notice-unrecognised-writer-body = Não se identifica. É lido como um projeto Scrivener.
import-scrivener-notice-invalid-item-identifier-title = { $count ->
    [one] 1 item do arquivador tem um identificador que não é válido
   *[other] { $count } itens do arquivador têm um identificador que não é válido
}
import-scrivener-notice-invalid-item-identifier-body = Esses identificadores são postos de lado. Os itens são mantidos.
import-scrivener-notice-items-without-identifier-title = { $count ->
    [one] 1 item do arquivador não tem identificador
   *[other] { $count } itens do arquivador não têm identificador
}
import-scrivener-notice-items-without-identifier-body = Não é possível encontrar os respetivos ficheiros, por isso chegam vazios.
import-scrivener-notice-duplicate-item-key-title = Dois itens do arquivador partilham um identificador: «{ $item }»
import-scrivener-notice-duplicate-item-key-body = O primeiro fica com os documentos. O segundo mantém o seu lugar e chega vazio.
import-scrivener-notice-lock-here-title = O Scrivener tem este projeto aberto neste computador
import-scrivener-notice-lock-here-body = Feche-o primeiro no Scrivener, para que todas as alterações fiquem guardadas.
import-scrivener-notice-lock-title = O Scrivener tinha este projeto aberto em «{ $host }»
import-scrivener-notice-lock-elsewhere-title = O Scrivener tinha este projeto aberto noutro computador
import-scrivener-notice-lock-body = Se ainda estiver aberto lá, feche-o primeiro: os ficheiros podem ainda não conter as últimas alterações. Se o Scrivener foi fechado há muito tempo, pode importá-lo tal como está.
import-scrivener-notice-archived-while-open-title = Este arquivo foi criado enquanto o Scrivener tinha o projeto aberto
import-scrivener-notice-archived-while-open-body = As alterações feitas depois desse momento podem não estar nele.
import-scrivener-notice-content-missing-notes = Faltam as notas de «{ $item }»
import-scrivener-notice-content-missing-synopsis = Falta a sinopse de «{ $item }»
import-scrivener-notice-content-missing-text = Falta o texto de «{ $item }»
import-scrivener-notice-content-missing-body = O ficheiro correspondente não está nesta cópia, por isso não é trazido nenhum texto. Um serviço de sincronização pode ainda estar a copiar o projeto.
import-scrivener-notice-content-missing-salvaged = O ficheiro correspondente não está nesta cópia, mas o índice de pesquisa do Scrivener ainda guardava as suas palavras. São mantidas como texto simples, talvez de uma versão mais antiga.
import-scrivener-notice-content-truncated-notes = As notas de «{ $item }» estão vazias no disco
import-scrivener-notice-content-truncated-synopsis = A sinopse de «{ $item }» está vazia no disco
import-scrivener-notice-content-truncated-text = O texto de «{ $item }» está vazio no disco
import-scrivener-notice-content-truncated-body = O Scrivener registou aqui texto, mas o ficheiro correspondente não contém nenhum: uma sincronização pode ter parado a meio. Não é trazido nenhum texto.
import-scrivener-notice-missing-media-title = Falta o ficheiro de «{ $item }»
import-scrivener-notice-missing-media-body = A imagem ou o ficheiro de pesquisa que ele contém não está nesta cópia.
import-scrivener-notice-sync-conflict-copies-title = { $count ->
    [one] Foi encontrada 1 cópia de sincronização de um documento
   *[other] Foram encontradas { $count } cópias de sincronização de documentos
}
import-scrivener-notice-sync-conflict-copies-body = { $missing ->
    [0] São trazidas como textos recuperados, numa pasta própria, para que as possa comparar com os documentos.
   *[other] São trazidas como textos recuperados, numa pasta própria. Algumas são cópias de documentos que já não estão no projeto.
}
import-scrivener-notice-recovered-unreferenced-text-title = { $count ->
    [one] Foi encontrado 1 documento que nenhum item do arquivador lista
   *[other] Foram encontrados { $count } documentos que nenhum item do arquivador lista
}
import-scrivener-notice-recovered-unreferenced-text-body = São trazidos como textos recuperados, numa pasta própria.
import-scrivener-notice-sync-debris-ignored-title = { $count ->
    [one] 1 cópia a mais deixada por um serviço de sincronização é posta de lado
   *[other] { $count } cópias a mais deixadas por um serviço de sincronização são postas de lado
}
import-scrivener-notice-sync-debris-ignored-body = O Scrivener também não as lê. Só os ficheiros do próprio projeto são lidos.
import-scrivener-notice-unrecognised-files-title = { $count ->
    [one] 1 ficheiro que não faz parte de um projeto Scrivener é posto de lado
   *[other] { $count } ficheiros que não fazem parte de um projeto Scrivener são postos de lado
}
import-scrivener-notice-examples = Por exemplo { $examples }.
import-scrivener-notice-symlink-not-followed-title = { $count ->
    [one] 1 ligação dentro do projeto não é seguida
   *[other] { $count } ligações dentro do projeto não são seguidas
}
import-scrivener-notice-unreadable-files-skipped-title = { $count ->
    [one] Não foi possível ler 1 ficheiro de que a importação não precisa
   *[other] Não foi possível ler { $count } ficheiros de que a importação não precisa
}
import-scrivener-notice-cloud-placeholders-skipped-title = { $count ->
    [one] 1 ficheiro ainda na nuvem é ignorado
   *[other] { $count } ficheiros ainda na nuvem são ignorados
}
import-scrivener-notice-cloud-placeholders-skipped-body = A importação não precisa deles.
import-scrivener-notice-no-text-found-title = { $count ->
    [one] 1 documento, e não contém texto
   *[other] { $count } documentos, e nenhum contém texto
}
import-scrivener-notice-no-text-found-body = Este projeto parece vazio. É importado tal como está.
import-scrivener-notice-metadata-unreadable-title = Não foi possível ler «{ $member }»
import-scrivener-notice-metadata-unreadable-body = O que o Scrivener lá guardava não é trazido.
import-scrivener-notice-dangling-keyword-title = { $count ->
    [one] 1 referência aponta para uma palavra-chave que o projeto não define
   *[other] { $count } referências apontam para palavras-chave que o projeto não define
}
import-scrivener-notice-dangling-label-title = { $count ->
    [one] 1 referência aponta para um rótulo que o projeto não define
   *[other] { $count } referências apontam para rótulos que o projeto não define
}
import-scrivener-notice-dangling-status-title = { $count ->
    [one] 1 referência aponta para um estado que o projeto não define
   *[other] { $count } referências apontam para estados que o projeto não define
}
import-scrivener-notice-dangling-section-type-title = { $count ->
    [one] 1 referência aponta para um tipo de secção que o projeto não define
   *[other] { $count } referências apontam para tipos de secção que o projeto não define
}
import-scrivener-notice-dangling-custom-field-title = { $count ->
    [one] 1 referência aponta para um campo de metadados personalizados que o projeto não define
   *[other] { $count } referências apontam para campos de metadados personalizados que o projeto não define
}
import-scrivener-notice-dangling-item-title = { $count ->
    [one] 1 referência aponta para um documento que o projeto não define
   *[other] { $count } referências apontam para documentos que o projeto não define
}
import-scrivener-notice-dangling-references-body = São deixadas de fora.
import-scrivener-notice-snapshots-not-imported-line = { $count ->
    [one] 1 instantâneo fica no seu projeto Scrivener: o Skribisto não traz instantâneos.
   *[other] { $count } instantâneos ficam no seu projeto Scrivener: o Skribisto não traz instantâneos.
}
import-scrivener-notice-writing-history-not-imported-line = { $days ->
    [one] O registo do Scrivener das palavras escritas em cada dia, que abrange 1 dia, não é trazido. O Skribisto inicia a sua própria contagem a partir da primeira gravação.
   *[other] O registo do Scrivener das palavras escritas em cada dia, que abrange { $days } dias, não é trazido. O Skribisto inicia a sua própria contagem a partir da primeira gravação.
}
import-scrivener-notice-title-placeholder-rejected-hint = O título nas definições do projeto é um marcador de posição de um modelo, por isso é proposto antes o nome do projeto.
import-scrivener-notice-author-placeholder-rejected-hint = O nome do autor nas definições do projeto é um marcador de posição de um modelo, por isso não é proposto nenhum.
import-scrivener-notice-title-from-project-hint-compile-settings = Este título é o indicado nas definições de compilação do projeto. Enquanto não for definido lá um título, o Scrivener mantém o nome que o projeto tinha quando foi criado.
import-scrivener-notice-title-from-project-hint-template = Este título é o do próprio modelo.
import-scrivener-notice-title-from-project-hint-folder-name = Este título é o nome do projeto.
import-scrivener-notice-title-from-project-hint-project-properties = Este título é o indicado nas propriedades do projeto.
import-scrivener-notice-author-from-project-hint = Este nome é o indicado nas definições do projeto. O autor de um modelo pode ter ficado lá, e seria impresso numa página de título.
import-scrivener-notice-language-not-stated-hint = O projeto não indica o seu idioma, por isso é proposto o idioma em que o Skribisto é apresentado. O idioma decide como as palavras são contadas e como as aspas são escritas.
import-scrivener-notice-language-from-script-hint = Proposto a partir do sistema de escrita do manuscrito ({ $tag }).

## Before you import: what the import does with the binder, or leaves behind
import-scrivener-structure-script-mode-draft-line = { $script ->
    [one] { $documents ->
        [one] O único documento do manuscrito está escrito em modo de guião. O Skribisto mantém as suas palavras, não a sua formatação de guião.
       *[other] { $script } dos { $documents } documentos do manuscrito está escrito em modo de guião. O Skribisto mantém as suas palavras, não a sua formatação de guião.
    }
   *[other] { $script } dos { $documents } documentos do manuscrito estão escritos em modo de guião. O Skribisto mantém as suas palavras, não a sua formatação de guião.
}
import-scrivener-structure-one-document-manuscript-line = { $count ->
    [one] Um documento do manuscrito contém os seus capítulos como títulos internos.
   *[other] { $count } documentos do manuscrito contêm os seus capítulos como títulos internos.
}
import-scrivener-structure-title-repeats-number-line = { $count ->
    [one] 1 título de capítulo ou de parte também inclui o seu número, por isso uma exportação pode imprimir o número duas vezes.
   *[other] { $count } títulos de capítulos ou de partes também incluem o seu número, por isso uma exportação pode imprimir o número duas vezes.
}
import-scrivener-structure-claim-overruled-line = { $count ->
    [one] 1 item fica um nível abaixo do que reivindica, porque um item acima dele já ocupa esse nível.
   *[other] { $count } itens ficam um nível abaixo do que reivindicam, porque um item acima deles já ocupa esse nível.
}
import-scrivener-structure-adopted-by-chapter-line = { $count ->
    [one] 1 documento que segue um capítulo torna-se o fim desse capítulo.
   *[other] { $count } documentos que seguem um capítulo tornam-se o fim desse capítulo.
}
import-scrivener-structure-looks-like-manuscript-line = { $count ->
    [one] 1 pasta de nível superior mantida nas notas parece manuscrito. Pode enviá-la para o manuscrito como outro livro.
   *[other] { $count } pastas de nível superior mantidas nas notas parecem manuscrito. Pode enviá-las para o manuscrito como outros livros.
}
import-scrivener-structure-generic-sheet-title-line = { $count ->
    [one] 1 entrada da bíblia da história tem o título de um modelo, como «Herói», que as menções vão procurar no texto.
   *[other] { $count } entradas da bíblia da história têm o título de um modelo, como «Herói», que as menções vão procurar no texto.
}
import-scrivener-structure-placeholder-title-line = { $count ->
    [one] 1 título apenas nomeava o seu nível e é deixado em branco: o arquivador e as exportações dão-lhe um número.
   *[other] { $count } títulos apenas nomeavam o seu nível e são deixados em branco: o arquivador e as exportações dão-lhes um número.
}
import-scrivener-structure-untitled-item-line = { $count ->
    [one] 1 item não tinha título no Scrivener e fica sem título.
   *[other] { $count } itens não tinham título no Scrivener e ficam sem título.
}
import-scrivener-structure-title-cleaned-line = { $count ->
    [one] 1 título continha quebras de linha ou tabulações, que são removidas.
   *[other] { $count } títulos continham quebras de linha ou tabulações, que são removidas.
}
import-scrivener-structure-own-text-moved-line = { $count ->
    [one] 1 pasta tem texto próprio, que passa para um primeiro documento dentro dela.
   *[other] { $count } pastas têm texto próprio, que passa para um primeiro documento dentro delas.
}
import-scrivener-structure-text-with-children-line = { $count ->
    [one] 1 documento com documentos dentro dele torna-se uma pasta.
   *[other] { $count } documentos com documentos dentro deles tornam-se pastas.
}
import-scrivener-structure-empty-folder-plain-line = { $count ->
    [one] 1 pasta vazia continua a ser uma pasta simples, para que nenhuma exportação imprima um cabeçalho que o Scrivener nunca imprimiu.
   *[other] { $count } pastas vazias continuam a ser pastas simples, para que nenhuma exportação imprima um cabeçalho que o Scrivener nunca imprimiu.
}
import-scrivener-structure-book-inside-book-line = { $count ->
    [one] 1 livro dentro de um livro torna-se uma parte.
   *[other] { $count } livros dentro de um livro tornam-se partes.
}
import-scrivener-structure-prologue-not-numbered-line = { $count ->
    [one] 1 prólogo ou epílogo fica fora da numeração dos capítulos.
   *[other] { $count } prólogos ou epílogos ficam fora da numeração dos capítulos.
}
import-scrivener-structure-matter-inside-book-line = { $count ->
    [one] 1 item de elementos pré-textuais ou pós-textuais de fora do manuscrito passa para dentro do livro como paratexto, deixado fora das exportações até o incluir.
   *[other] { $count } itens de elementos pré-textuais ou pós-textuais de fora do manuscrito passam para dentro do livro como paratexto, deixados fora das exportações até os incluir.
}
import-scrivener-structure-paratext-not-counted-line = { $documents ->
    [one] 1 documento do manuscrito torna-se paratexto (elementos pré-textuais ou pós-textuais): { $words ->
        [one] a sua { $words } palavra já não conta
       *[other] as suas { $words } palavras já não contam
    } como manuscrito.
   *[other] { $documents } documentos do manuscrito tornam-se paratexto (elementos pré-textuais ou pós-textuais): { $words ->
        [one] a sua { $words } palavra já não conta
       *[other] as suas { $words } palavras já não contam
    } como manuscrito.
}
import-scrivener-structure-document-notes-line = { $count ->
    [one] As notas de 1 documento tornam-se uma nota ao lado dele.
   *[other] As notas de { $count } documentos tornam-se notas ao lado deles.
}
import-scrivener-structure-draft-content-as-note-line = O texto próprio da pasta do manuscrito torna-se uma nota no topo do manuscrito.
import-scrivener-structure-story-bible-folder-line = { $count ->
    [one] 1 pasta de personagens ou de lugares torna-se uma etiqueta da bíblia da história.
   *[other] { $count } pastas de personagens ou de lugares tornam-se etiquetas da bíblia da história.
}
import-scrivener-structure-templates-created-line = { $count ->
    [one] 1 modelo de documento torna-se um modelo de nota.
   *[other] { $count } modelos de documento tornam-se modelos de notas.
}
import-scrivener-structure-template-nesting-flattened-line = { $count ->
    [one] 1 pasta de modelos dentro de outra é aplanada: cada texto que contém torna-se um modelo.
   *[other] { $count } pastas de modelos dentro de outras são aplanadas: cada texto que contêm torna-se um modelo.
}
import-scrivener-structure-template-kept-as-note-line = { $count ->
    [one] 1 modelo contém o que um modelo de nota não pode conter, e é mantido como nota.
   *[other] { $count } modelos contêm o que um modelo de nota não pode conter, e são mantidos como notas.
}
import-scrivener-structure-recovered-texts-line = { $count ->
    [one] 1 texto que nenhum item do arquivador listava é trazido numa pasta própria.
   *[other] { $count } textos que nenhum item do arquivador listava são trazidos numa pasta própria.
}
import-scrivener-structure-project-notes-line = { $count ->
    [one] 1 nota de projeto é trazida numa pasta própria.
   *[other] { $count } notas de projeto são trazidas numa pasta própria.
}
import-scrivener-structure-trash-brought-across-line = { $items ->
    [one] O lixo é trazido: 1 item, que pode restaurar a partir do lixo.
   *[other] O lixo é trazido: { $items } itens, { $writing } deles com escrita, que pode restaurar a partir do lixo.
}
import-scrivener-structure-not-compiled-line = { $count ->
    [one] 1 item que o Scrivener deixava fora da compilação fica também fora das exportações.
   *[other] { $count } itens que o Scrivener deixava fora da compilação ficam também fora das exportações.
}
import-scrivener-structure-media-in-draft-line = { $count ->
    [one] 1 imagem ou ficheiro no manuscrito torna-se uma nota, já que um livro não pode conter isso.
   *[other] { $count } imagens ou ficheiros no manuscrito tornam-se notas, já que um livro não pode conter isso.
}
import-scrivener-structure-left-out-line = { $count ->
    [one] 1 pasta de nível superior fica de fora, com { $items ->
        [one] 1 item
       *[other] { $items } itens
    }.
   *[other] { $count } pastas de nível superior ficam de fora, com { $items ->
        [one] 1 item
       *[other] { $items } itens
    }.
}
import-scrivener-structure-trash-left-out-line = { $items ->
    [one] O lixo é deixado para trás: 1 item.
   *[other] O lixo é deixado para trás: { $items } itens, { $writing } deles com escrita.
}
import-scrivener-structure-card-image-left-behind-line = { $count ->
    [one] 1 imagem de cartão num item do manuscrito é deixada para trás: seria impressa no livro.
   *[other] { $count } imagens de cartão em itens do manuscrito são deixadas para trás: seriam impressas no livro.
}
import-scrivener-structure-default-template-not-linked-line = { $count ->
    [one] O modelo de documento predefinido de 1 pasta não a pode acompanhar.
   *[other] Os modelos de documento predefinidos de { $count } pastas não as podem acompanhar.
}
import-scrivener-structure-page-break-not-kept-line = { $count ->
    [one] 1 definição «Quebra de página antes» não tem lugar no Skribisto e não é mantida.
   *[other] { $count } definições «Quebra de página antes» não têm lugar no Skribisto e não são mantidas.
}
import-scrivener-structure-compile-as-is-not-kept-line = { $count ->
    [one] 1 definição «Compilar tal como está» não tem equivalente no Skribisto e não é mantida.
   *[other] { $count } definições «Compilar tal como está» não têm equivalente no Skribisto e não são mantidas.
}

## What the import did with the labels, the status list and the dates
import-scrivener-metadata-status-categories-guessed-line = { $count ->
    [one] 1 estado que o Skribisto não reconhece recebe um tipo de acordo com o seu lugar na lista. Verifique-o em Definições ▸ Projeto ▸ Estados.
   *[other] { $count } estados que o Skribisto não reconhece recebem um tipo de acordo com o seu lugar na lista. Verifique-os em Definições ▸ Projeto ▸ Estados.
}
import-scrivener-metadata-dates-unreadable-line = { $count ->
    [one] Não foi possível ler as datas de 1 item: fica com a data da importação.
   *[other] Não foi possível ler as datas de { $count } itens: ficam com a data da importação.
}
import-scrivener-metadata-labels-became-tags-line = { $tags ->
    [one] Os rótulos tornam-se 1 etiqueta, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
   *[other] Os rótulos tornam-se { $tags } etiquetas, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
}
import-scrivener-metadata-statuses-became-ladder-line = { $rungs ->
    [one] A lista de estados torna-se o único estado do projeto, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
   *[other] A lista de estados torna-se os { $rungs } estados do projeto, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
}
import-scrivener-metadata-ladder-seeded-line = { $count ->
    [one] O projeto não tinha lista de estados, por isso começa com o estado predefinido do Skribisto.
   *[other] O projeto não tinha lista de estados, por isso começa com os { $count } estados predefinidos do Skribisto.
}
import-scrivener-metadata-label-renamed-line = O campo Rótulo chamava-se «{ $title }» no Scrivener. Esse nome está nos detalhes de cada etiqueta.
import-scrivener-metadata-status-renamed-line = O campo Estado chamava-se «{ $title }» no Scrivener. Esse nome está nos detalhes de cada estado.
import-scrivener-metadata-none-label-collapsed-line = { $count ->
    [one] O rótulo «{ $name }», mantido ao lado do rótulo vazio próprio do Scrivener com o mesmo nome, é lido como ausência de rótulo em 1 item.
   *[other] O rótulo «{ $name }», mantido ao lado do rótulo vazio próprio do Scrivener com o mesmo nome, é lido como ausência de rótulo em { $count } itens.
}
import-scrivener-metadata-none-status-collapsed-line = { $count ->
    [one] O estado «{ $name }», mantido ao lado do estado vazio próprio do Scrivener com o mesmo nome, é lido como ausência de estado em 1 item.
   *[other] O estado «{ $name }», mantido ao lado do estado vazio próprio do Scrivener com o mesmo nome, é lido como ausência de estado em { $count } itens.
}
import-scrivener-metadata-unnamed-label-line = Um rótulo sem nome recebe o nome do seu número, { $id }.
import-scrivener-metadata-unnamed-status-line = Um estado sem nome recebe o nome do seu número, { $id }.
import-scrivener-metadata-tag-name-disambiguated-line = { $count ->
    [one] 1 etiqueta recebe um novo nome, porque outra etiqueta já tem o seu.
   *[other] { $count } etiquetas recebem um novo nome, porque outras etiquetas já têm os seus.
}
import-scrivener-metadata-rung-names-disambiguated-line = { $count ->
    [one] 1 estado tem o mesmo nome que outro, por isso recebe um número a seguir ao nome. Verifique-o em Definições ▸ Projeto ▸ Estados.
   *[other] { $count } estados têm o mesmo nome que outro, por isso cada um recebe um número a seguir ao nome. Verifique-os em Definições ▸ Projeto ▸ Estados.
}
import-scrivener-metadata-unused-labels-skipped-line = { $count ->
    [one] 1 rótulo que nenhum documento usa não é importado.
   *[other] { $count } rótulos que nenhum documento usa não são importados.
}
import-scrivener-metadata-labels-without-row-line = { $count ->
    [one] O rótulo de 1 item sem lugar no novo projeto não é trazido.
   *[other] Os rótulos de { $count } itens sem lugar no novo projeto não são trazidos.
}
import-scrivener-metadata-statuses-without-row-line = { $count ->
    [one] O estado de 1 item sem lugar no novo projeto não é trazido.
   *[other] Os estados de { $count } itens sem lugar no novo projeto não são trazidos.
}

import-scrivener-metadata-unnamed-keyword-line = Uma palavra-chave sem nome recebe o nome do seu número, { $id }.
import-scrivener-metadata-keywords-without-row-line = { $count ->
    [one] As palavras-chave de 1 item sem lugar no novo projeto não são trazidas.
   *[other] As palavras-chave de { $count } itens sem lugar no novo projeto não são trazidas.
}
import-scrivener-metadata-custom-values-without-row-line = { $count ->
    [one] Os metadados personalizados de 1 item sem lugar no novo projeto não são trazidos.
   *[other] Os metadados personalizados de { $count } itens sem lugar no novo projeto não são trazidos.
}
import-scrivener-metadata-bookmarks-without-row-line = { $count ->
    [one] Os marcadores de 1 item sem lugar no novo projeto não são trazidos.
   *[other] Os marcadores de { $count } itens sem lugar no novo projeto não são trazidos.
}
import-scrivener-metadata-targets-without-row-line = { $count ->
    [one] O objetivo de 1 item sem lugar no novo projeto não é trazido.
   *[other] Os objetivos de { $count } itens sem lugar no novo projeto não são trazidos.
}

## What the import does with the keywords, custom metadata, collections, bookmarks, targets,
## the deadline, the auto-complete list and the icons. Each line is said twice: on the Review
## page before the import, then in its report, so it reads the same at both times. `$note`
## and `$folder` are the titles the import gives a note and its "From Scrivener" folder;
## `$day` is a day as YYYY-MM-DD.
import-scrivener-metadata-pov-hint-line = { $count ->
    [one] O rótulo ou valor de campo { $values } corresponde à entrada da bíblia da história { $matches }. Se indicar o ponto de vista, defina-o no Inspetor: a importação não liga nada.
   *[other] Os rótulos ou valores de campo { $values } correspondem às entradas da bíblia da história { $matches }. Se indicarem pontos de vista, defina-os no Inspetor: a importação não liga nada.
}
import-scrivener-metadata-checkbox-default-unknown-line = O campo de caixa de verificação «{ $field }» tem uma predefinição que o Skribisto não consegue ler, por isso não consegue saber que documentos estão marcados por predefinição. Só os documentos marcados um a um recebem a sua etiqueta.
import-scrivener-metadata-goals-in-other-unit-line = { $count ->
    [one] 1 meta está definida na outra unidade de contagem. É mantida e aparece quando mudar a unidade de contagem em Definições ▸ Projeto ▸ Estrutura.
   *[other] { $count } metas estão definidas na outra unidade de contagem. São mantidas e aparecem quando mudar a unidade de contagem em Definições ▸ Projeto ▸ Estrutura.
}
import-scrivener-metadata-counting-differs-line = O Scrivener contava o objetivo do rascunho sobre outros documentos que não os que o Skribisto conta. A meta do livro conta as cenas incluídas nas exportações.
import-scrivener-metadata-writing-days-not-read-line = Não é possível ler os dias de escrita «{ $raw }», por isso o Ritmo do livro conta todos os dias como dias de escrita. Altere-os lá.
import-scrivener-metadata-script-mode-rows-line = { $count ->
    [one] 1 item estava em modo de guião no Scrivener. O Skribisto mantém as suas palavras como prosa, sem a formatação de guião.
   *[other] { $count } itens estavam em modo de guião no Scrivener. O Skribisto mantém as suas palavras como prosa, sem a formatação de guião.
}
import-scrivener-metadata-keywords-became-tags-line = { $tags ->
    [one] 1 palavra-chave torna-se uma etiqueta, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
   *[other] { $tags } palavras-chave tornam-se etiquetas, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
}
import-scrivener-metadata-keyword-hierarchy-flattened-line = { $count ->
    [one] 1 palavra-chave estava dentro de outra no Scrivener. As etiquetas não têm hierarquia: os detalhes da sua etiqueta indicam onde ela estava.
   *[other] { $count } palavras-chave estavam dentro de outras no Scrivener. As etiquetas não têm hierarquia: os detalhes de cada etiqueta indicam onde estava a sua palavra-chave.
}
import-scrivener-metadata-fields-became-tags-line = { $fields ->
    [one] 1 campo de lista ou de caixa de verificação torna-se { $tags ->
        [one] { $tags } etiqueta
       *[other] { $tags } etiquetas
    }, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
   *[other] { $fields } campos de lista e de caixa de verificação tornam-se { $tags ->
        [one] { $tags } etiqueta
       *[other] { $tags } etiquetas
    }, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
}
import-scrivener-metadata-collections-became-tags-line = { $tags ->
    [one] 1 coleção torna-se uma etiqueta, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
   *[other] { $tags } coleções tornam-se etiquetas, em { $rows ->
        [one] { $rows } item
       *[other] { $rows } itens
    }.
}
import-scrivener-metadata-collection-order-lost-line = { $count ->
    [one] Uma etiqueta não guarda nenhuma ordem, por isso a ordem dos documentos de 1 coleção não é mantida.
   *[other] Uma etiqueta não guarda nenhuma ordem, por isso a ordem dos documentos de { $count } coleções não é mantida.
}
import-scrivener-metadata-bookmarks-became-links-line = { $count ->
    [one] 1 marcador torna-se uma ligação entre duas notas, ou entre uma cena e uma entrada da bíblia da história: veja Ligações e Elenco.
   *[other] { $count } marcadores tornam-se ligações entre notas, ou entre cenas e entradas da bíblia da história: veja Ligações e Elenco.
}
import-scrivener-metadata-goals-kept-line = { $count ->
    [one] 1 objetivo de documento torna-se uma meta.
   *[other] { $count } objetivos de documentos tornam-se metas.
}
import-scrivener-metadata-draft-target-kept-words-line = { $value ->
    [one] O objetivo do rascunho de 1 palavra torna-se a meta do livro.
   *[other] O objetivo do rascunho de { $value } palavras torna-se a meta do livro.
}
import-scrivener-metadata-draft-target-kept-characters-line = { $value ->
    [one] O objetivo do rascunho de 1 carácter torna-se a meta do livro.
   *[other] O objetivo do rascunho de { $value } caracteres torna-se a meta do livro.
}
import-scrivener-metadata-pace-planned-line = O prazo, { $day }, torna-se o Ritmo do livro, que planeia as palavras de cada dia de escrita que escolheu no Scrivener, a partir do dia da importação.
import-scrivener-metadata-pace-planned-day-off-line = O prazo, { $day }, torna-se o Ritmo do livro, que planeia as palavras de cada dia de escrita que escolheu no Scrivener, a partir do dia da importação. Tal como no Scrivener, o próprio dia do prazo é um dia de descanso.
import-scrivener-metadata-dictionary-words-line = { $count ->
    [one] 1 palavra de preenchimento automático vai para o dicionário pessoal do projeto, para não ser assinalada como erro ortográfico.
   *[other] { $count } palavras de preenchimento automático vão para o dicionário pessoal do projeto, para não serem assinaladas como erros ortográficos.
}
import-scrivener-metadata-listed-project-line = { $count ->
    [one] 1 pormenor do projeto está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] { $count } pormenores do projeto estão listados na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-listed-project-bookmarks-line = { $count ->
    [one] 1 marcador do projeto está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] { $count } marcadores do projeto estão listados na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-listed-replacements-line = { $count ->
    [one] 1 substituição de compilação está listada na nota «{ $note }» da pasta «{ $folder }». O Skribisto não a aplica.
   *[other] { $count } substituições de compilação estão listadas na nota «{ $note }» da pasta «{ $folder }». O Skribisto não as aplica.
}
import-scrivener-metadata-listed-auto-complete-line = { $count ->
    [one] 1 entrada de preenchimento automático com várias palavras está listada na nota «{ $note }» da pasta «{ $folder }».
   *[other] { $count } entradas de preenchimento automático com várias palavras estão listadas na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-listed-saved-searches-line = { $count ->
    [one] 1 pesquisa guardada está listada na nota «{ $note }» da pasta «{ $folder }».
   *[other] { $count } pesquisas guardadas estão listadas na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-listed-custom-metadata-line = { $count ->
    [one] 1 valor de metadados personalizados que uma etiqueta não pode conter está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] { $count } valores de metadados personalizados que uma etiqueta não pode conter estão listados na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-listed-targets-line = { $count ->
    [one] 1 objetivo de documento está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] { $count } objetivos de documentos estão listados na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-listed-bookmarks-line = { $count ->
    [one] 1 marcador que nenhuma ligação do Skribisto mostra está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] { $count } marcadores que nenhuma ligação do Skribisto mostra estão listados na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-targets-on-notes-line = { $count ->
    [one] 1 objetivo de documento está numa nota, cuja escrita nenhuma meta conta, por isso não se torna uma meta.
   *[other] { $count } objetivos de documentos estão em notas, cuja escrita nenhuma meta conta, por isso não se tornam metas.
}
import-scrivener-metadata-targets-on-split-documents-line = { $count ->
    [one] 1 objetivo de documento está num documento dividido nos seus títulos internos, cujos capítulos nenhuma meta conta em conjunto, por isso não se torna uma meta.
   *[other] { $count } objetivos de documentos estão em documentos divididos nos seus títulos internos, cujos capítulos nenhuma meta conta em conjunto, por isso não se tornam metas.
}
import-scrivener-metadata-bookmark-ends-without-row-line = { $count ->
    [one] 1 marcador leva a um documento sem lugar no novo projeto, como um modelo de documento. Está listado segundo o sítio onde o documento estava no Scrivener.
   *[other] { $count } marcadores levam a documentos sem lugar no novo projeto, como modelos de documento. Estão listados segundo o sítio onde os documentos estavam no Scrivener.
}
import-scrivener-metadata-unused-keywords-skipped-line = { $count ->
    [one] 1 palavra-chave que nenhum documento usa não é importada.
   *[other] { $count } palavras-chave que nenhum documento usa não são importadas.
}
import-scrivener-metadata-list-value-invalid-line = { $count ->
    [one] 1 valor do campo de lista «{ $field }» não corresponde a nenhuma das suas opções e não é importado.
   *[other] { $count } valores do campo de lista «{ $field }» não correspondem a nenhuma das suas opções e não são importados.
}
import-scrivener-metadata-undefined-custom-field-line = { $count ->
    [one] 1 valor pertence a um campo personalizado que o projeto já não define, e que o Scrivener também não mostra. Não é importado.
   *[other] { $count } valores pertencem a campos personalizados que o projeto já não define, e que o Scrivener também não mostra. Não são importados.
}
import-scrivener-metadata-collection-members-missing-line = { $count ->
    [one] 1 documento da coleção «{ $name }» não tem lugar no novo projeto, ou já não está no projeto Scrivener.
   *[other] { $count } documentos da coleção «{ $name }» não têm lugar no novo projeto, ou já não estão no projeto Scrivener.
}
import-scrivener-metadata-empty-collections-skipped-line = { $count ->
    [one] 1 coleção que não contém nenhum documento não é importada.
   *[other] { $count } coleções que não contêm nenhum documento não são importadas.
}
import-scrivener-metadata-bookmarks-unresolved-line = { $count ->
    [one] 1 marcador leva a um documento que o projeto Scrivener já não contém, e não é importado.
   *[other] { $count } marcadores levam a documentos que o projeto Scrivener já não contém, e não são importados.
}
import-scrivener-metadata-targets-in-pages-line = { $count ->
    [one] 1 objetivo de documento conta páginas, que o Skribisto não conta, por isso não se torna uma meta.
   *[other] { $count } objetivos de documentos contam páginas, que o Skribisto não conta, por isso não se tornam metas.
}
import-scrivener-metadata-targets-unreadable-line = { $count ->
    [one] 1 objetivo não é importado: a sua contagem não é um número, é inferior a zero ou é maior do que uma meta pode conter.
   *[other] { $count } objetivos não são importados: as suas contagens não são números, são inferiores a zero ou são maiores do que uma meta pode conter.
}
import-scrivener-metadata-draft-target-in-pages-line = { $value ->
    [one] O objetivo do rascunho de 1 página não se torna uma meta: o Skribisto conta palavras ou caracteres. Está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] O objetivo do rascunho de { $value } páginas não se torna uma meta: o Skribisto conta palavras ou caracteres. Está listado na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-words-line = { $value ->
    [one] O objetivo do rascunho de 1 palavra não se torna uma meta, porque o manuscrito forma vários livros, ou nenhum. Está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] O objetivo do rascunho de { $value } palavras não se torna uma meta, porque o manuscrito forma vários livros, ou nenhum. Está listado na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-draft-target-no-book-characters-line = { $value ->
    [one] O objetivo do rascunho de 1 carácter não se torna uma meta, porque o manuscrito forma vários livros, ou nenhum. Está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] O objetivo do rascunho de { $value } caracteres não se torna uma meta, porque o manuscrito forma vários livros, ou nenhum. Está listado na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-deadline-passed-line = O prazo, { $day }, já passou, por isso não cria nenhum Ritmo. Está listado na nota «{ $note }» da pasta «{ $folder }».
import-scrivener-metadata-deadline-no-writing-day-line = Não resta nenhum dia de escrita entre o dia da importação e o prazo, { $day }, por isso este não cria nenhum Ritmo. Está listado na nota «{ $note }» da pasta «{ $folder }».
import-scrivener-metadata-deadline-template-line = O prazo, { $day }, foi guardado no modelo, por isso não cria nenhum Ritmo. Está listado na nota «{ $note }» da pasta «{ $folder }».
import-scrivener-metadata-deadline-no-word-goal-line = O prazo, { $day }, não cria nenhum Ritmo, porque o rascunho não tem um objetivo de palavras para o qual planear. Está listado na nota «{ $note }» da pasta «{ $folder }».
import-scrivener-metadata-deadline-character-goal-line = O prazo, { $day }, não cria nenhum Ritmo, porque o objetivo do rascunho conta caracteres e um Ritmo conta palavras. Está listado na nota «{ $note }» da pasta «{ $folder }».
import-scrivener-metadata-deadline-no-book-line = O prazo, { $day }, não cria nenhum Ritmo, porque o manuscrito forma vários livros, ou nenhum. Está listado na nota «{ $note }» da pasta «{ $folder }».
import-scrivener-metadata-deadline-unreadable-line = Não é possível ler o prazo, por isso não cria nenhum Ritmo. Está listado tal como está escrito na nota «{ $note }» da pasta «{ $folder }».
import-scrivener-metadata-session-target-words-line = { $value ->
    [one] O objetivo de sessão de 1 palavra não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos. Está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] O objetivo de sessão de { $value } palavras não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos. Está listado na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-session-target-characters-line = { $value ->
    [one] O objetivo de sessão de 1 carácter não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos, em palavras. Está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] O objetivo de sessão de { $value } caracteres não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos, em palavras. Está listado na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-session-target-pages-line = { $value ->
    [one] O objetivo de sessão de 1 página não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos, em palavras. Está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] O objetivo de sessão de { $value } páginas não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos, em palavras. Está listado na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-words-line = { $value ->
    [one] O objetivo de sessão de 1 palavra, que o Scrivener calculou a partir do prazo e dos dias de escrita, não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos. Está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] O objetivo de sessão de { $value } palavras, que o Scrivener calculou a partir do prazo e dos dias de escrita, não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos. Está listado na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-characters-line = { $value ->
    [one] O objetivo de sessão de 1 carácter, que o Scrivener calculou a partir do prazo e dos dias de escrita, não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos, em palavras. Está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] O objetivo de sessão de { $value } caracteres, que o Scrivener calculou a partir do prazo e dos dias de escrita, não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos, em palavras. Está listado na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-session-target-calculated-pages-line = { $value ->
    [one] O objetivo de sessão de 1 página, que o Scrivener calculou a partir do prazo e dos dias de escrita, não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos, em palavras. Está listado na nota «{ $note }» da pasta «{ $folder }».
   *[other] O objetivo de sessão de { $value } páginas, que o Scrivener calculou a partir do prazo e dos dias de escrita, não é importado: no Skribisto, o objetivo da sessão de escrita é uma única definição para todos os projetos, em palavras. Está listado na nota «{ $note }» da pasta «{ $folder }».
}
import-scrivener-metadata-icons-dropped-line = { $count ->
    [one] 1 item tinha um ícone no arquivador, que não é trazido: o Skribisto mostra o ícone de cada item a partir do seu tipo.
   *[other] { $count } itens tinham ícones no arquivador, que não são trazidos: o Skribisto mostra o ícone de cada item a partir do seu tipo.
}

## What converting the prose did, as the import's report says it. `$count` is how many texts,
## or how many of a thing, as each line says; `$example` quotes the project.
import-scrivener-prose-unreadable-line = { $count ->
    [one] Não foi possível ler 1 texto. Chega vazio.
   *[other] Não foi possível ler { $count } textos. Chegam vazios.
}
import-scrivener-prose-comments-unreadable-line = { $count ->
    [one] Não foi possível ler os comentários e as notas de rodapé ligadas de 1 texto. As palavras que eles marcavam são mantidas.
   *[other] Não foi possível ler os comentários e as notas de rodapé ligadas de { $count } textos. As palavras que eles marcavam são mantidas.
}
import-scrivener-prose-read-as-plain-text-line = { $count ->
    [one] 1 texto estava guardado como texto simples em vez de texto formatado, e é lido como tal.
   *[other] { $count } textos estavam guardados como texto simples em vez de texto formatado, e são lidos como tal.
}
import-scrivener-prose-inline-note-unmatched-line = { $count ->
    [one] 1 marca de nota em linha não faz par com nenhuma outra e é mantida como texto.
   *[other] { $count } marcas de notas em linha não fazem par com nenhuma outra e são mantidas como texto.
}
import-scrivener-prose-style-unresolved-line = { $count ->
    [one] 1 marca de estilo indica um estilo que o projeto não define. É removida, e o texto mantém a sua formatação.
   *[other] { $count } marcas de estilo indicam um estilo que o projeto não define. São removidas, e o texto mantém a sua formatação.
}
import-scrivener-prose-styles-lost-in-export-line = { $count ->
    [one] 1 parágrafo pode ser um título que perdeu o seu estilo quando o Scrivener 3 exportou o projeto para o seu formato mais antigo, e é trazido como um parágrafo comum. Verifique-o, ou importe o projeto a partir do qual ele foi exportado.
   *[other] { $count } parágrafos podem ser títulos que perderam o seu estilo quando o Scrivener 3 exportou o projeto para o seu formato mais antigo, e são trazidos como parágrafos comuns. Verifique-os, ou importe o projeto a partir do qual eles foram exportados.
}
import-scrivener-prose-comment-anchor-without-entry-line = { $count ->
    [one] 1 comentário ou nota de rodapé tinha perdido o seu conteúdo no Scrivener. As palavras que marcava são mantidas, sem ele.
   *[other] { $count } comentários ou notas de rodapé tinham perdido o seu conteúdo no Scrivener. As palavras que marcavam são mantidas, sem eles.
}
import-scrivener-prose-comment-entry-without-anchor-line = { $count ->
    [one] 1 comentário já não tinha lugar no seu texto e é associado ao primeiro parágrafo.
   *[other] { $count } comentários já não tinham lugar no seu texto e são associados ao primeiro parágrafo.
}
import-scrivener-prose-footnote-entry-without-anchor-line = { $count ->
    [one] 1 nota de rodapé ligada já não tinha lugar no seu texto e é citada no fim dele.
   *[other] { $count } notas de rodapé ligadas já não tinham lugar no seu texto e são citadas no fim dele.
}
import-scrivener-prose-range-comment-reading-line = { $count ->
    [one] 1 comentário de um Scrivener mais antigo foi colocado segundo outra leitura da sua posição. Verifique se está sobre as palavras certas.
   *[other] { $count } comentários de um Scrivener mais antigo foram colocados segundo outra leitura da sua posição. Verifique se estão sobre as palavras certas.
}
import-scrivener-prose-range-comment-reading-markers-counted-line = { $count ->
    [one] 1 comentário de um Scrivener mais antigo foi colocado contando as marcas das suas notas em linha. Verifique se está sobre as palavras certas.
   *[other] { $count } comentários de um Scrivener mais antigo foram colocados contando as marcas das suas notas em linha. Verifique se estão sobre as palavras certas.
}
import-scrivener-prose-range-comment-reading-pictures-not-counted-line = { $count ->
    [one] 1 comentário de um Scrivener mais antigo foi colocado sem contar as suas imagens. Verifique se está sobre as palavras certas.
   *[other] { $count } comentários de um Scrivener mais antigo foram colocados sem contar as suas imagens. Verifique se estão sobre as palavras certas.
}
import-scrivener-prose-range-comment-approximate-line = { $count ->
    [one] 1 comentário de um Scrivener mais antigo não corresponde exatamente a nenhuma leitura da sua posição, e pode estar desviado alguns caracteres.
   *[other] { $count } comentários de um Scrivener mais antigo não correspondem exatamente a nenhuma leitura da sua posição, e podem estar desviados alguns caracteres.
}
import-scrivener-prose-document-link-unresolved-line = { $count ->
    [one] 1 ligação aponta para um documento que a importação não trouxe. As suas palavras ficam.
   *[other] { $count } ligações apontam para documentos que a importação não trouxe. As suas palavras ficam.
}
import-scrivener-prose-linked-picture-not-read-line = { $count ->
    [one] 1 imagem ligada a um ficheiro do seu disco, «{ $example }», não foi trazida.
   *[other] { $count } imagens ligadas a ficheiros do seu disco, como «{ $example }», não foram trazidas.
}
import-scrivener-prose-heading-repeats-title-line = { $count ->
    [one] 1 texto começa com um cabeçalho que repete o seu título, por isso uma exportação pode imprimir o título duas vezes.
   *[other] { $count } textos começam com um cabeçalho que repete o seu título, por isso uma exportação pode imprimir o título duas vezes.
}
import-scrivener-prose-markdown-as-text-line = { $count ->
    [one] 1 parágrafo escrito em Markdown é mantido tal como está.
   *[other] { $count } parágrafos escritos em Markdown são mantidos tal como estão.
}
import-scrivener-prose-citation-keys-as-text-line = { $count ->
    [one] 1 parágrafo contém chaves de citação para um programa de bibliografia, mantidas como texto.
   *[other] { $count } parágrafos contêm chaves de citação para um programa de bibliografia, mantidas como texto.
}
import-scrivener-prose-placeholders-in-exported-text-line = { $count ->
    [one] 1 marcador de posição do Scrivener, { $example }, é mantido como texto simples no que exportar. Substitua-o à mão.
   *[other] { $count } marcadores de posição do Scrivener, como { $example }, são mantidos como texto simples no que exportar. Substitua-os à mão.
}
import-scrivener-prose-placeholders-in-other-text-line = { $count ->
    [one] 1 marcador de posição do Scrivener, { $example }, é mantido como texto simples num texto que não é exportado.
   *[other] { $count } marcadores de posição do Scrivener, como { $example }, são mantidos como texto simples em textos que não são exportados.
}
import-scrivener-prose-ornament-lines-kept-line = { $count ->
    [one] 1 linha centrada de ornamentos pode marcar uma quebra de cena e é mantida tal como foi escrita.
   *[other] { $count } linhas centradas de ornamentos podem marcar quebras de cena e são mantidas tal como foram escritas.
}
import-scrivener-prose-long-document-line = { $count ->
    [one] 1 texto é muito longo e pode demorar a abrir.
   *[other] { $count } textos são muito longos e podem demorar a abrir.
}
import-scrivener-prose-pictures-without-description-line = { $count ->
    [one] 1 imagem não tem descrição para os leitores que não a podem ver. Pode acrescentar uma no editor.
   *[other] { $count } imagens não têm descrição para os leitores que não as podem ver. Pode acrescentar uma no editor.
}
import-scrivener-prose-struck-text-compiled-away-line = { $count ->
    [one] O texto rasurado em 1 texto é mantido. A compilação do Scrivener deixava-o fora do livro, mas uma exportação do Skribisto imprime-o.
   *[other] O texto rasurado em { $count } textos é mantido. A compilação do Scrivener deixava-o fora do livro, mas uma exportação do Skribisto imprime-o.
}
import-scrivener-prose-compile-deleted-style-printed-line = { $count ->
    [one] O texto no estilo «{ $example }», que a compilação do Scrivener eliminava, é mantido como prosa, e uma exportação do Skribisto imprime-o.
   *[other] O texto em { $count } estilos que a compilação do Scrivener eliminava, como «{ $example }», é mantido como prosa, e uma exportação do Skribisto imprime-o.
}
import-scrivener-prose-compile-raw-style-printed-line = { $count ->
    [one] O texto no estilo «{ $example }», que a compilação do Scrivener transmitia como código para outro programa, é mantido como prosa, e uma exportação do Skribisto imprime-o.
   *[other] O texto em { $count } estilos que a compilação do Scrivener transmitia como código para outro programa, como «{ $example }», é mantido como prosa, e uma exportação do Skribisto imprime-o.
}
import-scrivener-prose-left-out-after-document-line = { $count ->
    [one] 1 fragmento de um ficheiro danificado, depois do seu fim, é deixado de fora.
   *[other] { $count } fragmentos de ficheiros danificados, depois do seu fim, são deixados de fora.
}
import-scrivener-prose-left-out-header-or-footer-line = { $count ->
    [one] 1 cabeçalho ou rodapé de página é deixado de fora: não faz parte do texto.
   *[other] { $count } cabeçalhos e rodapés de página são deixados de fora: não fazem parte do texto.
}
import-scrivener-prose-left-out-list-label-picture-line = { $count ->
    [one] 1 imagem usada como marca de lista é deixada de fora.
   *[other] { $count } imagens usadas como marcas de lista são deixadas de fora.
}
import-scrivener-prose-left-out-line = { $count ->
    [one] 1 fragmento de um ficheiro que não pertence a nenhum texto é deixado de fora.
   *[other] { $count } fragmentos de ficheiros que não pertencem a nenhum texto são deixados de fora.
}
import-scrivener-prose-undecodable-characters-line = { $count ->
    [one] 1 carácter não pôde ser descodificado e aparece como uma marca de substituição (�) onde estava.
   *[other] { $count } caracteres não puderam ser descodificados e aparecem como marcas de substituição (�) onde estavam.
}
import-scrivener-prose-inline-notes-merged-line = { $count ->
    [one] 1 nota em linha que o Scrivener tinha dividido onde a sua formatação mudava é novamente unida.
   *[other] { $count } notas em linha que o Scrivener tinha dividido onde a sua formatação mudava são novamente unidas.
}
import-scrivener-prose-inline-notes-joined-across-paragraphs-line = { $count ->
    [one] 1 nota em linha escrita um parágrafo de cada vez é unida numa única nota.
   *[other] { $count } notas em linha escritas um parágrafo de cada vez são unidas em notas de vários parágrafos.
}
import-scrivener-prose-blank-notes-dropped-line = { $count ->
    [one] 1 nota em linha vazia é deixada de fora.
   *[other] { $count } notas em linha vazias são deixadas de fora.
}
import-scrivener-prose-annotations-became-comments-line = { $count ->
    [one] 1 anotação em linha tornou-se um comentário.
   *[other] { $count } anotações em linha tornaram-se comentários.
}
import-scrivener-prose-annotation-only-document-kept-line = { $count ->
    [one] 1 documento continha apenas uma anotação. As suas palavras são mantidas como prosa.
   *[other] { $count } documentos continham apenas uma anotação. As suas palavras são mantidas como prosa.
}
import-scrivener-prose-preserve-formatting-passages-line = { $count ->
    [one] 1 passagem marcada com «Preservar formatação» chega como texto comum.
   *[other] { $count } passagens marcadas com «Preservar formatação» chegam como texto comum.
}
import-scrivener-prose-nested-note-unwrapped-line = { $count ->
    [one] 1 nota dentro de um comentário ou de uma nota de rodapé é mantida como texto, onde estava.
   *[other] { $count } notas dentro de comentários ou de notas de rodapé são mantidas como texto, onde estavam.
}
import-scrivener-prose-style-not-carried-line = { $count ->
    [one] O estilo «{ $example }» não tem equivalente no Skribisto. O seu texto mantém as suas palavras.
   *[other] { $count } estilos, como «{ $example }», não têm equivalente no Skribisto. O seu texto mantém as suas palavras.
}
import-scrivener-prose-document-links-dropped-line = { $count ->
    [one] 1 ligação para outro documento não tem lugar no Skribisto. As suas palavras ficam.
   *[other] { $count } ligações para outros documentos não têm lugar no Skribisto. As suas palavras ficam.
}
import-scrivener-prose-document-links-kept-line = { $count ->
    [one] 1 ligação para outro documento é mantida como referência.
   *[other] { $count } ligações para outros documentos são mantidas como referências.
}
import-scrivener-prose-other-links-kept-as-text-line = { $count ->
    [one] 1 ligação para outro projeto ou para um ficheiro é mantida como texto.
   *[other] { $count } ligações para outros projetos ou para ficheiros são mantidas como texto.
}
import-scrivener-prose-soft-breaks-split-line = { $count ->
    [one] 1 quebra de linha dentro de um parágrafo inicia agora um parágrafo próprio.
   *[other] { $count } quebras de linha dentro de parágrafos iniciam agora parágrafos próprios.
}
import-scrivener-prose-typed-indents-removed-line = { $count ->
    [one] 1 avanço escrito no início de um parágrafo é removido.
   *[other] { $count } avanços escritos no início de parágrafos são removidos.
}
import-scrivener-prose-left-indents-not-kept-line = { $count ->
    [one] 1 parágrafo recuado em relação à margem perde o seu avanço.
   *[other] { $count } parágrafos recuados em relação à margem perdem o seu avanço.
}
import-scrivener-prose-blank-lines-removed-line = { $count ->
    [one] 1 parágrafo vazio entre dois outros é removido.
   *[other] { $count } parágrafos vazios entre outros são removidos.
}
import-scrivener-prose-verse-stanza-gaps-lost-line = { $count ->
    [one] 1 linha em branco entre as estrofes de um poema é removida, por isso as estrofes ficam seguidas.
   *[other] { $count } linhas em branco entre as estrofes de um poema são removidas, por isso as estrofes ficam seguidas.
}
import-scrivener-prose-blank-lines-became-scene-breaks-line = { $count ->
    [one] 1 linha em branco entre parágrafos tornou-se uma quebra de cena.
   *[other] { $count } linhas em branco entre parágrafos tornaram-se quebras de cena.
}
import-scrivener-prose-ornaments-became-scene-breaks-line = { $count ->
    [one] 1 linha de ornamentos tornou-se uma quebra de cena.
   *[other] { $count } linhas de ornamentos tornaram-se quebras de cena.
}
import-scrivener-prose-table-first-row-header-line = { $count ->
    [one] 1 tabela começa agora com uma linha de cabeçalho.
   *[other] { $count } tabelas começam agora com uma linha de cabeçalho.
}
import-scrivener-prose-table-cells-joined-line = { $count ->
    [one] 1 célula de tabela continha vários parágrafos, agora unidos num só.
   *[other] { $count } células de tabela continham vários parágrafos, agora unidos num só.
}
import-scrivener-prose-formatting-lost-colour-line = { $count ->
    [one] A cor do texto é removida em 1 texto.
   *[other] A cor do texto é removida em { $count } textos.
}
import-scrivener-prose-formatting-lost-highlight-line = { $count ->
    [one] O realce é removido em 1 texto.
   *[other] O realce é removido em { $count } textos.
}
import-scrivener-prose-formatting-lost-small-caps-line = { $count ->
    [one] Os versaletes aparecem como letras comuns em 1 texto.
   *[other] Os versaletes aparecem como letras comuns em { $count } textos.
}
import-scrivener-prose-formatting-lost-all-caps-line = { $count ->
    [one] O texto apresentado em maiúsculas aparece tal como foi escrito em 1 texto.
   *[other] O texto apresentado em maiúsculas aparece tal como foi escrito em { $count } textos.
}
import-scrivener-prose-formatting-lost-hidden-line = { $count ->
    [one] O texto oculto é deixado de fora em 1 texto.
   *[other] O texto oculto é deixado de fora em { $count } textos.
}
import-scrivener-prose-formatting-lost-line = { $count ->
    [one] A formatação para a qual o Skribisto não tem lugar é removida em 1 texto.
   *[other] A formatação para a qual o Skribisto não tem lugar é removida em { $count } textos.
}
import-scrivener-prose-justified-text-not-kept-line = { $count ->
    [one] 1 parágrafo justificado segue antes o estilo da exportação.
   *[other] { $count } parágrafos justificados seguem antes o estilo da exportação.
}
import-scrivener-prose-comment-colours-lost-line = { $count ->
    [one] 1 comentário perde a sua cor.
   *[other] { $count } comentários perdem a sua cor.
}
import-scrivener-prose-control-characters-removed-line = { $count ->
    [one] 1 carácter de controlo invisível é removido.
   *[other] { $count } caracteres de controlo invisíveis são removidos.
}
import-scrivener-prose-note-on-heading-lost-line = { $count ->
    [one] 1 nota de rodapé ou comentário num título de capítulo não pôde ficar lá: um comentário passa para o primeiro parágrafo do capítulo, uma nota de rodapé é deixada de fora.
   *[other] { $count } notas de rodapé ou comentários em títulos de capítulos não puderam ficar lá: um comentário passa para o primeiro parágrafo do capítulo, uma nota de rodapé é deixada de fora.
}
import-scrivener-prose-media-file-copied-line = { $count ->
    [one] 1 ficheiro de pesquisa que o Skribisto não consegue abrir foi copiado para junto do novo projeto. A sua nota indica onde.
   *[other] { $count } ficheiros de pesquisa que o Skribisto não consegue abrir foram copiados para junto do novo projeto. As suas notas indicam onde.
}
import-scrivener-prose-media-file-not-carried-line = { $count ->
    [one] 1 ficheiro de pesquisa que o Skribisto não consegue abrir fica no seu projeto Scrivener. A sua nota indica qual é.
   *[other] { $count } ficheiros de pesquisa que o Skribisto não consegue abrir ficam no seu projeto Scrivener. As suas notas indicam quais são.
}

## When the import fails
import-scrivener-error-refused = { $refusal } Nada foi escrito.
import-scrivener-error-question = O projeto coloca agora uma pergunta a que a sua revisão não respondeu. Reveja-o novamente e depois importe. Nada foi escrito.
import-scrivener-error-not-accepted = O projeto levanta uma questão que ainda não aceitou. Reveja-o novamente e depois importe. Nada foi escrito.
import-scrivener-error-changed = O projeto mudou depois de o ter revisto. Reveja-o novamente e depois importe. Nada foi escrito.
import-scrivener-error-choice = A estrutura escolhida na revisão já não se adequa a este projeto. Reveja-o novamente e depois importe. Nada foi escrito.

export-title = Exportar
export-close = Fechar
export-scope-label = O quê
export-format-label = Formato
export-style-label = Estilo
export-path-label = Ficheiro
export-browse = Procurar…
export-show-non-exportable = Mostrar não exportáveis
export-choose-empty = Nenhum projeto para escolher.
export-cancel = Cancelar
export-export = Exportar
export-save-dialog-title = Exportar para um ficheiro
export-section-what = O que exportar
export-section-style = Estilo
export-section-destination = Destino
export-custom-selection = Seleção personalizada
export-selected-count = { $count } selecionados
export-chip-chapters-none = Capítulos: nenhum
export-chip-chapters-numbered = Capítulos: numerados
export-chip-chapters-title = Capítulos: apenas o título
export-chip-chapters-both = Capítulos: número + título
export-chip-scene-break-glyph = Quebra de cena: { $glyph }
export-chip-scene-break-blank = Quebra de cena: linha em branco
export-chip-scene-break-none = Quebra de cena: nenhuma
export-chip-major-break-glyph = Quebra principal: { $glyph }
export-chip-major-break-blank = Quebra principal: linha em branco
export-chip-major-break-none = Quebra principal: nenhuma
export-chip-major-break-same = Ambos os níveis de quebra iguais
export-chip-spacing-single = Espaçamento: simples
export-chip-spacing-onehalf = Espaçamento: 1½
export-chip-spacing-double = Espaçamento: duplo
export-chip-notes-included = Notas incluídas
export-chip-notes-excluded = Notas excluídas
export-format-docx = Word
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Texto
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Substituir o ficheiro existente?
export-overwrite-text = «{ $name }» já existe. Substituí-lo?
export-progress-title = A exportar…
export-cancelled = Exportação cancelada
export-done = { $count ->
    [one] { $count } item exportado
   *[other] { $count } itens exportados
}
export-open-file = Abrir
export-show-in-folder = Mostrar na pasta
export-error-title = Não foi possível exportar
export-error-details = Detalhes

backup-progress-start = A iniciar…
backup-progress-retention = A limpar as cópias de segurança antigas…
backup-progress-done = Concluído
backup-progress-destination = Destino { $i } de { $n }
backup-details = Detalhes
backup-issues-title = Problemas da cópia de segurança
backup-failed-title = A cópia de segurança falhou
backup-complete-prune-warning = Cópia de segurança concluída ({ $ok } guardadas, { $skipped } já atualizadas). Não foi possível remover algumas cópias de segurança antigas

search = Pesquisar
search-query-placeholder = Pesquisar…
search-replace-placeholder = Substituir por…
search-replace-toggle = Mostrar ou ocultar a substituição
search-replace-all = Substituir tudo
search-opt-case = Maiúsculas/minúsculas
search-opt-whole-word = Palavra inteira
search-opt-diacritics = Distinguir acentos
search-scope-body = Corpo
search-scope-title = Título
search-scope-synopsis = Sinopse
search-scope-label = Rótulo
search-facet-book = Livros
search-facet-part = Partes
search-facet-chapter = Capítulos
search-facet-scene = Cenas
search-facet-note = Notas
search-facet-folder = Pastas
search-tip-case = Maiúsculas/minúsculas: tratar maiúsculas e minúsculas como diferentes, por isso «Elena» e «elena» são correspondências distintas.
search-tip-whole-word = Palavra inteira: corresponder apenas a palavras completas, por isso «cat» não é encontrado dentro de «category».
search-tip-diacritics = Distinguir acentos: tratar as letras acentuadas como distintas, por isso «cafe» não corresponde a «café».
search-tip-body = Corpo: pesquisar a prosa das cenas e das notas.
search-tip-title = Título: pesquisar os títulos dos itens do arquivador.
search-tip-synopsis = Sinopse: pesquisar o resumo de cada item de escrita.
search-tip-label = Rótulo: pesquisar a nota curta mostrada por baixo do título de um item do arquivador.
search-tip-comment = Comentários: pesquisar o texto das conversas de comentários e das suas respostas. Um comentário é sobre o manuscrito e não faz parte dele, por isso tem o seu próprio interruptor, e uma substituição deixa as correspondências em comentários desmarcadas até que as marque.
search-tip-book = Livros: o contentor do livro e os seus marcadores de início e de fim.
search-tip-part = Partes: divisões ao nível da parte.
search-tip-chapter = Capítulos: os capítulos, seja qual for a forma como o projeto os guarda.
search-tip-scene = Cenas: os itens que contêm a sua prosa.
search-tip-note = Notas: notas livres.
search-tip-folder = Pastas: pastas simples de organização e separadores.
search-tip-paratext = Um texto que pertence ao livro mas não à sua história: um prefácio, uma dedicatória, um posfácio. Nunca é contado no manuscrito.
search-tip-preserve-case = Preservar maiúsculas: a substituição adota a capitalização que encontrou, por isso «ELENA» passa a «MARTA» e «Elena» passa a «Marta».
search-tip-replace = Substituir: mostrar o campo de substituição e Substituir tudo.
search-error = A pesquisa falhou: { $message }
search-no-matches = Sem correspondências
search-count =
    { $matches ->
        [one] { $matches } correspondência
       *[other] { $matches } correspondências
    } em { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } correspondência
       *[other] { $matches } correspondências
    } em { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    } (apenas os primeiros resultados)
search-occurrences = { $count }
search-field-body = Corpo
search-field-title = Título
search-field-synopsis = Sinopse
search-field-label = Rótulo
search-field-epigraph = Epígrafe
search-field-comment = Comentário
search-field-comment-reply = Resposta
search-field-footnote = Nota de rodapé
search-include-in-replace = Incluir em Substituir tudo
search-collapse-all = Recolher todos os resultados
search-replace-here = Substituir isto
search-dismiss = Retirar dos resultados
search-undo-dismiss = Repor o último resultado retirado
search-replace-nothing = (nada)
search-replace-confirm-title = Substituir todas as correspondências?
search-replace-confirm-text =
    Substituir { $occurrences ->
        [one] { $occurrences } ocorrência
       *[other] { $occurrences } ocorrências
    } de «{ $query }» por «{ $replacement }» em { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }? Pode anular esta ação a partir da notificação.
search-replace-done-title = Substituição concluída
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } ocorrência substituída
       *[other] { $occurrences } ocorrências substituídas
    } em { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } ocorrência substituída
       *[other] { $occurrences } ocorrências substituídas
    } em { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }. { $skipped ->
        [one] { $skipped } campo ignorado
       *[other] { $skipped } campos ignorados
    } (alterados desde a pesquisa).
search-replace-undo = Anular
search-replace-failed-title = A substituição falhou
search-replace-skipped-title = Nada substituído
search-replace-skipped-body =
    { $fields ->
        [one] Este texto mudou desde a pesquisa, por isso foi deixado intacto.
       *[other] { $fields } destes textos mudaram desde a pesquisa, por isso foram deixados intactos.
    } Pesquise novamente para ver onde estão agora as palavras.
search-replace-undo-failed-title = A anulação falhou
search-preview = Pré-visualização
search-preview-empty = Selecione um resultado para o pré-visualizar aqui
search-preview-no-prose = Esta correspondência não tem texto editável
search-preview-prompt = Para ver uma pré-visualização aqui, faça uma pesquisa.
search-preview-open-search = Pesquisar no projeto
search-preview-footnote-prompt = Esta correspondência está no texto de uma nota de rodapé. Abra-a no painel Notas de rodapé para a ver e editar.
search-preview-open-footnotes = Abrir as notas de rodapé

find-placeholder = Procurar neste documento…
find-count = { $current } de { $total }
find-no-results = Sem resultados
find-close = Fechar a procura
find-previous = Correspondência anterior (Shift+Enter)
find-next = Correspondência seguinte (Enter)
find-opt-case = Maiúsculas/minúsculas
find-opt-whole-word = Palavra inteira
find-replace-toggle = Mostrar ou ocultar substituição (Ctrl+R)
find-replace-placeholder = Substituir por…
find-replace = Substituir
find-replace-all = Substituir tudo
find-preserve-case = Preservar maiúsculas

dict-download-title = A transferir { $name }…
dict-download-done = { $name } instalado
dict-download-failed = Não foi possível transferir { $name }: { $error }
dict-removed = { $name } removido
dict-accept-first = Aceite a licença antes de transferir { $name }
settings-dict-tab-installed = Instalado
settings-dict-tab-get-more = Obter mais
settings-dict-tab-personal = Palavras pessoais
dict-installed-empty = Ainda não foram encontrados dicionários neste computador.
dict-get-more-search = Pesquisar idiomas
dict-system-badge = no seu sistema
dict-unusable-badge = inutilizável
dict-download-button = Transferir
dict-downloading = A transferir…
dict-installed-label = Instalado
dict-remove = Remover
dict-view-license = Ver licença
dict-approx-size = ~{ $size }
dict-personal-empty = Ainda não há palavras pessoais neste projeto.
dict-personal-add = Adicionar
dict-personal-placeholder = Adicionar uma palavra…
dict-missing-toast = { $count ->
    [one] Este projeto usa { $count } dicionário que não tem instalado
   *[other] Este projeto usa { $count } dicionários que não tem instalados
}
dict-missing-action = Obter dicionários
dict-license-title = Licença de { $name }
dict-license-accept = Aceitar e transferir
dict-license-cancel = Cancelar
dict-license-close = Fechar
dict-add-button = Adicionar dicionário…
dict-add-title = Adicionar um dicionário
dict-add-close = Fechar
dict-add-name = Nome
dict-add-name-placeholder = p. ex., O meu dicionário de latim
dict-add-code = Código da língua
dict-add-code-placeholder = p. ex., la ou fr-FR-x-custom
dict-add-code-hint = Um código curto à sua escolha. É o que escolhe como idioma de um documento.
dict-add-aff = Ficheiro de afixos (.aff)
dict-add-dic = Lista de palavras (.dic)
dict-add-submit = Adicionar
dict-add-cancel = Cancelar
dict-add-name-required = Dê um nome ao dicionário.
dict-add-code-required = Introduza um código de idioma.
dict-add-code-invalid = Use apenas letras, dígitos e - _ .
dict-add-code-reserved = Esse é um código de dicionário integrado. Escolha outro.
dict-add-file-required = Escolha um ficheiro.
dict-add-file-missing = Esse ficheiro não existe.
dict-add-code-taken = Um dicionário para esse código já está instalado.
dict-add-done = { $name } adicionado
dict-add-unusable = Estes ficheiros não são um dicionário utilizável: { $error }
dict-add-failed = Não foi possível adicionar o dicionário: { $error }
inspector-export = Exportar
inspector-exportable = Incluir nas exportações
ctx-number = &Numerar este capítulo
ctx-unnumber = &Não numerar este capítulo
inspector-numbering = Numeração
inspector-numbered = Numerado
inspector-numbered-tip = Este capítulo ocupa o seu lugar na numeração do livro. Desative-o para um prólogo, um epílogo ou um interlúdio.
inspector-numbered-tip-more = Um capítulo não numerado permanece no livro exatamente como estava: o seu cabeçalho, a sua prosa e a sua contagem de palavras não são alterados. Simplesmente não imprime nenhum número, nem gasta um: o capítulo a seguir a um prólogo é o capítulo um, não o capítulo dois. Deixá-lo fora da exportação é outro interruptor, acima, e esse retira o capítulo completamente do livro.
inspector-apply-to-children = Aplicar aos filhos
inspector-milestone = Data do marco
inspector-milestone-none = Sem data
inspector-milestone-clear = Limpar
inspector-tags = Etiquetas
inspector-aliases = Também conhecido como
inspector-dict-language = Idioma
inspector-apply-language-to-children = Aplicar o idioma aos filhos
settings-page-language = Idioma
settings-field-dict-language = Idiomas
dict-tradeoff-hint = Cada idioma adicional aceita mais palavras, por isso são detetados menos erros.
lang-inherit-hint = Herdado: esta cena usa os idiomas do livro ou do projeto.
lang-pill-list = Idiomas
lang-pill-add = Adicionar um idioma
lang-pill-remove = Remover { $name }
lang-pill-mute = Desligar a verificação ortográfica para { $name }
lang-pill-unmute = Ligar a verificação ortográfica para { $name }

settings-page-personal-dictionary = Dicionário pessoal
settings-user-dict-desc = As palavras que adicionar aqui são consideradas corretamente escritas em toda a aplicação e nunca mais são assinaladas.
settings-user-dict-add-placeholder = Adicionar uma palavra…
settings-user-dict-add = Adicionar palavra
settings-user-dict-search = Filtrar palavras
settings-user-dict-count = { $count ->
    [one] { $count } palavra
   *[other] { $count } palavras
}
settings-user-dict-rename = Mudar o nome
settings-user-dict-remove = Remover
settings-user-dict-empty = Ainda não há palavras. Adicione palavras que o verificador ortográfico deve ignorar, ou importe uma lista.
settings-user-dict-import = Importar…
settings-user-dict-export = Exportar…
settings-user-dict-import-tip = Adicione palavras de um ficheiro de texto simples, uma palavra por linha. As palavras existentes são mantidas.
settings-user-dict-export-tip = Guardar a lista inteira num ficheiro de texto simples, uma palavra por linha.
settings-user-dict-txt-filter = Ficheiros de texto
settings-user-dict-duplicate = Já está no dicionário
settings-user-dict-imported = { $count ->
    [one] { $count } palavra importada
   *[other] { $count } palavras importadas
} ({ $duplicates } já presentes).
settings-user-dict-import-failed = Não foi possível ler a lista de palavras: { $error }
settings-user-dict-exported = { $count ->
    [one] { $count } palavra guardada.
   *[other] { $count } palavras guardadas.
}
settings-user-dict-export-failed = Não foi possível guardar a lista de palavras: { $error }

settings-page-text-replacements = Substituições de texto
settings-text-repl-desc = Substituir abreviaturas pelo texto completo à medida que escreve: «btw» passa a «by the way» assim que escreve um espaço ou um sinal de pontuação.
settings-text-repl-enable = Usar substituições de texto neste projeto
settings-text-repl-disabled-hint = Ative isto para definir abreviaturas que se expandem à medida que escreve.
settings-text-repl-add = Adicionar uma regra
settings-text-repl-trigger-placeholder = Abreviatura
settings-text-repl-replacement-placeholder = Em que se transforma
settings-text-repl-added = «{ $trigger }» adicionada
settings-text-repl-duplicate = «{ $trigger }» já tem uma regra
settings-text-repl-filter = Filtrar regras
settings-text-repl-count = { $n ->
    [one] 1 regra
   *[other] { $n } regras
}
settings-text-repl-row-enabled = Usar esta regra
settings-text-repl-delete = Eliminar a regra de { $trigger }
settings-text-repl-deleted = Regra de «{ $trigger }» eliminada
settings-text-repl-empty = Ainda não há regras.
settings-text-repl-csv-filter = Ficheiros CSV
settings-text-repl-import = Importar…
settings-text-repl-export = Exportar…
settings-text-repl-imported = { $added } importadas, { $skipped } ignoradas
settings-text-repl-exported = { $n ->
    [one] 1 regra exportada
   *[other] { $n } regras exportadas
}

editor-menu-no-suggestions = Sem sugestões
editor-menu-add-to-dictionary = Adicionar «{ $word }» ao dicionário
editor-menu-add-words-to-dictionary = Adicionar as palavras selecionadas ao dicionário
editor-dict-added = «{ $word }» adicionada ao seu dicionário.
editor-dict-added-multi = { $count ->
    [one] { $count } palavra adicionada ao seu dicionário.
   *[other] { $count } palavras adicionadas ao seu dicionário.
}
toast-undo = Anular

undo-superseded-title = Esse passo já não é o último
undo-superseded-body = Entretanto, outra coisa mudou neste projeto. Anular desfaria essa alteração em vez desta, por isso não foi feito. Nada do que este passo fez foi revertido. Use Editar ▸ Anular para recuar no histórico por si.
undo-failed = A anulação falhou: { $error }

menu-edit = &Editar
menu-edit-undo = &Anular
menu-edit-redo = &Refazer
menu-edit-undo-target = &Anular { $target }
menu-edit-redo-target = &Refazer { $target }
shortcut-name-edit-undo = Anular
shortcut-name-edit-redo = Refazer
undo-target-typing = a escrita
undo-target-project = a última alteração a este projeto
prose-history-reset-title = Histórico de escrita reposto
prose-history-reset-body = { $count ->
    [one] Uma cena aberta foi restaurada a partir de um estado anterior, por isso o seu histórico de escrita já não se aplica.
   *[other] { $count } cenas abertas foram restauradas a partir de um estado anterior, por isso o seu histórico de escrita já não se aplica.
}
undo-target-trash = o envio para o lixo
undo-target-restore = o restauro a partir do lixo
undo-target-delete-forever = a eliminação definitiva
undo-target-replace-all = a substituição em todo o projeto
undo-target-import = a importação de documentos
undo-target-duplicate = a duplicação
undo-target-move = a deslocação
undo-target-merge = a fusão de duas cenas
undo-target-split = a divisão de uma cena
undo-target-promote = a mudança de tipo
undo-target-tidy-titles = a arrumação dos títulos dos capítulos
undo-target-import-tags = a importação de etiquetas
undo-target-import-templates = a importação de modelos de notas
undo-target-create = a criação
undo-target-remove = a eliminação
undo-target-rename = a mudança de nome
undo-target-edit = essa edição
menu-edit-find = &Localizar…
menu-edit-find-next = Localizar s&eguinte
menu-edit-find-prev = Localizar an&terior
menu-edit-replace = Localizar e su&bstituir…
menu-edit-find-in-project = Localizar no pro&jeto…
menu-edit-replace-in-project = Substituir &no projeto…
undo-frozen = Anular («Sempre em frente» está ativo)
redo-frozen = Refazer («Sempre em frente» está ativo)
menu-edit-cut = &Cortar
menu-edit-copy = Co&piar
menu-edit-paste = C&olar
menu-edit-paste-plain = Colar se&m formatação
menu-edit-select-all = &Selecionar tudo
shortcut-name-edit-cut = Cortar
shortcut-name-edit-copy = Copiar
shortcut-name-edit-paste = Colar
shortcut-name-edit-paste-plain = Colar sem formatação
shortcut-name-edit-select-all = Selecionar tudo

titlebar-spellcheck-on = A verificação ortográfica está ativa. Clique para parar a verificação (F7)
titlebar-spellcheck-off = A verificação ortográfica está desativada. Clique para voltar a verificar (F7)
menu-spellcheck = &Verificar ortografia
menu-comments = &Comentários
settings-page-spellcheck = Verificação ortográfica
settings-group-spellcheck = Verificação ortográfica
settings-spellcheck-enabled = Verificar a ortografia enquanto escrevo
settings-spellcheck-hint = Sublinha as palavras que nenhum dicionário instalado conhece. Desativar isto para toda a verificação, em todos os projetos, até que a volte a ativar. Para deixar de verificar apenas um idioma, desmarque-o no campo Idioma do projeto ou de um item.

menu-trash = &Lixo
trash-title = Lixo
trash-empty-state = O lixo está vazio.
trash-empty-button = Esvaziar o lixo…
trash-restore = &Restaurar
trash-restore-to = Restaurar &para…
trash-delete-forever = &Eliminar definitivamente
trash-restored-ok = { $count } restaurados.
trash-restore-error = Não foi possível restaurar: { $error }
trash-restore-orphaned = O local original deste item desapareceu. Escolha onde restaurá-lo.
trash-restore-no-project = Nenhum projeto está aberto, por isso não há nada para restaurar.
trash-delete-no-project = Nenhum projeto está aberto, por isso não há nada para eliminar.
trash-empty-confirm-title = Esvaziar o lixo?
trash-empty-confirm-text = { $count ->
    [one] { $count } entrada do lixo será eliminada permanentemente.
   *[other] { $count } entradas do lixo serão eliminadas permanentemente.
} Saem do lixo de vez. Anular ainda as pode recuperar até fazer outra coisa.
trash-emptied-title = Lixo esvaziado
trash-emptied-body = Tudo o que estava no lixo foi eliminado permanentemente.
trash-empty-no-project = Nenhum projeto está aberto, por isso não há lixo para esvaziar.
trash-delete-forever-confirm-title = Eliminar definitivamente?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } item será eliminado permanentemente.
   *[other] { $count } itens serão eliminados permanentemente.
} Saem do lixo de vez. Anular ainda os pode recuperar até fazer outra coisa.
trash-deleted-title = Eliminado definitivamente
trash-deleted-body = { $count } eliminados permanentemente.
trash-undo = Anular
trash-restore-picker-title = Restaurar para…
trash-restore-to-confirm-title = Restaurar aqui?
trash-restore-to-confirm-text = Restaurar «{ $item }» em «{ $destination }»?
trash-restore-picker-restore-here = Restaurar aqui
trash-restore-picker-cancel = Cancelar
trash-restore-picker-empty = Ainda não há arquivadores. Crie um primeiro.
trash-banner-title = Este item está no lixo
trash-banner-description = Não aparecerá na estrutura nem nas exportações até o restaurar.
trash-banner-restore = Restaurar…
trash-tab-tooltip = No lixo

settings-styles-sheet-yes = Sim
settings-styles-sheet-no = Não
settings-styles-sheet-font = Tipo de letra
settings-styles-sheet-indent = Avanço da primeira linha
settings-styles-sheet-para-spacing = Espaçamento de parágrafos
settings-styles-sheet-page = Tamanho da página
settings-styles-sheet-margins = Margens (sup./dir./inf./esq.)
settings-styles-sheet-title-page = Página de título do livro
settings-styles-sheet-heading-language = Idioma dos cabeçalhos
settings-styles-sheet-auto = Segue o texto
settings-styles-sheet-digits = Algarismos
settings-styles-sheet-direction = Direção do texto
settings-styles-sheet-formats = Formatos
settings-styles-sheet-all-formats = Todos

settings-styles-editor-group = Editor de estilo
settings-styles-editor-missing = Esse estilo já não está disponível.

settings-styles-page-letter = Carta (Letter)
settings-styles-digits-western = Ocidentais (0–9)
settings-styles-digits-eastern-arabic = Árabes orientais (٠–٩)
settings-styles-direction-ltr = Da esquerda para a direita
settings-styles-direction-rtl = Da direita para a esquerda

format-dock-title = Formato
format-panel-empty = Coloque o cursor numa cena, nota ou sinopse para ver as opções de formatação.
format-group-marks = Texto
format-group-block = Parágrafo
format-group-lists = Listas
format-group-tables = Tabela
format-group-breaks = Quebras de cena
format-superscript = Superior à linha
format-subscript = Inferior à linha
format-link = Ligação…
link-dialog-insert-title = Inserir ligação
link-dialog-edit-title = Editar ligação
link-dialog-text-label = Texto
link-dialog-text-placeholder = as palavras que o leitor vê
link-dialog-url-label = Liga a
link-dialog-url-placeholder = example.com
link-dialog-insert = Inserir
link-dialog-apply = Aplicar
link-dialog-cancel = Cancelar
link-dialog-remove = Remover ligação
link-scheme-refused = Só é possível abrir ligações web e de e-mail. { $url } foi deixado intacto.
menu-format-link = &Ligação…
format-clear = Limpar a formatação
format-heading = Nível de título
format-heading-normal = Texto normal
format-heading-1 = Título 1
format-heading-2 = Título 2
format-heading-3 = Título 3
format-heading-4 = Título 4
format-heading-5 = Título 5
format-heading-6 = Título 6
format-align-left = Alinhar à esquerda
format-align-center = Centrar
format-direction-rtl = Parágrafo da direita para a esquerda
format-blockquote = Citação em bloco
format-list-bullet = Lista com marcas
format-list-numbered = Lista numerada
format-indent = Aumentar avanço
format-outdent = Diminuir avanço
format-table-insert = Inserir tabela
format-table-row-above = Inserir linha acima
format-table-row-below = Inserir linha abaixo
format-table-col-before = Inserir coluna antes
format-table-col-after = Inserir coluna depois
format-table-row-delete = Eliminar linha
format-table-col-delete = Eliminar coluna
format-table-remove = Remover tabela

menu-format-marks-bold = &Negrito
menu-format-marks-italic = It&álico
menu-format-marks-underline = &Sublinhado
menu-format-marks-strike = &Rasurado
menu-format-marks-superscript = S&uperior à linha
menu-format-marks-subscript = In&ferior à linha
menu-format-marks-clear = Lim&par a formatação
menu-format-heading = &Título
menu-format-heading-normal = &Texto normal
menu-format-heading-1 = Título &1
menu-format-heading-2 = Título &2
menu-format-heading-3 = Título &3
menu-format-heading-4 = Título &4
menu-format-heading-5 = Título &5
menu-format-heading-6 = Título &6
menu-format-alignment = &Alinhamento
menu-format-align-left = &Alinhar à esquerda
menu-format-align-center = &Centrar
menu-format-direction = Dir&eção
menu-format-direction-auto = &Automática
menu-format-direction-ltr = &Esquerda para a direita
menu-format-direction-rtl = &Direita para a esquerda
menu-format-blockquote = &Citação em bloco
menu-format-lists = L&istas
menu-format-list-bullet = &Lista com marcas
menu-format-list-numbered = L&ista numerada
menu-format-indent = &Aumentar avanço
menu-format-outdent = &Diminuir avanço
menu-format-table = Ta&bela
menu-format-table-insert = &Inserir tabela
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = I&nserir linha acima
menu-format-table-row-below = In&serir linha abaixo
menu-format-table-col-before = Inse&rir coluna antes
menu-format-table-col-after = Ins&erir coluna depois
menu-format-table-row-delete = Eliminar &linha
menu-format-table-col-delete = Eliminar c&oluna
menu-format-table-remove = Re&mover tabela

menu-format-typo-marks = &Marca de formatação
menu-format-typo-nbsp = &Espaço sem quebra
menu-format-typo-nbhyphen = &Hífen sem quebra
menu-format-typo-soft-hyphen = Hífen &opcional
menu-format-typo-nnbsp = Espaço est&reito sem quebra
menu-format-typo-zwsp = Espaço de largura &zero
menu-format-typo-word-joiner = &Unificador de palavras


menu-about = &Acerca do Skribisto…
about-title = Acerca do Skribisto
about-version = Versão { $version }
about-update-available = A versão { $version } está disponível
about-update-see-notes = Ver o que mudou
about-update-flathub = O Flathub mantém esta cópia atualizada.
about-update-distro = A sua distribuição mantém esta cópia atualizada.
about-tagline = Uma aplicação de escrita de romances para ficção longa, escrita em Rust com o toolkit Teksilo.
about-license = Distribuído sob a Licença Pública Geral GNU, versão 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Fechar

native-menu-about = Acerca de { $app }
native-menu-hide = Ocultar { $app }
native-menu-quit = Sair de { $app }
native-menu-settings = Definições…
native-menu-window = Janela
native-menu-minimize = Minimizar
native-menu-zoom = Zoom

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Janela { $n })
window-title-empty = { $app }

first-run-window-title = Boas-vindas
first-run-title = Configurar { $app }
first-run-body = O { $app } guarda as suas definições separadamente do Skribisto, por isso começa vazio. As suas preferências, projetos recentes, dicionários e disposição das janelas podem ser copiados agora.
first-run-from = Copiar de
first-run-to = Copiar para
first-run-copy-note = Nada é movido nem removido. O Skribisto mantém todas as suas próprias definições e continua a funcionar exatamente como antes.
first-run-import = Importar definições
first-run-start-fresh = Começar de novo
first-run-import-failed = Algumas definições não puderam ser importadas: { $error }

settings-page-punctuation = Pontuação predefinida
settings-group-punctuation = Pontuação inteligente
settings-page-work-punctuation = Pontuação
settings-punctuation-override = Dar a este projeto as suas próprias regras de pontuação
settings-punctuation-override-hint = Desativado: o projeto segue a preferência da aplicação. Estas regras viajam dentro do .skrib, por isso um coautor que abra o ficheiro escreve com a mesma tipografia.
settings-punctuation-dashes = Transformar -- num traço meia-risca, --- num travessão
settings-punctuation-ellipsis = Transformar ... em reticências
settings-punctuation-quotes = Curvar as aspas e os apóstrofos
settings-quote-style = Aspas
settings-quote-style-locale = Predefinição do idioma
settings-quote-style-curly = “Duplas”
settings-quote-style-curly-single = ‘Simples’
settings-quote-style-guillemets = «Guillemets»
settings-quote-style-low-high = „Baixas-altas“
settings-punctuation-spacing = Espaço antes de ; : ! ?
settings-punctuation-spacing-hint = A tipografia francesa coloca um espaço estreito sem quebra antes de ; ! ? e um espaço inteiro antes de :. Aplica-se apenas ao texto escrito em francês. O espaço dentro das aspas angulares « » vem com as próprias aspas angulares.
settings-punctuation-sample = O seu idioma dá
settings-punctuation-app-hint = O que todos os projetos fazem, salvo se tiverem regras próprias em Projeto ▸ Pontuação.
settings-punctuation-dialogue = Abrir com um travessão de diálogo um parágrafo escrito como "- "
settings-punctuation-dialogue-hint = Para línguas que marcam a fala com um travessão em vez de aspas: francês, espanhol, russo e outras. Só atua no início de um parágrafo.

statusbar-go-to = Ir para…
go-to-placeholder = Pesquisar no arquivador
go-to-no-matches = Nenhum item corresponde a essa pesquisa.
menu-go-to = &Ir para…

settings-page-distraction-free-themes = Temas sem distrações
settings-themes-builtin = Temas integrados
settings-themes-builtin-badge = Integrado
settings-themes-user = Os meus temas
settings-themes-editor-group = Editar tema
settings-themes-editor-empty = Escolha um tema em «Os meus temas» para o editar.
settings-themes-use = Usar
settings-themes-duplicate = Duplicar
settings-themes-edit = Editar
settings-themes-delete = Eliminar
settings-themes-export = Exportar…
settings-themes-import = Importar…
settings-themes-copy-suffix = cópia
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema importado
settings-themes-import-failed = Não foi possível importar esse tema
settings-themes-exported = Tema exportado
settings-themes-export-failed = Não foi possível exportar esse tema
settings-themes-low-contrast = baixo contraste
settings-themes-low-contrast-band = o realce esconde o texto
settings-themes-field-name = Nome
settings-themes-field-paper = Página
settings-themes-field-ink = Texto
settings-themes-field-general = Fundo
settings-themes-field-widget-text = Texto da barra de controlo
settings-themes-field-caret-band = Realce em torno do cursor

statusbar-focus-settings = Definições do modo sem distrações
statusbar-focus-manage-themes = Gerir temas…

comments-title = Comentários
comments-document-title = Este documento
comments-empty-project = Ainda não há comentários neste projeto.
comments-empty-document = Não há comentários neste documento.
comments-filter-all = Todos
comments-filter-open = Abertos
comments-filter-resolved = Resolvidos
comments-filter-orphaned = Órfãos
comments-status-open = Aberto
comments-status-resolved = Resolvido
comments-status-orphaned = Perdeu o texto associado
comments-orphan-snippet = (o texto comentado já não existe)
comments-status-unplaced = Sem posição de texto
comments-unplaced-snippet = (não ancorado a nenhum texto)
comments-reply-count = { $count ->
    [one] { $count } resposta
   *[other] { $count } respostas
}
comments-menu-resolve = Resolver
comments-menu-reopen = Reabrir
comments-menu-delete = Eliminar
comments-sort-document = Por ordem do documento
comments-sort-newest = Mais recentes primeiro
comments-sort-reader = Agrupados por leitor
comments-unsigned-author = Sem assinatura
comments-agreement = { $count ->
    [one] { $count } leitor
   *[other] { $count } leitores
}
comments-menu-add = Adicionar um comentário
comments-menu-add-paragraph = Comentar este parágrafo
overview-col-comments = Comentários
overview-col-total-comments = Total de comentários
comments-card-placeholder = Escreva um comentário…
comments-card-unknown-author = Autor desconhecido
comments-unsigned-toast = Os seus comentários não estão assinados: não está definido nenhum nome para si neste computador.
comments-unsigned-action = Definir o seu nome
comments-card-reply = Responder
comments-card-reply-placeholder = Responder…
comments-card-actions = Ações do comentário
comments-reply-actions = Ações da resposta
comments-menu-delete-reply = Eliminar a resposta
comments-menu-delete-all = Eliminar todos os comentários aqui
comments-deleted-toast = Comentário eliminado
comments-reply-deleted-toast = Resposta eliminada
comments-deleted-all-toast = { $count ->
    [one] { $count } comentário eliminado
   *[other] { $count } comentários eliminados
}
comments-undo = Anular

analysis-segment = Análise
analysis-scope-book = A analisar este livro
analysis-run = Executar a análise
analysis-stale = Alterado desde a última análise
analysis-not-run = Ainda não analisado.
analysis-running = A ler o manuscrito…
analysis-failed = A análise não pôde terminar.
analysis-no-scenes = Ainda não há cenas neste livro.

analysis-shape = Forma

analysis-arrivals = Chegadas
analysis-arrivals-explainer = Como o texto chegou a este projeto enquanto esteve aberto. Diz por que via vieram os caracteres, e nada sobre quem os escreveu: um escritor que redige noutro sítio e cola, colou; um que dita, ditou. Não há aqui nenhum número a atingir.
analysis-arrivals-scope = Todo este projeto, não apenas este livro.
analysis-arrivals-session = Desde que o abriu. Fechar o projeto recomeça a contagem.
analysis-arrivals-nothing = Ainda não chegou nenhum texto nesta sessão.
analysis-arrivals-typed = Escrito ao teclado
analysis-arrivals-pasted = Colado
analysis-arrivals-dictated = Ditado
analysis-arrivals-imported = Importado
analysis-arrivals-programmatic = Inserido automaticamente
analysis-arrivals-count = { $count ->
    [one] { $count } carácter
   *[other] { $count } caracteres
}
analysis-arrivals-none = nenhum

analysis-words-per-scene = Palavras por cena
analysis-median-words = A cena mediana deste livro tem { $count ->
    [one] { $count } palavra.
   *[other] { $count } palavras.
}
analysis-median-line = Mediana: { $count ->
    [one] { $count } palavra
   *[other] { $count } palavras
}
analysis-dialogue = Diálogo
analysis-dialogue-unsupported = O diálogo ainda não é medido para este idioma.

analysis-footnote-words = Palavras das notas de rodapé
analysis-footnote-words-count = { $count ->
    [one] { $count } palavra está nas notas de rodapé deste livro, contada à parte do total do manuscrito.
   *[other] { $count } palavras estão nas notas de rodapé deste livro, contadas à parte do total do manuscrito.
}
analysis-footnote-words-pending = A contar as notas de rodapé…

analysis-ignore-empty = Ignorar os textos ainda sem nada escrito
analysis-empty-hidden = { $count ->
        [one] { $count } texto vazio oculto.
       *[other] { $count } textos vazios ocultos.
    }
analysis-all-texts-empty = Todos os textos deste livro ainda estão vazios.

binder-filter-count = { $shown } de { $total } mostrados
binder-filter-clear = Limpar
binder-filter-none = Nada corresponde a «{ $query }».

settings-paratext-intro = Os elementos pré-textuais e pós-textuais com que um novo projeto pode começar. Cada estrutura pertence a uma tradição editorial, e os títulos das suas páginas estão escritos na língua dessa tradição. Pode mudar-lhes o nome livremente depois de criar o projeto.
settings-paratext-structures = Estruturas
settings-paratext-broken = Não foi possível ler
settings-paratext-edit = Editar
settings-paratext-duplicate = Duplicar
settings-paratext-delete = Eliminar
settings-paratext-new = Nova estrutura
settings-paratext-save = Guardar
settings-paratext-editor-hint = Um nome, as páginas a criar antes do manuscrito e as páginas a criar depois dele. Onde ficam depende de si: são itens comuns assim que o projeto existe.


image-insert = I&nserir imagem…
image-no-project = Abra um projeto antes de inserir uma imagem.
image-choose-title = Escolher uma imagem
image-filter-label = Imagens
image-large-title = Esta é uma imagem grande
image-large-text =
    { $name } tem { $megapixels } megapíxeis ({ $width }×{ $height }).
    Mantê-la como está guarda o seu ficheiro original no projeto: viaja
    com cada cópia de segurança e cada exportação. Otimizar guarda antes uma cópia
    mais pequena, com no máximo 2560 píxeis no lado maior.
image-large-keep = Manter original
image-large-downscale = Otimizar
image-large-remember = Fazer isto a partir de agora, não voltar a perguntar
image-not-recorded = A imagem foi guardada, mas não pôde ser registada no projeto.
image-describe-title = Descrever a imagem
image-describe-explain = O que a imagem mostra, para um leitor que não a pode ver. Não faz parte do manuscrito: nunca é contada, pesquisada nem exportada como prosa.
image-describe-placeholder = um farol contra um céu cinzento
image-resize-title = Redimensionar a imagem
image-resize-explain = Uma percentagem do tamanho com que é mostrada agora. 100 deixa-a como está.
image-resize-invalid = Introduza um número entre 1 e 1000.
image-menu-describe = &Descrever a imagem…
image-menu-resize = &Redimensionar a imagem…
image-menu-reset-size = &Tamanho original

cover-choose = &Capa do livro…
cover-clear = Remo&ver a capa
cover-choose-title = Escolher uma capa
cover-set = A capa está definida.
cover-cleared = A capa foi removida. A imagem ainda está no projeto.
card-label = Rótulo
card-exportable = Exportado
card-numbered = Numerado
card-created = Criado
card-modified = Modificado
card-yes = sim
card-no = não
card-type = Tipo
card-position = Posição
card-children = Filhos
card-goal = Objetivo
card-words = Palavras
card-synopsis = Sinopse
card-point-of-view = Ponto de vista
card-aliases = Também conhecido como

export-orphan-footnotes-title = Notas de rodapé sem referência
export-comments-dropped =
    { $count ->
        [one] Um comentário não pôde ser colocado no texto exportado e foi deixado de fora.
       *[other] { $count } comentários não puderam ser colocados no texto exportado e foram deixados de fora.
    }
export-orphan-footnotes =
    { $count ->
        [one] Uma nota de rodapé já não é referida em nenhum lugar do manuscrito. O seu texto não aparecerá no livro exportado.
       *[other] { $count } notas de rodapé já não são referidas em nenhum lugar do manuscrito. O seu texto não aparecerá no livro exportado.
    }


menu-footnotes = &Notas de rodapé
footnotes-title = Notas de rodapé
footnotes-insert = In&serir nota de rodapé
footnotes-empty = Ainda não há notas de rodapé. Coloque o cursor numa cena e use + acima, ou Ctrl+Alt+F.
footnotes-filter-all = Todas
footnotes-filter-document = Este documento
footnotes-filter-orphaned = Órfãs
footnotes-orphaned = Já nada aponta para esta nota
footnotes-untitled-home = Sem título
footnotes-body-placeholder = a nota em si
footnotes-insert-tooltip = Inserir uma nota de rodapé no cursor (Ctrl+Alt+F)
footnotes-actions = Ações da nota de rodapé
footnotes-delete = &Eliminar a nota e a sua referência
footnotes-no-project = Abra um projeto antes de inserir uma nota de rodapé.
footnotes-no-caret = Coloque o cursor no texto de uma cena para inserir uma nota de rodapé.
footnotes-not-created = A nota de rodapé não pôde ser adicionada ao projeto.
footnotes-deleted-toast = Nota eliminada
footnotes-undo-delete = Anular

import-document-title = Importar documentos
import-document-close = Fechar
import-document-step-files = Ficheiros
import-document-step-review = Revisão
import-document-step-destination = Destino
import-document-step-reconcile = Fundir
import-document-reconcile-hint = Alguns destes itens já existem no seu projeto. Indique o que deve acontecer a cada um.
import-document-reconcile-all-new = Nada neste ficheiro corresponde ao seu projeto: todos os itens serão adicionados como novos.
import-document-duplicate-returns = Estes ficheiros são mais do que uma cópia do mesmo manuscrito a regressar. Importe-os um de cada vez: a correspondência só alinha uma cópia devolvida com o seu projeto, por isso importar várias em conjunto acrescentaria uma segunda cópia do livro em vez de a fundir.
import-document-hunk-added = Aceitar este novo parágrafo
import-document-hunk-removed = Descartar este parágrafo
import-document-hunk-changed = Aceitar esta reescrita
import-document-col-stray-prose = O seu texto
import-document-stray-as-paratext = Manter como paratexto
import-document-stray-discard = Descartar o texto
import-document-col-current = No seu projeto
import-document-col-incoming = Neste ficheiro
import-document-col-status = Estado
import-document-col-action = O que fazer
import-document-status-identical = Igual
import-document-status-editor-edited = Alterado pelo leitor
import-document-status-you-edited = Alterado por si
import-document-status-conflict = Alterado por ambos
import-document-status-different = Diferente
import-document-status-new = Novo
import-document-status-missing = Não está neste ficheiro
import-document-status-moved = Movido
import-document-action-comments-only = Apenas comentários
import-document-action-take-import = Usar esta versão
import-document-action-keep-current = Manter a minha
import-document-action-create-new = Adicionar como novo
import-document-action-ignore = Ignorar
import-document-compare = Comparar
import-document-compare-legend = A sua versão comparada com a deste ficheiro. Apenas para leitura.
import-document-compare-close = Fechar
import-document-drop-title = Largue os documentos aqui
import-document-drop-hint = Markdown (.md), Word (.docx), OpenDocument (.odt) e texto simples (.txt)
import-document-browse = Procurar…
import-document-move-up = Subir
import-document-move-down = Descer
import-document-remove-file = Remover
import-document-file-count = { $count ->
    [one] 1 ficheiro
   *[other] { $count } ficheiros
}
import-document-no-files = Ainda não foi escolhido nenhum ficheiro.
import-document-col-included = Importar
import-document-col-title = Título
import-document-col-type = Tipo
import-document-col-words = Palavras
import-document-col-breaks = Quebras
import-document-col-comments = Comentários
import-document-col-footnotes = Notas de rodapé
import-document-col-epigraph = Epígrafe
import-document-col-source = Origem
import-document-level-rules = Níveis de título
import-document-level-n = Título { $level }
import-document-add-top-level = Adicionar nível superior
import-document-add-top-level-tooltip = Inserir um livro acima de todos os itens analisados, para ficheiros de capítulos que não tenham título de livro
import-document-destination = Destino
import-document-destination-hint = Escolha um arquivador ou um item: os novos itens ficam dentro de uma pasta, ou depois de uma cena.
import-document-destination-empty = Ainda não há arquivadores. Crie um primeiro.
import-document-plan-empty = Ainda não há nada para importar.
import-document-pictures = { $count ->
    [one] Uma imagem vem com estes documentos.
   *[other] { $count } imagens vêm com estes documentos.
}
import-document-summary = { $rows ->
    [one] 1 item
   *[other] { $rows } itens
} · { $breaks ->
    [one] 1 quebra de cena
   *[other] { $breaks } quebras de cena
}
import-document-back = Voltar
import-document-cancel = Cancelar
import-document-analyse = Seguinte
import-document-analysing = A ler os documentos…
import-document-step-analysing = Leitura
import-document-cancel-analysis = Parar de ler
import-document-analyse-failed = Não foi possível ler os documentos.
import-document-details = Detalhes
import-document-import = Importar
import-document-done = { $count ->
    [one] 1 item importado
   *[other] { $count } itens importados
}
import-document-undo = Anular
import-diagnostic-file-unreadable = Não foi possível ler «{ $path }»: { $detail }. Os outros ficheiros continuam a ser importados.
import-diagnostic-nested-too-deep = «{ $path }» não foi lido: o seu conteúdo tem mais de { $limit } níveis de aninhamento. Nenhum documento real se aproxima disso, e lê-lo poderia fazer o Skribisto falhar, por isso foi deixado de fora. Os outros ficheiros continuam a ser importados.
import-diagnostic-archive-too-large = «{ $path }» não foi lido: é um ficheiro comprimido feito para se descomprimir num tamanho muito maior do que o seu, o que poderia esgotar a memória e fazer o Skribisto falhar, por isso foi deixado de fora. Os outros ficheiros continuam a ser importados.
import-diagnostic-lossy-decode = { $count ->
    [one] Um carácter em «{ $path }» não foi descodificado. Guarde novamente o ficheiro em UTF-8 para o manter.
   *[other] { $count } caracteres em «{ $path }» não foram descodificados. Guarde novamente o ficheiro em UTF-8 para os manter.
}
import-diagnostic-decoded-from-bom = «{ $path }» foi descodificado como { $detail }, não como UTF-8.
import-diagnostic-empty-file = «{ $path }» está vazio.
import-diagnostic-no-headings = «{ $path }» não tem títulos, por isso chega como um único item.
import-diagnostic-unsupported-format = Nada lê ficheiros «.{ $detail }», por isso «{ $path }» foi ignorado.
import-diagnostic-front-matter-not-flat = Metadados iniciais em «{ $path }»: «{ $detail }» não é um valor simples e foi ignorado.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Uma nota de rodapé em «{ $path }» chega como texto simples: as notas de rodapé não são lidas a partir de Markdown.
   *[other] { $count } notas de rodapé em «{ $path }» chegam como texto simples: as notas de rodapé não são lidas a partir de Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Uma nota de rodapé em «{ $path }» não pôde ser trazida. As outras foram.
   *[other] { $count } notas de rodapé em «{ $path }» não puderam ser trazidas. As outras foram.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Um bloco de HTML em bruto em «{ $path }» foi descartado.
   *[other] { $count } blocos de HTML em bruto em «{ $path }» foram descartados.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Uma quebra de cena dentro de uma citação ou lista em «{ $path }» foi descartada. Só é mantida uma quebra numa linha própria.
   *[other] { $count } quebras de cena dentro de citações ou listas em «{ $path }» foram descartadas. Só é mantida uma quebra numa linha própria.
}
import-diagnostic-image-not-ingested-format = Em «{ $path }», a imagem «{ $detail }» está num formato que o Skribisto não consegue mostrar, por isso foi deixada de fora. As imagens em PNG, JPEG, WebP, GIF, TIFF ou BMP são importadas; guarde-a num desses formatos e insira-a à mão.
import-diagnostic-image-not-ingested-remote = Em «{ $path }», a imagem «{ $detail }» está na web. Uma importação nunca transfere imagens, por isso foi deixada de fora.
import-diagnostic-image-not-ingested-outside = Em «{ $path }», a imagem «{ $detail }» está fora do documento e da sua pasta, por isso foi deixada de fora. Só são importadas as imagens guardadas no documento ou na própria pasta do documento.
import-diagnostic-image-not-ingested-missing = Em «{ $path }», não foi possível encontrar a imagem «{ $detail }», por isso foi deixada de fora.
import-diagnostic-image-not-ingested-too-large = Em «{ $path }», a imagem «{ $detail }» é maior do que um projeto aceita, por isso foi deixada de fora.
import-diagnostic-image-not-ingested-unreadable = Em «{ $path }», não foi possível ler a imagem «{ $detail }» como imagem, por isso foi deixada de fora.
import-diagnostic-image-not-ingested-heading = Em «{ $path }», a imagem «{ $detail }» está num título. Um título do documento torna-se o título de um item, que não contém imagens, por isso foi deixada de fora.
import-diagnostic-image-not-ingested-note = Em «{ $path }», a imagem «{ $detail }» está numa nota de rodapé, numa nota de fim ou num comentário. Estes mantêm as suas palavras, mas nenhuma imagem, por isso foi deixada de fora.
import-diagnostic-image-not-ingested-html = Em «{ $path }», a imagem «{ $detail }» está escrita em HTML, e uma importação deixa o HTML de fora, por isso foi deixada de fora com ele. Escreva-a como uma imagem Markdown para a importar.
import-diagnostic-image-not-ingested-document-full = Em «{ $path }», a imagem «{ $detail }» foi deixada de fora: as imagens deste documento, no seu conjunto, ultrapassam o que uma importação traz. Importe o documento em partes, ou insira as restantes à mão.
import-diagnostic-picture-first-frame-only = Em «{ $path }», a imagem animada «{ $detail }» chega como o seu primeiro fotograma. As imagens de um projeto não se movem.
import-diagnostic-duplicate-title = «{ $title }» aparece { $count } vezes. Se já importou estes ficheiros antes, isto vai duplicá-los.
import-diagnostic-heading-level-jump = «{ $title }» salta do nível de título { $from } para { $to }; é colocado um nível abaixo do seu pai.
import-diagnostic-illegal-combination = «{ $title }» contém prosa, mas o tipo { $kind } não pode conter nenhuma. A importação fica suspensa até mudar o seu tipo ou deixá-lo de fora. Caso contrário, nada é importado.
import-diagnostic-tracked-changes-flattened = { $path } estava a meio de uma revisão: { $count ->
    [one] { $count } alteração registada foi aceite
   *[other] { $count } alterações registadas foram aceites
}, e as eliminações foram descartadas. Este é o texto final. Para manter antes alguma da sua própria redação, use Comparar no último passo.
import-diagnostic-tracked-changes-flattened-by = { $path } estava a meio de uma revisão: { $count ->
    [one] { $count } alteração registada de { $names } foi aceite
   *[other] { $count } alterações registadas de { $names } foram aceites
}, e as eliminações foram descartadas. Este é o texto final. Para manter antes alguma da sua própria redação, use Comparar no último passo.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } contém { $count } caixa de texto. O seu texto está fora do fluxo do documento, por isso não é possível saber onde pertence num manuscrito. Não é importada.
   *[other] { $path } contém { $count } caixas de texto. O seu texto está fora do fluxo do documento, por isso não é possível saber onde pertence num manuscrito. Não são importadas.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } contém { $count } objeto incorporado: um gráfico, uma equação ou semelhante. Não há nada num manuscrito que o possa conter.
   *[other] { $path } contém { $count } objetos incorporados: um gráfico, uma equação ou semelhante. Não há nada num manuscrito que os possa conter.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } contém { $count } campo: um número de página, uma referência cruzada, uma data. Mantém o texto que mostrava da última vez e não voltará a ser atualizado.
   *[other] { $path } contém { $count } campos: um número de página, uma referência cruzada, uma data. Cada um mantém o texto que mostrava da última vez e não voltará a ser atualizado.
}
import-diagnostic-unknown-style-level = { $path } usa o estilo «{ $detail }», que parece um título mas não indica nenhum nível. Esses parágrafos são importados como prosa, em vez de se adivinhar uma profundidade.
import-diagnostic-comment-unanchored = O comentário «{ $detail }» em { $path } não pôde ser associado às palavras a que se referia. É mantido no seu item, de onde o pode mover.
import-diagnostic-comment-not-carried = O comentário «{ $detail }» em { $path } não é importado, porque nada no ficheiro chega como texto onde possa ser mantido. Copie-o do ficheiro se precisar dele.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } resposta em { $path } referia um comentário que não está no ficheiro, por isso chega como um comentário independente.
   *[other] { $count } respostas em { $path } referiam um comentário que não está no ficheiro, por isso chegam como comentários independentes.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } parágrafo em { $path } não pôde ser trazido com a formatação exatamente como estava escrita, por isso chega como texto simples. Compare-o com o original.
   *[other] { $count } parágrafos em { $path } não puderam ser trazidos com a formatação exatamente como estava escrita, por isso chegam como texto simples. Compare-os com o original.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] Em { $path }, { $count } trecho de espaço em branco estava sublinhado ou rasurado, por exemplo uma linha deixada para preencher. O Skribisto não mantém nenhuma dessas linhas apenas sobre espaços, por isso chega como espaços simples, ou não chega de todo quando começa ou termina um parágrafo.
   *[other] Em { $path }, { $count } trechos de espaço em branco estavam sublinhados ou rasurados, por exemplo linhas deixadas para preencher. O Skribisto não mantém nenhuma dessas linhas apenas sobre espaços, por isso chegam como espaços simples, ou não chegam de todo quando começam ou terminam um parágrafo.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } item de lista em { $path } tinha mais de { $limit } níveis de aninhamento. Chega ao nível { $limit }, o mais profundo que o Skribisto mantém, com as suas palavras e formatação.
   *[other] { $count } itens de lista em { $path } tinham mais de { $limit } níveis de aninhamento. Chegam ao nível { $limit }, o mais profundo que o Skribisto mantém, com as suas palavras e formatação.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabela em { $path } tem uma linha muito mais larga do que as outras, demasiado desequilibrada para ser disposta em grelha. As suas células chegam como parágrafos, uns a seguir aos outros, com as suas palavras e formatação.
   *[other] { $count } tabelas em { $path } têm uma linha muito mais larga do que as outras, demasiado desequilibradas para serem dispostas em grelha. As suas células chegam como parágrafos, uns a seguir aos outros, com as suas palavras e formatação.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } sequência de espaços em { $path } tinha mais de { $limit } espaços e chega encurtada para { $limit }. As palavras à volta estão todas lá.
   *[other] { $count } sequências de espaços em { $path } tinham mais de { $limit } espaços e chegam encurtadas para { $limit }. As palavras à volta estão todas lá.
}
import-diagnostic-epigraph-not-carried = «{ $title }» é encabeçado por uma epígrafe, mas o tipo { $kind } não pode conter nenhuma. A citação é mantida antes no início do seu texto.
import-diagnostic-epigraph-placement-ambiguous = Uma epígrafe está entre «{ $title }» e «{ $below }» e podia encabeçar qualquer um deles. Foi atribuída a «{ $title }», que é onde uma epígrafe costuma ficar.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 coisa a saber
           *[other] { $warnings } coisas a saber
        }
   *[other] { $errors ->
            [one] Não foi possível ler 1 ficheiro
           *[other] Não foi possível ler { $errors } ficheiros
        }
}
import-document-diagnostics-none = Não há nada a reportar.

versions-title = Versões
versions-scope-synopsis = Sinopse
versions-scope-prose = Texto
versions-loading = A procurar nas suas cópias de segurança…
versions-empty = Ainda não há nenhuma versão anterior disto
versions-error = Não foi possível ler as suas cópias de segurança. Nada se perdeu, mas esta lista pode estar incompleta
versions-did-not-exist = Ainda não existia em { $date }
versions-deleted-after = Eliminado algum tempo depois de { $date }
versions-unreadable = { $count ->
    [one] Não foi possível ler { $count } cópia de segurança
   *[other] Não foi possível ler { $count } cópias de segurança
}
versions-thinned = { $count ->
    [one] As versões mais antigas vão rareando com o tempo. O histórico próprio do projeto já descartou { $count } estado anterior deste texto.
   *[other] As versões mais antigas vão rareando com o tempo. O histórico próprio do projeto já descartou { $count } estados anteriores deste texto.
}
versions-source-backup = De uma cópia de segurança
versions-source-project = Do histórico próprio do projeto
versions-list-caption = Uma entrada por alteração, não por cópia de segurança
versions-pick-a-version = Escolha uma versão para ver o que mudou
versions-earliest = A versão mais antiga registada. Não há nada mais antigo com que a comparar.
versions-no-change = Nada mudou nesta parte do item
versions-formatting-only = Apenas a formatação foi alterada aqui. As palavras são as mesmas
versions-show-unchanged = Mostrar o que não mudou
versions-hide-unchanged = Ocultar o que não mudou
versions-next-change = Alteração seguinte
versions-near = perto de «{ $text }»
versions-words-added = { $count ->
    [one] { $count } palavra adicionada
   *[other] { $count } palavras adicionadas
}
versions-words-removed = { $count ->
    [one] { $count } palavra removida
   *[other] { $count } palavras removidas
}
versions-blocks-moved = { $count ->
    [one] { $count } parágrafo movido
   *[other] { $count } parágrafos movidos
}
versions-pin = Fixar a cópia de segurança de onde veio esta versão. A limpeza automática nunca a eliminará
versions-unpin = Desafixar a cópia de segurança de onde veio esta versão. A limpeza automática poderá voltar a eliminá-la
versions-pinned-only = Mostrar apenas as versões fixadas
versions-pin-note = Só as versões de uma cópia de segurança podem ser fixadas. Fixar mantém um ficheiro, e o histórico próprio do projeto não é um ficheiro.
versions-pinned-empty = Ainda não há nada fixado aqui
versions-pinned-empty-log = Ainda não há nada fixado aqui. Só as versões de uma cópia de segurança podem ser fixadas. Fixar mantém um ficheiro, e o histórico próprio do projeto não é um ficheiro.
versions-range-filter = Mostrar apenas as versões entre duas datas
versions-filtered-empty = Nenhuma versão corresponde aos filtros definidos
versions-clear-filters = Limpar os filtros
versions-last-30-days = Últimos 30 dias
versions-restore-button = Restaurar esta versão
versions-restore-confirm-title = Substituir este texto pela versão de { $date }?
versions-restore-confirm-text = O que tem agora será substituído pelo texto que este item tinha em { $date }.
versions-restore-confirm-with-comments = O que tem agora será substituído pelo texto que este item tinha em { $date }. { $count ->
    [one] { $count } comentário está ancorado no texto atual e pode ficar órfão.
   *[other] { $count } comentários estão ancorados no texto atual e podem ficar órfãos.
}
versions-restore-confirm-undo-note = Primeiro é feita uma cópia de segurança, e Ctrl+Z anula isto num só passo.
versions-restored-toast = Versão de { $date } restaurada
versions-undo = Anular
versions-restore-row-gone = Esse item já não está neste projeto
versions-restore-no-home = Este item mudou de tipo desde então, e o texto antigo não tem para onde ir nele
versions-restore-no-safety-copy = A cópia de segurança preventiva não foi feita, por isso nada foi alterado
versions-restore-failed = A restauração falhou: { $error }
versions-restore-backup-busy = Já está em curso uma cópia de segurança. Tente novamente daqui a pouco
versions-restore-in-backup-file = Está a ver um ficheiro de cópia de segurança; abra o próprio projeto para restaurar nele
versions-restore-no-project = Nenhum projeto está aberto
versions-recreate-button = Recuperar isto…
versions-recreate-picker-title = Para onde deve ir?
versions-recreate-picker-empty = Este projeto não tem nenhum arquivador onde o colocar
versions-recreate-picker-confirm = Recuperá-lo aqui
versions-recreate-picker-cancel = Cancelar
versions-recreate-untitled = este item
versions-recreate-confirm-title = Recuperar «{ $item }»?
versions-recreate-confirm-text = Será adicionado a { $destination }, com o texto que tinha em { $date }.
versions-recreate-confirm-undo-note = Anular, na mensagem que se segue, volta a retirá-lo de imediato.
versions-recreated-toast = «{ $item }» está de volta ao seu projeto
versions-recreated-partial-toast = { $count ->
    [one] «{ $item }» está de volta, mas um dos seus textos não pôde ser lido
   *[other] «{ $item }» está de volta, mas { $count } dos seus textos não puderam ser lidos
}
versions-recreate-already-here = Esse item já está no seu projeto
versions-recreate-no-destination = Escolha um lugar no arquivador onde o colocar
versions-recreate-unreadable = Não foi possível ler essa cópia de segurança, por isso nada foi adicionado
versions-recreate-failed = Não foi possível recuperá-lo: { $error }
versions-changed-percent = { $percent }% disto mudou
versions-hidden-paragraphs = { $count ->
    [one] … { $count } parágrafo sem alterações …
   *[other] … { $count } parágrafos sem alterações …
}

timeline-title = Voltar atrás no tempo
timeline-coverage = { $count ->
    [one] { $count } versão registada, desde { $oldest }
   *[other] { $count } versões registadas, desde { $oldest }
}
timeline-loading = A percorrer o passado do seu projeto…
timeline-empty = Ainda não foi registada nenhuma versão deste projeto
timeline-no-changes = Nada mudou desde então
timeline-no-text-changes = Nenhum texto foi alterado desde então
timeline-slider-label = Versão registada
timeline-series-name = Tamanho do projeto
timeline-bars-caption = Cada barra é uma versão registada, com a altura que o projeto tinha nessa altura. A destacada é a que está a ver.
timeline-bars-caption-periods = Há demasiadas versões para as mostrar uma a uma, por isso cada barra agrupa um período (unidade: { $period }), com a altura que o projeto tinha no fim desse período.
timeline-unit-hour = hora
timeline-unit-day = dia
timeline-unit-week = semana
timeline-unit-month = mês
timeline-range-filter = Mostrar apenas as versões entre duas datas
timeline-range-empty = Não foi registada nenhuma versão entre essas datas
timeline-open-period = Abrir este período
timeline-show-all = Mostrar todo o histórico
timeline-changed-since = { $count ->
    [one] { $count } item difere entre { $date } e o seu projeto atual
   *[other] { $count } itens diferem entre { $date } e o seu projeto atual
}
timeline-kind-added = Escrito desde então
timeline-kind-removed = Já não está no projeto
timeline-kind-changed = Editado desde então
timeline-kind-moved = Movido desde então
timeline-not-yet-written = Isto ainda não existia nessa altura
timeline-no-text-of-its-own = Isto não tem texto próprio. É um título para o que está dentro dele
timeline-reader-close = Fechar
timeline-reader-stamp = Como estava em { $date }
timeline-reader-loading = A abrir a versão registada…
timeline-reader-unreadable = Não foi possível ler este registo. A cópia de segurança pode ter sido movida, eliminada ou estar num disco que não está ligado.
timeline-reader-compared = Como estava em { $date }, comparado com o que diz agora
timeline-reader-diff-legend = O rasurado desapareceu desde então; o sublinhado foi acrescentado.
timeline-reader-view-label = O que mostrar
timeline-reader-view-diff = Alterações
timeline-reader-view-text = Texto
timeline-reader-deleted = Isto já não está no seu projeto. Pode lê-lo e copiá-lo daqui.
timeline-prose-only-record = Este ponto vem do histórico próprio do projeto, que guarda o texto e nada mais, por isso o que foi eliminado ou movido desde então não pode ser mostrado. Escolha uma cópia de segurança para isso.

settings-sec-extensions = Extensões

settings-group-games-forward = Sempre em frente
settings-games-forward-toggle = Jogar «Sempre em frente»
settings-games-forward-hint =
    Enquanto joga, nada do que escreveu pode ser desfeito: Backspace, Delete,
    Cortar, arrastar e largar e Anular ficam todos desativados nas superfícies que escolher abaixo.
    Pode continuar a escrever, colar, formatar e deslocar-se livremente. O rascunho só cresce.
settings-games-session-warning =
    Esta escolha vale apenas para a sessão: nunca é guardada. Fechar o projeto ou sair do
    Skribisto termina sempre o jogo, e tudo o que escreveu enquanto jogava volta
    a poder ser anulado nesse momento.
settings-group-games-scope = Onde se aplica
settings-games-in-prose = Prosa do manuscrito
settings-games-in-synopsis = Sinopses
settings-games-scope-hint =
    Os comentários, as notas de rodapé e os títulos nunca ficam congelados: é neles que anota a correção
    que acabou de se proibir de fazer.
settings-games-inert-warning =
    «Sempre em frente» está ativo mas não se aplica a nada. Marque pelo menos uma superfície acima,
    ou não muda nada enquanto escreve.

games-title = Jogos de escrita
games-forward-name = Sempre em frente
games-forward-blurb =
    Escreva o rascunho sem voltar atrás. Apagar, cortar e anular ficam desativados
    enquanto joga. Escreva a frase seguinte em vez de corrigir a última.
games-forward-playing = A jogar: apagar está desativado
games-forward-idle = Fora de jogo
games-session-note = Termina quando fechar o projeto.
games-scope-prose-and-synopsis = Aplica-se à sua prosa e às suas sinopses.
games-scope-prose = Aplica-se à sua prosa.
games-scope-synopsis = Aplica-se às suas sinopses.
games-scope-nothing = Ainda não se aplica a nada. Escolha uma superfície em Definições.
games-settings-link = Definições dos jogos de escrita…
statusbar-games-forward = Sempre em frente
statusbar-games-forward-tooltip =
    «Sempre em frente» está ativo: apagar, cortar e anular estão desativados enquanto
    escreve o rascunho. Clique para parar de jogar.
goal-progress-words = { $g ->
    [one] { $count } de { $goal } palavra
   *[other] { $count } de { $goal } palavras
}
goal-progress-characters = { $g ->
    [one] { $count } de { $goal } carácter
   *[other] { $count } de { $goal } caracteres
}
goal-count-words = { $n ->
    [one] { $count } palavra
   *[other] { $count } palavras
}
goal-count-characters = { $n ->
    [one] { $count } carácter
   *[other] { $count } caracteres
}
goal-unit-words = Palavras
goal-unit-characters = Caracteres

inspector-goal = Meta
inspector-goal-none = Sem meta

overview-col-goal = Meta
overview-excluded-from-export = Deixado de fora da exportação, por isso não conta para nenhum total

goal-subtree-total-words = { $count ->
    [one] { $items } meta no interior soma { $words } palavras
   *[other] { $items } metas no interior somam { $words } palavras
}
goal-subtree-total-characters = { $count ->
    [one] { $items } meta no interior soma { $words } caracteres
   *[other] { $items } metas no interior somam { $words } caracteres
}

new-work-goal-unit = Contar em
new-work-goal-unit-hint = Usado para todas as metas de palavras ou de caracteres deste projeto. Pode ser alterado mais tarde.

settings-group-goal-unit = Metas
settings-goal-unit-switch-title = Mudar a unidade de contagem
settings-goal-unit-switch-text = As metas e os marcos já definidos para este projeto foram introduzidos em { $from }. Mudar para { $to } não os converte, por isso todos os números existentes passam a ser lidos como { $to }.
settings-goal-unit-switch-informative = Nada se perde. Volte atrás a qualquer momento para repor a leitura original e depois atualize as metas que quer manter.
settings-goal-unit-switch-confirm = Mudar

goal-distribute-action = Distribuir…
distribute-title = Distribuir a meta
distribute-empty = Não há nada dentro disto por onde repartir a meta.
distribute-weight = Repartir
distribute-weight-length = Por comprimento
distribute-weight-rows = Por peças
distribute-weight-even = Por igual
distribute-overwrite = Substituir as metas já definidas
distribute-col-item = Item
distribute-col-current = Agora
distribute-col-proposed = Depois
distribute-total = Somam { $total }, para uma meta de { $goal }.
distribute-over-budget = As metas já definidas no interior excedem em { $over } a deste contentor. Aumente a meta dele, reduza as dos itens interiores ou substitua-as.
distribute-apply = Distribuir
distribute-cancel = Cancelar

milestone-target-gone = Alvo eliminado
milestone-no-target = Sem alvo
milestone-add = Adicionar
milestone-add-label = O que deve ser alcançado

pace-summary-title = Ponto da situação do livro
pace-summary-remaining = Faltam { $words }
pace-summary-open = Abrir o plano
pace-summary-close = Fechar
pace-summary-dont-show = Não mostrar isto ao abrir
pace-summary-menu = Plano de escrita…

menu-help-topics = &Tópicos de ajuda
menu-help-shortcuts = Atal&hos de teclado…
menu-help-website = &Skribisto na Web
menu-help-report = &Comunicar um problema…
menu-help-check-updates = Procurar at&ualizações
menu-command-palette = &Paleta de comandos…

learn-help-topics = Tópicos de ajuda
learn-shortcuts = Atalhos de teclado
learn-website = Skribisto na web

help-window-title = Ajuda
help-filter-topics = Filtrar tópicos
help-no-matching-topic = Nenhum tópico corresponde.
help-topic-missing = Este tópico já não está disponível.
help-back = Voltar
help-not-translated = Esta página ainda não foi traduzida, pelo que é apresentada em inglês.

help-section-getting-started = Primeiros passos
help-section-writing = Escrita
help-section-reviewing = Revisão
help-section-exchanging = Fazer entrar e sair o texto
help-section-keeping = Manter o seu trabalho em segurança
help-section-extensions = Extensões

help-topic-getting-started = O seu primeiro projeto
help-topic-writing-model = Como um livro é montado
help-topic-drafts-and-old-versions = Manter um rascunho mais antigo
help-topic-goals-and-pace = Metas e ritmo
help-topic-comments = Comentários
help-topic-round-trip = Enviar o seu livro a um leitor
help-topic-export = Exportação
help-topic-import-documents = Importar documentos
help-topic-import-projects = Trazer um projeto inteiro
help-topic-import-scrivener = Vindo do Scrivener
help-topic-backups-and-versions = Cópias de segurança e versões

help-shortcuts-title = Atalhos de teclado
help-shortcuts-filter = Filtrar atalhos
help-shortcuts-no-matches = Nenhum atalho corresponde.
help-shortcuts-rebind = Alterar atalhos…
help-shortcuts-close = Fechar

command-palette-placeholder = Escreva um comando

shortcut-name-binder-duplicate = Duplicar
shortcut-name-comments-add = Adicionar comentário
shortcut-name-comments-add-paragraph = Comentar o parágrafo
shortcut-name-spellcheck-toggle = Verificar ortografia
shortcut-name-editor-tab-close = Fechar separador
shortcut-name-editor-tab-pin = Fixar ou desafixar separador
shortcut-name-editor-save = Guardar
shortcut-name-work-export = Exportar…
shortcut-name-work-new = Novo projeto
shortcut-name-work-open = Abrir projeto
shortcut-name-window-new = Nova janela
shortcut-name-work-close = Fechar projeto
shortcut-name-work-import-scrivener = Importar projeto Scrivener…
shortcut-name-work-import-scrivener-report = Mostrar o relatório de importação do Scrivener
shortcut-name-app-settings = Definições
shortcut-name-app-quit = Sair
shortcut-name-editor-insert-footnote = Inserir nota de rodapé
shortcut-name-format-scene-break = Inserir quebra de cena
shortcut-name-format-major-scene-break = Inserir quebra de cena principal
shortcut-name-format-link = Inserir ligação
shortcut-name-format-mark-nbsp = Inserir espaço sem quebra
shortcut-name-format-mark-nbhyphen = Inserir hífen sem quebra
shortcut-name-format-mark-nnbsp = Inserir espaço estreito sem quebra
shortcut-name-format-mark-zwsp = Inserir espaço de largura zero
shortcut-name-go-next = Seguinte
shortcut-name-go-prev = Anterior
shortcut-name-go-to = Ir para
shortcut-name-outline-toggle = Mostrar ou ocultar a estrutura
shortcut-name-preview-toggle = Mostrar ou ocultar a faixa de pré-visualização
shortcut-name-view-fullscreen = Alternar ecrã inteiro
shortcut-name-view-focus-mode = Alternar o modo sem distrações
shortcut-name-editor-size-increase = Aumentar o tamanho do texto
shortcut-name-editor-size-decrease = Diminuir o tamanho do texto
shortcut-name-editor-size-reset = Repor o tamanho do texto
shortcut-name-editor-find = Localizar
shortcut-name-editor-replace = Substituir
shortcut-name-editor-find-next = Correspondência seguinte
shortcut-name-editor-find-prev = Correspondência anterior
shortcut-name-search-show = Pesquisar no projeto
shortcut-name-search-replace = Substituir no projeto
shortcut-name-outline-open-to-side = Abrir ao lado
shortcut-name-help-topics = Tópicos de ajuda
shortcut-name-help-shortcuts = Atalhos de teclado
shortcut-name-help-website = Skribisto na Web
shortcut-name-help-report = Comunicar um problema
shortcut-name-help-check-updates = Procurar atualizações
update-check-behind = A versão { $version } está disponível.
update-check-current = Está a utilizar a versão mais recente.
update-check-failed = Não foi possível procurar uma versão mais recente. { $error }
shortcut-name-command-palette = Paleta de comandos
help-section-reference = O que são as coisas

help-concept-scene-break = Quebra de cena
help-concept-major-scene-break = Quebra de cena principal
help-concept-find-in-prose = Procurar na prosa
help-concept-story-bible = Bíblia da história
help-concept-goal-unit = Palavras ou caracteres
help-concept-goal-progress = Progresso
help-concept-manuscript-words = O que conta como o manuscrito
help-concept-exportable = Deixado de fora da exportação
help-concept-distribute = Repartir uma meta
help-concept-subtree-total = O que somam as metas no interior
help-concept-milestone = Marco
help-concept-pace-plan = Plano de ritmo

help-concept-tag = Etiqueta
help-concept-label = Rótulo
help-concept-point-of-view = Ponto de vista
help-concept-epigraph = Epígrafe
help-concept-footnote = Nota de rodapé
help-concept-chapter-mode = Forma dos capítulos
help-concept-comment = Comentário
help-concept-backup = Cópia de segurança
help-concept-version = Versão
help-concept-trash = Lixo
help-concept-spellcheck = Verificação ortográfica
help-concept-search-replace = Procurar e substituir
help-concept-note-template = Modelo de nota
help-concept-story-bible-entry = Entrada da bíblia da história
help-concept-text-replacement = Substituição de texto
help-concept-smart-punctuation = Pontuação inteligente
help-concept-export-style = Estilo de exportação
help-concept-round-trip-marks = Marcadores de ida e volta

epigraph-mark-attribution = Linha de origem
epigraph-mark-attribution-tip = Marcar a linha onde está o cursor como a fonte da citação, para que seja impressa como atribuição
margin-lane-name = Marcas de margem
margin-lane-provider-comments = Comentários
margin-lane-provider-story-bible = Entrada da bíblia da história
margin-lane-provider-story-bible-hint = Na leitura Em prosa de uma nota, onde essa entrada é nomeada, e as cenas contadas pelos seus olhos. Também marca esses nomes na própria prosa.
margin-lane-mark-point-of-view = Narrado a partir daqui
margin-lane-mark-named-here = Nomeado aqui
margin-lane-provider-comments-hint = Onde uma nota está associada ao texto
margin-lane-provider-search = Resultados da pesquisa
margin-lane-provider-search-hint = Todos os resultados da sua última pesquisa, onde quer que estejam no documento
margin-lane-provider-boundaries = Onde cada documento começa
margin-lane-provider-boundaries-hint = Uma linha no topo de cada cena de um fluxo, para que veja onde está
margin-lane-provider-spelling = Ortografia
margin-lane-provider-spelling-hint = Onde o verificador ortográfico assinalou uma palavra. Desativado salvo se o pedir: marca a opinião de uma máquina sobre a sua prosa
margin-lane-spelling = { $word }, possível erro ortográfico
margin-lane-search-hit = { $text }, correspondência { $index } de { $total }
margin-lane-search-current = { $text }, correspondência { $index } de { $total }, aquela em que está
margin-lane-boundary = Início de { $title }
margin-lane-boundary-untitled = um documento sem título
settings-page-margin-lane = Marcas de margem
settings-desc-margin-lane = A faixa ao lado da barra de deslocamento e o que mostra
settings-margin-lane-enabled = Mostrar a faixa da margem
settings-margin-lane-enabled-hint = O mesmo interruptor que Ver ▸ Marcas de margem.
settings-margin-lane-enabled-more = Nunca diz que algo está errado: mostra-lhe onde estão as coisas e deixa consigo o que fazer com isso.
settings-group-margin-lane-marks = O que marca
settings-margin-lane-no-providers = Ainda nada marca a faixa.
settings-group-margin-lane-texture = Textura do diálogo
settings-margin-lane-texture = Mostrar a textura do diálogo
settings-margin-lane-texture-hint = Uma barra por parágrafo: o seu comprimento, e quanto dele é falado.
settings-margin-lane-texture-more = Não é medida para idiomas sem convenção estabelecida, em que a barra mostra apenas o comprimento.
settings-group-margin-lane-surfaces = Onde aparece
settings-margin-lane-surface-editor = Editor de texto
settings-margin-lane-surface-stream = Fluxos
settings-margin-lane-surface-search-preview = Pré-visualização da pesquisa
menu-margin-lane = &Marcas de margem

status-preset-drafting = Redação
status-preset-passes = Passagens de revisão
status-preset-plume = Plume Creator
status-todo = Por fazer
status-draft = Rascunho
status-revised = Revisto
status-final = Final
status-outline = Esboço
status-first-edit = 1.ª revisão
status-second-edit = 2.ª revisão
status-done = Concluído
status-plume-draft-1 = 1.º rascunho
status-plume-draft-2 = 2.º rascunho
status-plume-draft-3 = 3.º rascunho
status-plume-edit-1 = 1.ª revisão
status-plume-edit-2 = 2.ª revisão
status-plume-edit-3 = 3.ª revisão
status-plume-proofread = Revisão de provas
status-plume-finished = Terminado
new-work-statuses = Fluxo de trabalho
new-work-statuses-hint = As etapas por que faz passar uma cena. Pode mudar-lhes o nome, reordená-las ou acrescentar outras a qualquer momento.
status-none = Sem estado
inspector-status = Estado
overview-col-status = Estado
overview-status-mixed = as partes abaixo discordam
help-concept-status = Estado
status-completion-title = Ponto da situação do livro
status-completion-headline = { $done } de { $total } cenas terminadas
status-completion-empty = Ainda não há cenas. Isto lê o manuscrito, por isso vai-se preenchendo à medida que escreve.
status-completion-open = Ponto da situação do livro…

settings-page-statuses = Estados
settings-desc-statuses = Mudar o nome, reordenar, adicionar e remover as etapas por que faz passar uma cena
settings-statuses-add = Adicionar estado
settings-statuses-add-placeholder = Novo nome de estado
settings-statuses-desc =
    As etapas por que passa uma cena, por ordem, da menos acabada no topo à mais
    acabada em baixo. Essa ordem é o que «menos acabado do que» significa em todo o resto
    da aplicação, por isso disponha-as como o seu processo realmente decorre.
settings-statuses-apply-preset = Aplicar uma predefinição…
settings-statuses-preset-applied = { $added ->
    [one] 1 estado adicionado
   *[other] { $added } estados adicionados
}
settings-statuses-preset-refused = Este projeto já tem uma escala de estados. As predefinições só preenchem uma escala vazia. Elimine primeiro os degraus que não quer.
settings-statuses-duplicate = «{ $name }» já está nesta escala
settings-statuses-added = «{ $name }» adicionado
settings-statuses-move-up = Subir (menos acabado)
settings-statuses-move-down = Descer (mais acabado)
settings-statuses-details-placeholder = O que significa esta etapa (opcional)
settings-statuses-delete = Eliminar «{ $name }»
settings-statuses-delete-in-use = { $count ->
    [one] Eliminar «{ $name }»: 1 item tem este estado e vai perdê-lo
   *[other] Eliminar «{ $name }»: { $count } itens têm este estado e vão perdê-lo
}
settings-statuses-deleted = «{ $name }» eliminado
settings-statuses-deleted-in-use = { $count ->
    [one] «{ $name }» eliminado. 1 item já não tem estado.
   *[other] «{ $name }» eliminado. { $count } itens já não têm estado.
}
settings-statuses-empty-title = Ainda não há fluxo de trabalho
settings-statuses-empty-body = Um estado indica em que ponto está uma cena. Adicione um acima, ou comece a partir de uma predefinição.
status-category-planned = Planeado
status-category-drafting = Em redação
status-category-needs-work = Precisa de trabalho
status-category-revised = Revisto
status-category-final = Final

progress-percent = { $percent }%
progress-percent-with-label = { $percent }% · { $label }
progress-import-opening = A abrir o projeto…
progress-import-outline = A ler a estrutura…
progress-import-converting = A converter capítulos e cenas…
progress-import-writing = A escrever o ficheiro .skrib…
progress-import-done = Concluído
progress-import-structure = A determinar a estrutura…
progress-export-start = A iniciar…
progress-export-writing = A escrever…
progress-project-reading = A ler o projeto…
settings-sec-work-titled = Projeto: { $title }
import-notice-flattened-rich-texts = { $count ->
    [one] Um texto estava aninhado a uma profundidade maior do que um projeto Skribisto suporta; as suas palavras foram mantidas como texto simples, sem a formatação.
   *[other] { $count } textos estavam aninhados a uma profundidade maior do que um projeto Skribisto suporta; as suas palavras foram mantidas como texto simples, sem a formatação.
}
import-notice-free-text-badges = { $count ->
    [one] Um distintivo de cena parece texto livre e não uma lista de estados, por isso foi mantido como rótulo do item em vez de se tornar uma etiqueta.
   *[other] { $count } distintivos de cena diferentes parecem texto livre e não uma lista de estados, por isso foram mantidos como rótulos dos itens em vez de se tornarem etiquetas.
}
import-notice-separator-text-dropped = O separador «{ $name }» continha texto, que não tem lugar num projeto Skribisto; o texto não foi importado.
import-notice-separator-without-scene = O separador «{ $name }» não tinha nenhuma cena antes dele no seu capítulo a que se pudesse juntar, por isso não foi importado.
import-notice-chapter-without-scenes = O capítulo «{ $name }» não tem cenas, por isso os seus separadores não foram importados.
import-notice-rich-text-unconverted = Não foi possível converter algum texto formatado ({ $error }); esse texto não foi importado.
import-notice-unresolved-cross-link = Não foi possível seguir uma ligação para a entrada n.º { $number } do elenco: a entrada não existe ou estava no lixo.
import-notice-read-copy = Foi lida a cópia { $container ->
    [single-file] em ficheiro único
   *[folder] em pasta
} deste projeto.
import-notice-read-copy-dated = Foi lida a cópia { $container ->
    [single-file] em ficheiro único
   *[folder] em pasta
} deste projeto, alterada pela última vez em { $day }.
import-notice-original-format = Este projeto está no formato original de 2016 do Manuskript. Foi lido na íntegra; nada nele foi alterado.
import-notice-newer-single-file-beside = «{ $file }» é mais recente do que esta pasta ({ $file_day } contra { $folder_day }). O Manuskript deixa a pasta antiga no lugar quando um projeto passa para o modo de ficheiro único, por isso esta pasta pode ser uma cópia abandonada; importar antes o ficheiro .msk leria a versão mais recente.
import-notice-pickle-skipped = «{ $file }» não foi lido. É um pickle de Python, e carregar um a partir de um ficheiro de projeto era uma falha de segurança do próprio Manuskript (CVE-2021-35196); as definições que contém são preferências de apresentação, não escrita.
import-notice-file-unreadable = Não foi possível ler «{ $file }» ({ $error }), pelo que foi ignorado; tudo o resto foi importado.
import-notice-not-utf8 = «{ $file }» não é UTF-8 válido; os caracteres que não puderam ser lidos foram substituídos, em vez de interromper a importação.
import-notice-table-unreadable = Não foi possível ler «{ $file }» ({ $error }); o que continha não foi importado.
import-notice-outline-missing = «{ $file }» não existe, por isso este projeto não tem manuscrito para importar.
import-notice-outline-unreadable = Não foi possível ler «{ $file }» ({ $error }); nenhum manuscrito foi importado.
import-notice-named-none-row = «{ $file }» começa com uma linha com nome («{ $name }») onde o Manuskript guarda a sua entrada vazia «none». Essa linha foi deixada de fora, para que as linhas seguintes mantenham os números que o manuscrito usa.
import-notice-unknown-field = «{ $file }» tem um campo que este importador não conhece, «{ $field }». Esse campo não foi importado.
import-notice-colorless-labels = { $count ->
    [one] «{ $file }» tem uma linha sem cor. Foi importada sem cor; o Manuskript deixa de carregar o projeto numa linha assim.
   *[other] «{ $file }» tem { $count } linhas sem cor. Foram importadas sem cor; o Manuskript deixa de carregar o projeto numa linha assim.
}
import-notice-settings-unreadable = Não foi possível ler «{ $file }» ({ $error }); a língua do projeto não foi importada.
import-notice-revisions-unreadable = Não foi possível ler «{ $file }» ({ $error }); o histórico do projeto não foi importado, mas nada mais foi afetado.
import-notice-foreign-files = { $count ->
    [one] Este projeto contém um ficheiro que não faz parte do formato Manuskript e não foi importado: { $files }.
   *[other] Este projeto contém { $count } ficheiros que não fazem parte do formato Manuskript e não foram importados: { $files }.
}
import-notice-foreign-files-more = Este projeto contém { $count } ficheiros que não fazem parte do formato Manuskript e não foram importados: { $files } e { $more ->
    [one] mais um
   *[other] mais { $more }
}.
import-notice-not-a-character-file = «{ $file }» não é um ficheiro de personagem do Manuskript e foi deixado de fora.
import-notice-not-a-text-item = «{ $file }» não é um item de texto do Manuskript e foi deixado fora da estrutura.
import-notice-missing-blank-line = «{ $file }» não tem uma linha em branco depois do cabeçalho. O último campo foi mantido, embora o Manuskript o perca.
import-notice-folder-without-metadata = «{ $folder }» não tem { $file }, por isso foi importada como uma pasta simples com o nome do diretório. O Manuskript ignora um diretório assim e tudo o que contém.
import-notice-rows-given-ids = { $count ->
    [one] Uma linha da estrutura não tinha ID próprio e recebeu um. O Manuskript descarta uma linha assim, ou deixa de carregar o projeto quando é uma pasta.
   *[other] { $count } linhas da estrutura não tinham ID próprio e receberam um. O Manuskript descarta uma linha assim, ou deixa de carregar o projeto quando é uma pasta.
}
import-notice-plots-unreadable = Não foi possível ler «{ $file }» ({ $error }); os enredos não foram importados.
import-notice-world-unreadable = Não foi possível ler «{ $file }» ({ $error }); as entradas do mundo não foram importadas.
import-notice-world-not-opml = «{ $file }» não é um documento OPML; as entradas do mundo não foram importadas.
import-notice-text-flattened = O texto de «{ $title }» está aninhado a uma profundidade maior do que um projeto Skribisto suporta; as suas palavras foram mantidas como texto simples, sem a formatação.
import-notice-text-unconverted = Não foi possível converter o texto de «{ $title }» ({ $error }); foi mantido exatamente como foi escrito.
import-notice-text-without-file-name = Não foi possível dar um nome de ficheiro { $kind ->
    [scene] ao texto da cena
    [note] ao texto da nota
    [synopsis] à sinopse
   *[title] ao título
} de «{ $title }», por isso ficou por importar.
import-notice-unknown-label = «{ $title }» tem o rótulo { $index }, que não está na lista de rótulos deste projeto. Foi importado sem rótulo.
import-notice-unknown-status = «{ $title }» tem o estado { $index }, que não está na lista de estados deste projeto. Foi importado sem estado.
import-notice-unknown-point-of-view = «{ $title }» é narrado através de uma personagem que não está no elenco deste projeto. O seu ponto de vista não foi definido.
import-notice-unresolved-references = Não foi possível resolver as referências no texto de «{ $title }» ({ $error }); o texto foi mantido tal como foi convertido, incluindo os marcadores de referência.
import-notice-custom-icons = { $count ->
    [one] Um item tinha um ícone personalizado. O Skribisto dá a cada item o ícone do seu tipo, por isso o ícone não foi importado; nada mais no item mudou.
   *[other] { $count } itens tinham um ícone personalizado. O Skribisto dá a cada item o ícone do seu tipo, por isso os ícones não foram importados; nada mais nesses itens mudou.
}
import-notice-character-colors = { $count ->
    [one] Uma personagem tinha uma cor junto ao nome. O Skribisto colore etiquetas e não itens, por isso a cor não foi importada; a personagem em si foi.
   *[other] { $count } personagens tinham uma cor junto ao nome. O Skribisto colore etiquetas e não itens, por isso as cores não foram importadas; as personagens em si foram todas.
}
import-notice-versions-kept = { $found ->
    [one] Este projeto registou uma versão anterior
   *[other] Este projeto registou { $found } versões anteriores
}; { $kept ->
    [one] foi mantida uma
   *[other] foram mantidas { $kept }
}, rareadas da mesma forma que o Skribisto rareia o seu próprio histórico. Estão no painel Versões.
import-notice-versions-unplaced = { $count ->
    [one] Uma versão anterior pertence a um item que já não está no projeto e não pôde ser colocada.
   *[other] { $count } versões anteriores pertencem a um item que já não está no projeto e não puderam ser colocadas.
}
dialog-open-work-title = Abrir projeto Skribisto
dialog-save-as-file-title = Guardar como um único ficheiro .skrib
dialog-save-as-folder-title = Escolha uma pasta-mãe para o projeto
dialog-manuskript-file-title = Escolha um projeto Manuskript
dialog-manuskript-folder-title = Escolha a pasta de um projeto Manuskript
dialog-plume-file-title = Escolha um projeto Plume Creator
dialog-filter-skrib = Projeto Skribisto
dialog-filter-manuskript = Projeto Manuskript
dialog-filter-plume = Projeto Plume Creator
dialog-filter-hunspell-aff = Ficheiro de afixos Hunspell (.aff)
dialog-filter-hunspell-dic = Dicionário Hunspell (.dic)
dialog-filter-export = Ficheiro exportado
shortcut-name-view-margin-lane = Alternar marcas de margem
df-theme-paper = Papel
df-theme-sepia = Sépia
df-theme-night = Noite
df-theme-high-contrast = Alto contraste
df-theme-midnight = Meia-noite
welcome-example-byline = de { $author }
welcome-sections-a11y = Secções de boas-vindas
comments-a11y-summary-author = { $author }: { $text }
comments-a11y-summary-replies = { $text } ({ $count ->
    [one] 1 resposta
   *[other] { $count } respostas
})
comments-a11y-summary-author-replies = { $author }: { $text } ({ $count ->
    [one] 1 resposta
   *[other] { $count } respostas
})
margin-lane-a11y-mark = { $label }, a { $percent } por cento do texto
margin-lane-a11y-marks = { $label }, { $count } marcas, a { $percent } por cento do texto
margin-lane-a11y-fallback = { $count ->
    [one] Marcas de margem, 1 marca
   *[other] Marcas de margem, { $count } marcas
}
export-choose-unnamed-binder = Arquivador sem nome
export-style-neutral = Neutro
export-style-manuscript-shunn = Manuscrito padrão (Shunn)
export-style-ebook-clean = E-book simples
date-time-short = { $day }/{ $month }/{ $year } { $hour }:{ $minute }
pace-day-label = { $day }/{ $month }
byte-size-bytes = { $value } B
byte-size-kb = { $value } KB
byte-size-mb = { $whole },{ $tenth } MB
byte-size-gb = { $whole },{ $tenth } GB
unit-percent = { $value }%
unit-pixels = { $value } px
settings-paratext-error-no-name = Uma estrutura precisa de um `name`.
export-error-no-project = Não há nenhum projeto aberto para exportar.

shortcut-group-work = Projeto
shortcut-group-edit = Editar
shortcut-group-view = Ver
shortcut-group-document = Documento
shortcut-group-format = Formato
shortcut-group-go = Ir
shortcut-group-tools = Ferramentas
shortcut-group-help = Ajuda

## Manuskript import: the headings of the notes it writes and the title of a project with no name.
import-manuskript-section-paragraph-summary = Resumo num parágrafo
import-manuskript-section-full-summary = Resumo completo
import-manuskript-section-motivation = Motivação
import-manuskript-section-epiphany = Epifania
import-manuskript-section-notes = Notas
import-manuskript-section-description = Descrição
import-manuskript-section-passion = Paixão
import-manuskript-section-result = Resultado
import-manuskript-section-summary = Resumo
import-manuskript-section-meta = Metadados
import-manuskript-section-series = Série
import-manuskript-section-volume = Volume
import-manuskript-section-genre = Género
import-manuskript-section-license = Licença
import-manuskript-section-email = E-mail
import-manuskript-section-situation = Situação
import-manuskript-section-sentence = Uma frase
import-manuskript-section-paragraph = Um parágrafo
import-manuskript-section-page = Uma página
import-manuskript-section-full = Resumo completo
import-manuskript-untitled-project = Projeto Manuskript importado

## Plume import: the story-bible groups an old project implies, and the title of a project with no name.
import-plume-group-characters = Personagens
import-plume-group-items = Objetos
import-plume-group-places = Lugares
import-plume-untitled-project = Projeto Plume importado
