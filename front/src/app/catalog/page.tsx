import { CatalogBrowser } from "@/components/catalog-browser";

export default function CatalogPage() {
  return (
    <section className="section">
      <div className="section-heading">
        <div>
          <p className="eyebrow">Каталог</p>
          <h1 className="section-title">Послуги та товари для гідного прощання без хаосу й біганини.</h1>
          <p className="lead">
            Переглядайте кремаційні послуги, домовини, урни та супутню атрибутику. Обирайте спокійно:
            коли день і так важкий, останнє, чого хочеться, це шукати добру домовину по всьому місту.
          </p>
        </div>
      </div>

      <CatalogBrowser />
    </section>
  );
}
