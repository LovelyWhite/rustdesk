lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "สถานะ"),
        ("Your Desktop", "หน้าจอของคุณ"),
        ("desk_tip", "คุณสามารถเข้าถึงเดสก์ท็อปของคุณได้ด้วย ID และรหัสผ่านต่อไปนี้"),
        ("Password", "รหัสผ่าน"),
        ("Ready", "พร้อม"),
        ("Established", "เชื่อมต่อแล้ว"),
        ("connecting_status", "กำลังเชื่อมต่อไปยังเครือข่าย RustDesk..."),
        ("Enable service", "เปิดใช้การงานเซอร์วิส"),
        ("Start service", "เริ่มต้นใช้งานเซอร์วิส"),
        ("Service is running", "เซอร์วิสกำลังทำงาน"),
        ("Service is not running", "เซอร์วิสไม่ทำงาน"),
        ("not_ready_status", "ไม่พร้อมใช้งาน กรุณาตรวจสอบการเชื่อมต่ออินเทอร์เน็ตของคุณ"),
        ("Control Remote Desktop", "การควบคุมเดสก์ท็อปปลายทาง"),
        ("Transfer file", "การถ่ายโอนไฟล์"),
        ("Connect", "เชื่อมต่อ"),
        ("Recent sessions", "เซสชันล่าสุด"),
        ("Address book", "สมุดรายชื่อ"),
        ("Confirmation", "การยืนยัน"),
        ("TCP tunneling", "อุโมงค์การเชื่อมต่อ TCP"),
        ("Remove", "ลบ"),
        ("Refresh random password", "รีเฟรชรหัสผ่านใหม่แบบสุ่ม"),
        ("Set your own password", "ตั้งรหัสผ่านของคุณเอง"),
        ("Enable keyboard/mouse", "เปิดการใช้งาน คีย์บอร์ด/เมาส์"),
        ("Enable clipboard", "เปิดการใช้งาน คลิปบอร์ด"),
        ("Enable file transfer", "เปิดการใช้งาน การถ่ายโอนไฟล์"),
        ("Enable TCP tunneling", "เปิดการใช้งาน อุโมงค์การเชื่อมต่อ TCP"),
        ("IP Whitelisting", "IP ไวท์ลิสต์"),
        ("ID/Relay Server", "เซิร์ฟเวอร์ ID/Relay"),
        ("Import server config", "นำเข้าการตั้งค่าเซิร์ฟเวอร์"),
        ("Export Server Config", "ส่งออกการตั้งค่าเซิร์ฟเวอร์"),
        ("Import server configuration successfully", "นำเข้าการตั้งค่าเซิร์ฟเวอร์เสร็จสมบูรณ์"),
        ("Export server configuration successfully", "ส่งออกการตั้งค่าเซิร์ฟเวอร์เสร็จสมบูรณ์"),
        ("Invalid server configuration", "การตั้งค่าของเซิร์ฟเวอร์ไม่ถูกต้อง"),
        ("Clipboard is empty", "คลิปบอร์ดว่างเปล่า"),
        ("Stop service", "หยุดการใช้งานเซอร์วิส"),
        ("Change ID", "เปลี่ยน ID"),
        ("Your new ID", "ID ใหม่ของคุณ"),
        ("length %min% to %max%", "ความยาวตั้งแต่ %min% ถึง %max%"),
        ("starts with a letter", "เริ่มต้นด้วยตัวอักษร"),
        ("allowed characters", "ตัวอักขระที่อนุญาต"),
        ("id_change_tip", "อนุญาตเฉพาะตัวอักษร a-z A-Z 0-9 และ _ (ขีดล่าง) เท่านั้น โดยตัวอักษรขึ้นต้นจะต้องเป็น a-z หรือไม่ก็ A-Z และมีความยาวระหว่าง 6 ถึง 16 ตัวอักษร"),
        ("Website", "เว็บไซต์"),
        ("About", "เกี่ยวกับ"),
        ("Slogan_tip", "ทำด้วยใจ ในโลกที่วุ่นวาย!"),
        ("Privacy Statement", "คำแถลงเกี่ยวกับความเป็นส่วนตัว"),
        ("Mute", "ปิดเสียง"),
        ("Build Date", "วันที่ Build"),
        ("Version", "เวอร์ชัน"),
        ("Home", "หน้าหลัก"),
        ("Audio Input", "ออดิโออินพุท"),
        ("Enhancements", "การปรับปรุง"),
        ("Hardware Codec", "ฮาร์ดแวร์ Codec"),
        ("Adaptive bitrate", "Bitrate ผันแปร"),
        ("ID Server", "เซิร์ฟเวอร์ ID"),
        ("Relay Server", "เซิร์ฟเวอร์ Relay"),
        ("API Server", "เซิร์ฟเวอร์ API"),
        ("invalid_http", "ต้องขึ้นต้นด้วย http:// หรือ https:// เท่านั้น"),
        ("Invalid IP", "IP ไม่ถูกต้อง"),
        ("Invalid format", "รูปแบบไม่ถูกต้อง"),
        ("server_not_support", "ยังไม่รองรับโดยเซิร์ฟเวอร์"),
        ("Not available", "ไม่พร้อมใช้งาน"),
        ("Too frequent", "ดำเนินการถี่เกินไป"),
        ("Cancel", "ยกเลิก"),
        ("Skip", "ข้าม"),
        ("Close", "ปิด"),
        ("Retry", "ลองใหม่อีกครั้ง"),
        ("OK", "ตกลง"),
        ("Password Required", "ต้องใช้รหัสผ่าน"),
        ("Please enter your password", "กรุณาใส่รหัสผ่านของคุณ"),
        ("Remember password", "จดจำรหัสผ่าน"),
        ("Wrong Password", "รหัสผ่านไม่ถูกต้อง"),
        ("Do you want to enter again?", "ต้องการใส่ข้อมูลอีกครั้งหรือไม่?"),
        ("Connection Error", "การเชื่อมต่อผิดพลาด"),
        ("Error", "ข้อผิดพลาด"),
        ("Reset by the peer", "รีเซ็ตโดยอีกฝั่ง"),
        ("Connecting...", "กำลังเชื่อมต่อ..."),
        ("Connection in progress. Please wait.", "กำลังดำเนินการเชื่อมต่อ กรุณารอซักครู่"),
        ("Please try 1 minute later", "กรุณาลองใหม่อีกครั้งใน 1 นาที"),
        ("Login Error", "การเข้าสู่ระบบผิดพลาด"),
        ("Successful", "สำเร็จ"),
        ("Connected, waiting for image...", "เชื่อมต่อสำเร็จ กำลังรับข้อมูลภาพ..."),
        ("Name", "ชื่อ"),
        ("Type", "ประเภท"),
        ("Modified", "แก้ไขล่าสุด"),
        ("Size", "ขนาด"),
        ("Show Hidden Files", "แสดงไฟล์ที่ถูกซ่อน"),
        ("Receive", "รับ"),
        ("Send", "ส่ง"),
        ("Refresh File", "รีเฟรชไฟล์"),
        ("Local", "ต้นทาง"),
        ("Remote", "ปลายทาง"),
        ("Remote Computer", "คอมพิวเตอร์ปลายทาง"),
        ("Local Computer", "คอมพิวเตอร์ต้นทาง"),
        ("Confirm Delete", "ยืนยันการลบ"),
        ("Delete", "ลบ"),
        ("Properties", "ข้อมูล"),
        ("Multi Select", "เลือกหลายรายการ"),
        ("Select All", "เลือกทั้งหมด"),
        ("Unselect All", "ยกเลิกการเลือกทั้งหมด"),
        ("Empty Directory", "ไดเรกทอรีว่างเปล่า"),
        ("Not an empty directory", "ไม่ใช่ไดเรกทอรีว่างเปล่า"),
        ("Are you sure you want to delete this file?", "คุณแน่ใจหรือไม่ที่จะลบไฟล์นี้?"),
        ("Are you sure you want to delete this empty directory?", "คุณแน่ใจหรือไม่ที่จะลบไดเรอทอรีว่างเปล่านี้?"),
        ("Are you sure you want to delete the file of this directory?", "คุณแน่ใจหรือไม่ที่จะลบไฟล์ของไดเรกทอรีนี้?"),
        ("Do this for all conflicts", "ดำเนินการแบบเดียวกันสำหรับรายการทั้งหมด"),
        ("This is irreversible!", "การดำเนินการนี้ไม่สามารถย้อนกลับได้!"),
        ("Deleting", "กำลังลบ"),
        ("files", "ไฟล์"),
        ("Waiting", "กำลังรอ"),
        ("Finished", "เสร็จแล้ว"),
        ("Speed", "ความเร็ว"),
        ("Custom Image Quality", "คุณภาพของภาพแบบกำหนดเอง"),
        ("Privacy mode", "โหมดความเป็นส่วนตัว"),
        ("Block user input", "บล็อคอินพุทจากผู้ใช้งาน"),
        ("Unblock user input", "ยกเลิกการบล็อคอินพุทจากผู้ใช้งาน"),
        ("Adjust Window", "ปรับขนาดหน้าต่าง"),
        ("Original", "ต้นฉบับ"),
        ("Shrink", "ย่อ"),
        ("Stretch", "ยืด"),
        ("Scrollbar", "แถบเลื่อน"),
        ("ScrollAuto", "เลื่อนอัตโนมัติ"),
        ("Good image quality", "ภาพคุณภาพดี"),
        ("Balanced", "สมดุล"),
        ("Optimize reaction time", "เน้นการตอบสนอง"),
        ("Custom", "กำหนดเอง"),
        ("Show remote cursor", "แสดงเคอร์เซอร์ปลายทาง"),
        ("Show quality monitor", "แสดงคุณภาพหน้าจอ"),
        ("Disable clipboard", "ปิดการใช้งานคลิปบอร์ด"),
        ("Lock after session end", "ล็อคหลังจากจบเซสชัน"),
        ("Insert", "แทรก"),
        ("Insert Lock", "แทรกล็อค"),
        ("Refresh", "รีเฟรช"),
        ("ID does not exist", "ไม่พอข้อมูล ID"),
        ("Failed to connect to rendezvous server", "การเชื่อมต่อไปยังเซิร์ฟเวอร์นัดพบล้มเหลว"),
        ("Please try later", "กรุณาลองใหม่ในภายหลัง"),
        ("Remote desktop is offline", "เดสก์ท็อปปลายทางออฟไลน์"),
        ("Key mismatch", "คีย์ไม่ถูกต้อง"),
        ("Timeout", "หมดเวลา"),
        ("Failed to connect to relay server", "การเชื่อมต่อไปยังเซิร์ฟเวอร์ Relay ล้มเหลว"),
        ("Failed to connect via rendezvous server", "การเชื่อมต่อผ่านเซิร์ฟเวอร์นัดพบล้มเหลว"),
        ("Failed to connect via relay server", "การเชื่อมต่อผ่านเซิร์ฟเวอร์ Relay ล้มเหลว"),
        ("Failed to make direct connection to remote desktop", "การเชื่อมต่อตรงไปยังเดสก์ท็อปปลายทางล้มเหลว"),
        ("Set Password", "ตั้งรหัสผ่าน"),
        ("OS Password", "รหัสผ่านระบบปฏิบัติการ"),
        ("install_tip", "เนื่องด้วยข้อจำกัดของการใช้งาน UAC ทำให้ RustDesk ไม่สามารถทำงานได้ปกติในฝั่งปลายทางในบางครั้ง เพื่อหลีกเลี่ยงข้อจำกัดของ UAC กรุณากดปุ่มด้านล่างเพื่อติดตั้ง RustDesk ไปยังระบบของคุณ"),
        ("Click to upgrade", "คลิกเพื่ออัปเกรด"),
        ("Click to download", "คลิกเพื่อดาวน์โหลด"),
        ("Click to update", "คลิกเพื่ออัปเดต"),
        ("Configure", "ปรับแต่งค่า"),
        ("config_acc", "เพื่อที่จะควบคุมเดสก์ท็อปปลายทางของคุณ คุณจำเป็นจะต้องอนุญาตสิทธิ์ \"การเข้าถึง\" ให้แก่ RustDesk"),
        ("config_screen", "เพื่อที่จะควบคุมเดสก์ท็อปปลายทางของคุณ คุณจำเป็นจะต้องอนุญาตสิทธิ์ \"การบันทึกภาพหน้าจอ\" ให้แก่ RustDesk"),
        ("Installing ...", "กำลังติดตั้ง ..."),
        ("Install", "ติดตั้ง"),
        ("Installation", "การติดตั้ง"),
        ("Installation Path", "ตำแหน่งที่ติดตั้ง"),
        ("Create start menu shortcuts", "สร้างทางลัดไปยัง Start Menu"),
        ("Create desktop icon", "สร้างไอคอนบนเดสก์ท็อป"),
        ("agreement_tip", "ในการเริ่มต้นการติดตั้ง ถือว่าคุณได้ยอมรับข้อตกลงใบอนุญาตแล้ว"),
        ("Accept and Install", "ยอมรับและติดตั้ง"),
        ("End-user license agreement", "ข้อตกลงใบอนุญาตผู้ใช้งาน"),
        ("Generating ...", "กำลังสร้าง ..."),
        ("Your installation is lower version.", "การติดตั้งของคุณเป็นเวอร์ชันที่ต่ำกว่า"),
        ("not_close_tcp_tip", "อย่าปิดหน้าต่างนี้ในขณะที่คุณกำลังใช้งานอุโมงค์การเชื่อมต่อ"),
        ("Listening ...", "กำลังรอรับข้อมูล ..."),
        ("Remote Host", "โฮสต์ปลายทาง"),
        ("Remote Port", "พอร์ทปลายทาง"),
        ("Action", "การดำเนินการ"),
        ("Add", "เพิ่ม"),
        ("Local Port", "พอร์ทต้นทาง"),
        ("Local Address", "ที่อยู่ต้นทาง"),
        ("Change Local Port", "เปลี่ยนพอร์ทต้นทาง"),
        ("setup_server_tip", "เพื่อการเชื่อมต่อที่เร็วขึ้น กรุณาเซ็ตอัปเซิร์ฟเวอร์ของคุณเอง"),
        ("Too short, at least 6 characters.", "สั้นเกินไป ต้องไม่ต่ำกว่า 6 ตัวอักษร"),
        ("The confirmation is not identical.", "การยืนยันข้อมูลไม่ถูกต้อง"),
        ("Permissions", "สิทธิ์การใช้งาน"),
        ("Accept", "ยอมรับ"),
        ("Dismiss", "ปิด"),
        ("Disconnect", "ยกเลิกการเชื่อมต่อ"),
        ("Enable file copy and paste", "อนุญาตให้มีการคัดลอกและวางไฟล์"),
        ("Connected", "เชื่อมต่อแล้ว"),
        ("Direct and encrypted connection", "การเชื่อมต่อตรงที่มีการเข้ารหัส"),
        ("Relayed and encrypted connection", "การเชื่อมต่อแบบ Relay ที่มีการเข้ารหัส"),
        ("Direct and unencrypted connection", "การเชื่อมต่อตรงที่ไม่มีการเข้ารหัส"),
        ("Relayed and unencrypted connection", "การเชื่อมต่อแบบ Relay ที่ไม่มีการเข้ารหัส"),
        ("Enter Remote ID", "กรอก ID ปลายทาง"),
        ("Enter your password", "กรอกรหัสผ่าน"),
        ("Logging in...", "กำลังเข้าสู่ระบบ..."),
        ("Enable RDP session sharing", "เปิดการใช้งานการแชร์เซสชัน RDP"),
        ("Auto Login", "เข้าสู่ระบอัตโนมัติ"),
        ("Enable direct IP access", "เปิดการใช้งาน IP ตรง"),
        ("Rename", "ปลายทาง"),
        ("Space", "พื้นที่ว่าง"),
        ("Create desktop shortcut", "สร้างทางลัดบนเดสก์ท็อป"),
        ("Change Path", "เปลี่ยนตำแหน่ง"),
        ("Create Folder", "สร้างโฟลเดอร์"),
        ("Please enter the folder name", "กรุณาใส่ชื่อโฟลเดอร์"),
        ("Fix it", "แก้ไข"),
        ("Warning", "คำเตือน"),
        ("Login screen using Wayland is not supported", "หน้าจอการเข้าสู่ระบบโดยใช้ Wayland ยังไม่ถูกรองรับ"),
        ("Reboot required", "จำเป็นต้องเริ่มต้นระบบใหม่"),
        ("Unsupported display server", "เซิร์ฟเวอร์การแสดงผลที่ไม่รองรับ"),
        ("x11 expected", "ต้องใช้งาน x11"),
        ("Port", "พอร์ท"),
        ("Settings", "ตั้งค่า"),
        ("Username", "ชื่อผู้ใช้งาน"),
        ("Invalid port", "พอร์ทไม่ถูกต้อง"),
        ("Closed manually by the peer", "ถูกปิดโดยอีกฝั่งของการเชื่อมต่อ"),
        ("Enable remote configuration modification", "เปิดการใช้งานการแก้ไขการตั้งค่าปลายทาง"),
        ("Run without install", "ใช้งานโดยไม่ต้องติดตั้ง"),
        ("Connect via relay", "เชื่อมต่อผ่าน Relay"),
        ("Always connect via relay", "เชื่อมต่อผ่าน Relay เสมอ"),
        ("whitelist_tip", "อนุญาตเฉพาะการเชื่อมต่อจาก IP ที่ไวท์ลิสต์"),
        ("Login", "เข้าสู่ระบบ"),
        ("Verify", "ยืนยันความถูกต้อง"),
        ("Remember me", "จดจำฉัน"),
        ("Trust this device", "เชื่อถืออุปกรณ์นี้"),
        ("Verification code", "รหัสยืนยันความถูกต้อง"),
        ("verification_tip", "รหัสยืนยันความถูกต้องได้ถูกส่งไปยังอีเมล์ที่ลงทะเบียนแล้ว กรุณากรอกรหัสยืนยันความถูกต้องเพื่อดำเนินการเข้าสู่ระบบต่อ"),
        ("Logout", "ออกจากระบบ"),
        ("Tags", "แท็ก"),
        ("Search ID", "ค้นหา ID"),
        ("whitelist_sep", "คั่นโดยเครื่องหมาย comma semicolon เว้นวรรค หรือ ขึ้นบรรทัดใหม่"),
        ("Add ID", "เพิ่ม ID"),
        ("Add Tag", "เพิ่มแท็ก"),
        ("Unselect all tags", "ยกเลิกการเลือกแท็กทั้งหมด"),
        ("Network error", "ข้อผิดพลาดของเครือข่าย"),
        ("Username missed", "ไม่พบข้อมูลผู้ใช้งาน"),
        ("Password missed", "ไม่พบรหัสผ่าน"),
        ("Wrong credentials", "ข้อมูลสำหรับเข้าสู่ระบบไม่ถูกต้อง"),
        ("The verification code is incorrect or has expired", "รหัสยืนยันไม่ถูกต้องหรือหมดอายุแล้ว"),
        ("Edit Tag", "แก้ไขแท็ก"),
        ("Forget Password", "ยกเลิกการจดจำรหัสผ่าน"),
        ("Favorites", "รายการโปรด"),
        ("Add to Favorites", "เพิ่มไปยังรายการโปรด"),
        ("Remove from Favorites", "ลบออกจากรายการโปรด"),
        ("Empty", "ว่างเปล่า"),
        ("Invalid folder name", "ชื่อโฟลเดอร์ไม่ถูกต้อง"),
        ("Socks5 Proxy", "พรอกซี Socks5"),
        ("Socks5/Http(s) Proxy", "พรอกซี Socks5/Http(s)"),
        ("Discovered", "ค้นพบ"),
        ("install_daemon_tip", "หากต้องการใช้งานขณะระบบเริ่มต้น คุณจำเป็นจะต้องติดตั้งเซอร์วิส"),
        ("Remote ID", "ID ปลายทาง"),
        ("Paste", "วาง"),
        ("Paste here?", "วางที่นี่หรือไม่?"),
        ("Are you sure to close the connection?", "คุณแน่ใจหรือไม่ที่จะปิดการเชื่อมต่อ?"),
        ("Download new version", "ดาวน์โหลดเวอร์ชันใหม่"),
        ("Touch mode", "โหมดการสัมผัส"),
        ("Mouse mode", "โหมดการใช้เมาส์"),
        ("One-Finger Tap", "แตะนิ้วเดียว"),
        ("Left Mouse", "เมาส์ซ้าย"),
        ("One-Long Tap", "แตะยาวหนึ่งครั้ง"),
        ("Two-Finger Tap", "แตะสองนิ้ว"),
        ("Right Mouse", "เมาส์ขวา"),
        ("One-Finger Move", "ลากนิ้วเดียว"),
        ("Double Tap & Move", "แตะเบิ้ลและลาก"),
        ("Mouse Drag", "ลากเมาส์"),
        ("Three-Finger vertically", "สามนิ้วแนวตั้ง"),
        ("Mouse Wheel", "ลูกลิ้งเมาส์"),
        ("Two-Finger Move", "ลากสองนิ้ว"),
        ("Canvas Move", "ลากแคนวาส"),
        ("Pinch to Zoom", "ถ่างเพื่อขยาย"),
        ("Canvas Zoom", "ขยายแคนวาส"),
        ("Reset canvas", "รีเซ็ตแคนวาส"),
        ("No permission of file transfer", "ไม่มีสิทธิ์ในการถ่ายโอนไฟล์"),
        ("Note", "บันทึกข้อความ"),
        ("Connection", "การเชื่อมต่อ"),
        ("Share Screen", "แชร์หน้าจอ"),
        ("Chat", "แชท"),
        ("Total", "รวม"),
        ("items", "รายการ"),
        ("Selected", "ถูกเลือก"),
        ("Screen Capture", "บันทึกหน้าจอ"),
        ("Input Control", "ควบคุมอินพุท"),
        ("Audio Capture", "บันทึกเสียง"),
        ("File Connection", "การเชื่อมต่อไฟล์"),
        ("Screen Connection", "การเชื่อมต่อหน้าจอ"),
        ("Do you accept?", "ยอมรับหรือไม่?"),
        ("Open System Setting", "เปิดการตั้งค่าระบบ"),
        ("How to get Android input permission?", "เปิดสิทธิ์การใช้งานอินพุทของแอนดรอยด์ได้อย่างไร?"),
        ("android_input_permission_tip1", "ในการที่จะอนุญาตให้เครื่องปลายทางควบคุมอุปกรณ์แอนดรอยด์ของคุณโดยใช้เมาส์หรือการสัมผัส คุณจำเป็นจะต้องอนุญาตสิทธิ์ \"การเข้าถึง\" ให้แก่เซอร์วิสของ RustDesk"),
        ("android_input_permission_tip2", "กรุณาไปยังหน้าตั้งค่าถัดไป ค้นหาและเข้าไปยัง [เซอร์วิสที่ถูกติดตั้ง] และเปิดการใช้งานเซอร์วิส [อินพุท RustDesk]"),
        ("android_new_connection_tip", "ได้รับคำขอควบคุมใหม่ที่ต้องการควบคุมอุปกรณ์ของคุณ"),
        ("android_service_will_start_tip", "การเปิดการใช้งาน \"การบันทึกหน้าจอ\" จะเป็นการเริ่มต้นการทำงานของเซอร์วิสโดยอัตโนมัติ ที่จะอนุญาตให้อุปกรณ์อื่นๆ ส่งคำขอเข้าถึงมายังอุปกรณ์ของคุณได้"),
        ("android_stop_service_tip", "การปิดการใช้งานเซอร์วิสจะปิดการเชื่อมต่อทั้งหมดโดยอัตโนมัติ"),
        ("android_version_audio_tip", "เวอร์ชันแอนดรอยด์ปัจจุบันของคุณไม่รองรับการบันทึกข้อมูลเสียง กรุณาอัปเกรดเป็นแอนดรอยด์เวอร์ชัน 10 หรือสูงกว่า"),
        ("android_start_service_tip", "แตะ [เริ่มต้นใช้งานเซอร์วิส] หรือเปิดสิทธิ์การใช้งาน [บันทึกหน้าจอ] เพื่อเริ่มต้นใช้งานเซอร์วิสสำหรับการแบ่งปันหน้าจอ"),
        ("android_permission_may_not_change_tip", "สิทธิ์การใช้งานสำหรับการเชื่อมต่อที่กำลังเปิดใช้งานอยู่อาจจะไม่ได้เปลี่ยนแปลงในทันทีจนกว่าจะเริ่มต้นการเชื่อมต่อใหม่อีกครั้ง"),
        ("Account", "บัญชี"),
        ("Overwrite", "เขียนทับ"),
        ("This file exists, skip or overwrite this file?", "พบไฟล์ที่มีอยู่แล้ว ต้องการเขียนทับหรือไม่?"),
        ("Quit", "ออก"),
        ("Help", "ช่วยเหลือ"),
        ("Failed", "ล้มเหลว"),
        ("Succeeded", "สำเร็จ"),
        ("Someone turns on privacy mode, exit", "มีใครบางคนเปิดใช้งานโหมดความเป็นส่วนตัว กำลังออก"),
        ("Unsupported", "ไม่รองรับ"),
        ("Peer denied", "ถูกปฏิเสธโดยอีกฝั่ง"),
        ("Please install plugins", "กรุณาติดตั้งปลั๊กอิน"),
        ("Peer exit", "อีกฝั่งออก"),
        ("Failed to turn off", "การปิดล้มเหลว"),
        ("Turned off", "ปิด"),
        ("Language", "ภาษา"),
        ("Keep RustDesk background service", "คงสถานะการทำงานเบื้องหลังของเซอร์วิส RustDesk"),
        ("Ignore Battery Optimizations", "เพิกเฉยการตั้งค่าการใช้งาน Battery Optimization"),
        ("android_open_battery_optimizations_tip", "หากคุณต้องการปิดการใช้งานฟีเจอร์นี้ กรุณาไปยังหน้าตั้งค่าในแอปพลิเคชัน RustDesk ค้นหาหัวข้อ [Battery] และยกเลิกการเลือกรายการ [Unrestricted]"),
        ("Start on boot", "เริ่มต้นเมื่อเปิดเครื่อง"),
        ("Start the screen sharing service on boot, requires special permissions", "เริ่มต้นใช้งานเซอร์วิสสำหรับการแบ่งปันหน้าจอเมื่อเปิดเครื่อง (ต้องมีการให้สิทธิ์การใช้งานพิเศษเพิ่มเติม)"),
        ("Connection not allowed", "การเชื่อมต่อไม่อนุญาต"),
        ("Legacy mode", "โหมดดั้งเดิม"),
        ("Map mode", "โหมดการจับคู่"),
        ("Translate mode", "โหมดแปลงค่า"),
        ("Use permanent password", "ใช้รหัสผ่านถาวร"),
        ("Use both passwords", "ใช้รหัสผ่านทั้งสองแบบ"),
        ("Set permanent password", "ตั้งค่ารหัสผ่านถาวร"),
        ("Enable remote restart", "เปิดการใช้งานการรีสตาร์ทระบบทางไกล"),
        ("Restart remote device", "รีสตาร์ทอุปกรณ์ปลายทาง"),
        ("Are you sure you want to restart", "คุณแน่ใจหรือไม่ที่จะรีสตาร์ท"),
        ("Restarting remote device", "กำลังรีสตาร์ทระบบปลายทาง"),
        ("remote_restarting_tip", "ระบบปลายทางกำลังรีสตาร์ท กรุณาปิดกล่องข้อความนี้และดำเนินการเขื่อมต่อใหม่อีกครั้งด้วยรหัสผ่านถาวรหลังจากผ่านไปซักครู่"),
        ("Copied", "คัดลอกแล้ว"),
        ("Exit Fullscreen", "ออกจากเต็มหน้าจอ"),
        ("Fullscreen", "เต็มหน้าจอ"),
        ("Mobile Actions", "การดำเนินการบนมือถือ"),
        ("Select Monitor", "เลือกหน้าจอ"),
        ("Control Actions", "การดำเนินการควบคุม"),
        ("Display Settings", "การตั้งค่าแสดงผล"),
        ("Ratio", "อัตราส่วน"),
        ("Image Quality", "คุณภาพภาพ"),
        ("Scroll Style", "ลักษณะการเลื่อน"),
        ("Show Toolbar", "แสดงแถบเครื่องมือ"),
        ("Hide Toolbar", "ซ่อนแถบเครื่องมือ"),
        ("Direct Connection", "การเชื่อมต่อตรง"),
        ("Relay Connection", "การเชื่อมต่อแบบ Relay "),
        ("Secure Connection", "การเชื่อมต่อที่ปลอดภัย"),
        ("Insecure Connection", "การเชื่อมต่อที่ไม่ปลอดภัย"),
        ("Scale original", "ขนาดเดิม"),
        ("Scale adaptive", "ขนาดยืดหยุ่น"),
        ("General", "ทั่วไป"),
        ("Security", "ความปลอดภัย"),
        ("Theme", "ธีม"),
        ("Dark Theme", "ธีมมืด"),
        ("Light Theme", "ธีมสว่าง"),
        ("Dark", "มืด"),
        ("Light", "สว่าง"),
        ("Follow System", "ตามระบบ"),
        ("Enable hardware codec", "เปิดการใช้งานฮาร์ดแวร์ codec"),
        ("Unlock Security Settings", "ปลดล็อคการตั้งค่าความปลอดภัย"),
        ("Enable audio", "เปิดการใช้งานเสียง"),
        ("Unlock Network Settings", "ปลดล็อคการตั้งค่าเครือข่าย"),
        ("Server", "เซิร์ฟเวอร์"),
        ("Direct IP Access", "การเข้าถึง IP ตรง"),
        ("Proxy", "พรอกซี"),
        ("Apply", "นำไปใช้"),
        ("Disconnect all devices?", "ยกเลิกการเชื่อมต่ออุปกรณ์ทั้งหมด?"),
        ("Clear", "ล้างข้อมูล"),
        ("Audio Input Device", "อุปกรณ์รับอินพุทข้อมูลเสียง"),
        ("Use IP Whitelisting", "ใช้งาน IP ไวท์ลิสต์"),
        ("Network", "เครือข่าย"),
        ("Pin Toolbar", "ปักหมุดแถบเครื่องมือ"),
        ("Unpin Toolbar", "ยกเลิกการปักหมุดแถบเครื่องมือ"),
        ("Recording", "การบันทึก"),
        ("Directory", "ไดเรกทอรี่"),
        ("Automatically record incoming sessions", "บันทึกเซสชันขาเข้าโดยอัตโนมัติ"),
        ("Change", "เปลี่ยน"),
        ("Start session recording", "เริ่มต้นการบันทึกเซสชัน"),
        ("Stop session recording", "หยุดการบันทึกเซสซัน"),
        ("Enable recording session", "เปิดใช้งานการบันทึกเซสชัน"),
        ("Enable LAN discovery", "เปิดการใช้งานการค้นหาในวง LAN"),
        ("Deny LAN discovery", "ปฏิเสธการใช้งานการค้นหาในวง LAN"),
        ("Write a message", "เขียนข้อความ"),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", "กรุณารอการยืนยันจาก UAC..."),
        ("elevated_foreground_window_tip", "หน้าต่างปัจจุบันของเครื่องปลายทางต้องการสิทธิ์การใช้งานที่สูงขึ้นสำหรับการทำงาน ดังนั้นเมาส์และคีย์บอร์ดจะไม่สามารถใช้งานได้ชั่วคราว คุณสามารถขอผู้ใช้งานปลายทางให้ย่อหน้าต่าง หรือคลิกปุ่มให้สิทธิ์การใช้งานในหน้าต่างการจัดการการเชื่อมต่อ เพื่อหลีกเลี่ยงปัญหานี้เราแนะนำให้ดำเนินการติดตั้งซอฟท์แวร์ในเครื่องปลายทาง"),
        ("Disconnected", "ยกเลิกการเชื่อมต่อ"),
        ("Other", "อื่นๆ"),
        ("Confirm before closing multiple tabs", "ยืนยันการปิดหลายแท็บ"),
        ("Keyboard Settings", "การตั้งค่าคีย์บอร์ด"),
        ("Full Access", "การเข้าถึงทั้งหมด"),
        ("Screen Share", "การแชร์จอ"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland ต้องการ Ubuntu เวอร์ชัน 21.04 หรือสูงกว่า"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland ต้องการลินุกซ์เวอร์ชันที่สูงกว่านี้ กรุณาเปลี่ยนไปใช้เดสก์ท็อป X11 หรือเปลี่ยนระบบปฏิบัติการของคุณ"),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "กรุณาเลือกหน้าจอที่ต้องการแชร์ (ใช้งานในอีกฝั่งของการเชื่อมต่อ)"),
        ("Show RustDesk", "แสดง RustDesk"),
        ("This PC", "พีซีเครื่องนี้"),
        ("or", "หรือ"),
        ("Continue with", "ทำต่อด้วย"),
        ("Elevate", "ยกระดับ"),
        ("Zoom cursor", "ขยายเคอร์เซอร์"),
        ("Accept sessions via password", "ยอมรับการเชื่อมต่อด้วยรหัสผ่าน"),
        ("Accept sessions via click", "ยอมรับการเชื่อมต่อด้วยการคลิก"),
        ("Accept sessions via both", "ยอมรับการเชื่อมต่อด้วยทั้งสองวิธิ"),
        ("Please wait for the remote side to accept your session request...", "กรุณารอให้อีกฝั่งยอมรับการเชื่อมต่อของคุณ..."),
        ("One-time Password", "รหัสผ่านครั้งเดียว"),
        ("Use one-time password", "ใช้รหัสผ่านครั้งเดียว"),
        ("One-time password length", "ความยาวรหัสผ่านครั้งเดียว"),
        ("Request access to your device", "คำขอการเข้าถึงอุปกรณ์ของคุณ"),
        ("Hide connection management window", "ซ่อนหน้าต่างการจัดการการเชื่อมต่อ"),
        ("hide_cm_tip", "อนุญาตการซ่อนก็ต่อเมื่อยอมรับการเชื่อมต่อด้วยรหัสผ่าน และต้องเป็นรหัสผ่านถาวรเท่านั้น"),
        ("wayland_experiment_tip", "การสนับสนุน Wayland ยังอยู่ในขั้นตอนการทดลอง กรุณาใช้ X11 หากคุณต้องการใช้งานการเข้าถึงแบบไม่มีผู้ดูแล"),
        ("Right click to select tabs", "คลิกขวาเพื่อเลือกแท็บ"),
        ("Skipped", "ข้าม"),
        ("Add to address book", "เพิ่มไปยังสมุดรายชื่อ"),
        ("Group", "กลุ่ม"),
        ("Search", "ค้นหา"),
        ("Closed manually by web console", "ถูกปิดโดยเว็บคอนโซล"),
        ("Local keyboard type", "ประเภทคีย์บอร์ด"),
        ("Select local keyboard type", "เลือกประเภทคีย์บอร์ด"),
        ("software_render_tip", "ถ้าคุณใช้กราฟิกการ์ดกับระบบ Linux และหน้าต่างของเครื่องปลายทางปิดในทันทีหลังจากการเชื่อมต่อ การเปลี่ยนไปใช้ไดรเวอร์ Nouveau และเลือกใช้โหมดการเรนเดอร์แบบซอฟท์แวร์อาจช่วยได้ (ต้องรีสตาร์ทโปรแกรม)"),
        ("Always use software rendering", "ใช้การเรนเดอร์แบบซอฟท์แวร์เสมอ"),
        ("config_input", "เพื่อที่จะควบคุมเครื่องเดสก์ท็อปปลายทางด้วยคีย์บอร์ด คุณจำเป็นจะต้องอนุญาตสิทธิ์ \"การตรวจสอบ Input\" ให้แก่ RustDesk"),
        ("config_microphone", "เพื่อที่จะส่งเสียงพูดไปยังปลายทาง คุณจำเป็นจะต้องอนุญาตสิทธิ์ \"การบันทึกเสียง\" ให้แก่ RustDesk"),
        ("request_elevation_tip", "คุณสามารถขอยกระดับสิทธิ์การใช้งานได้ หากมีผู้ใช้งานอยู่ในฝั่งเครื่องปลายทาง"),
        ("Wait", "รอ"),
        ("Elevation Error", "การยกระดับสิทธิ์การใช้งานผิดพลาด"),
        ("Ask the remote user for authentication", "ขอความช่วยเหลือผู้ใช้งานปลายทางเพื่อพิสูจน์ตัวตน"),
        ("Choose this if the remote account is administrator", "เลือกข้อนี้ถ้าบัญชีผู้ใช้งานปลายทางเป็นผู้ดูแลระบบ"),
        ("Transmit the username and password of administrator", "ส่งข้อมูลผู้ใช้งานและรหัสผ่านของผู้ดูแลระบบ"),
        ("still_click_uac_tip", "ผู้ใช้งานปลายทางยังจำเป็นที่จะต้องกดปุ่ม ตกลง บนหน้าต่าง UAC ของ RustDesk"),
        ("Request Elevation", "ขอยกระดับสิทธิ์การใช้งาน"),
        ("wait_accept_uac_tip", "กรุณารอผู้ใช้งานปลายทางกดยินยอมหน้าต่าง UAC"),
        ("Elevate successfully", "การยกระดับสิทธิ์การใช้งานสำเร็จ"),
        ("uppercase", "พิมพ์ใหญ่"),
        ("lowercase", "พิมพ์เล็ก"),
        ("digit", "หลัก"),
        ("special character", "อักขระพิเศษ"),
        ("length>=8", "ความยาวมากกว่า 8"),
        ("Weak", "ไม่ปลอดภัย"),
        ("Medium", "กลาง"),
        ("Strong", "ปลอดภัย"),
        ("Switch Sides", "สลับฝั่ง"),
        ("Please confirm if you want to share your desktop?", "กรุณายืนยันว่าคุณต้องการแบ่งปันหน้าเดสก์ท็อปของคุณ"),
        ("Display", "จอแสดงผล"),
        ("Default View Style", "แสดงผลแบบเริ่มต้น"),
        ("Default Scroll Style", "การเลื่อนแบบเริ่มต้น"),
        ("Default Image Quality", "คุณภาพของภาพแบบเริ่มต้น"),
        ("Default Codec", "Codec เริ่มต้น"),
        ("Bitrate", ""),
        ("FPS", ""),
        ("Auto", "อัตโนมัติ"),
        ("Other Default Options", "ตัวเลือกเริ่มต้นอื่นๆ"),
        ("Voice call", "การโทรด้วยเสียง"),
        ("Text chat", "การสนทนาด้วยข้อความ"),
        ("Stop voice call", "หยุดการโทรด้วยเสียง"),
        ("relay_hint_tip", "การเชื่อมต่อโดยตรงอาจเป็นไปไม่ได้ ดังนั้นคุณสามารถลองเชื่อมต่อผ่าน Relay หรือตั้งค่าให้เชื่อมต่อผ่าน Relay  เป็นค่าเริ่มต้น คุณสามารถเพิ่ม \"/r\" ต่อท้ายไปยัง ID หรือเลือกตัวเลือก \"เชื่อมต่อผ่าน Relay เสมอ\" ในการ์ดของการเชื่อมต่อล่าสุด (ถ้ามี)"),
        ("Reconnect", "เชื่อมต่ออีกครั้ง"),
        ("Codec", ""),
        ("Resolution", "ความละเอียด"),
        ("No transfers in progress", "ไม่มีการถ่ายโอนในขณะนี้"),
        ("Set one-time password length", "ตั้งค่าความยาวรหัสผ่านครั้งเดียว"),
        ("RDP Settings", "การตั้งค่า RDP"),
        ("Sort by", "เรียงลำดับโดย"),
        ("New Connection", "การเชื่อมต่อใหม่"),
        ("Restore", "คืนค่า"),
        ("Minimize", "ย่อ"),
        ("Maximize", "ขยาย"),
        ("Your Device", "อุปกรณ์ของคุณ"),
        ("empty_recent_tip", "คุณยังไม่มีการเชื่อมต่อล่าสุด ได้เวลาวางแผนเพื่อเริ่มต้นแล้ว"),
        ("empty_favorite_tip", "ยังไม่มีการเชื่อมต่อรายการโปรดเหรอ? มาเริ่มต้นหาใครซักคนเพื่อเชื่อมต่อด้วย และเพิ่มเข้าไปยังรายการโปรดของคุณกัน"),
        ("empty_lan_tip", "ไม่นะ ดูเหมือนว่าเราจะยังไม่พบใครตรงนี้"),
        ("empty_address_book_tip", "ดูเหมือนว่าคุณยังไม่มีใครถูกบันทึกในสมุดรายชื่อของคุณ"),
        ("eg: admin", "เช่น ผู้ดูแลระบบ"),
        ("Empty Username", "ชื่อผู้ใช้งานว่างเปล่า"),
        ("Empty Password", "รหัสผ่านว่างเปล่า"),
        ("Me", "ฉัน"),
        ("identical_file_tip", "ไฟล์นี้เหมือนกับไฟล์ของอีกฝั่ง"),
        ("show_monitors_tip", "แสดงหน้าจอในแถบเครื่องมือ"),
        ("View Mode", "โหมดการดู"),
        ("login_linux_tip", "คุณจำเป็นจะต้องเข้าสู่ระบบไปยังบัญชีลินุกซ์ปลายทางเพื่อใช้งานเดสก์ท็อปเซสชัน X"),
        ("verify_rustdesk_password_tip", "ยืนยันความถูกต้องรหัสผ่านของ RustDesk"),
        ("remember_account_tip", "จดจำบัญชีนี้"),
        ("os_account_desk_tip", "บัญชีนี้จะถูกใช้ในการเข้าสู่ระบบเครื่องปลายทางและเริ่มใช้งานเดสก์ท็อปเซสชันแบบ headless"),
        ("OS Account", "บัญชีระบบปฏิบัติการ"),
        ("another_user_login_title_tip", "ผู้ใช้งานอื่นเข้าสู่ระบบอยู่แล้ว"),
        ("another_user_login_text_tip", "ยกเลิกการเชื่อมต่อ"),
        ("xorg_not_found_title_tip", "ไม่พบ Xorg"),
        ("xorg_not_found_text_tip", "กรุณาติดตั้ง Xorg"),
        ("no_desktop_title_tip", "ไม่มีหน้าเดสก์ท็อปที่ใช้งานได้"),
        ("no_desktop_text_tip", "กรุณาติดตั้ง GNOME เดสกท็อป"),
        ("No need to elevate", "ไม่จำเป็นต้องยกระดับสิทธิ์การใช้งาน"),
        ("System Sound", "เสียงของระบบ"),
        ("Default", "ค่าเริ่มต้น"),
        ("New RDP", "RDP ใหม่"),
        ("Fingerprint", "ลายนิ้วมือ"),
        ("Copy Fingerprint", "คัดลอกลายนิ้วมือ"),
        ("no fingerprints", "ไม่มีลายนิ้วมือ"),
        ("Select a peer", "เลือกผู้ใช้งาน"),
        ("Select peers", "เลือกผู้ใช้งาน"),
        ("Plugins", "ปลั๊กอิน"),
        ("Uninstall", "ถอนการติดตั้ง"),
        ("Update", "อัปเดต"),
        ("Enable", "เปิดใช้งาน"),
        ("Disable", "ปิดใช้งาน"),
        ("Options", "ตัวเลือก"),
        ("resolution_original_tip", "ความละเอียดดั้งเดิม"),
        ("resolution_fit_local_tip", "ความละเอียดตามต้นทาง"),
        ("resolution_custom_tip", "ความละเอียดแบบกำหนดเอง"),
        ("Collapse toolbar", "พับแถบเครื่องมือ"),
        ("Accept and Elevate", "ยอมรับ และยกระดับสิทธิ์การใช้งาน"),
        ("accept_and_elevate_btn_tooltip", "ยอมรับการเชื่อมต่อ และยกระดับสิทธิ์ UAC"),
        ("clipboard_wait_response_timeout_tip", "หมดเวลารอการตอบสนองของการคัดลอก"),
        ("Incoming connection", "การเชื่อมต่อขาเข้า"),
        ("Outgoing connection", "การเชื่อมต่อขาออก"),
        ("Exit", "ออก"),
        ("Open", "เปิด"),
        ("logout_tip", "คุณแน่ใจที่จะออกจากระบบหรือไม่?"),
        ("Service", "เซอร์วิส"),
        ("Start", "เริ่ม"),
        ("Stop", "หยุด"),
        ("exceed_max_devices", "จำนวนอุปกรณ์ที่จัดการของคุณเต็มจำนวนแล้ว"),
        ("Sync with recent sessions", "Sync กับเซสชันล่าสุด"),
        ("Sort tags", "เรียงแท็ก"),
        ("Open connection in new tab", "เริ่มการเชื่อมต่อในแท็บใหม่"),
        ("Move tab to new window", "ย้ายแท็บไปหน้าต่างใหม่"),
        ("Can not be empty", "ไม่สามารถเว้นว่างได้"),
        ("Already exists", "มีอยู่แล้ว"),
        ("Change Password", "เปลี่ยนรหัสผ่าน"),
        ("Refresh Password", "รีเฟรชรหัสผ่าน"),
        ("ID", ""),
        ("Grid View", "มุมมองแบบช่อง"),
        ("List View", "มุมมองแบบรายการ"),
        ("Select", "เลือก"),
        ("Toggle Tags", "สลับแท็ก"),
        ("pull_ab_failed_tip", "การรีเฟรชสมุดรายชื่อล้มเหลว"),
        ("push_ab_failed_tip", "การ Sync สมุดรายชื่อไปยังเซิร์ฟเวอร์ล้มเหลว"),
        ("synced_peer_readded_tip", "อุปกรณ์ที่อยู่ในรายการล่าสุดจะถูก sync กลับไปยังสมุดรายชื่อ"),
        ("Change Color", "เปลี่ยนสี"),
        ("Primary Color", "สีหลัก"),
        ("HSV Color", "สี HSV"),
        ("Installation Successful!", "การติดตั้งเสร็จสมบูรณ์"),
        ("Installation failed!", "การติดตั้งล้มเหลว"),
        ("Reverse mouse wheel", "เลื่อมลูกกลิ้งเมาส์แบบกลับด้าน"),
        ("{} sessions", "{} เซสชัน"),
        ("scam_title", "คุณอาจกำลังถูกหลอกลวง!"),
        ("scam_text1", "ถ้าคุณกำลังคุยโทรศัพท์กับคนที่คุณไม่รู้จักและไม่ไว้ใจ และคนๆนั้นกำลังขอให้คุณเปิดใช้งาน RustDesk อย่าทำตามและรีบวางสายในทันที"),
        ("scam_text2", "เขาเหล่านั้นอาจเป็นมิจฉาชีพที่กำลังพยายามจะขโมยเงินและข้อมูลส่วนตัวของคุณ"),
        ("Don't show again", "อย่าแสดงอีก"),
        ("I Agree", "ยอมรับ"),
        ("Decline", "ปฏิเสธ"),
        ("Timeout in minutes", "หมดเวลาในอีกซักครู่"),
        ("auto_disconnect_option_tip", "ยกเลิกการเชื่อมต่ออัตโนมัติในกรณีที่ผู้ใช้งานไม่มีการเคลื่อนไหว"),
        ("Connection failed due to inactivity", "การเชื่อมต่อล้มเหลวเนื่องจากไม่มีการเคลื่อนไหว"),
        ("Check for software update on startup", "ตรวจสอบการอัปเดตโปรแกรมเมื่อเริ่มต้นใช้งาน"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", "กรุณาอัปเดต RustDesk Server Pro ไปยังเวอร์ชัน {} หรือใหม่กว่า!"),
        ("pull_group_failed_tip", "การเรียกใช้งานกลุ่มล้มเหลว"),
        ("Filter by intersection", ""),
        ("Remove wallpaper during incoming sessions", ""),
        ("Test", ""),
        ("display_is_plugged_out_msg", ""),
        ("No displays", ""),
        ("elevated_switch_display_msg", ""),
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
        ("switch_display_elevated_connections_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("capture_display_elevated_connections_tip", ""),
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
    ].iter().cloned().collect();
}
