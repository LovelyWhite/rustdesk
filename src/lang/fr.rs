lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Statut"),
        ("Your Desktop", "Votre bureau"),
        ("desk_tip", "Votre bureau est accessible via l'identifiant et le mot de passe ci-dessous."),
        ("Password", "Mot de passe"),
        ("Ready", "Prêt"),
        ("Established", "Établi"),
        ("connecting_status", "Connexion au réseau RustDesk..."),
        ("Enable Service", "Autoriser le service"),
        ("Start Service", "Démarrer le service"),
        ("Service is running", "Le service est en cours d'exécution"),
        ("Service is not running", "Le service ne fonctionne pas"),
        ("not_ready_status", "Pas prêt, veuillez vérifier la connexion réseau"),
        ("Control Remote Desktop", "Contrôler le bureau à distance"),
        ("Transfer File", "Transférer le fichier"),
        ("Connect", "Connecter"),
        ("Recent Sessions", "Sessions récentes"),
        ("Address Book", "Carnet d'adresses"),
        ("Confirmation", "Confirmation"),
        ("TCP Tunneling", "Tunneling TCP"),
        ("Remove", "Supprimer"),
        ("Refresh random password", "Actualiser le mot de passe aléatoire"),
        ("Set your own password", "Définir votre propre mot de passe"),
        ("Enable Keyboard/Mouse", "Activer le contrôle clavier/souris"),
        ("Enable Clipboard", "Activer la synchronisation du presse-papier"),
        ("Enable File Transfer", "Activer le transfert de fichiers"),
        ("Enable TCP Tunneling", "Activer le tunneling TCP"),
        ("IP Whitelisting", "Liste blanche IP"),
        ("ID/Relay Server", "ID/Serveur Relais"),
        ("Stop service", "Arrêter le service"),
        ("Change ID", "Changer d'ID"),
        ("Website", "Site Web"),
        ("About", "À propos de"),
        ("Mute", "Muet"),
        ("Audio Input", "Entrée audio"),
        ("Enhancements", ""),
        ("Hardware Codec", ""),
        ("Adaptive Bitrate", ""),
        ("ID Server", "Serveur ID"),
        ("Relay Server", "Serveur relais"),
        ("API Server", "Serveur API"),
        ("invalid_http", "Doit commencer par http:// ou https://"),
        ("Invalid IP", "IP invalide"),
        ("id_change_tip", "Seules les lettres a-z, A-Z, 0-9, _ (trait de soulignement) peuvent être utilisées. La première lettre doit être a-z, A-Z. La longueur doit être comprise entre 6 et 16."),
        ("Invalid format", "Format invalide"),
        ("server_not_support", "Pas encore supporté par le serveur"),
        ("Not available", "Indisponible"),
        ("Too frequent", "Modifié trop fréquemment, veuillez réessayer plus tard"),
        ("Cancel", "Annuler"),
        ("Skip", "Ignorer"),
        ("Close", "Fermer"),
        ("Retry", "Réessayer"),
        ("OK", "Confirmer"),
        ("Password Required", "Mot de passe requis"),
        ("Please enter your password", "Veuillez saisir votre mot de passe"),
        ("Remember password", "Mémoriser le mot de passe"),
        ("Wrong Password", "Mauvais mot de passe"),
        ("Do you want to enter again?", "Voulez-vous participer à nouveau ?"),
        ("Connection Error", "Erreur de connexion"),
        ("Error", "Erreur"),
        ("Reset by the peer", "La connexion a été fermée par le pair"),
        ("Connecting...", "Connexion..."),
        ("Connection in progress. Please wait.", "Connexion en cours. Veuillez patienter."),
        ("Please try 1 minute later", "Réessayez dans une minute"),
        ("Login Error", "Erreur de connexion"),
        ("Successful", "Succès"),
        ("Connected, waiting for image...", "Connecté, en attente de transmission d'image..."),
        ("Name", "Nom"),
        ("Type", "Taper"),
        ("Modified", "Modifié"),
        ("Size", "Taille"),
        ("Show Hidden Files", "Afficher les fichiers cachés"),
        ("Receive", "Accepter"),
        ("Send", "Envoyer"),
        ("Refresh File", "Actualiser le fichier"),
        ("Local", "Local"),
        ("Remote", "Distant"),
        ("Remote Computer", "Ordinateur distant"),
        ("Local Computer", "Ordinateur local"),
        ("Confirm Delete", "Confirmer la suppression"),
        ("Delete", "Supprimer"),
        ("Properties", "Propriétés"),
        ("Multi Select", "Choix multiple"),
        ("Empty Directory", "Répertoire vide"),
        ("Not an empty directory", "Pas un répertoire vide"),
        ("Are you sure you want to delete this file?", "Voulez-vous vraiment supprimer ce fichier?"),
        ("Are you sure you want to delete this empty directory?", "Voulez-vous vraiment supprimer ce répertoire vide ?"),
        ("Are you sure you want to delete the file of this directory?", "Voulez-vous vraiment supprimer le fichier de ce répertoire ?"),
        ("Do this for all conflicts", "Appliquer à d'autres conflits"),
        ("This is irreversible!", "C'est irréversible !"),
        ("Deleting", "Suppression"),
        ("files", "fichier"),
        ("Waiting", "En attente..."),
        ("Finished", "Terminé"),
        ("Speed", "Vitesse"),
        ("Custom Image Quality", "Définir la qualité d'image"),
        ("Privacy mode", "Mode privé"),
        ("Block user input", "Bloquer la saisie de l'utilisateur"),
        ("Unblock user input", "Débloquer l'entrée de l'utilisateur"),
        ("Adjust Window", "Ajuster la fenêtre"),
        ("Original", "Ratio d'origine"),
        ("Shrink", "Rétrécir"),
        ("Stretch", "Étirer"),
        ("Good image quality", "Bonne qualité d'image"),
        ("Balanced", "Qualité d'image normale"),
        ("Optimize reaction time", "Optimiser le temps de réaction"),
        ("Custom", "Qualité d'image personnalisée"),
        ("Show remote cursor", "Afficher le curseur distant"),
        ("Show quality monitor", ""),
        ("Disable clipboard", "Désactiver le presse-papier"),
        ("Lock after session end", "Verrouiller l'ordinateur distant après la déconnexion"),
        ("Insert", "Insérer"),
        ("Insert Lock", "Verrouiller l'ordinateur distant"),
        ("Refresh", "Rafraîchir l'écran"),
        ("ID does not exist", "L'ID n'existe pas"),
        ("Failed to connect to rendezvous server", "Échec de la connexion au serveur rendezvous"),
        ("Please try later", "Veuillez essayer plus tard"),
        ("Remote desktop is offline", "Le bureau à distance est hors ligne"),
        ("Key mismatch", "Discordance de clés"),
        ("Timeout", "Connexion expirée"),
        ("Failed to connect to relay server", "Échec de la connexion au serveur relais"),
        ("Failed to connect via rendezvous server", "Échec de l'établissement d'une connexion via le serveur rendezvous"),
        ("Failed to connect via relay server", "Impossible d'établir une connexion via le serveur relais"),
        ("Failed to make direct connection to remote desktop", "Impossible d'établir une connexion directe"),
        ("Set Password", "Définir le mot de passe"),
        ("OS Password", "Mot de passe du système d'exploitation"),
        ("install_tip", "Vous utilisez une version non installée. En raison des restrictions UAC, en tant que terminal contrôlé, dans certains cas, il ne sera pas en mesure de contrôler la souris et le clavier ou d'enregistrer l'écran. Veuillez cliquer sur le bouton ci-dessous pour installer RustDesk au système pour éviter la question ci-dessus."),
        ("Click to upgrade", "Cliquez pour mettre à niveau"),
        ("Click to download", "Cliquez pour télécharger"),
        ("Click to update", "Cliquez pour mettre à jour"),
        ("Configure", "Configurer"),
        ("config_acc", "Afin de pouvoir contrôler votre bureau à distance, veuillez donner l'autorisation \"accessibilité\" à RustDesk."),
        ("config_screen", "Afin de pouvoir accéder à votre bureau à distance, veuillez donner à RustDesk l'autorisation \"enregistrement d'écran\"."),
        ("Installing ...", "Installation..."),
        ("Install", "Installer"),
        ("Installation", "Installation"),
        ("Installation Path", "Chemin d'installation"),
        ("Create start menu shortcuts", "Créer des raccourcis dans le menu démarrer"),
        ("Create desktop icon", "Créer une icône sur le bureau"),
        ("agreement_tip", "Démarrer l'installation signifie accepter le contrat de licence."),
        ("Accept and Install", "Accepter et installer"),
        ("End-user license agreement", "Contrat d'utilisateur"),
        ("Generating ...", "Génération..."),
        ("Your installation is lower version.", "La version que vous avez installée est inférieure à la version en cours d'exécution."),
        ("not_close_tcp_tip", "Veuillez ne pas fermer cette fenêtre lors de l'utilisation du tunnel"),
        ("Listening ...", "En attente de connexion tunnel..."),
        ("Remote Host", "Hôte distant"),
        ("Remote Port", "Port distant"),
        ("Action", "Action"),
        ("Add", "Ajouter"),
        ("Local Port", "Port local"),
        ("setup_server_tip", "Si vous avez besoin d'une vitesse de connexion plus rapide, vous pouvez choisir de créer votre propre serveur"),
        ("Too short, at least 6 characters.", "Trop court, au moins 6 caractères."),
        ("The confirmation is not identical.", "Les deux entrées ne correspondent pas"),
        ("Permissions", "Autorisations"),
        ("Accept", "Accepter"),
        ("Dismiss", "Rejeter"),
        ("Disconnect", "Déconnecter"),
        ("Allow using keyboard and mouse", "Autoriser l'utilisation du clavier et de la souris"),
        ("Allow using clipboard", "Autoriser l'utilisation du presse-papier"),
        ("Allow hearing sound", "Autoriser l'audition du son"),
        ("Allow file copy and paste", "Autoriser le copier-coller de fichiers"),
        ("Connected", "Connecté"),
        ("Direct and encrypted connection", "Connexion directe chiffrée"),
        ("Relayed and encrypted connection", "Connexion relais chiffrée"),
        ("Direct and unencrypted connection", "Connexion directe non chiffrée"),
        ("Relayed and unencrypted connection", "Connexion relais non chiffrée"),
        ("Enter Remote ID", "Entrez l'ID de l'appareil à distance"),
        ("Enter your password", "Entrez votre mot de passe"),
        ("Logging in...", "Se connecter..."),
        ("Enable RDP session sharing", "Activer le partage de session RDP"),
        ("Auto Login", "Connexion automatique (le verrouillage ne sera effectif qu'après la désactivation du premier paramètre)"),
        ("Enable Direct IP Access", "Autoriser l'accès direct par IP"),
        ("Rename", "Renommer"),
        ("Space", "Espace"),
        ("Create Desktop Shortcut", "Créer un raccourci sur le bureau"),
        ("Change Path", "Changer de chemin"),
        ("Create Folder", "Créer un dossier"),
        ("Please enter the folder name", "Veuillez saisir le nom du dossier"),
        ("Fix it", "Réparez-le"),
        ("Warning", "Avertissement"),
        ("Login screen using Wayland is not supported", "L'écran de connexion utilisant Wayland n'est pas pris en charge"),
        ("Reboot required", "Redémarrage pour prendre effet"),
        ("Unsupported display server ", "Le serveur d'affichage actuel n'est pas pris en charge"),
        ("x11 expected", "Veuillez passer à x11"),
        ("Port", "Port"),
        ("Settings", "Paramètres"),
        ("Username", " Nom d'utilisateur"),
        ("Invalid port", "Port invalide"),
        ("Closed manually by the peer", "Fermé manuellement par le pair"),
        ("Enable remote configuration modification", "Autoriser la modification de la configuration à distance"),
        ("Run without install", "Exécuter sans installer"),
        ("Always connected via relay", "Forcer la connexion relais"),
        ("Always connect via relay", "Forcer la connexion relais"),
        ("whitelist_tip", "Seul l'IP dans la liste blanche peut accéder à mon appareil"),
        ("Login", "Connexion"),
        ("Logout", "Déconnexion"),
        ("Tags", "Étiqueter"),
        ("Search ID", "Rechercher un ID"),
        ("Current Wayland display server is not supported", "Le serveur d'affichage Wayland n'est pas pris en charge"),
        ("whitelist_sep", "Vous pouvez utiliser une virgule, un point-virgule, un espace ou une nouvelle ligne comme séparateur"),
        ("Add ID", "Ajouter un ID"),
        ("Add Tag", "Ajouter une balise"),
        ("Unselect all tags", "Désélectionner toutes les balises"),
        ("Network error", "Erreur réseau"),
        ("Username missed", "Nom d'utilisateur manqué"),
        ("Password missed", "Mot de passe manqué"),
        ("Wrong credentials", "Identifiant ou mot de passe erroné"),
        ("Edit Tag", "Modifier la balise"),
        ("Unremember Password", "Mot de passe oublié"),
        ("Favorites", "Favoris"),
        ("Add to Favorites", "Ajouter aux Favoris"),
        ("Remove from Favorites", "Retirer des favoris"),
        ("Empty", "Vide"),
        ("Invalid folder name", "Nom de dossier invalide"),
        ("Socks5 Proxy", "Socks5 Agents"),
        ("Hostname", "Nom d'hôte"),
        ("Discovered", "Découvert"),
        ("install_daemon_tip", "Pour démarrer au démarrage, vous devez installer le service système."),
        ("Remote ID", "ID de l'appareil à distance"),
        ("Paste", "Coller"),
        ("Paste here?", "Coller ici ?"),
        ("Are you sure to close the connection?", "Êtes-vous sûr de fermer la connexion?"),
        ("Download new version", "Télécharger la nouvelle version"),
        ("Touch mode", "Mode tactile"),
        ("Mouse mode", "Mode souris"),
        ("One-Finger Tap", "Tapez d'un doigt"),
        ("Left Mouse", "Souris gauche"),
        ("One-Long Tap", "Un long robinet"),
        ("Two-Finger Tap", "Tapez à deux doigts"),
        ("Right Mouse", "Bouton droit de la souris"),
        ("One-Finger Move", "Mouvement à un doigt"),
        ("Double Tap & Move", "Appuyez deux fois et déplacez"),
        ("Mouse Drag", "Glissement de la souris"),
        ("Three-Finger vertically", "Trois doigts verticalement"),
        ("Mouse Wheel", "Roulette de la souris"),
        ("Two-Finger Move", "Mouvement à deux doigts"),
        ("Canvas Move", "Déplacement de la toile"),
        ("Pinch to Zoom", "Pincer pour zoomer"),
        ("Canvas Zoom", "Zoom sur la toile"),
        ("Reset canvas", "Réinitialiser le canevas"),
        ("No permission of file transfer", "Aucune autorisation de transfert de fichiers"),
        ("Note", "Noter"),
        ("Connection", "Connexion"),
        ("Share Screen", "Partager l'écran"),
        ("CLOSE", "FERMER"),
        ("OPEN", "OUVRIR"),
        ("Chat", "Discuter"),
        ("Total", "Total"),
        ("items", "éléments"),
        ("Selected", "Choisi"),
        ("Screen Capture", "Capture d'écran"),
        ("Input Control", "Contrôle de saisie"),
        ("Audio Capture", "Capture audio"),
        ("File Connection", "Connexion de fichier"),
        ("Screen Connection", "Connexion de l'écran"),
        ("Do you accept?", "Accepter vous ?"),
        ("Open System Setting", "Ouvrir les paramètres système"),
        ("How to get Android input permission?", "Comment obtenir l'autorisation d'entrée Android ?"),
        ("android_input_permission_tip1", "Pour qu'un appareil distant puisse contrôler votre appareil Android via la souris ou le toucher, vous devez autoriser RustDesk à utiliser le service \"Accessibilité\"."),
        ("android_input_permission_tip2", "Veuillez accéder à la page suivante des paramètres système, recherchez et entrez [Services installés], activez le service [RustDesk Input]."),
        ("android_new_connection_tip", "Une nouvelle demande de contrôle a été reçue, elle souhaite contrôler votre appareil actuel."),
        ("android_service_will_start_tip", "L'activation de la capture d'écran démarrera automatiquement le service, permettant à d'autres appareils de demander une connexion à partir de cet appareil."),
        ("android_stop_service_tip", "La fermeture du service fermera automatiquement toutes les connexions établies."),
        ("android_version_audio_tip", "La version actuelle d'Android ne prend pas en charge la capture audio, veuillez passer à Android 10 ou supérieur."),
        ("android_start_service_tip", "Appuyez sur [Démarrer le service] ou sur l'autorisation OUVRIR [Capture d'écran] pour démarrer le service de partage d'écran."),
        ("Account", "Compte"),
        ("Overwrite", "Écraser"),
        ("This file exists, skip or overwrite this file?", "Ce fichier existe, ignorer ou écraser ce fichier ?"),
        ("Quit", "Quitter"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Aider"),
        ("Failed", "échouer"),
        ("Succeeded", "Succès"),
        ("Someone turns on privacy mode, exit", "Quelqu'un active le mode de confidentialité, quittez"),
        ("Unsupported", "Non pris en charge"),
        ("Peer denied", "Pair refusé"),
        ("Please install plugins", "Veuillez installer les plugins"),
        ("Peer exit", "Sortie des pairs"),
        ("Failed to turn off", "Échec de la désactivation"),
        ("Turned off", "Éteindre"),
        ("In privacy mode", "en mode privé"),
        ("Out privacy mode", "hors mode de confidentialité"),
        ("Language", "Langue"),
    ].iter().cloned().collect();
}
