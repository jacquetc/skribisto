#ifndef CSTANTIDOTEAPI_H
#define CSTANTIDOTEAPI_H

namespace AntidoteAPI
{
// Constantes pour le chemin d'objet et le nom du service de l'API DBus
static const char* kPrefixeCheminObjet         = "/com/druide/antidote/dbus/api/v1/";
static const char* kPrefixeNomService          = "com.druide.antidote.dbus.api.v1.";

static const char* kCheminVersAntidote         = "/usr/local/bin/Antidote";// chemin pour lancer Antidote 9																							--	Path to launch Antidote 9
static const char* kCheminVersAgentConnectix   = "/usr/local/bin/AgentConnectix"; // chemin pour lancer Antidote 10																							--	Path to launch Antidote 10

static const char* kPrefixeModule              = "--module";
static const char* kChaineNomModule            = "API_DBus:v1.0";

static const char* kPrefixeIdentificateur      = "--identificateur";
static const char* kPrefixeOutil               = "--outil";

static const char* kPrefixeLangue              = "--langue";
static const char* kValeurLangueFr             = "FR";
static const char* kValeurLangueEn             = "EN";

// Constantes pour les différents outils d'Antidote
static const char* kOutilEditeurInterne				= "E";									//	éditeur interne dAntidote (Linux seulement)																--	Antidote internal editor (Linux only)
static const char* kOutilCorrecteur						= "C";									//	correcteur																																	--	corrector
static const char* kDictionnaireDernierChoisi = "D";									//	dernier dictionnaire appelé																									--	last dictionary opened
static const char* kDictionnaireDefaut				= "D_Defaut";						//	dictionnaire par défaut																											--	default dictionary
static const char* kDictionnaireDefinitions		= "D_Definitions";			//	dictionnaire Définitions																										--	Definitions dictionary
static const char* kDictionnaireSynonymes			= "D_Synonymes";				//	dictionnaire Synonymes																											--	Synonyms dictionary
static const char* kDictionnaireAntonymes			= "D_Antonymes";				//	dictionnaire Antonymes (module français seulement)													--	Antonyms dictionary (French module only)
static const char* kDictionnaireCooccurrences = "D_Cooccurrences";		//	dictionnaire Cooccurrences																									--	Combinations dictionary
static const char* kDictionnaireChampLexical	= "D_ChampLexical";			//	dictionnaire Champ lexical																									--	Semantic Field dictionary
static const char* kDictionnaireConjugaison		= "D_Conjugaison";			//	dictionnaire Conjugaison																										--	Conjugation dictionary
static const char* kDictionnaireFamille				= "D_Famille";					//	dictionnaire Famille																												--	Family dictionary
static const char* kDictionnaireCitations			= "D_Citations";				//	dictionnaire Citations																											--	Quotations dictionary
static const char* kDictionnaireHistorique		= "D_Historique";				//  dictionnaire Historique																											--	History dictionary
static const char* kDictionnaireWikipedia			= "D_Wikipedia";				//	dictionnaire Wikipédia																											--	Wikipedia dictionary
static const char* kGuideDernierChoisi				= "G";									//	dernier guide appelé																												--	last guide opened
static const char* kGuideDefaut								= "G_Defaut";						//	guide par défaut																														--	default guide
static const char* kGuideOrthographe					= "G_Orthographe";			//	guide Orthographe																														--	Spelling guide
static const char* kGuideLexique							= "G_Lexique";					//	guide Lexique																																--	Lexicon guide
static const char* kGuideGrammaire						= "G_Grammaire";				//	guide Grammaire																															--	Grammar guide
static const char* kGuideSyntaxe							= "G_Syntaxe";					//	guide Syntaxe																																--	Syntax guide
static const char* kGuidePonctuation					= "G_Ponctuation";			//	guide Ponctuation																														--	Punctuation guide
static const char* kGuideStyle								= "G_Style";						//	guide Style																																	--	Style guide
static const char* kGuideRedaction						= "G_Redaction";				//	guide Rédaction																															--	Business Writing guide
static const char* kGuideTypographie					= "G_Typographie";			//	guide Typographie																														--	Typography guide
static const char* kGuidePhonetique						= "G_Phonetique	";			//	guide Phonétique																														--	Phonetics guide
static const char* kGuideHistorique						= "G_Historique";				//	guide Historique																														--	History guide
static const char* kGuidePointsDeLangue				= "G_PointsDeLangue";		//	guide Points de langue																											--	Language Matters guide
}

#endif // CSTAPIANTIDOTE_H
