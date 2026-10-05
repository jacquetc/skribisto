# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

wm-book = Um livro do seu projeto.
wm-book-more = Um livro é tão simples ou tão estratificado quanto quiser. Um conto pode ter apenas algumas [cenas](:wm-scene). Uma obra mais longa pode aninhar [partes](:wm-part) e [capítulos](:wm-chapter) até à profundidade que desejar. A forma é sua.
wm-part = Uma parte que agrupa capítulos.
wm-part-more = Uma parte reúne [capítulos](:wm-chapter) sob um mesmo título. Tal como um capítulo, estende-se até à parte seguinte ou ao fim do [livro](:wm-book).
wm-chapter = Um capítulo do livro.
wm-chapter-more = Um capítulo estende-se daqui até ao capítulo seguinte, à [parte](:wm-part) seguinte ou ao [fim do livro](:wm-end-of-book). Existe em duas formas: uma pasta de capítulo que contém [cenas](:wm-scene), ou um capítulo simples com a sua própria prosa. Escolha a forma usada pelos novos capítulos em Definições, em Estrutura.
wm-scene = Uma cena de prosa.
wm-scene-more = Uma cena contém a sua escrita. Todos os itens de escrita têm dois lados: o texto principal e uma [sinopse](:wm-synopsis) para planear. Ambos abrem juntos no editor.
wm-note = Uma nota.
wm-note-more = Uma nota guarda texto e uma [sinopse](:wm-synopsis), tal como uma [cena](:wm-scene), para pesquisa, apartes ou lembretes. Qualquer item pode fazer parte do livro compilado, incluindo uma nota colocada dentro de um [capítulo](:wm-chapter).
wm-note-folder = Uma nota que contém outros itens.
wm-note-folder-more = Uma [nota](:wm-note) que também contém itens por baixo dela, para poder agrupar notas relacionadas. Mantém a sua própria [sinopse](:wm-synopsis), e os itens no interior mantêm o seu próprio texto.
wm-folder = Uma pasta de organização.
wm-folder-more = Um contentor para organizar os itens no arquivador da forma que lhe convier. Tem uma [sinopse](:wm-synopsis), mas nenhuma prosa própria. São os itens no interior que contêm a escrita.
wm-end-of-book = Marca onde termina um livro.
wm-end-of-book-more = Todos os livros partilham uma única lista contínua, por isso o fim de um livro não é decidido pelo aninhamento. Este marcador indica que o [livro](:wm-book) termina aqui. Tudo o que vem depois pertence ao livro seguinte.
wm-story-bible-entry = Uma nota sobre a sua história, com etiquetas e outros nomes.
wm-story-bible-entry-more = Uma [nota](:wm-note) sobre uma personagem, um lugar ou qualquer outra coisa que mereça ser arquivada, criada com um nome, etiquetas, outros nomes e um modelo inicial, tudo num só passo. Depois disso, nada na página a distingue, pois continua a ser uma nota comum a que pode mudar o nome, as etiquetas ou o tipo como a qualquer outra.
wm-synopsis = Um resumo de um item de escrita.
wm-synopsis-more = Um resumo associado a qualquer item de escrita, normalmente um parágrafo curto, mas tão longo quanto quiser. Use-o para planear e navegar antes de a prosa existir. Fica ao lado do texto principal no editor.

scene-break-minor = Uma quebra de cena comum: uma mudança de tempo, lugar ou ponto de vista dentro de um capítulo.
scene-break-minor-more =
    Marca uma quebra *onde a colocar*, incluindo no meio de uma [cena](:wm-scene). Dividir a prosa em dois itens é uma escolha de organização, por isso nunca cria uma quebra por si só.

    Escreve-se na prosa como três asteriscos separados por espaços. A forma como é impressa é decidida pelo estilo de exportação, não pelo que escreve: um manuscrito Shunn compõe-na como um único #, uma edição comercial como um asterismo ou dinkus, e a maior parte da edição francesa, alemã, espanhola, russa e italiana como um simples espaço em branco, sem qualquer marca.
scene-break-major = Uma divisão mais forte: um grande salto no tempo ou uma mudança decisiva de ponto de vista.
scene-break-major-more =
    A mesma ideia que uma quebra de cena comum, um nível acima. Use-a quando uma quebra simples subestimaria o salto.

    Escreve-se como # # #, e o estilo de exportação imprime-a de forma diferente do nível comum. O formato de manuscrito padrão de Shunn usa um único # por oposição a # # # precisamente para esta distinção. Quando uma tradição não tem marca mais forte, os dois níveis são impressos da mesma forma.
wm-find-in-prose = O Skribisto procura este item na sua prosa.
wm-find-in-prose-more = Ative isto para etiquetas que nomeiam aquilo sobre que escreve: personagens, lugares, objetos. Qualquer item com uma etiqueta destas é procurado na sua prosa pelo título e pelos outros nomes que lhe der, para que cada [cena](:wm-scene) indique quem e o que nela aparece sem ter de ligar nada à mão. Deixe-o desativado para etiquetas que descrevem um item em vez de o nomear, como um estado do rascunho ou um lembrete para verificar a continuidade. Todos os itens encontrados passam também a fazer parte da sua [bíblia da história](:wm-story-bible).
wm-story-bible = Todas as personagens, lugares e outras entradas etiquetadas, reunidos numa grelha.
wm-story-bible-more = Abre-se no separador próprio de qualquer pasta de notas. Os cartões agrupam-se por etiqueta, e cada um indica a quantos outros nomes uma entrada responde e em quantas cenas apareceu, a partir de todas as etiquetas [procurar na prosa](:wm-find-in-prose) do projeto. Filtrar por livro apenas restringe os cartões mostrados; nunca decide que entradas existem.

tooltip-go-to = Saltar para qualquer item do arquivador (Ctrl+G)
synopsis-collapse-tooltip = Ocultar a coluna da sinopse

wm-paratext = Um texto que não faz parte da história.
wm-paratext-more = Um prefácio, uma dedicatória, um posfácio, um cólofon: texto que pertence ao livro, mas não ao seu corpo. Exportado onde quer que o coloque, e nunca contado na contagem de palavras do manuscrito. Onde o coloca depende de si: as convenções variam consoante o país e a editora.
wm-paratext-folder = Uma pasta para paratextos.
wm-paratext-folder-more = Um lugar para guardar prefácios e posfácios sem atravancar o arquivador. Serve apenas para organizar: tem uma [sinopse](:wm-synopsis), mas não acrescenta nada ao livro exportado, nem sequer o seu próprio nome.

goal-target = A extensão que este texto deve ter.
goal-target-more = Defina uma meta em qualquer item: uma [cena](:wm-scene), um [capítulo](:wm-chapter), uma [parte](:wm-part), o [livro](:wm-book) inteiro. Cada meta é um número próprio e independente, por isso definir uma meta num capítulo nunca altera a do livro, e a meta de um livro nunca é a soma dos capítulos que contém. O que se soma é a escrita: o [progresso](:goal-progress) de um contentor é tudo o que está escrito por baixo dele. Uma meta deixada a zero simplesmente não está definida. O número é contado em [palavras ou caracteres](:goal-unit), consoante o que este projeto usa.
goal-unit = Se este projeto conta em palavras ou em caracteres.
goal-unit-more = Uma escolha para todo o projeto, porque uma unidade de extensão pertence ao manuscrito e ao seu mercado e não a uma secção: a edição alemã e francesa mede em sinais, a japonesa em folhas de 400 caracteres, a chinesa aos milhares de caracteres, e a maior parte da edição em língua inglesa em palavras. Os dois números são mantidos lado a lado e nenhum é alguma vez convertido no outro, por isso mudar a unidade faz cada [meta](:goal-target) apontar para o outro número, e voltar atrás repõe o primeiro. Mas nada é convertido, por isso as metas já introduzidas terão de ser atualizadas à mão.
goal-progress = Quanto está escrito, face à meta.
goal-progress-explained = Um texto com prosa própria é medido pela sua própria extensão; um contentor, por [tudo o que está escrito por baixo dele](:goal-manuscript-words). A barra passa do vermelho ao âmbar e ao verde à medida que a escrita avança, e muda de novo quando o texto ultrapassa claramente o que estava planeado, o que convém notar quando um capítulo tem uma extensão a respeitar.
goal-manuscript-words = O que conta como o manuscrito.
goal-manuscript-words-more = Apenas a prosa que seria realmente exportada: [cenas](:wm-scene) e [capítulos](:wm-chapter) com texto próprio. Uma [nota](:wm-note) ou um [paratexto](:wm-paratext) não fazem parte do manuscrito e nunca contam para ele. Tudo o que está no lixo fica de fora, tal como tudo o que foi [deixado fora da exportação](:goal-exportable).
goal-exportable = Este item fica fora da exportação.
goal-exportable-more = A sua própria extensão continua a ser mostrada, porque a escrita continua lá, mas não conta para nenhum total, tal como não aparecerá em nenhum livro exportado. Deixar um contentor de fora não deixa de fora os itens que contém: cada item responde por si, e é para isso que serve o botão «Aplicar aos filhos» junto ao interruptor.
goal-distribute = Repartir esta meta pelo que está no interior.
goal-distribute-more = Divide a meta de um [livro](:wm-book) ou de uma [parte](:wm-part) pelos seus filhos diretos, para que as partes somem exatamente esse valor. Por predefinição, preenche apenas os que ainda não têm meta e deixa os seus próprios números intactos. O que escreve são [metas](:goal-target) comuns, que pode editar depois e que podem divergir livremente: é uma ação pontual, não uma regra permanente.
goal-subtree-total = O que somam as metas no interior.
goal-subtree-total-more = Um facto, não uma meta. Este projeto nunca trata a meta de um contentor como a soma das metas no seu interior, porque é assim que o número de uma pasta acaba por mudar sozinho quando se dá um número a uma cena lá dentro. Esta linha diz-lhe simplesmente quanto somam os seus próprios números, para que os possa comparar com a [meta](:goal-target) do contentor, ou [repartir uma](:goal-distribute).
goal-milestone = Uma data até à qual se deve atingir algo.
goal-milestone-more = Há dois tipos. Um diz que o [livro](:wm-book) deve ter uma dada extensão numa data, um ponto de passagem na sua própria curva. O outro diz que um determinado [capítulo](:wm-chapter) ou [parte](:wm-part) deve ter atingido [a sua própria meta](:goal-target) até essa data, e lê esse número em tempo real em vez de guardar uma cópia, por isso editar a meta atualiza também o marco. Ambos aparecem no [plano de ritmo](:pace-plan) do livro.
pace-plan = O calendário de escrita do livro.
pace-plan-more = Uma [meta](:goal-target) e um prazo, com os dias em que escreve, convertidos no que é preciso escrever por dia. Lê a meta do próprio livro, por isso defini-la aqui ou no Inspetor é o mesmo número em dois lugares. Os pontos de passagem pelo caminho são [marcos](:goal-milestone).

concept-tag = Uma etiqueta colorida que pode pôr em qualquer item, reutilizável em todo o projeto.
concept-tag-more = Etiquete um item para o assinalar: um lugar, um tema, um fio narrativo que está a seguir. A mesma etiqueta pode estar em qualquer número de itens, e um item pode ter várias. Ative [procurar na prosa](:wm-find-in-prose) para uma etiqueta que nomeia algo do seu livro, uma personagem ou um lugar, e o Skribisto passa a procurá-lo na sua prosa.
concept-label = Uma nota curta que escreve por baixo do título de um item, só para si.
concept-label-more = Não é uma [etiqueta](:concept-tag): um rótulo pertence a um único item, texto simples sem cor nem catálogo por trás, algo como «1.º ponto de viragem» ou «falta um nome». Defina-o no menu de contexto do item, ou edite-o diretamente na coluna Rótulo da Vista geral; aparece como um pequeno subtítulo por baixo do título, tanto na estrutura como no fluxo.
concept-point-of-view = Através dos olhos de quem uma cena é narrada: uma ou mais personagens do elenco da sua história.
concept-point-of-view-more = Define-se no Inspetor, junto ao elenco. Escolher alguém que ainda não está no elenco acrescenta-o também lá. Mais de um ponto de vista é uma escolha válida, útil para uma cena partilhada ou uma mudança de olhar a meio do livro, e não algo que o Skribisto impeça. Os candidatos vêm dos itens que uma etiqueta [procurar na prosa](:wm-find-in-prose) coloca na sua bíblia da história.
concept-epigraph = Uma citação colocada no início de uma parte ou de um capítulo.
concept-epigraph-more = Um campo próprio, separado da prosa do [capítulo](:wm-chapter) ou da [parte](:wm-part); uma [cena](:wm-scene) ou uma [nota](:wm-note) não têm nenhum. Escreva uma como uma citação comum; escreva várias e cada uma torna-se uma citação em bloco, impressa como uma epígrafe própria. As suas palavras nunca contam como palavras do manuscrito.
concept-footnote = Uma chamada marcada na prosa, impressa como nota numerada.
concept-footnote-more = A chamada é uma única marca no meio das suas palavras, por isso move-se e apaga-se exatamente como as palavras à sua volta. O seu número nunca é guardado: o Skribisto calcula-o de novo a partir da posição de cada chamada em todo o manuscrito, por isso inserir uma renumera todas as notas seguintes. As palavras de uma nota de rodapé são contadas à parte do manuscrito, nunca incluídas nele.
concept-chapter-mode = Se um novo capítulo é uma pasta de cenas ou um único item simples.
concept-chapter-mode-more = Define-se em Definições, em Estrutura. Escreve diretamente num [capítulo](:wm-chapter) em ambos os casos; a única diferença é se também pode conter cenas por baixo. As duas formas compilam o mesmo livro, e pode misturá-las livremente num projeto. Converter em converte um capítulo existente entre as duas sem perder uma palavra.
concept-comment = Uma nota ancorada num trecho exato de prosa, mostrada na margem.
concept-comment-more = Lembra-se das palavras para que aponta e do que as rodeia, não de uma posição fixa, por isso sobrevive à edição normal e a um recarregamento completo. Responda por baixo para continuar a conversa. Se a citação exata já não puder ser encontrada, o comentário di-lo, em vez de se deslocar silenciosamente para a frase errada.
concept-backup = Uma cópia de todo o projeto num determinado momento, guardada ao lado dele.
concept-backup-more = Feita segundo um calendário à sua escolha: ao abrir, ao fechar, a intervalos, ou qualquer combinação, com as cópias antigas eliminadas por idade ou por número. Abrir uma carrega-a na sua própria janela: pode consultá-la e até editá-la, mas só Guardar como ou Restaurar conservam alguma coisa. Diferente de uma [versão](:concept-version), que acompanha o passado de um único item.
concept-version = O passado de um item: cada alteração ao seu texto ou à sua sinopse.
concept-version-more = Construída a partir do histórico de gravações do próprio projeto e das suas cópias de segurança em conjunto, uma entrada por alteração e não uma por ficheiro. Escolha uma entrada para ver o que mudou e restaure apenas o texto desse item; o resto do projeto não é tocado, e é feita primeiro uma cópia preventiva. Diferente de uma [cópia de segurança](:concept-backup), que é uma cópia de tudo.
concept-trash = Um item no lixo fica oculto, não é movido: permanece exatamente onde estava.
concept-trash-more = Enviar para o lixo apenas muda um indicador: o item mantém o seu lugar no arquivador, marcado como inativo em vez de removido. Restaure-o e reaparece exatamente nesse lugar, sem nada para voltar a arrumar. Esvaziar o lixo remove definitivamente os itens que lá estão, e cada uma destas ações é um passo que se pode anular, como qualquer outra edição do projeto.
concept-spellcheck = Sublinha as palavras que nenhum dicionário instalado reconhece, por idioma do projeto.
concept-spellcheck-more = Cada projeto tem o seu idioma ou idiomas de trabalho; um item pode substituí-los numa cena escrita noutra língua. Adicione uma palavra que o Skribisto não conhece a partir do menu de contexto do editor, num dicionário pessoal que vale para todos os projetos. Desativar a verificação ortográfica para-a em todo o lado, até a voltar a ativar.
concept-search-replace = Encontra uma palavra ou expressão em todo o projeto, não apenas no documento aberto.
concept-search-replace-more = Percorre todas as cenas, notas, títulos, sinopses e rótulos, e também os comentários e as notas de rodapé. Maiúsculas/minúsculas, palavra inteira e acentos são três interruptores separados. Substituir tudo reescreve todas as correspondências revistas numa só passagem e num só passo de anulação, por isso Ctrl+Z desfaz o lote inteiro, nunca apenas a última alteração.
concept-note-template = Um texto reutilizável que insere em qualquer coisa em que esteja a trabalhar.
concept-note-template-more = Uma ficha de personagem em branco, um perfil de lugar, uma grelha de cenas: escreva-o uma vez e insira-o onde precisar, em qualquer item, não apenas numa nota. Comece a partir de uma predefinição integrada, importe um ficheiro .md ou .djot, ou escreva algo e escolha Documento ▸ Guardar como modelo. É guardado no projeto, por isso um coautor que o abra recebe o mesmo conjunto.
concept-text-replacement = As suas próprias abreviaturas, expandidas automaticamente à medida que escreve.
concept-text-replacement-more = Defina um acionador e a sua substituição, «btw» para «by the way», e ela expande-se assim que escrever um espaço ou um sinal de pontuação a seguir. As maiúsculas seguem o que escreveu: escreva o acionador com maiúscula e a expansão também fica com maiúscula. Um Backspace logo a seguir a uma expansão anula apenas essa substituição. Ativa-se por projeto, e é distinta da [pontuação inteligente](:concept-smart-punctuation).
concept-smart-punctuation = Tipografia adaptada ao idioma, aplicada automaticamente à medida que escreve: aspas, travessões, reticências.
concept-smart-punctuation-more = Converte as aspas retas em aspas tipográficas, -- num traço meia-risca e --- num travessão, ... em reticências e, se o ativar, abre com um travessão de diálogo um parágrafo escrito como "- ". Define-se por projeto e viaja dentro do .skrib, para que um coautor escreva com as mesmas regras. Distinta das [suas próprias regras](:concept-text-replacement), que escreve por si.
concept-export-style = Um conjunto nomeado de opções de exportação: cabeçalhos, espaçamento, o que é incluído.
concept-export-style-more = Um estilo, reutilizado em todas as exportações, seja qual for o formato escolhido. Os estilos integrados são só de leitura; duplique um para obter um ponto de partida que possa realmente alterar. Entre as suas opções está incluir ou não [marcadores de ida e volta](:concept-round-trip-marks), para um ficheiro que vai a um revisor e regressa.
concept-round-trip-marks = Marcadores invisíveis que permitem que um DOCX ou ODT editado regresse reconhecido.
concept-round-trip-marks-more = Escritos apenas nos dois formatos que um revisor pode devolver, DOCX e ODT; nenhuma outra exportação os inclui. Um marcador de texto, não um atributo personalizado: o LibreOffice elimina os atributos personalizados ao guardar, mas deixa intacto um marcador. Ativam-se por [estilo de exportação](:concept-export-style), em «Envio a um revisor».
concept-status = Em que ponto do seu próprio fluxo de trabalho está um item: uma etapa de cada vez, a avançar.
concept-status-more = Não é uma [etiqueta](:concept-tag): um item tem exatamente um estado, as etapas estão ordenadas e a lista pertence ao projeto, por isso pode mudar-lhes o nome, reordená-las ou acrescentar etapas sempre que quiser. Um asterisco junto a um capítulo ou livro significa que as partes abaixo não estão todas de acordo com ele. Defina-o a partir do Inspetor, de um cartão, de um item do fluxo, ou aqui.
