use gtk4::gdk::ModifierType;
use gtk4::prelude::*;
use gtk4::{glib, AboutDialog, Align, Application, ApplicationWindow, Builder, Button, CheckButton, ComboBoxText, Entry, EventControllerKey, Frame, Label, Orientation, Separator, ShortcutsWindow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let app = Application::builder().build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let gtk_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("CharTrace")
        .child(&gtk_box)
        .build();

    let settings_box = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .margin_start(10)
        .margin_top(10)
        .build();

    let language_selector = ComboBoxText::builder()
        .build();

    language_selector.append(Some("en"), "English");
    language_selector.append(Some("ru"), "Russian");
    language_selector.append(Some("kz"), "Kazakh");
    language_selector.set_active_id(Some("en"));

    let texts = Rc::new(HashMap::from([
        ("en".to_string(), vec!["Enter a string", "Enter a character", "Find", "Count:", "Positions:", "None", "Please enter a single character.", "Case sensitive", "About", "This app helps you find character occurrences in strings.", "Shortcuts", "General", "Find", "Toggle Case Sensitivity"]),
        ("ru".to_string(), vec!["Введите строку", "Введите символ", "Найти", "Количество:", "Позиции:", "Нет", "Пожалуйста, введите один символ.", "Чувствительный к регистру", "О программе", "Это приложение помогает находить вхождения символов в строках.", "Горячие клавиши", "Общие", "Найти", "Переключить чувствительность к регистру"]),
        ("kz".to_string(), vec!["Жолды енгізіңіз", "Таңбаны енгізіңіз", "Табу", "Саны:", "Позициялар:", "Жоқ", "Өтінемін, бір таңбаны енгізіңіз.", "Регистрге сезімтал", "Бағдарлама туралы", "Бұл бағдарлама жолдардағы таңба орындарын табуға көмектеседі.", "Ыстық пернелер", "Жалпы", "Табу", "Регистке сезімталдықты ауыстыру"])
    ]));

    let current_language = Rc::new(RefCell::new("en".to_string()));

    let theme_selector = ComboBoxText::builder()
        .build();

    theme_selector.append(Some("dark"), "Dark");
    theme_selector.append(Some("light"), "Light");
    theme_selector.set_active_id(Some("dark"));

    settings_box.append(&language_selector);
    settings_box.append(&theme_selector);

    theme_selector.connect_changed(move |combo| {
        if let Some(theme) = combo.active_id() {
            let theme_name = theme.to_string();
            set_theme(&theme_name);
        }
    });

    let about_box = gtk4::Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .margin_end(10)
        .margin_bottom(10)
        .halign(Align::End)
        .build();

    let about_button = Button::builder()
        .label(texts["en"][8])
        .build();

    let texts_ref = texts.clone();
    let current_language_ref = current_language.clone();
    about_button.connect_clicked(move |_| {
        let lang = current_language_ref.borrow();
        let comments = texts_ref.get(lang.as_str())
            .and_then(|v| v.get(9))
            .unwrap_or(&"This app helps you find character occurrences in strings.");

        let about_dialog = AboutDialog::builder()
            .modal(true)
            .program_name("CharTrace")
            .comments(*comments)
            .version("1.0.0")
            .authors(["dev-dmitrii-g"])
            .copyright("© 2024")
            .build();

        about_dialog.show();
    });

    let shortcuts_button = Button::builder()
        .label(texts["en"][10])
        .build();

    shortcuts_button.connect_clicked(move |_| {
        let ui_data = include_str!("../shortcuts.ui");

        let builder = Builder::from_string(ui_data);

        let shortcuts_window: ShortcutsWindow = builder
            .object("shortcuts_window")
            .expect("Could not find shortcuts_window in the UI file");

        shortcuts_window.show();
    });

    about_box.append(&about_button);
    about_box.append(&shortcuts_button);

    let main_box = gtk4::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(7)
        .vexpand(true)
        .valign(Align::Center)
        .halign(Align::Center)
        .width_request(350)
        .build();

    let string_label = Label::builder()
        .halign(Align::Start)
        .label(texts["en"][0])
        .build();

    let string_entry = Entry::builder()
        .margin_bottom(15)
        .build();

    let char_label = Label::builder()
        .halign(Align::Start)
        .label(texts["en"][1])
        .build();

    let char_entry = Entry::builder()
        .margin_bottom(5)
        .build();

    let button = Button::builder()
        .label(texts["en"][2])
        .margin_bottom(10)
        .build();

    let separator = Separator::builder()
        .orientation(Orientation::Vertical)
        .build();

    let result_label = Label::builder()
        .margin_start(10)
        .margin_top(10)
        .margin_bottom(10)
        .halign(Align::Start)
        .build();

    let string_case_separator = Separator::builder()
        .orientation(Orientation::Horizontal)
        .build();

    let check_uppercase = CheckButton::builder()
        .label(texts["en"][7])
        .build();

    let check_uppercase_ref = check_uppercase.clone();
    let event = EventControllerKey::new();
    event.connect_key_pressed(move |_, key, _, modifiers| {
        if modifiers.contains(ModifierType::CONTROL_MASK) && key == gtk4::gdk::Key::u {
            check_uppercase_ref.set_active(!check_uppercase_ref.is_active());

            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    });

    window.add_controller(event);

    language_selector.connect_changed({
        let string_label = string_label.clone();
        let char_label = char_label.clone();
        let button = button.clone();
        let texts = texts.clone();
        let current_language = current_language.clone();
        let check_uppercase = check_uppercase.clone();

        move |combo| {
            if let Some(lang) = combo.active_id() {
                *current_language.borrow_mut() = lang.to_string();
                string_label.set_text(&texts[&*current_language.borrow()][0]);
                char_label.set_text(&texts[&*current_language.borrow()][1]);
                button.set_label(&texts[&*current_language.borrow()][2]);
                check_uppercase.set_label(Some(&texts[&*current_language.borrow()][7]));
                about_button.set_label(&texts[&*current_language.borrow()][8]);
                shortcuts_button.set_label(&texts[&*current_language.borrow()][10]);
            }
        }
    });

    let result_frame = Frame::builder()
        .margin_top(10)
        .visible(false)
        .build();

    let string_ref = string_entry.clone();
    let char_ref = char_entry.clone();
    let result_ref = result_label.clone();
    let result_frame_ref = result_frame.clone();
    let check_uppercase_ref = check_uppercase.clone();
    let texts_ref = texts.clone();
    let current_language_ref = current_language.clone();


    let find_and_display_char_occurrences = Rc::new(move || {
        let input_string = string_ref.text().to_string();
        let input_char = char_ref.text().to_string();
        let case_sensitive = check_uppercase_ref.is_active();

        if input_char.chars().count() == 1 {
            let target_char = input_char.chars().next().unwrap();

            let (input_string, target_char) = if !case_sensitive {
                (input_string.to_lowercase(), target_char.to_lowercase().next().unwrap())
            } else {
                (input_string, target_char)
            };

            let count = input_string.chars().filter(|&c| c == target_char).count();
            let positions: Vec<usize> = input_string
                .char_indices()
                .filter(|&(_, c)| c == target_char)
                .map(|(i, _)| i)
                .collect();

            let positions_string = if positions.is_empty() {
                texts_ref.get(&*current_language_ref.borrow())
                    .and_then(|v| v.get(5))
                    .unwrap_or(&"No occurrences found.")
                    .to_string()
            } else {
                positions.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ")
            };

            result_ref.set_text(&format!(
                "{} {}\n{} {}",
                &texts_ref.get(&*current_language_ref.borrow()).unwrap_or(&vec![""])[3],
                count,
                &texts_ref.get(&*current_language_ref.borrow()).unwrap_or(&vec![""])[4],
                positions_string
            ));
        } else {
            result_ref.set_text(&texts_ref.get(&*current_language_ref.borrow()).unwrap_or(&vec![""])[6]);
        }
        result_frame_ref.set_visible(true);
    });

    let find_and_display_char_occurrences_clone = find_and_display_char_occurrences.clone();
    button.connect_clicked(move |_| {
        find_and_display_char_occurrences_clone();
    });

    let event = EventControllerKey::new();
    event.connect_key_pressed(move |_, key, _, modifiers| {
        if modifiers.contains(ModifierType::CONTROL_MASK) && key == gtk4::gdk::Key::Return {
            find_and_display_char_occurrences();
            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    });

    window.add_controller(event);

    result_frame.set_child(Some(&result_label));

    main_box.append(&string_label);
    main_box.append(&string_entry);
    main_box.append(&char_label);
    main_box.append(&char_entry);
    main_box.append(&button);
    main_box.append(&separator);
    main_box.append(&check_uppercase);
    main_box.append(&string_case_separator);
    main_box.append(&result_frame);

    gtk_box.append(&settings_box);
    gtk_box.append(&main_box);
    gtk_box.append(&about_box);

    window.present();

    set_theme("Dark");
}

fn set_theme(theme: &str) {
    let settings = gtk4::Settings::default();

    match theme {
        "light" => settings.unwrap().set_gtk_application_prefer_dark_theme(false),
        _ => settings.unwrap().set_gtk_application_prefer_dark_theme(true),
    }
}