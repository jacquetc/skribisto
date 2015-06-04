# plume-creator
Software for writers


## For developers 
You must set up the workspace before beginning to develop Plume.
Add to your environnmeent variables :

PLUME_DEVELOP_FROM=/path/to/plume/source/code

### In linux :
export PLUME_DEVELOP_FROM=/home/cyril/Devel/workspace_eclipse/plume-creator

### Necessities :
Make sure you have pyQt 5.4 (for python3) or more, and it's dev tools (pyrcc5), installed

sudo apt-get install pyqt5-dev-tools
sudo apt-get install python3-pyqt5
sudo apt-get install python3-pyqt5.qtsql
sudo apt-get install python3-pyqt5.qtopengl

Python 3 needs a few modules to launch Plume properly
In a terminal :
sudo easy_install3 pip
sudo pip3 install yapsy
