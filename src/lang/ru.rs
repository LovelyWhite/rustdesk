lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Cтатус"),
        ("Your Desktop", "Ваш рабочий стол"),
        ("desk_tip", "Ваш рабочий стол доступен с этим идентификатором и паролем"),
        ("Password", "Пароль"),
        ("Ready", "Готово"),
        ("Established", "Установлено"),
        ("connecting_status", "Подключение к сети RustDesk..."),
        ("Enable Service", "Включить службу"),
        ("Start Service", "Запустить службу"),
        ("Service is running", "Служба работает"),
        ("Service is not running", "Служба не запущена"),
        ("not_ready_status", "Не готово. Пожалуйста, проверьте подключение"),
        ("Control Remote Desktop", "Управление удаленным рабочим столом"),
        ("Transfer File", "Передать файл"),
        ("Connect", "Подключиться"),
        ("Recent Sessions", "Последние сеансы"),
        ("Address Book", "Адресная книга"),
        ("Confirmation", "Подтверждение"),
        ("TCP Tunneling", "TCP-туннелирование"),
        ("Remove", "Удалить"),
        ("Refresh random password", "Обновить случайный пароль"),
        ("Set your own password", "Установить свой пароль"),
        ("Enable Keyboard/Mouse", "Включить клавиатуру/мышь"),
        ("Enable Clipboard", "Включить буфер обмена"),
        ("Enable File Transfer", "Включить передачу файлов"),
        ("Enable TCP Tunneling", "Включить туннелирование TCP"),
        ("IP Whitelisting", "Список разрешенных IP-адресов"),
        ("ID/Relay Server", "ID/Сервер ретрансляции"),
        ("Stop service", "Остановить службу"),
        ("Change ID", "Изменить ID"),
        ("Website", "Веб-сайт"),
        ("About", "О RustDesk"),
        ("Mute", "Отключить звук"),
        ("Audio Input", "Аудиовход"),
        ("Enhancements", "Улучшения"),
        ("Hardware Codec", "Аппаратный кодек"),
        ("Adaptive Bitrate", "Адаптивная скорость потока"),
        ("ID Server", "ID-сервер"),
        ("Relay Server", "Сервер ретрансляции"),
        ("API Server", "API-сервер"),
        ("invalid_http", "Должен начинаться с http:// или https://"),
        ("Invalid IP", "Неверный IP-адрес"),
        ("id_change_tip", "Допускаются только символы a-z, A-Z, 0-9 и _ (подчеркивание). Первая буква должна быть a-z, A-Z. Длина от 6 до 16"),
        ("Invalid format", "Неверный формат"),
        ("server_not_support", "Пока не поддерживается сервером"),
        ("Not available", "Недоступно"),
        ("Too frequent", "Слишком часто"),
        ("Cancel", "Отменить"),
        ("Skip", "Пропустить"),
        ("Close", "Закрыть"),
        ("Retry", "Попробовать снова"),
        ("OK", "ОК"),
        ("Password Required", "Требуется пароль"),
        ("Please enter your password", "Пожалуйста, введите ваш пароль"),
        ("Remember password", "Запомнить пароль"),
        ("Wrong Password", "Неверный пароль"),
        ("Do you want to enter again?", "Вы хотите снова войти?"),
        ("Connection Error", "Ошибка подключения"),
        ("Error", "Ошибка"),
        ("Reset by the peer", "Сброшено пиром"),
        ("Connecting...", "Подключение..."),
        ("Connection in progress. Please wait.", "Выполняется подключение. Пожалуйста, подождите."),
        ("Please try 1 minute later", "Попробуйте через 1 минуту"),
        ("Login Error", "Ошибка входа"),
        ("Successful", "Операция успешна"),
        ("Connected, waiting for image...", "Подключено, ожидание изображения..."),
        ("Name", "Имя"),
        ("Type", "Тип"),
        ("Modified", "Изменено"),
        ("Size", "Размер"),
        ("Show Hidden Files", "Показать скрытые файлы"),
        ("Receive", "Получить"),
        ("Send", "Отправить"),
        ("Refresh File", "Обновить файл"),
        ("Local", "Локальный"),
        ("Remote", "Удаленный"),
        ("Remote Computer", "Удаленный компьютер"),
        ("Local Computer", "Локальный компьютер"),
        ("Confirm Delete", "Подтвердить удаление"),
        ("Delete", "Удалить"),
        ("Properties", "Свойства"),
        ("Multi Select", "Многоэлементный выбор"),
        ("Empty Directory", "Пустая папка"),
        ("Not an empty directory", "Папка не пуста"),
        ("Are you sure you want to delete this file?", "Вы уверены, что хотите удалить этот файл?"),
        ("Are you sure you want to delete this empty directory?", "Вы уверены, что хотите удалить пустую папку?"),
        ("Are you sure you want to delete the file of this directory?", "Вы уверены, что хотите удалить файл из этой папки?"),
        ("Do this for all conflicts", "Это относится ко всем конфликтам"),
        ("This is irreversible!", "Это необратимо!"),
        ("Deleting", "Удаление"),
        ("files", "файлы"),
        ("Waiting", "Ожидание"),
        ("Finished", "Завершено"),
        ("Speed", "Скорость"),
        ("Custom Image Quality", "Пользовательское качество изображения"),
        ("Privacy mode", "Режим конфиденциальности"),
        ("Block user input", "Блокировать пользовательский ввод"),
        ("Unblock user input", "Разблокировать пользовательский ввод"),
        ("Adjust Window", "Настроить окно"),
        ("Original", "Оригинал"),
        ("Shrink", "Уменьшить"),
        ("Stretch", "Растянуть"),
        ("Good image quality", "Хорошее качество изображения"),
        ("Balanced", "Сбалансированный"),
        ("Optimize reaction time", "Оптимизировать время реакции"),
        ("Custom", "Пользовательский"),
        ("Show remote cursor", "Показать удаленный курсор"),
        ("Show quality monitor", "Показать качество"),
        ("Disable clipboard", "Отключить буфер обмена"),
        ("Lock after session end", "Выход из учётной записи после завершения сеанса"),
        ("Insert", "Вставить"),
        ("Insert Lock", "Установить замок"),
        ("Refresh", "Обновить"),
        ("ID does not exist", "ID не существует"),
        ("Failed to connect to rendezvous server", "Не удалось подключиться к промежуточному серверу"),
        ("Please try later", "Пожалуйста, попробуйте позже"),
        ("Remote desktop is offline", "Удаленный рабочий стол не в сети"),
        ("Key mismatch", "Несоответствие ключей"),
        ("Timeout", "Тайм-аут"),
        ("Failed to connect to relay server", "Не удалось подключиться к серверу ретрансляции"),
        ("Failed to connect via rendezvous server", "Не удалось подключиться через промежуточный сервер"),
        ("Failed to connect via relay server", "Не удалось подключиться через сервер ретрансляции"),
        ("Failed to make direct connection to remote desktop", "Не удалось установить прямое подключение к удаленному рабочему столу"),
        ("Set Password", "Установить пароль"),
        ("OS Password", "Пароль ОС"),
        ("install_tip", "В некоторых случаях из-за UAC RustDesk может работать некорректно на удаленном узле. Чтобы избежать UAC, нажмите кнопку ниже, чтобы установить RustDesk в системе"),
        ("Click to upgrade", "Нажмите, чтобы проверить наличие обновлений"),
        ("Click to download", "Нажмите, чтобы скачать"),
        ("Click to update", "Нажмите, чтобы обновить"),
        ("Configure", "Настроить"),
        ("config_acc", "Чтобы удаленно управлять своим рабочим столом, вы должны предоставить RustDesk права \"доступа\""),
        ("config_screen", "Для удаленного доступа к рабочему столу вы должны предоставить RustDesk права \"снимок экрана\""),
        ("Installing ...", "Устанавливается..."),
        ("Install", "Установить"),
        ("Installation", "Установка"),
        ("Installation Path", "Путь установки"),
        ("Create start menu shortcuts", "Создать ярлыки меню \"Пуск\""),
        ("Create desktop icon", "Создать значок на рабочем столе"),
        ("agreement_tip", "Начиная установку, вы принимаете условия лицензионного соглашения"),
        ("Accept and Install", "Принять и установить"),
        ("End-user license agreement", "Лицензионное соглашение с конечным пользователем"),
        ("Generating ...", "Генерация..."),
        ("Your installation is lower version.", "Ваша установка более ранней версии"),
        ("not_close_tcp_tip", "Не закрывать это окно при использовании туннеля"),
        ("Listening ...", "Ожидаем..."),
        ("Remote Host", "Удаленная машина"),
        ("Remote Port", "Удаленный порт"),
        ("Action", "Действие"),
        ("Add", "Добавить"),
        ("Local Port", "Локальный порт"),
        ("setup_server_tip", "Для более быстрого подключения настройте свой собственный сервер подключения"),
        ("Too short, at least 6 characters.", "Слишком коротко, минимум 6 символов"),
        ("The confirmation is not identical.", "Подтверждение не совпадает"),
        ("Permissions", "Разрешения"),
        ("Accept", "Принять"),
        ("Dismiss", "Отклонить"),
        ("Disconnect", "Отключить"),
        ("Allow using keyboard and mouse", "Разрешить использование клавиатуры и мыши"),
        ("Allow using clipboard", "Разрешить использование буфера обмена"),
        ("Allow hearing sound", "Разрешить передачу звука"),
        ("Allow file copy and paste", "Разрешить копирование и вставку файлов"),
        ("Connected", "Подключено"),
        ("Direct and encrypted connection", "Прямое и зашифрованное соединение"),
        ("Relayed and encrypted connection", "Ретранслируемое и зашифрованное соединение"),
        ("Direct and unencrypted connection", "Прямое и незашифрованное соединение"),
        ("Relayed and unencrypted connection", "Ретранслируемое и незашифрованное соединение"),
        ("Enter Remote ID", "Введите удаленный ID"),
        ("Enter your password", "Введите пароль"),
        ("Logging in...", "Вход..."),
        ("Enable RDP session sharing", "Включить общий доступ к сеансу RDP"),
        ("Auto Login", "Автоматический вход (действителен, только если вы установили \"Завершение пользовательского сеанса после завершения удалённого подключения\""),
        ("Enable Direct IP Access", "Включить прямой IP-доступ"),
        ("Rename", "Переименовать"),
        ("Space", "Место"),
        ("Create Desktop Shortcut", "Создать ярлык на рабочем столе"),
        ("Change Path", "Изменить путь"),
        ("Create Folder", "Создать папку"),
        ("Please enter the folder name", "Пожалуйста, введите имя папки"),
        ("Fix it", "Исправить"),
        ("Warning", "Предупреждение"),
        ("Login screen using Wayland is not supported", "Вход в систему с использованием Wayland не поддерживается"),
        ("Reboot required", "Требуется перезагрузка"),
        ("Unsupported display server ", "Неподдерживаемый сервер дисплея"),
        ("x11 expected", "Ожидается X11"),
        ("Port", "Порт"),
        ("Settings", "Настройки"),
        ("Username", "Имя пользователя"),
        ("Invalid port", "Неверный порт"),
        ("Closed manually by the peer", "Закрыто узлом вручную"),
        ("Enable remote configuration modification", "Разрешить удаленное изменение конфигурации"),
        ("Run without install", "Запустить без установки"),
        ("Always connected via relay", "Всегда подключен через ретрансляционный сервер"),
        ("Always connect via relay", "Всегда подключаться через ретрансляционный сервер"),
        ("whitelist_tip", "Только IP-адреса из белого списка могут получить доступ ко мне"),
        ("Login", "Войти"),
        ("Logout", "Выйти"),
        ("Tags", "Ключевые слова"),
        ("Search ID", "Поиск по ID"),
        ("Current Wayland display server is not supported", "Текущий сервер отображения Wayland не поддерживается"),
        ("whitelist_sep", "Раздельно запятой, точкой с запятой, пробелом или новой строкой"),
        ("Add ID", "Добавить ID"),
        ("Add Tag", "Добавить ключевое слово"),
        ("Unselect all tags", "Отменить выбор всех тегов"),
        ("Network error", "Ошибка сети"),
        ("Username missed", "Имя пользователя отсутствует"),
        ("Password missed", "Забыли пароль"),
        ("Wrong credentials", "Неверные учетные данные"),
        ("Edit Tag", "Редактировать тег"),
        ("Unremember Password", "Не сохранять пароль"),
        ("Favorites", "Избранное"),
        ("Add to Favorites", "Добавить в избранное"),
        ("Remove from Favorites", "Удалить из избранного"),
        ("Empty", "Пусто"),
        ("Invalid folder name", "Недопустимое имя папки"),
        ("Socks5 Proxy", "Прокси-сервер Socks5"),
        ("Hostname", "Имя ПК"),
        ("Discovered", "Найдено"),
        ("install_daemon_tip", "Для запуска при загрузке необходимо установить системную службу"),
        ("Remote ID", "Удаленный идентификатор"),
        ("Paste", "Вставить"),
        ("Paste here?", "Вставить сюда?"),
        ("Are you sure to close the connection?", "Вы уверены, что хотите завершить подключение?"),
        ("Download new version", "Скачать новую версию"),
        ("Touch mode", "Сенсорный режим"),
        ("Mouse mode", "Режим мыши"),
        ("One-Finger Tap", "Касание одним пальцем"),
        ("Left Mouse", "Левая кнопка мыши"),
        ("One-Long Tap", "Одно долгое нажатие пальцем"),
        ("Two-Finger Tap", "Касание двумя пальцами"),
        ("Right Mouse", "Правая мышь"),
        ("One-Finger Move", "Движение одним пальцем"),
        ("Double Tap & Move", "Двойное нажатие и перемещение"),
        ("Mouse Drag", "Перетаскивание мышью"),
        ("Three-Finger vertically", "Тремя пальцами по вертикали"),
        ("Mouse Wheel", "Колесико мыши"),
        ("Two-Finger Move", "Движение двумя пальцами"),
        ("Canvas Move", "Перемещение холста"),
        ("Pinch to Zoom", "Сожмите, чтобы увеличить"),
        ("Canvas Zoom", "Масштаб холста"),
        ("Reset canvas", "Сбросить холст"),
        ("No permission of file transfer", "Нет разрешения на передачу файлов"),
        ("Note", "Примечание"),
        ("Connection", "Соединение"),
        ("Share Screen", "Поделиться экраном"),
        ("CLOSE", "ЗАКРЫТЬ"),
        ("OPEN", "ОТКРЫТЬ"),
        ("Chat", "Чат"),
        ("Total", "Всего"),
        ("items", "элементы"),
        ("Selected", "Выбрано"),
        ("Screen Capture", "Захват экрана"),
        ("Input Control", "Входной контроль"),
        ("Audio Capture", "Захват аудио"),
        ("File Connection", "Файловое соединение"),
        ("Screen Connection", "Подключение экрана"),
        ("Do you accept?", "Вы согласны?"),
        ("Open System Setting", "Открыть настройки системы"),
        ("How to get Android input permission?", "Как получить разрешение на ввод Android?"),
        ("android_input_permission_tip1", "Чтобы удаленное устройство могло управлять вашим Android-устройством с помощью мыши или касания, вам необходимо разрешить RustDesk использовать службу \"Специальные возможности\"."),
        ("android_input_permission_tip2", "Перейдите на следующую страницу системных настроек, найдите и войдите в [Установленные службы], включите службу [RustDesk Input]."),
        ("android_new_connection_tip", "Получен новый запрос на управление вашим текущим устройством."),
        ("android_service_will_start_tip", "Включение захвата экрана автоматически запускает службу, позволяя другим устройствам запрашивать соединение с этого устройства."),
        ("android_stop_service_tip", "Закрытие службы автоматически закроет все установленные соединения."),
        ("android_version_audio_tip", "Текущая версия Android не поддерживает захват звука, обновите ее до Android 10 или выше."),
        ("android_start_service_tip", "Нажмите [Запуск промежуточного сервера] или ОТКРЫТЬ разрешение [Захват экрана], чтобы запустить службу демонстрации экрана."),
        ("Account", "Аккаунт"),
        ("Overwrite", "Перезаписать"),
        ("This file exists, skip or overwrite this file?", "Этот файл существует, пропустить или перезаписать файл?"),
        ("Quit", "Выйти"),
        ("doc_mac_permission", "https://rustdesk.com/docs/ru/manual/mac/#включение-разрешений"),
        ("Help", "Помощь"),
        ("Failed", "Не удалось"),
        ("Succeeded", "Успешно"),
        ("Someone turns on privacy mode, exit", "Кто-то включает режим конфиденциальности, выход"),
        ("Unsupported", "Не поддерживается"),
        ("Peer denied", "Отклонено удалённым компьютером"),
        ("Please install plugins", "Пожалуйста, установите плагины"),
        ("Peer exit", "Отключен удалённым компьютером"),
        ("Failed to turn off", "Не удалось выключить"),
        ("Turned off", "Выключен"),
        ("In privacy mode", "В режиме конфиденциальности"),
        ("Out privacy mode", "Выход из режима конфиденциальности"),
        ("Language", "Язык"),
        ("Keep RustDesk background service", "Сохранить фоновый службу RustDesk"),
        ("Ignore Battery Optimizations", "Игнорировать оптимизацию батареи"),
        ("android_open_battery_optimizations_tip", "Перейдите на следующую страницу настроек "),
        ("Connection not allowed", "Подключение не разрешено"),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use temporary password", "Использовать временный пароль"),
        ("Use permanent password", "Использовать постоянный пароль"),
        ("Use both passwords", "Использовать оба пароля"),
        ("Set permanent password", "Установить постоянный пароль"),
        ("Set temporary password length", "Длина временного пароля"),
        ("Enable Remote Restart", "Включить удаленный перезапуск"),
        ("Allow remote restart", "Разрешить удаленный перезапуск"),
        ("Restart Remote Device", "Перезапустить удаленное устройство"),
        ("Are you sure you want to restart", "Вы уверены, что хотите выполнить перезапуск?"),
        ("Restarting Remote Device", "Перезагрузка удаленного устройства"),
        ("remote_restarting_tip", "Удаленное устройство перезапускается. Пожалуйста, закройте это сообщение и через некоторое время переподключитесь, используя постоянный пароль."),
    ].iter().cloned().collect();
}
