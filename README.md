- [Skribisto](#skribisto)
  * [Goals](#goals)
  * [User manual](#user-manual)
  * [Support](#support)
  * [Help is always appreciated](#help-is-always-appreciated)
    + [Easier tasks for beginners](#easier-tasks-for-beginners)
  * [For tech people, under the hood](#for-tech-people--under-the-hood)
    + [Parts](#parts)
    + [Languages used](#languages-used)
    + [Plugins](#plugins)
      - [Interfaces](#interfaces)
      - [Existing plugins](#existing-plugins)
  * [Build it, test it](#build-it--test-it)
    + [The quickest and the easiest for development](#the-quickest-and-the-easiest-for-development)
      - [Building prerequisites](#building-prerequisites)
      - [Building](#building)
      - [Running it](#running-it)
    + [Windows](#windows)
        * [Automated building & packaging](#automated-building---packaging)
    + [Linux](#linux)
      - [By hand, for development but not the easiest](#by-hand--for-development-but-not-the-easiest)
      - [Flatpak](#flatpak)
        * [Flatpak prerequisites](#flatpak-prerequisites)
        * [Flatpak from GitHub master branch](#flatpak-from-github-master-branch)
        * [Flatpak from local source code](#flatpak-from-local-source-code)
    + [MacOS](#macos)
  * [Translation](#translation)
    + [Transifex integration with Skribisto](#transifex-integration-with-skribisto)
  * [To contact me](#to-contact-me)
  
# Skribisto

**Skribisto** is born from the ashes of **Plume Creator**, keeping the goals while adopting more recent ways to think an application.

Where its ancestor was geared toward writing novels, Skribisto aims to be more generic. The user can organize his project with items and folders. Each item displays a 'page' and
can be of a different type : 

- Text

   Dedicated to writing. Texts can have its own plan and can be linked to other items, or create them on the fly while writing.
   
- Folder

   Can contain child items or folders.
   
- Whiteboard (to be implemented)

   Think "OneNote". Write wherever you want on a white board, insert images, tables, lists... Then, you can modify, move and resize elements on the board.
   
- Section (to be implemented)

   Visible separations (book, act, chapter, end of book)

- Folder-Section (to be implemented)

  Folder with a section role.
  
Other types can be added in the future.  

The user is free to use tags to define any item. 

What Skribisto is not : LibreOffice, Calligra or Word. Any project can be exported to .odt so as to make use of these complete text processors formatting abilities before printing.

Accessibility is too often forgotten. I'm trying to keep the interface accessible for screen readers, as much as Qt let me implement it. Jaws and NVDA are my screen readers for testing. 
Please contact me if there is a glaring lack in the accessibility. Some technical choices have already been made so as to not hinder accessibility, 
like the seemingly strange choice of a classic drop-down menu on the top left of the window.

## Goals

Short term goal is to rejoin its ancestor Plume Creator feature-wise. A few outstanding features are below. Bold means this feature is already implemented

- **navigating between texts**
- **distraction-free mode**
- **e-ink friendly**
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
- **display quickly the end of the previous text**
- advanced search/replace
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
- **Windows 10 support**
- **Examples**
- **Help page**
- project management
- each item can take snapshots
- on-the-fly notes from the context menu

Medium term goals are :
- Adjoining documents to texts (without insertion)
- Insert images into the text
- Sort of gallery to manage all external documents/images
- Android support

Other features will be implemented more for fun. Thanks to the plugin system, Skribisto can accomodate other goals than writing. I added the "Writing Games" plugin for fun.


## User manual

The dedicated website for the user manual is [here](https://manual.skribisto.eu/en_US/manual.html)

The dedicated website for the FAQ is [here](https://manual.skribisto.eu/en_US/faq.html)

Each one can be translated (see [Translation](#translation) section)

## Discussions

A Discord server is available. Do you need help, offer suggestions or talk ? Join us [here](https://discord.gg/5BSkvQmyVH)

## Support

This is a GPL v3 project, so support is on a voluntary basis. Personally, I'll only accept bug issues from users if this user runs Skribisto using theses packaging methods :
- on Linux: Flatpak only
- on Windows: from a InnoSetup setup package generated by the PowerShell script

## Help is always appreciated

If this project takes your interest, if you want to help or wish for more details, you can contact me or create issues. 

### Easier tasks for beginners

- Solve one of the [good first issue](https://github.com/jacquetc/skribisto/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
- Comment whatever function you understand. C++ is obviously better organized than QML.
- Translate the software, the User manual or the FAQ (see [Translation](#translation) section)
- Complete the FAQ or the User Manual in the dedicated repositroy [here](https://github.com/jacquetc/skribisto-help-website/tree/develop)


## For tech people, under the hood

All the application is rewritten from scratch using C++ Qt for back-end and QML for the front-end. The QML allows for a touch friendly and dynamic interface.

Each project is a SQLite3 file, more robust than the zipped projects in Plume.

### Parts

- app

    QML UI, main.cpp and other C++ files dedicated to the UI
    
- libskribisto-data

    Back-end. Manage project files and offer an interface for all operations on a project, an item, plugins...

- plugins

   Contains all Qt-style C++ plugins.     

- translations

   Contains all the translation sources. DO NOT TOUCH directly. See the [Translation](#translation) section for details.

### Languages used
- C++ with Qt 6.2.2
- QML and Qt Quick
- Qt's Javascript
- SQL basics with SQLite3 only for the skribisto-data library.

C++/QML bindings are extensively used.

### Plugins

#### Interfaces

More and more, Skribisto allows to be extended with plugins. For now, plugins allow:

- add new pages types, combining theses interfaces: 
  - SKRCoreInterface, mandatory to allow activition/deactivation of plugins
  - SKRPageInterface, gives the details of a page, its QML view and the necessary to be an item in the navigation
  - SKRPageExporterInterface, allow a page to export their content to be exported/printed
  - SKRPageToolboxInterface, add a toolbox on the right dock for a specific page
  - SKRProjectToolboxInterface, add a toolbox on the left dock which will always stay

Plugin interfaces to come soon :
- exemples
- project templates
- import
- export

#### Existing plugins

- TextPage
- ThemePage

## Build it, test it

### The quickest and the easiest for development

Tested on Ubuntu 20.04, Fedora 33/34, Windows 10, MacOS Big Sur

Minimum Qt : 5.15
If you have not Qt 5.15, use the Qt installer found at [Qt website](https://www.qt.io/download-open-source)
Install 5.15 Desktop or superior and Qt Creator

#### Building prerequisites 

- Download the latest from GitHub, then you can use Qt Creator to open the superbuild at *cmake/Superbuild/CMakeLists.txt* in the project. 
- Configure against Qt 6.2 minimum to be sure.
- **Before** compiling it, set the build directory (in Projects tab) to *build_skribisto_Release* just outside the skribisto folder
    - Example:
    
       Git repo: /home/cyril/Devel/skribisto
       
       Superbuild's CMakeLists.txt: /home/cyril/Devel/skribisto/cmake/Superbuild/CMakeLists.txt
       
       Superbuild's build directory: /home/cyril/Devel/build_skribisto_Release

- add the CMake variable QT_VERSION_MAJOR and set its value to 5 or 6 depending of your Qt version, apply the change
- Compile it
- Ignore errors about Skribisto, we only want to build dependencies
- After compiling it, close the Skribisto-Superbuild project

#### Building 

- Open CMakeLists.txt at the root of the project
- In Qt Creator, in Projects tab, in the build subsection, add the CMake variable SKR_DEV (bool) and set its value to "ON", apply the change
- Compile it

#### Running it

- Run skribisto, optionally with --testProject



### Windows

##### Automated building & packaging
- install Inno Setup found at https://jrsoftware.org/isdl.php
- unlock running Powershell scripts by running in admin Powershell :  *set-ExecutionPolicy RemoteSigned*
- Open PowerShell
	- cd skribisto\package\windows\
	- run .\packaging.ps1

### Linux

#### By hand, for development but not the easiest

Needed sources and libs :
- hunspell (devel)
- zlib (devel)
- quazip (devel), version 1.1 minimum needed. Install it by hand.

Minimum Qt : 6.2.2
If you have not Qt 6.2.2, use the Qt installer found at [Qt website](https://www.qt.io/download-open-source)
Install 6.2.2 Desktop or superior and Qt Creator
Open the project using the CMakeLists.txt file
Build and run it, optionally with --testProject


#### Flatpak

##### Flatpak prerequisites

- make sure to have *flatpak* and *flatpak-builder* installed on your system

Prerequisites (>1Go):
```
flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
flatpak install flathub org.kde.Sdk//6.2
flatpak install flathub org.kde.Platform//6.2
```

Older flatpak-builder doesn't support YAML manifest files. Convert it to JSON using one of the online website easily found, then place the new file by eu.skribisto.skribisto.yml. Adapt the flatpak commands to point to this new file instead of the .yml file.

##### Flatpak from GitHub master branch

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


##### Flatpak from local source code

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

### MacOS

Using the Linux Superbuild instructions, it runs.

## Translation

![alt text](https://www.transifex.com/_/charts/redirects/skribisto/skribisto/image_png/ "Translation advancement")

As you can see in the chart, all the main languages are translated. That's not the reality ! A first pass of translation was done with Google Translate, 
so if you find errors or a distinct lack of logic in a few words, you know the culprit ! You are invited to fix such mistakes by following the below instructions.

SKribisto uses Transifex to manage translation from english to any language. 

[https://www.transifex.com/skribisto/skribisto/](https://www.transifex.com/skribisto/skribisto/)


To help with translation, go [here](https://www.transifex.com/skribisto/skribisto/) and click on the "Help translate" button. It's free. No tool required, nothing to install.

If your language isn't listed and you want to translate it, please create an (issue)[https://github.com/jacquetc/skribisto/issues] or send me an email at cyril.jacquet@skribisto.eu and I'll add it.

### Transifex integration with Skribisto

The source language is en_US (american english), the file is [src/translations/skribisto_en_US.ts](https://github.com/jacquetc/skribisto/blob/master/src/translations/skribisto_en_US.ts) from the "develop" branch. The en_US translation file is the only translation updated when a new to-be-translated sentence is added on the source code. 

There is no need to manualy use Qt's lupdate or lrelease. Any push request with any translation file other than skribisto_en_US.ts will be rejected.

Each time the project is built, skribisto_en_US.ts is automatically updated. Moreover, all the languages are compiled in ".qm" files at build time.

Any new language is detected without any declaration in the source code.

Transifex updates all languages with the new sentences without intervention.

The process is written in this [CMakeLists.txt](https://github.com/jacquetc/skribisto/blob/master/src/translations/skribisto_de_DE.ts)

Thank you to the Scribus project to have written a clean way to automatise translation. [See here](https://github.com/scribusproject/scribus/blob/master/resources/translations/CMakeLists.txt). I learnt a lot from it.

## To contact me

cyril.jacquet@skribisto.eu (UTC+1)







