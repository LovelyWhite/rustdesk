lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stato"),
        ("Your Desktop", "Il tuo desktop"),
        ("desk_tip", "Puoi accedere al tuo desktop usando l'ID e la password riportati qui."),
        ("Password", "Password"),
        ("Ready", "Pronto"),
        ("Established", "Stabilito"),
        ("connecting_status", "Connessione alla rete RustDesk in corso..."),
        ("Enable Service", "Abilita servizio"),
        ("Start Service", "Avvia servizio"),
        ("Service is running", "Il servizio è in esecuzione"),
        ("Service is not running", "Il servizio non è in esecuzione"),
        ("not_ready_status", "Non pronto. Verifica la tua connessione"),
        ("Control Remote Desktop", "Controlla un desktop remoto"),
        ("Transfer File", "Trasferisci file"),
        ("Connect", "Connetti"),
        ("Recent Sessions", "Sessioni recenti"),
        ("Address Book", "Rubrica"),
        ("Confirmation", "Conferma"),
        ("TCP Tunneling", "Tunnel TCP"),
        ("Remove", "Rimuovi"),
        ("Refresh random password", "Nuova password casuale"),
        ("Set your own password", "Imposta la tua password"),
        ("Enable Keyboard/Mouse", "Abilita tastiera/mouse"),
        ("Enable Clipboard", "Abilita appunti"),
        ("Enable File Transfer", "Abilita trasferimento file"),
        ("Enable TCP Tunneling", "Abilita tunnel TCP"),
        ("IP Whitelisting", "IP autorizzati"),
        ("ID/Relay Server", "Server ID/Relay"),
        ("Import Server Config", "Importa configurazione Server"),
        ("Export Server Config", "Esporta configurazione Server"),
        ("Import server configuration successfully", "Configurazione Server importata con successo"),
        ("Export server configuration successfully", "Configurazione Server esportata con successo"),
        ("Invalid server configuration", "Configurazione Server non valida"),
        ("Clipboard is empty", "Gli appunti sono vuoti"),
        ("Stop service", "Arresta servizio"),
        ("Change ID", "Cambia ID"),
        ("Your new ID", "Il tuo nuovo ID"),
        ("length %min% to %max%", "da lunghezza %min% a %max%"),
        ("starts with a letter", "inizia con una lettera"),
        ("allowed characters", "caratteri consentiti"),
        ("id_change_tip", "Puoi usare solo i caratteri a-z, A-Z, 0-9 e _ (underscore). Il primo carattere deve essere a-z o A-Z. La lunghezza deve essere fra 6 e 16 caratteri."),
        ("Website", "Sito web"),
        ("About", "Informazioni"),
        ("Slogan_tip", "Fatta con il cuore in questo mondo caotico!"),
        ("Privacy Statement", "Informativa sulla privacy"),
        ("Mute", "Silenzia"),
        ("Build Date", "Data della build"),
        ("Version", "Versione"),
        ("Home", "Home"),
        ("Audio Input", "Input audio"),
        ("Enhancements", "Miglioramenti"),
        ("Hardware Codec", "Codifica Hardware"),
        ("Adaptive Bitrate", "Bitrate Adattivo"),
        ("ID Server", "ID server"),
        ("Relay Server", "Server relay"),
        ("API Server", "Server API"),
        ("invalid_http", "deve iniziare con http:// o https://"),
        ("Invalid IP", "Indirizzo IP non valido"),
        ("Invalid format", "Formato non valido"),
        ("server_not_support", "Non ancora supportato dal server"),
        ("Not available", "Non disponibile"),
        ("Too frequent", "Troppo frequente"),
        ("Cancel", "Annulla"),
        ("Skip", "Ignora"),
        ("Close", "Chiudi"),
        ("Retry", "Riprova"),
        ("OK", "OK"),
        ("Password Required", "Password Richiesta"),
        ("Please enter your password", "Inserisci la tua password"),
        ("Remember password", "Ricorda password"),
        ("Wrong Password", "Password Errata"),
        ("Do you want to enter again?", "Vuoi riprovare?"),
        ("Connection Error", "Errore di connessione"),
        ("Error", "Errore"),
        ("Reset by the peer", "Reimpostata dal peer"),
        ("Connecting...", "Connessione..."),
        ("Connection in progress. Please wait.", "Connessione in corso. Attendi."),
        ("Please try 1 minute later", "Per favore riprova fra 1 minuto"),
        ("Login Error", "Errore Login"),
        ("Successful", "Successo"),
        ("Connected, waiting for image...", "Connesso, in attesa dell'immagine..."),
        ("Name", "Nome"),
        ("Type", "Tipo"),
        ("Modified", "Modificato"),
        ("Size", "Dimensione"),
        ("Show Hidden Files", "Mostra file nascosti"),
        ("Receive", "Ricevi"),
        ("Send", "Invia"),
        ("Refresh File", "Aggiorna file"),
        ("Local", "Locale"),
        ("Remote", "Remote"),
        ("Remote Computer", "Computer remoto"),
        ("Local Computer", "Computer locale"),
        ("Confirm Delete", "Conferma cancellazione"),
        ("Delete", "Eliminare"),
        ("Properties", "Proprietà"),
        ("Multi Select", "Selezione multipla"),
        ("Select All", "Seleziona tutto"),
        ("Unselect All", "Deseleziona tutto"),
        ("Empty Directory", "Directory vuota"),
        ("Not an empty directory", "Non una directory vuota"),
        ("Are you sure you want to delete this file?", "Vuoi davvero eliminare questo file?"),
        ("Are you sure you want to delete this empty directory?", "Sei sicuro di voler eliminare questa directory vuota?"),
        ("Are you sure you want to delete the file of this directory?", "Sei sicuro di voler eliminare il file di questa directory?"),
        ("Do this for all conflicts", "Ricorca questa scelta per tutti i conflitti"),
        ("This is irreversible!", "Questo è irreversibile!"),
        ("Deleting", "Cancellazione di"),
        ("files", "file"),
        ("Waiting", "In attesa"),
        ("Finished", "Terminato"),
        ("Speed", "Velocità"),
        ("Custom Image Quality", "Qualità immagine personalizzata"),
        ("Privacy mode", "Modalità privacy"),
        ("Block user input", "Blocca l'input dell'utente"),
        ("Unblock user input", "Sbloccare l'input dell'utente"),
        ("Adjust Window", "Adatta la finestra"),
        ("Original", "Originale"),
        ("Shrink", "Restringi"),
        ("Stretch", "Allarga"),
        ("Scrollbar", "Barra di scorrimento"),
        ("ScrollAuto", "Scorri automaticamente"),
        ("Good image quality", "Qualità immagine migliore"),
        ("Balanced", "Bilanciato"),
        ("Optimize reaction time", "Ottimizza il tempo di reazione"),
        ("Custom", "Personalizzato"),
        ("Show remote cursor", "Mostra il cursore remoto"),
        ("Show quality monitor", "Visualizza qualità video"),
        ("Disable clipboard", "Disabilita appunti"),
        ("Lock after session end", "Blocca al termine della sessione"),
        ("Insert", "Inserisci"),
        ("Insert Lock", "Blocco inserimento"),
        ("Refresh", "Aggiorna"),
        ("ID does not exist", "L'ID non esiste"),
        ("Failed to connect to rendezvous server", "Errore di connessione al server rendezvous"),
        ("Please try later", "Riprova più tardi"),
        ("Remote desktop is offline", "Il desktop remoto è offline"),
        ("Key mismatch", "La chiave non corrisponde"),
        ("Timeout", "Timeout"),
        ("Failed to connect to relay server", "Errore di connessione al server relay"),
        ("Failed to connect via rendezvous server", "Errore di connessione tramite il server rendezvous"),
        ("Failed to connect via relay server", "Errore di connessione tramite il server relay"),
        ("Failed to make direct connection to remote desktop", "Impossibile connettersi direttamente al desktop remoto"),
        ("Set Password", "Imposta password"),
        ("OS Password", "Password del sistema operativo"),
        ("install_tip", "A causa del Controllo Account Utente, RustDesk potrebbe non funzionare correttamente come desktop remoto. Per evitare questo problema, fai click sul tasto qui sotto per installare RustDesk a livello di sistema."),
        ("Click to upgrade", "Fai click per aggiornare"),
        ("Click to download", "Cliquez per scaricare"),
        ("Click to update", "Fare clic per aggiornare"),
        ("Configure", "Configura"),
        ("config_acc", "Per controllare il tuo desktop dall'esterno, devi fornire a RustDesk il permesso \"Accessibilità\"."),
        ("config_screen", "Per controllare il tuo desktop dall'esterno, devi fornire a RustDesk il permesso \"Registrazione schermo\"."),
        ("Installing ...", "Installazione ..."),
        ("Install", "Installa"),
        ("Installation", "Installazione"),
        ("Installation Path", "Percorso di installazione"),
        ("Create start menu shortcuts", "Crea i collegamenti nel menu di avvio"),
        ("Create desktop icon", "Crea un'icona sul desktop"),
        ("agreement_tip", "Avviando l'installazione, accetti i termini del contratto di licenza."),
        ("Accept and Install", "Accetta e installa"),
        ("End-user license agreement", "Contratto di licenza con l'utente finale"),
        ("Generating ...", "Generazione ..."),
        ("Your installation is lower version.", "La tua installazione non è aggiornata."),
        ("not_close_tcp_tip", "Non chiudere questa finestra mentre stai usando il tunnel"),
        ("Listening ...", "In ascolto ..."),
        ("Remote Host", "Host remoto"),
        ("Remote Port", "Porta remota"),
        ("Action", "Azione"),
        ("Add", "Aggiungi"),
        ("Local Port", "Porta locale"),
        ("Local Address", "Indirizzo locale"),
        ("Change Local Port", "Cambia porta locale"),
        ("setup_server_tip", "Per una connessione più veloce, configura un tuo server"),
        ("Too short, at least 6 characters.", "Troppo breve, almeno 6 caratteri"),
        ("The confirmation is not identical.", "La conferma non corrisponde"),
        ("Permissions", "Permessi"),
        ("Accept", "Accetta"),
        ("Dismiss", "Rifiuta"),
        ("Disconnect", "Disconnetti"),
        ("Allow using keyboard and mouse", "Consenti l'uso di tastiera e mouse"),
        ("Allow using clipboard", "Consenti l'uso degli appunti"),
        ("Allow hearing sound", "Consenti la riproduzione dell'audio"),
        ("Allow file copy and paste", "Consenti copia e incolla di file"),
        ("Connected", "Connesso"),
        ("Direct and encrypted connection", "Connessione diretta e cifrata"),
        ("Relayed and encrypted connection", "Connessione tramite relay e cifrata"),
        ("Direct and unencrypted connection", "Connessione diretta e non cifrata"),
        ("Relayed and unencrypted connection", "Connessione tramite relay e non cifrata"),
        ("Enter Remote ID", "Inserisci l'ID remoto"),
        ("Enter your password", "Inserisci la tua password"),
        ("Logging in...", "Autenticazione..."),
        ("Enable RDP session sharing", "Abilita la condivisione della sessione RDP"),
        ("Auto Login", "Accesso automatico"),
        ("Enable Direct IP Access", "Abilita l'accesso diretto tramite IP"),
        ("Rename", "Rinomina"),
        ("Space", "Spazio"),
        ("Create Desktop Shortcut", "Crea collegamento sul desktop"),
        ("Change Path", "Cambia percorso"),
        ("Create Folder", "Crea cartella"),
        ("Please enter the folder name", "Inserisci il nome della cartella"),
        ("Fix it", "Risolvi"),
        ("Warning", "Avviso"),
        ("Login screen using Wayland is not supported", "La schermata di accesso non è supportata utilizzando Wayland"),
        ("Reboot required", "Riavvio necessario"),
        ("Unsupported display server", "Display server non supportato"),
        ("x11 expected", "x11 necessario"),
        ("Port", "Porta"),
        ("Settings", "Impostazioni"),
        ("Username", " Nome utente"),
        ("Invalid port", "Numero di porta non valido"),
        ("Closed manually by the peer", "Chiuso manualmente dal peer"),
        ("Enable remote configuration modification", "Abilita la modifica remota della configurazione"),
        ("Run without install", "Esegui senza installare"),
        ("Connect via relay", "Collegati tramite relay"),
        ("Always connect via relay", "Collegati sempre tramite relay"),
        ("whitelist_tip", "Solo gli indirizzi IP autorizzati possono connettersi a questo desktop"),
        ("Login", "Accedi"),
        ("Verify", "Verifica"),
        ("Remember me", "Ricordami"),
        ("Trust this device", "Registra questo dispositivo come attendibile"),
        ("Verification code", "Codice di verifica"),
        ("verification_tip", "È stato rilevato un nuovo dispositivo e un codice di verifica è stato inviato all'indirizzo e-mail registrato; inserire il codice di verifica per continuare l'accesso."),
        ("Logout", "Esci"),
        ("Tags", "Tag"),
        ("Search ID", "Cerca ID"),
        ("whitelist_sep", "Separati da virgola, punto e virgola, spazio o a capo"),
        ("Add ID", "Aggiungi ID"),
        ("Add Tag", "Aggiungi tag"),
        ("Unselect all tags", "Deseleziona tutti i tag"),
        ("Network error", "Errore di rete"),
        ("Username missed", "Nome utente mancante"),
        ("Password missed", "Password mancante"),
        ("Wrong credentials", "Credenziali errate"),
        ("Edit Tag", "Modifica tag"),
        ("Unremember Password", "Dimentica password"),
        ("Favorites", "Preferiti"),
        ("Add to Favorites", "Aggiungi ai preferiti"),
        ("Remove from Favorites", "Rimuovi dai preferiti"),
        ("Empty", "Vuoto"),
        ("Invalid folder name", "Nome della cartella non valido"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", "Nome host"),
        ("Discovered", "Rilevati"),
        ("install_daemon_tip", "Per avviarsi all'accensione, è necessario installare il servizio di sistema."),
        ("Remote ID", "ID remoto"),
        ("Paste", "Incolla"),
        ("Paste here?", "Incolla qui?"),
        ("Are you sure to close the connection?", "Sei sicuro di voler chiudere la connessione?"),
        ("Download new version", "Scarica nuova versione"),
        ("Touch mode", "Modalità tocco"),
        ("Mouse mode", "Modalità mouse"),
        ("One-Finger Tap", "Tocca con un dito"),
        ("Left Mouse", "Mouse sinistro"),
        ("One-Long Tap", "Tocco lungo con un dito"),
        ("Two-Finger Tap", "Tocca con due dita"),
        ("Right Mouse", "Mouse destro"),
        ("One-Finger Move", "Movimento con un dito"),
        ("Double Tap & Move", "Tocca due volte e sposta"),
        ("Mouse Drag", "Trascina il mouse"),
        ("Three-Finger vertically", "Tre dita in verticale"),
        ("Mouse Wheel", "Rotellina del mouse"),
        ("Two-Finger Move", "Movimento con due dita"),
        ("Canvas Move", "Sposta tela"),
        ("Pinch to Zoom", "Pizzica per zoomare"),
        ("Canvas Zoom", "Zoom canvas"),
        ("Reset canvas", "Ripristina canvas"),
        ("No permission of file transfer", "Nessun permesso di trasferimento di file"),
        ("Note", "Nota"),
        ("Connection", "Connessione"),
        ("Share Screen", "Condividi schermo"),
        ("CLOSE", "CHIUDERE"),
        ("OPEN", "APRIRE"),
        ("Chat", "Chat"),
        ("Total", "Totale"),
        ("items", "Oggetti"),
        ("Selected", "Selezionato"),
        ("Screen Capture", "Cattura schermo"),
        ("Input Control", "Controllo di input"),
        ("Audio Capture", "Acquisizione audio"),
        ("File Connection", "Connessione file"),
        ("Screen Connection", "Connessione schermo"),
        ("Do you accept?", "Accetti?"),
        ("Open System Setting", "Apri impostazioni di sistema"),
        ("How to get Android input permission?", "Come ottenere l'autorizzazione di input su Android?"),
        ("android_input_permission_tip1", "Affinché un dispositivo remoto possa controllare il tuo dispositivo Android tramite mouse o tocco, devi consentire a RustDesk di utilizzare il servizio \"Accessibilità\"."),
        ("android_input_permission_tip2", "Vai alla pagina delle impostazioni di sistema che si aprirà di seguito, trova e accedi a [Servizi installati], attiva il servizio [RustDesk Input]."),
        ("android_new_connection_tip", "È stata ricevuta una nuova richiesta di controllo per il dispositivo corrente."),
        ("android_service_will_start_tip", "L'attivazione di Cattura schermo avvierà automaticamente il servizio, consentendo ad altri dispositivi di richiedere una connessione da questo dispositivo."),
        ("android_stop_service_tip", "La chiusura del servizio chiuderà automaticamente tutte le connessioni stabilite."),
        ("android_version_audio_tip", "L'attuale versione di Android non supporta l'acquisizione audio, esegui l'upgrade ad Android 10 o versioni successive."),
        ("android_start_service_tip", "Toccare [Avvia servizio] o APRI l'autorizzazione [Cattura schermo] per avviare il servizio di condivisione dello schermo."),
        ("Account", "Account"),
        ("Overwrite", "Sovrascrivi"),
        ("This file exists, skip or overwrite this file?", "Questo file esiste, saltare o sovrascrivere questo file?"),
        ("Quit", "Esci"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Aiuto"),
        ("Failed", "Fallito"),
        ("Succeeded", "Successo"),
        ("Someone turns on privacy mode, exit", "Qualcuno attiva la modalità privacy, esci"),
        ("Unsupported", "Non supportato"),
        ("Peer denied", "Peer negato"),
        ("Please install plugins", "Si prega di installare i plugin"),
        ("Peer exit", "Uscita tra pari"),
        ("Failed to turn off", "Impossibile spegnere"),
        ("Turned off", "Spegni"),
        ("In privacy mode", "In modalità privacy"),
        ("Out privacy mode", "Fuori modalità privacy"),
        ("Language", "Linguaggio"),
        ("Keep RustDesk background service", "Mantieni il servizio di RustDesk in background"),
        ("Ignore Battery Optimizations", "Ignora le ottimizzazioni della batteria"),
        ("android_open_battery_optimizations_tip", "Se si desidera disabilitare questa funzione, andare nelle impostazioni dell'applicazione RustDesk, aprire la sezione [Batteria] e deselezionare [Senza restrizioni]."),
        ("Start on Boot", "Avvia all'accensione"),
        ("Start the screen sharing service on boot, requires special permissions", "L'avvio del servizio di condivisione dello schermo all'accensione, richiede autorizzazioni speciali"),
        ("Connection not allowed", "Connessione non consentita"),
        ("Legacy mode", "Modalità legacy"),
        ("Map mode", "Modalità mappa"),
        ("Translate mode", "Modalità di traduzione"),
        ("Use permanent password", "Usa password permanente"),
        ("Use both passwords", "Usa entrambe le password"),
        ("Set permanent password", "Imposta password permanente"),
        ("Enable Remote Restart", "Abilita riavvio da remoto"),
        ("Allow remote restart", "Consenti riavvio da remoto"),
        ("Restart Remote Device", "Riavvia dispositivo remoto"),
        ("Are you sure you want to restart", "Sei sicuro di voler riavviare?"),
        ("Restarting Remote Device", "Il dispositivo remoto si sta riavviando"),
        ("remote_restarting_tip", "Riavviare il dispositivo remoto"),
        ("Copied", "Copiato"),
        ("Exit Fullscreen", "Esci dalla modalità schermo intero"),
        ("Fullscreen", "A schermo intero"),
        ("Mobile Actions", "Azioni mobili"),
        ("Select Monitor", "Seleziona schermo"),
        ("Control Actions", "Azioni di controllo"),
        ("Display Settings", "Impostazioni di visualizzazione"),
        ("Ratio", "Rapporto"),
        ("Image Quality", "Qualità dell'immagine"),
        ("Scroll Style", "Stile di scorrimento"),
        ("Show Menubar", "Mostra la barra dei menu"),
        ("Hide Menubar", "nascondi la barra dei menu"),
        ("Direct Connection", "Connessione diretta"),
        ("Relay Connection", "Connessione relay"),
        ("Secure Connection", "Connessione sicura"),
        ("Insecure Connection", "Connessione non sicura"),
        ("Scale original", "Scala originale"),
        ("Scale adaptive", "Scala adattiva"),
        ("General", "Generale"),
        ("Security", "Sicurezza"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tema Scuro"),
        ("Light Theme", ""),
        ("Dark", "Scuro"),
        ("Light", "Chiaro"),
        ("Follow System", "Segui il sistema"),
        ("Enable hardware codec", "Abilita codec hardware"),
        ("Unlock Security Settings", "Sblocca impostazioni di sicurezza"),
        ("Enable Audio", "Abilita audio"),
        ("Unlock Network Settings", "Sblocca impostazioni di rete"),
        ("Server", "Server"),
        ("Direct IP Access", "Accesso IP diretto"),
        ("Proxy", "Proxy"),
        ("Apply", "Applica"),
        ("Disconnect all devices?", "Disconnettere tutti i dispositivi?"),
        ("Clear", "Ripulisci"),
        ("Audio Input Device", "Dispositivo di input audio"),
        ("Deny remote access", "Nega accesso remoto"),
        ("Use IP Whitelisting", "Utilizza la whitelist di IP"),
        ("Network", "Rete"),
        ("Enable RDP", "Abilita RDP"),
        ("Pin menubar", "Blocca la barra dei menu"),
        ("Unpin menubar", "Sblocca la barra dei menu"),
        ("Recording", "Registrazione"),
        ("Directory", "Cartella"),
        ("Automatically record incoming sessions", "Registra automaticamente le sessioni in entrata"),
        ("Change", "Cambia"),
        ("Start session recording", "Inizia registrazione della sessione"),
        ("Stop session recording", "Ferma registrazione della sessione"),
        ("Enable Recording Session", "Abilita registrazione della sessione"),
        ("Allow recording session", "Permetti di registrare la sessione"),
        ("Enable LAN Discovery", "Abilita il rilevamento della LAN"),
        ("Deny LAN Discovery", "Nega il rilevamento della LAN"),
        ("Write a message", "Scrivi un messaggio"),
        ("Prompt", "Richiede"),
        ("Please wait for confirmation of UAC...", "Attendi la conferma dell'UAC..."),
        ("elevated_foreground_window_tip", "La finestra corrente del desktop remoto richiede privilegi più elevati per funzionare, quindi non è in grado di utilizzare temporaneamente il mouse e la tastiera. È possibile chiedere all'utente remoto di ridurre a icona la finestra corrente o di fare clic sul pulsante di elevazione nella finestra di gestione della connessione. Per evitare questo problema, si consiglia di installare il software sul dispositivo remoto."),
        ("Disconnected", "Disconnesso"),
        ("Other", "Altro"),
        ("Confirm before closing multiple tabs", "Conferma prima di chiudere più schede"),
        ("Keyboard Settings", "Impostazioni tastiera"),
        ("Full Access", "Accesso completo"),
        ("Screen Share", "Condivisione dello schermo"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland richiede Ubuntu 21.04 o successiva."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland richiede una versione superiore della distribuzione Linux. Prova X11 desktop o cambia il tuo sistema operativo."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Seleziona lo schermo da condividere (opera sul lato peer)."),
        ("Show RustDesk", "Mostra RustDesk"),
        ("This PC", "Questo PC"),
        ("or", "O"),
        ("Continue with", "Continua con"),
        ("Elevate", "Eleva"),
        ("Zoom cursor", "Cursore zoom"),
        ("Accept sessions via password", "Accetta sessioni via password"),
        ("Accept sessions via click", "Accetta sessioni via click"),
        ("Accept sessions via both", "Accetta sessioni con entrambi"),
        ("Please wait for the remote side to accept your session request...", "Attendere che il lato remoto accetti la richiesta di sessione..."),
        ("One-time Password", "Password monouso"),
        ("Use one-time password", "Usa password monouso"),
        ("One-time password length", "Lunghezza password monouso"),
        ("Request access to your device", "Richiedi l'accesso al tuo dispositivo"),
        ("Hide connection management window", "Nascondi la finestra di gestione delle connessioni"),
        ("hide_cm_tip", "Permetti di nascondere solo se si accettano sessioni con password permanente"),
        ("wayland_experiment_tip", "Il supporto Wayland è in fase sperimentale, utilizza X11 se necessiti di un accesso stabile."),
        ("Right click to select tabs", "Clic con il tasto destro per selezionare le schede"),
        ("Skipped", "Saltato"),
        ("Add to Address Book", "Aggiungi alla rubrica"),
        ("Group", "Gruppo"),
        ("Search", "Cerca"),
        ("Closed manually by web console", "Chiudi manualmente dalla console Web"),
        ("Local keyboard type", "Tipo di tastiera locale"),
        ("Select local keyboard type", "Seleziona il tipo di tastiera locale"),
        ("software_render_tip", "Se si dispone di una scheda grafica Nvidia e la finestra remota si chiude immediatamente dopo la connessione, l'installazione del driver nouveau e la scelta di utilizzare il rendering software possono aiutare. È necessario un riavvio del software."),
        ("Always use software rendering", "Usa sempre il render Software"),
        ("config_input", "Per controllare il desktop remoto con la tastiera, è necessario concedere le autorizzazioni a RustDesk \"Monitoraggio dell'input\"."),
        ("config_microphone", "Per poter chiamare, è necessario concedere l'autorizzazione a RustDesk \"Registra audio\"."),
        ("request_elevation_tip", "È possibile richiedere l'elevazione se c'è qualcuno sul lato remoto."),
        ("Wait", "Attendi"),
        ("Elevation Error", "Errore durante l'elevazione dei diritti"),
        ("Ask the remote user for authentication", "Chiedere l'autenticazione all'utente remoto"),
        ("Choose this if the remote account is administrator", "Scegliere questa opzione se l'account remoto è amministratore"),
        ("Transmit the username and password of administrator", "Trasmettere il nome utente e la password dell'amministratore"),
        ("still_click_uac_tip", "Richiede ancora che l'utente remoto faccia clic su OK nella finestra UAC dell'esecuzione di RustDesk."),
        ("Request Elevation", "Richiedi elevazione dei diritti"),
        ("wait_accept_uac_tip", "Attendere che l'utente remoto accetti la finestra di dialogo UAC."),
        ("Elevate successfully", "Elevazione dei diritti effettuata con successo"),
        ("uppercase", "Maiuscola"),
        ("lowercase", "Minuscola"),
        ("digit", "Numero"),
        ("special character", "Carattere speciale"),
        ("length>=8", "Lunghezza >= 8"),
        ("Weak", "Debole"),
        ("Medium", "Media"),
        ("Strong", "Forte"),
        ("Switch Sides", "Cambia lato"),
        ("Please confirm if you want to share your desktop?", "Vuoi condividere il tuo desktop?"),
        ("Display", "Visualizzazione"),
        ("Default View Style", "Stile Visualizzazione Predefinito"),
        ("Default Scroll Style", "Stile Scorrimento Predefinito"),
        ("Default Image Quality", "Qualità Immagine Predefinita"),
        ("Default Codec", "Codec Predefinito"),
        ("Bitrate", "Bitrate"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Altre Opzioni Predefinite"),
        ("Voice call", "Chiamata vocale"),
        ("Text chat", "Chat testuale"),
        ("Stop voice call", "Interrompi la chiamata vocale"),
        ("relay_hint_tip", "Se non è possibile connettersi direttamente, si può provare a farlo tramite relay.\nInoltre, se si desidera utilizzare il relay al primo tentativo, è possibile aggiungere il suffisso \"/r\" all'ID o selezionare l'opzione \"Collegati sempre tramite relay\" nella scheda peer."),
        ("Reconnect", "Riconnetti"),
        ("Codec", "Codec"),
        ("Resolution", "Risoluzione"),
        ("No transfers in progress", "Nessun trasferimento in corso"),
        ("Set one-time password length", ""),
        ("Install driver cert (test cert)", ""),
        ("Virtual display need", ""),
        ("instsall_cert_tip", "")
    ].iter().cloned().collect();
}
