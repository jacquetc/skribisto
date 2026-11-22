# SPDX-License-Identifier: GPL-3.0-only
# SPDX-FileCopyrightText: 2026 Cyril Jacquet

# Zdroje: soubory a webové odkazy připojené k položkám pořadače.

## Panel

resources-title = Zdroje
shortcut-name-resources-show = Zobrazit zdroje
resources-add-files = Přidat soubory
resources-add-files-title = Přidat soubory k této položce
resources-add-link = Přidat webový odkaz
resources-drop-hint = Přetáhněte sem soubory nebo webové adresy a připojte je
resources-no-item = Otevřete položku, abyste viděli její zdroje.
resources-empty = K této položce zatím není nic připojeno. Přidejte soubory nebo webové odkazy tlačítky nahoře, nebo je sem přetáhněte.
resources-actions = Akce
resources-not-found = Nenalezeno
resources-row-linked = Propojený soubor
resources-row-linked-size = Propojený soubor, { $size }
resources-row-name = { $title }, { $kind }, { $detail }
resources-row-name-missing = { $title }, { $kind }, { $detail }, nenalezeno

## Druhy souborů, jak je ohlásí čtečka obrazovky

resources-kind-picture = obrázek
resources-kind-pdf = dokument PDF
resources-kind-audio = zvukový záznam
resources-kind-video = video
resources-kind-document = dokument
resources-kind-ebook = e-kniha
resources-kind-archive = archiv
resources-kind-file = soubor
resources-kind-web-link = webový odkaz

## Nabídka řádku

resources-open = Otevřít
resources-show-in-folder = Zobrazit ve složce
resources-locate = Vyhledat…
resources-insert-into-text = Vložit do textu
resources-insert-picture = Vložit obrázek do textu
resources-insert-link = Vložit odkaz do textu
resources-edit = Přejmenovat a popsat…
resources-detach = Odpojit od této položky
resources-remove = Odebrat z projektu…

## Dvě dialogová okna

resources-link-dialog-title = Přidat webový odkaz
resources-link-dialog-address = Adresa
resources-link-dialog-address-placeholder = https://
resources-link-dialog-name = Název
resources-link-dialog-name-placeholder = Název webu, pokud pole necháte prázdné
resources-link-dialog-add = Přidat
resources-edit-dialog-title = Přejmenovat a popsat
resources-edit-dialog-name = Název
resources-edit-dialog-description = Popis
resources-edit-dialog-description-placeholder = Co to je a proč jste si to ponechali
resources-edit-dialog-save = Uložit
resources-dialog-cancel = Zrušit

## Přidávání

resources-large-title = Velký soubor
resources-large-text = „{ $name }“ má { $size }. Uložený v projektu cestuje s každou kopií a každou zálohou projektu. Propojený zůstane tam, kde je, a projekt si pamatuje jen to, kde ho najde.
resources-large-text-many = „{ $name }“ má { $size } a { $others ->
    [one] ještě jeden další soubor je také velký
    [few] ještě { $others } další soubory jsou také velké
    [many] ještě { $others } dalšího souboru je také velkého
   *[other] ještě { $others } dalších souborů je také velkých
}. Uložené v projektu cestují s každou kopií a každou zálohou projektu. Propojené zůstanou tam, kde jsou, a projekt si pamatuje jen to, kde je najde.
resources-large-store = Uložit do projektu
resources-large-link = Raději propojit
resources-large-remember = Už se neptat
resources-storing-title = Přidávání souborů do projektu
resources-store-cancel = Zrušit
resources-store-cancelled = Nebyly přidány žádné soubory.
resources-store-busy = Soubory se stále přidávají. Zkuste to znovu, až budou přidány.
resources-store-failed = Soubory se nepodařilo přidat: { $error }
resources-no-project = Před připojením souborů otevřete projekt.
resources-moved-away = Projekt byl přesunut během kopírování souboru.
resources-some-failed = { $count ->
    [one] Jeden soubor se nepodařilo přidat: { $names }
    [few] { $count } soubory se nepodařilo přidat: { $names }
    [many] { $count } souboru se nepodařilo přidat: { $names }
   *[other] { $count } souborů se nepodařilo přidat: { $names }
}
resources-add-failed = Zdroje se nepodařilo přidat: { $error }
resources-added = { $count ->
    [one] Přidán jeden zdroj
    [few] Přidány { $count } zdroje
    [many] Přidáno { $count } zdroje
   *[other] Přidáno { $count } zdrojů
}
resources-dropped-added = { $count ->
    [one] Přidáno jako zdroj této položky a propojeno v textu
    [few] { $count } soubory přidány jako zdroje této položky a propojeny v textu
    [many] { $count } souboru přidáno jako zdroje této položky a propojeno v textu
   *[other] { $count } souborů přidáno jako zdroje této položky a propojeno v textu
}
resources-dropped-undo-kept = Text se od přetažení změnil, takže nic nebylo vráceno zpět.
resources-undo = Vrátit zpět
resources-link-refused = „{ $url }“ není webová adresa, kterou si Skribisto může uchovat. Použijte adresu, která začíná http:// nebo https://, bez uživatelského jména a hesla.

## Upozornění pro projekt v jediném souboru

resources-folder-title = Tento projekt se zvětšuje
resources-folder-text = Jeho uložené soubory teď zabírají { $size }. Projekt uložený jako jediný soubor se při každém uložení zapisuje celý znovu, včetně svých souborů. Uložený jako složka ukládá jen to, co se změnilo. Můžete ho teď uložit jako složku a odteď pracovat ve složce.
resources-folder-switch = Uložit jako složku…
resources-folder-not-now = Teď ne
resources-folder-never = U tohoto projektu se už neptat

## Otevírání

resources-not-found-toast = „{ $name }“ není tam, kde ho projekt očekává.
resources-unsafe-title = Otevřít „{ $name }“?
resources-unsafe-text = Skribisto neví, zda je tento druh souboru bezpečný. Jeho otevření by mohlo spustit program ve vašem počítači a projekt může pocházet od někoho jiného. Nejdřív ho zobrazte v jeho složce, abyste viděli, co to je.
resources-open-anyway = Přesto otevřít
resources-open-failed = Soubor se nepodařilo otevřít: { $error }
resources-no-data-folder = tento systém neposkytuje Skribistu žádnou datovou složku, ve které by mohl otevřít kopii
resources-link-gone = Tento odkaz vede na zdroj, který už projekt nemá.
resources-locate-title = Kde je teď „{ $name }“?
resources-relinked-more = { $count ->
    [one] Ve stejné složce se našel ještě jeden chybějící soubor a byl znovu propojen.
    [few] Ve stejné složce se našly ještě { $count } chybějící soubory a byly znovu propojeny.
    [many] Ve stejné složce se našlo ještě { $count } chybějícího souboru a bylo znovu propojeno.
   *[other] Ve stejné složce se našlo ještě { $count } chybějících souborů a bylo znovu propojeno.
}
resources-changed-elsewhere = Soubor „{ $name }“ byl změněn v jiné aplikaci. Projekt si ponechá novou verzi.

## Úpravy

resources-edit-failed = Zdroj se nepodařilo změnit: { $error }
resources-remove-title = Odebrat „{ $title }“ z projektu?
resources-remove-text = Odpojí se od každé položky, ke které je připojen. Soubor uložený v projektu odejde s ním; propojený soubor a webová stránka zůstanou tam, kde jsou. Příkaz Vrátit zpět ho obnoví.
resources-remove-confirm = Odebrat
resources-remove-failed = Zdroj se nepodařilo odebrat: { $error }
resources-removed-toast = „{ $title }“ byl odebrán z projektu.
resources-removed-many-toast = { $count ->
    [few] { $count } zdroje byly odebrány z projektu.
   *[other] { $count } zdrojů bylo odebráno z projektu.
}
resources-detached-toast = „{ $title }“ byl odpojen.
resources-attached-one-toast = „{ $title }“ byl přidán ke zdrojům „{ $item }“.
resources-attached-many-toast = { $count ->
    [few] { $count } zdroje byly přidány k „{ $item }“.
   *[other] { $count } zdrojů bylo přidáno k „{ $item }“.
}

## Do textu

resources-no-text = Nejprve umístěte kurzor do textu, potom tam zdroj vložte.
resources-drop-insert-picture = Vložit obrázek
resources-drop-insert-link = Vložit odkaz na zdroj

## Průběh

progress-resources-storing = Kopírování souborů
progress-resources-stored = Soubory zkopírovány

## Stránka Zdroje u složky

segment-folder = Složka
resources-folder-count = { $count ->
    [one] Jeden zdroj
    [few] { $count } zdroje
    [many] { $count } zdroje
   *[other] { $count } zdrojů
}
resources-view-label = Zobrazit zdroje jako
resources-view-list = Seznam
resources-view-grid = Mřížka
resources-grid-label = Zdroje
resources-folder-drop-hint = Přetáhněte sem soubory nebo webové adresy a připojte je k této složce
resources-folder-empty = K této složce ani k ničemu v ní zatím není nic připojeno. Přidejte soubory nebo webové odkazy tlačítky nahoře, nebo je sem přetáhněte: připojí se ke složce.
resources-attached-to = U
resources-attached-separator = ,
resources-attached-more = { $count ->
    [one] a jedné další
    [few] a { $count } dalších
    [many] a { $count } dalších
   *[other] a { $count } dalších
}
resources-row-name-attached = { $name }, u { $items }
resources-open-item = Otevřít „{ $title }“
resources-detach-from = Odpojit od „{ $title }“

## Obrázkové zdroje v Galerii

gallery-resource-badge = Zdroj
gallery-tile-resource = Obrázkový zdroj položky { $title }: { $description }

## Stránka Zdroje v nastavení projektu

settings-page-resources = Zdroje
settings-desc-resources = Soubory a webové odkazy, které už nenese žádná položka: znovu je připojte, odeberte je nebo vyexportujte kopie
settings-resources-description = Zdroj není připojen k ničemu, jakmile ho odpojíte od jeho poslední položky, nebo jakmile je položka, u které byl, nadobro smazána. Zůstává v projektu a uložený soubor putuje s každou kopií a každou zálohou projektu, dokud ho znovu nepřipojíte nebo neodeberete.
settings-resources-group-storage = Úložiště
settings-resources-group-unattached = Nepřipojené k ničemu
settings-resources-stored = Soubory uložené v tomto projektu: { $size }
settings-resources-folder-advice = Projekt uložený jako jediný soubor se při každém uložení zapisuje celý znovu, i s uloženými soubory. Uložený jako složka zapisuje jen to, co se změnilo, a to je rychlejší.
settings-resources-count = { $count ->
    [one] Jeden zdroj
    [few] { $count } zdroje
    [many] { $count } zdroje
   *[other] { $count } zdrojů
}
settings-resources-empty = Každý zdroj tohoto projektu je připojen k nějaké položce.
resources-attach-to-items = Připojit k položkám…
resources-export-copies = Exportovat kopie…
resources-export-copy = Exportovat kopii…
resources-remove-many-title = Odebrat z projektu zdroje ({ $count })?
resources-attach-title = Připojit k položkám
resources-attach-hint = Vyberte položky, ke kterým se má připojit. Ctrl+klepnutí nebo Shift+klepnutí vybere několik.
resources-attach-empty = Tento projekt zatím nemá žádné položky.
resources-attach-confirm = Připojit
resources-attached = { $count ->
    [one] Jeden zdroj připojen
    [few] { $count } zdroje připojeny
    [many] { $count } zdroje připojeno
   *[other] { $count } zdrojů připojeno
} k { $items ->
    [one] jedné položce
    [few] { $items } položkám
    [many] { $items } položky
   *[other] { $items } položkám
}
resources-export-title = Exportovat kopie do složky
resources-exported = { $count ->
    [one] Exportován jeden zdroj
    [few] Exportovány { $count } zdroje
    [many] Exportováno { $count } zdroje
   *[other] Exportováno { $count } zdrojů
}
resources-export-missing = { $count ->
    [one] Jeden soubor není tam, kde ho projekt čeká, a byl vynechán: { $names }
    [few] { $count } soubory nejsou tam, kde je projekt čeká, a byly vynechány: { $names }
    [many] { $count } souboru není tam, kde je projekt čeká, a bylo vynecháno: { $names }
   *[other] { $count } souborů není tam, kde je projekt čeká, a bylo vynecháno: { $names }
}
resources-export-failed = { $count ->
    [one] Jeden soubor nešlo zkopírovat: { $names }
    [few] { $count } soubory nešlo zkopírovat: { $names }
    [many] { $count } souboru nešlo zkopírovat: { $names }
   *[other] { $count } souborů nešlo zkopírovat: { $names }
}

## Předvolba pro velké soubory (Chování editoru)

settings-resources-large-file = Soubory nad 10 MB
settings-resources-large-ask = Pokaždé se zeptat
settings-resources-large-store = Vždy uložit do projektu
settings-resources-large-link = Vždy propojit se souborem
settings-hint-resources-large-file = Co se stane se souborem nad 10 MB, který připojíte jako zdroj. Uložený v projektu putuje s každou kopií a každou zálohou projektu. Propojený zůstane, kde je, a projekt si pamatuje jen, kde ho najít.
