#ifndef IMPLEMENTATIONANTIDOTE_H
#define IMPLEMENTATIONANTIDOTE_H

#include <QtCore>
#include <QtGui>
#include <QtDBus>
#include <QQuickTextDocument>

class AdaptateurAntidote;

class ImplementationAntidote : public QObject {
    Q_OBJECT
    Q_PROPERTY(
        QQuickTextDocument *
        textDocument READ textDocument WRITE setTextDocument NOTIFY textDocumentChanged)
    Q_PROPERTY(
        int selectionStart READ selectionStart WRITE setSelectionStart NOTIFY cursorPositionChanged)
    Q_PROPERTY(
        int selectionEnd READ selectionEnd WRITE setSelectionEnd NOTIFY cursorPositionChanged)
    Q_PROPERTY(
        QString title MEMBER m_title)
    Q_PROPERTY(
        int textTopMargin MEMBER m_textTopMargin READ textTopMargin WRITE setTextTopMargin NOTIFY textTopMarginChanged)
    Q_PROPERTY(int textIndent MEMBER m_textIndent READ textIndent WRITE setTextIndent NOTIFY textIndentChanged)

public:

    enum TypeOuvrage {
        Correcteur, Dictionnaires, Guides,
        DicoDernierChoisi, DicoDefaut, DicoDefinition, DicoSynonymes, DicoAntonymes, DicoCoocurrences, DicoChampLexical,
        DicoConjugaison, DicoFamille, DicoCitations, DicoHistorique, DicoWikipedia,
        GuidesDernierChoisi, GuidesDefaut, GuidesOrthographe, GuidesLexique, GuidesGrammaire, GuidesSyntaxe,
        GuidesPonctuation, GuidesStyle, GuidesRedaction, GuidesTypographie, GuidesPhonetique, GuidesHistorique,
        GuidesPointsDeLangue
    };
    Q_ENUM(TypeOuvrage)
    enum Language {
        Context, French, English
    };
    Q_ENUM(Language)


    ImplementationAntidote(QObject *parent = nullptr);
    ~ImplementationAntidote();

    inline QString identificateur() const {
        return m_identificateur;
    }

    QQuickTextDocument* textDocument() const;
    void                setTextDocument(QQuickTextDocument *textDocument);


    int                 selectionStart() const;
    void                setSelectionStart(int selectionStart);

    int                 selectionEnd() const;
    void                setSelectionEnd(int selectionEnd);


    void                launchAntidote(TypeOuvrage ouvrage);


    Q_INVOKABLE void    launchCorrector() {
        launchAntidote(Correcteur);
    }

    Q_INVOKABLE void launchDictionaries() {
        launchAntidote(Dictionnaires);
    }

    Q_INVOKABLE void launchGuides() {
        launchAntidote(Guides);
    }

    void selectLanguage(Language language);
    void launchTool(const QString& toolName);

    void createFormats();
    int  textTopMargin() const;
    void setTextTopMargin(int textTopMargin);

    int  textIndent() const;
    void setTextIndent(int textIndent);

public slots:

    void activeDocument();
    void corrigeDansTexteur(int,
                            int,
                            const QString&,
                            bool);
    QString donneBloc(int,
                      int);
    int     donneDebutSelection();
    int     donneFinDocument();
    int     donneFinSelection();
    int     donnePositionFinBoite(int);
    QString donneRetourDeCharriot();
    QString donneTitreDocument();
    bool    permetsRetourDeCharriot();
    void    retourneAuTexteur();
    void    rompsLienCorrecteur();
    void    rompsLienTexteur();
    void    selectionneIntervalle(int,
                                  int);

signals:

    void textDocumentChanged();
    void cursorPositionChanged();
    void activateDocument();
    void forceFocusOnTextAreaCalled();
    void select(int begin,
                int end);
    void textTopMarginChanged(int value);
    void textIndentChanged(int value);

private:

    QDBusConnection m_connexionDBus;
    AdaptateurAntidote *m_adaptateur;
    QString m_identificateur;
    QQuickTextDocument *m_textDoc;
    int m_selectionStart;
    int m_selectionEnd;
    QString m_title;
    Language m_language;
    QTextCursor m_textCursor;
    QTextBlockFormat m_blockFormat;
    int m_textTopMargin;
    int m_textIndent;
};

#endif // IMPLEMENTATIONANTIDOTE_H
