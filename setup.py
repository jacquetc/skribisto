'''
Created on 30 avr. 2015

@author: Cyril Jacquet
'''
import sys, os
from cx_Freeze import setup, Executable


#############################################################################
# preparation des options
 
# chemins de recherche des modules
# ajouter d'autres chemins (absolus) si necessaire: sys.path + ["chemin1", "chemin2"]




path = sys.path + ["src/plume"]
 
# options d'inclusion/exclusion des modules
includes = [ "PyQt5.QtCore",
            "PyQt5.QtWidgets",
#            "PyQt5.QtGui",
#            "PyQt5.Qt",
#            "PyQt5.Qt",
#            "PyQt5.Qt",
            "PyQt5.QtGui",
            "PyQt5.Qt",
            "sip"
            ]  # nommer les modules non trouves par cx_freeze
excludes = ["PyQt5.QtNetwork",
            "PyQt5.QtSql",
            "PyQt5.QtSvg",
            "PyQt5.QtXml",
            "PyQt5.QtTest",
            "PyQt5.Enginio",
            "PyQt5.DBus",
            "PyQt5.Multimedia"
            
           # "PyQt5.",
            ]
packages = []  # nommer les packages utilises
 

# copier les fichiers non-Python et/ou repertoires et leur contenu:
includefiles = []
 
if sys.platform == "win32":
    pass
    # includefiles += [...] : ajouter les recopies specifiques à Windows
elif sys.platform == "linux2":
    pass
    # includefiles += [...] : ajouter les recopies specifiques à Linux
else:
    pass
    # includefiles += [...] : cas du Mac OSX non traite ici
 

base = None
if sys.platform == "win32":
    base = "Win32GUI"
    
binpathincludes = []
if sys.platform == "linux2":
    binpathincludes += ["/usr/lib"]

# niveau d'optimisation pour la compilation en bytecodes
optimize = 0
 
# si True, n'affiche que les warning et les erreurs pendant le traitement cx_freeze
silent = True


options = {"path": path,
#           "includes": includes,
           "excludes": excludes,
           "packages": packages,
           "include_files": includefiles,
           "bin_path_includes": binpathincludes,
#           "create_shared_zip": False,  # <= ne pas generer de fichier zip
#           "include_in_shared_zip": False,  # <= ne pas generer de fichier zip
#           "compressed": False,  # <= ne pas generer de fichier zip
           "optimize": optimize,
           "silent": silent
           }

build_exe_options = {"optimize": 2}


# pour inclure sous Windows les dll system de Windows necessaires
if sys.platform == "win32":
    options["include_msvcr"] = True
 
#############################################################################
# preparation des cibles
base = None
if sys.platform == "win32":
    base = "Win32GUI"  # pour application graphique sous Windows
    # base = "Console" # pour application en console sous Windows
 
icone = None
if sys.platform == "win32":
    icone = "icone.ico"
    
    
    
    
exe = Executable(
# what to build
   script = "src/plume/plume.py", # the name of your main python script goes here
   base=base,
   initScript = None,
   targetName = "plume-creator", # this is the name of the executable file
   copyDependentFiles = True,
   compress = True,
   appendScriptToExe = True,
   appendScriptToLibrary = True,
   icon = None # if you want to use an icon file, specify the file name here
)


setup(

    name = "plume-creator",

    version = "0.90",

    description = "Software for writers and novelists",
    options={"build_exe": options},
    executables = [exe],

)