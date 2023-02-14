lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Estado"),
        ("Your Desktop", "Tu escritorio"),
        ("desk_tip", "Puedes acceder a tu escritorio con esta ID y contraseña."),
        ("Password", "Contraseña"),
        ("Ready", "Listo"),
        ("Established", "Establecido"),
        ("connecting_status", "Conexión a la red RustDesk en progreso..."),
        ("Enable Service", "Habilitar Servicio"),
        ("Start Service", "Iniciar Servicio"),
        ("Service is running", "El servicio se está ejecutando"),
        ("Service is not running", "El servicio no se está ejecutando"),
        ("not_ready_status", "No está listo. Comprueba tu conexión"),
        ("Control Remote Desktop", "Controlar escritorio remoto"),
        ("Transfer File", "Transferir archivo"),
        ("Connect", "Conectar"),
        ("Recent Sessions", "Sesiones recientes"),
        ("Address Book", "Libreta de direcciones"),
        ("Confirmation", "Confirmación"),
        ("TCP Tunneling", "Túnel TCP"),
        ("Remove", "Quitar"),
        ("Refresh random password", "Actualizar contraseña aleatoria"),
        ("Set your own password", "Establece tu propia contraseña"),
        ("Enable Keyboard/Mouse", "Habilitar teclado/ratón"),
        ("Enable Clipboard", "Habilitar portapapeles"),
        ("Enable File Transfer", "Habilitar transferencia de archivos"),
        ("Enable TCP Tunneling", "Habilitar túnel TCP"),
        ("IP Whitelisting", "Direcciones IP admitidas"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Import Server Config", "Importar configuración de servidor"),
        ("Export Server Config", "Exportar configuración del servidor"),
        ("Import server configuration successfully", "Configuración de servidor importada con éxito"),
        ("Export server configuration successfully", "Configuración de servidor exportada con éxito"),
        ("Invalid server configuration", "Configuración de servidor incorrecta"),
        ("Clipboard is empty", "El portapapeles está vacío"),
        ("Stop service", "Detener servicio"),
        ("Change ID", "Cambiar ID"),
        ("Website", "Sitio web"),
        ("About", "Acerca de"),
        ("Slogan_tip", "Hecho con corazón en este mundo caótico!"),
        ("Privacy Statement", "Declaración de privacidad"),
        ("Mute", "Silenciar"),
        ("Build Date", ""),
        ("Version", ""),
        ("Home", ""),
        ("Audio Input", "Entrada de audio"),
        ("Enhancements", "Mejoras"),
        ("Hardware Codec", "Códec de hardware"),
        ("Adaptive Bitrate", "Tasa de bits adaptativa"),
        ("ID Server", "Servidor de IDs"),
        ("Relay Server", "Servidor Relay"),
        ("API Server", "Servidor API"),
        ("invalid_http", "debe comenzar con http:// o https://"),
        ("Invalid IP", "IP incorrecta"),
        ("id_change_tip", "Solo puedes usar caracteres a-z, A-Z, 0-9 e _ (guion bajo). El primer carácter debe ser a-z o A-Z. La longitud debe estar entre 6 y 16 caracteres."),
        ("Invalid format", "Formato incorrecto"),
        ("server_not_support", "Aún no es compatible con el servidor"),
        ("Not available", "No disponible"),
        ("Too frequent", "Demasiado frecuente"),
        ("Cancel", "Cancelar"),
        ("Skip", "Omitir"),
        ("Close", "Cerrar"),
        ("Retry", "Reintentar"),
        ("OK", ""),
        ("Password Required", "Se requiere contraseña"),
        ("Please enter your password", "Por favor, introduzca su contraseña"),
        ("Remember password", "Recordar contraseña"),
        ("Wrong Password", "Contraseña incorrecta"),
        ("Do you want to enter again?", "Quieres volver a entrar?"),
        ("Connection Error", "Error de conexión"),
        ("Error", ""),
        ("Reset by the peer", "Restablecido por el par"),
        ("Connecting...", "Conectando..."),
        ("Connection in progress. Please wait.", "Conexión en curso. Espere por favor."),
        ("Please try 1 minute later", "Intente 1 minuto más tarde"),
        ("Login Error", "Error de inicio de sesión"),
        ("Successful", "Exitoso"),
        ("Connected, waiting for image...", "Conectado, esperando imagen..."),
        ("Name", "Nombre"),
        ("Type", "Tipo"),
        ("Modified", "Modificado"),
        ("Size", "Tamaño"),
        ("Show Hidden Files", "Mostrar archivos ocultos"),
        ("Receive", "Recibir"),
        ("Send", "Enviar"),
        ("Refresh File", "Actualizar archivo"),
        ("Local", ""),
        ("Remote", "Remoto"),
        ("Remote Computer", "Computadora remota"),
        ("Local Computer", "Computadora local"),
        ("Confirm Delete", "Confirmar eliminación"),
        ("Delete", "Eliminar"),
        ("Properties", "Propiedades"),
        ("Multi Select", "Selección múltiple"),
        ("Select All", "Seleccionar Todo"),
        ("Unselect All", "Deseleccionar Todo"),
        ("Empty Directory", "Directorio vacío"),
        ("Not an empty directory", "No es un directorio vacío"),
        ("Are you sure you want to delete this file?", "Estás seguro de que quieres eliminar este archivo?"),
        ("Are you sure you want to delete this empty directory?", "Está seguro de que desea eliminar este directorio vacío?"),
        ("Are you sure you want to delete the file of this directory?", "Está seguro de que desea eliminar el archivo de este directorio?"),
        ("Do this for all conflicts", "Haga esto para todos los conflictos"),
        ("This is irreversible!", "Esto es irreversible!"),
        ("Deleting", "Eliminando"),
        ("files", "archivos"),
        ("Waiting", "Esperando"),
        ("Finished", "Terminado"),
        ("Speed", "Velocidad"),
        ("Custom Image Quality", "Calidad de imagen personalizada"),
        ("Privacy mode", "Modo privado"),
        ("Block user input", "Bloquear entrada de usuario"),
        ("Unblock user input", "Desbloquear entrada de usuario"),
        ("Adjust Window", "Ajustar ventana"),
        ("Original", "Original"),
        ("Shrink", "Encoger"),
        ("Stretch", "Estirar"),
        ("Scrollbar", "Barra de desplazamiento"),
        ("ScrollAuto", "Desplazamiento automático"),
        ("Good image quality", "Buena calidad de imagen"),
        ("Balanced", "Equilibrado"),
        ("Optimize reaction time", "Optimizar el tiempo de reacción"),
        ("Custom", "Personalizado"),
        ("Show remote cursor", "Mostrar cursor remoto"),
        ("Show quality monitor", "Mostrar calidad del monitor"),
        ("Disable clipboard", "Deshabilitar portapapeles"),
        ("Lock after session end", "Bloquear después del final de la sesión"),
        ("Insert", "Insertar"),
        ("Insert Lock", "Insertar bloqueo"),
        ("Refresh", "Actualizar"),
        ("ID does not exist", "La ID no existe"),
        ("Failed to connect to rendezvous server", "No se pudo conectar al servidor de encuentro"),
        ("Please try later", "Por favor intente mas tarde"),
        ("Remote desktop is offline", "El escritorio remoto está desconectado"),
        ("Key mismatch", "La clave no coincide"),
        ("Timeout", "Tiempo agotado"),
        ("Failed to connect to relay server", "No se pudo conectar al servidor de retransmisión"),
        ("Failed to connect via rendezvous server", "No se pudo conectar a través del servidor de encuentro"),
        ("Failed to connect via relay server", "No se pudo conectar a través del servidor de retransmisión"),
        ("Failed to make direct connection to remote desktop", "No se pudo establecer la conexión directa con el escritorio remoto"),
        ("Set Password", "Configurar la contraseña"),
        ("OS Password", "Contraseña del sistema operativo"),
        ("install_tip", "Debido al Control de cuentas de usuario, es posible que RustDesk no funcione correctamente como escritorio remoto. Para evitar este problema, haga clic en el botón de abajo para instalar RustDesk a nivel de sistema."),
        ("Click to upgrade", "Clic para actualizar"),
        ("Click to download", "Clic para descargar"),
        ("Click to update", "Clic para refrescar"),
        ("Configure", "Configurar"),
        ("config_acc", "Para controlar su escritorio desde el exterior, debe otorgar permiso a RustDesk de \"Accesibilidad\"."),
        ("config_screen", "Para controlar su escritorio desde el exterior, debe otorgar permiso a RustDesk de \"Grabación de pantalla\"."),
        ("Installing ...", "Instalando ..."),
        ("Install", "Instalar"),
        ("Installation", "Instalación"),
        ("Installation Path", "Ruta de instalación"),
        ("Create start menu shortcuts", "Crear accesos directos en el menú de inicio"),
        ("Create desktop icon", "Crear icono de escritorio"),
        ("agreement_tip", "Al iniciar la instalación, acepta los términos del acuerdo de licencia."),
        ("Accept and Install", "Aceptar e instalar"),
        ("End-user license agreement", "Acuerdo de licencia de usuario final"),
        ("Generating ...", "Generando ..."),
        ("Your installation is lower version.", "Su instalación es una versión inferior."),
        ("not_close_tcp_tip", "No cierre esta ventana mientras esté usando el túnel"),
        ("Listening ...", "Escuchando ..."),
        ("Remote Host", "Anfitrión remoto"),
        ("Remote Port", "Puerto remoto"),
        ("Action", "Acción"),
        ("Add", "Agregar"),
        ("Local Port", "Puerto local"),
        ("Local Address", "Dirección Local"),
        ("Change Local Port", "Cambiar Puerto Local"),
        ("setup_server_tip", "Para una conexión más rápida, configure su propio servidor"),
        ("Too short, at least 6 characters.", "Demasiado corto, al menos 6 caracteres."),
        ("The confirmation is not identical.", "La confirmación no coincide."),
        ("Permissions", "Permisos"),
        ("Accept", "Aceptar"),
        ("Dismiss", "Cancelar"),
        ("Disconnect", "Desconectar"),
        ("Allow using keyboard and mouse", "Permitir el uso del teclado y el mouse"),
        ("Allow using clipboard", "Permitir usar portapapeles"),
        ("Allow hearing sound", "Permitir escuchar sonido"),
        ("Allow file copy and paste", "Permitir copiar y pegar archivos"),
        ("Connected", "Conectado"),
        ("Direct and encrypted connection", "Conexión directa y cifrada"),
        ("Relayed and encrypted connection", "Conexión retransmitida y cifrada"),
        ("Direct and unencrypted connection", "Conexión directa y sin cifrar"),
        ("Relayed and unencrypted connection", "Conexión retransmitida y sin cifrar"),
        ("Enter Remote ID", "Introduzca el ID remoto"),
        ("Enter your password", "Introduzca su contraseña"),
        ("Logging in...", "Iniciando sesión..."),
        ("Enable RDP session sharing", "Habilitar el uso compartido de sesiones RDP"),
        ("Auto Login", "Inicio de sesión automático"),
        ("Enable Direct IP Access", "Habilitar acceso IP directo"),
        ("Rename", "Renombrar"),
        ("Space", "Espacio"),
        ("Create Desktop Shortcut", "Crear acceso directo del escritorio"),
        ("Change Path", "Cambiar ruta"),
        ("Create Folder", "Crear carpeta"),
        ("Please enter the folder name", "Por favor introduzca el nombre de la carpeta"),
        ("Fix it", "Resolver"),
        ("Warning", "Aviso"),
        ("Login screen using Wayland is not supported", "La pantalla de inicio de sesión con Wayland no es compatible"),
        ("Reboot required", "Reinicio requerido"),
        ("Unsupported display server ", "Servidor de visualización no compatible"),
        ("x11 expected", "x11 necesario"),
        ("Port", "Puerto"),
        ("Settings", "Ajustes"),
        ("Username", " Nombre de usuario"),
        ("Invalid port", "Puerto incorrecto"),
        ("Closed manually by the peer", "Cerrado manualmente por el par"),
        ("Enable remote configuration modification", "Habilitar modificación remota de configuración"),
        ("Run without install", "Ejecutar sin instalar"),
        ("Connect via relay", ""),
        ("Always connect via relay", "Conéctese siempre a través de relay"),
        ("whitelist_tip", "Solo las direcciones IP autorizadas pueden conectarse a este escritorio"),
        ("Login", "Iniciar sesión"),
        ("Verify", "Verificar"),
        ("Remember me", "Recordarme"),
        ("Trust this device", "Confiar en este dispositivo"),
        ("Verification code", "Código de verificación"),
        ("verification_tip", "Se ha detectado un nuevo dispositivo y se ha enviado un código de verificación a la dirección de correo registrada. Introduzca el código de verificación para continuar con el inicio de sesión."),
        ("Logout", "Salir"),
        ("Tags", "Tags"),
        ("Search ID", "Buscar ID"),
        ("whitelist_sep", "Separados por coma, punto y coma, espacio o nueva línea"),
        ("Add ID", "Agregar ID"),
        ("Add Tag", "Agregar tag"),
        ("Unselect all tags", "Deseleccionar todos los tags"),
        ("Network error", "Error de red"),
        ("Username missed", "Olvidó su nombre de usuario"),
        ("Password missed", "Olvidó su contraseña"),
        ("Wrong credentials", "Credenciales incorrectas"),
        ("Edit Tag", "Editar tag"),
        ("Unremember Password", "Olvidaste tu contraseña"),
        ("Favorites", "Favoritos"),
        ("Add to Favorites", "Agregar a favoritos"),
        ("Remove from Favorites", "Quitar de favoritos"),
        ("Empty", "Vacío"),
        ("Invalid folder name", "Nombre de carpeta incorrecto"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Hostname", "Nombre de host"),
        ("Discovered", "Descubierto"),
        ("install_daemon_tip", "Para comenzar en el encendido, debe instalar el servicio del sistema."),
        ("Remote ID", "ID remoto"),
        ("Paste", "Pegar"),
        ("Paste here?", "Pegar aqui?"),
        ("Are you sure to close the connection?", "Estás seguro de cerrar la conexión?"),
        ("Download new version", "Descargar nueva versión"),
        ("Touch mode", "Modo táctil"),
        ("Mouse mode", "Modo ratón"),
        ("One-Finger Tap", "Toque con un dedo"),
        ("Left Mouse", "Ratón izquierdo"),
        ("One-Long Tap", "Un toque largo"),
        ("Two-Finger Tap", "Toque con dos dedos"),
        ("Right Mouse", "Botón derecho"),
        ("One-Finger Move", "Movimiento con un dedo"),
        ("Double Tap & Move", "Toca dos veces y mueve"),
        ("Mouse Drag", "Arrastre de ratón"),
        ("Three-Finger vertically", "Tres dedos verticalmente"),
        ("Mouse Wheel", "Rueda de ratón"),
        ("Two-Finger Move", "Movimiento con dos dedos"),
        ("Canvas Move", "Movimiento de lienzo"),
        ("Pinch to Zoom", "Pellizcar para ampliar"),
        ("Canvas Zoom", "Ampliar lienzo"),
        ("Reset canvas", "Restablecer lienzo"),
        ("No permission of file transfer", "Sin permiso de transferencia de archivos"),
        ("Note", "Nota"),
        ("Connection", "Conexión"),
        ("Share Screen", "Compartir pantalla"),
        ("CLOSE", "CERRAR"),
        ("OPEN", "ABRIR"),
        ("Chat", "Chat"),
        ("Total", "Total"),
        ("items", "items"),
        ("Selected", "Seleccionado"),
        ("Screen Capture", "Captura de pantalla"),
        ("Input Control", "Control de entrada"),
        ("Audio Capture", "Captura de audio"),
        ("File Connection", "Conexión de archivos"),
        ("Screen Connection", "Conexión de pantalla"),
        ("Do you accept?", "Aceptas?"),
        ("Open System Setting", "Configuración del sistema abierto"),
        ("How to get Android input permission?", "Cómo obtener el permiso de entrada de Android?"),
        ("android_input_permission_tip1", "Para que un dispositivo remoto controle su dispositivo Android a través del mouse o toque, debe permitir que RustDesk use el servicio de \"Accesibilidad\"."),
        ("android_input_permission_tip2", "Vaya a la página de configuración del sistema que se abrirá a continuación, busque y acceda a [Servicios instalados], active el servicio [RustDesk Input]."),
        ("android_new_connection_tip", "Se recibió una nueva solicitud de control para el dispositivo actual."),
        ("android_service_will_start_tip", "Habilitar la captura de pantalla iniciará automáticamente el servicio, lo que permitirá que otros dispositivos soliciten una conexión desde este dispositivo."),
        ("android_stop_service_tip", "Cerrar el servicio cerrará automáticamente todas las conexiones establecidas."),
        ("android_version_audio_tip", "La versión actual de Android no admite la captura de audio, actualice a Android 10 o posterior."),
        ("android_start_service_tip", "Toque el permiso [Iniciar servicio] o ABRIR [Captura de pantalla] para iniciar el servicio de uso compartido de pantalla."),
        ("Account", "Cuenta"),
        ("Overwrite", "Sobrescribir"),
        ("This file exists, skip or overwrite this file?", "Este archivo existe, ¿omitir o sobrescribir este archivo?"),
        ("Quit", "Salir"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Ayuda"),
        ("Failed", "Fallido"),
        ("Succeeded", "Logrado"),
        ("Someone turns on privacy mode, exit", "Alguien active el modo privacidad, salga"),
        ("Unsupported", "No soportado"),
        ("Peer denied", "Par denegado"),
        ("Please install plugins", "Instale complementos"),
        ("Peer exit", "Par salio"),
        ("Failed to turn off", "Error al apagar"),
        ("Turned off", "Apagado"),
        ("In privacy mode", "En modo de privacidad"),
        ("Out privacy mode", "Fuera del modo de privacidad"),
        ("Language", "Idioma"),
        ("Keep RustDesk background service", "Dejar RustDesk como Servicio en 2do plano"),
        ("Ignore Battery Optimizations", "Ignorar optimizacioens de bateria"),
        ("android_open_battery_optimizations_tip", "Si deseas deshabilitar esta característica, por favor, ve a la página siguiente de ajustes, busca y entra en  [Batería] y desmarca [Sin restricción]"),
        ("Connection not allowed", "Conexión no disponible"),
        ("Legacy mode", "Modo heredado"),
        ("Map mode", "Modo mapa"),
        ("Translate mode", "Modo traducido"),
        ("Use permanent password", "Usar contraseña permamente"),
        ("Use both passwords", "Usar ambas contraseñas"),
        ("Set permanent password", "Establecer contraseña permamente"),
        ("Enable Remote Restart", "Habilitar reinicio remoto"),
        ("Allow remote restart", "Permitir reinicio remoto"),
        ("Restart Remote Device", "Reiniciar dispositivo"),
        ("Are you sure you want to restart", "Esta Seguro que desea reiniciar?"),
        ("Restarting Remote Device", "Reiniciando dispositivo remoto"),
        ("remote_restarting_tip", "El dispositivo remoto se está reiniciando. Por favor cierre este mensaje y vuelva a conectarse con la contraseña peremanente en unos momentos."),
        ("Copied", "Copiado"),
        ("Exit Fullscreen", "Salir de pantalla completa"),
        ("Fullscreen", "Pantalla completa"),
        ("Mobile Actions", "Acciones móviles"),
        ("Select Monitor", "Seleccionar monitor"),
        ("Control Actions", "Acciones de control"),
        ("Display Settings", "Configuración de pantalla"),
        ("Ratio", "Relación"),
        ("Image Quality", "Calidad de imagen"),
        ("Scroll Style", "Estilo de desplazamiento"),
        ("Show Menubar", "Mostrar barra de menú"),
        ("Hide Menubar", "Ocultar barra de menú"),
        ("Direct Connection", "Conexión directa"),
        ("Relay Connection", "Conexión Relay"),
        ("Secure Connection", "Conexión segura"),
        ("Insecure Connection", "Conexión insegura"),
        ("Scale original", "Escala original"),
        ("Scale adaptive", "Escala adaptativa"),
        ("General", ""),
        ("Security", "Seguridad"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tema Oscuro"),
        ("Dark", "Oscuro"),
        ("Light", "Claro"),
        ("Follow System", "Tema del sistema"),
        ("Enable hardware codec", "Habilitar códec por hardware"),
        ("Unlock Security Settings", "Desbloquear ajustes de seguridad"),
        ("Enable Audio", "Habilitar Audio"),
        ("Unlock Network Settings", "Desbloquear Ajustes de Red"),
        ("Server", "Servidor"),
        ("Direct IP Access", "Acceso IP Directo"),
        ("Proxy", ""),
        ("Apply", "Aplicar"),
        ("Disconnect all devices?", "¿Desconectar todos los dispositivos?"),
        ("Clear", "Borrar"),
        ("Audio Input Device", "Dispositivo de entrada de audio"),
        ("Deny remote access", "Denegar acceso remoto"),
        ("Use IP Whitelisting", "Usar lista de IPs admitidas"),
        ("Network", "Red"),
        ("Enable RDP", "Habilitar RDP"),
        ("Pin menubar", "Pin barra de menú"),
        ("Unpin menubar", "Desbloquear barra de menú"),
        ("Recording", "Grabando"),
        ("Directory", "Directorio"),
        ("Automatically record incoming sessions", "Grabación automática de sesiones entrantes"),
        ("Change", "Cambiar"),
        ("Start session recording", "Comenzar grabación de sesión"),
        ("Stop session recording", "Detener grabación de sesión"),
        ("Enable Recording Session", "Habilitar grabación de sesión"),
        ("Allow recording session", "Permitir grabación de sesión"),
        ("Enable LAN Discovery", "Habilitar descubrimiento de LAN"),
        ("Deny LAN Discovery", "Denegar descubrimiento de LAN"),
        ("Write a message", "Escribir un mensaje"),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", "Por favor, espera confirmación de UAC"),
        ("elevated_foreground_window_tip", "La ventana actual del escritorio remoto necesita privilegios elevados para funcionar, así que no puedes usar ratón y teclado temporalmente. Puedes solicitar al usuario remoto que minimize la ventana actual o hacer clic en el botón de elevación de la ventana de gestión de conexión. Para evitar este problema, se recomienda instalar el programa en el dispositivo remto."),
        ("Disconnected", "Desconectado"),
        ("Other", "Otro"),
        ("Confirm before closing multiple tabs", "Confirmar antes de cerrar múltiples pestañas"),
        ("Keyboard Settings", "Ajustes de teclado"),
        ("Full Access", "Acceso completo"),
        ("Screen Share", "Compartir pantalla"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland requiere Ubuntu 21.04 o una versión superior."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland requiere una versión superior de la distribución de Linux. Pruebe el escritorio X11 o cambie su sistema operativo."),
        ("JumpLink", "Ver"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Seleccione la pantalla que se compartirá (Operar en el lado del par)."),
        ("Show RustDesk", "Mostrar RustDesk"),
        ("This PC", "Este PC"),
        ("or", "o"),
        ("Continue with", "Continuar con"),
        ("Elevate", "Elevar"),
        ("Zoom cursor", "Ampliar cursor"),
        ("Accept sessions via password", "Aceptar sesiones a través de contraseña"),
        ("Accept sessions via click", "Aceptar sesiones a través de clic"),
        ("Accept sessions via both", "Aceptar sesiones a través de ambos"),
        ("Please wait for the remote side to accept your session request...", "Por favor, espere a que el lado remoto acepte su solicitud de sesión"),
        ("One-time Password", "Constaseña de un solo uso"),
        ("Use one-time password", "Usar contraseña de un solo uso"),
        ("One-time password length", "Longitud de la contraseña de un solo uso"),
        ("Request access to your device", "Solicitud de acceso a su dispositivo"),
        ("Hide connection management window", "Ocultar ventana de gestión de conexión"),
        ("hide_cm_tip", "Permitir ocultar solo si se aceptan sesiones a través de contraseña y usando contraseña permanente"),
        ("wayland_experiment_tip", "El soporte para Wayland está en fase experimental, por favor, use X11 si necesita acceso desatendido."),
        ("Right click to select tabs", "Clic derecho para seleccionar pestañas"),
        ("Skipped", "Omitido"),
        ("Add to Address Book", "Añadir a la libreta de direcciones"),
        ("Group", "Grupo"),
        ("Search", "Búsqueda"),
        ("Closed manually by web console", "Cerrado manualmente por la consola web"),
        ("Local keyboard type", "Tipo de teclado local"),
        ("Select local keyboard type", "Seleccionar tipo de teclado local"),
        ("software_render_tip", "Si tienes una gráfica Nvidia y la ventana remota se cierra inmediatamente, instalar el driver nouveau y elegir renderizado por software podría ayudar. Se requiere reiniciar la aplicación."),
        ("Always use software rendering", "Usar siempre renderizado por software"),
        ("config_input", "Para controlar el escritorio remoto con el teclado necesitas dar a RustDesk permisos de \"Monitorización de entrada\"."),
        ("config_microphone", ""),
        ("request_elevation_tip", "También puedes solicitar elevación si hay alguien en el lado remoto."),
        ("Wait", "Esperar"),
        ("Elevation Error", "Error de elevación"),
        ("Ask the remote user for authentication", "Pida autenticación al usuario remoto"),
        ("Choose this if the remote account is administrator", "Elegir si la cuenta remota es de administrador"),
        ("Transmit the username and password of administrator", "Transmitir usuario y contraseña del administrador"),
        ("still_click_uac_tip", "Aún se necesita que el usuario remoto haga click en OK en la ventana UAC del RusDesk en ejecución."),
        ("Request Elevation", "Solicitar Elevación"),
        ("wait_accept_uac_tip", "Por favor espere a que el usuario remoto acepte el diálogo UAC."),
        ("Elevate successfully", "Elevar con éxito"),
        ("uppercase", "mayúsculas"),
        ("lowercase", "minúsculas"),
        ("digit", "dígito"),
        ("special character", "carácter especial"),
        ("length>=8", "longitud>=8"),
        ("Weak", "Débil"),
        ("Medium", "Media"),
        ("Strong", "Fuerte"),
        ("Switch Sides", "Intercambiar lados"),
        ("Please confirm if you want to share your desktop?", "Por favor, confirma si quieres compartir tu escritorio"),
        ("Closed as expected", ""),
        ("Display", "Pantalla"),
        ("Default View Style", "Estilo de vista predeterminado"),
        ("Default Scroll Style", "Estilo de desplazamiento predeterminado"),
        ("Default Image Quality", "Calidad de imagen predeterminada"),
        ("Default Codec", "Códec predeterminado"),
        ("Bitrate", "Tasa de bits"),
        ("FPS", ""),
        ("Auto", ""),
        ("Other Default Options", "Otras opciones predeterminadas"),
        ("Voice call", "Llamada de voz"),
        ("Text chat", "Chat de texto"),
        ("Stop voice call", "Detener llamada de voz"),
        ("relay_hint_tip", ""),
    ].iter().cloned().collect();
}
