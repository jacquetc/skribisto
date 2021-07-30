#include <sys/types.h>
#include <unistd.h>
#include "ImplementationAntidote.h"
#include "AdaptateurAntidote.h"
#include "CstAntidoteAPI.h"
#include <QDateTime>

ImplementationAntidote::ImplementationAntidote(QObject *parent)
    : QObject(parent),     m_textDoc(nullptr), m_language(Context),
    m_connexionDBus(QDBusConnection::sessionBus()), m_adaptateur(nullptr)
{
    m_textTopMargin = 2;
    m_textIndent    = 2;


    // L'utilisation du pid permet de s'assurer que l'identificateur est unique,
    // ce qui
    // permet l'utilisation de plusieurs instances du texteur en parallèle.
    m_identificateur =
        QString("skribisto_%1_%2").arg(::getpid()).arg(QDateTime::currentDateTimeUtc().toString("yyyyMMddhhmmsszzz"));

    QString cheminObjet = AntidoteAPI::kPrefixeCheminObjet + m_identificateur;

    if (m_connexionDBus.registerObject(cheminObjet, this) == false) {
        qWarning() << "Erreur à  l'enregistrement de l'objet (" << cheminObjet << ") : " <<
            m_connexionDBus.lastError().message();
        return;
    }

    QString nomService = AntidoteAPI::kPrefixeNomService + m_identificateur;

    if (m_connexionDBus.registerService(nomService) == false) {
        qWarning() << "Erreur à  l'enregistrement du service (" << nomService << ") : " <<
            m_connexionDBus.lastError().message();
        return;
    }

    m_adaptateur = new AdaptateurAntidote(this);
}

// -------------------------------------------------------------------------

ImplementationAntidote::~ImplementationAntidote()
{
    m_connexionDBus.unregisterService(AntidoteAPI::kPrefixeNomService + m_identificateur);
    m_connexionDBus.unregisterObject(AntidoteAPI::kPrefixeCheminObjet + m_identificateur);
}

// -------------------------------------------------------------------------

int ImplementationAntidote::textIndent() const
{
    return m_textIndent;
}

void ImplementationAntidote::setTextIndent(int textIndent)
{
    m_textIndent = textIndent;
    this->createFormats();
    emit textIndentChanged(textIndent);
}

// -------------------------------------------------------------------------

int ImplementationAntidote::textTopMargin() const
{
    return m_textTopMargin;
}

void ImplementationAntidote::setTextTopMargin(int textTopMargin)
{
    m_textTopMargin = textTopMargin;
    this->createFormats();
    emit textTopMarginChanged(textTopMargin);
}

// -------------------------------------------------------------------------
void ImplementationAntidote::createFormats() {
    m_blockFormat.setTopMargin(m_textTopMargin);
    m_blockFormat.setTextIndent(m_textIndent);
}

// -------------------------------------------------------------------------

QQuickTextDocument * ImplementationAntidote::textDocument() const
{
    return m_textDoc;
}

void ImplementationAntidote::setTextDocument(QQuickTextDocument *textDocument)
{
    if (m_textDoc) {
        disconnect(m_textDoc->textDocument());
    }

    m_textDoc = textDocument;
    emit textDocumentChanged();

    if (m_textDoc) {
        m_textCursor = QTextCursor(m_textDoc->textDocument());
    }
}

// -------------------------------------------------------------------------

void ImplementationAntidote::activeDocument()
{
    emit activateDocument();
}

// -------------------------------------------------------------------------

int ImplementationAntidote::selectionStart() const
{
    return m_selectionStart;
}

void ImplementationAntidote::setSelectionStart(int selectionStart)
{
    m_selectionStart = selectionStart;
}

// -------------------------------------------------------------------------

int ImplementationAntidote::selectionEnd() const
{
    return m_selectionEnd;
}

void ImplementationAntidote::setSelectionEnd(int selectionEnd)
{
    m_selectionEnd = selectionEnd;
}

// -------------------------------------------------------------------------

void ImplementationAntidote::launchAntidote(TypeOuvrage ouvrage)
{
    QString cheminCompletVersAntidote(AntidoteAPI::kCheminVersAgentConnectix);

    // Liste d'arguments (argv) à  passer au lancement d'Antidote
    QStringList arguments;

    arguments << AntidoteAPI::kPrefixeModule << AntidoteAPI::kChaineNomModule;
    arguments << AntidoteAPI::kPrefixeIdentificateur << identificateur();

    // Forcer la langue
    if (m_language == French) arguments << AntidoteAPI::kPrefixeLangue << AntidoteAPI::kValeurLangueFr;
    else if (m_language == English) arguments << AntidoteAPI::kPrefixeLangue <<
            AntidoteAPI::kValeurLangueEn;

    // Ouvrage à  lancer
    arguments << AntidoteAPI::kPrefixeOutil;

    switch (ouvrage) {
    case Correcteur:
        arguments << AntidoteAPI::kOutilCorrecteur;
        break;

    case Dictionnaires:
        arguments << AntidoteAPI::kDictionnaireDernierChoisi;
        break;

    case Guides:
        arguments << AntidoteAPI::kGuideDernierChoisi;
        break;

    case DicoDernierChoisi:
        arguments << AntidoteAPI::kDictionnaireDernierChoisi;
        break;

    case DicoDefaut:
        arguments << AntidoteAPI::kDictionnaireDefaut;
        break;

    case DicoDefinition:
        arguments << AntidoteAPI::kDictionnaireDefinitions;
        break;

    case DicoSynonymes:
        arguments << AntidoteAPI::kDictionnaireSynonymes;
        break;

    case DicoAntonymes:
        arguments << AntidoteAPI::kDictionnaireAntonymes;
        break;

    case DicoCoocurrences:
        arguments << AntidoteAPI::kDictionnaireCooccurrences;
        break;

    case DicoChampLexical:
        arguments << AntidoteAPI::kDictionnaireChampLexical;
        break;

    case DicoConjugaison:
        arguments << AntidoteAPI::kDictionnaireConjugaison;
        break;

    case DicoFamille:
        arguments << AntidoteAPI::kDictionnaireFamille;
        break;

    case DicoCitations:
        arguments << AntidoteAPI::kDictionnaireCitations;
        break;

    case DicoHistorique:
        arguments << AntidoteAPI::kDictionnaireHistorique;
        break;

    case DicoWikipedia:
        arguments << AntidoteAPI::kDictionnaireWikipedia;
        break;

    case GuidesDernierChoisi:
        arguments << AntidoteAPI::kGuideDernierChoisi;
        break;

    case GuidesDefaut:
        arguments << AntidoteAPI::kGuideDefaut;
        break;

    case GuidesOrthographe:
        arguments << AntidoteAPI::kGuideOrthographe;
        break;

    case GuidesLexique:
        arguments << AntidoteAPI::kGuideLexique;
        break;

    case GuidesGrammaire:
        arguments << AntidoteAPI::kGuideGrammaire;
        break;

    case GuidesSyntaxe:
        arguments << AntidoteAPI::kGuideSyntaxe;
        break;

    case GuidesPonctuation:
        arguments << AntidoteAPI::kGuidePonctuation;
        break;

    case GuidesStyle:
        arguments << AntidoteAPI::kGuideStyle;
        break;

    case GuidesRedaction:
        arguments << AntidoteAPI::kGuideRedaction;
        break;

    case GuidesTypographie:
        arguments << AntidoteAPI::kGuideTypographie;
        break;

    case GuidesPhonetique:
        arguments << AntidoteAPI::kGuidePhonetique;
        break;

    case GuidesHistorique:
        arguments << AntidoteAPI::kGuideHistorique;
        break;

    case GuidesPointsDeLangue:
        arguments << AntidoteAPI::kGuidePointsDeLangue;
        break;
    }

    QProcess::startDetached(cheminCompletVersAntidote, arguments);
}

// -------------------------------------------------------------------------

void ImplementationAntidote::selectLanguage(Language language)
{
    m_language = language;

    //    if (language == Context)
    //    {
    //        actionUtiliser_le_contexte->setChecked(true);
    //        actionForcer_le_fran_ais->setChecked(false);
    //        actionForcer_l_anglais->setChecked(false);
    //    }
    //    else if (language == French)
    //    {
    //        actionUtiliser_le_contexte->setChecked(false);
    //        actionForcer_le_fran_ais->setChecked(true);
    //        actionForcer_l_anglais->setChecked(false);
    //    }
    //    else if (language == English)
    //    {
    //        actionUtiliser_le_contexte->setChecked(false);
    //        actionForcer_le_fran_ais->setChecked(false);
    //        actionForcer_l_anglais->setChecked(true);
    //    }
}

// -------------------------------------------------------------------------

void ImplementationAntidote::launchTool(const QString& toolName)
{
    if (toolName == "actionCorrecteur")
    {
        launchCorrector();
    }
    else if (toolName == "actionDernier_choisi")
    {
        launchAntidote(DicoDernierChoisi);
    }
    else if (toolName == "actionD_faut")
    {
        launchAntidote(DicoDefaut);
    }
    else if (toolName == "actionD_finitions")
    {
        launchAntidote(DicoDefinition);
    }
    else if (toolName == "actionSynonymes")
    {
        launchAntidote(DicoSynonymes);
    }
    else if (toolName == "actionAntonymes")
    {
        launchAntidote(DicoAntonymes);
    }
    else if (toolName == "actionCoocurrences")
    {
        launchAntidote(DicoCoocurrences);
    }
    else if (toolName == "actionChamp_lexical")
    {
        launchAntidote(DicoChampLexical);
    }
    else if (toolName == "actionConjugaison")
    {
        launchAntidote(DicoConjugaison);
    }
    else if (toolName == "actionFamille")
    {
        launchAntidote(DicoFamille);
    }
    else if (toolName == "actionCitations")
    {
        launchAntidote(DicoCitations);
    }
    else if (toolName == "actionHistorique")
    {
        launchAntidote(DicoHistorique);
    }
    else if (toolName == "actionWikip_dia")
    {
        launchAntidote(DicoWikipedia);
    }
    else if (toolName == "actionDernier_choisi_2")
    {
        launchAntidote(GuidesDernierChoisi);
    }
    else if (toolName == "actionD_faut_2")
    {
        launchAntidote(GuidesDefaut);
    }
    else if (toolName == "actionOrthographe")
    {
        launchAntidote(GuidesOrthographe);
    }
    else if (toolName == "actionLexique")
    {
        launchAntidote(GuidesLexique);
    }
    else if (toolName == "actionGrammaire")
    {
        launchAntidote(GuidesGrammaire);
    }
    else if (toolName == "actionSyntaxe")
    {
        launchAntidote(GuidesSyntaxe);
    }
    else if (toolName == "actionPonctuation")
    {
        launchAntidote(GuidesPonctuation);
    }
    else if (toolName == "actionStyle")
    {
        launchAntidote(GuidesStyle);
    }
    else if (toolName == "actionR_daction")
    {
        launchAntidote(GuidesRedaction);
    }
    else if (toolName == "actionTypographie")
    {
        launchAntidote(GuidesTypographie);
    }
    else if (toolName == "actionPhon_tique")
    {
        launchAntidote(GuidesPhonetique);
    }
    else if (toolName == "actionHistorique_2")
    {
        launchAntidote(GuidesHistorique);
    }
    else if (toolName == "actionPoints_de_langue")
    {
        launchAntidote(GuidesPointsDeLangue);
    }
}

// -------------------------------------------------------------------------

void ImplementationAntidote::corrigeDansTexteur(int debut, int fin, const QString& chaine, bool automatique)
{
    if ((debut >= 0) && (fin >= 0)) {
        m_textCursor.setPosition(debut);
        m_textCursor.movePosition(QTextCursor::NextCharacter, QTextCursor::KeepAnchor, fin - debut);
        m_textCursor.insertText(chaine);
        m_textCursor.clearSelection();
        m_textCursor.setPosition(debut);
        m_textCursor.movePosition(QTextCursor::NextCharacter, QTextCursor::KeepAnchor, chaine.length());
        m_textCursor.setBlockFormat(m_blockFormat);
    }

    // En cas de correction automatique, il peut y avoir de très nombreuses
    // corrections en peu de temps.
    // Il est alors inutile de défiler à chaque correction. CorrigeDansTexteur()
    // sera appelé une dernière
    // fois avec le paramètre automatique à false pour réactiver l'affichage et
    // le défilement
    if (automatique) {
        // m_texteur->setUpdatesEnabled(false);
    } else {
        // m_texteur->setUpdatesEnabled(true);
    }
}

QString ImplementationAntidote::donneBloc(int debut, int fin)
{
    m_textCursor.movePosition(QTextCursor::Start);
    m_textCursor.movePosition(QTextCursor::NextCharacter, QTextCursor::MoveAnchor, debut);
    m_textCursor.movePosition(QTextCursor::NextCharacter, QTextCursor::KeepAnchor, fin - debut);
    return m_textCursor.selectedText();
}

int ImplementationAntidote::donneDebutSelection()
{
    return m_selectionStart;
}

int ImplementationAntidote::donneFinDocument()
{
    m_textCursor.movePosition(QTextCursor::End);
    return m_textCursor.position();
}

int ImplementationAntidote::donneFinSelection()
{
    return m_selectionEnd;
}

int ImplementationAntidote::donnePositionFinBoite(int /*unePos*/)
{
    // Texteur simple, on n'y traite pas des sections
    return 0;
}

QString ImplementationAntidote::donneRetourDeCharriot()
{
    return "\n";
}

QString ImplementationAntidote::donneTitreDocument()
{
    return m_title;
}

bool ImplementationAntidote::permetsRetourDeCharriot()
{
    return true;
}

void ImplementationAntidote::retourneAuTexteur()
{
    emit forceFocusOnTextAreaCalled();
}

void ImplementationAntidote::rompsLienCorrecteur()
{ // Pas de désinitialisation car pas d'initialisation
}

void ImplementationAntidote::rompsLienTexteur()
{ // Pas de désinitialisation car pas d'initialisation
}

void ImplementationAntidote::selectionneIntervalle(int debut, int fin)
{
    m_textCursor.setPosition(debut);
    m_textCursor.movePosition(QTextCursor::NextCharacter, QTextCursor::KeepAnchor, fin - debut);

    // emit select(textCursor.position(), textCursor.anchor());
}
