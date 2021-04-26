# Skribisto



**Skribisto** is born from the ashes of **Plume Creator**, keeping the goals while adopting more recent ways to think an application.

Skribisto is geared toward helping anyone write anything, be it a novel or course notes. The user is free to use tags to define texts or 
write hundreds of notes. The tools are designed to be most unobstrusive, so you can write, write and write a bit more without too much distraction.

With Skribisto, the user can create and organize text papers (called "sheets"). Exactly the same is possible with the notes. Sheets can link to a synopsis and multiple notes, or create them on the fly while writing.

What Skribisto is not : LibreOffice, Calligra or Word. Any project can be exported to .odt so as to make use of these complete text processors formatting abilities before printing.

Accessibility is too often forgotten. I'm trying to keep the interface accessible for screen readers, as much as Qt let me implement it. Jaws and NVDA are my screen readers for testing. Please contact me if there is a glaring lack in the accessibility.

## Goals

Short term goal is to rejoin its ancestor Plume Creator feature-wise. A few outstanding features are below. Bold means this feature is already implemented

- **navigating between texts**
- **distraction-free mode**
- **rich text (bold/italic/underline/strikeout)**
- **synopsis**
- **label (named 'tag' in Plume) next to each text title**
- **autosave**
- **spellcheking**
- **color themes**
- **overview of all texts**
- **character/word count**
- **exporting to .txt/.odt/.PDF**
- **printing**
- advanced search/replace
- display quickly the end of the previous text and the beginning of the next text
- character/word goal

Skribisto will add to these features with :

- **dynamic layouts adaptating to all devices (like a phone)**
- **tagging system**
- **touch-friendly**
- **navigating between notes**
- **a text can have several notes in addition of the synopsis**
- **manual save**
- **backup with mutliple paths**
- **accessible for screen readers (NVDA or JAWS)**
- **Open texts in a new window**
- **Open texts in tabs**
- **Linux (Flatpak) support**
- **importing old Plume Creator projects**
- **texts overview**
- **notes overview**
- Windows 10 support
- Examples
- Help page
- project management
- each note or text can take snapshots
- on-the-fly notes from the context menu

Medium term goals are :
- Adjoining documents to texts (without insertion)
- Insert images into the text
- Gallery tab to manage all external documents/images
- Android support


## For tech people, under the hood

All the application is rewritten from scratch using C++ Qt for back-end and QML for the front-end. The QML allows for a touch friendly and dynamic interface.

Each project is a SQLite3 file, more robust than the zipped projects in Plume.



## Build it, test it

### Windows

#### By hand, for development

Install Qt 5.15.2 on Windows 10

##### Building prerequisites 

- Download the latest from GitHub, then you can use Qt Creator to open the superbuild at *cmake/Superbuild/CMakeLists.txt* in the project. 
- Configure against Qt 5.15 MinGW minimum to be sure.
- **Before** compiling it, set the build directory (in Projects tab) to *build_skribisto_Release* just outside the skribisto folder
- add the CMake variable QT_VERSION_MAJOR and set its value to 5 or 6 depending of your Qt version, apply the change
- Build it against 
- After successfully compiling it, close the Skribisto-Superbuild project

##### Building 

- Open CMakeLists.txt_ at the root of the project
- In Qt Creator, in Projects tab, in the build subsection, add the CMake variable SKR_DEV (bool) and set its value to "ON", apply the change
- Compile it

##### Running it

- In Qt Creator, in Projects tab, in the run subsection, copy the value of environment variable *Path* in a text editor.
- Add to the line the paths of the prerequisites dll. For example, I added to the original Path value : 
	;C:\Users\jacqu\Devel\build_skribisto_Release\3rdparty\zlib\bin;
	C:\Users\jacqu\Devel\build_skribisto_Release\3rdparty\quazip\bin;
	C:\Users\jacqu\Devel\build_skribisto_Release\3rdparty\hunspell\bin;
	(make it only one line)
- run skribisto


##### automated building & packaging
- install Inno Setup found at https://jrsoftware.org/isdl.php
- unlock running Powershell scripts by running in admin Powershell :  *set-ExecutionPolicy RemoteSigned*
- Open PowerShell
	- cd skribisto\package\windows\
	- run .\packaging.ps1

### Linux

#### By hand, for development

Needed sources and libs :
- hunspell (devel)
- zlib (devel)
- quazip (devel), version 1.1 minimum needed. Install it by hand.

Minimum Qt : 5.15
If you have not Qt 5.15, use the Qt installer found at [Qt website](https://www.qt.io/download-open-source)
Install 5.15 Desktop or superior and Qt Creator
Open the project using the CMakeLists.txt file
Build and run it


#### Flatpak

##### Flatpak prerequisites

- make sure to have *flatpak* and *flatpak-builder* installed on your system

Prerequisites (>1Go):
```
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install flathub org.kde.Sdk//5.15
flatpak install flathub org.kde.Platform//5.15
```

Older flatpak-builder doesn't support YAML manifest files. Convert it to JSON using one of the online website easily found, then place the new file by eu.skribisto.skribisto.yml. Adapt the flatpak commands to point to this new file instead of the .yml file.

##### Flatpak from GitHub master branch :

- type in a terminal :



Compile :
```
mkdir ~/Devel
cd ~/Devel
git clone https://github.com/jacquetc/skribisto.git
flatpak-builder --user --repo=local-repo build-dir skribisto/package/flatpak/eu.skribisto.skribisto.yml --force-clean
```

Run only once :
```
flatpak build-update-repo local-repo
flatpak --user remote-add --no-gpg-verify local-repo local-repo
```

Install :

```
flatpak install local-repo eu.skribisto.skribisto -y --reinstall
```

Later, when a new version is online, you can update with this single line:
```
cd ~/Devel && flatpak-builder --user --repo=local-repo build-dir skribisto/package/flatpak/eu.skribisto.skribisto.yml --force-clean && flatpak install local-repo eu.skribisto.skribisto -y --reinstall
```


##### Flatpak from local source code:

You can copy/paste in ~/Devel/ the file *eu.skribisto.skribisto* found in \[skribisto-repo\]/package/flatpak/local/

Near the end of the file, in **skribisto** build module, adapt **path:** to your local repository (ex: path: /home/cyril/Devel/skribisto)



```
mkdir ~/Devel
cd ~/Devel
git clone https://github.com/jacquetc/skribisto.git
flatpak-builder --user --repo=local-repo build-dir skribisto/package/flatpak/local/eu.skribisto.skribisto.yml --force-clean
```

Run only once :
```
flatpak build-update-repo local-repo
flatpak --user remote-add --no-gpg-verify local-repo local-repo
```

Install :
```
flatpak install local-repo eu.skribisto.skribisto -y --reinstall
```


After you modified the code you want in whichever git branch you want, type this command :

```
cd ~/Devel && flatpak-builder --user --repo=local-repo build-dir skribisto/package/flatpak/local/eu.skribisto.skribisto.yml --force-clean && flatpak install local-repo eu.skribisto.skribisto -y --reinstall
```

## Translation :

<script type="text/javascript" src="https://www.google.com/jsapi"></script>
<script type="text/javascript" src="https://www.transifex.com/_/charts/js/skribisto/skribisto/inc_js/src-translations-skribisto-en-us-ts--develop/"></script>
<div id="txchart-skribisto-src-translations-skribisto-en-us-ts--develop">Loading chart...</div>

SKribisto uses Transifex to manage translation from english to any language.

https://www.transifex.com/skribisto/skribisto/


To help with translation, go [here](https://www.transifex.com/skribisto/skribisto/) and click on the "Help translate" button.

If your language isn't listed and you want to translate it, please create an (issue)[https://github.com/jacquetc/skribisto/issues] or send me an email at cyril.jacquet@skribisto.eu and I'll add it.

## Transififex integration with Skribisto

The source language is en_US (american english), the file is [src/translations/skribisto_en_US.ts](https://github.com/jacquetc/skribisto/blob/master/src/translations/skribisto_en_US.ts) from the "develop" branch. The en_US translation file is the only translation updated when a new to-be-translated sentence is added on the source code. 

There is no need to manualy use Qt's lupdate or lrelease. Any push request with any translation file other than skribisto_en_US.ts will be rejected.

Each time the project is built, skribisto_en_US.ts is automatically updated. Moreover, all the languages are compiled in ".qm" files at build time.

Any new language is detected without any declaration in the source code.

Transifex updates all languages with the new sentences without intervention.

The process is written in this [CMakeLists.txt](https://github.com/jacquetc/skribisto/blob/master/src/translations/skribisto_de_DE.ts)

Thank you to the Scribus project to have written a clean way to automatise translation. [See here](https://github.com/scribusproject/scribus/blob/master/resources/translations/CMakeLists.txt). I learnt a lot from it.

## To contact me :

cyril.jacquet@skribisto.eu







