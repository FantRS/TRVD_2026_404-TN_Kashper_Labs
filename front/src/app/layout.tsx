import type { Metadata } from "next";

import { Providers } from "@/components/providers";
import { SiteFooter } from "@/components/site-footer";
import { SiteHeader } from "@/components/site-header";

import "@/app/globals.css";

export const metadata: Metadata = {
  title: "CrematoryRs",
  description: "Повнофункціональний сайт крематорію з каталогом, гаманцем, замовленнями та бронюванням.",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="uk">
      <body>
        <Providers>
          <div className="app-frame">
            <SiteHeader />
            <main className="site-shell page-content">{children}</main>
            <SiteFooter />
          </div>
        </Providers>
      </body>
    </html>
  );
}
