# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

menu-work = &Projeto
menu-new-work = N&ovo projeto
menu-open-work = &Abrir projeto…
menu-new-window = &Nova janela
menu-import-from = &Importar de
menu-create-from = &Criar a partir de
menu-import-plume = &Plume Creator (.plume)…
menu-import-document = &Documentos (Markdown, Palavra, ODT)...
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
menu-scene-break = Ins&erir quebra de cena
menu-major-scene-break = Inserir quebra &de cena principal

menu-go = &Ir
menu-go-next-scene = &Próxima Cena
menu-go-prev-scene = &Cena Anterior
menu-go-next-chapter = Cap&ítulo Seguinte
menu-go-prev-chapter = C&apítulo anterior
menu-go-next-note = N&ota Seguinte
menu-go-prev-note = &Nota anterior

menu-tools = Fe&rramentas
menu-help = &Ajuda

ctx-add = &Adicionar
ctx-new-item = &Novo elemento
ctx-new-folder = &Nova pasta
ctx-rename = &Mudar o nome
ctx-duplicate = &Duplicar
ctx-import-here = &Importar aqui…
ctx-indent = &Aumentar avanço
ctx-outdent = Dimin&uir avanço
ctx-trash = M&over para o lixo
ctx-open-to-side = A&brir para o Lado
ctx-open = &Abrir
ctx-reveal-in-outline = &Revelar em Contexto
ctx-move-up = &Subir
ctx-move-down = &Descer

ctx-tab-close = &Fechar
ctx-tab-close-others = F&echar os outros
ctx-tab-close-all = Fe&char todos
ctx-tab-open-to-side = &Abrir para o Lado
ctx-tab-open-in-main = &Abrir no painel principal
ctx-tab-move-to-side = &Mover para o lado
ctx-tab-move-to-main = &Mover para o painel principal
ctx-tab-move-to-new-window = M&over para uma nova janela
ctx-tab-move-window-unsaved = Gravar este projecto primeiro: uma segunda janela abre para um ficheiro no disco
ctx-tab-pin = &Pintar esta aba
ctx-tab-unpin = &Desbloquear esta página
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
create-story-bible-entry = Entrada da bíblia narrativa…
type-book-start = Início do livro
type-text = Texto

new-item-book = Novo livro
new-item-part = Nova parte
new-item-chapter = Novo capítulo
new-item-note = Nova nota
new-item-note-folder = Nova pasta de notas
new-item-folder = Nova pasta
new-item-paratext = Novo Paratexto
new-item-paratext-folder = Nova Pasta de Paratexto
new-item-story-bible-entry = Entrada da Bíblia em Nova História

placement-inside = dentro
placement-after = após
placement-after-parent = após a mãe
placement-top-level = nível superior


ctx-promote = &Converter em
promote-chapter-folder = Capítulo em pasta
promote-flat-chapter = Capítulo simples
promote-lossy-title = Nenhum lugar para manter o texto
promote-lossy-text = Um { $target } não tem onde guardar estes: { $kinds }. Mova ou limpe esse texto primeiro, depois converta.
content-scene-text = Texto da cena
content-note-text = Texto da nota
content-book-title = Título do livro
content-book-subtitle = Legenda do livro
content-part-title = Título da parte
content-chapter-title = Título do capítulo
content-epigraph-text = Epígrafe
content-paratext-text = Paratexto
promote-blocked-title = Capítulo não vazio
promote-blocked-text = { $count ->
    [one] Este capítulo ainda contém { $count } item. Mova-o ou destrua-o antes de convertê-lo para um capítulo plano.
   *[other] Este capítulo ainda contém { $count } itens. Mova-os ou destrua-os antes de convertê-los para um capítulo plano.
}

inspector = Propriedades
inspector-empty = Abra um item para inspecioná-lo.
inspector-promote = Converter em…
statusbar-toggle-outline = Mostrar ou ocultar o organizador
statusbar-toggle-inspector = Mostrar ou ocultar as propriedades
statusbar-save-unsaved = Alterações por gravar. Carregue para gravar
statusbar-save-saved = Todas as alterações salvas
statusbar-save-autosave = Gravar automaticamente está ligado. As alterações são gravadas à medida que escreve
statusbar-saving = A guardar…
statusbar-word-count = { $count ->
    [one] { $count } palavra
   *[other] { $count } palavras
}
statusbar-word-count-tooltip = Palavras na cena que estás a editar
statusbar-word-char-count = { $words ->
    [one] { $words } palavra
   *[other] { $words } palavras
} · { $chars ->
    [one] { $chars } caracteres
   *[other] { $chars } carateres
}
session-toggle = Sessão de gravação: iniciar ou pausar um sprint focado
session-configure = Definir o objetivo e o limite de tempo da palavra da sessão
session-configure-title = Sessão de gravação
session-word-goal = Objetivo de palavras
session-time-limit = Prazo
session-time-limit-unit = min
session-no-goal = Sem golo
session-no-limit = Sem limite
session-reset = Reiniciar sessão
session-readout = { $words } palavras · { $time }
session-readout-timed = { $words } palavras · { $time } esquerda
statusbar-focus-exit = Sair do modo livre de distracção
statusbar-focus-synopsis = Sinopse
statusbar-focus-go-prev = Rubrica anterior (Alt+Up)
statusbar-focus-go-next = Próximo item (Alt+Down)

rail-settings = Definições

language = Idioma
theme = Tema
english = Inglês
french = Francês
light = Claro
dark = Escuro
settings-text-width = Largura do texto
settings-preview-width = Largura da antevisão da pesquisa
settings-autosave = Gravar automaticamente no disco
settings-show-welcome = Mostrar o lançador na inicialização
settings-show-welcome-tip = Quando desligado, o último projeto é reaberto.
settings-check-for-updates = Verificar as novas versões
settings-check-for-updates-tip = Uma vez por dia, no máximo, Skribisto pergunta ao site do projeto qual versão é atual. A solicitação não carrega nenhum identificador, e nem mesmo a versão que você está executando: a comparação acontece neste computador.

settings-title = Definições
settings-close = Fechar
settings-search = Definições de pesquisa
settings-reset = Repor predefinições
settings-done = Concluído
settings-cancel = Cancelar
settings-apply = Aplicar
settings-ok = OK
settings-reset-confirm-title = Reiniciar todas as definições como padrão?
settings-reset-confirm-body = Restaura as definições de aparência, editor e escrita deste aplicação. As definições do seu próprio projeto, atalhos de teclado e estilos guardados não são afetadas. Ele não pode ser desfeito.
settings-empty-title = Sem definições aqui ainda
settings-empty-hint = Esta seção ganhará opções em uma atualização futura.

settings-sec-appearance-behaviour = Comportamento de Aparência
settings-sec-editor = Editor
settings-sec-spelling = Ortografia
settings-sec-backup = Sincronização da Cópia de Segurança
settings-sec-compile = Compilação e exportação
settings-page-appearance = Aparência
settings-field-image-size-policy = Imagens grandes
settings-image-policy-ask = Perguntar de cada vez
settings-image-policy-keep = Manter original
settings-image-policy-downscale = Otimizar
settings-hint-image-size-policy = O que fazer quando uma imagem inserida é maior do que o manuscrito precisa.
settings-page-notifications = Notificação
settings-page-scene = Cena
settings-page-synopsis = Sinopse
settings-page-notes = Notas
settings-page-editor-behavior = Comportamento do editor
settings-page-goals = Metas Contagem de Palavras
settings-page-games = Gravando jogos
settings-page-corkboard = Quadro de cartões
settings-page-distraction-free = Livre de distração
settings-page-dictionaries = Dicionários
settings-page-autosave = Gravação automática
settings-page-export = Formatos de exportação
settings-page-paratext = Estruturas de Paratexto
settings-page-keymap = Atalhos de teclado
settings-keymap-filter = Filtrar atalhos

settings-desc-sec-appearance-behaviour = Como o aplicação em si parece, e o que ele faz quando começa.
settings-desc-sec-editor = A superfície da escrita: como ela parece, e o que ela faz como você digita.
settings-desc-sec-spelling = Verificação ortográfica, e os dicionários por trás disso.
settings-desc-sec-backup = Como seu trabalho atinge o disco, e quais cópias são mantidas.
settings-desc-sec-compile = O que deixa Skribisto, e em que forma.
settings-desc-sec-work = Definições que pertencem a este projeto e viajam dentro de seu ficheiro.
settings-desc-sec-extensions = Páginas contribuídas pelas extensões instaladas aqui.
settings-desc-group-typography = Uma página por tipo de texto: tipo, tamanho, altura da linha e espaçamento.
settings-desc-appearance = Idioma da interface, tema, tamanho do texto e janela inicial.
settings-desc-notifications = Cada mensagem que esta sessão tem mostrado, e as ações que você pode reproduzir.
settings-desc-scene = Como a prosa da cena é definida: tipo, tamanho, altura da linha, indentações e espaçamento.
settings-desc-synopsis = Como o painel de sinopse é definido, independentemente do manuscrito.
settings-desc-notes = Como as notas são definidas, independentemente do manuscrito.
settings-desc-corkboard = Tamanho de cartão, o que um cartão mostra, e como o conselho organiza-los.
settings-desc-distraction-free = Tipografia, largura da coluna e tira de controle para escrita em ecrã cheia.
settings-desc-distraction-free-themes = A biblioteca de temas para escrita em ecrã cheia: as enviadas e as suas próprias.
settings-desc-editor-behavior = Largura do texto, posição da sinopse, rolagem da máquina de escrever e o destaque do cuidado.
settings-desc-punctuation = O estilo tipográfico da casa iniciam- se novos projectos. Cada projecto segue isto, a menos que defina os seus próprios projectos.
settings-desc-goals = Alvos de palavras e personagens, e como as palavras são contadas.
settings-desc-games = Restrições que você definir-se durante a elaboração, como Sempre para a frente.
settings-desc-spellcheck = O único interruptor que liga e desliga por todo o lado.
settings-desc-dictionaries = Instale, remova e navegue os dicionários ortográficos nesta máquina.
settings-desc-autosave = Se as suas edições são gravadas no disco por conta própria.
settings-desc-backup = Quando as cópias são tiradas, onde são mantidas, e quantos. Cada projeto segue isto, a menos que ele define o seu próprio.
settings-desc-export = Os estilos cada exportação compila através: os enviados e o seu próprio.
settings-desc-paratext = A matéria dianteira e traseira um novo projeto pode começar com.
settings-desc-keymap = Cada atalho, e o que é obrigado a fazer.
settings-page-user = Utilizador
settings-desc-user = Quem tu és, pelos comentários que escreves.
settings-group-identity = Identidade
settings-field-user-name = O seu nome.
settings-field-user-name-placeholder = Opcional
settings-field-user-name-hint = Assina os comentários e respostas que escreve. Distingue- se do nome de autor de um projecto, que é a linha do livro e viaja dentro do ficheiro.
settings-field-user-initials = Suas iniciais
settings-field-user-initials-placeholder = Do teu nome
settings-field-user-initials-hint = O que um processador de texto mostra ao lado do seu comentário na margem. Deixe- o em branco para usar os mostrados, retirados do seu nome.
settings-field-user-hint = Ambos são opcionais e aplicam- se a todos os projectos deste computador. Mudando- os assina os comentários que escreve a seguir. Os comentários já escritos mantêm o nome em que foram escritos.
settings-desc-author = O nome que vai neste projecto.
settings-desc-structure = Se os capítulos deste projeto são pastas ou itens individuais.
settings-desc-language = A linguagem da prosa deste projeto é verificada.
settings-desc-work-backup = A própria política de apoio deste projecto, ou a política geral.
settings-desc-personal-dictionary = As palavras que este projeto trata como corretamente soletrado.
settings-desc-tags = Os rótulos codificados por cores com os quais este projecto marca os seus itens.
settings-desc-templates = Os modelos de notas que você pode inserir enquanto escreve neste projeto.
settings-desc-text-replacements = Atalhos que se expandem à medida que você digita neste projeto.
settings-desc-work-punctuation = Seguem-se as citações, os traços e o espaçamento da prosa deste projeto.

settings-group-typography = Tipografia
settings-group-writing-column = Gravando coluna
settings-group-writing-view = A gravar a janela
settings-group-distraction-free-strip = Faixa de controle
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
editor-size-changed-corkboard = Tamanho do texto do cartão de cortiça: { $percent }
editor-size-changed-corkboard-expanded = Tamanho de texto do editor expandido: { $percent }
editor-size-changed-distraction-free = Tamanho de texto livre de distração: { $percent }

menu-text-size-increase = &Aumentar o tamanho do texto
menu-text-size-decrease = &Diminuir o tamanho do texto
menu-text-size-reset = &Reiniciar o tamanho do texto
settings-field-line-height = Altura da linha
settings-field-first-line-indent = Indentação de primeira linha
settings-field-paragraph-spacing-before = Espaço antes do parágrafo
settings-field-paragraph-spacing-after = Espaço após o parágrafo
settings-field-column-width = Largura da coluna
settings-distraction-free-width-hint = Aplica-se apenas enquanto o modo livre de distração estiver ligado. As larguras das colunas Cena, Sinopse e Notas não são tocadas.
settings-distraction-free-title = Manter o nome do item
settings-distraction-free-word-count = Manter a contagem de palavras
settings-distraction-free-session = Manter a sessão de escrita
settings-distraction-free-go-to = Mantenha o botão Ir para...
settings-distraction-free-go = Manter os botões Anterior e Próximo
settings-distraction-free-chrome-hint = O botão Sair fica sempre, o que você escolher aqui. É a sua saída se Escapar estiver ocupado.
settings-field-app-theme = Tema
settings-theme-light = Claro
settings-theme-dark = Escuro
settings-field-text-scale = Tamanho do texto da interface
settings-field-language = Linguagem da interface
settings-synopsis-placement = Posição da sinopse
settings-synopsis-placement-none = Nenhum
settings-synopsis-placement-top = Acima
settings-synopsis-placement-side = Ao lado
settings-typewriter = Rolagem da máquina de escrever
settings-typewriter-tip = Mantém a linha que você está escrevendo em uma altura fixa enquanto o manuscrito rola por baixo dele. Clicando ainda coloca o cuidado onde você clica.
settings-typewriter-position = Posição da linha
settings-typewriter-position-top-third = Terceiro superior
settings-typewriter-position-middle = Médio
settings-typewriter-position-bottom-quarter = Trimestre inferior
settings-highlight-scope = Realce em torno do cuidador
settings-highlight-scope-none = Nenhum
settings-highlight-scope-sentence = Sentença
settings-highlight-scope-paragraph = Parágrafo
settings-highlight-scope-tip-none = Deixe a página simples. Nada está sombreado como você escreve.
settings-highlight-scope-tip-sentence = Solucione a frase que está a escrever, por isso, destaca-se dos que a rodeiam.
settings-highlight-scope-tip-paragraph = Realça o parágrafo em que está a escrever, para manter visível a passagem em que está a trabalhar.
settings-group-container-views = Vistas dos contentores
settings-remember-view = Recordar a última vista para cada tipo de item
settings-remember-view-tip = Abra um contentor na janela que você usou pela última vez para esse tipo
settings-remember-view-tip-more =
    Um livro, parte e capítulo cada oferecer várias vistas (sua própria página, o
    manuscrito, a sinopse completa). Ligue isto e cada tipo reabre sobre o
    view que você escolheu por último, por exemplo, mude um Capítulo para Capítulo Completo e o
    próximo capítulo você abre mostra Capítulo completo também. Cada tipo se lembra de sua própria visão.
settings-group-counting = Contagem de palavras
settings-counting-auto = Automático (por idioma)
settings-counting-whitespace = Dividir nos espaços
settings-counting-unicode-words = Palavras Unicode
settings-counting-cjk-hybrid = Ciente de CJK (por caractere)
settings-counting-hint = Contagem automática em chinês e japonês por personagem e todas as outras línguas por palavra. Mude-o apenas se a contagem parecer errada para o seu idioma.
settings-group-goals-display = Visualização
settings-show-characters = Mostrar o número de caracteres na barra de estado
settings-autosave-hint = As alterações são escritas para o disco automaticamente enquanto você escreve.

settings-sec-work = Projeto
settings-page-structure = Estrutura
settings-page-author = Autor
settings-field-author-name = Nome do autor
settings-field-author-placeholder = Opcional
settings-field-author-hint = Aparece na página de título compilada e nos metadados de arquivos exportados. Deixe- o em branco para omiti- lo. Este é o byline do livro. O nome com o qual seus comentários são assinados está em Definições ▸ Utilizador.
settings-group-chapters = Capítulos
tidy-titles-title = Títulos de capítulos arrumados
tidy-titles-none = Nenhum título de capítulo ou parte é apenas repetir o seu próprio número.
tidy-titles-lead = { $count ->
        [one] Um capítulo ou parte é intitulado apenas com seu próprio número.
       *[other] { $count } capítulos e partes são intitulados com nada mais do que seu próprio número.
    }
tidy-titles-explain = Limpar esses títulos deixa cada um com o nome do número que o livro já sabe, o mesmo número que as impressões de exportação. Nada mais muda, e um desfazer coloca- as todas de volta.
menu-document-tidy-titles = &Títulos de capítulos arrumados...
settings-group-numbering = Numeração
settings-number-chapters = Capítulos e partes de números
settings-number-chapters-tip = Capítulos e peças carregam um número elaborado a partir de onde eles se sentam no livro, mostrado ao lado de seu título aqui, e impresso pela exportação.
settings-number-chapters-tip-more = O número nunca é armazenado no título, por isso permanece direito quando você reordenar, inserir ou excluir. Desligue isso e os títulos de impressão de exportação sozinho, seja qual for o estilo de exportação que pedir. Para deixar um capítulo de fora (um prólogo, um interlúdio), use o botão de Numeração do Inspetor em vez disso: que o mantém no livro, mas o impede de usar um número.
settings-part-resets-chapter = Reiniciar os números dos capítulos em cada parte
settings-part-resets-chapter-tip = Off por padrão: capítulos correr em linha reta através das partes de um livro, então "Parte Dois" abre no Capítulo Onze.
settings-part-resets-chapter-tip-more = Esta é a prática comercial habitual, e o que um leitor espera. Ligue-o para um livro cujas partes são destinadas a ler como volumes separados, onde cada um abre no Capítulo Um.
settings-chapter-flat = Capítulos simples
settings-chapter-flat-hint = Em: um capítulo é um único item. Você escreve nele, e ele não contém cenas. Desligado: um capítulo é uma pasta. Você ainda escreve nele, mas ele também pode conter cenas. Novos capítulos seguem esta configuração; os já existentes convertem- se via Promove.

settings-styles-builtin = Estilos incorporados
settings-styles-user = Os meus estilos
settings-styles-builtin-badge = Incorporado
settings-styles-duplicate = Duplicar
settings-styles-edit = Editar
settings-styles-delete = Eliminar
settings-styles-import = Importar...
settings-styles-export = Exportar...
settings-styles-copy-suffix = (cópia)
settings-styles-json-filter = Estilo de exportação
settings-styles-editor-title = Editar o estilo
settings-styles-editor-none = Selecione um estilo personalizado para o editar ou duplique um estilo integrado.
settings-styles-imported = Estilo importado
settings-styles-import-failed = Não foi possível importar o estilo
settings-styles-exported = Estilo exportado
settings-styles-export-failed = Não foi possível exportar o estilo
settings-styles-field-name = Nome
settings-styles-field-chapters = Rubricas dos capítulos
settings-styles-field-parts = Títulos das partes
settings-styles-field-scene-break = Quebra de cena
settings-styles-field-major-scene-break = Grande quebra de cena
settings-styles-field-spacing = Espaçamento de linhas
settings-styles-field-justify = Justificar texto
settings-styles-field-notes = Incluir notas
settings-styles-field-synopses = Incluir as sinopses
settings-styles-field-scene-titles = Incluir os títulos das cenas
settings-styles-field-epigraphs = Incluir epígrafes
settings-styles-field-paratexts = Incluir paratextos
settings-styles-field-epigraph-placement = Posição epígrafe
settings-styles-epigraph-after = Após o título
settings-styles-epigraph-before = Antes do título
settings-styles-field-footnotes = Incluir notas de rodapé
settings-styles-field-footnote-numbering = Numeração da nota de rodapé
settings-styles-footnote-numbering-continuous = Continua
settings-styles-footnote-numbering-per-chapter = Reiniciar cada capítulo
settings-styles-footnote-numbering-per-book = Reiniciar cada livro
settings-styles-field-images = Imagens
menu-image = I&magem
settings-styles-images-beside = Ao lado do documento
settings-styles-images-embed = Dentro do documento
settings-styles-images-omit = Deixa-te de fora.
settings-styles-group-round-trip = Envio a um revisor
settings-styles-field-comments = Incluir comentários
settings-styles-field-round-trip-marks = Incluir marcadores de reimportação
settings-styles-round-trip-hint = Apenas DOCX e ODT. Estes marcadores invisíveis permitem atualizar o projeto ao reimportar o ficheiro revisto, em vez de criar outra cópia.
settings-styles-group-pages = Páginas
settings-styles-field-cover = Abrir com a capa
settings-styles-field-word-count = Contagem de palavras na página de título
settings-styles-field-page-books = Nova página em cada livro
settings-styles-field-page-parts = Nova página em cada parte
settings-styles-field-page-chapters = Nova página em cada capítulo
settings-styles-field-page-paratexts = Nova página em cada paratexto
settings-styles-heading-none = Sem posição
settings-styles-heading-numbered = Apenas número
settings-styles-heading-title = Apenas título
settings-styles-heading-both = Número + título
settings-styles-break-blank = Linha em branco
settings-styles-break-none = Nenhum
settings-styles-spacing-single = Única
settings-styles-spacing-onehalf = 1½
settings-styles-spacing-double = Duplo

welcome-title = Bem-vindo ao Skribisto
welcome-version = Versão { $version }
welcome-update-available = Versão { $version } está disponível
welcome-update-tip = Abre a página de download no seu navegador.
welcome-search = Trabalhos de pesquisa
welcome-open = Abrir
welcome-new-work = Novo projeto
welcome-create-from = Criar a partir de…
welcome-recent-works = Projetos recentes
welcome-empty-recents = Ainda não há trabalhos recentes.
welcome-no-matches = Nenhum trabalho recente corresponde à sua pesquisa.
welcome-learn-soon = Guias e dicas estão chegando em breve.
welcome-about-blurb = Skribisto, a Rust + Teksilo reescrever o aplicação de escrita.
welcome-tagline = Um lugar tranquilo para escrever coisas longas.
welcome-github = GitHub
welcome-discord = Discórdia
nav-works = Projectos
nav-examples = Exemplos
nav-learn = Aprender
nav-about = Sobre

synopsis = Sinopse
epigraph = Epígrafe
pane-manuscript = Manuscrito
corkboard = Quadro de cartões
overview = Visão geral

corkboard-card-count = { $count ->
    [one] Cartão { $count }
   *[other] { $count } cartas
}
corkboard-child-count = { $count ->
    [one] { $count } ponto
   *[other] { $count } pontos
}
corkboard-view-nested = Aninhado
corkboard-view-flat = Plana
corkboard-layout-hint = Aninhado mostra as elementos diretamente contidos de um contentor. Abra um cartão de pasta para entrar nele. Flat mostra todas as cenas em todo o contentor de uma vez.
corkboard-card-size = Tamanho da carta
corkboard-search-placeholder = Filtrar as cartas...
corkboard-empty-title = Ainda nada aqui.
corkboard-empty-hint = Use “+ Novo” acima para adicionar o primeiro.
corkboard-new = Novo
corkboard-show-card-numbers = Número das cartas
corkboard-modal-size = Tamanho expandido do editor
corkboard-scope-hint = Aplica-se a cada quadro aberto, em cada projeto.
corkboard-sort-manuscript = Ordem manuscrita
corkboard-sort-title-asc = Título A–Z
corkboard-sort-title-desc = Título Z–A
corkboard-card-number = Cartão { $number }

duplicate = Duplicar
duplicate-n = { $count ->
    [one] Cartão duplicado { $count }
   *[other] Placas duplicadas { $count }
}
set-label-n = { $count ->
    [one] Definir o rótulo na placa { $count }
   *[other] Definir o rótulo das { $count } cartas
}
move-to-trash-n = { $count ->
    [one] Mover { $count } carta para o lixo
   *[other] Mover { $count } cartas para o lixo
}
reveal-in-outline = Revelar em linhas gerais

corkboard-move-to = Mover para...
corkboard-move-to-n = { $count ->
    [one] Mover { $count } cartas para...
   *[other] Mover { $count } cartas para...
}
corkboard-move-picker-title = Mover para...
corkboard-move-picker-empty = Sem fichários ainda. Crie um primeiro.
corkboard-move-picker-cancel = Cancelar
corkboard-move-here = Mover para Aqui
corkboard-moved-ok = { $count ->
    [one] { $count } cartão movido
   *[other] { $count } cartas movidas
}
corkboard-move-into-self = Um contentor não pode ser movido dentro de si. Escolha um destino fora dele.
corkboard-move-failed = Essas cartas não podiam ser movidas para lá.

overview-col-title = Título
overview-col-type = Tipo
overview-col-label = Legenda
overview-col-tags = Etiquetas
overview-col-books = Livros
overview-col-own-words = Palavras
overview-col-total-words = Total
overview-row-count = { $count ->
    [one] { $count } ponto
   *[other] { $count } pontos
}
overview-search-placeholder = Filtrar os itens...
overview-expand-all = Expandir tudo
overview-collapse-all = Recolher tudo
overview-table-label = Sumário
overview-empty-title = Ainda nada aqui.
overview-gone-title = Este contentor desapareceu.
overview-gone-hint = Foi movido para o Lixo. Restaure-o ou feche esta página.
overview-empty-hint = Use “+ Novo” acima para adicionar o primeiro.
corkboard-grid-label = Cartas de cartão de cortiça
corkboard-rename-field = Mudar o nome do elemento
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
settings-group-corkboard-cards = Cartas
corkboard-show-word-count = Mostrar a contagem de palavras nas cartas
text-heading = Texto
no-content = Este item não tem conteúdo editável.
untitled = Sem título
placeholder-title = Título…
placeholder-subtitle = Subtítulo…
placeholder-chapter-title = Título do capítulo...
split-editor = Dividir editor
close-split-view = Fechar a visualização dividida
drop-open-here = Abrir aqui
drop-open-to-side = Abrir para o lado

segment-chapter = Capítulo
segment-part = Parte
segment-book = Livro
segment-pace = Ritmo
segment-notes = Notas
segment-story-bible = Bíblia narrativa
segment-note-own = Nota
segment-note-details = Detalhes
segment-note-in-prose = Em prosa
pace-placeholder = O planejador Pace aparece aqui.
pace-empty-title = Planeje o ritmo deste livro
pace-empty-body = Definir um objetivo de palavra e um prazo, e Skribisto funciona o ritmo diário para chegar lá.
pace-start-planning = Iniciar o planeamento
pace-section-schedule = Agendar
pace-section-progress = Progresso
pace-advancement = Avançar
pace-goal = Objetivo de palavras
pace-deadline = Prazo
pace-active = Plano de ritmo ativo
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
pace-card-streak = raia do dia
pace-card-ahead = palavras à frente do ritmo
pace-card-behind = palavras atrás do ritmo
pace-charts-empty = Os gráficos de progresso aparecem aqui uma vez que a sua contagem de palavras é gravada numa gravação.
pace-chart-progression = Palavras escritas vs. alvo
pace-chart-words-per-day = Palavras por dia
pace-series-actual = Real
pace-series-target = Objetivo
pace-series-words-per-day = Palavras/dia
pace-daily-target-line = Mesmo ritmo: { $count ->
    [one] { $count } palavra/dia
   *[other] { $count } palavras/dia
}
pace-section-holidays = Feriados
pace-section-milestones = Marcos
pace-holidays-none = Não há férias, todos os dias marcados contam.
pace-holiday-label = Nome do feriado
pace-add-holiday = Adicionar
pace-remove = Remover
pace-milestones-none = Não há marcos ainda. Coloque um em uma parte ou capítulo no Inspetor.
full-chapter = Capítulo completo
full-part = Parte Completa
full-book = Livro completo
full-synopsis = Sinopse Completa
rename = Mudar o nome
set-label = Definir a legenda
insert-scene = Inserir cena
insert-chapter = Inserir capítulo
split-scene = Dividir a cena
move-up = Subir
move-down = Mover para baixo
merge-with-previous = Juntar com o anterior
move-to-trash = Mover para o lixo
rename-chapter = Mudar o nome do capítulo
add-scene = Adicionar nova cena
add-chapter = Adicionar um novo capítulo
new-scene-title = Nova Cena
placeholder-scene-name = Nome da cena...
menu-cut = Cortar
menu-copy = Copiar
menu-paste = Colar
menu-paste-unformatted = Colar Não Formatado
menu-select-all = Seleccionar Tudo
format-bold = Negrito
format-italic = Itálico
format-underline = Sublinhado
format-strikethrough = Rasurado

binder = Organizador
no-work = Sem trabalho carregado
no-recent-works = Nenhum trabalho recente
switcher-open-section = Aberto atualmente
switcher-recent-section = Recente
switcher-this-window = esta janela
open-project-title = Abrir projeto
open-project-question = Como você quer abrir “{ $title }”?
open-in-new-window = Abrir numa nova janela
open-here = Abrir aqui

binder-all = Todos os organizadores
binder-show-all = Mostrar todos os arquivos
binder-new = Nova pasta...
binder-default-name = Novo fichário
binder-item-count = { $count ->
    [one] { $count } ponto
   *[other] { $count } pontos
}
binder-search-placeholder = Filtrar o contorno...
binder-search-scope = Procurar todos os ficheiros
binder-trash-confirm-title = Mover o ficheiro para o lixo?
binder-trash-confirm-text = “{ $name }” e todos os seus itens serão movidos para o lixo.

dialog-rename = Mudar o nome
dialog-new-binder = Novo fichário
dialog-set-label = Definir a legenda
dialog-new-scene = Nova cena
close-work-question = Gravar as alterações antes de fechar o trabalho?
quit-question = Gravar as alterações antes de sair?
quit-save-work-question = Guardar mudanças para { $title } antes de sair?
unsaved-changes = Este trabalho tem mudanças não salvas.
new-work-unsaved-question = Gravar as alterações antes de criar um novo trabalho?
open-work-unsaved-question = Gravar as alterações antes de abrir outro trabalho?
switch-backup-discard-title = Descartar alterações nesta cópia de segurança?
switch-backup-discard-text = As alterações em um backup não podem ser salvas nele. Use Guardar como ou Restaurar para mantê-los, ou descartá-los e abrir o outro trabalho.
switch-save-failed = O trabalho não pôde ser guardado, então não foi substituído: { $error }
switch-save-not-started = O trabalho não pôde ser guardado, por isso não foi substituído.
close-save-failed = O trabalho não pôde ser guardado, então não foi fechado: { $error }
close-save-not-started = O trabalho não pôde ser guardado, por isso não estava fechado.
save-not-started = O trabalho não pôde ser guardado.

could-not-open-work = Não foi possível abrir "{ $file }"
could-not-open-work-too-new = "{ $file }" precisa de um novo Skribisto
could-not-open-work-too-new-detail = Guardado por Skribisto formato { $written_by }; abrindo ele precisa de formato { $requires } ou mais recente, e esta compilação suporta até o formato { $supported }. Atualizar Skribisto para abri-lo.
could-not-open-work-importing = "{ $file }" está sendo escrito por uma importação
could-not-open-work-importing-detail = Uma importação está escrevendo sobre este projeto e ainda não terminou. Abra- o uma vez que a importação seja feita: aberto agora, ele mostraria o projeto que a importação está substituindo, e sua próxima gravação escreveria esse projeto de volta sobre a importação.
could-not-open-example = Não foi possível abrir o exemplo: { $error }
could-not-open-link = Não foi possível abrir { $url }: { $error }
could-not-create-work = Não foi possível criar trabalho: { $error }
saving-as-file = A poupar como { $target }...
saving-as-folder = A poupar como { $target }/...
saved-as = Guardado em { $target }
save-error = Não foi possível guardar: { $error }
save-flush-failed = O salvamento passou, mas esses documentos não puderam ser entregues a ele e ainda não foram guardados: { $titles } ({ $error })
backup-error = Não foi possível fazer backup: { $error }
backing-up = A fazer backup...
backup-nothing-open = Nenhum projeto está aberto para fazer backup.
backup-already-running = Um backup já está em andamento.
backup-complete = Cópia de segurança completa ({ $ok } guardado, { $skipped } já atual)
backup-partial = A cópia de segurança terminou: { $ok } guardado, { $failed ->
    [one] { $failed } destino falhou
   *[other] { $failed } destinos falhados
}
backup-no-destination-title = Nenhum local de backup disponível
backup-no-destination-text = Nenhum dos destinos de backup configurados pode ser alcançado (por exemplo, uma unidade externa pode ser desconectada). Conecte-a e tente novamente ou saia sem fazer backup.
backup-failed-close-title = A cópia de segurança não pôde ser salva
backup-failed-close-text = Nenhuma cópia de backup poderia ser escrita antes de fechar. Todo destino falhou (a unidade pode ter sido removida, ou pode estar cheia ou protegida por gravação). Corrigir e tentar novamente, ou sair sem fazer backup.

backup-choice-title = Ficheiro de cópia de segurança
backup-choice-heading = Você abriu uma cópia de backup
backup-choice-subtitle = Este é um backup pontual de um projeto.
backup-choice-subtitle-dated = Reforços capturados { $date }.
backup-choice-body = Você pode abri-lo e editar livremente, mas as alterações só podem ser mantidas com o Save As. O ficheiro do projeto original não é tocado. Ou restaurar este projeto para exatamente este backup.
backup-choice-open = Abrir a cópia de segurança
backup-choice-restore = Restaurar este projeto até este ponto...
backup-choice-not-a-backup = Não, abre normalmente.
backup-banner-title = Cópia de segurança: as alterações não podem ser salvas aqui
backup-banner-description = Use Guardar como para manter suas edições em um novo ficheiro, ou Restaurar para substituir o projeto original com este backup.
backup-banner-restore = Restaurar…
backup-banner-save-as = Guardar como...
backup-restore-original-missing = Não foi possível encontrar o projeto original para restaurar. Use Guardar como para manter este backup como um novo projeto.
backup-restore-close-elsewhere-title = Projeto aberto em outra janela
backup-restore-close-elsewhere-text = O projeto que você está restaurando está aberto em outra janela. Por favor, feche-o lá primeiro, então tente novamente.
backup-restore-focus-window = Foco nessa janela
backup-restore-confirm-title = Restaurar este backup?
backup-restore-confirm-text = A versão atual do projeto será copiada de lado como um backup de segurança antes de ser substituído por este.
backup-restore-confirm-ok = Restaurar
backup-restore-error = Não foi possível restaurar: { $error }
backup-restored-ok = Projecto restaurado.
backup-restored-with-safety = O projecto foi restaurado. A sua versão anterior foi gravada para { $path }.
close-backup-discard-title = Descartar alterações nesta cópia de segurança?
close-backup-discard-text = As alterações a uma cópia de segurança não podem ser salvas nela. Use Guardar como para mantê-las, ou descartá-las e fechá-las.
quit-backup-discard-title = Descartar as alterações e desistir?
quit-backup-discard-work-question = Descartar as alterações para { $title } e desistir?
quit-backup-discard-text = As alterações a uma cópia de segurança não podem ser gravadas nela. Use o Save As para mantê- las, ou descarte e demita- as.
backup-nudge-text = Não estão configurados backups para este projeto.
backup-nudge-action = Configurar backups...

menu-backups-list = C&ópias de segurança...
backups-title = Cópias de segurança
backups-loading = A procurar cópias de segurança…
backups-empty = Ainda não foram encontrados backups para este projeto.
backups-open = Abrir
backups-reveal = Mostrar localização
backups-delete = Apagar esta cópia de segurança
backups-delete-confirm-title = Apagar esta cópia de segurança?
backups-delete-confirm-text = “{ $name }” será excluído permanentemente. Isto não pode ser desfeito.
backups-delete-error = Não foi possível excluir o backup: { $error }
backups-refresh = Atualizar
backups-close = Fechar

settings-page-backup = Por omissão da cópia de segurança
settings-page-work-backup = Cópias de segurança
settings-backup-general-title = Definições de backup padrão
settings-backup-work-title = Cópias de segurança para este projeto
settings-backup-inherit = Usar as definições gerais de backup
settings-backup-inheriting = Este projeto usa as definições gerais de backup.
settings-backup-none-hint = Não são configurados backups automáticos (cada gatilho está desligado).
settings-backup-last = Última cópia de segurança: { $date }
settings-backup-last-never = Ainda não há reforços.
settings-backup-open-list = Abrir a lista de cópias de segurança...
settings-backup-triggers = Quando fazer backup
settings-backup-on-close = Fazer backup ao fechar o projeto
settings-backup-on-open = Recuar ao abrir o projeto
settings-backup-interval = Recuar periodicamente, cada
settings-backup-destinations = Destinos de backup
settings-backup-default-location = Localização padrão
settings-backup-usage-since = { $count ->
    [one] { $count } cópia de segurança · { $size } · mais antiga { $oldest }
   *[other] { $count } cópias de segurança · { $size } · mais antiga { $oldest }
}

settings-backup-usage = { $count ->
    [one] { $count } cópia de segurança · { $size }
   *[other] { $count } cópias de segurança · { $size }
}

settings-backup-usage-empty = Ainda não há nada aqui.
settings-backup-usage-measuring = Medindo...
settings-backup-reveal-root = Mostrar pasta
settings-backup-dest-none = Nenhum destino. As cópias de segurança são salvas ao lado do projeto.
settings-backup-dest-remove = Remover
settings-backup-dest-add = Adicionar pasta...
settings-backup-dest-refresh = Atualizar
settings-backup-retention = Quantos para guardar?
settings-backup-retention-tiered = Por níveis
settings-backup-retention-keep-n = Manter o último N
settings-backup-retention-tip = Como os backups antigos são podados.
settings-backup-retention-tip-more = Em camadas mantém um backup por hora para um dia, por dia para uma semana, por semana para um mês, e por mês além, por isso a história recente permanece densa e velha história diminui. “Manter último N” simplesmente mantém as cópias mais recentes N. Em ambos os modos as novas cópias (o mínimo abaixo) são sempre mantidas.
settings-backup-gfs-hourly = Por hora (últimas 24 h)
settings-backup-gfs-daily = Diariamente (última semana)
settings-backup-gfs-weekly = Semanal (último mês)
settings-backup-gfs-monthly = Mensal
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
new-work-convert-later = Você pode converter entre formatos mais tarde.
new-work-location = Local
new-work-will-create = Irá criar
new-work-language = Língua padrão
new-work-language-hint = Aplicado a novos textos. Cada texto pode ser mudado para outro idioma.
new-work-template = Modelo
new-work-template-none = Nenhum
new-work-template-empty-novel = Romance vazio
new-work-template-light-novel = Romance de luz
new-work-template-novel = Romance
new-work-template-novel-in-parts = Romance em partes
new-work-template-notebook = Caderno
new-work-cancel = Cancelar
new-work-create = Criar um Projecto
new-work-step-details = Detalhes
new-work-step-language = Estrutura linguística
new-work-step-template = Modelo
new-work-back = Anterior
new-work-next = Seguinte
new-work-documents-title = Novo Projeto a partir de documentos
new-work-step-import = Importar
new-work-create-and-import = Criar importação...
new-work-documents-next-title = Os seus documentos vêm a seguir.
new-work-documents-next-body = Criando o projeto abre o assistente de importação sobre ele, onde você escolhe os documentos, rever a estrutura Skribisto lê a partir deles, e escolher onde ele pousa. Nada é escrito no projeto até que você confirmá-lo lá.
new-work-documents-no-template = Este projeto começa vazio de propósito: nenhum modelo, então os documentos importados são a única coisa nele.
new-work-documents-chapter-scene-hint = Aplica- se aos capítulos que a importação cria. Poderá alterá- la mais tarde nas definições do projecto.
new-work-single-file-desc = Um ficheiro .skrib (zip). Portátil, fácil de fazer backup.
new-work-bundle-desc = Uma pasta com todos os ativos de texto. Mais amigável para o controle de versão.
new-work-template-none-count = aglutinante vazio
new-work-template-empty-novel-count = 1 capítulo
new-work-template-light-novel-count = 15 capítulos
new-work-template-novel-count = 20 capítulos
new-work-template-novel-in-parts-count = 3 partes, 24 capítulos
new-work-template-notebook-count = Notas de forma livre
new-work-chapter-scene = Capítulos simples
new-work-chapter-scene-tip = Cada capítulo é um único item em que você escreve diretamente, sem cenas embaixo dele. Deixe isso fora para o layout clássico, onde um capítulo é uma pasta: você escreve diretamente para isso também, mas ele também pode conter cenas.
new-work-chapter-scene-tip-more = Você escreve em um capítulo de qualquer forma. A única diferença é se ele pode *conter * cenas. Skribisto árvore de aglutinante é organizacional apenas, para que ambos os layouts compilar para o mesmo livro, você pode misturá-los livremente, e Promove converte um capítulo entre os dois sem perder uma palavra.
new-work-name-required = Digite um nome para o trabalho
new-work-name-invalid = Este nome não tem caracteres utilizáveis
new-work-location-required = Escolha uma localização
new-work-location-missing = Esta pasta não existe
new-work-location-not-folder = Este caminho não é uma pasta
new-work-location-readonly = Esta pasta não pode ser escrita
new-work-target-exists = Um projecto com este nome já está nesta pasta. A criação do trabalho irá perguntar antes de o substituir
new-work-target-is-folder = Uma pasta com este nome já está neste local. Escolha outro nome
new-work-overwrite-title = Substituir o projeto existente?
new-work-overwrite-text = “{ $name }” já existe nesta pasta. Substituí-la pelo novo projeto?
new-work-target-open-title = Este projeto está aberto
new-work-target-open-text = “{ $name }” está aberto em Skribisto. Criar um novo projeto em seu lugar substituiria o projeto em que você está trabalhando, e seu próximo salvamento escreveria esse projeto de volta sobre o novo. Escolha outro nome ou pasta, ou feche o projeto primeiro.
target-importing-title = Uma importação está a escrever este projecto
target-importing-text = Uma importação ainda está escrevendo “{ $name }”. Espere até que tenha terminado, ou escolha outro nome.
import-plume-busy-title = Outro projecto de Plume Creator está a ser importado
import-plume-busy-text = Plume Creator projetos são importados um de cada vez. Espere até que a importação em andamento tenha terminado, ou cancelá-lo, em seguida, começar este novamente.
import-manuskript-busy-title = Outro projecto de Manuskript está a ser importado
import-manuskript-busy-text = Manuskript projetos são importados um de cada vez. Espere até que a importação em andamento tenha terminado, ou cancelá-lo, em seguida, começar este novamente.
backup-restore-importing-text = Uma importação ainda está escrevendo “{ $name }”, o projeto este backup seria restaurado sobre. Restaurar o backup uma vez que a importação tenha terminado.

new-work-manuscript = Manuscrito
new-work-notes = Notas
new-work-research = Pesquisa
new-work-notebook = Caderno
new-work-scene = Cena
new-work-note = Nota
new-work-front-matter = Matéria frontal
new-work-back-matter = Matéria de fundo
new-work-paratext = Estrutura do livro
new-work-paratext-none = Sem estrutura
new-work-paratext-hint = A matéria frontal e traseira uma tradição abre e fecha um livro com. Você pode mover, renomear ou excluir qualquer um deles depois.
new-work-tags = Etiquetas
new-work-tags-none = Sem etiquetas
new-work-tags-hint = Uma paleta inicial para marcar caracteres, lugares e o resto. Opcional, e cada tag pode ser renomeada, recolorada ou apagada depois.
new-work-note-templates = Modelos de notas
new-work-note-templates-none = Sem modelos
new-work-note-templates-hint = A forma em que uma nota de história-bible começa. Opcional, e cada modelo pode ser editado ou excluído depois.
new-work-characters = Caracteres
new-work-places = Locais

import-plume-title = Importação Plume Creator projecto
import-plume-close = Fechar
import-plume-source = Projeto Plume
import-plume-source-hint = Escolha um ficheiro .plume ou .plume_backup (qualquer versão Plume Creator).
import-plume-location = Pasta de destino
import-plume-name = Nome do ficheiro
import-plume-name-placeholder = Nome do projeto
import-plume-will-create = Irá criar
import-plume-trash-warning = Os elementos no lixo ou eliminados não são importados.

import-plume-cancel = Cancelar
import-plume-import = Importar
import-plume-source-required = Escolha um ficheiro de projeto Plume
import-plume-source-missing = Este ficheiro não existe
import-plume-source-not-file = Este caminho não é um ficheiro
import-plume-location-required = Escolha uma pasta de destino
import-plume-location-missing = Esta pasta não existe
import-plume-location-not-folder = Este caminho não é uma pasta
import-plume-location-readonly = Esta pasta não pode ser escrita
import-plume-name-required = Digite um nome de ficheiro
import-plume-name-exists = Já existe aqui um ficheiro com este nome. A importação irá confirmar a substituição
import-plume-overwrite-title = Substituir o ficheiro existente?
import-plume-overwrite-text = “{ $name }” já existe. Substituí-lo pelo projeto importado?
import-plume-manuscript-binder = Manuscrito
import-plume-story-bible-binder = Bíblia da História
import-plume-progress-title = Importando projeto Plume...
import-plume-cancel-import = Cancelar
import-plume-cancelled = Importação cancelada
import-plume-done = Importado { $imported ->
    [one] { $imported } ponto.
   *[other] { $imported } itens.
} { $skipped ->
    [one] { $skipped } itens destruídos não foram migrados.
   *[other] { $skipped } itens destruídos não foram migrados.
}
import-plume-open-now = Abre agora.
import-plume-warnings = { $count ->
    [one] 1 coisa não poderia ser importada exatamente
   *[other] { $count } coisas não podem ser importadas exatamente
}
import-plume-details = Detalhes
import-plume-warnings-title = Advertências de importação
import-plume-error-title = Não foi possível importar o projeto
import-plume-nested-too-deep = O projeto não foi importado: sua parte “{ $part }” está aninhada a mais de { $limit } níveis de profundidade. No Plume Creator projeto chega perto disso, e lendo que poderia cair Skribisto. Nada foi escrito.
import-plume-declares-entities = O projeto não foi importado: sua parte “{ $part }” declara entidades XML, que no Plume Creator projeto faz. Expandi-los poderia levar mais memória do que o computador tem e falhar Skribisto. Nada foi escrito.
import-plume-error-details = Detalhes

import-manuskript-title = Importação Manuskript projecto
import-manuskript-close = Fechar
import-manuskript-source = Manuskript projecto
import-manuskript-source-hint = Escolha o ficheiro .msk ou a pasta do projeto (qualquer versão Manuskript).
import-manuskript-source-file = Escolher ficheiro...
import-manuskript-source-folder = Escolher pasta...
import-manuskript-location = Pasta de destino
import-manuskript-name = Nome do ficheiro
import-manuskript-name-placeholder = Nome do projeto
import-manuskript-will-create = Irá criar
import-manuskript-cancel = Cancelar
import-manuskript-import = Importar
import-manuskript-source-required = Escolha um projeto Manuskript
import-manuskript-source-missing = Este ficheiro ou pasta não existe
import-manuskript-location-required = Escolha uma pasta de destino
import-manuskript-location-missing = Esta pasta não existe
import-manuskript-location-not-folder = Este caminho não é uma pasta
import-manuskript-location-readonly = Esta pasta não pode ser escrita
import-manuskript-name-required = Digite um nome de ficheiro
import-manuskript-name-exists = Já existe aqui um ficheiro com este nome. A importação irá confirmar a substituição
import-manuskript-overwrite-title = Substituir o ficheiro existente?
import-manuskript-overwrite-text = “{ $name }” já existe. Substituí-lo pelo projeto importado?
import-manuskript-manuscript-binder = Manuscrito
import-manuskript-story-bible-binder = Bíblia narrativa
import-manuskript-characters-group = Caracteres
import-manuskript-world-group = Mundo
import-manuskript-plots-group = Gráficos
import-manuskript-project-info-note = Informação do projecto
import-manuskript-summary-note = Resumo
import-manuskript-importance-minor = Menor
import-manuskript-importance-secondary = Secundário
import-manuskript-importance-main = Principal
import-manuskript-progress-title = Importando o projeto Manuskript...
import-manuskript-cancel-import = Cancelar
import-manuskript-cancelled = Importação cancelada
import-manuskript-done = { $imported ->
    [one] { $imported } produto importado.
   *[other] { $imported } produtos importados.
} { $revisions ->
    [0] { "" }
    [one] { $revisions } versão anterior veio com ele.
   *[other] { $revisions } versões anteriores vieram com eles.
}
import-manuskript-open-now = Abre agora.
import-manuskript-warnings = { $count ->
    [one] 1 coisa para saber sobre esta importação
   *[other] { $count } coisas para saber sobre esta importação
}
import-manuskript-details = Detalhes
import-manuskript-warnings-title = Sobre esta importação
import-manuskript-error-title = Não foi possível importar o projeto
import-manuskript-nested-too-deep = O projeto não foi importado: seu ficheiro “{ $part }” está aninhado mais de { $limit } níveis de profundidade. No Manuskript projeto chega perto disso, e lendo-o poderia cair Skribisto. Nada foi escrito.
import-manuskript-folders-too-deep = O projeto não foi importado: um de seus arquivos fica mais do que { $limit } pastas profundas. No Manuskript projeto chega perto disso, e lendo-o poderia cair Skribisto. Nada foi escrito.
import-manuskript-declares-entities = O projeto não foi importado: seu ficheiro “{ $part }” declara entidades XML, que no Manuskript projeto faz. Expandi-los poderia levar mais memória do que o computador tem e falhar Skribisto. Nada foi escrito.
import-manuskript-error-details = Detalhes

import-target-open-title = Este projeto está aberto
import-target-open-text = “{ $name }” está aberto em Skribisto. Importar sobre ele substituiria o projeto em que você está trabalhando, e sua próxima gravação escreveria esse projeto de volta sobre a importação. Escolha outro nome de ficheiro, ou feche o projeto primeiro.
import-warnings-announcement = { $result } { $notice }.

export-title = Exportar
export-close = Fechar
export-scope-label = O quê?
export-format-label = Formato
export-style-label = Estilo
export-path-label = Ficheiro
export-browse = Procurar…
export-show-non-exportable = Mostrar não exportável
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
export-chip-chapters-title = Capítulos: apenas título
export-chip-chapters-both = Capítulos: número + título
export-chip-scene-break-glyph = Quebra de cena: { $glyph }
export-chip-scene-break-blank = Quebra de cena: linha em branco
export-chip-scene-break-none = Quebra de cena: nenhuma
export-chip-major-break-glyph = Grande pausa { $glyph }
export-chip-major-break-blank = Quebra maior: linha em branco
export-chip-major-break-none = Grande pausa: nenhuma
export-chip-major-break-same = Ambos os níveis de quebra
export-chip-spacing-single = Espaçamento: simples
export-chip-spacing-onehalf = Espaçamento: 11⁄2
export-chip-spacing-double = Espaçamento: duplo
export-chip-notes-included = Notas incluídas
export-chip-notes-excluded = Notas excluídas
export-format-docx = Palavra
export-format-odt = LibreOffice
export-format-html = HTML
export-format-markdown = Markdown
export-format-djot = Djot
export-format-text = Texto
export-format-latex = LaTeX
export-format-epub = EPUB
export-format-pdf = PDF
export-overwrite-title = Substituir o ficheiro existente?
export-overwrite-text = “{ $name }” já existe. Substituí-lo?
export-progress-title = Exportando...
export-cancelled = Exportar cancelada
export-done = Exportado { $count ->
    [one] { $count } ponto
   *[other] { $count } pontos
}
export-open-file = Abrir
export-show-in-folder = Mostrar na pasta
export-error-title = Não foi possível exportar
export-error-details = Detalhes

backup-progress-start = A iniciar…
backup-progress-retention = Limpando os backups antigos...
backup-progress-done = Concluído
backup-progress-destination = Destino { $i } de { $n }
backup-details = Detalhes
backup-issues-title = Problemas de backup
backup-failed-title = A cópia de segurança falhou
backup-complete-prune-warning = Cópia de segurança completa ({ $ok } guardado, { $skipped } já atual). Alguns backups antigos não puderam ser removidos

search = Pesquisar
search-query-placeholder = Pesquisar…
search-replace-placeholder = Substituir por...
search-replace-toggle = Alternar substituição
search-replace-all = Substituir tudo
search-opt-case = Coincidir com a caixa
search-opt-whole-word = Palavra inteira
search-opt-diacritics = Coincidir com acentos
search-scope-body = Texto
search-scope-title = Título
search-scope-synopsis = Sinopse
search-scope-label = Legenda
search-facet-book = Livros
search-facet-part = Partes
search-facet-chapter = Capítulos
search-facet-scene = Cenas
search-facet-note = Notas
search-facet-folder = Pastas
search-tip-case = Caso de correspondência: tratar maiúsculas e minúsculas como diferentes, então “Elena” e “elena” são correspondências separadas.
search-tip-whole-word = Palavra inteira: só combinam palavras completas, então “gato” não é encontrado dentro da “categoria”.
search-tip-diacritics = Acentos de correspondência: tratar letras acentuadas como distintas, por isso “cafe” não corresponde “café”.
search-tip-body = Corpo: procure a prosa de cenas e notas.
search-tip-title = Título: pesquise os títulos dos itens do fichário.
search-tip-synopsis = Sinopse: procure o resumo de cada item de escrita.
search-tip-label = Rótulo: pesquise a nota curta mostrada sob o título de um item de aglutinante.
search-tip-comment = Comentários: pesquise o texto dos threads de comentários e suas respostas. Um comentário é sobre o manuscrito em vez de parte dele, então ele tem seu próprio interruptor, e um comentário de folhas de substituição coincide até que você os marque.
search-tip-book = Livros: o contêiner do livro e seus marcadores de início / fim.
search-tip-part = Partes: divisórias de nível parcial.
search-tip-chapter = Capítulos: capítulos, de qualquer forma que o projeto armazená-los.
search-tip-scene = Cenas: os itens que mantêm a sua prosa.
search-tip-note = Notas: notas de forma livre.
search-tip-folder = Pastas: pastas de organização simples e separadores.
search-tip-paratext = Um texto que pertence ao livro, mas não à sua história: um prefácio, uma dedicação, uma palavra após a palavra. Nunca contada no manuscrito.
search-tip-preserve-case = Preservar caso: um substituto toma o caso que encontrou, então “ELENA” se torna “MARTA” e “Elena” torna-se “Marta”.
search-tip-replace = Substituir: mostrar o campo de substituição e Substituir Tudo.
search-error = A pesquisa falhou: { $message }
search-no-matches = Sem correspondências
search-count =
    { $matches ->
        [one] { $matches } jogo
       *[other] { $matches } jogos
    } em { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }
search-count-truncated =
    { $matches ->
        [one] { $matches } jogo
       *[other] { $matches } jogos
    } em { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    } (apenas os primeiros resultados)
search-occurrences = { $count }
search-field-body = Texto
search-field-title = Título
search-field-synopsis = Sinopse
search-field-label = Legenda
search-field-epigraph = Epígrafe
search-field-comment = Comentário
search-field-comment-reply = Responder
search-field-footnote = Nota de rodapé
search-include-in-replace = Incluir em Substituir Tudo
search-collapse-all = Recolher todos os resultados
search-replace-here = Substituir isto
search-dismiss = É negado provimento aos resultados.
search-undo-dismiss = Trazer de volta o último resultado rejeitado
search-replace-nothing = (nada)
search-replace-confirm-title = Substituir todas as correspondências?
search-replace-confirm-text =
    Substituir { $occurrences ->
        [one] { $occurrences } ocorrência
       *[other] { $occurrences } ocorrências
    } de "{ $query }" com "{ $replacement }" transversal { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }? Você pode desfazer isso da notificação depois.
search-replace-done-title = Substituir completado
search-replace-done =
    { $occurrences ->
        [one] { $occurrences } ocorrência
       *[other] { $occurrences } ocorrências
    } substituído em { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }.
search-replace-done-skipped =
    { $occurrences ->
        [one] { $occurrences } ocorrência
       *[other] { $occurrences } ocorrências
    } substituído em { $items ->
        [one] { $items } documento
       *[other] { $items } documentos
    }. { $skipped ->
        [one] { $skipped } campo
       *[other] { $skipped } campos
    } pulou (alterado desde a busca).
search-replace-undo = Anular
search-replace-failed-title = A substituição falhou
search-replace-skipped-title = Nada substituído
search-replace-skipped-body =
    { $fields ->
        [one] Este texto
       *[other] { $fields } destes textos
    } mudou desde a busca, então { $fields ->
        [one] Foi, sim.
       *[other] Eles eram...
    } deixado sozinho. Procure novamente para ver onde as palavras estão agora.
search-replace-undo-failed-title = Desfazer falhou
search-preview = Antevisão
search-preview-empty = Selecione um resultado para visualizá- lo aqui
search-preview-no-prose = Esta correspondência não tem texto editável
search-preview-prompt = Para ver uma antevisão aqui, execute uma pesquisa.
search-preview-open-search = Pesquisar no projeto
search-preview-footnote-prompt = Esta correspondência está no texto de uma nota de rodapé. Abra- a no dock de Notas para vê- la e editá- la.
search-preview-open-footnotes = Abrir as Notas de rodapé

find-placeholder = Procurar neste documento...
find-count = { $current } de { $total }
find-no-results = Sem resultados
find-close = Fechar a procura
find-previous = Jogo anterior (Shift+Enter)
find-next = Próxima ocorrência (Entrar)
find-opt-case = Coincidir com a caixa
find-opt-whole-word = Palavra inteira
find-replace-toggle = Mostrar ou ocultar substituição (Ctrl+R)
find-replace-placeholder = Substituir por...
find-replace = Substituir
find-replace-all = Substituir tudo
find-preserve-case = Preservar o processo

dict-download-title = Baixando { $name }...
dict-download-done = Instalado { $name }
dict-download-failed = Não foi possível baixar { $name }: { $error }
dict-removed = Removido { $name }
dict-accept-first = Aceite a licença antes de baixar { $name }
settings-dict-tab-installed = Instalado
settings-dict-tab-get-more = Obter mais
settings-dict-tab-personal = Palavras pessoais
dict-installed-empty = Ainda não foram encontrados dicionários neste computador.
dict-get-more-search = Procurar idiomas
dict-system-badge = no seu sistema
dict-unusable-badge = inutilizável
dict-download-button = Baixar
dict-downloading = Baixando...
dict-installed-label = Instalado
dict-remove = Remover
dict-view-license = Ver licença
dict-approx-size = ~{ $size }
dict-personal-empty = Ainda não há palavras pessoais neste projecto.
dict-personal-add = Adicionar
dict-personal-placeholder = Adicione uma palavra...
dict-missing-toast = { $count ->
    [one] Este projeto usa { $count } dicionário que você não tem instalado
   *[other] Este projeto usa { $count } dicionários que você não instalou
}
dict-missing-action = Obter dicionários
dict-license-title = { $name } licença
dict-license-accept = Aceitar download
dict-license-cancel = Cancelar
dict-license-close = Fechar
dict-add-button = Adicionar um dicionário...
dict-add-title = Adicionar um dicionário
dict-add-close = Fechar
dict-add-name = Nome
dict-add-name-placeholder = Por exemplo, o meu dicionário em Latim
dict-add-code = Código da língua
dict-add-code-placeholder = por exemplo, la ou fr-FR-x-custom
dict-add-code-hint = Uma pequena etiqueta de sua escolha. Isto é o que você escolhe como idioma de um documento.
dict-add-aff = Ficheiro de afixos (.aff)
dict-add-dic = Lista de palavras (.dic)
dict-add-submit = Adicionar
dict-add-cancel = Cancelar
dict-add-name-required = Dê um nome ao dicionário.
dict-add-code-required = Digite um código de idioma.
dict-add-code-invalid = Use apenas letras, dígitos e - _ .
dict-add-code-reserved = Isso é um código de dicionário incorporado. Escolha outro.
dict-add-file-required = Escolha um ficheiro.
dict-add-file-missing = Esse ficheiro não existe.
dict-add-code-taken = Um dicionário para esse código já está instalado.
dict-add-done = Adicionado { $name }
dict-add-unusable = Estes arquivos não são um dicionário utilizável: { $error }
dict-add-failed = Não foi possível adicionar o dicionário: { $error }
inspector-export = Exportar
inspector-exportable = Incluir nas exportações
ctx-number = &Número deste capítulo
ctx-unnumber = &Não numerar este Capítulo
inspector-numbering = Numeração
inspector-numbered = Numerado
inspector-numbered-tip = Este capítulo ocupa o seu lugar na numeração do livro. Desligue-o para um prólogo, um epílogo ou um interlúdio.
inspector-numbered-tip-more = Um capítulo não numerado permanece no livro exatamente como era: seu título, sua prosa e sua contagem de palavras não são tocados. Simplesmente não imprime nenhum número, e não usa um: o capítulo após um prólogo é o capítulo um, não o capítulo dois. Deixando-o fora da exportação, em vez disso, é um interruptor diferente, acima, e que um remove o capítulo do livro completamente.
inspector-apply-to-children = Aplicar a elementos subordinados
inspector-milestone = Data do marco
inspector-milestone-none = Sem data
inspector-milestone-clear = Limpar
inspector-tags = Etiquetas
inspector-aliases = Outros nomes
inspector-dict-language = Idioma
inspector-apply-language-to-children = Aplicar o idioma às elementos subordinados
settings-page-language = Idioma
settings-field-dict-language = Línguas
dict-tradeoff-hint = Cada idioma extra aceita mais palavras, então menos erros são pegos.
lang-inherit-hint = Herdado: esta cena usa as línguas do livro ou projeto.
lang-pill-list = Línguas
lang-pill-add = Adicionar um idioma
lang-pill-remove = Remover { $name }
lang-pill-mute = Desligar a verificação ortográfica para { $name }
lang-pill-unmute = Rodar a verificação ortográfica para { $name }

settings-page-personal-dictionary = Dicionário pessoal
settings-user-dict-desc = Palavras que você adicionar aqui são tratadas como corretamente soletrado em todos os lugares no aplicação, e nunca marcou novamente.
settings-user-dict-add-placeholder = Adicione uma palavra...
settings-user-dict-add = Adicionar palavra
settings-user-dict-search = Filtrar palavras
settings-user-dict-count = { $count ->
    [one] { $count } palavra
   *[other] { $count } palavras
}
settings-user-dict-rename = Mudar o nome
settings-user-dict-remove = Remover
settings-user-dict-empty = Ainda não há palavras. Adicione palavras que o verificador ortográfico deve ignorar ou importar uma lista.
settings-user-dict-import = Importar...
settings-user-dict-export = Exportar...
settings-user-dict-import-tip = Adicione palavras de um ficheiro de texto simples, uma palavra por linha. As palavras existentes são mantidas.
settings-user-dict-export-tip = Salve a lista inteira para um ficheiro de texto simples, uma palavra por linha.
settings-user-dict-txt-filter = Ficheiros de texto
settings-user-dict-duplicate = Já no dicionário
settings-user-dict-imported = Importado { $count ->
    [one] { $count } palavra
   *[other] { $count } palavras
} ({ $duplicates } já presentes).
settings-user-dict-import-failed = Não foi possível ler a lista de palavras: { $error }
settings-user-dict-exported = Gravado { $count ->
    [one] { $count } palavra.
   *[other] { $count } palavras.
}
settings-user-dict-export-failed = Não foi possível guardar a lista de palavras: { $error }

settings-page-text-replacements = Substituições de texto
settings-text-repl-desc = Substituir abreviatura com o texto completo à medida que você digita: "btw" torna-se "a propósito" no momento em que você digita um espaço ou pontuação.
settings-text-repl-enable = Usar substituições de texto neste projeto
settings-text-repl-disabled-hint = Ligue isto para definir a abreviatura que se expande à medida que escreve.
settings-text-repl-add = Adicionar uma regra
settings-text-repl-trigger-placeholder = Abreviatura
settings-text-repl-replacement-placeholder = O que se torna
settings-text-repl-added = Adicionado "{ $trigger }"
settings-text-repl-duplicate = "{ $trigger }" já tem uma regra
settings-text-repl-filter = Regras do filtro
settings-text-repl-count = { $n ->
    [one] 1 regra
   *[other] { $n } regras
}
settings-text-repl-row-enabled = Usar esta regra
settings-text-repl-delete = Suprimir a regra para { $trigger }
settings-text-repl-deleted = Suprimido a regra de "{ $trigger }"
settings-text-repl-empty = Ainda não há regras.
settings-text-repl-csv-filter = CSV ficheiros
settings-text-repl-import = Importar...
settings-text-repl-export = Exportar...
settings-text-repl-imported = Importado { $added }, pulou { $skipped }
settings-text-repl-exported = Exportado { $n ->
    [one] 1 regra
   *[other] { $n } regras
}

editor-menu-no-suggestions = Sem sugestões
editor-menu-add-to-dictionary = Adicionar “{ $word }” ao dicionário
editor-menu-add-words-to-dictionary = Adicionar palavras selecionadas ao dicionário
editor-dict-added = Adicionado “{ $word }” ao seu dicionário.
editor-dict-added-multi = { $count ->
    [one] Adicionado { $count } palavra ao seu dicionário.
   *[other] Adicionado { $count } palavras ao seu dicionário.
}
toast-undo = Anular

undo-superseded-title = Esse passo já não é o último.
undo-superseded-body = Algo mais mudou neste projeto desde então. Desfazer iria retirar isso em vez disso, por isso não mudou. Nada deste passo foi invertido. Use Editar O Desfazer para voltar ao histórico você mesmo.
undo-failed = Falha ao desfazer: { $error }

menu-edit = &Editar
menu-edit-undo = &Anular
menu-edit-redo = &Refazer
menu-edit-undo-target = &Anular { $target }
menu-edit-redo-target = &Refazer { $target }
shortcut-name-edit-undo = Anular
shortcut-name-edit-redo = Refazer
undo-target-typing = digitando
undo-target-project = a última mudança para este projeto
prose-history-reset-title = Repor o histórico de digitação
prose-history-reset-body = { $count ->
    [one] Uma cena aberta foi restaurada de um estado anterior, de modo que seu histórico de digitação não se aplica mais.
   *[other] { $count } cenas abertas foram restauradas de um estado anterior, de modo que sua história de digitação não se aplica mais.
}
undo-target-trash = indo para o lixo
undo-target-restore = restaurando do lixo
undo-target-delete-forever = apagar para sempre
undo-target-replace-all = substituição em todo o projeto
undo-target-import = a importação do documento
undo-target-duplicate = duplicando
undo-target-move = em movimento
undo-target-merge = fundir duas cenas
undo-target-split = dividir uma cena
undo-target-promote = alterando o tipo
undo-target-tidy-titles = arrumando os títulos do capítulo
undo-target-import-tags = etiquetas de importação
undo-target-import-templates = importar modelos de notas
undo-target-create = criação de
undo-target-remove = apagar
undo-target-rename = mudança de nome
undo-target-edit = que editar
menu-edit-find = &Localizar…
menu-edit-find-next = Procurar a Pr&óxima
menu-edit-find-prev = Proc&urar o anterior
menu-edit-replace = Procurar &e substituir...
menu-edit-find-in-project = Procurar &no projecto...
menu-edit-replace-in-project = Su&bstituir no projecto...
undo-frozen = Desfazer (“Sempre para a frente” está ligado)
redo-frozen = Refazer (“Sempre para a frente” está ligado)
menu-edit-cut = &Cortar
menu-edit-copy = Co&piar
menu-edit-paste = C&olar
menu-edit-paste-plain = Colar se&m formatação
menu-edit-select-all = &Selecionar todos
shortcut-name-edit-cut = Cortar
shortcut-name-edit-copy = Copiar
shortcut-name-edit-paste = Colar
shortcut-name-edit-paste-plain = Colar sem formatação
shortcut-name-edit-select-all = Selecionar todos

titlebar-spellcheck-on = A verificação ortográfica está ligada. Carregue para parar de verificar (F7)
titlebar-spellcheck-off = A verificação ortográfica está desligada. Carregue para verificar novamente (F7)
menu-spellcheck = &Verificar ortografia
menu-comments = &Comentários
settings-page-spellcheck = Verificação ortográfica
settings-group-spellcheck = Verificação ortográfica
settings-spellcheck-enabled = Verificar a ortografia enquanto escrevo
settings-spellcheck-hint = Sublinha as palavras que nenhum dicionário instalado sabe. Desligar isto pára todas as verificações, em cada projecto, até que o ligue de novo. Para parar de verificar apenas uma língua, limpe a sua marca de verificação no campo Linguagem do trabalho ou de um item.

menu-trash = &Lixo
trash-title = Lixo
trash-empty-state = O lixo está vazio.
trash-empty-button = Esvaziar o lixo…
trash-restore = &Restaurar
trash-restore-to = &Restaurar para...
trash-delete-forever = &Apagar Para Sempre
trash-restored-ok = { $count } restaurado.
trash-restore-error = Não foi possível restaurar: { $error }
trash-restore-orphaned = O local original deste item desapareceu. Escolha onde restaurá-lo.
trash-restore-no-project = Nenhum projeto está aberto, então não há nada para restaurar.
trash-delete-no-project = Nenhum projeto está aberto, então não há nada para excluir.
trash-empty-confirm-title = Esvaziar o lixo?
trash-empty-confirm-text = { $count ->
    [one] { $count } entrada lixo será permanentemente excluído.
   *[other] { $count } entradas destruídas serão excluídas permanentemente.
} Eles deixam o lixo para sempre. Desfazer ainda pode trazê-los de volta até que você faça outra coisa.
trash-emptied-title = Lixo esvaziado
trash-emptied-body = Tudo no lixo foi apagado permanentemente.
trash-empty-no-project = Nenhum projeto está aberto, então não há lixo para esvaziar.
trash-delete-forever-confirm-title = Apagar para sempre?
trash-delete-forever-confirm-text = { $count ->
    [one] { $count } item será permanentemente excluído.
   *[other] { $count } itens serão excluídos permanentemente.
} Eles deixam o lixo para sempre. Desfazer ainda pode trazê-los de volta até que você faça outra coisa.
trash-deleted-title = Apagado para sempre
trash-deleted-body = { $count } suprimidos permanentemente.
trash-undo = Anular
trash-restore-picker-title = Restaurar para...
trash-restore-to-confirm-title = Restaurar aqui?
trash-restore-to-confirm-text = Restaurar "{ $item }" em "{ $destination }"?
trash-restore-picker-restore-here = Restaurar Aqui
trash-restore-picker-cancel = Cancelar
trash-restore-picker-empty = Sem fichários ainda. Crie um primeiro.
trash-banner-title = Este artigo está no Lixo
trash-banner-description = Ele não aparecerá no esboço ou exportações até que você restaurá-lo.
trash-banner-restore = Restaurar…
trash-tab-tooltip = No Lixo

settings-styles-sheet-yes = Sim.
settings-styles-sheet-no = Não
settings-styles-sheet-font = Fonte
settings-styles-sheet-indent = Indentação de primeira linha
settings-styles-sheet-para-spacing = Espaçamento de parágrafos
settings-styles-sheet-page = Tamanho da página
settings-styles-sheet-margins = Margens (sup./dir./inf./esq.)
settings-styles-sheet-title-page = Página de título do livro
settings-styles-sheet-heading-language = Língua do cabeçalho
settings-styles-sheet-auto = Segue o texto
settings-styles-sheet-digits = Números
settings-styles-sheet-direction = Direcção do texto
settings-styles-sheet-formats = Formatos
settings-styles-sheet-all-formats = Tudo

settings-styles-editor-group = Editor de estilo
settings-styles-editor-missing = Esse estilo já não está disponível.

settings-styles-page-letter = Carta
settings-styles-digits-western = Oeste (0–9)
settings-styles-digits-eastern-arabic = Árabe oriental ('')
settings-styles-direction-ltr = Esquerda para direita
settings-styles-direction-rtl = Direita para esquerda

format-dock-title = Formato
format-panel-empty = Coloque seu cursor em uma cena, nota ou sinopse para ver as opções de formatação.
format-group-marks = Texto
format-group-block = Parágrafo
format-group-lists = Listas
format-group-tables = Quadro
format-group-breaks = Quebras de cena
format-superscript = Superior à linha
format-subscript = Inferior à linha
format-link = Ligação…
link-dialog-insert-title = Inserir um link
link-dialog-edit-title = Editar o link
link-dialog-text-label = Texto
link-dialog-text-placeholder = as palavras que o leitor vê
link-dialog-url-label = Ligações para
link-dialog-url-placeholder = exemplo.com
link-dialog-insert = Inserir
link-dialog-apply = Aplicar
link-dialog-cancel = Cancelar
link-dialog-remove = Remover link
link-scheme-refused = Apenas web e e-mail links podem ser abertos. { $url } foi deixado sozinho.
menu-format-link = &Ligação…
format-clear = Limpar a formatação
format-heading = Nível de orientação
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
format-table-col-before = Inserir uma coluna antes
format-table-col-after = Inserir coluna após
format-table-row-delete = Apagar linha
format-table-col-delete = Apagar a coluna
format-table-remove = Remover tabela

menu-format-marks-bold = &Negrito
menu-format-marks-italic = It&álico
menu-format-marks-underline = &Sublinhado
menu-format-marks-strike = &Rasurado
menu-format-marks-superscript = S&uperior à linha
menu-format-marks-subscript = In&ferior à linha
menu-format-marks-clear = Lim&par a Formatação
menu-format-heading = &Título
menu-format-heading-normal = T&exto Normal
menu-format-heading-1 = &Título 1
menu-format-heading-2 = T&ítulo 2
menu-format-heading-3 = Tít&ulo 3
menu-format-heading-4 = Títu&lo 4
menu-format-heading-5 = Títul&o 5
menu-format-heading-6 = Título &6
menu-format-alignment = &Alinhamento
menu-format-align-left = &Alinhar à Esquerda
menu-format-align-center = &Centrar
menu-format-direction = Direc&ção
menu-format-direction-auto = &Automático
menu-format-direction-ltr = &Esquerda para Direita
menu-format-direction-rtl = &Direita para Esquerda
menu-format-blockquote = &Citação em bloco
menu-format-lists = L&istas
menu-format-list-bullet = &Lista com marcas
menu-format-list-numbered = L&ista Numerada
menu-format-indent = &Aumentar avanço
menu-format-outdent = &Diminuir avanço
menu-format-table = &Quadro
menu-format-table-insert = &Inserir Tabela
menu-format-table-2x2 = &2 x 2
menu-format-table-3x3 = &3 x 3
menu-format-table-4x4 = &4 x 4
menu-format-table-row-above = I&nserir Linha Acima
menu-format-table-row-below = In&serir Linha Abaixo
menu-format-table-col-before = Inse&rir uma Coluna Antes
menu-format-table-col-after = Ins&erir Coluna Depois
menu-format-table-row-delete = &Apagar Linha
menu-format-table-col-delete = A&pagar a Coluna
menu-format-table-remove = Re&mover tabela

menu-format-typo-marks = &Marca de Formatação
menu-format-typo-nbsp = &Espaço sem quebras
menu-format-typo-nbhyphen = &Hífen sem quebra
menu-format-typo-soft-hyphen = &Hífen opcional
menu-format-typo-nnbsp = &Espaço Estreito Sem Quebra
menu-format-typo-zwsp = &Espaço de Largura Zero
menu-format-typo-word-joiner = &Unificador de palavras


menu-about = &Cerca de Skribisto...
about-title = Cerca de Skribisto
about-version = Versão { $version }
about-update-available = Versão { $version } está disponível
about-update-see-notes = Ver o que mudou
about-update-flathub = O Flathub mantém esta cópia actualizada.
about-update-distro = A sua distribuição mantém esta cópia actualizada.
about-tagline = Um aplicação de escrita de romance para ficção de longa duração, escrito em Rust com o kit de ferramentas Teksilo.
about-license = Lançado sob a Licença Pública Geral GNU, versão 3.
about-copyright = © 2026 Cyril Jacquet
about-close = Fechar

native-menu-about = Cerca de { $app }
native-menu-hide = Esconder { $app }
native-menu-quit = Sair de { $app }
native-menu-settings = Definições...
native-menu-window = Janela
native-menu-minimize = Minimizar
native-menu-zoom = Ampliação

window-title = { $title } - { $app }
window-title-numbered = { $title } - { $app } (Janela { $n })
window-title-empty = { $app }

first-run-window-title = Boas-vindas
first-run-title = Configurar { $app }
first-run-body = { $app } mantém suas definições separadas de Skribisto, então ele começa em branco. Suas preferências, projetos recentes, dicionários e layout de janela podem ser copiados agora.
first-run-from = Copiar de
first-run-to = Copiar para
first-run-copy-note = Nada é movido ou removido. Skribisto mantém cada um de suas próprias definições, e continua trabalhando exatamente como antes.
first-run-import = Configuração da importação
first-run-start-fresh = Começar de novo
first-run-import-failed = Algumas definições não puderam ser importadas: { $error }

settings-page-punctuation = Padrões de pontuação
settings-group-punctuation = Pontuação inteligente
settings-page-work-punctuation = Pontuação
settings-punctuation-override = Dê a este projeto suas próprias regras de pontuação
settings-punctuation-override-hint = Off: o projeto segue a preferência do aplicação. Essas regras viajam dentro do .skrib, então um co-autor abrindo o ficheiro escreve com a mesma tipografia.
settings-punctuation-dashes = Transforme -- em um traço, --- em um traço
settings-punctuation-ellipsis = Transforme ... em uma elipse
settings-punctuation-quotes = Aspas e apóstrofos curvos
settings-quote-style = Cotação
settings-quote-style-locale = Predefinição da língua
settings-quote-style-curly = “Duplo”
settings-quote-style-curly-single = «Solteiro»
settings-quote-style-guillemets = «Guillemets»
settings-quote-style-low-high = „Baixo-alto“
settings-punctuation-spacing = Espaço antes ; : ?
settings-punctuation-spacing-hint = A tipografia francesa estabelece um espaço estreito sem quebra antes ; ! ? e um espaço completo antes :. Aplica-se apenas ao texto escrito em francês. O espaço dentro do guillemets « » vem com as próprias citações guillemet.
settings-punctuation-sample = A sua língua dá
settings-punctuation-app-hint = O que cada projeto faz, a menos que tenha regras próprias no Projeto ▸ Pontuação.
settings-punctuation-dialogue = Abrir um parágrafo digitado como "-" com um traço de diálogo
settings-punctuation-dialogue-hint = Para línguas que marcam o discurso com um traço em vez de aspas: francês, espanhol, russo e outros. Só dispara no início de um parágrafo.

statusbar-go-to = Vai para...
go-to-placeholder = Procurar o ficheiro
go-to-no-matches = Nenhum item corresponde a essa pesquisa.
menu-go-to = &Vai para...

settings-page-distraction-free-themes = Temas livres de distração
settings-themes-builtin = Temas incorporados
settings-themes-builtin-badge = Incorporado
settings-themes-user = Os meus temas
settings-themes-editor-group = Editar tema
settings-themes-editor-empty = Escolha um tema em "Meus temas" para editá-lo.
settings-themes-use = Utilização
settings-themes-duplicate = Duplicar
settings-themes-edit = Editar
settings-themes-delete = Eliminar
settings-themes-export = Exportar...
settings-themes-import = Importar...
settings-themes-copy-suffix = cópia
settings-themes-json-filter = Tema (JSON)
settings-themes-imported = Tema importado
settings-themes-import-failed = Não foi possível importar esse tema
settings-themes-exported = Tema exportado
settings-themes-export-failed = Não foi possível exportar esse tema
settings-themes-low-contrast = contraste baixo
settings-themes-low-contrast-band = realce esconde o texto
settings-themes-field-name = Nome
settings-themes-field-paper = Página
settings-themes-field-ink = Texto
settings-themes-field-general = Contexto
settings-themes-field-widget-text = Controlar o texto da tira
settings-themes-field-caret-band = Realce em torno do cuidador

statusbar-focus-settings = Definições livres de distração
statusbar-focus-manage-themes = Gerir temas...

comments-title = Comentários
comments-document-title = Este documento
comments-empty-project = Ainda não há comentários neste projecto.
comments-empty-document = Não há comentários sobre este documento.
comments-filter-all = Tudo
comments-filter-open = Abrir
comments-filter-resolved = Resolvido
comments-filter-orphaned = Sem texto associado
comments-status-open = Abrir
comments-status-resolved = Resolvido
comments-status-orphaned = Perdeu o texto associado
comments-orphan-snippet = (o texto comentado se foi)
comments-status-unplaced = Sem posição de texto
comments-unplaced-snippet = (não ancorado a nenhum texto)
comments-reply-count = { $count ->
    [one] { $count } resposta
   *[other] { $count } respostas
}
comments-menu-resolve = Resolver
comments-menu-reopen = Abrir de novo
comments-menu-delete = Eliminar
comments-sort-document = Por ordem do documento
comments-sort-newest = Mais recentes primeiro
comments-sort-reader = Agrupado pelo leitor
comments-unsigned-author = Sem Assinatura
comments-agreement = { $count ->
    [one] { $count } leitores
   *[other] { $count } leitores
}
comments-menu-add = Adicionar um comentário
comments-menu-add-paragraph = Comentário sobre este parágrafo
overview-col-comments = Comentários
overview-col-total-comments = Total das observações
comments-card-placeholder = Escreva um comentário…
comments-card-unknown-author = Autor desconhecido
comments-unsigned-toast = Seus comentários não estão assinados: nenhum nome está definido para você neste computador.
comments-unsigned-action = Definir o seu nome
comments-card-reply = Responder
comments-card-reply-placeholder = Responder…
comments-card-actions = Comentar ações
comments-reply-actions = Responder ações
comments-menu-delete-reply = Apagar a resposta
comments-menu-delete-all = Apagar todos os comentários aqui
comments-deleted-toast = Comentário apagado
comments-reply-deleted-toast = Resposta apagada
comments-deleted-all-toast = { $count ->
    [one] { $count } comentário suprimido
   *[other] { $count } comentários suprimidos
}
comments-undo = Anular

analysis-segment = Análise
analysis-scope-book = Analisando este livro
analysis-run = Análise de execução
analysis-stale = Mudou desde que isto foi executado
analysis-not-run = Ainda não analisado.
analysis-running = Leitura do manuscrito…
analysis-failed = A análise não pôde terminar.
analysis-no-scenes = Ainda não há cenas neste livro.

analysis-shape = Forma

analysis-arrivals = Chegadas
analysis-arrivals-explainer = Como o texto chegou a este projeto enquanto ele foi aberto. Ele diz qual rota os caracteres vieram para baixo, e nada sobre quem os escreveu: um escritor que rascunhou em outro lugar e cole, e um que dita tem ditado. Não há nenhum número para apontar para aqui.
analysis-arrivals-scope = Todo este projecto, não só este livro.
analysis-arrivals-session = Desde que o abriste, fechar o projecto começa de novo a contar.
analysis-arrivals-nothing = Ainda não chegou nenhum texto nesta sessão.
analysis-arrivals-typed = Dactilografia
analysis-arrivals-pasted = Colado
analysis-arrivals-dictated = Ditado
analysis-arrivals-imported = Importado
analysis-arrivals-programmatic = Inserído para você
analysis-arrivals-count = { $count ->
    [one] { $count } caracteres
   *[other] { $count } carateres
}
analysis-arrivals-none = nenhum

analysis-words-per-scene = Palavras por cena
analysis-median-words = A cena mediana deste livro é executada { $count ->
    [one] { $count } palavra.
   *[other] { $count } palavras.
}
analysis-median-line = Mediana: { $count ->
    [one] { $count } palavra
   *[other] { $count } palavras
}
analysis-dialogue = Diálogo
analysis-dialogue-unsupported = O diálogo ainda não foi medido para esta língua.

analysis-footnote-words = Palavras na nota de rodapé
analysis-footnote-words-count = { $count ->
    [one] { $count } palavra está nas notas de rodapé deste livro, mantido à parte do total do manuscrito.
   *[other] { $count } palavras estão nas notas de rodapé deste livro, mantidos à parte do total do manuscrito.
}
analysis-footnote-words-pending = Contando as notas de rodapé…

analysis-ignore-empty = Ignorar os textos sem nada escrito ainda
analysis-empty-hidden = { $count } vazio { $count ->
        [one] texto
       *[other] textos
    } escondidos.
analysis-all-texts-empty = Todos os textos deste livro ainda estão vazios.

binder-filter-count = { $shown } de { $total } mostrado
binder-filter-clear = Limpar
binder-filter-none = Nada corresponde a “{ $query }”.

settings-paratext-intro = A matéria frontal e traseira um novo projeto pode começar com. Cada estrutura pertence a uma tradição de publicação, e seus títulos de página são escritos na própria língua da tradição. Renomeie-os livremente uma vez que um projeto é criado.
settings-paratext-structures = Estruturas
settings-paratext-broken = Não foi possível ler
settings-paratext-edit = Editar
settings-paratext-duplicate = Duplicar
settings-paratext-delete = Eliminar
settings-paratext-new = Nova estrutura
settings-paratext-save = Guardar
settings-paratext-editor-hint = Um nome, as páginas a criar antes do manuscrito, e as páginas a criar depois dele. Onde elas acabam é seu: elas são itens comuns uma vez que o projeto existe.


image-insert = I&nserir imagem…
image-no-project = Abra um projeto antes de inserir uma imagem.
image-choose-title = Escolha uma imagem
image-filter-label = Imagens
image-large-title = Esta é uma imagem grande
image-large-text =
    { $name } é { $megapixels } megapixels ({ $width }×{ $height }).
    Mantendo-o como é armazena seu ficheiro original no projeto: ele viaja
    com cada backup e cada exportação. Optimizando armazena uma cópia menor
    em vez disso, até 2560 pixels em seu lado mais longo.
image-large-keep = Manter original
image-large-downscale = Otimizar
image-large-remember = Faz isto de agora em diante, não voltes a perguntar.
image-not-recorded = A imagem foi salva mas não pôde ser gravada no projeto.
image-describe-title = Descreva a imagem
image-describe-explain = O que a imagem mostra, para um leitor que não pode vê-la. Não faz parte do manuscrito: nunca é contado, pesquisado ou exportado como prosa.
image-describe-placeholder = um farol contra um céu cinzento
image-resize-title = Redimensionar a imagem
image-resize-explain = Uma porcentagem do tamanho que é mostrado em agora. 100 deixa-o como é.
image-resize-invalid = Digite um número entre 1 e 1000.
image-menu-describe = &Descreva a imagem…
image-menu-resize = &Redimensionar a imagem…
image-menu-reset-size = &Tamanho original

cover-choose = &Capa do livro…
cover-clear = Remo&ver a tampa
cover-choose-title = Escolha uma capa
cover-set = A capa está pronta.
cover-cleared = A capa foi removida. A imagem ainda está no projeto.
card-label = Legenda
card-exportable = Exportado
card-numbered = Numerado
card-created = Criado
card-modified = Modificado
card-yes = sim
card-no = no
card-type = Tipo
card-position = Posição
card-children = Crianças
card-goal = Objetivo
card-words = Palavras
card-synopsis = Sinopse
card-point-of-view = Ponto de vista
card-aliases = Outros nomes

export-orphan-footnotes-title = Notas de rodapé sem referência
export-comments-dropped =
    { $count ->
        [one] Um comentário não pôde ser colocado no texto exportado e foi deixado de fora.
       *[other] { $count } comentários não puderam ser colocados no texto exportado e foram deixados de fora.
    }
export-orphan-footnotes =
    { $count ->
        [one] Uma nota de rodapé não é mais referenciada em nenhum lugar do manuscrito. Seu texto não aparecerá no livro exportado.
       *[other] { $count } notas de rodapé não são mais referenciadas em nenhum lugar do manuscrito. Seu texto não aparecerá no livro exportado.
    }


menu-footnotes = &Notas de rodapé
footnotes-title = Notas de rodapé
footnotes-insert = In&serir nota de rodapé
footnotes-empty = Sem notas de rodapé ainda. Coloque o cursor em uma cena, em seguida, use + acima, ou Ctrl+Alt+F.
footnotes-filter-all = Tudo
footnotes-filter-document = Este documento
footnotes-filter-orphaned = Sem texto associado
footnotes-orphaned = Nada mais aponta para esta nota.
footnotes-untitled-home = Sem título
footnotes-body-placeholder = a nota em si
footnotes-insert-tooltip = Inserir uma nota de rodapé ao cursor (Ctrl+Alt+F)
footnotes-actions = Acções da Nota de rodapé
footnotes-delete = &Apagar a nota e a sua referência
footnotes-no-project = Abra um projeto antes de inserir uma nota de rodapé.
footnotes-no-caret = Coloque o cursor no texto de uma cena para inserir uma nota de rodapé.
footnotes-not-created = A nota de rodapé não pôde ser adicionada ao projeto.
footnotes-deleted-toast = Nota apagada
footnotes-undo-delete = Anular

import-document-title = Importar documentos
import-document-close = Fechar
import-document-step-files = Ficheiros
import-document-step-review = Revisão
import-document-step-destination = Destino
import-document-step-reconcile = Unir
import-document-reconcile-hint = Alguns desses itens são aqueles que você já tem. Diga o que deve acontecer com cada um.
import-document-reconcile-all-new = Nada neste ficheiro corresponde ao seu projeto: cada item será adicionado como novo.
import-document-duplicate-returns = Esses arquivos são mais de uma cópia do mesmo manuscrito voltando. Importe-os um de cada vez: combinando apenas uma cópia retornando com seu projeto, então importar vários juntos adicionaria uma segunda cópia do livro em vez de fundi-lo.
import-document-hunk-added = Tome este novo parágrafo.
import-document-hunk-removed = Largar este parágrafo
import-document-hunk-changed = Tome esta reescrita
import-document-col-stray-prose = Seu texto
import-document-stray-as-paratext = Manter como paratexto
import-document-stray-discard = Largar o texto
import-document-col-current = Em seu projeto
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
import-document-drop-title = Soltar os documentos aqui
import-document-drop-hint = Markdown (.md), Palavra (.docx), Documento Aberto (.odt) e texto simples (.txt)
import-document-browse = Procurar…
import-document-move-up = Subir
import-document-move-down = Mover para baixo
import-document-remove-file = Remover
import-document-file-count = { $count ->
    [one] 1 ficheiro
   *[other] { $count } ficheiros
}
import-document-no-files = Nenhum ficheiro escolhido ainda.
import-document-col-included = Importar
import-document-col-title = Título
import-document-col-type = Tipo
import-document-col-words = Palavras
import-document-col-breaks = Quebras
import-document-col-comments = Comentários
import-document-col-footnotes = Notas de rodapé
import-document-col-epigraph = Epígrafe
import-document-col-source = Origem
import-document-level-rules = Níveis das rubricas
import-document-level-n = Rubrica { $level }
import-document-add-top-level = Adicionar nível superior
import-document-add-top-level-tooltip = Inserir um Livro acima de cada item analisado, para arquivos de capítulo que não tenham cabeçalho de livro
import-document-destination = Destino
import-document-destination-hint = Escolha um aglutinante ou item: novos itens pousam dentro de uma pasta, ou depois de uma cena.
import-document-destination-empty = Sem fichários ainda. Crie um primeiro.
import-document-plan-empty = Nada para importar ainda.
import-document-summary = { $rows ->
    [one] 1 ponto
   *[other] { $rows } pontos
} · { $breaks ->
    [one] 1 pausa de cena
   *[other] { $breaks } quebras de cena
}
import-document-back = Anterior
import-document-cancel = Cancelar
import-document-analyse = Seguinte
import-document-analysing = Documentos de leitura…
import-document-step-analysing = Leitura
import-document-cancel-analysis = Parar de ler
import-document-analyse-failed = Os documentos não puderam ser lidos.
import-document-details = Detalhes
import-document-import = Importar
import-document-done = { $count ->
    [one] 1 item importado
   *[other] { $count } produtos importados
}
import-document-undo = Anular
import-diagnostic-file-unreadable = “{ $path }” não pôde ser lido: { $detail }. Os outros arquivos ainda importam.
import-diagnostic-nested-too-deep = “{ $path }” não foi lido: seu conteúdo é aninhado mais de { $limit } níveis de profundidade. Nenhum documento real chega perto disso, e lendo-o poderia bater Skribisto, por isso é deixado de fora. Os outros arquivos ainda importar.
import-diagnostic-archive-too-large = “{ $path }” não foi lido: é um ficheiro compactado construído para desempacotar muito maior do que é, que poderia esgotar a memória e falhar Skribisto, por isso é deixado de fora. Os outros arquivos ainda importar.
import-diagnostic-lossy-decode = { $count ->
    [one] Um caractere em “{ $path }” não descodificado. Salve o ficheiro como UTF-8 para mantê-lo.
   *[other] { $count } caracteres em “{ $path }” não descodificaram. Salve o ficheiro como UTF-8 para mantê-los.
}
import-diagnostic-decoded-from-bom = “{ $path }” foi decodificado como { $detail }, não UTF-8.
import-diagnostic-empty-file = “{ $path }” está vazio.
import-diagnostic-no-headings = “{ $path }” não tem nenhum título, por isso chega como um item.
import-diagnostic-unsupported-format = Nada lê ".{ $detail }" arquivos, então "{ $path }" foi ignorado.
import-diagnostic-front-matter-not-flat = Matéria frontal em “{ $path }”: “{ $detail }” não é um valor simples e foi ignorado.
import-diagnostic-footnotes-degraded = { $count ->
    [one] Uma nota de rodapé em “{ $path }” chega como texto simples: notas de rodapé não são lidas de Markdown.
   *[other] { $count } notas de rodapé em “{ $path }” chegam como texto simples: notas de rodapé não são lidas de Markdown.
}
import-diagnostic-footnote-not-carried = { $count ->
    [one] Uma nota de rodapé em “{ $path }” não pôde ser trazida. Os outros foram.
   *[other] { $count } notas de rodapé em “{ $path }” não puderam ser trazidas. Os outros foram.
}
import-diagnostic-raw-html-dropped = { $count ->
    [one] Um bloco de HTML bruto em “{ $path }” foi derrubado.
   *[other] { $count } blocos de HTML brutos em “{ $path }” foram derrubados.
}
import-diagnostic-nested-break-dropped = { $count ->
    [one] Uma quebra de cena dentro de uma citação ou lista em “{ $path }” foi abandonada. Apenas uma ruptura em sua própria linha é mantida.
   *[other] { $count } quebras de cena dentro de citações ou listas em “{ $path }” foram abandonadas. Apenas uma ruptura em sua própria linha é mantida.
}
import-diagnostic-image-not-ingested = “{ $path }” refere-se à imagem “{ $detail }”. A referência chega como texto; a imagem em si não é copiada.
import-diagnostic-duplicate-title = “{ $title }” aparece { $count } vezes. Se você já importou esses arquivos antes, isso irá duplicá-los.
import-diagnostic-heading-level-jump = “{ $title }” salta do nível do cabeçalho { $from } para { $to }; é colocado um nível abaixo de seu pai.
import-diagnostic-illegal-combination = “{ $title }” carrega prosa, mas um { $kind } não pode segurar qualquer. Importação é realizada até que você mude seu tipo ou deixá-lo de fora. Nada é importado de outra forma.
import-diagnostic-tracked-changes-flattened = { $path } foi a meio da revisão: { $count ->
    [one] { $count } mudanças rastreadas foram aceitas
   *[other] { $count } alterações rastreadas foram aceitas
}, e as exclusões caíram. Esse é o texto final. Para manter qualquer uma das suas próprias palavras, use Comparar no último passo.
import-diagnostic-tracked-changes-flattened-by = { $path } foi a meio da revisão: { $count ->
    [one] { $count } mudanças rastreadas por { $names } foi aceito
   *[other] { $count } alterações rastreadas por { $names } foram aceitas
}, e as exclusões caíram. Esse é o texto final. Para manter qualquer uma das suas próprias palavras, use Comparar no último passo.
import-diagnostic-text-box-dropped = { $count ->
    [one] { $path } possui { $count } caixa de texto. Seu texto fica fora do fluxo do documento, de modo que onde ele pertence a um manuscrito não pode ser respondido.
   *[other] { $path } possui { $count } caixas de texto. Seu texto fica fora do fluxo do documento, portanto, onde ele pertence a um manuscrito não pode ser respondido. Eles não são importados.
}
import-diagnostic-embedded-object-dropped = { $count ->
    [one] { $path } possui { $count } objeto embutido: um gráfico, uma equação ou similar. Não há nada em um manuscrito que poderia segurá-lo.
   *[other] { $path } contém { $count } objetos embutidos: um gráfico, uma equação ou similar. Não há nada em um manuscrito que possa segurá-los.
}
import-diagnostic-field-flattened = { $count ->
    [one] { $path } possui { $count } campo: um número de página, uma referência cruzada, uma data. Mantém o texto que foi mostrado pela última vez e não irá atualizar novamente.
   *[other] { $path } contém { $count } campos: um número de página, uma referência cruzada, uma data. Cada um mantém o texto que foi mostrado pela última vez e não irá atualizar novamente.
}
import-diagnostic-unknown-style-level = { $path } usa o estilo “{ $detail }”, que parece um título, mas não nomeia nenhum nível. Esses parágrafos são importados como prosa em vez de adivinhado em uma profundidade.
import-diagnostic-comment-unanchored = O comentário “{ $detail }” em { $path } não pôde ser anexado às palavras que era sobre. É mantido em seu item, onde você pode movê-lo.
import-diagnostic-comment-not-carried = O comentário “{ $detail }” em { $path } não é importado, porque nada no ficheiro chega como texto que poderia ser mantido. Copie-o do ficheiro se você precisar dele.
import-diagnostic-comment-replies-flattened = { $count ->
    [one] { $count } resposta em { $path } nomeado um comentário que não está no ficheiro, por isso ele chega como um comentário próprio.
   *[other] { $count } respostas em { $path } nomearam um comentário que não está no ficheiro, então eles chegam como comentários próprios.
}
import-diagnostic-prose-not-verbatim = { $count ->
    [one] { $count } parágrafo em { $path } não poderia ser trazido com sua formatação exatamente como escrito, de modo que chega como texto simples. Compare-o com o original.
   *[other] { $count } parágrafos em { $path } não puderam ser trazidos com sua formatação exatamente como escrito, de modo que eles chegam como texto simples. Compare-os com o original.
}
import-diagnostic-styled-spaces-not-carried = { $count ->
    [one] Em { $path }, { $count } trecho de espaço em branco foi sublinhado ou atravessado, uma linha deixada para preencher, por exemplo. Skribisto não mantém nenhuma linha em espaços sozinhos, de modo que chega como espaços simples, ou não em tudo onde começa ou termina um parágrafo.
   *[other] Em { $path }, { $count } trechos de espaço em branco foram sublinhados ou atravessados, linhas deixadas para preencher, por exemplo. Skribisto não mantém nenhuma linha em espaços sozinhos, de modo que chegam como espaços simples, ou não em tudo onde começam ou terminam um parágrafo.
}
import-diagnostic-list-nesting-flattened = { $count ->
    [one] { $count } item listado em { $path } foi aninhado mais de { $limit } níveis de profundidade. Chega ao nível { $limit }, o mais profundo Skribisto mantém, com suas palavras e formatação.
   *[other] { $count } itens listados em { $path } foram aninhados mais de { $limit } níveis de profundidade. Eles chegam ao nível { $limit }, o mais profundo Skribisto mantém, com suas palavras e formatação.
}
import-diagnostic-table-read-as-paragraphs = { $count ->
    [one] { $count } tabela em { $path } tem uma fileira muito mais larga do que o resto, muito desleixado para se estabelecer como uma grade. Suas células chegam como parágrafos, um após o outro, com suas palavras e formatação.
   *[other] { $count } tabelas em { $path } têm uma fileira muito mais larga do que o resto, muito desleixado para se estabelecer como uma grade. Suas células chegam como parágrafos, um após o outro, com suas palavras e formatação.
}
import-diagnostic-spaces-shortened = { $count ->
    [one] { $count } corrida de espaços em { $path } foi maior do que { $limit } espaços, e chega encurtado para { $limit }. As palavras ao redor dele estão todos lá.
   *[other] { $count } corridas de espaços em { $path } foram maiores que { $limit } espaços, e chegar encurtado para { $limit }. As palavras ao redor deles estão todos lá.
}
import-diagnostic-epigraph-not-carried = “{ $title }” é encabeçado por uma epígrafe, mas um { $kind } não pode segurar um. A citação é mantida no topo de seu texto em vez disso.
import-diagnostic-epigraph-placement-ambiguous = Uma epígrafe situa-se entre “{ $title }” e “{ $below }” e poderia ir para qualquer um deles. Foi dada para “{ $title }”, que é onde uma epígrafe geralmente vai.
import-document-diagnostics = { $errors ->
    [0] { $warnings ->
            [one] 1 coisa para saber
           *[other] { $warnings } coisas para saber
        }
   *[other] { $errors ->
            [one] Não foi possível ler 1 ficheiro
           *[other] { $errors } arquivos não puderam ser lidos
        }
}
import-document-diagnostics-none = Não há nada a reportar.

versions-title = Versões
versions-scope-synopsis = Sinopse
versions-scope-prose = Texto
versions-loading = Olhando através de seus backups…
versions-empty = Nenhuma versão anterior disto ainda
versions-error = Não foi possível ler os seus backups. Nada foi perdido, mas esta lista pode estar incompleta
versions-did-not-exist = Ainda não existia em { $date }
versions-deleted-after = Excluído algum tempo após { $date }
versions-unreadable = { $count ->
    [one] { $count } backup não pôde ser lido
   *[other] { $count } backups não puderam ser lidos
}
versions-thinned = { $count ->
    [one] Versões mais antigas diminuem à medida que envelhecem. A história do próprio projeto já caiu { $count } estado anterior deste texto.
   *[other] Versões mais antigas diminuem à medida que envelhecem. A própria história do projeto já deixou cair { $count } estados anteriores deste texto.
}
versions-source-backup = De uma cópia de segurança
versions-source-project = Da história do próprio projeto
versions-list-caption = Uma entrada por alteração, não por backup
versions-pick-a-version = Escolha uma versão para ver o que mudou
versions-earliest = Não há nada mais antigo para comparar.
versions-no-change = Nada mudou nesta parte do item
versions-formatting-only = Apenas a formatação foi alterada aqui. As palavras são as mesmas
versions-show-unchanged = Mostrar inalterado
versions-hide-unchanged = Ocultar inalterado
versions-next-change = Próxima alteração
versions-near = perto de "{ $text }"
versions-words-added = { $count ->
    [one] { $count } palavra adicionada
   *[other] { $count } palavras adicionadas
}
versions-words-removed = { $count ->
    [one] { $count } palavra removida
   *[other] { $count } palavras removidas
}
versions-blocks-moved = { $count ->
    [one] { $count } n.o
   *[other] { $count } parágrafos movidos
}
versions-pin = Pin a cópia de segurança de que esta versão veio. A limpeza automática nunca a apagará
versions-unpin = Desbloqueie a cópia de segurança de onde esta versão veio. A limpeza automática poderá excluí- la novamente
versions-pinned-only = Mostrar apenas versões fixas
versions-pin-note = Apenas versões de um backup podem ser fixadas. Um pino mantém um ficheiro, e o próprio histórico do projeto não é um.
versions-pinned-empty = Nada aqui está preso ainda.
versions-pinned-empty-log = Nada aqui está preso ainda. Somente versões de um backup podem ser fixadas. Um pino mantém um ficheiro, e o próprio histórico do projeto não é um.
versions-range-filter = Mostrar apenas versões entre duas datas
versions-filtered-empty = Nenhuma versão corresponde aos filtros que você definiu
versions-clear-filters = Limpar os filtros
versions-last-30-days = Últimos 30 dias
versions-restore-button = Restaurar esta versão
versions-restore-confirm-title = Substituir este texto pela versão de { $date }?
versions-restore-confirm-text = O que você tem agora será substituído pelo texto que este item tinha em { $date }.
versions-restore-confirm-with-comments = O que você tem agora será substituído pelo texto que este item tinha em { $date }. { $count ->
    [one] O comentário { $count } está ancorado no texto atual e pode ficar órfão.
   *[other] { $count } comentários estão ancorados no texto atual e podem ficar órfãos.
}
versions-restore-confirm-undo-note = Um backup é feito primeiro, e Ctrl+Z desfaz isso em um passo.
versions-restored-toast = Restaurada a versão de { $date }
versions-undo = Anular
versions-restore-row-gone = Esse item já não está neste projeto
versions-restore-no-home = Este item mudou de tipo desde então, e o texto antigo não tem para onde ir nele
versions-restore-no-safety-copy = Os reforços de segurança não funcionaram, por isso nada mudou.
versions-restore-failed = A restauração falhou: { $error }
versions-restore-backup-busy = Uma cópia de segurança já está em execução. Tente novamente num momento
versions-restore-in-backup-file = Você está olhando para um ficheiro de backup; abra o próprio projeto para restaurá-lo
versions-restore-no-project = Nenhum projeto está aberto
versions-recreate-button = Traga isto de volta…
versions-recreate-picker-title = Para onde deve ir?
versions-recreate-picker-empty = Este projeto não tem nenhum ficheiro para colocá-lo
versions-recreate-picker-confirm = Trá-lo de volta aqui.
versions-recreate-picker-cancel = Cancelar
versions-recreate-untitled = este item
versions-recreate-confirm-title = Trazer "{ $item }" de volta?
versions-recreate-confirm-text = Será adicionado a { $destination }, com o texto que tinha em { $date }.
versions-recreate-confirm-undo-note = Desfazer, na mensagem que se segue, leva-a de volta.
versions-recreated-toast = “{ $item }” está de volta ao seu projeto
versions-recreated-partial-toast = { $count ->
    [one] “{ $item }” está de volta, mas um de seus textos não pôde ser lido
   *[other] “{ $item }” está de volta, mas { $count } de seus textos não puderam ser lidos
}
versions-recreate-already-here = Esse item já está no seu projeto
versions-recreate-no-destination = Escolha um lugar na pasta para colocá-lo.
versions-recreate-unreadable = Aquele backup não podia ser lido, então nada foi adicionado
versions-recreate-failed = Não consegui trazê-lo de volta: { $error }
versions-changed-percent = { $percent }% destas alterações
versions-hidden-paragraphs = { $count ->
    [one] … { $count } inalterado parágrafo …
   *[other] … { $count } parágrafos inalterados …
}

timeline-title = Voltar atrás no tempo
timeline-coverage = { $count ->
    [one] { $count } versão gravada, voltando para { $oldest }
   *[other] { $count } versões gravadas, voltando para { $oldest }
}
timeline-loading = Olhando o passado do seu projeto…
timeline-empty = Nenhuma versão deste projeto foi gravada ainda
timeline-no-changes = Nada mudou desde então.
timeline-no-text-changes = Nenhum texto foi alterado desde então
timeline-slider-label = Versão gravada
timeline-series-name = Tamanho do projeto
timeline-bars-caption = Cada barra é uma versão gravada, tão alta como o projeto era então. O destaque é o que você está olhando.
timeline-bars-caption-periods = Muitas versões para mostrar uma por uma, então cada barra é um { $period }, tão alto quanto o projeto era no final dele.
timeline-unit-hour = hora
timeline-unit-day = dia
timeline-unit-week = semana
timeline-unit-month = mês
timeline-range-filter = Mostrar apenas versões entre duas datas
timeline-range-empty = Nenhuma versão foi gravada nessas datas
timeline-open-period = Abrir este período
timeline-show-all = Mostrar todo o histórico
timeline-changed-since = { $count ->
    [one] { $count } item difere entre { $date } e seu projeto agora
   *[other] { $count } itens diferem entre { $date } e seu projeto agora
}
timeline-kind-added = Escrito desde então
timeline-kind-removed = Não está mais no projeto
timeline-kind-changed = Editado desde então
timeline-kind-moved = Movido desde
timeline-not-yet-written = Isto ainda não existia nessa altura.
timeline-no-text-of-its-own = Isto não tem texto próprio, é um rumo para o que está dentro dele.
timeline-reader-close = Fechar
timeline-reader-stamp = Como foi em { $date }
timeline-reader-loading = Abertura da versão gravada…
timeline-reader-unreadable = Esta gravação não pôde ser lida. O backup pode ter sido movido, excluído, ou estar em uma unidade que não está conectada.
timeline-reader-compared = Como foi em { $date }, colocado contra o que ele diz agora
timeline-reader-diff-legend = Atravessaram-se desde então; sublinharam-se.
timeline-reader-view-label = O que mostrar
timeline-reader-view-diff = Alterações
timeline-reader-view-text = Texto
timeline-reader-deleted = Isto já não está no teu projecto, podes lê-lo e copiá-lo aqui.
timeline-prose-only-record = Este ponto vem do próprio histórico do projeto, que mantém o texto e nada mais, então o que foi excluído ou movido já que não pode ser mostrado. Escolha um backup para isso.

settings-sec-extensions = Extensões

settings-group-games-forward = Sempre em frente
settings-games-forward-toggle = Jogue "Sempre para a frente"
settings-games-forward-hint =
    Enquanto você joga, nada do que você escreveu pode ser retirado: Backspace, Delete,
    Cortar, arrastar e soltar e Desfazer estão todos desativados nas superfícies que você escolher abaixo.
    Você ainda pode digitar, colar, formatar e mover livremente. O rascunho só cresce.
settings-games-session-warning =
    Esta é uma escolha por sessão: ela nunca é salva. Fechando o projeto ou desistindo
    Skribisto, sempre termina o jogo, e tudo o que você escreveu enquanto jogava se torna
    Não é possível, no momento em que o fizer.
settings-group-games-scope = Quando aplicável
settings-games-in-prose = Prosa manuscrita
settings-games-in-synopsis = Sinopses
settings-games-scope-hint =
    Comentários, notas de rodapé e títulos nunca são congelados: eles são onde você nota a correção
    Tu acabaste de te proibir de fazer.
settings-games-inert-warning =
    "Sempre para a frente" está ligado mas não se aplica a nada. Tique pelo menos uma superfície acima,
    ou não muda nada enquanto escreves.

games-title = Gravando jogos
games-forward-name = Sempre em frente
games-forward-blurb =
    Rascunho sem levar nada de volta. Apagar, cortar e desfazer estão desativados
    Escreva a próxima frase em vez de consertar a última.
games-forward-playing = Reproduzindo: excluir está desativado
games-forward-idle = Não jogar
games-session-note = Termina quando fechares o projecto.
games-scope-prose-and-synopsis = Aplica-se à sua prosa e sinopses.
games-scope-prose = Aplica-se à sua prosa.
games-scope-synopsis = Aplica-se às vossas sinopses.
games-scope-nothing = Ainda não se aplica a nada. Escolha uma superfície em Definições.
games-settings-link = Configuração do jogo de gravação…
statusbar-games-forward = Sempre em frente
statusbar-games-forward-tooltip =
    "Sempre para a frente" está ligado: excluir, cortar e desfazer estão desativados enquanto você
    rascunho. Clique para parar de jogar.
goal-progress-words = { $g ->
    [one] { $count } de { $goal } palavra
   *[other] { $count } de { $goal } palavras
}
goal-progress-characters = { $g ->
    [one] { $count } de { $goal } caráter
   *[other] { $count } de { $goal } carateres
}
goal-count-words = { $n ->
    [one] { $count } palavra
   *[other] { $count } palavras
}
goal-count-characters = { $n ->
    [one] { $count } caracteres
   *[other] { $count } carateres
}
goal-unit-words = Palavras
goal-unit-characters = Caracteres

inspector-goal = Objetivo
inspector-goal-none = Sem objetivo

overview-col-goal = Objetivo
overview-excluded-from-export = Deixado de fora da exportação, por isso não conta para nenhum total

goal-subtree-total-words = { $count ->
    [one] { $items } alvo dentro soma até { $words } palavras
   *[other] { $items } alvos dentro adicionar até { $words } palavras
}
goal-subtree-total-characters = { $count ->
    [one] { $items } alvo dentro adiciona até { $words } caracteres
   *[other] { $items } alvos dentro adicionar até { $words } caracteres
}

new-work-goal-unit = Contar em
new-work-goal-unit-hint = Usado para cada palavra ou alvo de caracteres neste projecto. Alterável mais tarde.

settings-group-goal-unit = Alvos
settings-goal-unit-switch-title = Mudar a unidade de contagem
settings-goal-unit-switch-text = Os objetivos e marcos já estabelecidos para este projeto foram inseridos em { $from }. Mudar para { $to } não os converte, então cada número existente será agora lido como { $to }.
settings-goal-unit-switch-informative = Nada está perdido. Alternar de volta a qualquer momento para restaurar a leitura original e atualizar os alvos que você deseja manter.
settings-goal-unit-switch-confirm = Mudar

goal-distribute-action = Distribuir…
distribute-title = Distribuir o alvo
distribute-empty = Não há nada dentro disto para partilhar o alvo.
distribute-weight = Partilhar
distribute-weight-length = Por comprimento
distribute-weight-rows = Por peças
distribute-weight-even = Por igual
distribute-overwrite = Substituir os alvos que já estão definidos
distribute-col-item = Item
distribute-col-current = Agora.
distribute-col-proposed = Após
distribute-total = Estes somam até { $total }, contra um alvo de { $goal }.
distribute-over-budget = Os alvos já estabelecidos no interior chegam a { $over } mais do que os deste contentor. Levantem o seu alvo, baixem o deles ou substituam-nos.
distribute-apply = Distribuir
distribute-cancel = Cancelar

milestone-target-gone = Alvo apagado
milestone-no-target = Sem objetivo
milestone-add = Adicionar
milestone-add-label = O que deve ser alcançado

pace-summary-title = Onde está o livro
pace-summary-remaining = Faltam { $words }.
pace-summary-open = Abra o plano
pace-summary-close = Fechar
pace-summary-dont-show = Não mostrar isto ao abrir
pace-summary-menu = Plano de escrita…

menu-help-topics = &Tópicos de Ajuda
menu-help-shortcuts = &Atalhos do teclado…
menu-help-website = &Skribisto na Web
menu-help-report = &Relatório um problema…
menu-help-check-updates = Pr&ocurar atualizações
menu-command-palette = &Paleta de Comando…

learn-help-topics = Tópicos de ajuda
learn-shortcuts = Atalhos de teclado
learn-website = Skribisto na web

help-window-title = Ajuda
help-filter-topics = Filtrar tópicos
help-no-matching-topic = Nenhum tópico coincide.
help-topic-missing = Este tópico já não está disponível.
help-back = Anterior
help-not-translated = Esta página ainda não foi traduzida, pelo que é apresentada em inglês.

help-section-getting-started = Começar
help-section-writing = Gravando
help-section-reviewing = Revisão
help-section-exchanging = Obtendo palavras dentro e fora
help-section-keeping = Mantendo seu trabalho seguro
help-section-extensions = Extensões

help-topic-getting-started = Seu primeiro projeto
help-topic-writing-model = Como um livro é montado
help-topic-drafts-and-old-versions = Manter um rascunho mais antigo
help-topic-goals-and-pace = Alvos e ritmo
help-topic-comments = Comentários
help-topic-round-trip = Enviando seu livro para um leitor
help-topic-export = Exportar
help-topic-import-documents = Importação de documentos
help-topic-import-projects = Trazer todo um projeto
help-topic-backups-and-versions = Cópias de segurança e versões

help-shortcuts-title = Atalhos de teclado
help-shortcuts-filter = Filtrar atalhos
help-shortcuts-no-matches = Nenhuma combinação de atalhos.
help-shortcuts-rebind = Mudar atalhos…
help-shortcuts-close = Fechar

command-palette-placeholder = Digite um comando

shortcut-name-binder-duplicate = Duplicar
shortcut-name-comments-add = Adicionar Comentário
shortcut-name-comments-add-paragraph = Comentário sobre o Parágrafo
shortcut-name-spellcheck-toggle = Verificar ortografia
shortcut-name-editor-tab-close = Fechar aba
shortcut-name-editor-tab-pin = Fixar ou desafixar separador
shortcut-name-editor-save = Guardar
shortcut-name-work-export = Exportar...
shortcut-name-work-new = Novo projeto
shortcut-name-work-open = Abrir projeto
shortcut-name-window-new = Nova janela
shortcut-name-work-close = Fechar projeto
shortcut-name-app-settings = Definições
shortcut-name-app-quit = Sair
shortcut-name-editor-insert-footnote = Inserir uma Nota de rodapé
shortcut-name-format-scene-break = Inserir quebra de cena
shortcut-name-format-major-scene-break = Inserir quebra de cena principal
shortcut-name-format-link = Inserir uma Ligação
shortcut-name-format-mark-nbsp = Inserir espaço sem quebras
shortcut-name-format-mark-nbhyphen = Inserir Hífen Sem Quebra
shortcut-name-format-mark-nnbsp = Inserir Espaço Estreito Sem Quebra
shortcut-name-format-mark-zwsp = Inserir Espaço de Largura Zero
shortcut-name-go-next = Seguinte
shortcut-name-go-prev = Anterior
shortcut-name-go-to = Ir para
shortcut-name-outline-toggle = Mostrar ou ocultar estrutura
shortcut-name-preview-toggle = Mostrar ou ocultar pré-visualização
shortcut-name-view-fullscreen = Alternar ecrã inteiro
shortcut-name-view-focus-mode = Alternar o Modo sem Distração
shortcut-name-editor-size-increase = Aumentar o Tamanho do Texto
shortcut-name-editor-size-decrease = Diminuir o Tamanho do Texto
shortcut-name-editor-size-reset = Reiniciar o Tamanho do Texto
shortcut-name-editor-find = Procurar
shortcut-name-editor-replace = Substituir
shortcut-name-editor-find-next = Próxima Correspondência
shortcut-name-editor-find-prev = Correspondência Anterior
shortcut-name-search-show = Pesquisar no projeto
shortcut-name-search-replace = Substituir no Projeto
shortcut-name-outline-open-to-side = Abrir para o Lado
shortcut-name-help-topics = Tópicos de Ajuda
shortcut-name-help-shortcuts = Atalhos do Teclado
shortcut-name-help-website = Skribisto na Web
shortcut-name-help-report = Relatar um Problema
shortcut-name-help-check-updates = Procurar atualizações
update-check-behind = A versão { $version } está disponível.
update-check-current = Você está executando a versão mais nova.
update-check-failed = Não foi possível verificar uma versão mais recente. { $error }
shortcut-name-command-palette = Paleta de Comando
help-section-reference = O que são as coisas

help-concept-scene-break = Quebra de cena
help-concept-major-scene-break = Grande quebra de cena
help-concept-find-in-prose = Procurar no texto
help-concept-story-bible = Bíblia narrativa
help-concept-goal-unit = Palavras ou caracteres
help-concept-goal-progress = Progresso
help-concept-manuscript-words = O que conta como o manuscrito
help-concept-exportable = Deixado de fora da exportação
help-concept-distribute = Partilhar um alvo
help-concept-subtree-total = O que os alvos dentro somam a
help-concept-milestone = Marco
help-concept-pace-plan = Plano de ritmo

help-concept-tag = Etiqueta
help-concept-label = Legenda
help-concept-point-of-view = Ponto de vista
help-concept-epigraph = Epígrafe
help-concept-footnote = Nota de rodapé
help-concept-chapter-mode = Forma de capítulo
help-concept-comment = Comentário
help-concept-backup = Cópia de Segurança
help-concept-version = Versão
help-concept-trash = Lixo
help-concept-spellcheck = Verificação ortográfica
help-concept-search-replace = Procurar e substituir
help-concept-note-template = Modelo de nota
help-concept-story-bible-entry = Entrada da Bíblia da História
help-concept-text-replacement = Substituição de texto
help-concept-smart-punctuation = Pontuação inteligente
help-concept-export-style = Estilo de exportação
help-concept-round-trip-marks = Marcadores de ida e volta

epigraph-mark-attribution = Linha de origem
epigraph-mark-attribution-tip = Marque a linha em que o careta está como fonte da citação, para que imprima como uma atribuição
margin-lane-name = Marcas de margem
margin-lane-provider-comments = Comentários
margin-lane-provider-story-bible = Entrada da Bíblia da História
margin-lane-provider-story-bible-hint = Em nota em prosa leitura, onde essa entrada é nomeada, e as cenas contadas através de seus olhos. Também marca esses nomes na prosa em si.
margin-lane-mark-point-of-view = Foi dito daqui.
margin-lane-mark-named-here = Nome dado aqui
margin-lane-provider-comments-hint = Quando uma nota é anexada ao texto
margin-lane-provider-search = Pesquisar hits
margin-lane-provider-search-hint = Cada hit para o que você procurou pela última vez, onde quer que esteja no documento
margin-lane-provider-boundaries = Onde cada documento começa
margin-lane-provider-boundaries-hint = Uma regra no topo de cada cena em um fluxo, para que você possa ver onde você está
margin-lane-provider-spelling = Ortografia
margin-lane-provider-spelling-hint = Onde o verificador de feitiços marcou uma palavra. Off a menos que você pedir por ela: marca a opinião de uma máquina de sua prosa
margin-lane-spelling = { $word }, possível erro ortográfico
margin-lane-search-hit = { $text }, partida { $index } de { $total }
margin-lane-search-current = { $text }, partida { $index } de { $total }, a que você está em
margin-lane-boundary = Início de { $title }
margin-lane-boundary-untitled = um documento sem título
settings-page-margin-lane = Marcas de margem
settings-desc-margin-lane = A tira ao lado da barra de rolagem, e o que ela mostra
settings-margin-lane-enabled = Mostrar a faixa de margem
settings-margin-lane-enabled-hint = O mesmo interruptor que as marcas de Margem View ▸.
settings-margin-lane-enabled-more = Nunca diz nada de errado: mostra onde as coisas estão, e deixa o que fazer com isso para você.
settings-group-margin-lane-marks = O que marca
settings-margin-lane-no-providers = Ainda nada marca a pista.
settings-group-margin-lane-texture = Textura do diálogo
settings-margin-lane-texture = Mostrar a textura do diálogo
settings-margin-lane-texture-hint = Uma barra por parágrafo: quanto tempo é, e quanto é falado.
settings-margin-lane-texture-more = Não medido para idiomas sem convenção com curadoria, onde a barra mostra comprimento sozinho.
settings-group-margin-lane-surfaces = Onde ele aparece
settings-margin-lane-surface-editor = Editor de texto
settings-margin-lane-surface-stream = Fluxos
settings-margin-lane-surface-search-preview = Visualização da pesquisa
menu-margin-lane = &Marcas de margem

status-preset-drafting = Redação
status-preset-passes = Passes de revisão
status-preset-plume = Plume Creator
status-todo = Para fazer
status-draft = Rascunho
status-revised = Revisado
status-final = Final
status-outline = Estrutura
status-first-edit = 1a edição
status-second-edit = 2a edição
status-done = Concluído
status-plume-draft-1 = 1o rascunho
status-plume-draft-2 = 2o rascunho
status-plume-draft-3 = 3o rascunho
status-plume-edit-1 = 1a edição
status-plume-edit-2 = 2a edição
status-plume-edit-3 = Terceira revisão
status-plume-proofread = Revisão final
status-plume-finished = Terminado
new-work-statuses = Fluxo de trabalho
new-work-statuses-hint = As etapas pelas quais você move uma cena. Você pode renomear, reordenar ou adicionar a elas a qualquer momento.
status-none = Sem status
inspector-status = Estado
overview-col-status = Estado
overview-status-mixed = as partes abaixo discordam
help-concept-status = Estado
status-completion-title = Onde está o livro
status-completion-headline = { $done } de { $total } cenas terminadas
status-completion-empty = Isto lê o manuscrito, por isso preenche-o enquanto escreves.
status-completion-open = Onde o livro está…

settings-page-statuses = Status
settings-desc-statuses = Renomear, reordenar, adicionar e remover as etapas pelas quais você move uma cena
settings-statuses-add = Adicionar status
settings-statuses-add-placeholder = Novo nome de estado
settings-statuses-desc =
    As etapas uma cena move-se, em ordem, de pelo menos terminado no topo para a maioria
    terminou na parte inferior. Essa ordem é o que "menos terminado do que" significa em qualquer outro lugar
    no aplicação, então coloque-os da forma como seu processo realmente funciona.
settings-statuses-apply-preset = Aplicar uma predefinição...
settings-statuses-preset-applied = { $added ->
    [one] 1 status adicionado
   *[other] { $added } status acrescentado
}
settings-statuses-preset-refused = Este projecto já tem uma escada. As predefinições preenchem apenas uma vazia. Apague os degraus que não deseja primeiro.
settings-statuses-duplicate = "{ $name }" já está nesta escada.
settings-statuses-added = Adicionado "{ $name }"
settings-statuses-move-up = Mover para cima (menos terminado)
settings-statuses-move-down = Mover para baixo (mais terminado)
settings-statuses-details-placeholder = O que significa esta fase (opcional)
settings-statuses-delete = Excluir "{ $name }"
settings-statuses-delete-in-use = { $count ->
    [one] Excluir "{ $name }": 1 item está nele e perderá seu status
   *[other] Excluir "{ $name }": { $count } itens estão nele e perderá seu status
}
settings-statuses-deleted = Suprimido "{ $name }"
settings-statuses-deleted-in-use = { $count ->
    [one] Excluído "{ $name }". 1 item já não tem um status.
   *[other] Excluido "{ $name }". { $count } itens não têm mais status.
}
settings-statuses-empty-title = Sem fluxo de trabalho ainda
settings-statuses-empty-body = Um status diz a distância de uma cena. Adicione uma acima, ou comece a partir de uma predefinição.
status-category-planned = Planeado
status-category-drafting = Redação
status-category-needs-work = Precisa de trabalho
status-category-revised = Revisado
status-category-final = Final
