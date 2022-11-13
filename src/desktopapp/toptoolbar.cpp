#include "toptoolbar.h"
#include "interfaces/projectpageinterface.h"
#include "invoker.h"
#include "skrdata.h"
#include "ui_toptoolbar.h"
#include "viewmanager.h"


#include <QTimer>
#include <QToolBar>

TopToolBar::TopToolBar(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::TopToolBar)
{
    ui->setupUi(this);

    m_middleToolBar = new QToolBar;
    ui->horizontalLayout->addWidget(m_middleToolBar);
    ui->horizontalLayout->setStretchFactor(m_middleToolBar, 1);
    ui->horizontalLayout->setAlignment(m_middleToolBar, Qt::AlignHCenter);

    m_rightToolBar = new QToolBar;
    ui->horizontalLayout->addWidget(m_rightToolBar);

    setBackgroundRole(QPalette::Window);

    QTimer::singleShot(0, this, &TopToolBar::init);

}

    //---------------------------------------

void TopToolBar::init(){

    auto actionSwitch_theme = invoke<QAction>(this, "actionSwitch_theme");
    m_rightToolBar->addAction(actionSwitch_theme);





    QList<ProjectPageInterface *> pluginList =
            skrpluginhub->pluginsByType<ProjectPageInterface>();



    // reorder by weight, lightest is top, heavier is last

    std::sort(pluginList.begin(), pluginList.end(),
              [](ProjectPageInterface *plugin1, ProjectPageInterface
              *plugin2) -> bool {
        return plugin1->weight() < plugin2->weight();
    }
    );

    for(auto *plugin : pluginList){
        if(plugin->locations().contains("top-toolbar")){
            QAction *action = new QAction(QIcon(plugin->iconSource()), plugin->showButtonText(), this);
            action->setShortcutContext(Qt::WindowShortcut);

            QList<QKeySequence> sequences;
            for(const QString &shortcutString : plugin->shortcutSequences()){
                sequences.append(QKeySequence(shortcutString));
            }

            action->setShortcuts(sequences);

            QObject::connect(action, &QAction::triggered, this, [this, plugin](){
                int activeProjectId = skrdata->projectHub()->getActiveProject();

                ViewManager *viewManager = invoke<ViewManager>(this, "viewManager");
                viewManager->openViewAtCurrentViewHolder(plugin->pageType(), activeProjectId);
            });



            m_middleToolBar->addAction(action);
            m_middleToolBar->repaint();

        }
    }
}

TopToolBar::~TopToolBar()
{
    delete ui;
}


