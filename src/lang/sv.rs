lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Ditt skrivbord"),
        ("desk_tip", "Ditt skrivbord kan delas med hjälp av detta ID och lösenord"),
        ("Password", "Lösenord"),
        ("Ready", "Redo"),
        ("Established", "Uppkopplad"),
        ("connecting_status", "Ansluter till RustDesk..."),
        ("Enable service", "Sätt på tjänsten"),
        ("Start service", "Starta tjänsten"),
        ("Service is running", "Tjänsten är startad"),
        ("Service is not running", "Tjänsten är ej startad"),
        ("not_ready_status", "Ej redo. Kontrollera din nätverksanslutning"),
        ("Control Remote Desktop", "Kontrollera fjärrskrivbord"),
        ("Transfer file", "Överför fil"),
        ("Connect", "Anslut"),
        ("Recent sessions", "Dina senaste sessioner"),
        ("Address book", "Addressbok"),
        ("Confirmation", "Bekräftelse"),
        ("TCP tunneling", "TCP Tunnel"),
        ("Remove", "Ta bort"),
        ("Refresh random password", "Skapa nytt slumpmässigt lösenord"),
        ("Set your own password", "Skapa ditt eget lösenord"),
        ("Enable keyboard/mouse", "Tillåt tangentbord/mus"),
        ("Enable clipboard", "Tillåt urklipp"),
        ("Enable file transfer", "Tillåt filöverföring"),
        ("Enable TCP tunneling", "Tillåt TCP tunnel"),
        ("IP Whitelisting", "IP Vitlisting"),
        ("ID/Relay Server", "ID/Relay Server"),
        ("Import server config", "Importera Server config"),
        ("Export Server Config", "Exportera Server config"),
        ("Import server configuration successfully", "Importering lyckades"),
        ("Export server configuration successfully", "Exportering lyckades"),
        ("Invalid server configuration", "Ogiltig server config"),
        ("Clipboard is empty", "Urklippet är tomt"),
        ("Stop service", "Avsluta tjänsten"),
        ("Change ID", "Byt ID"),
        ("Your new ID", "Ditt nya ID"),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "Bara a-z, A-Z, 0-9 och _ (understräck) tecken är tillåtna. Den första bokstaven måste vara a-z, A-Z. Längd mellan 6 och 16."),
        ("Website", "Hemsida"),
        ("About", "Om"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Tyst"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Ljud input"),
        ("Enhancements", "Förbättringar"),
        ("Hardware Codec", "Hårdvarucodec"),
        ("Adaptive bitrate", "Adaptiv Bitrate"),
        ("ID Server", "ID server"),
        ("Relay Server", "Relay Server"),
        ("API Server", "API Server"),
        ("invalid_http", "måste börja med http:// eller https://"),
        ("Invalid IP", "Ogiltig IP"),
        ("Invalid format", "Ogiltigt format"),
        ("server_not_support", "Stöds ännu inte av servern"),
        ("Not available", "Ej tillgänglig"),
        ("Too frequent", "För ofta"),
        ("Cancel", "Avbryt"),
        ("Skip", "Hoppa över"),
        ("Close", "Stäng"),
        ("Retry", "Försök igen"),
        ("OK", "OK"),
        ("Password Required", "Lösenord krävs"),
        ("Please enter your password", "Skriv in ditt lösenord"),
        ("Remember password", "Kom ihåg lösenord"),
        ("Wrong Password", "Fel lösenord"),
        ("Do you want to enter again?", "Vill du skriva in igen?"),
        ("Connection Error", "Anslutningsfel"),
        ("Error", "Ett fel uppstod"),
        ("Reset by the peer", "Återställt av klienten"),
        ("Connecting...", "Ansluter..."),
        ("Connection in progress. Please wait.", "Anslutning pågår. Var god vänta."),
        ("Please try 1 minute later", "Försök igen om en minut"),
        ("Login Error", "Inloggningsfel"),
        ("Successful", "Lyckat"),
        ("Connected, waiting for image...", "Ansluten, väntar på bild..."),
        ("Name", "Namn"),
        ("Type", "Typ"),
        ("Modified", "Modifierad"),
        ("Size", "Storlek"),
        ("Show Hidden Files", "Visa gömda filer"),
        ("Receive", "Ta emot"),
        ("Send", "Skicka"),
        ("Refresh File", "Uppdatera fil"),
        ("Local", "Lokalt"),
        ("Remote", "Fjärr"),
        ("Remote Computer", "Fjärrdator"),
        ("Local Computer", "Lokal dator"),
        ("Confirm Delete", "Bekräfta borttagning"),
        ("Delete", "Ta bort"),
        ("Properties", "Egenskaper"),
        ("Multi Select", "Välj flera"),
        ("Select All", "Markera alla "),
        ("Unselect All", "Avmärkera alla"),
        ("Empty Directory", "Tom mapp"),
        ("Not an empty directory", "Inte en tom mapp"),
        ("Are you sure you want to delete this file?", "Är du säker att du vill ta bort filen?"),
        ("Are you sure you want to delete this empty directory?", "Är du säker att du vill ta bort den tomma mappen?"),
        ("Are you sure you want to delete the file of this directory?", "Är du säker att du vill ta bort filen ur mappen?"),
        ("Do this for all conflicts", "Gör för alla konflikter"),
        ("This is irreversible!", "Detta går ej att ångra!"),
        ("Deleting", "Tar bort"),
        ("files", "filer"),
        ("Waiting", "Väntnar"),
        ("Finished", "Klar"),
        ("Speed", "Hastighet"),
        ("Custom Image Quality", "Anpassad bildkvalitet"),
        ("Privacy mode", "Säkerhetsläge"),
        ("Block user input", "Blokera användarinput"),
        ("Unblock user input", "Tillåt användarinput"),
        ("Adjust Window", "Ändra fönster"),
        ("Original", "Orginal"),
        ("Shrink", "Krymp"),
        ("Stretch", "Sträck ut"),
        ("Scrollbar", "Scrollbar"),
        ("ScrollAuto", "ScrollAuto"),
        ("Good image quality", "Bra bildkvalitet"),
        ("Balanced", "Balanserad"),
        ("Optimize reaction time", "Optimera reaktionstid"),
        ("Custom", "Anpassat"),
        ("Show remote cursor", "Visa fjärrmus"),
        ("Show quality monitor", "Visa bildkvalitet"),
        ("Disable clipboard", "Stäng av urklipp"),
        ("Lock after session end", "Lås efter sessionens slut"),
        ("Insert", "Insert"),
        ("Insert Lock", "Insert lås"),
        ("Refresh", "Uppdatera"),
        ("ID does not exist", "Detta ID existerar inte"),
        ("Failed to connect to rendezvous server", "Lyckades inte ansluta till randezvous servern"),
        ("Please try later", "Försök igen senare"),
        ("Remote desktop is offline", "Fjärrskrivbordet är offline"),
        ("Key mismatch", "Nyckeln stämmer inte"),
        ("Timeout", "Timeout"),
        ("Failed to connect to relay server", "Lyckades inte ansluta till relay servern"),
        ("Failed to connect via rendezvous server", "Lyckades inte ansluta via randezvous servern"),
        ("Failed to connect via relay server", "Lyckades inte ansluta via relay servern"),
        ("Failed to make direct connection to remote desktop", "Lyckades inte ansluta direkt till fjärrskrivbordet"),
        ("Set Password", "Välj lösenord"),
        ("OS Password", "OS lösenord"),
        ("install_tip", "På grund av UAC, kan inte RustDesk fungera ordentligt på klientsidan. För att undvika problem med UAC, tryck på knappen nedan för att installera RustDesk på systemet."),
        ("Click to upgrade", "Klicka för att nedgradera"),
        ("Click to download", "Klicka för att ladda ner"),
        ("Click to update", "Klicka för att uppdatera"),
        ("Configure", "Konfigurera"),
        ("config_acc", "För att kontrollera din dator på distans måste du ge RustDesk \"Tillgänglighets\" rättigheter."),
        ("config_screen", "För att kontrollera din dator på distans måste du ge RustDesk \"Skärminspelnings\" rättigheter."),
        ("Installing ...", "Installerar..."),
        ("Install", "Installera"),
        ("Installation", "Installation"),
        ("Installation Path", "Installationsplats"),
        ("Create start menu shortcuts", "Skapa startmeny genväg"),
        ("Create desktop icon", "Skapa ikon på skrivbordet"),
        ("agreement_tip", "Genom att starta installationen accepterar du licensavtalet."),
        ("Accept and Install", "Acceptera och installera"),
        ("End-user license agreement", "End-user license agreement"),
        ("Generating ...", "Genererar..."),
        ("Your installation is lower version.", "Ditt skrivbord har en lägre version"),
        ("not_close_tcp_tip", "Stäng inde detta fönster när du använder tunneln"),
        ("Listening ...", "Lyssnar..."),
        ("Remote Host", "Fjärrhost"),
        ("Remote Port", "Fjärrport"),
        ("Action", "Handling"),
        ("Add", "Lägg till"),
        ("Local Port", "Lokal port"),
        ("Local Address", "Lokal adress"),
        ("Change Local Port", "Ändra lokal port"),
        ("setup_server_tip", "Sätt upp din egen server för en snabbare anslutning"),
        ("Too short, at least 6 characters.", "För kort, minst 6 tecken."),
        ("The confirmation is not identical.", "Bekräftelsen stämmer inte."),
        ("Permissions", "Rättigheter"),
        ("Accept", "Acceptera"),
        ("Dismiss", "Tillåt inte"),
        ("Disconnect", "Koppla ifrån"),
        ("Enable file copy and paste", "Tillåt kopiering av filer"),
        ("Connected", "Ansluten"),
        ("Direct and encrypted connection", "Direkt och krypterad anslutning"),
        ("Relayed and encrypted connection", "Vidarebefodrad och krypterad anslutning"),
        ("Direct and unencrypted connection", "Direkt och okrypterad anslutning"),
        ("Relayed and unencrypted connection", "Vidarebefodrad och okrypterad anslutning"),
        ("Enter Remote ID", "Skriv in fjärr-ID"),
        ("Enter your password", "Skriv in ditt lösenord"),
        ("Logging in...", "Loggar in..."),
        ("Enable RDP session sharing", "Tillåt RDP sessionsdelning"),
        ("Auto Login", "Auto Login (Bara giltigt om du sätter \"Lås efter sessionens slut\")"),
        ("Enable direct IP access", "Tillåt direkt IP anslutningar"),
        ("Rename", "Byt namn"),
        ("Space", "Mellanslag"),
        ("Create desktop shortcut", "Skapa skrivbordsgenväg"),
        ("Change Path", "Ändra plats"),
        ("Create Folder", "Skapa mapp"),
        ("Please enter the folder name", "Skriv in namnet på mappen"),
        ("Fix it", "Fixa det"),
        ("Warning", "Varning"),
        ("Login screen using Wayland is not supported", "Login med Wayland stöds inte"),
        ("Reboot required", "Omstart krävs"),
        ("Unsupported display server", "Displayserver stöds inte "),
        ("x11 expected", "x11 förväntades"),
        ("Port", "Port"),
        ("Settings", "Inställningar"),
        ("Username", "Användarnamn"),
        ("Invalid port", "Ogiltig port"),
        ("Closed manually by the peer", "Stängd manuellt av klienten"),
        ("Enable remote configuration modification", "Tillåt fjärrkonfigurering"),
        ("Run without install", "Kör utan installation"),
        ("Connect via relay", "Anslut via relay"),
        ("Always connect via relay", "Anslut alltid via relay"),
        ("whitelist_tip", "Bara vitlistade IPs kan koppla upp till mig"),
        ("Login", "Logga in"),
        ("Verify", "Verifiera"),
        ("Remember me", "Kom ihåg mig"),
        ("Trust this device", "Lita på denna enhet"),
        ("Verification code", "Verifikationskod"),
        ("verification_tip", ""),
        ("Logout", "Logga ut"),
        ("Tags", "Taggar"),
        ("Search ID", "Sök ID"),
        ("whitelist_sep", "Separerat av ett comma, semikolon, mellanslag eller ny linje"),
        ("Add ID", "Lägg till ID"),
        ("Add Tag", "Lägg till Tagg"),
        ("Unselect all tags", "Avmarkera alla taggar"),
        ("Network error", "Nätverksfel"),
        ("Username missed", "Användarnamn saknas"),
        ("Password missed", "Lösenord saknas"),
        ("Wrong credentials", "Fel användarnamn eller lösenord"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "Ändra Tagg"),
        ("Forget Password", "Glöm lösenord"),
        ("Favorites", "Favoriter"),
        ("Add to Favorites", "Lägg till favorit"),
        ("Remove from Favorites", "Ta bort från favoriter"),
        ("Empty", "Tom"),
        ("Invalid folder name", "Ogiltigt mappnamn"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Socks5/Http(s) Proxy", "Socks5/Http(s) Proxy"),
        ("Discovered", "Upptäckt"),
        ("install_daemon_tip", "För att starta efter boot måste du installera systemtjänsten."),
        ("Remote ID", "Fjärr ID"),
        ("Paste", "Klistra in"),
        ("Paste here?", "Klistra in här?"),
        ("Are you sure to close the connection?", "Är du säker att du vill avsluta anslutningen?"),
        ("Download new version", "Ladda ner ny version"),
        ("Touch mode", "Touchläge"),
        ("Mouse mode", "Musläge"),
        ("One-Finger Tap", "En fingers tryck"),
        ("Left Mouse", "Vänster mus"),
        ("One-Long Tap", "Långt tryck"),
        ("Two-Finger Tap", "Långt tryck med två fingrar"),
        ("Right Mouse", "Höger mus"),
        ("One-Finger Move", "En fingers drag"),
        ("Double Tap & Move", "Dubbeltryck och flytta"),
        ("Mouse Drag", "Dra med musen"),
        ("Three-Finger vertically", "Tre fingrar vertikalt"),
        ("Mouse Wheel", "Scrollhjul"),
        ("Two-Finger Move", "Två fingers flytt"),
        ("Canvas Move", "Flytta canvas"),
        ("Pinch to Zoom", "Nyp för zoom"),
        ("Canvas Zoom", "Canvas zoom"),
        ("Reset canvas", "Återställ canvas"),
        ("No permission of file transfer", "Rättigheter saknas"),
        ("Note", "Notering"),
        ("Connection", "Anslutning"),
        ("Share Screen", "Dela skärm"),
        ("Chat", "Chatt"),
        ("Total", "Totalt"),
        ("items", "föremål"),
        ("Selected", "Valda"),
        ("Screen Capture", "Skärminspelning"),
        ("Input Control", "Inputkontroll"),
        ("Audio Capture", "Ljudinspelning"),
        ("File Connection", "Fil anslutning"),
        ("Screen Connection", "Skärm anslutning"),
        ("Do you accept?", "Accepterar du?"),
        ("Open System Setting", "Öppna systeminställnig"),
        ("How to get Android input permission?", "Hur får man Android rättigheter?"),
        ("android_input_permission_tip1", "Android rättigheter saknas"),
        ("android_input_permission_tip2", "Gå till systeminställningarna, hitta [Installed Services], sätt på [RustDesk Input] tjänsten."),
        ("android_new_connection_tip", "Ny kontrollförfrågan mottagen, denna vill kontrollera din enhet."),
        ("android_service_will_start_tip", "Sätter du på \"skärminspelning\" kommer tjänsten automatiskt att starta. Detta tillåter andra enheter att kontrollera din enhet."),
        ("android_stop_service_tip", "Genom att stänga av tjänsten kommer alla enheter att kopplas ifrån."),
        ("android_version_audio_tip", "Din version av Android stödjer inte ljudinspelning, Android 10 eller nyare krävs"),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "Konto"),
        ("Overwrite", "Skriv över"),
        ("This file exists, skip or overwrite this file?", "Filen finns redan, hoppa över eller skriv över filen?"),
        ("Quit", "Avsluta"),
        ("Help", "Hjälp"),
        ("Failed", "Misslyckades"),
        ("Succeeded", "Lyckades"),
        ("Someone turns on privacy mode, exit", "Någon sätter på säkerhetesläge, avsluta"),
        ("Unsupported", "Stöds inte"),
        ("Peer denied", "Klienten nekade"),
        ("Please install plugins", "Var god installera plugins"),
        ("Peer exit", "Avsluta klient"),
        ("Failed to turn off", "Misslyckades med avstängning"),
        ("Turned off", "Avstängd"),
        ("Language", "Språk"),
        ("Keep RustDesk background service", "Behåll RustDesk i bakgrunden"),
        ("Ignore Battery Optimizations", "Ignorera batterioptimering"),
        ("android_open_battery_optimizations_tip", "Om du vill stänga av denna funktion, gå till nästa RustDesk programs inställningar, hitta [Batteri], Checka ur [Obegränsad]"),
        ("Start on boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", "Anslutning ej tillåten"),
        ("Legacy mode", "Legacy mode"),
        ("Map mode", "Kartläge"),
        ("Translate mode", "Översättningsläge"),
        ("Use permanent password", "Använd permanent lösenord"),
        ("Use both passwords", "Använd båda lösenorden"),
        ("Set permanent password", "Ställ in permanent lösenord"),
        ("Enable remote restart", "Sätt på fjärromstart"),
        ("Restart remote device", "Starta om fjärrenheten"),
        ("Are you sure you want to restart", "Är du säker att du vill starta om?"),
        ("Restarting remote device", "Startar om fjärrenheten"),
        ("remote_restarting_tip", "Enheten startar om, stäng detta meddelande och anslut igen om en liten stund"),
        ("Copied", "Kopierad"),
        ("Exit Fullscreen", "Gå ur fullskärmsläge"),
        ("Fullscreen", "Fullskärm"),
        ("Mobile Actions", "Mobila återgärder"),
        ("Select Monitor", "Välj skärm"),
        ("Control Actions", "Kontroller"),
        ("Display Settings", "Skärminställningar"),
        ("Ratio", "Ratio"),
        ("Image Quality", "Bildkvalitet"),
        ("Scroll Style", "Scrollstil"),
        ("Show Toolbar", ""),
        ("Hide Toolbar", ""),
        ("Direct Connection", "Direktanslutning"),
        ("Relay Connection", "Relayanslutning"),
        ("Secure Connection", "Säker anslutning"),
        ("Insecure Connection", "Osäker anslutning"),
        ("Scale original", "Skala orginal"),
        ("Scale adaptive", "Skala adaptivt"),
        ("General", "Generellt"),
        ("Security", "Säkerhet"),
        ("Theme", "Tema"),
        ("Dark Theme", "Mörkt tema"),
        ("Light Theme", ""),
        ("Dark", "Mörk"),
        ("Light", "Ljus"),
        ("Follow System", "Följ system"),
        ("Enable hardware codec", "Aktivera hårdvarucodec"),
        ("Unlock Security Settings", "Lås upp säkerhetsinställningar"),
        ("Enable audio", "Sätt på ljud"),
        ("Unlock Network Settings", "Lås upp nätverksinställningar"),
        ("Server", "Server"),
        ("Direct IP Access", "Direkt IP åtkomst"),
        ("Proxy", "Proxy"),
        ("Apply", "Tillämpa"),
        ("Disconnect all devices?", "Koppla ifrån alla enheter?"),
        ("Clear", "Töm"),
        ("Audio Input Device", "Inmatningsenhet för ljud"),
        ("Use IP Whitelisting", "Använd IP-Vitlistning"),
        ("Network", "Nätverk"),
        ("Pin Toolbar", ""),
        ("Unpin Toolbar", ""),
        ("Recording", "Spelar in"),
        ("Directory", "Katalog"),
        ("Automatically record incoming sessions", "Spela in inkommande sessioner automatiskt"),
        ("Change", "Byt"),
        ("Start session recording", "Starta inspelning"),
        ("Stop session recording", "Avsluta inspelning"),
        ("Enable recording session", "Sätt på sessionsinspelning"),
        ("Enable LAN discovery", "Sätt på LAN upptäckt"),
        ("Deny LAN discovery", "Neka LAN upptäckt"),
        ("Write a message", "Skriv ett meddelande"),
        ("Prompt", "Prompt"),
        ("Please wait for confirmation of UAC...", "Var god vänta för UAC bekräftelse..."),
        ("elevated_foreground_window_tip", "Detta fönster hos klienten kräver en högre behörighet. Du kan be användaren att minimera fönstret, eller att ge högre behörigheter i fönstret för anslutningsinställningar. För att undvika detta problem i framtiden, installera programmet på klientens sida."),
        ("Disconnected", "Frånkopplad"),
        ("Other", "Övrigt"),
        ("Confirm before closing multiple tabs", "Bekräfta innan du stänger flera flikar"),
        ("Keyboard Settings", "Tangentbordsinställningar"),
        ("Full Access", "Full tillgång"),
        ("Screen Share", "Skärmdelning"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland kräver Ubuntu 21.04 eller högre."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland kräver en högre version av linux. Försök igen eller byt OS."),
        ("JumpLink", "JumpLink"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Välj skärm att dela"),
        ("Show RustDesk", "Visa RustDesk"),
        ("This PC", "Denna dator"),
        ("or", "eller"),
        ("Continue with", "Fortsätt med"),
        ("Elevate", "Höj upp"),
        ("Zoom cursor", "Zoom"),
        ("Accept sessions via password", "Acceptera sessioner via lösenord"),
        ("Accept sessions via click", "Acceptera sessioner via klick"),
        ("Accept sessions via both", "Acceptera sessioner via båda"),
        ("Please wait for the remote side to accept your session request...", "Var god vänta på att klienten accepterar din förfrågan..."),
        ("One-time Password", "En-gångs lösenord"),
        ("Use one-time password", "Använd en-gångs lösenord"),
        ("One-time password length", "Längd på en-gångs lösenord"),
        ("Request access to your device", "Begär åtkomst till din enhet"),
        ("Hide connection management window", "Göm hanteringsfönster"),
        ("hide_cm_tip", "Tillåt att gömma endast om accepterande sessioner med lösenord och permanenta lösenord"),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to address book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", ""),
        ("Always use software rendering", ""),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", ""),
        ("Elevation Error", ""),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", ""),
        ("Request Elevation", ""),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", ""),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", ""),
        ("Default Scroll Style", ""),
        ("Default Image Quality", ""),
        ("Default Codec", ""),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", ""),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", ""),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("RDP Settings", ""),
        ("Sort by", ""),
        ("New Connection", ""),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", ""),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", ""),
        ("Empty Username", ""),
        ("Empty Password", ""),
        ("Me", ""),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", ""),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", ""),
        ("System Sound", ""),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", ""),
        ("no fingerprints", ""),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", ""),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", ""),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", ""),
        ("Refresh Password", ""),
        ("ID", ""),
        ("Grid View", ""),
        ("List View", ""),
        ("Select", ""),
        ("Toggle Tags", ""),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", ""),
        ("Primary Color", ""),
        ("HSV Color", ""),
        ("Installation Successful!", ""),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", ""),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", ""),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", ""),
        ("2FA code", ""),
        ("More", ""),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
        ("Email verification code must be 6 characters.", ""),
        ("2FA code must be 6 digits.", ""),
        ("Multiple Windows sessions found", ""),
        ("Please select the session you want to connect to", ""),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", ""),
        ("My address book", ""),
        ("Personal", ""),
        ("Owner", ""),
        ("Set shared password", ""),
        ("Exist in", ""),
        ("Read-only", ""),
        ("Read/Write", ""),
        ("Full Control", ""),
        ("share_warning_tip", ""),
        ("Everyone", ""),
        ("ab_web_console_tip", ""),
        ("allow-only-conn-window-open-tip", ""),
        ("no_need_privacy_mode_no_physical_displays_tip", ""),
        ("Follow remote cursor", ""),
        ("Follow remote window focus", ""),
        ("default_proxy_tip", ""),
        ("no_audio_input_device_tip", ""),
        ("Incoming", ""),
        ("Outgoing", ""),
        ("Clear Wayland screen selection", ""),
        ("clear_Wayland_screen_selection_tip", ""),
        ("confirm_clear_Wayland_screen_selection_tip", ""),
        ("android_new_voice_call_tip", ""),
        ("texture_render_tip", ""),
        ("Use texture rendering", ""),
        ("Floating window", ""),
        ("floating_window_tip", ""),
        ("Keep screen on", ""),
        ("Never", ""),
        ("During controlled", ""),
        ("During service is on", ""),
        ("Capture screen using DirectX", ""),
        ("Back", ""),
        ("Apps", ""),
        ("Volume up", ""),
        ("Volume down", ""),
        ("Power", ""),
        ("Telegram bot", ""),
        ("enable-bot-tip", ""),
        ("enable-bot-desc", ""),
    ].iter().cloned().collect();
}
