# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Um livro no teu projecto.
wm-book-more = Um livro é tão simples ou tão em camadas como você gosta. Um conto pode ser apenas alguns [cenas](:wm-scene). Um trabalho mais longo pode nidificar [partes](:wm-part) e [Capítulos](:wm-chapter) tão profundo quanto você quiser. A forma é sua.
wm-part = Uma parte agrupando capítulos.
wm-part-more = Uma parte reúne [Capítulos](:wm-chapter) em um título. Como um capítulo, ele funciona até a parte seguinte ou o fim do [livro](:wm-book).
wm-chapter = Um capítulo do livro.
wm-chapter-more = Um capítulo vai daqui até o próximo capítulo, o próximo [parte](:wm-part), ou o [fim do livro](:wm-end-of-book). Ele vem em duas formas: uma pasta de capítulo que contém [cenas](:wm-scene), ou um capítulo plano que carrega sua própria prosa. Escolha qual um novo capítulo usar em Definições, sob Estrutura.
wm-scene = Uma cena de prosa.
wm-scene-more = Uma cena contém a sua escrita. Cada item de escrita tem dois lados: o texto principal e um [sinopse](:wm-synopsis) para planejar com. Ambos abertos juntos no editor.
wm-note = Um bilhete.
wm-note-more = Uma nota mantém o texto e um [sinopse](:wm-synopsis), como um [cena](:wm-scene), para pesquisa, apartes, ou lembretes. Qualquer item pode ser parte do livro compilado, incluindo uma nota sentado dentro de um [capítulo](:wm-chapter).
wm-note-folder = Uma nota que contém outros itens.
wm-note-folder-more = Um [nota](:wm-note) que também contém itens abaixo dele, para que você possa agrupar notas relacionadas juntos. Ele mantém o seu próprio [sinopse](:wm-synopsis), e os itens dentro manter seu próprio texto.
wm-folder = Uma pasta de organização.
wm-folder-more = Um contentor para organizar itens no fichário, no entanto, combina com você. Ele carrega um [sinopse](:wm-synopsis), mas nenhuma prosa própria. Os itens dentro segurar a escrita.
wm-end-of-book = Marca onde termina um livro.
wm-end-of-book-more = Cada livro compartilha uma lista contínua, de modo que o fim de um livro não é decidido pelo aninhamento. Este marcador diz que o [livro](:wm-book) pára aqui. Qualquer coisa depois que ele pertence ao próximo livro.
wm-story-bible-entry = Um bilhete marcado e pseudónimo sobre a sua história.
wm-story-bible-entry-more = Um [nota](:wm-note) sobre um caractere, um local, ou qualquer outra coisa que valha a pena arquivar, criado com um nome, etiquetas, pseudônimos e um template inicial tudo em um passo. Nada na página o distingue depois, uma vez que permanece uma nota comum que você pode renomear, remarcar ou converter como qualquer outro.
wm-synopsis = Um resumo de um item escrito.
wm-synopsis-more = Um resumo anexado a qualquer item de escrita, geralmente um parágrafo curto, mas o tempo que quiser. Use-o para planejar e navegar antes da prosa existir. Ele fica ao lado do texto principal no editor.

scene-break-minor = Uma quebra de cena comum: uma mudança de tempo, lugar ou ponto de vista dentro de um capítulo.
scene-break-minor-more =
    Marca uma separação *onde a inserir*, mesmo no meio de uma [cena](:wm-scene). Dividir o texto em dois elementos é uma escolha de organização e não cria uma separação por si só.

    Datilografado na prosa como três asteriscos separados por espaços. Como ele imprime é decidido pelo estilo de exportação, não pelo que você digita: um manuscrito Shunn define-o como um único #, um papel de troca como um dinkus, e a maioria francês, alemão, espanhol, russo e italiano publicação como uma lacuna nua sem marca em tudo.
scene-break-major = Uma divisão mais forte: um salto de grande escala, ou uma mudança decisiva de ponto de vista.
scene-break-major-more =
    A mesma ideia de uma quebra de cena normal, um passo acima. Use-a quando uma quebra simples subestimar o salto.

    Datilografado como # # #, e impresso de forma diferente da camada normal pelo estilo de exportação. O formato padrão do manuscrito de Shunn define um único # contra # # # # para exatamente esta distinção. Onde uma tradição não tem marca mais forte, ambos os níveis imprimir o mesmo.
wm-find-in-prose = Skribisto procura este item na sua prosa.
wm-find-in-prose-more = Active isto para as marcas que nomeiam as coisas sobre as quais escreve: caracteres, locais, objectos. Qualquer item que carregue uma tal marca corresponde à sua prosa pelo seu título e pelos outros nomes que lhe dá, de modo que cada [cena](:wm-scene) lista quem e o que aparece nela sem ligar nada à mão. Deixe- o de lado para as marcas que descrevem um item em vez de nomear um, como um estado de rascunho ou um lembrete para verificar a continuidade. Cada item que corresponde também se junta ao seu [Bíblia da história](:wm-story-bible).
wm-story-bible = Cada caractere, lugar e outra entrada marcada, reunida em uma grade.
wm-story-bible-more = Abre a partir de uma página de qualquer pasta de notas. As cartas agrupam- se por etiqueta, e cada nome em que nomeia uma resposta de entrada e em quantas cenas apareceu, desenhadas a partir de cada marca [encontrar em prosa](:wm-find-in-prose) no projecto. Filtrar por livro apenas estreita as cartas que mostram; nunca decide quais as entradas que existem.

tooltip-go-to = Pular para qualquer item na pasta (Ctrl+G)
synopsis-collapse-tooltip = Esconder a coluna de sinopse

wm-paratext = Um texto que não faz parte da história.
wm-paratext-more = Um prefácio, uma dedicação, uma palavra após a palavra, um colofão: escrever que pertence ao livro, mas não ao seu corpo. Exportado para onde quer que você o coloque, e nunca contado na contagem de palavras do manuscrito. Onde ele vai é até você: convenções diferem por país e editor.
wm-paratext-folder = Uma pasta para paratextos.
wm-paratext-folder-more = Em algum lugar para manter prefácios e após-palavras para que eles não atrapalhem o fichário. Organizando apenas: ele carrega um [sinopse](:wm-synopsis), mas não acrescenta nada ao livro exportado, nem mesmo seu próprio nome.

goal-target = Quanto tempo é que esta peça deve ser.
goal-target-more = Defina um alvo em qualquer coisa: um [cena](:wm-scene), um [capítulo](:wm-chapter), um [parte](:wm-part), todo o [livro](:wm-book). Cada um é o seu próprio número e está por conta própria, de modo que definir um alvo em um capítulo nunca muda o seu livro, e o alvo de um livro nunca é uma soma dos capítulos dentro dele. O que se soma é a escrita: o [progresso](:goal-progress) de um contentor é tudo escrito por baixo dele. Deixe um alvo em zero e simplesmente não está definido. O número é contado em [palavras ou caracteres](:goal-unit), qualquer que seja o uso deste projeto.
goal-unit = Se este projeto conta em palavras ou caracteres.
goal-unit-more = Uma escolha em todo o projeto, porque uma unidade de comprimento pertence ao manuscrito e seu mercado em vez de uma seção: medida de publicação em alemão e francês em sinais, japonês em folhas de 400 caracteres, chinês pelos mil caracteres, a maioria da publicação em língua inglesa em palavras. Ambos os números são mantidos lado a lado e nenhum deles é convertido para o outro, então mudar os pontos da unidade a cada [alvo](:goal-target) na outra figura e mudar de volta restaura o primeiro. O que ele não faz é converter nada, então os alvos já inseridos vão precisar de atualização à mão.
goal-progress = Quanto está escrito, contra o alvo.
goal-progress-explained = Uma peça com prosa própria é medida contra o seu próprio comprimento; um contentor contra [tudo escrito por baixo dele](:goal-manuscript-words). A barra aquece de vermelho a âmbar a verde à medida que a escrita entra, e muda novamente uma vez que a peça passou bem além do que foi planejado, o que vale a pena notar quando um capítulo tem um comprimento para manter.
goal-manuscript-words = O que conta como o manuscrito.
goal-manuscript-words-more = Apenas prosa que seria realmente exportada: [cenas](:wm-scene) e [Capítulos](:wm-chapter) carregando seu próprio texto. Um [nota](:wm-note) ou um [paratexto](:wm-paratext) não faz parte do manuscrito e nunca é contado para um. Qualquer coisa no lixo cai, e assim faz qualquer coisa [deixado fora da exportação](:goal-exportable).
goal-exportable = Este item é deixado fora da exportação.
goal-exportable-more = Seu próprio comprimento ainda mostra, porque a escrita ainda está lá, mas não conta para nenhum total, exatamente como irá aparecer em nenhum livro exportado. Deixar um contentor para fora não deixa as peças dentro dele para fora: cada item responde por si mesmo, que é o que o botão "Aplicar às elementos subordinados" ao lado do interruptor é para.
goal-distribute = Compartilhe este alvo através do que está dentro.
goal-distribute-more = Dividi um alvo [livro](:wm-book) ou [parte's](:wm-part) através de seus elementos subordinados imediatos, de modo que as peças se somam exatamente a ele. Por padrão, ele preenche apenas os que ainda não têm nenhum alvo e deixa seus próprios números sozinhos. O que ele escreve são comuns [alvos](:goal-target), livre para editar depois e livre para se separar: esta é uma ação, não um arranjo permanente.
goal-subtree-total = O que os alvos lá dentro somam.
goal-subtree-total-more = Um fato, não um alvo. Este projeto nunca trata o alvo de um contêiner como a soma dos que estão dentro dele, porque é assim que a figura de uma pasta acaba se movendo por conta própria quando uma cena dentro dela é dada um número. Esta linha simplesmente lhe diz para que seus próprios números chegam, então você pode compará-la com o próprio [alvo](:goal-target) do contêiner, ou [compartilhar um fora](:goal-distribute).
goal-milestone = Uma data para chegar a alguma coisa.
goal-milestone-more = Dois tipos. Um diz que o [livro](:wm-book) deve estar em um determinado comprimento por uma data, um ponto de passagem em sua própria curva. O outro diz que um particular [capítulo](:wm-chapter) ou [parte](:wm-part) deve ter atingido [o seu próprio alvo](:goal-target) até então, e lê que o número ao vivo em vez de manter uma cópia dele, então editar o alvo atualiza o marco também. Ambos aparecem no [plano de ritmo](:pace-plan) do livro.
pace-plan = O livro está a escrever.
pace-plan-more = Um [alvo](:goal-target) e um prazo, com os dias em que você escreve, transformou-se em quanto um dia precisa. Ele lê o próprio alvo do livro, então, defini-lo aqui e colocá-lo no Inspetor são o mesmo número em dois lugares. Waypoints ao longo do caminho são [marcos](:goal-milestone).

concept-tag = Uma etiqueta codificada por cores que você pode colocar em qualquer item, reutilizável em todo o projeto.
concept-tag-more = Marque um item para marcar: um status, um local, um thread que você está rastreando. A mesma tag pode sentar em qualquer número de itens, e um item pode carregar vários. Ligue [encontrar em prosa](:wm-find-in-prose) para uma tag que nomeie algo em seu livro, um personagem ou um lugar, e Skribisto começa a procurar por ele em sua prosa.
concept-label = Uma pequena nota que você escreve sob o título de um item, para seus próprios olhos.
concept-label-more = Não é um [tag](:concept-tag): uma etiqueta pertence apenas a um item, texto simples sem cor ou catálogo por trás dele, algo como "1o ponto de gráfico" ou "necessita de um nome". Defina-o a partir do menu de contexto do item, ou edite-o em linha na coluna de Legenda de Visão Geral; ele mostra como um pequeno subtítulo sob o título, no contorno e no fluxo similar.
concept-point-of-view = Cujos olhos são narrados através de uma cena: um ou mais do elenco de sua história.
concept-point-of-view-more = Colocado no Inspetor, ao lado do elenco. Escolhendo alguém ainda não no elenco adiciona-los lá também. Mais de um ponto de vista é uma escolha legal, útil para uma cena compartilhada ou uma mudança de olhos meio-livro, não algo Skribisto blocos. Candidatos vêm dos itens uma etiqueta [encontrar em prosa](:wm-find-in-prose) coloca em sua bíblia história.
concept-epigraph = Uma citação definida na cabeça de uma parte ou capítulo.
concept-epigraph-more = Seu próprio campo, separado do [capítulo](:wm-chapter) ou do [parte](:wm-part) prosa própria; um [cena](:wm-scene) ou um [nota](:wm-note) não carrega nenhum. Escreva um como uma citação comum; escreva vários e cada um se torna sua própria blockquote, impresso como sua própria epígrafe. Suas palavras nunca são contadas como palavras do manuscrito.
concept-footnote = Uma citação marcada na prosa, impressa como uma nota numerada.
concept-footnote-more = A referência é uma única marca dentro de suas palavras, então ela se move e apaga exatamente como as palavras ao seu redor fazem. Seu número nunca é armazenado: Skribisto trabalha-lo fresco de onde cada referência se senta em todo o manuscrito, de modo que inserir uma renumeração de cada nota depois dele. As próprias palavras de uma nota de rodapé são contadas para além do manuscrito, nunca dobrado nele.
concept-chapter-mode = Se um novo capítulo é uma pasta de cenas ou um único item plano.
concept-chapter-mode-more = Defina em Definições, sob Estrutura. Você escreve diretamente em um [capítulo](:wm-chapter) de qualquer forma; a única diferença é se ele também pode conter cenas abaixo dele. Ambas as codificações compilam para o mesmo livro, e você pode misturá- las livremente em um projeto. Promova converte um capítulo existente entre os dois sem perder uma palavra.
concept-comment = Uma nota ancorada em um trecho exato de prosa, mostrada na margem.
concept-comment-more = Ele se lembra das palavras que aponta e o que as cerca, não uma posição fixa, então sobrevive à edição normal e a uma recarga completa. Responda abaixo para manter um thread indo. Se a citação exata não puder mais ser encontrada, o comentário diz isso ao invés de silenciosamente desviar para a sentença errada.
concept-backup = Uma cópia ponto-em-tempo de todo o projeto, mantido ao lado dele.
concept-backup-more = Tomado em um cronograma que você escolher: em aberto, em close, em um intervalo, ou qualquer mistura, com cópias antigas podadas por idade ou por contagem. Abrindo um carrega-o em sua própria janela: você pode olhar e até mesmo editar, mas só guardar como ou restaurar mantém qualquer coisa. Diferente de um [versão](:concept-version), que rastreia o próprio passado de um item.
concept-version = O próprio passado de um item: cada mudança em seu texto ou sinopse.
concept-version-more = Construído a partir do próprio histórico de salvamento do projeto e seus backups juntos, um item por mudança ao invés de um por ficheiro. Escolha um item para ver o que mudou e restaure apenas o texto desse item para ele; o resto do projeto está intocado, e uma cópia de segurança é tirada primeiro. Diferente de um [backup](:concept-backup), que é uma cópia de tudo.
concept-trash = Um item destruído é escondido, não movido: fica exatamente onde estava.
concept-trash-more = O lixo só vira uma bandeira: o item mantém o seu lugar na pasta, marcada como inativa, em vez de removida. Restaure- a e reaparece nesse local exato, sem re- arquivamento para fazer. A Lixeira Esvaziada remove os itens destruídos para sempre, e cada um deles é uma etapa inabalável, como qualquer outra edição do projeto.
concept-spellcheck = Sublinha palavras que nenhum dicionário instalado reconhece, por linguagem de projeto.
concept-spellcheck-more = Cada projecto tem a sua própria língua de trabalho ou línguas; um item pode substituí- los por uma cena escrita noutra língua. Adicionar uma palavra Skribisto não sabe a partir do menu de contexto do editor, para um dicionário pessoal que passa para cada projecto. A verificação ortográfica desencadeia- a em todo o lado, até que o ligue de novo.
concept-search-replace = Encontra uma palavra ou frase em toda parte no projeto, não apenas o documento aberto.
concept-search-replace-more = Olha através de cada cena, nota, título, sinopse e etiqueta, e através de comentários e notas de rodapé também. Caso, correspondência de palavras inteiras e acento são três switches separados. Substituir Todos reescreve cada jogo revisado em uma passagem e um passo de desfazer, então Ctrl+Z leva de volta o lote inteiro, nunca apenas a última mudança.
concept-note-template = Um pedaço reutilizável de escrita que você inserir em qualquer coisa em que você está trabalhando.
concept-note-template-more = Uma folha de personagens em branco, um perfil de localização, uma folha de batida: escreva- a uma vez e solte- a onde quiser, em qualquer item, não só uma nota. Comece a partir de uma predefinição incorporada, importe um ficheiro .md ou .djot, ou escreva alguma coisa e escolha Documento. Salve como modelo. Armazenado no projeto, então um co- autor que o abre recebe o mesmo conjunto.
concept-text-replacement = Sua própria taquigrafia, expandida automaticamente à medida que digita.
concept-text-replacement-more = Defina um gatilho e sua substituição, "btw" para "a propósito", e ele expande o momento em que você digita um espaço ou pontuação depois dele. O caso segue o que você digitou: capitalize o gatilho e a expansão também capitaliza. Um Backspace logo após uma expansão desfaz apenas essa substituição. Alternado por projeto, distinto de [pontuação inteligente](:concept-smart-punctuation).
concept-smart-punctuation = Tipografia local aplicada automaticamente à medida que escreve: aspas, traços, elipses.
concept-smart-punctuation-more = Curls reta citações em tipografias, transforma -- em um traço e --- em um traço em, transforma ... em uma elipse, e, onde você liga, abre um parágrafo digitado como "-" com um traço de diálogo. Defina por projeto, e viaja dentro do .skrib de modo que um co-autor escreve com as mesmas regras. Distinto de [suas próprias regras](:concept-text-replacement), que você escreve a si mesmo.
concept-export-style = Um pacote chamado de opções de exportação: cabeçalhos, espaçamento, o que é incluído.
concept-export-style-more = Um estilo, reutilizado em todos os formatos de exportação que escolher. Os estilos incorporados são apenas de leitura; o duplicado para obter um ponto de partida que poderá realmente mudar. Entre as suas opções está a inclusão do [Marcadores de ida e volta](:concept-round-trip-marks), para um ficheiro com destino a um editor e de volta.
concept-round-trip-marks = Marcadores invisíveis que deixaram um editado DOCX ou ODT voltar para casa reconhecido.
concept-round-trip-marks-more = Escrito apenas nos dois formatos que um editor pode enviar de volta, DOCX e ODT; cada outra exportação não carrega nenhuma. Um marcador, não um atributo personalizado: LibreOffice tira atributos personalizados ao guardar, mas deixa um favorito intacto. Ligado por [estilo de exportação](:concept-export-style), em "Enviar para um editor".
concept-status = Onde um item está em seu próprio fluxo de trabalho: uma etapa de cada vez, avançando.
concept-status-more = Não um [tag](:concept-tag): um item tem exatamente um status, as etapas são ordenadas, e o projeto possui a lista, para que você possa renomear, reordenar ou adicionar a ele sempre que quiser. Um asterisco ao lado de um capítulo ou livro significa que as partes abaixo não concordam com ele. Defina-o a partir do Inspetor, um cartão, um item de fluxo, ou aqui.
