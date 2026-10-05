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
import-plume-busy-title = Outro projeto Plume Creator está a ser importado
import-plume-busy-text = Os projetos Plume Creator são importados um de cada vez. Aguarde até que a importação em curso termine, ou cancele-a, e depois inicie esta novamente.
import-manuskript-busy-title = Outro projeto Manuskript está a ser importado
import-manuskript-busy-text = Os projetos Manuskript são importados um de cada vez. Aguarde até que a importação em curso termine, ou cancele-a, e depois inicie esta novamente.
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
import-plume-close = Fechar
import-plume-source = Projeto Plume
import-plume-source-hint = Escolha um ficheiro .plume ou .plume_backup (qualquer versão do Plume Creator).
import-plume-location = Pasta de destino
import-plume-name = Nome do ficheiro
import-plume-name-placeholder = Nome do projeto
import-plume-will-create = Será criado
import-plume-trash-warning = ⚠ Os itens no lixo ou eliminados não são migrados.

import-plume-cancel = Cancelar
import-plume-import = Importar
import-plume-source-required = Escolha um ficheiro de projeto Plume
import-plume-source-missing = Este ficheiro não existe
import-plume-source-not-file = Este caminho não é um ficheiro
import-plume-location-required = Escolha uma pasta de destino
import-plume-location-missing = Esta pasta não existe
import-plume-location-not-folder = Este caminho não é uma pasta
import-plume-location-readonly = Não é possível escrever nesta pasta
import-plume-name-required = Introduza um nome de ficheiro
import-plume-name-exists = Já existe aqui um ficheiro com este nome. A importação pedirá confirmação para o substituir
import-plume-overwrite-title = Substituir o ficheiro existente?
import-plume-overwrite-text = «{ $name }» já existe. Substituí-lo pelo projeto importado?
import-plume-manuscript-binder = Manuscrito
import-plume-story-bible-binder = Bíblia da história
import-plume-progress-title = A importar o projeto Plume…
import-plume-cancel-import = Cancelar
import-plume-cancelled = Importação cancelada
import-plume-done = { $imported ->
    [one] { $imported } item importado.
   *[other] { $imported } itens importados.
} { $skipped ->
    [one] { $skipped } item no lixo não foi migrado.
   *[other] { $skipped } itens no lixo não foram migrados.
}
import-plume-open-now = Abrir agora
import-plume-warnings = { $count ->
    [one] 1 elemento não pôde ser importado exatamente
   *[other] { $count } elementos não puderam ser importados exatamente
}
import-plume-details = Detalhes
import-plume-warnings-title = Avisos de importação
import-plume-error-title = Não foi possível importar o projeto
import-plume-nested-too-deep = O projeto não foi importado: a sua parte «{ $part }» tem mais de { $limit } níveis de aninhamento. Nenhum projeto Plume Creator se aproxima disso, e lê-lo poderia fazer o Skribisto falhar. Nada foi escrito.
import-plume-declares-entities = O projeto não foi importado: a sua parte «{ $part }» declara entidades XML, o que nenhum projeto Plume Creator faz. Expandi-las poderia exigir mais memória do que o computador tem e fazer o Skribisto falhar. Nada foi escrito.
import-plume-error-details = Detalhes

import-manuskript-title = Importar um projeto Manuskript
import-manuskript-close = Fechar
import-manuskript-source = Projeto Manuskript
import-manuskript-source-hint = Escolha o ficheiro .msk ou a pasta do projeto (qualquer versão do Manuskript).
import-manuskript-source-file = Escolher ficheiro…
import-manuskript-source-folder = Escolher pasta…
import-manuskript-location = Pasta de destino
import-manuskript-name = Nome do ficheiro
import-manuskript-name-placeholder = Nome do projeto
import-manuskript-will-create = Será criado
import-manuskript-cancel = Cancelar
import-manuskript-import = Importar
import-manuskript-source-required = Escolha um projeto Manuskript
import-manuskript-source-missing = Este ficheiro ou pasta não existe
import-manuskript-location-required = Escolha uma pasta de destino
import-manuskript-location-missing = Esta pasta não existe
import-manuskript-location-not-folder = Este caminho não é uma pasta
import-manuskript-location-readonly = Não é possível escrever nesta pasta
import-manuskript-name-required = Introduza um nome de ficheiro
import-manuskript-name-exists = Já existe aqui um ficheiro com este nome. A importação pedirá confirmação para o substituir
import-manuskript-overwrite-title = Substituir o ficheiro existente?
import-manuskript-overwrite-text = «{ $name }» já existe. Substituí-lo pelo projeto importado?
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
import-manuskript-progress-title = A importar o projeto Manuskript…
import-manuskript-cancel-import = Cancelar
import-manuskript-cancelled = Importação cancelada
import-manuskript-done = { $imported ->
    [one] { $imported } item importado.
   *[other] { $imported } itens importados.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } versão anterior veio com ele.
   *[other] { $revisions } versões anteriores vieram com eles.
}
import-manuskript-open-now = Abrir agora
import-manuskript-warnings = { $count ->
    [one] 1 coisa a saber sobre esta importação
   *[other] { $count } coisas a saber sobre esta importação
}
import-manuskript-details = Detalhes
import-manuskript-warnings-title = Sobre esta importação
import-manuskript-error-title = Não foi possível importar o projeto
import-manuskript-nested-too-deep = O projeto não foi importado: o seu ficheiro «{ $part }» tem mais de { $limit } níveis de aninhamento. Nenhum projeto Manuskript se aproxima disso, e lê-lo poderia fazer o Skribisto falhar. Nada foi escrito.
import-manuskript-folders-too-deep = O projeto não foi importado: um dos seus ficheiros está a mais de { $limit } pastas de profundidade. Nenhum projeto Manuskript se aproxima disso, e lê-lo poderia fazer o Skribisto falhar. Nada foi escrito.
import-manuskript-declares-entities = O projeto não foi importado: o seu ficheiro «{ $part }» declara entidades XML, o que nenhum projeto Manuskript faz. Expandi-las poderia exigir mais memória do que o computador tem e fazer o Skribisto falhar. Nada foi escrito.
import-manuskript-error-details = Detalhes

import-target-open-title = Este projeto está aberto
import-target-open-text = «{ $name }» está aberto no Skribisto. Importar por cima dele substituiria o projeto em que está a trabalhar, e a próxima gravação escreveria esse projeto de volta por cima da importação. Escolha outro nome de ficheiro, ou feche primeiro o projeto.
import-warnings-announcement = { $result } { $notice }.

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
import-diagnostic-image-not-ingested = «{ $path }» refere a imagem «{ $detail }». A referência chega como texto; a imagem em si não é copiada.
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
