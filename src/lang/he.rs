lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", ""),
        ("Your Desktop", ""),
        ("desk_tip", "ניתן לגשת לשולחן העבודה שלך עם מזהה וסיסמה זו."),
        ("Password", ""),
        ("Ready", ""),
        ("Established", ""),
        ("connecting_status", "מתחבר לרשת Yangdiskservice..."),
        ("Enable service", ""),
        ("Start service", ""),
        ("Service is running", ""),
        ("Service is not running", ""),
        ("not_ready_status", "לא מוכן. בדוק את החיבור שלך"),
        ("Control Remote Desktop", ""),
        ("Transfer file", ""),
        ("Connect", ""),
        ("Recent sessions", ""),
        ("Address book", ""),
        ("Confirmation", ""),
        ("TCP tunneling", ""),
        ("Remove", ""),
        ("Refresh random password", ""),
        ("Set your own password", ""),
        ("Enable keyboard/mouse", ""),
        ("Enable clipboard", ""),
        ("Enable file transfer", ""),
        ("Enable TCP tunneling", ""),
        ("IP Whitelisting", ""),
        ("ID/Relay Server", "שרת מזהה/ריליי"),
        ("Import server config", ""),
        ("Export Server Config", ""),
        ("Import server configuration successfully", ""),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", ""),
        ("Clipboard is empty", ""),
        ("Stop service", ""),
        ("Change ID", ""),
        ("Your new ID", ""),
        ("length %min% to %max%", ""),
        ("starts with a letter", ""),
        ("allowed characters", ""),
        ("id_change_tip", "מותרים רק תווים a-z, A-Z, 0-9 ו_ (קו תחתון). האות הראשונה חייבת להיות a-z, A-Z. אורך בין 6 ל-16."),
        ("Website", ""),
        ("About", ""),
        ("Slogan_tip", "נוצר בלב בעולם הזה הכאוטי!"),
        ("Privacy Statement", ""),
        ("Mute", ""),
        ("Build Date", "תאריך בנייה"),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "קלט שמע"),
        ("Enhancements", ""),
        ("Hardware Codec", "קודק חומרה"),
        ("Adaptive bitrate", ""),
        ("ID Server", "שרת מזהה"),
        ("Relay Server", "שרת ריליי"),
        ("API Server", "שרת API"),
        ("invalid_http", "חייב להתחיל עם http:// או https://"),
        ("Invalid IP", ""),
        ("Invalid format", ""),
        ("server_not_support", "עדיין לא נתמך על ידי השרת"),
        ("Not available", ""),
        ("Too frequent", ""),
        ("Cancel", ""),
        ("Skip", ""),
        ("Close", ""),
        ("Retry", ""),
        ("OK", ""),
        ("Password Required", "נדרשת סיסמה"),
        ("Please enter your password", ""),
        ("Remember password", ""),
        ("Wrong Password", "סיסמה שגויה"),
        ("Do you want to enter again?", ""),
        ("Connection Error", "שגיאת חיבור"),
        ("Error", ""),
        ("Reset by the peer", ""),
        ("Connecting...", ""),
        ("Connection in progress. Please wait.", ""),
        ("Please try 1 minute later", ""),
        ("Login Error", "שגיאת התחברות"),
        ("Successful", ""),
        ("Connected, waiting for image...", ""),
        ("Name", ""),
        ("Type", ""),
        ("Modified", ""),
        ("Size", ""),
        ("Show Hidden Files", "הצג קבצים נסתרים"),
        ("Receive", ""),
        ("Send", ""),
        ("Refresh File", "רענן קובץ"),
        ("Local", ""),
        ("Remote", ""),
        ("Remote Computer", "מחשב מרוחק"),
        ("Local Computer", "מחשב מקומי"),
        ("Confirm Delete", "אשר מחיקה"),
        ("Delete", ""),
        ("Properties", ""),
        ("Multi Select", "בחירה מרובה"),
        ("Select All", "בחר הכל"),
        ("Unselect All", "בטל בחירת הכל"),
        ("Empty Directory", "תיקייה ריקה"),
        ("Not an empty directory", ""),
        ("Are you sure you want to delete this file?", ""),
        ("Are you sure you want to delete this empty directory?", ""),
        ("Are you sure you want to delete the file of this directory?", ""),
        ("Do this for all conflicts", ""),
        ("This is irreversible!", ""),
        ("Deleting", ""),
        ("files", ""),
        ("Waiting", ""),
        ("Finished", ""),
        ("Speed", ""),
        ("Custom Image Quality", "איכות תמונה מותאמת אישית"),
        ("Privacy mode", ""),
        ("Block user input", ""),
        ("Unblock user input", ""),
        ("Adjust Window", "התאם חלון"),
        ("Original", ""),
        ("Shrink", ""),
        ("Stretch", ""),
        ("Scrollbar", ""),
        ("ScrollAuto", ""),
        ("Good image quality", ""),
        ("Balanced", ""),
        ("Optimize reaction time", ""),
        ("Custom", ""),
        ("Show remote cursor", ""),
        ("Show quality monitor", ""),
        ("Disable clipboard", ""),
        ("Lock after session end", ""),
        ("Insert Ctrl + Alt + Del", ""),
        ("Insert Lock", "הוסף נעילה"),
        ("Refresh", ""),
        ("ID does not exist", ""),
        ("Failed to connect to rendezvous server", ""),
        ("Please try later", ""),
        ("Remote desktop is offline", ""),
        ("Key mismatch", ""),
        ("Timeout", ""),
        ("Failed to connect to relay server", ""),
        ("Failed to connect via rendezvous server", ""),
        ("Failed to connect via relay server", ""),
        ("Failed to make direct connection to remote desktop", ""),
        ("Set Password", "הגדר סיסמה"),
        ("OS Password", "סיסמת מערכת הפעלה"),
        ("install_tip", "בגלל UAC, Yangdiskservice לא יכול לפעול כראוי כצד מרוחק בחלק מהמקרים. כדי להימנע מ-UAC, אנא לחץ על הכפתור למטה כדי להתקין את Yangdiskservice במערכת."),
        ("Click to upgrade", ""),
        ("Click to download", ""),
        ("Click to update", ""),
        ("Configure", ""),
        ("config_acc", "כדי לשלוט מרחוק בשולחן העבודה שלך, עליך להעניק ל-Yangdiskservice הרשאות \"נגישות\"."),
        ("config_screen", "כדי לגשת מרחוק לשולחן העבודה שלך, עליך להעניק ל-Yangdiskservice הרשאות \"הקלטת מסך\"."),
        ("Installing ...", ""),
        ("Install", ""),
        ("Installation", ""),
        ("Installation Path", "נתיב התקנה"),
        ("Create start menu shortcuts", ""),
        ("Create desktop icon", ""),
        ("agreement_tip", "על ידי התחלת ההתקנה, אתה מקבל את הסכם הרישיון."),
        ("Accept and Install", "קבל והתקן"),
        ("End-user license agreement", ""),
        ("Generating ...", ""),
        ("Your installation is lower version.", ""),
        ("not_close_tcp_tip", "אל תסגור חלון זה בזמן שאתה משתמש במנהרה"),
        ("Listening ...", ""),
        ("Remote Host", "מארח מרוחק"),
        ("Remote Port", "פורט מרוחק"),
        ("Action", ""),
        ("Add", ""),
        ("Local Port", "פורט מקומי"),
        ("Local Address", "כתובת מקומית"),
        ("Change Local Port", "שנה פורט מקומי"),
        ("setup_server_tip", "לחיבור מהיר יותר, אנא הגדר שרת משלך"),
        ("Too short, at least 6 characters.", ""),
        ("The confirmation is not identical.", ""),
        ("Permissions", ""),
        ("Accept", ""),
        ("Dismiss", ""),
        ("Disconnect", ""),
        ("Enable file copy and paste", ""),
        ("Connected", ""),
        ("Direct and encrypted connection", ""),
        ("Relayed and encrypted connection", ""),
        ("Direct and unencrypted connection", ""),
        ("Relayed and unencrypted connection", ""),
        ("Enter Remote ID", "הזן מזהה מרוחק"),
        ("Enter your password", ""),
        ("Logging in...", ""),
        ("Enable RDP session sharing", ""),
        ("Auto Login", "התחברות אוטומטית (תקפה רק אם הגדרת \"נעל לאחר סיום הסשן\")"),
        ("Enable direct IP access", ""),
        ("Rename", ""),
        ("Space", ""),
        ("Create desktop shortcut", ""),
        ("Change Path", "שנה נתיב"),
        ("Create Folder", "צור תיקייה"),
        ("Please enter the folder name", ""),
        ("Fix it", ""),
        ("Warning", ""),
        ("Login screen using Wayland is not supported", ""),
        ("Reboot required", ""),
        ("Unsupported display server", ""),
        ("x11 expected", ""),
        ("Port", ""),
        ("Settings", ""),
        ("Username", ""),
        ("Invalid port", ""),
        ("Closed manually by the peer", ""),
        ("Enable remote configuration modification", ""),
        ("Run without install", ""),
        ("Connect via relay", ""),
        ("Always connect via relay", ""),
        ("whitelist_tip", "רק IP ברשימה הלבנה יכול לגשת אלי"),
        ("Login", ""),
        ("Verify", ""),
        ("Remember me", ""),
        ("Trust this device", ""),
        ("Verification code", ""),
        ("verification_tip", "קוד אימות נשלח לכתובת הדוא\"ל הרשומה, הזן את קוד האימות כדי להמשיך בהתחברות."),
        ("Logout", ""),
        ("Tags", ""),
        ("Search ID", ""),
        ("whitelist_sep", "מופרד על ידי פסיק, נקודה פסיק, רווחים או שורה חדשה"),
        ("Add ID", ""),
        ("Add Tag", "הוסף תג"),
        ("Unselect all tags", ""),
        ("Network error", ""),
        ("Username missed", ""),
        ("Password missed", ""),
        ("Wrong credentials", "שם משתמש או סיסמה שגויים"),
        ("The verification code is incorrect or has expired", ""),
        ("Edit Tag", "ערוך תג"),
        ("Forget Password", "שכחת סיסמה"),
        ("Favorites", ""),
        ("Add to Favorites", "הוסף למועדפים"),
        ("Remove from Favorites", "הסר מהמועדפים"),
        ("Empty", ""),
        ("Invalid folder name", ""),
        ("Socks5 Proxy", "פרוקסי Socks5"),
        ("Socks5/Http(s) Proxy", "פרוקסי Socks5/Http(s)"),
        ("Discovered", ""),
        ("install_daemon_tip", "לצורך הפעלה בעת הפעלת המחשב, עליך להתקין שירות מערכת."),
        ("Remote ID", ""),
        ("Paste", ""),
        ("Paste here?", ""),
        ("Are you sure to close the connection?", "האם אתה בטוח שברצונך לסגור את החיבור?"),
        ("Download new version", ""),
        ("Touch mode", ""),
        ("Mouse mode", ""),
        ("One-Finger Tap", "הקשה באצבע אחת"),
        ("Left Mouse", "עכבר שמאלי"),
        ("One-Long Tap", "הקשה ארוכה באצבע אחת"),
        ("Two-Finger Tap", "הקשה בשתי אצבעות"),
        ("Right Mouse", "עכבר ימני"),
        ("One-Finger Move", "הזזה באצבע אחת"),
        ("Double Tap & Move", "הקשה כפולה והזזה"),
        ("Mouse Drag", "גרירת עכבר"),
        ("Three-Finger vertically", "שלוש אצבעות אנכית"),
        ("Mouse Wheel", "גלגלת עכבר"),
        ("Two-Finger Move", "הזזה בשתי אצבעות"),
        ("Canvas Move", "הזזת בד"),
        ("Pinch to Zoom", "צביטה לזום"),
        ("Canvas Zoom", "זום בד"),
        ("Reset canvas", ""),
        ("No permission of file transfer", ""),
        ("Note", ""),
        ("Connection", ""),
        ("Share Screen", "שיתוף מסך"),
        ("Chat", ""),
        ("Total", ""),
        ("items", ""),
        ("Selected", ""),
        ("Screen Capture", "לכידת מסך"),
        ("Input Control", "בקרת קלט"),
        ("Audio Capture", "לכידת שמע"),
        ("File Connection", "חיבור קובץ"),
        ("Screen Connection", "חיבור מסך"),
        ("Do you accept?", ""),
        ("Open System Setting", "פתח הגדרת מערכת"),
        ("How to get Android input permission?", ""),
        ("android_input_permission_tip1", "כדי שמכשיר מרוחק יוכל לשלוט במכשיר האנדרואיד שלך באמצעות עכבר או מגע, עליך לאפשר ל-Yangdiskservice להשתמש בשירות \"נגישות\"."),
        ("android_input_permission_tip2", "אנא עבור לדף ההגדרות של המערכת הבא, מצא והכנס ל[שירותים מותקנים], הפעל את שירות [Yangdiskservice Input]."),
        ("android_new_connection_tip", "בקשת שליטה חדשה התקבלה, שרוצה לשלוט במכשירך הנוכחי."),
        ("android_service_will_start_tip", "הפעלת \"לכידת מסך\" תתחיל אוטומטית את השירות, מאפשרת למכשירים אחרים לבקש חיבור למכשיר שלך."),
        ("android_stop_service_tip", "סגירת השירות תסגור אוטומטית את כל החיבורים המוקמים."),
        ("android_version_audio_tip", "גרסת האנדרואיד הנוכחית אינה תומכת בלכידת שמע, אנא שדרג לאנדרואיד 10 או גבוה יותר."),
        ("android_start_service_tip", "הקש על [התחל שירות] או אפשר הרשאת [לכידת מסך] כדי להתחיל את שירות שיתוף המסך."),
        ("android_permission_may_not_change_tip", "הרשאות עבור חיבורים שנוצרו עשויות לא להשתנות מייד עד להתחברות מחדש."),
        ("Account", ""),
        ("Overwrite", ""),
        ("This file exists, skip or overwrite this file?", ""),
        ("Quit", ""),
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
        ("Language", ""),
        ("Keep Yangdiskservice background service", ""),
        ("Ignore Battery Optimizations", "התעלם מאופטימיזציות סוללה"),
        ("android_open_battery_optimizations_tip", "אם ברצונך לבטל תכונה זו, אנא עבור לדף ההגדרות של יישום Yangdiskservice הבא, מצא והכנס ל[סוללה], הסר את הסימון מ-[לא מוגבל]"),
        ("Start on boot", ""),
        ("Start the screen sharing service on boot, requires special permissions", ""),
        ("Connection not allowed", ""),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", ""),
        ("Use both passwords", ""),
        ("Set permanent password", ""),
        ("Enable remote restart", ""),
        ("Restart remote device", ""),
        ("Are you sure you want to restart", ""),
        ("Restarting remote device", ""),
        ("remote_restarting_tip", "המכשיר המרוחק מתחיל מחדש, אנא סגור את תיבת ההודעה הזו והתחבר מחדש עם סיסמה קבועה לאחר זמן מה"),
        ("Copied", ""),
        ("Exit Fullscreen", "יציאה ממסך מלא"),
        ("Fullscreen", ""),
        ("Mobile Actions", "פעולות ניידות"),
        ("Select Monitor", "בחר מסך"),
        ("Control Actions", "פעולות בקרה"),
        ("Display Settings", "הגדרות תצוגה"),
        ("Ratio", ""),
        ("Image Quality", "איכות תמונה"),
        ("Scroll Style", "סגנון גלילה"),
        ("Show Toolbar", "הצג סרגל כלים"),
        ("Hide Toolbar", "הסתר סרגל כלים"),
        ("Direct Connection", "חיבור ישיר"),
        ("Relay Connection", "חיבור ריליי"),
        ("Secure Connection", "חיבור מאובטח"),
        ("Insecure Connection", "חיבור לא מאובטח"),
        ("Scale original", ""),
        ("Scale adaptive", ""),
        ("General", ""),
        ("Security", ""),
        ("Theme", ""),
        ("Dark Theme", "ערכת נושא כהה"),
        ("Light Theme", "ערכת נושא בהירה"),
        ("Dark", ""),
        ("Light", ""),
        ("Follow System", "עקוב אחר המערכת"),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", "פתח הגדרות אבטחה"),
        ("Enable audio", ""),
        ("Unlock Network Settings", "פתח הגדרות רשת"),
        ("Server", ""),
        ("Direct IP Access", "גישה ישירה ל-IP"),
        ("Proxy", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", "מכשיר קלט שמע"),
        ("Use IP Whitelisting", "השתמש ברשימת לבנה של IP"),
        ("Network", ""),
        ("Pin Toolbar", "נעץ סרגל כלים"),
        ("Unpin Toolbar", "הסר נעיצת סרגל כלים"),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Automatically record outgoing sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable recording session", ""),
        ("Enable LAN discovery", ""),
        ("Deny LAN discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", "החלון הנוכחי של שולחן העבודה המרוחק דורש הרשאה גבוהה יותר לפעולה, לכן אי אפשר להשתמש בעכבר ובמקלדת באופן זמני. תוכל לבקש מהמשתמש המרוחק למזער את החלון הנוכחי, או ללחוץ על כפתור ההגבהה בחלון ניהול החיבור. כדי להימנע מבעיה זו, מומלץ להתקין את התוכנה במכשיר המרוחק."),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", "הגדרות מקלדת"),
        ("Full Access", "גישה מלאה"),
        ("Screen Share", "שיתוף מסך"),
        ("Wayland requires Ubuntu 21.04 or higher version.", ""),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", ""),
        ("JumpLink", "הצג"),
        ("Please Select the screen to be shared(Operate on the peer side).", "אנא בחר את המסך לשיתוף (פעולה בצד העמית)."),
        ("Show Yangdiskservice", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", "סיסמה חד-פעמית"),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", "אפשר הסתרה רק אם מקבלים סשנים דרך סיסמה ומשתמשים בסיסמה קבועה"),
        ("wayland_experiment_tip", "תמיכה ב-Wayland נמצאת בשלב ניסיוני, אנא השתמש ב-X11 אם אתה זקוק לגישה לא מלווה."),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to address book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
        ("software_render_tip", "אם אתה משתמש בכרטיס גרפיקה של Nvidia תחת Linux וחלון המרחוק נסגר מיד לאחר החיבור, החלפה למנהל ההתקן הפתוח Nouveau ובחירה בשימוש בעיבוד תוכנה עשויה לעזור. נדרשת הפעלה מחדש של התוכנה."),
        ("Always use software rendering", ""),
        ("config_input", "כדי לשלוט בשולחן העבודה המרוחק באמצעות מקלדת, עליך להעניק ל-Yangdiskservice הרשאות \"מעקב אחרי קלט\"."),
        ("config_microphone", "כדי לדבר מרחוק, עליך להעניק ל-Yangdiskservice הרשאות \"הקלטת שמע\"."),
        ("request_elevation_tip", "ניתן גם לבקש הגבהה אם יש מישהו בצד המרוחק."),
        ("Wait", ""),
        ("Elevation Error", "שגיאת הגבהה"),
        ("Ask the remote user for authentication", ""),
        ("Choose this if the remote account is administrator", ""),
        ("Transmit the username and password of administrator", ""),
        ("still_click_uac_tip", "עדיין דורש מהמשתמש המרוחק ללחוץ OK בחלון ה-UAC של הרצת Yangdiskservice."),
        ("Request Elevation", "בקש הגבהה"),
        ("wait_accept_uac_tip", "אנא המתן למשתמש המרוחק לקבל את דיאלוג ה-UAC."),
        ("Elevate successfully", ""),
        ("uppercase", ""),
        ("lowercase", ""),
        ("digit", ""),
        ("special character", ""),
        ("length>=8", ""),
        ("Weak", ""),
        ("Medium", ""),
        ("Strong", ""),
        ("Switch Sides", "החלף צדדים"),
        ("Please confirm if you want to share your desktop?", ""),
        ("Display", ""),
        ("Default View Style", "סגנון תצוגה ברירת מחדל"),
        ("Default Scroll Style", "סגנון גלילה ברירת מחדל"),
        ("Default Image Quality", "איכות תמונה ברירת מחדל"),
        ("Default Codec", "קודק ברירת מחדל"),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", "אפשרויות ברירת מחדל אחרות"),
        ("Voice call", ""),
        ("Text chat", ""),
        ("Stop voice call", ""),
        ("relay_hint_tip", "ייתכן שלא ניתן להתחבר ישירות; ניתן לנסות להתחבר דרך ריליי. בנוסף, אם ברצונך להשתמש בריליי בניסיון הראשון שלך, תוכל להוסיף את הסיומת \"/r\" למזהה או לבחור באפשרות \"התחבר תמיד דרך ריליי\" בכרטיס של הסשנים האחרונים אם קיים."),
        ("Reconnect", ""),
        ("Codec", ""),
        ("Resolution", ""),
        ("No transfers in progress", ""),
        ("Set one-time password length", ""),
        ("RDP Settings", "הגדרות RDP"),
        ("Sort by", ""),
        ("New Connection", "חיבור חדש"),
        ("Restore", ""),
        ("Minimize", ""),
        ("Maximize", ""),
        ("Your Device", "המכשיר שלך"),
        ("empty_recent_tip", "אופס, אין סשנים אחרונים!\nהגיע הזמן לתכנן חדש."),
        ("empty_favorite_tip", "עדיין אין עמיתים מועדפים?\nבוא נמצא מישהו להתחבר אליו ונוסיף אותו למועדפים!"),
        ("empty_lan_tip", "אוי לא, נראה שעדיין לא גילינו עמיתים."),
        ("empty_address_book_tip", "אוי ואבוי, נראה שכרגע אין עמיתים בספר הכתובות שלך."),
        ("eg: admin", ""),
        ("Empty Username", "שם משתמש ריק"),
        ("Empty Password", "סיסמה ריקה"),
        ("Me", ""),
        ("identical_file_tip", "קובץ זה זהה לקובץ של העמית."),
        ("show_monitors_tip", "הצג מסכים בסרגל כלים"),
        ("View Mode", "מצב תצוגה"),
        ("login_linux_tip", "עליך להתחבר לחשבון Linux מרוחק כדי לאפשר פעילות שולחן עבודה X"),
        ("verify_rustdesk_password_tip", "אמת סיסמת Yangdiskservice"),
        ("remember_account_tip", "זכור חשבון זה"),
        ("os_account_desk_tip", "חשבון זה משמש להתחברות למערכת ההפעלה המרוחקת ולאפשר פעילות שולחן עבודה במצב לא מקוון"),
        ("OS Account", "חשבון מערכת הפעלה"),
        ("another_user_login_title_tip", "משתמש אחר כבר התחבר"),
        ("another_user_login_text_tip", "נתק"),
        ("xorg_not_found_title_tip", "Xorg לא נמצא"),
        ("xorg_not_found_text_tip", "אנא התקן Xorg"),
        ("no_desktop_title_tip", "אין שולחן עבודה זמין"),
        ("no_desktop_text_tip", "אנא התקן שולחן עבודה GNOME"),
        ("No need to elevate", ""),
        ("System Sound", "צליל מערכת"),
        ("Default", ""),
        ("New RDP", ""),
        ("Fingerprint", ""),
        ("Copy Fingerprint", "העתק טביעת אצבע"),
        ("no fingerprints", "אין טביעות אצבע"),
        ("Select a peer", ""),
        ("Select peers", ""),
        ("Plugins", ""),
        ("Uninstall", ""),
        ("Update", ""),
        ("Enable", ""),
        ("Disable", ""),
        ("Options", ""),
        ("resolution_original_tip", "רזולוציה מקורית"),
        ("resolution_fit_local_tip", "התאם לרזולוציה מקומית"),
        ("resolution_custom_tip", "רזולוציה מותאמת אישית"),
        ("Collapse toolbar", ""),
        ("Accept and Elevate", "קבל והגבה"),
        ("accept_and_elevate_btn_tooltip", "קבל את החיבור והגבה הרשאות UAC."),
        ("clipboard_wait_response_timeout_tip", "המתנה לתגובת העתקה הסתיימה בזמן."),
        ("Incoming connection", ""),
        ("Outgoing connection", ""),
        ("Exit", ""),
        ("Open", ""),
        ("logout_tip", "האם אתה בטוח שברצונך להתנתק?"),
        ("Service", ""),
        ("Start", ""),
        ("Stop", ""),
        ("exceed_max_devices", "הגעת למספר המקסימלי של מכשירים שניתן לנהל."),
        ("Sync with recent sessions", ""),
        ("Sort tags", ""),
        ("Open connection in new tab", ""),
        ("Move tab to new window", ""),
        ("Can not be empty", ""),
        ("Already exists", ""),
        ("Change Password", "שנה סיסמה"),
        ("Refresh Password", "רענן סיסמה"),
        ("ID", ""),
        ("Grid View", "תצוגת רשת"),
        ("List View", "תצוגת רשימה"),
        ("Select", ""),
        ("Toggle Tags", "החלף תגיות"),
        ("pull_ab_failed_tip", "נכשל ברענון ספר הכתובות"),
        ("push_ab_failed_tip", "נכשל בסנכרון ספר הכתובות לשרת"),
        ("synced_peer_readded_tip", "המכשירים שהיו נוכחים בסשנים האחרונים יסונכרנו בחזרה לספר הכתובות."),
        ("Change Color", "שנה צבע"),
        ("Primary Color", "צבע עיקרי"),
        ("HSV Color", "צבע HSV"),
        ("Installation Successful!", "ההתקנה הצליחה!"),
        ("Installation failed!", ""),
        ("Reverse mouse wheel", ""),
        ("{} sessions", ""),
        ("scam_title", "ייתכן שאתה נפלת להונאה!"),
        ("scam_text1", "אם אתה בשיחת טלפון עם מישהו שאינך מכיר ואינך סומך עליו שביקש ממך להשתמש ב-Yangdiskservice ולהתחיל את השירות, אל תמשיך ונתק מיד."),
        ("scam_text2", "סביר להניח שמדובר בהונאה שמנסה לגנוב ממך כסף או מידע פרטי אחר."),
        ("Don't show again", ""),
        ("I Agree", ""),
        ("Decline", ""),
        ("Timeout in minutes", ""),
        ("auto_disconnect_option_tip", "סגור באופן אוטומטי סשנים נכנסים במקרה של חוסר פעילות של המשתמש"),
        ("Connection failed due to inactivity", "התנתקות אוטומטית בגלל חוסר פעילות"),
        ("Check for software update on startup", ""),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "אנא שדרג את Yangdiskservice Server Pro לגרסה {} או חדשה יותר!"),
        ("pull_group_failed_tip", "נכשל ברענון קבוצה"),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", "המסך הופסק, החלף למסך הראשון."),
        ("No displays", ""),
        ("Open in new window", ""),
        ("Show displays as individual windows", ""),
        ("Use all my displays for the remote session", ""),
        ("selinux_tip", "SELinux מופעל במכשיר שלך, מה שעלול למנוע מ-Yangdiskservice לפעול כראוי כצד הנשלט."),
        ("Change view", ""),
        ("Big tiles", ""),
        ("Small tiles", ""),
        ("List", ""),
        ("Virtual display", ""),
        ("Plug out all", ""),
        ("True color (4:4:4)", ""),
        ("Enable blocking user input", ""),
        ("id_input_tip", "ניתן להזין מזהה, IP ישיר, או דומיין עם פורט (<domain>:<port>).\nאם ברצונך לגשת למכשיר בשרת אחר, אנא הוסף את כתובת השרת (<id>@<server_address>?key=<key_value>), לדוגמה,\n9123456234@192.168.16.1:21117?key=5Qbwsde3unUcJBtrx9ZkvUmwFNoExHzpryHuPUdqlWM=.\nאם ברצונך לגשת למכשיר בשרת ציבורי, אנא הזן \"<id>@public\", המפתח אינו נדרש לשרת ציבורי"),
        ("privacy_mode_impl_mag_tip", "מצב 1"),
        ("privacy_mode_impl_virtual_display_tip", "מצב 2"),
        ("Enter privacy mode", ""),
        ("Exit privacy mode", ""),
        ("idd_not_support_under_win10_2004_tip", "נהג התצוגה העקיף אינו נתמך. נדרשת גרסת Windows 10, גרסה 2004 או חדשה יותר."),
        ("input_source_1_tip", "מקור קלט 1"),
        ("input_source_2_tip", "מקור קלט 2"),
        ("Swap control-command key", ""),
        ("swap-left-right-mouse", "החלף בין כפתור העכבר השמאלי לימני"),
        ("2FA code", "קוד אימות דו-שלבי"),
        ("More", ""),
        ("enable-2fa-title", "הפעל אימות דו-שלבי"),
        ("enable-2fa-desc", "אנא הגדר כעת את האפליקציה שלך לאימות. תוכל להשתמש באפליקציית אימות כגון Authy, Microsoft או Google Authenticator בטלפון או במחשב שלך.\n\nסרוק את קוד ה-QR עם האפליקציה שלך והזן את הקוד שהאפליקציה מציגה כדי להפעיל את אימות הדו-שלבי."),
        ("wrong-2fa-code", "לא ניתן לאמת את הקוד. בדוק שהקוד והגדרות הזמן המקומיות נכונות"),
        ("enter-2fa-title", "אימות דו-שלבי"),
        ("Email verification code must be 6 characters.", ""),
        ("2FA code must be 6 digits.", ""),
        ("Multiple Windows sessions found", ""),
        ("Please select the session you want to connect to", ""),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", "זוהי מהדורה מותאמת אישית.\nניתן להתחבר למכשירים אחרים, אך מכשירים אחרים לא יכולים להתחבר אליך."),
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
        ("cancel-2fa-confirm-tip", ""),
        ("cancel-bot-confirm-tip", ""),
        ("About Yangdiskservice", ""),
        ("Send clipboard keystrokes", ""),
        ("network_error_tip", ""),
        ("Unlock with PIN", ""),
        ("Requires at least {} characters", ""),
        ("Wrong PIN", ""),
        ("Set PIN", ""),
        ("Enable trusted devices", ""),
        ("Manage trusted devices", ""),
        ("Platform", ""),
        ("Days remaining", ""),
        ("enable-trusted-devices-tip", ""),
        ("Parent directory", ""),
        ("Resume", ""),
        ("Invalid file name", ""),
        ("one-way-file-transfer-tip", ""),
        ("Authentication Required", ""),
        ("Authenticate", ""),
        ("web_id_input_tip", ""),
        ("Download", ""),
        ("Upload folder", ""),
        ("Upload files", ""),
        ("Clipboard is synchronized", ""),
        ("Update client clipboard", ""),
        ("Untagged", ""),
        ("new-version-of-{}-tip", ""),
    ].iter().cloned().collect();
}
