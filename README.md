# Skribisto



**Skribisto** is born from the ashes of **Plume Creator**, keeping the goals while adopting more recent ways to think an application.

Skribisto is geared toward helping anyone write anything, be it a novel or course notes. The user is free to use tags to define texts or 
write hundreds of notes. The tools are designed to be most unobstrusive, so you can write, write and write a bit more without too much distraction.

With Skribisto, the user can create and organize text papers (called "sheets"). Exactly the same is possible with the notes. Sheets can link to a synopsis and multiple notes, 
or create them on the fly while writing.

What Skribisto is not : LibreOffice, Calligra or Word. Any project can be exported to .odt so as to make use of these complete text processors formatting abilities 
before printing.

Accessibility is too often forgotten. I'm trying to keep the interface accessible for screen readers, as much as Qt let me implement it. Please contact me if there 
is a glaring lack in the accessibility.

## Goals

Short term goal is to rejoin its ancestor Plume Creator feature-wise. A few outstanding features are below. Bold means this feature is already implemented

- **navigating between texts**
- **distraction-free mode**
- **rich text (bold/italic/underline/strikeout)**
- **synopsis**
- **label (named 'tag' in Plume) next to each text title**
- **autosave**
- advanced search/replace
- display quickly the end of the previous text and the beginning of the next text
- character/word count
- character/word goal
- overview of all texts
- exporting to .txt/.odt/.PDF
- printing
- spellcheking
- color themes

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
- Windows 10 support
- Examples
- Help page
- importing old Plume Creator projects
- texts overview
- notes overview
- project management
- each note or text can take snapshots
- on-the-fly notes from the context menu

Medium term goals are :
- Adjoining documents to texts (without insertion)
- Insert images into the text
- Gallery tab to manage all external documents/images
- Android support


## For tech people, under the hood

All the application is rewritten from scratch using C++ Qt for backend and QML for the front-end. The QML allows for a touch friendly and dynamic interface.

Each project is a SQLite3 file, more robust than the zipped projects in Plume.



## Test it

### Linux / Windows
Download the latest from GitHub, then you can use Qt Creator to open the top-most CMakeLists.txt at the root of the project. Build it against Qt 5.15 minimum to be sure.

### Linux
Also, thanks to the flatpak support, 
- make sure to have flatpak and flatpak-builder installed on your system
- type in a terminal :

prerequisiste :
```
flatpak install flathub org.kde.Sdk//5.15
```

Compile and install :
```
mkdir Devel
cd ~/Devel
git clone https://github.com/jacquetc/skribisto.git
flatpak uninstall eu.skribisto.skribisto -y
flatpak-builder --user --repo=local-repo build-dir skribisto/eu.skribisto.skribisto.yml --force-clean
flatpak install eu.skribisto.skribisto -y
```

It will download 500 Mo if dependencies the first time.


## To contact me :

cyril.jacquet@skribisto.eu







