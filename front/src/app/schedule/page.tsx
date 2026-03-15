import Link from "next/link";

import { ScheduleExplorer } from "@/components/schedule-explorer";

export default function SchedulePage() {
  return (
    <section className="section">
      <div className="section-heading">
        <div>
          <p className="eyebrow">Розклад</p>
          <h1 className="section-title">Оберіть час, коли все пройде без накладок, паніки й черг.</h1>
          <p className="lead">
            Тут можна переглянути вільні часові вікна й заздалегідь зрозуміти, коли ми зможемо все
            провести без штовханини. Навіть остання подорож любить хороший таймінг.
          </p>
        </div>
        <Link href="/account" className="button ghost">
          Відкрити кабінет
        </Link>
      </div>

      <ScheduleExplorer />
    </section>
  );
}
