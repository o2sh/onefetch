
cli-about = Инструмент командной строки для получения информации о Git
cli-usage-header = Использование
cli-arguments-header = Аргументы
cli-arguments-input = Запустить так, как будто onefetch был запущен в <ввод> вместо текущего рабочего каталога
cli-options-header = Опции
cli-options-help = Показать помощь { $short ->
    [true] (подробнее с '--help')
    *[false] (кратко с '-h')
}
cli-options-version = Показать версию

# Value name
cli-value-input = ВВОД
cli-value-num = КОЛ-ВО
cli-value-field = ПОЛЕ
cli-value-regex = РЕГ. ВЫРАЖЕНИЕ
cli-value-exclude = ИСКЛЮЧЕННОЕ
cli-value-type = ТИП
cli-value-separator = РАЗДЕЛИТЕЛЬ
cli-value-string = СТРОКА
cli-value-language = ЯЗЫК
cli-value-when = КОГДА
cli-value-image = ИЗОБРАЖЕНИЕ
cli-value-protocol = ПРОТОКОЛ
cli-value-value = ЗНАЧЕНИЕ
cli-value-format = ФОРМАТ
cli-value-shell = ОБОЛОЧКА

# INFO
cli-info-heading = ИНФОРМАЦИЯ
cli-info-disabled_fields = Позволяет отключить отображение ПОЛЯ(ПОЛЕЙ) в выводе
cli-info-no_title = Скрывает заголовок
cli-info-number_of_authors-short = Максимальное КОЛ-ВО авторов для отображения (по умолчанию: {$def})
cli-info-number_of_authors-long = 
    Максимальное КОЛ-ВО авторов для отображения 
    
    (по умолчанию: {$def})
cli-info-number_of_languages-short = Максимальное КОЛ-ВО языков для отображения (по умолчанию: {$def})
cli-info-number_of_languages-long = 
    Максимальное КОЛ-ВО языков для отображения 

    (по умолчанию: {$def})
cli-info-number_of_file_churns-short = Максимальное КОЛ-ВО изменений файлов для отображения (по умолчанию: {$def})
cli-info-number_of_file_churns-long = 
    Максимальное КОЛ-ВО изменений файлов для отображения 
    
    (по умолчанию: {$def})
cli-info-churn_pool_size-short = Минимальное КОЛ-ВО коммитов от HEAD, используемых для вычисления сводки изменений
cli-info-churn_pool_size-long = 
    Минимальное КОЛ-ВО коммитов от HEAD, используемых для вычисления сводки изменений

    По умолчанию фактическое значение недетерминировано из-за вычислений на основе времени
    и будет отображаться под заголовком информации "Изменения (КОЛ-ВО)"
cli-info-exclude = Игнорировать все файлы и каталоги, соответствующие ИСКЛЮЧЕННОМУ
cli-info-no_bots = Исключить коммиты [ботов]. Используйте <РЕГ. ВЫРАЖЕНИЕ> для переопределения шаблона по умолчанию
cli-info-no_merges = Игнорировать коммиты слияния
cli-info-email = Показать адрес электронной почты каждого автора
cli-info-http_url = Отображать URL репозитория как HTTP
cli-info-hide_token = Скрыть токен в URL репозитория
cli-info-include_hidden = Учитывать скрытые файлы и каталоги
cli-info-tipe-short = Фильтровать вывод по типу языка (по умолчанию: {$def}) (возможные значения: {$pos})
cli-info-tipe-long = 
    Фильтровать вывод по типу языка

    (по умолчанию: {$def}) 
    (возможные значения: {$pos})

# TEXT FORMATTING
cli-text-heading = ФОРМАТИРОВАНИЕ ТЕКСТА
cli-text-colors = 
    Изменяет цвета текста (X X X...)
    
    Идет в порядке заголовка, ~, подчеркивания, подзаголовка, двоеточия и информации
    
    Например:
    
    '--text-colors 9 10 11 12 13 14'
cli-text-iso_time = Использовать временные метки в формате ISO 8601
cli-text-number_separator = Какой РАЗДЕЛИТЕЛЬ тысяч использовать
cli-text-no_bold = Отключает жирное форматирование

# ASCII
cli-ascii-heading = ASCII
cli-ascii-ascii_input = 
    Принимает непустую СТРОКУ в качестве входных данных для замены ASCII логотипа

    Можно передать сгенерированную СТРОКУ с помощью подстановки команд

    Например:

    '--ascii-input "$(fortune | cowsay -W 25)"'
cli-ascii-ascii_colors = Цвета (X X X...) для печати ASCII арта
cli-ascii-ascii_language = ASCII арт какого ЯЗЫКА печатать
cli-ascii-true_color = 
    Указать, когда использовать true-color

    Если установлено в auto: true-color будет включен, если он поддерживается терминалом

# IMAGE
cli-image-heading = ИЗОБРАЖЕНИЕ
cli-image-image = Путь к файлу ИЗОБРАЖЕНИЯ
cli-image-image_protocol = Какой ПРОТОКОЛ изображения использовать
cli-image-color_resolution = ЗНАЧЕНИЕ разрешения цвета, используемого с бэкендом SIXEL

# VISUALS
cli-visuals-heading = ВИЗУАЛЬНЫЕ ЭЛЕМЕНТЫ
cli-visuals-no_color_palette = Скрывает цветовую палитру
cli-visuals-no_art = Скрывает ASCII арт или изображение, если оно предоставлено
cli-visuals-nerd_fonts = 
    Использовать иконки Nerd Font

    Заменяет языковые чипы иконками Nerd Font
# not sure if       ↑ this ↑       is right translation for "chips"

# DEVELOPER
cli-dev-heading = ДЛЯ РАЗРАБОТЧИКОВ
cli-dev-output = Выводит Onefetch в определенном формате
cli-dev-completion = Выводит файл автозаполнения для указанной ОБОЛОЧКИ

# OTHER
cli-other-heading = ПРОЧЕЕ
cli-other-languages = Выводит поддерживаемые языки
cli-other-package_managers = Выводит поддерживаемые менеджеры пакетов