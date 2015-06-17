# plume-creator
Software for writers


## For developers 
You must set up the workspace before beginning to develop Plume.
Add to your environnmeent variables :

PLUME_DEVELOP_FROM=/path/to/plume/source/code

### In linux :
export PLUME_DEVELOP_FROM=/home/cyril/Devel/workspace_eclipse/plume-creator

#### Necessities :
Make sure you have pyQt 5.4 (for python3) or more, and it's dev tools (pyrcc5), installed

sudo apt-get install pyqt5-dev-tools

sudo apt-get install python3-pyqt5

sudo apt-get install python3-pyqt5.qtsql

sudo apt-get install python3-pyqt5.qtopengl

Python 3 needs a few modules to launch Plume properly

In a terminal :

sudo easy_install3 pip

sudo pip3 install yapsy

### In Windows :
    1.Create an account on GitHub (ex : gmcduling)
    
    2.Fetch the last update of Plume (and stay in sync every update) :
    
        1.install Github for Windows
            1.Download and install GitHub : https://github-windows.s3.amazonaws.com/GitHubSetup.exe
        2.After the install, log in with the previously created account
        3.Click on the "+" on the top right of GitHub and clone the repository :   https://github.com/jacquetc/plume-creator
        4.click on "master" on the top and select develop in the drop-down menu, then click on "sync" button on the right top
        5.when you want to fetch updates, click on the "sync" button on the right top
        6.Plume is now in your personal "documents/github/plume-creator" folder !
    3.Install Python 3 for windows
        1.install Python 3
            1.Download and install https://www.python.org/ftp/python/3.4.3/python-3.4.3.msi
                1.when prompted to choose packages, allow the last one on the list to be installed ("add python.exe to the path")
                2.let the other settings with the default values
         2.install the necessary modules
            1.launch a command line prompt (start menu/all programs/accessories/command line prompt) .
            2.type :       
            
            pip3 install yapsy
            
    4.Install PyQt5
        1.Download and install : http://sourceforge.net/projects/pyqt/files/PyQt5/PyQt-5.4.2/PyQt5-5.4.2-gpl-Py3.4-Qt5.4.2-x32.exe
    5.launch Plume :
        1.In the explorer, go to "c:\users\grant\documents\github\plume-creator\src\plume\"
        2.double-click on "plume.py" (or "plume")
