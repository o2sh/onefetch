cli-input = Запустить так, как будто onefetch был запущен в <input> вместо текущего рабочего каталога

# Value name
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
cli-info-disabled-fields = Позволяет отключить отображение ПОЛЯ(ПОЛЕЙ) в выводе
cli-info-no-title = Скрывает заголовок
cli-info-number-of-authors = Максимальное КОЛ-ВО авторов для отображения
cli-info-number-of-languages = Максимальное КОЛ-ВО языков для отображения
cli-info-number-of-file-churns = Максимальное КОЛ-ВО изменений файлов для отображения
cli-info-churn-pool-size = 
    Минимальное КОЛ-ВО коммитов от HEAD, используемых для вычисления сводки изменений

    По умолчанию фактическое значение недетерминировано из-за вычислений на основе времени
    и будет отображаться под заголовком информации "Изменения (КОЛ-ВО)"
cli-info-exclude = Игнорировать все файлы и каталоги, соответствующие ИСКЛЮЧЕННОМУ
cli-info-no-bots = Исключить коммиты [ботов]. Используйте <РЕГ. ВЫРАЖЕНИЕ> для переопределения шаблона по умолчанию
cli-info-no-merges = Игнорировать коммиты слияния
cli-info-email = Показать адрес электронной почты каждого автора
cli-info-http-url = Отображать URL репозитория как HTTP
cli-info-hide-token = Скрыть токен в URL репозитория
cli-info-include-hidden = Учитывать скрытые файлы и каталоги
cli-info-type = Фильтровать вывод по типу языка

# TEXT FORMATTING
cli-text-heading = ФОРМАТИРОВАНИЕ ТЕКСТА
cli-text-colors = 
    Изменяет цвета текста (X X X...)
    
    Идет в порядке заголовка, ~, подчеркивания, подзаголовка, двоеточия и информации
    
    Например:
    
    '--text-colors 9 10 11 12 13 14'
cli-text-iso-time = Использовать временные метки в формате ISO 8601
cli-text-number-separator = Какой РАЗДЕЛИТЕЛЬ тысяч использовать
cli-text-no-bold = Отключает жирное форматирование

# ASCII
cli-ascii-heading = ASCII
cli-ascii-ascii-input = 
    Принимает непустую СТРОКУ в качестве входных данных для замены ASCII логотипа

    Можно передать сгенерированную СТРОКУ с помощью подстановки команд

    Например:

    '--ascii-input "$(fortune | cowsay -W 25)"'
cli-ascii-ascii-colors = Цвета (X X X...) для печати ASCII арта
cli-ascii-ascii-language = ASCII арт какого ЯЗЫКА печатать
cli-ascii-true-color = 
    Указать, когда использовать true-color

    Если установлено в auto: true-color будет включен, если он поддерживается терминалом

# VISUALS
cli-visuals-heading = ВИЗУАЛЬНЫЕ ЭЛЕМЕНТЫ
cli-visuals-no-color-palette = Скрывает цветовую палитру
cli-visuals-no-art = Скрывает ASCII арт или изображение, если оно предоставлено
cli-visuals-nerd-fonts = 
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
cli-other-package-managers = Выводит поддерживаемые менеджеры пакетов