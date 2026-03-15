docker compose run --rm --entrypoint /app/create_admin backend \
  --email admin@example.com \
  --password "StrongPass123" \
  --full-name "Main Admin" \
  --phone "+380501112233"


```json
{
  "visual_style_prompt": {

    "aesthetic": {
      "direction": "Урочистий мінімалізм, повага та спокій. Чисті лінії, багато простору (white space), класична типографіка. Жодних зайвих деталей.",
      "mood": "Спокійний, надійний, співчутливий, елегантний. Не гнітючий, але серйозний.",
      "inspiration": "Класичний журнальний (editorial) дизайн, сучасні меморіальні платформи. Естетика тиші та пам'яті.",
      "one_rule": "Якщо елемент не несе ясності або заспокоєння — приберіть його."
    },

    "colors": {
      "usage_principle": "Глибокі темні фони для створення атмосфери тиші. Контрастні, але м'які світлі акценти для читабельності. Жодних яскравих кольорів.",

      "palette": {
        "dominant":  "#0D0E10",
        "surface":   "#16181B",
        "surface_2": "#22252A",
        "cta":       "#E8E9EB",
        "accent_1":  "#8E95A0",
        "accent_2":  "#4A4F56",
        "text_main": "#F3F4F6",
        "text_muted":"#9CA3AF"
      },

      "roles": {
        "#0D0E10": "Основний фон сторінки, глибокі тіні (dominant)",
        "#16181B": "Фон карток, модальних вікон, навігації (surface)",
        "#22252A": "Виділені картки, стани наведення (hover), межі (surface_2)",
        "#E8E9EB": "Головні кнопки (CTA), ключові заголовки, іконки (cta)",
        "#8E95A0": "Другорядні кнопки, обведення, роздільники (accent_1)",
        "#4A4F56": "Неактивні стани, додаткові плашки (accent_2)",
        "#F3F4F6": "Основний текст, імена, важлива інформація (text_main)",
        "#9CA3AF": "Допоміжний текст, дати, підписи (text_muted)"
      },

      "combinations": {
        "hero_section":   "dominant bg + text_main text + cta accent",
        "card_default":   "surface bg + text_main text + text_muted secondary",
        "card_featured":  "surface_2 bg + text_main text + cta detail",
        "button_primary": "cta bg + dominant text",
        "button_ghost":   "transparent bg + accent_1 border + text_main text",
        "status_done":    "surface_2 bg + text_muted text"
      },

      "gradients": {
        "hero_overlay":   "linear-gradient(180deg, rgba(13,14,16,0) 0%, #0D0E10 100%)",
        "image_scrim":    "linear-gradient(to bottom, transparent 50%, rgba(22,24,27,0.8) 100%)",
        "accent_shimmer": "linear-gradient(90deg, #22252A, #4A4F56, #22252A)"
      },

      "forbidden": [
        "Будь-які неонові або яскраві кольори (червоний, зелений, фіолетовий тощо)",
        "Чистий чорний (#000000) для великих площин (використовуємо #0D0E10 для глибини та м'якості)",
        "Чистий білий (#FFFFFF) для фону в темній темі",
        "Більше двох відтінків в одній картці"
      ]
    },

    "typography": {
      "principle": "Два шрифти. Класична антиква (Serif) для заголовків та імен, чистий гротеск (Sans-Serif) для легкого читання описового тексту.",

      "fonts": {
        "primary": {
          "name": "Playfair Display / Lora",
          "character": "Елегантний, класичний, з повагою. Створює відчуття монументальності та пам'яті.",
          "use": "Заголовки, імена, важливі цитати"
        },
        "secondary": {
          "name": "Inter / Roboto",
          "character": "Нейтральний, максимально читабельний.",
          "use": "Основний текст, дати, кнопки, навігація, форми"
        }
      },

      "scale": {
        "display":      { "size": "36–42px", "weight": 400, "font": "primary" },
        "title":        { "size": "24–28px", "weight": 500, "font": "primary" },
        "subtitle":     { "size": "18–20px", "weight": 400, "font": "primary" },
        "card_heading": { "size": "16–18px", "weight": 500, "font": "secondary" },
        "body":         { "size": "14–16px", "weight": 400, "font": "secondary", "line_height": "1.6" },
        "label":        { "size": "11–12px", "weight": 500, "font": "secondary", "tracking": "0.08em", "transform": "uppercase" },
        "dates":        { "size": "13–14px", "weight": 400, "font": "secondary", "color": "text_muted" }
      },

      "rules": [
        "Ніколи не використовувати грайливі або рукописні шрифти (на кшталт Caveat)",
        "Великі міжрядкові інтервали для легкого читання (не менше 1.6 для body)",
        "Імена завжди виділяти шрифтом primary (Serif)"
      ]
    },

    "shape_and_space": {
      "corner_radius": {
        "container_lg": "16px (м'які, але стримані кути)",
        "card":         "8–12px",
        "button":       "4–6px (більш класичні, прямокутні форми)",
        "input":        "6px",
        "image_frame":  "4px або класичні арки (arch shape) для портретів"
      },

      "spacing": {
        "screen_horizontal_padding": "20–24px",
        "between_cards":             "16–24px",
        "card_inner_padding":        "24–32px",
        "section_gap":               "40–60px (багато простору)"
      },

      "white_space": "Критично важливо. Контент має «дихати». Порожній простір використовується для створення відчуття тиші та спокою.",

      "transitions": {
        "dark_to_light": "М'які градієнтні переходи або тонкі лінії (#22252A), жодних різких хвиль або круглих вирізів"
      }
    },

    "shadows": {
      "principle": "Майже непомітні тіні, що лише злегка відділяють елементи. Відчуття матового паперу.",
      "container": "0 20px 40px rgba(0,0,0,0.4)",
      "card_default": "0 4px 12px rgba(0,0,0,0.2)",
      "image_shadow": "0 8px 24px rgba(0,0,0,0.5)"
    },

    "decorative_elements": {
      "lines": {
        "description": "Тонкі (1px) горизонтальні або вертикальні лінії для розділення контенту",
        "color": "accent_2 (#4A4F56)"
      },
      "image_treatments": {
        "description": "Фотографії мають бути злегка знебарвлені (desaturated) або переведені в ЧБ для відповідності загальному тону",
        "filter": "grayscale(20%) contrast(1.1)"
      },
      "icons": {
        "description": "Мінімалістичні лінійні іконки. Тематичні: свічка, квітка, голуб, лист.",
        "stroke_width": "1.5px"
      }
    },

    "motion": {
      "principle": "Повільні, плавні та поважні анімації. Жодних різких рухів чи відскоків.",

      "screen_enter": {
        "effect":   "fade in",
        "keyframe": "opacity:0 → opacity:1",
        "duration": "0.8s",
        "easing":   "ease-in-out"
      },
      "card_hover":   { "effect": "легке освітлення фону або зміна opacity", "duration": "0.3s" },
      "image_reveal": { "effect": "повільний fade + мінімальний scale(1.02)", "duration": "1.2s" },

      "rules": [
        "Ніколи не використовувати 'spring' (пружинні) анімації",
        "Всі анімації мають тривати довше звичайного (мінімум 0.3s для мікроінтеракцій)",
        "Уникати зміщення контенту (translateY/X) при наведенні, лише зміна кольору або світла"
      ]
    },

    "component_patterns": {

      "hero_section": {
        "pattern": "Глибокий темний фон + приглушене фонове фото з градієнтом + класичний Serif заголовок по центру + дати"
      },

      "memorial_card": {
        "pattern": "surface bg + портрет (можливо у формі арки) + ім'я (Serif) + дати (muted) + тонка роздільна лінія",
        "hover": "Плавна поява легкої тіні"
      },

      "primary_button": {
        "pattern": "cta bg (світлий) + dominant text (темний) + строгі кути (4-6px)",
        "hover": "Зменшення яскравості (dimming), без підскакувань"
      },

      "empty_state": {
        "pattern": "Мінімалістична іконка (наприклад, свічка або гілка) + Serif заголовок + стриманий текст",
        "tone": "Співчутливий, інформативний, без емодзі"
      }
    },

    "forbidden": [
      "Яскраві градієнти (blobs), кольорові плями",
      "Емодзі у будь-якому вигляді",
      "Рукописні шрифти (Caveat, Pacifico тощо)",
      "Пружинні (bouncy) анімації або різкі ховери",
      "Повністю круглі кнопки (FAB) – вони виглядають надто грайливо"
    ]
  }
}
```