// Mock BinderBinderItemsListModel — flat list with indent-based tree structure

import QtQuick

ListModel {
    property int binderId: 1

    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-01T00:00:00Z"
        dictLanguage: "en_US"
        indent: 0
        isFavorite: false
        isPrintable: true
        itemId: 1
        label: "draft"
        role: "folder"
        subRole: ""
        subTitle: "Setting the stage"
        title: "Part One: The Beginning"
        updatedAt: "2026-01-15T10:30:00Z"
        wordCountGoal: 0
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-02T00:00:00Z"
        dictLanguage: "en_US"
        indent: 1
        isFavorite: true
        isPrintable: true
        itemId: 2
        label: "done"
        role: "folder"
        subRole: "chapter"
        subTitle: "Where it all starts"
        title: "Chapter 1: Dawn"
        updatedAt: "2026-01-16T09:00:00Z"
        wordCountGoal: 5000
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-03T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: false
        isPrintable: true
        itemId: 3
        label: "done"
        role: "item"
        subRole: "scene,text"
        subTitle: ""
        title: "The awakening"
        updatedAt: "2026-01-17T14:00:00Z"
        wordCountGoal: 1500
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-04T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: false
        isPrintable: true
        itemId: 4
        label: "done"
        role: "item"
        subRole: "scene,text"
        subTitle: ""
        title: "First steps"
        updatedAt: "2026-01-18T11:20:00Z"
        wordCountGoal: 2000
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-05T00:00:00Z"
        dictLanguage: "en_US"
        indent: 1
        isFavorite: false
        isPrintable: true
        itemId: 5
        label: "draft"
        role: "folder"
        subRole: "chapter"
        subTitle: "Uncovering the truth"
        title: "Chapter 2: Discovery"
        updatedAt: "2026-02-01T08:45:00Z"
        wordCountGoal: 6000
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-06T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: true
        isPrintable: true
        itemId: 6
        label: "draft"
        role: "item"
        subRole: "scene,text"
        subTitle: "Research begins"
        title: "The old library"
        updatedAt: "2026-02-02T10:00:00Z"
        wordCountGoal: 2000
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-07T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: false
        isPrintable: true
        itemId: 7
        label: "outline"
        role: "item"
        subRole: "text"
        subTitle: ""
        title: "Hidden messages"
        updatedAt: "2026-02-03T15:30:00Z"
        wordCountGoal: 1800
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-08T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: false
        isPrintable: true
        itemId: 8
        label: "outline"
        role: "item"
        subRole: "scene,text"
        subTitle: "The catalyst"
        title: "A stranger arrives"
        updatedAt: "2026-02-04T09:15:00Z"
        wordCountGoal: 2500
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-09T00:00:00Z"
        dictLanguage: "en_US"
        indent: 0
        isFavorite: false
        isPrintable: true
        itemId: 9
        label: "outline"
        role: "folder"
        subRole: ""
        subTitle: "On the road"
        title: "Part Two: The Journey"
        updatedAt: "2026-02-10T12:00:00Z"
        wordCountGoal: 0
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-10T00:00:00Z"
        dictLanguage: "en_US"
        indent: 1
        isFavorite: false
        isPrintable: true
        itemId: 10
        label: "outline"
        role: "folder"
        subRole: "chapter"
        subTitle: "Leaving home"
        title: "Chapter 3: Departure"
        updatedAt: "2026-02-12T16:00:00Z"
        wordCountGoal: 4000
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-11T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: false
        isPrintable: true
        itemId: 11
        label: ""
        role: "item"
        subRole: "scene,text"
        subTitle: ""
        title: "Packing up"
        updatedAt: "2026-02-13T08:00:00Z"
        wordCountGoal: 1200
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-12T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: false
        isPrintable: true
        itemId: 12
        label: ""
        role: "item"
        subRole: "text"
        subTitle: ""
        title: "The road ahead"
        updatedAt: "2026-02-14T10:30:00Z"
        wordCountGoal: 1800
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-13T00:00:00Z"
        dictLanguage: "en_US"
        indent: 1
        isFavorite: false
        isPrintable: true
        itemId: 13
        label: ""
        role: "folder"
        subRole: "chapter"
        subTitle: "Into the unknown"
        title: "Chapter 4: The Forest"
        updatedAt: "2026-02-15T14:45:00Z"
        wordCountGoal: 5500
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-14T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: false
        isPrintable: true
        itemId: 14
        label: ""
        role: "item"
        subRole: "scene,text"
        subTitle: ""
        title: "Into the woods"
        updatedAt: "2026-02-16T11:00:00Z"
        wordCountGoal: 2000
    }
    ListElement {
        activated: false
        charCountGoal: 0
        createdAt: "2026-01-15T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: false
        isPrintable: false
        itemId: 15
        label: ""
        role: "item"
        subRole: "text,note"
        subTitle: "Darkness descends"
        title: "Night falls"
        updatedAt: "2026-02-17T09:30:00Z"
        wordCountGoal: 1500
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-16T00:00:00Z"
        dictLanguage: "en_US"
        indent: 1
        isFavorite: false
        isPrintable: true
        itemId: 16
        label: ""
        role: "folder"
        subRole: "note"
        subTitle: ""
        title: "Notes"
        updatedAt: "2026-02-18T10:00:00Z"
        wordCountGoal: 0
    }
    ListElement {
        activated: true
        charCountGoal: 0
        createdAt: "2026-01-17T00:00:00Z"
        dictLanguage: "en_US"
        indent: 2
        isFavorite: false
        isPrintable: false
        itemId: 17
        label: ""
        role: "item"
        subRole: "note"
        subTitle: ""
        title: "Research notes"
        updatedAt: "2026-02-19T11:00:00Z"
        wordCountGoal: 0
    }
}
