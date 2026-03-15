import Link from "next/link";

import { HomeCatalogPreview } from "@/components/home-catalog-preview";

export default function HomePage() {
  return (
    <>
      <section className="hero">
        <div className="hero-grid">
          <div className="hero-copy">
            <p className="eyebrow">Крематорій та ритуальні послуги</p>
            <h1 className="display">
              Останній маршрут теж можна організувати без хаосу, паніки й кривих цвяхів.
            </h1>
            <p className="lead">
              Допомагаємо з кремацією, підбором домовини, урни та всього необхідного для церемонії.
              Якщо вже подія точно не скасується, то хоча б організуємо її швидко, акуратно і без
              зайвих пригод для живих.
            </p>
            <div className="inline-actions">
              <Link href="/catalog" className="button">
                Обрати послуги
              </Link>
              <Link href="/schedule" className="button ghost">
                Переглянути розклад
              </Link>
            </div>
          </div>

          <div className="hero-aside">
            <div className="panel elevated">
              <p className="eyebrow">Чому обирають нас</p>
              <div className="metric-grid">
                <div className="metric-card">
                  <span>Швидко</span>
                  <strong>Працюємо так оперативно, ніби дедлайн уже зовсім остаточний</strong>
                </div>
                <div className="metric-card">
                  <span>Охайно</span>
                  <strong>Усе рівно, чисто і без відчуття, що це робили нашвидкуруч лопатою</strong>
                </div>
                <div className="metric-card">
                  <span>Надійно</span>
                  <strong>Цвяхи добиваємо до кінця, графік теж, а халтуру лишаємо конкурентам</strong>
                </div>
                <div className="metric-card">
                  <span>Зручно</span>
                  <strong>Менше біганини для родини, більше порядку в момент, коли й так не до кардіо</strong>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <section className="section">
        <div className="section-heading">
          <div>
            <p className="eyebrow">Наш підхід</p>
            <h2 className="section-title">Беремо на себе те, що в складний момент не хочеться тягнути самому.</h2>
          </div>
        </div>

        <div className="card-grid three-columns">
          <article className="panel feature-card">
            <h3>Швидка організація</h3>
            <p className="muted">
              Без нескінченних дзвінків, списків і сімейних нарад на тему “а що тепер робити”.
              Збираємо все потрібне швидко й по-людськи.
            </p>
          </article>
          <article className="panel feature-card">
            <h3>Акуратне виконання</h3>
            <p className="muted">
              Домовини, урни, атрибутика й зал готуємо охайно. Бо чорний гумор чорним гумором,
              а криві шви й перекошені кришки нікому не смішні.
            </p>
          </article>
          <article className="panel feature-card">
            <h3>Людське ставлення</h3>
            <p className="muted">
              Пояснюємо просто, без казенної мови і без виразу обличчя, ніби ми тут уже все бачили
              й вам теж пора звикати.
            </p>
          </article>
        </div>
      </section>

      <HomeCatalogPreview />

      <section className="section split-section">
        <div className="panel">
          <p className="eyebrow">Як це відбувається</p>
          <h2 className="section-title">Простий шлях від першого звернення до підготовленої церемонії.</h2>
          <div className="timeline">
            <div className="timeline-item">
              <strong>01</strong>
              <div>
                <h4>Обираєте послуги</h4>
                <p className="muted">Переглядаєте кремаційні послуги, домовини, урни та все, що знадобиться для фінального сетапу.</p>
              </div>
            </div>
            <div className="timeline-item">
              <strong>02</strong>
              <div>
                <h4>Залишаєте контакти</h4>
                <p className="muted">Лишаєте свої дані, щоб ми могли все підтвердити без квесту “знайди родича, який щось вирішує”.</p>
              </div>
            </div>
            <div className="timeline-item">
              <strong>03</strong>
              <div>
                <h4>Узгоджуєте деталі</h4>
                <p className="muted">Обираєте час, адресу та склад замовлення, а ми стежимо, щоб нічого не поїхало не туди і не в той день.</p>
              </div>
            </div>
            <div className="timeline-item">
              <strong>04</strong>
              <div>
                <h4>Ми беремо все на себе</h4>
                <p className="muted">Працюємо вчасно, акуратно й з повагою. Коли вже фінал неминучий, нехай він хоча б буде гідно організований.</p>
              </div>
            </div>
          </div>
        </div>

        <div className="panel">
          <p className="eyebrow">Чому варто довірити це нам</p>
          <h2 className="section-title">Не говоримо зайвого. Просто робимо свою роботу якісно.</h2>
          <p className="lead">
            Ми цінуємо порядок, точність і здоровий чорний гумор. Тому працюємо так, щоб рідним не
            доводилося хвилюватися ані через терміни, ані через те, що хтось десь недобив гвіздок.
          </p>
          <div className="stack-list">
            <div className="detail-row">
              <span>Швидкість</span>
              <strong>Оперативно приймаємо замовлення, бо в таких справах “давайте після вихідних” звучить особливо погано</strong>
            </div>
            <div className="detail-row">
              <span>Якість</span>
              <strong>Охайно готуємо атрибутику, зал і кожну деталь, щоб усе виглядало пристойно, а не як останній розпродаж</strong>
            </div>
            <div className="detail-row">
              <span>Надійність</span>
              <strong>Усе робимо як слід: рівно, міцно й без недобитих цвяхів, бо сюрпризів тут уже й так достатньо</strong>
            </div>
          </div>
        </div>
      </section>
    </>
  );
}
