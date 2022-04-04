lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Ihr Desktop"),
        ("desk_tip", "Mit dieser ID und diesem Passwort können Sie auf Ihren Desktop zugreifen."),
        ("Password", "Passwort"),
        ("Ready", "Bereit"),
        ("Established", "Etabliert"),
        ("connecting_status", "Verbinden mit dem RustDesk-Netzwerk..."),
        ("Enable Service", "Verbindungsserver einschalten"),
        ("Start Service", "Starte Verbindungsserver"),
        ("Service is running", "Dienst läuft"),
        ("Service is not running", "Der Verbindungsserver läuft nicht"),
        ("not_ready_status", "Nicht bereit. Bitte überprüfen Sie Ihre Verbindung"),
        ("Control Remote Desktop", "Entfernten Desktop steuern"),
        ("Transfer File", "Datei übertragen"),
        ("Connect", "Verbinden"),
        ("Recent Sessions", "Letzte Sitzungen"),
        ("Address Book", "Adressbuch"),
        ("Confirmation", "Bestätigung"),
        ("TCP Tunneling", "TCP Tunneln"),
        ("Remove", "Entfernen"),
        ("Refresh random password", "Zufälliges Passwort aktualisieren"),
        ("Set your own password", "Legen Sie Ihr eigenes Passwort fest"),
        ("Enable Keyboard/Mouse", "Tastatur/Maus einschalten"),
        ("Enable Clipboard", "Zwischenablage einschalten"),
        ("Enable File Transfer", "Dateiübertragung aktivieren"),
        ("Enable TCP Tunneling", "TCP-Tunneling einschalten"),
        ("IP Whitelisting", "IP Freigabeliste"),
        ("ID/Relay Server", "ID/Verbindungsserver"),
        ("Stop service", "Verbindungsserver ausschalten"),
        ("Change ID", "ID wechseln"),
        ("Website", "Webseite"),
        ("About", "Über"),
        ("Mute", "Stummschalten"),
        ("Audio Input", "Audio-Eingang"),
        ("ID Server", "ID Server"),
        ("Relay Server", "Verbindungsserver Server"),
        ("API Server", "API Server"),
        ("invalid_http", "Muss mit http:// oder https:// beginnen"),
        ("Invalid IP", "Ungültige IP-Adresse"),
        ("id_change_tip", "Nur die Zeichen a-z, A-Z, 0-9 und _ (Unterstrich) sind erlaubt. Der erste Buchstabe muss a-z, A-Z sein. Länge zwischen 6 und 16."),
        ("Invalid format", "Ungültiges Format"),
        ("This function is turned off by the server", "Diese Funktion wird vom Server nicht bereitgestellt"),
        ("Not available", "Nicht verfügbar"),
        ("Too frequent", "Zu häufig"),
        ("Cancel", "Abbrechen"),
        ("Skip", "Überspringen"),
        ("Close", "Schließen"),
        ("Retry", "Nochmal versuchen"),
        ("OK", "OK"),
        ("Password Required", "Passwort erforderlich"),
        ("Please enter your password", "Bitte geben Sie Ihr Passwort ein"),
        ("Remember password", "Passwort merken"),
        ("Wrong Password", "Falsches Passwort"),
        ("Do you want to enter again?", "Möchten Sie erneut teilnehmen?"),
        ("Connection Error", "Verbindungsfehler"),
        ("Error", "Fehler"),
        ("Reset by the peer", "Zurücksetzen durch die Gegenstelle"),
        ("Connecting...", "Verbinden..."),
        ("Connection in progress. Please wait.", "Die Verbindung wird hergestellt. Bitte warten Sie."),
        ("Please try 1 minute later", "Bitte versuchen Sie es 1 Minute später"),
        ("Login Error", "Anmeldefehler"),
        ("Successful", "Erfolgreich"),
        ("Connected, waiting for image...", "Verbunden, warten auf Bild..."),
        ("Name", "Name"),
        ("Type", "Typ"),
        ("Modified", "Geändert"),
        ("Size", "Größe"),
        ("Show Hidden Files", "Versteckte Dateien anzeigen"),
        ("Receive", "Empfangen"),
        ("Send", "Senden"),
        ("Refresh File", "Datei aktualisieren"),
        ("Local", "Lokaler"),
        ("Remote", "Entfernter"),
        ("Remote Computer", "Entfernter Computer"),
        ("Local Computer", "Lokaler Computer"),
        ("Confirm Delete", "Löschen bestätigen"),
        ("Delete", "Löschen"),
        ("Properties", "Eigenschaften"),
        ("Multi Select", "Mehrfachauswahl"),
        ("Empty Directory", "Leeres Verzeichnis"),
        ("Not an empty directory", "Kein leeres Verzeichnis"),
        ("Are you sure you want to delete this file?", "Sind Sie sicher, dass Sie diese Datei löschen wollen?"),
        ("Are you sure you want to delete this empty directory?", "Sind Sie sicher, dass Sie dieses leere Verzeichnis löschen möchten?"),
        ("Are you sure you want to delete the file of this directory?", "Sind Sie sicher, dass Sie die Datei dieses Verzeichnisses löschen möchten?"),
        ("Do this for all conflicts", "Dies gilt für alle Konflikte"),
        ("This is irreversible!", "Dies ist irreversibel!"),
        ("Deleting", "Löschen"),
        ("files", "Dateien"),
        ("Waiting", "Warten"),
        ("Finished", "Fertiggestellt"),
        ("Speed", "Geschwindigkeit"),
        ("Custom Image Quality", "Individuelle Bildqualität"),
        ("Privacy mode", "Datenschutz-Modus"),
        ("Block user input", "Benutzereingaben blockieren"),
        ("Unblock user input", "Benutzereingaben freigeben"),
        ("Adjust Window", "Fenster anpassen"),
        ("Original", "Original"),
        ("Shrink", "Geschrumpft"),
        ("Stretch", "Gestreckt"),
        ("Good image quality", "Gute Bildqualität"),
        ("Balanced", "Ausgeglichen"),
        ("Optimize reaction time", "Optimierte Reaktionszeit"),
        ("Custom", "Benutzerdefiniert"),
        ("Show remote cursor", "Ferngesteuerten Cursor anzeigen"),
        ("Disable clipboard", "Zwischenablage deaktivieren"),
        ("Lock after session end", "Sperren nach Sitzungsende"),
        ("Insert", "Einfügen"),
        ("Insert Lock", "Sperre einfügen"),
        ("Refresh", "Aktualisieren"),
        ("ID does not exist", "Die ID existiert nicht"),
        ("Failed to connect to rendezvous server", "Verbindung zum Verbindungsserver fehlgeschlagen"),
        ("Please try later", "Bitte versuchen Sie es später"),
        ("Remote desktop is offline", "Entfernter Desktop ist offline"),
        ("Key mismatch", "Schlüssel nicht übereinstimmend"),
        ("Timeout", "Zeitüberschreitung"),
        ("Failed to connect to relay server", "Verbindung zum Verbindungsserver fehlgeschlagen"),
        ("Failed to connect via rendezvous server", "Verbindung über rendezvous server fehlgeschlagen"),
        ("Failed to connect via relay server", "Verbindung über den Verbindungsserver ist fehlgeschlagen"),
        ("Failed to make direct connection to remote desktop", "Direkte Verbindung zum Entfernten-Desktop konnte nicht hergestellt werden"),
        ("Set Password", "Passwort festlegen"),
        ("OS Password", "Betriebssystem-Passwort"),
        ("install_tip", "Aufgrund der UAC kann RustDesk in manchen Fällen nicht ordnungsgemäß auf der Gegenseite funktionieren. Um UAC zu vermeiden, klicken Sie bitte auf die Schaltfläche unten, um RustDesk auf dem System zu installieren"),
        ("Click to upgrade", "Zum Upgrade anklicken"),
        ("Click to download", "Zum Herunterladen klicken"),
        ("Click to update", "Zum Aktualisieren klicken"),
        ("Configuration Permissions", "Konfigurationsberechtigungen"),
        ("Configure", "Konfigurieren"),
        ("config_acc", "Um Ihren Desktop aus der Ferne zu steuern, müssen Sie RustDesk \"Zugangs\" Rechte erteilen."),
        ("config_screen", "Um aus der Ferne auf Ihren Desktop zugreifen zu können, müssen Sie RustDesk \"Bildschirm-Aufnahme\" Berechtigungen erteilen."),
        ("Installing ...", "Installiere ..."),
        ("Install", "Installieren"),
        ("Installation", "Einrichtung"),
        ("Installation Path", "Einrichtungs Pfad"),
        ("Create start menu shortcuts", "Startmenü Verknüpfungen erstellen"),
        ("Create desktop icon", "Desktop Symbol erstellen"),
        ("agreement_tip", "Wenn Sie die Einrichtung starten, akzeptieren Sie die Lizenzvereinbarung"),
        ("Accept and Install", "Akzeptieren und installieren"),
        ("End-user license agreement", "Lizenzvereinbarung für Endbenutzer"),
        ("Generating ...", "Generierung ..."),
        ("Your installation is lower version.", "Ihre Installation ist eine niedrigere Version."),
        ("not_close_tcp_tip", "Schließen Sie dieses Fenster nicht, während Sie den Tunnel benutzen."),
        ("Listening ...", "Hören ..."),
        ("Remote Host", "Entfernter Rechner"),
        ("Remote Port", "Entfernter Port"),
        ("Action", "Aktion"),
        ("Add", "Hinzufügen"),
        ("Local Port", "Lokaler Port"),
        ("setup_server_tip", "Für eine schnellere Verbindung, richten Sie bitte Ihren eigenen Verbindungsserver ein"),
        ("Too short, at least 6 characters.", "Zu kurz, mindestens 6 Zeichen."),
        ("The confirmation is not identical.", "Die Bestätigung ist nicht identisch."),
        ("Permissions", "Berechtigungen"),
        ("Accept", "Akzeptieren"),
        ("Dismiss", "Ablehnen"),
        ("Disconnect", "Verbindung trennen"),
        ("Allow using keyboard and mouse", "Erlaubt die Verwendung von Tastatur und Maus"),
        ("Allow using clipboard", "Verwendung der Zwischenablage zulassen"),
        ("Allow hearing sound", "Erlaubt das Hören von Sound"),
        ("Allow file transfer", "Dateiübertragung zulassen"),
        ("File transfer", "Datei Übertragung"),
        ("Connected", "Verbunden"),
        ("Direct and encrypted connection", "Direkte und verschlüsselte Verbindung"),
        ("Relayed and encrypted connection", "Vermittelte und verschlüsselte Verbindung"),
        ("Direct and unencrypted connection", "Direkte und unverschlüsselte Verbindung"),
        ("Relayed and unencrypted connection", "Vermittelte und unverschlüsselte Verbindung"),
        ("Enter Remote ID", "Remote-ID eingeben"),
        ("Enter your password", "Geben Sie Ihr Passwort ein"),
        ("Logging in...", "Anmeldung..."),
        ("Enable RDP session sharing", "RDP-Sitzungsfreigabe aktivieren"),
        ("Auto Login", "Automatisches Login (nur gültig, wenn Sie \"Sperren nach Sitzungsende\" eingestellt haben)"),
        ("Enable Direct IP Access", "Direkten IP-Zugang aktivieren"),
        ("Rename", "Umbenennen"),
        ("Space", "Platz"),
        ("Create Desktop Shortcut", "Desktop-Verknüpfung erstellen"),
        ("Change Path", "Pfad ändern"),
        ("Create Folder", "Ordner erstellen"),
        ("Please enter the folder name", "Bitte geben Sie den Ordnernamen ein"),
        ("Fix it", "Reparieren"),
        ("Warning", "Warnung"),
        ("Login screen using Wayland is not supported", "Anmeldebildschirm mit Wayland wird nicht unterstützt"),
        ("Reboot required", "Neustart erforderlich"),
        ("Unsupported display server ", "Nicht unterstützter Display-Server"),
        ("x11 expected", "X11 erwartet"),
        ("Port", "Port"),
        ("Settings", "Einstellungen"),
        ("Username", " Benutzername"),
        ("Invalid port", "Ungültiger Port"),
        ("Closed manually by the peer", "Vom Peer manuell geschlossen"),
        ("Enable remote configuration modification", "Änderung der Fernkonfiguration zulassen"),
        ("Run without install", "Ohne Installation ausführen"),
        ("Always connected via relay", "Immer über Verbindungsserver verbunden"),
        ("Always connect via relay", "Verbindung immer über Verbindungsserver"),
        ("whitelist_tip", "Nur IPs auf der Freigabeliste können auf mich zugreifen"),
        ("Login", "Anmeldung"),
        ("Logout", "Abmeldung"),
        ("Tags", "Stichworte"),
        ("Search ID", "Suche ID"),
        ("Current Wayland display server is not supported", "Der aktuelle Wayland-Anzeigeserver wird nicht unterstützt"),
        ("whitelist_sep", "Getrennt durch Komma, Semikolon, Leerzeichen oder Zeilenumbruch"),
        ("Add ID", "ID hinzufügen"),
        ("Add Tag", "Stichwort hinzufügen"),
        ("Unselect all tags", "Alle Stichworte abwählen"),
        ("Network error", "Netzwerkfehler"),
        ("Username missed", "Benutzername fehlt"),
        ("Password missed", "Passwort vergessen"),
        ("Wrong credentials", "Falsche Anmeldedaten"),
        ("Edit Tag", "Stichwort bearbeiten"),
        ("Unremember Password", "Passwort nicht merken"),
        ("Favorites", "Favoriten"),
        ("Add to Favorites", "Zu Favoriten hinzufügen"),
        ("Remove from Favorites", "Entferne von Favoriten"),
        ("Empty", "Leer"),
        ("Invalid folder name", "Ungültiger Ordnername"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Rechnername"),
        ("Discovered", "Gefunden"),
        ("install_daemon_tip", "Um beim Booten zu starten, müssen Sie den Systemdienst installieren"),
        ("Remote ID", "Entfernte ID"),
        ("Paste", "Einfügen"),
        ("Paste here?", "Hier einfügen?"),
        ("Are you sure to close the connection?", "Sind Sie sicher, dass Sie die Verbindung schließen wollen?"),
        ("Download new version", "Neue Version herunterladen"),
        ("Touch mode", "Berührungsmodus"),
        ("TouchPad mode", "TouchPad-Modus"),
        ("One-Finger Tap", "Ein Fingertipp"),
        ("Left Mouse", "Linke Maus"),
        ("One-Long Tap", "Tippen Sie mit einem Finger lang"),
        ("Two-Finger Tap", "Zwei Finger tippen"),
        ("Right Mouse", "Rechte Maus"),
        ("One-Finger Move", "Eine Fingerbewegung"),
        ("Double Tap & Move", "Doppeltippen und verschieben"),
        ("Mouse Drag", "Maus ziehen"),
        ("Two-Finger vertically", "Zwei Finger vertikal"),
        ("Mouse Wheel", "Mausrad"),
        ("Two-Finger Move", "Zwei Finger Bewegung"),
        ("Canvas Move", "Leinwand bewegen"),
        ("Pinch to Zoom", "Zum Zoomen kneifen"),
        ("Canvas Zoom", "Leinwand Zoom"),
        ("Reset canvas", "Anzeige zurücksetzen"),
        ("No permission of file transfer", "Keine Erlaubnis zur Dateiübertragung"),
        ("Note", "Notiz"),
        ("Connection", "Verbindung"),
        ("Share Screen", "Bildschirm freigeben"),
        ("CLOSE", "NAH DRAN"),
        ("OPEN", "OFFEN"),
        ("Chat", "Plaudern"),
        ("Total", "Gesamt"),
        ("items", "Artikel"),
        ("Selected", "Ausgewählt"),
        ("Screen Capture", "Bildschirmaufnahme"),
        ("Mouse Control", "Maussteuerung"),
        ("File Transfer", "Datei Übertragung"),
        ("Audio Capture", "Audioaufnahme"),
        ("File Connection", "Dateiverbindung"),
        ("Screen Connection", "Bildschirmanschluss"),
        ("Do you accept?", "Akzeptieren Sie?"),
        ("Open System Setting", "Systemeinstellung öffnen"),
        ("How to get Android input permission?", "Wie erhalte ich eine Android-Eingabeberechtigung?"),
        ("android_input_permission_tip1", "Nach Erhalt der Eingabeerlaubnis kann das entfernte Gerät dieses Android-Gerät per Maus steuern"),
        ("android_input_permission_tip2", "Bitte gehen Sie zur nächsten Systemeinstellungsseite, suchen und geben Sie [Installierte Dienste] ein, schalten Sie den Dienst [RustDesk Input] ein"),
        ("android_new_connection_tip", "Es wurde eine neue Steuerungsanforderung empfangen, die Ihr aktuelles Gerät steuern möchte."),
        ("android_service_will_start_tip", "Durch das Einschalten der Bildschirmaufnahme wird der Dienst automatisch gestartet, sodass andere Geräte eine Verbindung von diesem Gerät anfordern können."),
        ("android_stop_service_tip", "Durch das Schließen des Dienstes werden automatisch alle hergestellten Verbindungen geschlossen."),
        ("android_version_audio_tip", "Die aktuelle Android-Version unterstützt keine Audioaufnahme, bitte aktualisieren Sie auf Android 10 oder höher."),
        ("android_start_service_tip", "Tippen Sie auf [Dienst starten] oder ÖFFNEN Sie die Berechtigung [Bildschirmaufnahme], um den Bildschirmfreigabedienst zu starten."),
    ].iter().cloned().collect();
}
