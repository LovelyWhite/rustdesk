lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "狀態"),
        ("Your Desktop", "您的桌面"),
        ("desk_tip", "您可以透過此 ID 及密碼存取您的桌面"),
        ("Password", "密碼"),
        ("Ready", "就緒"),
        ("Established", "已建立"),
        ("connecting_status", "正在連接至 RustDesk 網路..."),
        ("Enable Service", "啟用服務"),
        ("Start Service", "啟動服務"),
        ("Service is running", "服務正在運行"),
        ("Service is not running", "服務尚未執行"),
        ("not_ready_status", "尚未就緒。請檢查您的網路連線"),
        ("Control Remote Desktop", "控制遠端桌面"),
        ("Transfer File", "傳輸檔案"),
        ("Connect", "連接"),
        ("Recent Sessions", "近期的工作階段"),
        ("Address Book", "通訊錄"),
        ("Confirmation", "確認"),
        ("TCP Tunneling", "TCP 通道"),
        ("Remove", "移除"),
        ("Refresh random password", "重新產生隨機密碼"),
        ("Set your own password", "自行設置密碼"),
        ("Enable Keyboard/Mouse", "啟用鍵盤/滑鼠"),
        ("Enable Clipboard", "啟用剪貼簿"),
        ("Enable File Transfer", "啟用檔案傳輸"),
        ("Enable TCP Tunneling", "啟用 TCP 通道"),
        ("IP Whitelisting", "IP 白名單"),
        ("ID/Relay Server", "ID/轉送伺服器"),
        ("Stop service", "停止服務"),
        ("Change ID", "更改 ID"),
        ("Website", "網站"),
        ("About", "關於"),
        ("Mute", "靜音"),
        ("Audio Input", "音訊輸入"),
        ("ID Server", "ID 伺服器"),
        ("Relay Server", "轉送伺服器"),
        ("API Server", "API 伺服器"),
        ("invalid_http", "開頭必須為 http:// 或 https://"),
        ("Invalid IP", "IP 無效"),
        ("id_change_tip", "僅能使用以下字元：a-z、A-Z、0-9、_ (底線)。首字元必須為 a-z 或 A-Z。長度介於 6 到 16 之間。"),
        ("Invalid format", "格式無效"),
        ("This function is turned off by the server", "此功能已被伺服器停用"),
        ("Not available", "無法使用"),
        ("Too frequent", "修改過於頻繁，請稍後再試。"),
        ("Cancel", "取消"),
        ("Skip", "跳過"),
        ("Close", "關閉"),
        ("Retry", "重試"),
        ("OK", "確定"),
        ("Password Required", "需要密碼"),
        ("Please enter your password", "請輸入您的密碼"),
        ("Remember password", "記住密碼"),
        ("Wrong Password", "密碼錯誤"),
        ("Do you want to enter again?", "您要重新輸入嗎？"),
        ("Connection Error", "連線錯誤"),
        ("Error", "錯誤"),
        ("Reset by the peer", "對方重置了連線"),
        ("Connecting...", "正在連接..."),
        ("Connection in progress. Please wait.", "正在連接，請稍候。"),
        ("Please try 1 minute later", "請於 1 分鐘後再試"),
        ("Login Error", "登入錯誤"),
        ("Successful", "成功"),
        ("Connected, waiting for image...", "已連線，等待畫面傳輸..."),
        ("Name", "名稱"),
        ("Type", "類型"),
        ("Modified", "修改時間"),
        ("Size", "大小"),
        ("Show Hidden Files", "顯示隱藏檔案"),
        ("Receive", "接收"),
        ("Send", "傳送"),
        ("Refresh File", "刷新文件"),
        ("Local", "本地"),
        ("Remote", "遠端"),
        ("Remote Computer", "遠端電腦"),
        ("Local Computer", "本地電腦"),
        ("Confirm Delete", "確認刪除"),
        ("Delete", "刪除"),
        ("Properties", "屬性"),
        ("Multi Select", "多選"),
        ("Empty Directory", "空文件夾"),
        ("Not an empty directory", "不是一個空文件夾"),
        ("Are you sure you want to delete this file?", "您確定要刪除此檔案嗎？"),
        ("Are you sure you want to delete this empty directory?", "您確定要刪除此空文件夾?"),
        ("Are you sure you want to delete the file of this directory?", "您確定要刪除文件夾下的文件?"),
        ("Do this for all conflicts", "套用到其他衝突"),
        ("This is irreversible!", "此操作不可逆！"),
        ("Deleting", "正在刪除"),
        ("files", "檔案"),
        ("Waiting", "正在等候..."),
        ("Finished", "已完成"),
        ("Speed", "速度"),
        ("Custom Image Quality", "自訂圖片品質"),
        ("Privacy mode", "隱私模式"),
        ("Block user input", "封鎖使用者輸入"),
        ("Unblock user input", "取消封鎖使用者輸入"),
        ("Adjust Window", "調整視窗"),
        ("Original", "原始"),
        ("Shrink", "縮減"),
        ("Stretch", "延展"),
        ("Good image quality", "畫面品質良好"),
        ("Balanced", "平衡"),
        ("Optimize reaction time", "回應速度最佳化"),
        ("Custom", "自訂"),
        ("Show remote cursor", "顯示遠端游標"),
        ("Disable clipboard", "停用剪貼簿"),
        ("Lock after session end", "工作階段結束後鎖定電腦"),
        ("Insert", "插入"),
        ("Insert Lock", "鎖定遠端電腦"),
        ("Refresh", "重新載入"),
        ("ID does not exist", "ID 不存在"),
        ("Failed to connect to rendezvous server", "無法連接至 rendezvous 伺服器"),
        ("Please try later", "請稍候再試"),
        ("Remote desktop is offline", "遠端電腦離線"),
        ("Key mismatch", "金鑰不符"),
        ("Timeout", "逾時"),
        ("Failed to connect to relay server", "無法連接至轉送伺服器"),
        ("Failed to connect via rendezvous server", "無法透過 rendezvous 伺服器連接"),
        ("Failed to connect via relay server", "無法透過轉送伺服器連接"),
        ("Failed to make direct connection to remote desktop", "無法直接連線至遠端電腦"),
        ("Set Password", "設置密碼"),
        ("OS Password", "作業系統密碼"),
        ("install_tip", "UAC 會導致 RustDesk 在某些情況下無法正常以遠端電腦運作。若要避開 UAC，請點擊下方按鈕將 RustDesk 安裝到系統中。"),
        ("Click to upgrade", "點擊以升級"),
        ("Click to download", "點擊以下載"),
        ("Click to update", "點擊以更新"),
        ("Configure", "設定"),
        ("config_acc", "您需要授予 RustDesk ｢協助工具」 權限才能遠端存取電腦。"),
        ("config_screen", "您需要授予 RustDesk ｢畫面錄製」 權限才能遠端存取電腦。"),
        ("Installing ...", "正在安裝..."),
        ("Install", "安裝"),
        ("Installation", "安裝"),
        ("Installation Path", "安裝路徑"),
        ("Create start menu shortcuts", "建立開始選單捷徑"),
        ("Create desktop icon", "建立桌面圖示"),
        ("agreement_tip", ""),
        ("Accept and Install", "接受並安裝"),
        ("End-user license agreement", "使用者授權合約"),
        ("Generating ...", "正在產生 ..."),
        ("Your installation is lower version.", "您的安裝版本過舊。"),
        ("not_close_tcp_tip", "使用通道時請不要關閉此視窗"),
        ("Listening ...", "正在等待通道連接..."),
        ("Remote Host", "遠端主機"),
        ("Remote Port", "遠端連接埠"),
        ("Action", "操作"),
        ("Add", "新增"),
        ("Local Port", "本機連接埠"),
        ("setup_server_tip", "若您需要更快的連接速度，可以選擇自行建立伺服器"),
        ("Too short, at least 6 characters.", "過短，至少需 6 個字元。"),
        ("The confirmation is not identical.", "兩次輸入不相符"),
        ("Permissions", "權限"),
        ("Accept", "接受"),
        ("Dismiss", "關閉"),
        ("Disconnect", "斷開連線"),
        ("Allow using keyboard and mouse", "允許使用鍵盤和滑鼠"),
        ("Allow using clipboard", "允許使用剪貼簿"),
        ("Allow hearing sound", "允許分享音訊"),
        ("Allow file transfer", "允許文件傳輸"),
        ("File transfer", "文件傳輸"),
        ("Connected", "已連接"),
        ("Direct and encrypted connection", "加密直接連線"),
        ("Relayed and encrypted connection", "加密轉送連線"),
        ("Direct and unencrypted connection", "未加密直接連線"),
        ("Relayed and unencrypted connection", "未加密轉送連線"),
        ("Enter Remote ID", "輸入遠端 ID"),
        ("Enter your password", "輸入您的密碼"),
        ("Logging in...", "正在登入..."),
        ("Enable RDP session sharing", "啟用 RDP 工作階段共享"),
        ("Auto Login", "自動登入 (鎖定將在設定關閉後套用)"),
        ("Enable Direct IP Access", "允許 IP 直接存取"),
        ("Rename", "重新命名"),
        ("Space", "空白"),
        ("Create Desktop Shortcut", "建立桌面捷徑"),
        ("Change Path", "更改路徑"),
        ("Create Folder", "建立資料夾"),
        ("Please enter the folder name", "請輸入資料夾名稱"),
        ("Fix it", "修復"),
        ("Warning", "警告"),
        ("Login screen using Wayland is not supported", "不支援使用 Wayland 的登入畫面"),
        ("Reboot required", "需要重新啟動"),
        ("Unsupported display server ", "不支援顯示伺服器"),
        ("x11 expected", "預期 x11"),
        ("Port", "連接埠"),
        ("Settings", "設定"),
        ("Username", " 使用者名稱"),
        ("Invalid port", "連接埠無效"),
        ("Closed manually by the peer", "由對方手動關閉"),
        ("Enable remote configuration modification", "啟用遠端更改設定"),
        ("Run without install", "跳過安裝直接執行"),
        ("Always connected via relay", "一律透過轉送連線"),
        ("Always connect via relay", "一律透過轉送連線"),
        ("whitelist_tip", "只有白名單中的 IP 可以存取"),
        ("Login", "登入"),
        ("Logout", "登出"),
        ("Tags", "標籤"),
        ("Search ID", "搜尋 ID"),
        ("Current Wayland display server is not supported", "目前不支援 Wayland 顯示伺服器"),
        ("whitelist_sep", "使用逗號、分號、空白，或是換行來分隔"),
        ("Add ID", "新增 ID"),
        ("Add Tag", "新增標籤"),
        ("Unselect all tags", "取消選取所有標籤"),
        ("Network error", "網路錯誤"),
        ("Username missed", "缺少使用者名稱"),
        ("Password missed", "缺少密碼"),
        ("Wrong credentials", "提供的登入資訊有誤"),
        ("Edit Tag", "編輯標籤"),
        ("Unremember Password", "忘掉密碼"),
        ("Favorites", "收藏"),
        ("Add to Favorites", "加入到收藏"),
        ("Remove from Favorites", "從收藏中刪除"),
        ("Empty", "空空如也"),
        ("Invalid folder name", "資料夾名稱無效"),
        ("Socks5 Proxy", "Socks5 代理"),
        ("Hostname", "主機名稱"),
        ("Discovered", "已發現"),
        ("install_daemon_tip", "為了開機啟動，請安裝系統服務。"),
        ("Remote ID", "遠端 ID"),
        ("Paste", "貼上"),
        ("Paste here?", "貼上到這裡?"),
        ("Are you sure to close the connection?", "您確定要關閉連線嗎？"),
        ("Download new version", "下載新版本"),
        ("Touch mode", "觸控模式"),
        ("Mouse mode", "鼠標模式"),
        ("One-Finger Tap", "單指輕觸"),
        ("Left Mouse", "鼠標左鍵"),
        ("One-Long Tap", "單指長按"),
        ("Two-Finger Tap", "雙指輕觸"),
        ("Right Mouse", "鼠標右鍵"),
        ("One-Finger Move", "單指移動"),
        ("Double Tap & Move", "雙擊並移動"),
        ("Mouse Drag", "鼠標選中拖動"),
        ("Two-Finger vertically", "雙指垂直滑動"),
        ("Mouse Wheel", "鼠標滾輪"),
        ("Two-Finger Move", "雙指移動"),
        ("Canvas Move", "移動畫布"),
        ("Pinch to Zoom", "雙指縮放"),
        ("Canvas Zoom", "縮放畫布"),
        ("Reset canvas", "重置畫布"),
        ("No permission of file transfer", "無文件傳輸權限"),
        ("Note", "備註"),
        ("Connection", "連接"),
        ("Share Screen", "屏幕錄製"),
        ("CLOSE", "關閉"),
        ("OPEN", "開啟"),
        ("Chat", "聊天消息"),
        ("Total", "總計"),
        ("items", "個項目"),
        ("Selected", "已選擇"),
        ("Screen Capture", "屏幕錄製"),
        ("Input Control", "輸入控制"),
        ("File Transfer", "文件傳輸"),
        ("Audio Capture", "音頻錄製"),
        ("File Connection", "文件連接"),
        ("Screen Connection", "屏幕連接"),
        ("Do you accept?", "是否接受？"),
        ("Open System Setting", "打開系統設置"),
        ("How to get Android input permission?", "如何獲取安卓的輸入權限？"),
        ("android_input_permission_tip1", "獲取輸入權限後可以讓遠程設備通過鼠標控制這台安卓設備"),
        ("android_input_permission_tip2", "請在接下來的系統設置頁面裡，找到並進入 [已安裝的服務] 頁面，將 [RustDesk Input] 服務開啟"),
        ("android_new_connection_tip", "收到新的連接控制請求，對方想要控制你當前的設備"),
        ("android_service_will_start_tip", "開啟錄屏權限將自動開啟服務，允許其他設備向此設備請求建立連接。"),
        ("android_stop_service_tip", "關閉服務將自動關閉所有已建立的連接。"),
        ("android_version_audio_tip", "當前安卓版本不支持音頻錄製，請升級至安卓10或更高。"),
        ("android_start_service_tip", "點擊 [啟動服務] 或打開 [屏幕錄製] 權限開啟手機屏幕共享服務。"),
        ("Account", "帳戶"),
    ].iter().cloned().collect();
}
