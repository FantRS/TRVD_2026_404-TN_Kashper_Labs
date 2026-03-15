import Link from "next/link";

export function SiteFooter() {
  return (
    <footer className="site-footer">
      <div className="site-shell footer-grid">
        <div className="footer-block">
          <p className="eyebrow">CrematoryRs</p>
          <h3 className="section-title">Повага, тиша, точність у кожному кроці.</h3>
          <p className="muted">
            Допомагаємо організувати прощання, підібрати потрібні товари й швидко узгодити деталі.
            Коли життя вже сказало своє останнє слово, ми беремося за решту організаційних реплік.
          </p>
        </div>

        <div className="footer-block">
          <p className="eyebrow">Навігація</p>
          <div className="footer-links">
            <Link href="/catalog">Каталог</Link>
            <Link href="/schedule">Розклад</Link>
            <Link href="/account">Особистий кабінет</Link>
            <Link href="/login">Вхід</Link>
          </div>
        </div>

        <div className="footer-block">
          <p className="eyebrow">Контакт</p>
          <div className="footer-links">
            <span>Київ, меморіальний сервісний центр</span>
            <span>Пн-Нд, 09:00-18:00</span>
            <span>support@crematoryrs.local</span>
          </div>
        </div>
      </div>
    </footer>
  );
}
