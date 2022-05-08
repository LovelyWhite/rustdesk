lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", ""),
        ("Your Desktop", ""),
        ("desk_tip", ""),
        ("Password", ""),
        ("Ready", ""),
        ("Established", ""),
        ("connecting_status", ""),
        ("Enable Service", ""),
        ("Start Service", ""),
        ("Service is running", ""),
        ("Service is not running", ""),
        ("not_ready_status", ""),
        ("Control Remote Desktop", ""),
        ("Transfer File", ""),
        ("Connect", ""),
        ("Recent Sessions", ""),
        ("Address Book", ""),
        ("Confirmation", ""),
        ("TCP Tunneling", ""),
        ("Remove", ""),
        ("Refresh random password", ""),
        ("Set your own password", ""),
        ("Enable Keyboard/Mouse", ""),
        ("Enable Clipboard", ""),
        ("Enable File Transfer", ""),
        ("Enable TCP Tunneling", ""),
        ("IP Whitelisting", ""),
        ("ID/Relay Server", ""),
        ("Stop service", ""),
        ("Change ID", ""),
        ("Website", ""),
        ("About", ""),
        ("Mute", ""),
        ("Audio Input", ""),
        ("ID Server", ""),
        ("Relay Server", ""),
        ("API Server", ""),
        ("invalid_http", ""),
        ("Invalid IP", ""),
        ("id_change_tip", ""),
        ("Invalid format", ""),
        ("This function is turned off by the server", ""),
        ("Not available", ""),
        ("Too frequent", ""),
        ("Cancel", ""),
        ("Skip", ""),
        ("Close", ""),
        ("Retry", ""),
        ("OK", ""),
        ("Password Required", ""),
        ("Please enter your password", ""),
        ("Remember password", ""),
        ("Wrong Password", ""),
        ("Do you want to enter again?", ""),
        ("Connection Error", ""),
        ("Error", ""),
        ("Reset by the peer", ""),
        ("Connecting...", ""),
        ("Connection in progress. Please wait.", ""),
        ("Please try 1 minute later", ""),
        ("Login Error", ""),
        ("Successful", ""),
        ("Connected, waiting for image...", ""),
        ("Name", ""),
        ("Type", ""),
        ("Modified", ""),
        ("Size", ""),
        ("Show Hidden Files", ""),
        ("Receive", ""),
        ("Send", ""),
        ("Refresh File", ""),
        ("Local", ""),
        ("Remote", ""),
        ("Remote Computer", ""),
        ("Local Computer", ""),
        ("Confirm Delete", ""),
        ("Delete", ""),
        ("Properties", ""),
        ("Multi Select", ""),
        ("Empty Directory", ""),
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
        ("Custom Image Quality", ""),
        ("Privacy mode", ""),
        ("Block user input", ""),
        ("Unblock user input", ""),
        ("Adjust Window", ""),
        ("Original", ""),
        ("Shrink", ""),
        ("Stretch", ""),
        ("Good image quality", ""),
        ("Balanced", ""),
        ("Optimize reaction time", ""),
        ("Custom", ""),
        ("Show remote cursor", ""),
        ("Disable clipboard", ""),
        ("Lock after session end", ""),
        ("Insert", ""),
        ("Insert Lock", ""),
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
        ("Set Password", ""),
        ("OS Password", ""),
        ("install_tip", ""),
        ("Click to upgrade", ""),
        ("Click to download", ""),
        ("Click to update", ""),
        ("Configure", ""),
        ("config_acc", ""),
        ("config_screen", ""),
        ("Installing ...", ""),
        ("Install", ""),
        ("Installation", ""),
        ("Installation Path", ""),
        ("Create start menu shortcuts", ""),
        ("Create desktop icon", ""),
        ("agreement_tip", ""),
        ("Accept and Install", ""),
        ("End-user license agreement", ""),
        ("Generating ...", ""),
        ("Your installation is lower version.", ""),
        ("not_close_tcp_tip", ""),
        ("Listening ...", ""),
        ("Remote Host", ""),
        ("Remote Port", ""),
        ("Action", ""),
        ("Add", ""),
        ("Local Port", ""),
        ("setup_server_tip", ""),
        ("Too short, at least 6 characters.", ""),
        ("The confirmation is not identical.", ""),
        ("Permissions", ""),
        ("Accept", ""),
        ("Dismiss", ""),
        ("Disconnect", ""),
        ("Allow using keyboard and mouse", ""),
        ("Allow using clipboard", ""),
        ("Allow hearing sound", ""),
        ("Allow file transfer", ""),
        ("File transfer", ""),
        ("Connected", ""),
        ("Direct and encrypted connection", ""),
        ("Relayed and encrypted connection", ""),
        ("Direct and unencrypted connection", ""),
        ("Relayed and unencrypted connection", ""),
        ("Enter Remote ID", ""),
        ("Enter your password", ""),
        ("Logging in...", ""),
        ("Enable RDP session sharing", ""),
        ("Auto Login", ""),
        ("Enable Direct IP Access", ""),
        ("Rename", ""),
        ("Space", ""),
        ("Create Desktop Shortcut", ""),
        ("Change Path", ""),
        ("Create Folder", ""),
        ("Please enter the folder name", ""),
        ("Fix it", ""),
        ("Warning", ""),
        ("Login screen using Wayland is not supported", ""),
        ("Reboot required", ""),
        ("Unsupported display server ", ""),
        ("x11 expected", ""),
        ("Port", ""),
        ("Settings", ""),
        ("Username", ""),
        ("Invalid port", ""),
        ("Closed manually by the peer", ""),
        ("Enable remote configuration modification", ""),
        ("Run without install", ""),
        ("Always connected via relay", ""),
        ("Always connect via relay", ""),
        ("whitelist_tip", ""),
        ("Login", ""),
        ("Logout", ""),
        ("Tags", ""),
        ("Search ID", ""),
        ("Current Wayland display server is not supported", ""),
        ("whitelist_sep", ""),
        ("Add ID", ""),
        ("Add Tag", ""),
        ("Unselect all tags", ""),
        ("Network error", ""),
        ("Username missed", ""),
        ("Password missed", ""),
        ("Wrong credentials", ""),
        ("Edit Tag", ""),
        ("Unremember Password", ""),
        ("Favorites", ""),
        ("Add to Favorites", ""),
        ("Remove from Favorites", ""),
        ("Empty", ""),
        ("Invalid folder name", ""),
        ("Socks5 Proxy", ""),
        ("Hostname", ""),
        ("Discovered", ""),
        ("install_daemon_tip", ""),
        ("Remote ID", ""),
        ("Paste", ""),
        ("Paste here?", ""),
        ("Are you sure to close the connection?", ""),
        ("Download new version", ""),
        ("Touch mode", ""),
        ("Mouse mode", ""),
        ("One-Finger Tap", ""),
        ("Left Mouse", ""),
        ("One-Long Tap", ""),
        ("Two-Finger Tap", ""),
        ("Right Mouse", ""),
        ("One-Finger Move", ""),
        ("Double Tap & Move", ""),
        ("Mouse Drag", ""),
        ("Two-Finger vertically", ""),
        ("Mouse Wheel", ""),
        ("Two-Finger Move", ""),
        ("Canvas Move", ""),
        ("Pinch to Zoom", ""),
        ("Canvas Zoom", ""),
        ("Reset canvas", ""),
        ("No permission of file transfer", ""),
        ("Note", ""),
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
        ("File Transfer", ""),
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
        ("Quit", ""),
        ("Help", ""),
    ].iter().cloned().collect();
}
