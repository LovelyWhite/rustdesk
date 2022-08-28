lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stato"),
        ("Your Desktop", "Via aparato"),
        ("desk_tip", "Via aparato povas esti alirita kun tiu identigilo kaj pasvorto"),
        ("Password", "Pasvorto"),
        ("Ready", "Preta"),
        ("Established", ""),
        ("connecting_status", "Konektante al la reto RustDesk..."),
        ("Enable Service", "Ebligi servon"),
        ("Start Service", "Starti servon"),
        ("Service is running", ""),
        ("Service is not running", "La servo ne funkcias"),
        ("not_ready_status", "Ne preta, bonvolu kontroli la retkonekto"),
        ("Control Remote Desktop", "Kontroli foran aparaton"),
        ("Transfer File", "Transigi dosieron"),
        ("Connect", "Konekti al"),
        ("Recent Sessions", "Lastaj sesioj"),
        ("Address Book", "Adresaro"),
        ("Confirmation", "Konfirmacio"),
        ("TCP Tunneling", "Tunelado TCP"),
        ("Remove", "Forigi"),
        ("Refresh random password", "Regeneri hazardan pasvorton"),
        ("Set your own password", "Agordi vian propran pasvorton"),
        ("Enable Keyboard/Mouse", "Ebligi klavaro/muso"),
        ("Enable Clipboard", "Sinkronigi poŝon"),
        ("Enable File Transfer", "Ebligi dosiertransigado"),
        ("Enable TCP Tunneling", "Ebligi tunelado TCP"),
        ("IP Whitelisting", "Listo de IP akceptataj"),
        ("ID/Relay Server", "Identigila/Relajsa servilo"),
        ("Stop service", "Haltu servon"),
        ("Change ID", "Ŝanĝi identigilon"),
        ("Website", "Retejo"),
        ("About", "Pri"),
        ("Mute", "Muta"),
        ("Audio Input", "Aŭdia enigo"),
        ("Enhancements", ""),
        ("Hardware Codec", ""),
        ("Adaptive Bitrate", ""),
        ("ID Server", "Servilo de identigiloj"),
        ("Relay Server", "Relajsa servilo"),
        ("API Server", "Servilo de API"),
        ("invalid_http", "Devas komenci kun http:// aŭ https://"),
        ("Invalid IP", "IP nevalida"),
        ("id_change_tip", "Nur la signoj a-z, A-Z, 0-9, _ (substreko) povas esti uzataj. La unua litero povas esti inter a-z, A-Z. La longeco devas esti inter 6 kaj 16."),
        ("Invalid format", "Formato nevalida"),
        ("server_not_support", "Ankoraŭ ne subtenata de la servilo"),
        ("Not available", "Nedisponebla"),
        ("Too frequent", "Tro ofte ŝanĝita, bonvolu reprovi poste"),
        ("Cancel", "Nuligi"),
        ("Skip", "Ignori"),
        ("Close", "Fermi"),
        ("Retry", "Reprovi"),
        ("OK", "Konfermi"),
        ("Password Required", "Pasvorto deviga"),
        ("Please enter your password", "Bonvolu tajpi vian pasvorton"),
        ("Remember password", "Memori pasvorton"),
        ("Wrong Password", "Erara pasvorto"),
        ("Do you want to enter again?", "Ĉu vi aliri denove?"),
        ("Connection Error", "Eraro de konektado"),
        ("Error", "Eraro"),
        ("Reset by the peer", "La konekto estas fermita de la samtavolano"),
        ("Connecting...", "Konektante..."),
        ("Connection in progress. Please wait.", "Konektado farata. Bonvolu atendi."),
        ("Please try 1 minute later", "Reprovi post 1 minuto"),
        ("Login Error", "Eraro de konektado"),
        ("Successful", "Sukceso"),
        ("Connected, waiting for image...", "Konektita, atendante bildon..."),
        ("Name", "Nomo"),
        ("Type", ""),
        ("Modified", "Modifita"),
        ("Size", "Grandeco"),
        ("Show Hidden Files", "Montri kaŝitajn dosierojn"),
        ("Receive", "Akcepti"),
        ("Send", "Sendi"),
        ("Refresh File", ""),
        ("Local", ""),
        ("Remote", ""),
        ("Remote Computer", "Fora komputilo"),
        ("Local Computer", "Loka komputilo"),
        ("Confirm Delete", "Konfermi la forigo"),
        ("Delete", ""),
        ("Properties", ""),
        ("Multi Select", ""),
        ("Empty Directory", ""),
        ("Not an empty directory", ""),
        ("Are you sure you want to delete this file?", "Ĉu vi vere volas forigi tiun dosieron?"),
        ("Are you sure you want to delete this empty directory?", ""),
        ("Are you sure you want to delete the file of this directory?", ""),
        ("Do this for all conflicts", "Same por ĉiuj konfliktoj"),
        ("This is irreversible!", ""),
        ("Deleting", "Forigado"),
        ("files", "dosiero"),
        ("Waiting", "Atendante..."),
        ("Finished", "Finita"),
        ("Speed", ""),
        ("Custom Image Quality", "Agordi bildan kvaliton"),
        ("Privacy mode", "Modo privata"),
        ("Block user input", "Bloki uzanta enigo"),
        ("Unblock user input", "Malbloki uzanta enigo"),
        ("Adjust Window", "Adapti fenestro"),
        ("Original", "Originala rilatumo"),
        ("Shrink", "Ŝrumpi"),
        ("Stretch", "Streĉi"),
        ("Scrollbar", "Rulumbreto"),
        ("ScrollAuto", "Rulumu Aŭtomate"),
        ("Good image quality", "Bona bilda kvalito"),
        ("Balanced", "Normala bilda kvalito"),
        ("Optimize reaction time", "Optimigi reakcia tempo"),
        ("Custom", "Personigi bilda kvalito"),
        ("Show remote cursor", "Montri foran kursoron"),
        ("Show quality monitor", ""),
        ("Disable clipboard", "Malebligi poŝon"),
        ("Lock after session end", "Ŝlosi foran komputilon post malkonektado"),
        ("Insert", "Enmeti"),
        ("Insert Lock", "Ŝlosi foran komputilon"),
        ("Refresh", "Refreŝigi ekranon"),
        ("ID does not exist", "La identigilo ne ekzistas"),
        ("Failed to connect to rendezvous server", "Malsukcesis konekti al la servilo rendezvous"),
        ("Please try later", "Bonvolu provi poste"),
        ("Remote desktop is offline", "La fora aparato estas senkonektita"),
        ("Key mismatch", "Miskongruo de klavoj"),
        ("Timeout", "Konekta posttempo"),
        ("Failed to connect to relay server", "Malsukcesis konekti al la relajsa servilo"),
        ("Failed to connect via rendezvous server", "Malsukcesis konekti per servilo rendezvous"),
        ("Failed to connect via relay server", "Malsukcesis konekti per relajsa servilo"),
        ("Failed to make direct connection to remote desktop", "Malsukcesis konekti direkte"),
        ("Set Password", "Agordi pasvorton"),
        ("OS Password", "Pasvorto de la operaciumo"),
        ("install_tip", "Vi ne uzas instalita versio. Pro limigoj pro UAC, kiel aparato kontrolata, en kelkaj kazoj, ne estos ebla kontroli la muson kaj klavaron aŭ registri la ekranon. Bonvolu alkliku la butonon malsupre por instali RustDesk sur la operaciumo por eviti la demando supre."),
        ("Click to upgrade", "Alklaki por plibonigi"),
        ("Click to download", "Alklaki por elŝuti"),
        ("Click to update", "Alklaki por ĝisdatigi"),
        ("Configure", "Konfiguri"),
        ("config_acc", "Por uzi vian foran aparaton, bonvolu doni la permeson \"alirebleco\" al RustDesk."),
        ("config_screen", "Por uzi vian foran aparaton, bonvolu doni la permeson \"ekranregistrado\" al RustDesk."),
        ("Installing ...", "Instalante..."),
        ("Install", "Instali"),
        ("Installation", "Instalado"),
        ("Installation Path", "Vojo de instalo"),
        ("Create start menu shortcuts", "Aldoni ligilojn sur la startmenuo"),
        ("Create desktop icon", "Aldoni ligilojn sur la labortablo"),
        ("agreement_tip", "Starti la instaladon signifas akcepti la permesilon."),
        ("Accept and Install", "Akcepti kaj instali"),
        ("End-user license agreement", "Uzanta permesilon"),
        ("Generating ...", "Generante..."),
        ("Your installation is lower version.", "Via versio de instalaĵo estas pli malalta ol la lasta."),
        ("not_close_tcp_tip", "Bonvolu ne fermu tiun fenestron dum la uzo de la tunelo"),
        ("Listening ...", "Atendante konekton al la tunelo..."),
        ("Remote Host", "Fora gastiganto"),
        ("Remote Port", "Fora pordo"),
        ("Action", "Ago"),
        ("Add", "Aldoni"),
        ("Local Port", "Loka pordo"),
        ("setup_server_tip", "Se vi bezonas pli rapida konekcio, vi povas krei vian propran servilon"),
        ("Too short, at least 6 characters.", "Tro mallonga, almenaŭ 6 signoj."),
        ("The confirmation is not identical.", "Ambaŭ enigoj ne kongruas"),
        ("Permissions", "Permesoj"),
        ("Accept", "Akcepti"),
        ("Dismiss", "Malakcepti"),
        ("Disconnect", "Malkonekti"),
        ("Allow using keyboard and mouse", "Permesi la uzon de la klavaro kaj muso"),
        ("Allow using clipboard", "Permesi la uzon de la poŝo"),
        ("Allow hearing sound", "Permesi la uzon de la sono"),
        ("Allow file copy and paste", "Permesu kopii kaj alglui dosierojn"),
        ("Connected", "Konektata"),
        ("Direct and encrypted connection", "Konekcio direkta ĉifrata"),
        ("Relayed and encrypted connection", "Konekcio relajsa ĉifrata"),
        ("Direct and unencrypted connection", "Konekcio direkta neĉifrata"),
        ("Relayed and unencrypted connection", "Konekcio relajsa neĉifrata"),
        ("Enter Remote ID", "Tajpu foran identigilon"),
        ("Enter your password", "Tajpu vian pasvorton"),
        ("Logging in...", "Konektante..."),
        ("Enable RDP session sharing", "Ebligi la kundivido de sesio RDP"),
        ("Auto Login", "Aŭtomata konektado (la ŝloso nur estos ebligita post la malebligado de la unua parametro)"),
        ("Enable Direct IP Access", "Permesi direkta eniro per IP"),
        ("Rename", "Renomi"),
        ("Space", "Spaco"),
        ("Create Desktop Shortcut", "Krei ligilon sur la labortablon"),
        ("Change Path", "Ŝanĝi vojon"),
        ("Create Folder", "Krei dosierujon"),
        ("Please enter the folder name", "Bonvolu enigi la dosiernomon"),
        ("Fix it", "Riparu ĝin"),
        ("Warning", "Averto"),
        ("Login screen using Wayland is not supported", "Konektajn ekranojn uzantajn Wayland ne estas subtenitaj"),
        ("Reboot required", "Restarto deviga"),
        ("Unsupported display server ", "La aktuala bilda servilo ne estas subtenita"),
        ("x11 expected", "Bonvolu uzi x11"),
        ("Port", "Pordo"),
        ("Settings", "Agordoj"),
        ("Username", " Uzanta nomo"),
        ("Invalid port", "Pordo nevalida"),
        ("Closed manually by the peer", "Manuale fermita de la samtavolano"),
        ("Enable remote configuration modification", "Permesi foran redaktadon de la konfiguracio"),
        ("Run without install", "Plenumi sen instali"),
        ("Always connected via relay", "Ĉiam konektata per relajso"),
        ("Always connect via relay", "Ĉiam konekti per relajso"),
        ("whitelist_tip", "Nur la IP en la blanka listo povas kontroli mian komputilon"),
        ("Login", "Konekti"),
        ("Logout", "Malkonekti"),
        ("Tags", "Etikedi"),
        ("Search ID", "Serĉi ID"),
        ("Current Wayland display server is not supported", "La aktuala bilda servilo Wayland ne estas subtenita"),
        ("whitelist_sep", "Vi povas uzi komon, punktokomon, spacon aŭ linsalton kiel apartigilo"),
        ("Add ID", "Aldoni identigilo"),
        ("Add Tag", "Aldoni etikedo"),
        ("Unselect all tags", "Malselekti ĉiujn etikedojn"),
        ("Network error", "Reta eraro"),
        ("Username missed", "Uzantnomo forgesita"),
        ("Password missed", "Pasvorto forgesita"),
        ("Wrong credentials", "Identigilo aŭ pasvorto erara"),
        ("Edit Tag", "Redakti etikedo"),
        ("Unremember Password", "Forgesi pasvorton"),
        ("Favorites", "Favorataj"),
        ("Add to Favorites", "Aldoni al la favorataj"),
        ("Remove from Favorites", "Forigi el la favorataj"),
        ("Empty", "Malplena"),
        ("Invalid folder name", "Dosiernomo nevalida"),
        ("Socks5 Proxy", "Socks5 prokura servilo"),
        ("Hostname", "Nomo de gastiga"),
        ("Discovered", "Malkovritaj"),
        ("install_daemon_tip", ""),
        ("Remote ID", "Fora identigilo"),
        ("Paste", "Alglui"),
        ("Paste here?", ""),
        ("Are you sure to close the connection?", "Ĉu vi vere volas fermi la konekton?"),
        ("Download new version", "Elŝuti la novan version"),
        ("Touch mode", "Tuŝa modo"),
        ("Mouse mode", ""),
        ("One-Finger Tap", ""),
        ("Left Mouse", ""),
        ("One-Long Tap", ""),
        ("Two-Finger Tap", ""),
        ("Right Mouse", ""),
        ("One-Finger Move", ""),
        ("Double Tap & Move", ""),
        ("Mouse Drag", ""),
        ("Three-Finger vertically", ""),
        ("Mouse Wheel", ""),
        ("Two-Finger Move", ""),
        ("Canvas Move", ""),
        ("Pinch to Zoom", ""),
        ("Canvas Zoom", ""),
        ("Reset canvas", "Restarigi kanvaso"),
        ("No permission of file transfer", "Neniu permeso de dosiertransigo"),
        ("Note", "Notu"),
        ("Connection", ""),
        ("Share Screen", ""),
        ("CLOSE", ""),
        ("OPEN", ""),
        ("Chat", ""),
        ("Total", ""),
        ("items", ""),
        ("Selected", ""),
        ("Screen Capture", ""),
        ("Input Control", ""),
        ("Audio Capture", ""),
        ("File Connection", ""),
        ("Screen Connection", ""),
        ("Do you accept?", ""),
        ("Open System Setting", ""),
        ("How to get Android input permission?", ""),
        ("android_input_permission_tip1", ""),
        ("android_input_permission_tip2", ""),
        ("android_new_connection_tip", ""),
        ("android_service_will_start_tip", ""),
        ("android_stop_service_tip", ""),
        ("android_version_audio_tip", ""),
        ("android_start_service_tip", ""),
        ("Account", ""),
        ("Overwrite", ""),
        ("This file exists, skip or overwrite this file?", ""),
        ("Quit", ""),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", ""),
        ("Failed", ""),
        ("Succeeded", ""),
        ("Someone turns on privacy mode, exit", ""),
        ("Unsupported", ""),
        ("Peer denied", ""),
        ("Please install plugins", ""),
        ("Peer exit", ""),
        ("Failed to turn off", ""),
        ("Turned off", ""),
        ("In privacy mode", ""),
        ("Out privacy mode", ""),
        ("Language", ""),
        ("Keep RustDesk background service", ""),
        ("Ignore Battery Optimizations", ""),
        ("android_open_battery_optimizations_tip", ""),
        ("Connection not allowed", ""),
        ("Use temporary password", ""),
        ("Use permanent password", ""),
        ("Use both passwords", ""),
        ("Set permanent password", ""),
        ("Set temporary password length", ""),
        ("Enable Remote Restart", ""),
        ("Allow remote restart", ""),
        ("Restart Remote Device", ""),
        ("Are you sure you want to restart", ""),
        ("Restarting Remote Device", ""),
        ("remote_restarting_tip", ""),
        ("Copied", ""),
        ("Exit Fullscreen", "Eliru Plenekranon"),
        ("Fullscreen", "Plenekrane"),
        ("Mobile Actions", "Poŝtelefonaj Agoj"),
        ("Select Monitor", "Elektu Monitoron"),
        ("Control Actions", "Kontrolaj Agoj"),
        ("Display Settings", "Montraj Agordoj"),
        ("Ratio", "Proporcio"),
        ("Image Quality", "Bilda Kvalito"),
        ("Scroll Style", "Ruluma Stilo"),
        ("Show Menubar", "Montru menubreton"),
        ("Hide Menubar", "kaŝi menubreton"),
    ].iter().cloned().collect();
}
